cask "h41" do
  version "0.1.3"

  on_arm do
    sha256 "8fe929aa861f0e460d4383fb02c13014ff95096a6135c480247e0a06c937143e"
    url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-arm64.tar.gz"
  end

  on_intel do
    sha256 "40cde2a79634efb21d3dcf56ea8f6c617308bc1674106b528af648ae9cc74842"
    url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-x64.tar.gz"
  end

  name "h41"
  desc "Discover and manage listening TCP ports via a web UI"
  homepage "https://github.com/9h41/h41"

  binary "h41"
end
