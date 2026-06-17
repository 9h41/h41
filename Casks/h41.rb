cask "h41" do
  version "0.1.3"

  on_arm do
    sha256 "cdccc6b73265bb3b14e579d4c08164a7a87fd4708262cc62308878f44f427ac4"
    url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-arm64.tar.gz"
  end

  on_intel do
    sha256 "ca0fb246c2487f674fb1d21b6a681c99babdb460b8aed9df2f345bd9a0b61ec4"
    url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-x64.tar.gz"
  end

  name "h41"
  desc "Discover and manage listening TCP ports via a web UI"
  homepage "https://github.com/9h41/h41"

  binary "h41"
end
