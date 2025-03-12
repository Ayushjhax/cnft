# lib/cnft.ex
defmodule Cnft do
  alias Cnft.{Config, NifBridge, Utils}

  def create_collection do
    with {:ok, keypair} <- Config.load_keypair(),
         {:ok, signer} <- NifBridge.create_signer(keypair),
         config = Config.collection_config(),
         {:ok, metadata} <- NifBridge.create_collection(Config.rpc_url(), signer, config) do
      Utils.save_to_file("collection", metadata)
      {:ok, metadata}
    end
  end

  def create_merkle_tree do
    with {:ok, keypair} <- Config.load_keypair(),
         {:ok, signer} <- NifBridge.create_signer(keypair),
         config = Config.merkle_config(),
         {:ok, tree} <- NifBridge.create_merkle_tree(Config.rpc_url(), signer, config) do
      Utils.save_to_file("merkle_tree", tree)
      {:ok, tree}
    end
  end

  def mint_cnft do
    with {:ok, keypair} <- Config.load_keypair(),
         {:ok, signer} <- NifBridge.create_signer(keypair),
         config = Config.mint_config(),
         recipients <- Utils.read_csv("addresses.csv"),
         {:ok, collection} <- Utils.read_from_file("collection"),
         {:ok, merkle_tree} <- Utils.read_from_file("merkle_tree") do
      NifBridge.mint_to_collection(
        Config.rpc_url(),
        signer,
        collection,
        merkle_tree,
        recipients,
        config
      )
    end
  end
end