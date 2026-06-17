cask "h41" do
  version "0.1.2"

  on_arm do
    sha256 "5bbd01d092911834bb8d497db95e7e8e4a543c4d05071c72ff2a8a2d016e34b8"
    url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-arm64.tar.gz"
  end

  on_intel do
    sha256 "eb97cc97510f5fb7548cce92335f2ae04f8c899df48d567ba834904fa6968e73"
    url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-x64.tar.gz"
  end

  name "h41"
  desc "Discover and manage listening TCP ports via a web UI"
  homepage "https://github.com/9h41/h41"

  binary "h41"
end
