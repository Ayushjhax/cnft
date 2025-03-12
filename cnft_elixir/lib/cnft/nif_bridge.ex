# lib/cnft/nif_bridge.ex
defmodule Cnft.NifBridge do
  use Rustler, otp_app: :cnft, crate: "c_nft_nifs"

  def create_collection(_rpc_url, _signer, _config), do: error()
  def create_merkle_tree(_rpc_url, _signer, _config), do: error()
  def mint_to_collection(_rpc_url, _signer, _collection_mint, _merkle_tree, _recipients, _config), do: error()
  
  defp error, do: :erlang.nif_error(:nif_not_loaded)
end