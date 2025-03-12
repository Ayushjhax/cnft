import Config

if System.get_env("NODE_ENV") == "production" do
  config :cnft,
    solana_rpc: System.fetch_env!("SOLANA_MAINNET_RPC_URL")
else
  config :cnft,
    solana_rpc: System.get_env("SOLANA_DEVNET_RPC_URL", "https://api.devnet.solana.com")
end