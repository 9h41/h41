cask "h41" do
  version "0.1.2"

  on_arm do
    sha256 "0e1943c9a09d1779af9918b991152e1f51f0401609dc101b31bd57f215f56b10"
    url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-arm64.tar.gz"
  end

  on_intel do
    sha256 "2fe11212fb25f6ce647640675f2fa970bba3ee62cc19c97b6ff45540681647ab"
    url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-x64.tar.gz"
  end

  name "h41"
  desc "Discover and manage listening TCP ports via a web UI"
  homepage "https://github.com/9h41/h41"

  binary "h41"
end
