import dotenv from 'dotenv';
dotenv.config();
import * as fs from 'fs';

import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';

import {
  mplTokenMetadata,
  createNft,
} from '@metaplex-foundation/mpl-token-metadata';

import {
  generateSigner,
  percentAmount,
  createGenericFile,
  signerIdentity,
  createSignerFromKeypair,
} from '@metaplex-foundation/umi';


import {
  COLLECTION_NAME,
  COLLECTION_SYMBOL,
  COLLECTION_DESCRIPTION,
  FEE_PERCENT,
  EXTERNAL_URL,
  METADATA_COLLECTION_URL,
  IMAGE_URL,
} from './config';


import {
  addrToLink
} from './utils';

const rpcURL =
  (process.env.NODE_ENV === 'production'
    ? process.env.SOLANA_MAINNET_RPC_URL
    : process.env.SOLANA_DEVNET_RPC_URL) || 'https://api.devnet.solana.com';

const payerKeyFile    = 'key.json';
const keyData         = fs.readFileSync(payerKeyFile, 'utf8');
const secretKey       = new Uint8Array(JSON.parse(keyData));


const run = async () => {
  try {
    const umi = createUmi(rpcURL)
      .use(mplTokenMetadata())

    const keyPair = umi.eddsa.createKeypairFromSecretKey(secretKey);
    const signer  = createSignerFromKeypair(
      { eddsa: umi.eddsa },
      keyPair
    );

    umi.use(signerIdentity(signer));

    // return;
    const collectionImageUri = IMAGE_URL;
    fs.writeFileSync(
      './data/collectionImageUri.txt',
      collectionImageUri
    );

    const collectionObject = {
      name                   : COLLECTION_NAME,
      symbol                 : COLLECTION_SYMBOL,
      description            : COLLECTION_DESCRIPTION,
      seller_fee_basis_points: FEE_PERCENT * 100,
      image                  : collectionImageUri,
      external_url           : EXTERNAL_URL,
      properties             : {
        category: 'image',
        files: [
          {
            file: collectionImageUri,
            type: 'image/png',
          },
        ],
      },
    };

    const collectionJsonUri = METADATA_COLLECTION_URL;
    console.log('collectionJsonUri:', collectionJsonUri);
    fs.writeFileSync(
      './data/collectionJsonUri.txt',
      collectionJsonUri
    );

    const collectionMint = generateSigner(umi);
    console.log("collectionMint:", collectionMint.publicKey);

    //return;

    await createNft(umi, {
      mint                : collectionMint,
      symbol              : COLLECTION_SYMBOL,
      name                : COLLECTION_NAME,
      uri                 : collectionJsonUri,
      sellerFeeBasisPoints: percentAmount(FEE_PERCENT),
      isCollection        : true,
    }).sendAndConfirm(umi);

    const collectionMintExplolerUrl = `https://explorer.solana.com/address/${
      collectionMint.publicKey
    }${process.env.NODE_ENV !== 'production' && '?cluster=devnet'}`

    console.log('collectionMint:', collectionMintExplolerUrl);

    let cluster = ""; if (process.env.NODE_ENV !== 'production') { cluster = '?cluster=devnet';}
    const txLink = addrToLink( collectionMint.publicKey, cluster);
    console.log(txLink);

    fs.writeFileSync(
      `./data/collectionMint${
        process.env.NODE_ENV === 'production' ? 'Mainnet' : 'Devnet'
      }.txt`,
      collectionMint.publicKey
    );

  } catch (e) {
    console.error(e);
  }
}

void run();