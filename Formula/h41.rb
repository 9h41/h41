class H41 < Formula
  desc "Discover and manage listening TCP ports via a web UI"
  homepage "https://github.com/9h41/h41"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-arm64.tar.gz"
      sha256 "17376c301280bc3cd81471ccf90868ad96c2b2bc0f6572d67e09235acdea5031"
    else
      url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-x64.tar.gz"
      sha256 "70463e2c031115345e6424d4fe948602ce710f2dc0ca062b79d6eeea26a55a75"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/9h41/h41/releases/download/v#{version}/h41-linux-arm64.tar.gz"
      sha256 "e8d1560851fb3bbf0febf6e349b7e77dbcfe8089f1e264ca463143c6cd6f29c7"
    else
      url "https://github.com/9h41/h41/releases/download/v#{version}/h41-linux-x64.tar.gz"
      sha256 "47db34e7b24ad59370ceb981e3e2c6e9cade5e17faa4116ecdc3059abc1317da"
    end
  end

  def install
    bin.install "h41"
  end

  test do
    assert_match "h41", shell_output("#{bin}/h41 --version")
  end
end
