use rustler::{Term, NifResult, Error};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer as SdkSigner},
    transaction::Transaction,
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    message::Message,
};
use mpl_token_metadata::{
    accounts::{Metadata, MasterEdition},
    instructions::{
        CreateMetadataAccountV3, CreateMetadataAccountV3InstructionArgs,
        CreateMasterEditionV3, CreateMasterEditionV3InstructionArgs,
    },
    types::{DataV2, Collection, Creator as MetadataCreator, UseMethod, Uses, CollectionDetails},
};
use mpl_bubblegum::{
    instructions::{CreateTreeConfig, CreateTreeConfigInstructionArgs, MintToCollectionV1, MintToCollectionV1InstructionArgs},
    types::{MetadataArgs, TokenProgramVersion, TokenStandard, Creator as BubblegumCreator},
};
use std::{str::FromStr, collections::HashMap, sync::Arc};
use serde::Deserialize;

rustler::init!(
    "Elixir.Cnft.NifBridge",
    [
        create_signer,
        create_collection,
        create_merkle_tree,
        mint_to_collection
    ]
);

#[derive(Debug, Clone)]
struct SignerHandle {
    keypair: Arc<Keypair>,
}

impl SdkSigner for SignerHandle {
    fn try_pubkey(&self) -> Result<Pubkey, solana_sdk::signer::SignerError> {
        Ok(self.keypair.pubkey())
    }

    fn try_sign_message(&self, message: &[u8]) -> Result<solana_sdk::signature::Signature, solana_sdk::signer::SignerError> {
        self.keypair.try_sign_message(message)
    }

    fn is_interactive(&self) -> bool {
        false
    }
}

impl rustler::types::Encoder for SignerHandle {
    fn encode<'a>(&self, env: rustler::Env<'a>) -> Term<'a> {
        self.keypair.pubkey().to_string().encode(env)
    }
}

impl<'a> rustler::types::Decoder<'a> for SignerHandle {
    fn decode(term: Term<'a>) -> NifResult<Self> {
        let pubkey_str: String = term.decode()?;
        let _pubkey = Pubkey::from_str(&pubkey_str)
            .map_err(|_| Error::Atom("invalid_pubkey"))?;
        Ok(SignerHandle { keypair: Arc::new(Keypair::new()) }) // Replace with actual keypair
    }
}

#[derive(Debug, Deserialize)]
struct CreatorInput {
    address: String,
    verified: bool,
    share: u8,
}

#[rustler::nif]
fn create_signer(secret_key: Vec<u8>) -> Result<SignerHandle, Error> {
    if secret_key.len() != 64 {
        return Err(Error::Atom("invalid_secret_key_length"));
    }

    Keypair::from_bytes(&secret_key)
        .map(|keypair| SignerHandle { keypair: Arc::new(keypair) })
        .map_err(|_| Error::Atom("invalid_secret_key"))
}

#[rustler::nif]
fn create_collection(
    rpc_url: String,
    signer: SignerHandle,
    config: HashMap<String, String>,
) -> Result<HashMap<String, String>, Error> {
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let collection_mint = Keypair::new();
    let payer_pubkey = signer.keypair.pubkey();

    let creators = if let Some(creators_json) = config.get("creators") {
        let creators: Vec<CreatorInput> = serde_json::from_str(creators_json)
            .map_err(|_| Error::Atom("invalid_creators_format"))?;
        
        Some(
            creators
                .iter()
                .map(|c| {
                    Ok(MetadataCreator {
                        address: Pubkey::from_str(&c.address)
                            .map_err(|_| Error::Atom("invalid_address"))?,
                        verified: c.verified,
                        share: c.share,
                    })
                })
                .collect::<Result<Vec<_>, Error>>()?
        )
    } else {
        None
    };

    let metadata_data = DataV2 {
        name: config.get("name").unwrap_or(&"Collection".to_string()).clone(),
        symbol: config.get("symbol").unwrap_or(&"COLL".to_string()).clone(),
        uri: config.get("metadata_url").unwrap_or(&"".to_string()).clone(),
        seller_fee_basis_points: config.get("fee_basis_points")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        creators,
        collection: Some(Collection {
            key: collection_mint.pubkey(),
            verified: false,
        }),
        uses: Some(Uses {
            use_method: UseMethod::Multiple,
            remaining: 0,
            total: 0,
        }),
    };

    let metadata_account = Metadata::find_pda(&collection_mint.pubkey()).0;
    let master_edition = MasterEdition::find_pda(&collection_mint.pubkey()).0;

    let create_metadata_ix = CreateMetadataAccountV3 {
        metadata: metadata_account,
        mint: collection_mint.pubkey(),
        mint_authority: payer_pubkey,
        payer: payer_pubkey,
        update_authority: (payer_pubkey, true),
        system_program: solana_sdk::system_program::id(),
        rent: Some(solana_sdk::sysvar::rent::id()),
    }.instruction(CreateMetadataAccountV3InstructionArgs {
        data: metadata_data,
        is_mutable: true,
        collection_details: Some(CollectionDetails::V1 { size: 0 }),
    });

    let create_master_edition_ix = CreateMasterEditionV3 {
        edition: master_edition,
        mint: collection_mint.pubkey(),
        update_authority: payer_pubkey,
        mint_authority: payer_pubkey,
        metadata: metadata_account,
        payer: payer_pubkey,
        token_program: spl_token::id(),
        system_program: solana_sdk::system_program::id(),
        rent: Some(solana_sdk::sysvar::rent::id()),
    }.instruction(CreateMasterEditionV3InstructionArgs {
        max_supply: Some(0),
    });

    let recent_blockhash = client.get_latest_blockhash()
        .map_err(|_| Error::Atom("blockhash_error"))?;

    let message = Message::new_with_blockhash(
        &[create_metadata_ix, create_master_edition_ix],
        Some(&payer_pubkey),
        &recent_blockhash,
    );

    let tx = Transaction::new(
        &[&*signer.keypair, &collection_mint],
        message,
        recent_blockhash,
    );

    client.send_and_confirm_transaction(&tx)
        .map_err(|e| {
            eprintln!("Transaction error: {:?}", e);
            Error::Atom("tx_failed")
        })?;

    let mut result = HashMap::new();
    result.insert("mint".to_string(), collection_mint.pubkey().to_string());
    result.insert("metadata".to_string(), metadata_account.to_string());
    result.insert("master_edition".to_string(), master_edition.to_string());
    Ok(result)
}

#[rustler::nif]
fn create_merkle_tree(
    rpc_url: String,
    signer: SignerHandle,
    config: HashMap<String, i64>,
) -> Result<HashMap<String, String>, Error> {
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let merkle_tree = Keypair::new();
    let payer_pubkey = signer.keypair.pubkey();

    let max_depth = config.get("max_depth").unwrap_or(&14).clone() as u32;
    let max_buffer_size = config.get("max_buffer_size").unwrap_or(&64).clone() as u32;

    let (tree_authority, _) = Pubkey::find_program_address(
        &[merkle_tree.pubkey().as_ref()],
        &mpl_bubblegum::ID,
    );

    let create_tree_ix = CreateTreeConfig {
        tree_creator: payer_pubkey,
        tree_config: tree_authority,
        merkle_tree: merkle_tree.pubkey(),
        payer: payer_pubkey,
        system_program: solana_sdk::system_program::id(),
        log_wrapper: spl_noop::id(),
        compression_program: spl_account_compression::id(),
    }.instruction(CreateTreeConfigInstructionArgs {
        max_depth,
        max_buffer_size,
        public: Some(true),
    });

    let recent_blockhash = client.get_latest_blockhash()
        .map_err(|_| Error::Atom("blockhash_error"))?;

    let message = Message::new_with_blockhash(
        &[create_tree_ix],
        Some(&payer_pubkey),
        &recent_blockhash,
    );

    let tx = Transaction::new(
        &[&*signer.keypair, &merkle_tree],
        message,
        recent_blockhash,
    );

    client.send_and_confirm_transaction(&tx)
        .map_err(|e| {
            eprintln!("Transaction error: {:?}", e);
            Error::Atom("tx_failed")
        })?;

    let mut result = HashMap::new();
    result.insert("address".to_string(), merkle_tree.pubkey().to_string());
    result.insert("tree_authority".to_string(), tree_authority.to_string());
    Ok(result)
}
#[rustler::nif]
fn mint_to_collection(
    rpc_url: String,
    signer: SignerHandle,
    collection_mint: String,
    merkle_tree: String,
    recipients: Vec<HashMap<String, String>>,
    config: HashMap<String, String>,
) -> Result<Vec<String>, Error> {
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let payer_pubkey = signer.keypair.pubkey();
    let collection_pubkey = Pubkey::from_str(&collection_mint)
        .map_err(|_| Error::Atom("invalid_collection"))?;
    let tree_pubkey = Pubkey::from_str(&merkle_tree)
        .map_err(|_| Error::Atom("invalid_tree"))?;

    let (tree_authority, _) = Pubkey::find_program_address(
        &[tree_pubkey.as_ref()],
        &mpl_bubblegum::ID,
    );

    let creators = if let Some(creators_json) = config.get("creators") {
        let creators: Vec<CreatorInput> = serde_json::from_str(creators_json)
            .map_err(|_| Error::Atom("invalid_creators_format"))?;
        
        Some(
            creators
                .iter()
                .map(|c| {
                    Ok(BubblegumCreator {
                        address: Pubkey::from_str(&c.address)
                            .map_err(|_| Error::Atom("invalid_address"))?,
                        verified: c.verified,
                        share: c.share,
                    })
                })
                .collect::<Result<Vec<_>, Error>>()?
        )
    } else {
        None
    };

    let fee_basis_points = config.get("fee_basis_points")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let mut signatures = Vec::new();

    for recipient in recipients {
        let owner_pubkey = Pubkey::from_str(recipient.get("address").unwrap())
            .map_err(|_| Error::Atom("invalid_address"))?;

        let metadata = MetadataArgs {
            name: config.get("item_name").unwrap_or(&"NFT Item".to_string()).clone(),
            symbol: config.get("symbol").unwrap_or(&"".to_string()).clone(),
            uri: config.get("metadata_url").unwrap_or(&"".to_string()).clone(),
            creators: creators.clone().unwrap_or_else(|| Vec::new()),
            seller_fee_basis_points: fee_basis_points,
            primary_sale_happened: false,
            is_mutable: true,
            edition_nonce: None,
            token_standard: Some(TokenStandard::NonFungible),
            collection: Some(mpl_bubblegum::types::Collection {
                verified: false,
                key: collection_pubkey,
            }),
            uses: None,
            token_program_version: TokenProgramVersion::Original,
        };

        let mint_ix = MintToCollectionV1 {
            tree_config: tree_authority,
            leaf_owner: owner_pubkey,
            leaf_delegate: owner_pubkey,
            merkle_tree: tree_pubkey,
            payer: payer_pubkey,
            tree_creator_or_delegate: payer_pubkey,
            collection_authority: payer_pubkey,
            collection_authority_record_pda: None,
            collection_mint: collection_pubkey,
            collection_metadata: Metadata::find_pda(&collection_pubkey).0,
            collection_edition: MasterEdition::find_pda(&collection_pubkey).0,
            bubblegum_signer: Pubkey::find_program_address(&[b"collection_cpi"], &mpl_bubblegum::ID).0,
            token_metadata_program: mpl_token_metadata::ID,
            compression_program: spl_account_compression::id(),
            system_program: solana_sdk::system_program::id(),
            log_wrapper: spl_noop::id(),
        }.instruction(MintToCollectionV1InstructionArgs { metadata });

        let recent_blockhash = client.get_latest_blockhash()
            .map_err(|_| Error::Atom("blockhash_error"))?;

        let message = Message::new_with_blockhash(
            &[mint_ix],
            Some(&payer_pubkey),
            &recent_blockhash,
        );

        let tx = Transaction::new(
            &[&*signer.keypair],
            message,
            recent_blockhash,
        );

        let signature = client.send_and_confirm_transaction(&tx)
            .map(|sig| sig.to_string())
            .map_err(|e| {
                eprintln!("Mint failed: {:?}", e);
                Error::Atom("mint_failed")
            })?;

        signatures.push(signature);
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    Ok(signatures)
}