# lib/cnft/utils.ex
defmodule Cnft.Utils do
  def save_to_file(prefix, content) do
    env = if Config.production?(), do: "Mainnet", else: "Devnet"
    path = "data/#{prefix}_#{env}.txt"
    File.mkdir_p!("data")
    File.write!(path, Jason.encode!(content))
  end

  def read_from_file(prefix) do
    env = if Config.production?(), do: "Mainnet", else: "Devnet"
    path = "data/#{prefix}_#{env}.txt"
    case File.read(path) do
      {:ok, content} -> Jason.decode(content)
      error -> error
    end
  end

  def read_csv(path) do
    path
    |> File.stream!()
    |> NimbleCSV.RFC4180.parse_stream()
    |> Enum.map(fn [address] -> %{address: address} end)
  end

  def tx_link(signature) do
    base = "https://explorer.solana.com/tx/#{signature}"
    if Config.production?(), do: base, else: "#{base}?cluster=devnet"
  end
end