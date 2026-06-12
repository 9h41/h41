#!/usr/bin/env bash
set -euo pipefail

# Usage: ./scripts/update-formula.sh v0.1.0
# Downloads release artifacts and generates Formula/h41.rb from the template.

VERSION="${1#v}"
TAG="v${VERSION}"
REPO="9h41/h41"
TEMPLATE="Formula/h41.rb.template"
FORMULA="Formula/h41.rb"

echo "Updating formula for ${TAG}..."

cp "$TEMPLATE" "$FORMULA"

for pair in \
  "SHA256_MACOS_ARM64:h41-macos-arm64.tar.gz" \
  "SHA256_MACOS_X64:h41-macos-x64.tar.gz" \
  "SHA256_LINUX_ARM64:h41-linux-arm64.tar.gz" \
  "SHA256_LINUX_X64:h41-linux-x64.tar.gz"; do

  key="${pair%%:*}"
  file="${pair#*:}"
  url="https://github.com/${REPO}/releases/download/${TAG}/${file}"
  echo "  Fetching SHA256 for ${file}..."
  sha=$(curl -sL "$url" | shasum -a 256 | cut -d' ' -f1)
  sed -i.bak "s/\${${key}}/${sha}/" "$FORMULA"
done

sed -i.bak "s/\${VERSION}/${VERSION}/" "$FORMULA"
rm -f "${FORMULA}.bak"

echo "Done. Formula updated for ${VERSION}."
