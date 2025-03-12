defmodule Cnft.CreateNftCollection do
  alias Cnft.{NifBridge, Utils, Config}

  def execute do
    with {:ok, signer} <- Config.load_signer(),
         {:ok, metadata} <- NifBridge.create_collection(
           Config.rpc_url(),
           signer,
           Config.collection_config()
         ) do
      Utils.save_to_file("collectionMint", metadata["mint"])
      explorer_url = Utils.addr_link(metadata["mint"])
      
      {:ok, %{
        mint: metadata["mint"],
        metadata: metadata["metadata"],
        master_edition: metadata["master_edition"],
        explorer_url: explorer_url
      }}
    else
      error -> 
        IO.puts("Error in CreateNftCollection.execute: #{inspect(error)}")
        error
    end
  end
end