cask "h41" do
  version "0.1.0"

  on_arm do
    sha256 "ddf7d1a2ff57e60cbabee61f72757968d4da53d6e287d8b273df4009cee2ad7e"
    url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-arm64.tar.gz"
  end

  on_intel do
    sha256 "637e3b1ff20c1e6aadbca705b146e2f5bf0fd8e64ed03a4921cc4b14ea9ed23a"
    url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-x64.tar.gz"
  end

  name "h41"
  desc "Discover and manage listening TCP ports via a web UI"
  homepage "https://github.com/9h41/h41"

  binary "h41"
end
