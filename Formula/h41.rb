class H41 < Formula
  desc "Discover and manage listening TCP ports via a web UI"
  homepage "https://github.com/9h41/h41"
  url "https://github.com/9h41/h41/archive/refs/tags/v0.1.3.tar.gz"
  sha256 "26fa2880ddc3a2ac8229444a54aa49c0133ab78064c5ee7858c9b11eb1ea3a04"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "h41", shell_output("#{bin}/h41 --version")
  end
end
