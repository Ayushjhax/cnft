import Config

config :cnft,
  ecto_repos: [],
  env: Mix.env()

config :rustler, :modules, [
  cnft_nifs: [
    path: "native/c_nft_nifs",
    mode: if(Mix.env() == :prod, do: :release, else: :debug),
  ]
]