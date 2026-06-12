cask "h41" do
  version "0.1.0"

  on_arm do
    sha256 "17376c301280bc3cd81471ccf90868ad96c2b2bc0f6572d67e09235acdea5031"
    url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-arm64.tar.gz"
  end

  on_intel do
    sha256 "70463e2c031115345e6424d4fe948602ce710f2dc0ca062b79d6eeea26a55a75"
    url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-x64.tar.gz"
  end

  name "h41"
  desc "Discover and manage listening TCP ports via a web UI"
  homepage "https://github.com/9h41/h41"

  binary "h41"
end
