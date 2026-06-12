class H41 < Formula
  desc "Discover and manage listening TCP ports via a web UI"
  homepage "https://github.com/9h41/h41"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-arm64.tar.gz"
      sha256 "ddf7d1a2ff57e60cbabee61f72757968d4da53d6e287d8b273df4009cee2ad7e"
    else
      url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-x64.tar.gz"
      sha256 "637e3b1ff20c1e6aadbca705b146e2f5bf0fd8e64ed03a4921cc4b14ea9ed23a"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/9h41/h41/releases/download/v#{version}/h41-linux-arm64.tar.gz"
      sha256 "b745af6353b8c491a1487f32f0cbcffc456986ccdf92a26f635e24bf82061e10"
    else
      url "https://github.com/9h41/h41/releases/download/v#{version}/h41-linux-x64.tar.gz"
      sha256 "40ee3141e499187687e507a1fdde430a2f1b480886250f4b401895ac6dc153b5"
    end
  end

  def install
    bin.install "h41"
  end

  test do
    assert_match "h41", shell_output("#{bin}/h41 --version")
  end
end
