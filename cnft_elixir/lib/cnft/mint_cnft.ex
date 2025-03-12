# mint_cnft.ex
defmodule Cnft.MintCnft do
  @moduledoc """
  Handles cNFT minting operations
  """
  alias Cnft.{NifBridge, Utils, Config}

  def execute do
    with {:ok, signer} <- Config.load_signer(),
         recipients <- Utils.read_csv(),
         {:ok, collection_mint} <- Utils.read_from_file("collectionMint"),
         {:ok, merkle_tree} <- Utils.read_from_file("merkleTree"),
         {:ok, signatures} <- NifBridge.mint_to_collection(
           Config.rpc_url(),
           signer,
           collection_mint,
           merkle_tree,
           recipients,
           Config.mint_config()
         ) do
      tx_links = Enum.map(signatures, &Utils.tx_link/1)
      {:ok, tx_links}
    else
      error -> handle_error(error)
    end
  end

  defp handle_error(error) do
    IO.puts("Minting failed: #{inspect(error)}")
    {:error, error}
  end
end