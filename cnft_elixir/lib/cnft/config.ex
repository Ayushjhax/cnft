# lib/cnft/config.ex
defmodule Cnft.Config do
  @moduledoc """
  Configuration management for CNFT operations
  """
  
  def production?, do: System.get_env("NODE_ENV") == "production"
  
  def rpc_url do
    if production?(), do: System.get_env("SOLANA_MAINNET_RPC_URL"),
    else: System.get_env("SOLANA_DEVNET_RPC_URL", "https://api.devnet.solana.com")
  end

  def collection_config do
    %{
      name: "100xDevs Collection",
      symbol: "100xDevs",
      metadata_url: "https://gist.githubusercontent.com/Ayushjhax/5eb6c0cb31e506d68ff0418dc704669a/raw/72b62a4f6fdb65b56f1ec47c4edd99a16a7f4979/cnft_metadata.json",
      fee_basis_points: 0,
      creators: [
        %{
          address: "ayEwtx4SbkdoyF7i7rA3Ygq7qkVsaJTr3JhQocdibn7",
          verified: false,
          share: 100
        }
      ]
    }
  end

  def merkle_config do
    %{
      max_depth: 14,
      max_buffer_size: 64
    }
  end

  def mint_config do
    %{
      item_name: "Ayush Limited Edition",
      metadata_url: "https://gist.githubusercontent.com/Ayushjhax/fd65289fa8ab637d4fa1b5f9226334c7/raw/c3d1c438b90b78a4447b545c4cc4cbd55ad3637d/cnft_item_metadata.json",
      fee_basis_points: 0,
      creators: [
        %{
          address: "ayEwtx4SbkdoyF7i7rA3Ygq7qkVsaJTr3JhQocdibn7",
          verified: false,
          share: 100
        }
      ]
    }
  end

  def load_keypair do
    key_file = "ayEwtx4SbkdoyF7i7rA3Ygq7qkVsaJTr3JhQocdibn7.json"
    with {:ok, key_data} <- File.read(key_file),
         {:ok, decoded} <- parse_key_data(key_data) do
      {:ok, decoded}
    else
      error -> {:error, "Failed to load keypair: #{inspect(error)}"}
    end
  end

  defp parse_key_data(data) do
    data
    |> String.trim()
    |> Jason.decode!()
    |> case do
      list when is_list(list) -> {:ok, :binary.list_to_bin(list)}
      str when is_binary(str) -> B58.decode58(str)
      _ -> {:error, "Invalid key format"}
    end
  end
end