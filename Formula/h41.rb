class H41 < Formula
  desc "Discover and manage listening TCP ports via a web UI"
  homepage "https://github.com/9h41/h41"
  url "https://github.com/9h41/h41/archive/refs/tags/v0.1.1.tar.gz"
  sha256 "d1c46b79e2f1748e03fc7e8df45d9179216208418bba76a3626ae14a82ee6faf"
  license "MIT"

  def install
    cargo = which("cargo") || Formula["rust"].opt_bin/"cargo"
    system cargo, "install", *std_cargo_args
  end

  test do
    assert_match "h41", shell_output("#{bin}/h41 --version")
  end
end
