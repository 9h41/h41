class H41 < Formula
  desc "Discover and manage listening TCP ports via a web UI"
  homepage "https://github.com/9h41/h41"
  url "https://github.com/9h41/h41/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "8e0a5f851d2876e24ee4a0df4551b31ddb9d86847ed2b60dfa55500b1131f44b"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "h41", shell_output("#{bin}/h41 --version")
  end
end
