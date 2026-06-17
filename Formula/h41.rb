class H41 < Formula
  desc "Discover and manage listening TCP ports via a web UI"
  homepage "https://github.com/9h41/h41"
  url "https://github.com/9h41/h41/archive/refs/tags/v0.1.2.tar.gz"
  sha256 "ef94877303741c9f0106f37e1e69fe6cb592cd5674f5bd436ebae9c0f32500f2"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "h41", shell_output("#{bin}/h41 --version")
  end
end
