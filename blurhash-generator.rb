class BlurhashGenerator < Formula
  desc "High-performance Rust tool for generating BlurHash strings from images"
  homepage "https://github.com/seoulcomix/blurhash-generator"
  url "https://github.com/seoulcomix/blurhash-generator/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "" # This will need to be filled in after creating a release
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/blurhash-generator", "--help"
  end
end 