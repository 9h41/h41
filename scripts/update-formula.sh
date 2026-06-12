#!/usr/bin/env bash
set -euo pipefail

# Usage: ./scripts/update-formula.sh v0.1.0
# Downloads release artifacts and updates Formula/h41.rb with correct version and SHA256 hashes.

VERSION="${1#v}"
TAG="v${VERSION}"
REPO="9h41/h41"
FORMULA="Formula/h41.rb"

echo "Updating formula for ${TAG}..."

declare -A FILES=(
  [MACOS_ARM64]="h41-macos-arm64.tar.gz"
  [MACOS_X64]="h41-macos-x64.tar.gz"
  [LINUX_ARM64]="h41-linux-arm64.tar.gz"
  [LINUX_X64]="h41-linux-x64.tar.gz"
)

for key in "${!FILES[@]}"; do
  file="${FILES[$key]}"
  url="https://github.com/${REPO}/releases/download/${TAG}/${file}"
  echo "  Fetching SHA256 for ${file}..."
  sha=$(curl -sL "$url" | shasum -a 256 | cut -d' ' -f1)
  sed -i '' "s/\${SHA256_${key}}/${sha}/" "$FORMULA" 2>/dev/null || \
    sed -i "s/\${SHA256_${key}}/${sha}/" "$FORMULA"
done

sed -i '' "s/\${VERSION}/${VERSION}/" "$FORMULA" 2>/dev/null || \
  sed -i "s/\${VERSION}/${VERSION}/" "$FORMULA"

echo "Done. Formula updated for ${VERSION}."
