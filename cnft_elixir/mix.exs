# mix.exs
defmodule Cnft.MixProject do
  use Mix.Project

  def project do
    [
      app: :cnft,
      version: "0.1.0",
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      compilers: Mix.compilers(),
      deps: deps(),
      package: package()
    ]
  end

  defp package do
    [
      files: ["lib", "native", "mix.exs", "README.md"],
      licenses: ["MIT"],
      links: %{"GitHub" => "https://github.com/yourname/cnft_elixir"}
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.23.0"},
      {:jason, "~> 1.4"},
      {:nimble_csv, "~> 1.2"},
      {:req, "~> 0.4.0"},
      {:b58, "~> 0.1.0"}  # Verified working version
    ]
  end
end