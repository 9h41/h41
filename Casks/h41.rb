cask "h41" do
  version "0.1.1"

  on_arm do
    sha256 "6fbba77c97335041ac4493fb2c165bfbc83e675dcaed15282f3ae5cce3ede42c"
    url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-arm64.tar.gz"
  end

  on_intel do
    sha256 "2802a4000472293c80a85b31e22d8e47bcf6d1847bcbce1247b48c9bca831c9f"
    url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-x64.tar.gz"
  end

  name "h41"
  desc "Discover and manage listening TCP ports via a web UI"
  homepage "https://github.com/9h41/h41"

  binary "h41"
end
