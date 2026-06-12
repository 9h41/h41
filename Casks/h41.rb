cask "h41" do
  version "0.1.0"

  on_arm do
    sha256 "f376e879287fac70bd5b9b1fba4744a701e6b86bde3631a175a1837f2178f0e6"
    url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-arm64.tar.gz"
  end

  on_intel do
    sha256 "c164d25272f0783d779bd549b3889b07fe916e5db939031535ec01361ff57e1f"
    url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-x64.tar.gz"
  end

  name "h41"
  desc "Discover and manage listening TCP ports via a web UI"
  homepage "https://github.com/9h41/h41"

  binary "h41"
end
