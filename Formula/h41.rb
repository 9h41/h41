class H41 < Formula
  desc "Discover and manage listening TCP ports via a web UI"
  homepage "https://github.com/9h41/h41"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-arm64.tar.gz"
      sha256 "e023bef58ea5847b101960e6074d16f796ab82fa9882d2cd49be4372c6be90de"
    else
      url "https://github.com/9h41/h41/releases/download/v#{version}/h41-macos-x64.tar.gz"
      sha256 "59740e18781760e21356e749db85ec2c788fa0872fd0ebcd4e5659e81e3c4b4a"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/9h41/h41/releases/download/v#{version}/h41-linux-arm64.tar.gz"
      sha256 "8e6ac8a5a7a64627fcaa9afa12a85860688972dbe3c3f10d0cc24a1af10ed002"
    else
      url "https://github.com/9h41/h41/releases/download/v#{version}/h41-linux-x64.tar.gz"
      sha256 "ea59e7f3fef151de038c479f3836ebadad5b250add90a7ef5fc49d91020fb0fb"
    end
  end

  def install
    bin.install "h41"
  end

  test do
    assert_match "h41", shell_output("#{bin}/h41 --version")
  end
end
