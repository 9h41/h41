class H41 < Formula
  desc "Discover and manage listening TCP ports via a web UI"
  homepage "https://github.com/9h41/h41"
  version "${VERSION}"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-arm64.tar.gz"
      sha256 "${SHA256_MACOS_ARM64}"
    else
      url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-x64.tar.gz"
      sha256 "${SHA256_MACOS_X64}"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/9h41/h41/releases/download/v#{version}/h41-linux-arm64.tar.gz"
      sha256 "${SHA256_LINUX_ARM64}"
    else
      url "https://github.com/9h41/h41/releases/download/v#{version}/h41-linux-x64.tar.gz"
      sha256 "${SHA256_LINUX_X64}"
    end
  end

  def install
    bin.install "h41"
  end

  test do
    assert_match "h41", shell_output("#{bin}/h41 --version")
  end
end
