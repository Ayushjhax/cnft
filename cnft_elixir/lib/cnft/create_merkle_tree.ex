# create_merkle_tree.ex
defmodule Cnft.CreateMerkleTree do
  @moduledoc """
  Handles Merkle tree creation using Rust NIFs
  """
  alias Cnft.{NifBridge, Utils, Config}

  def execute do
    with {:ok, signer} <- Config.load_signer(),
         {:ok, metadata} <- NifBridge.create_merkle_tree(
           Config.rpc_url(),
           signer,
           Config.merkle_config()
         ) do
      Utils.save_to_file("merkleTree", metadata["address"])
      explorer_url = Utils.addr_link(metadata["address"])
      
      {:ok, %{
        address: metadata["address"],
        tree_authority: metadata["tree_authority"],
        explorer_url: explorer_url
      }}
    end
  end
end