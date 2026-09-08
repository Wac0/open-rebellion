#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

VERSION="${1:-dev}"
if [[ ! "${VERSION}" =~ ^[A-Za-z0-9._-]+$ ]]; then
    echo "ERROR: version may contain only letters, numbers, dots, underscores, and hyphens."
    exit 1
fi
echo "Packaging Open Rebellion web build (v${VERSION})..."

# Build WASM
bash "$ROOT/scripts/build-wasm.sh"

# Create distribution directory
DIST="dist/open-rebellion-web-${VERSION}"
rm -rf "${DIST}"
mkdir -p "${DIST}"

# Copy web assets
cp web/index.html "${DIST}/"
cp web/gl.js "${DIST}/"
cp web/open-rebellion.wasm "${DIST}/"

# Runtime data is required by the WASM loading screen. Keep it local to the
# protected distribution; the original game data is intentionally not tracked.
if [ ! -d web/data/base ] || [ ! -d web/data/ui ]; then
    echo "ERROR: scripts/build-wasm.sh did not stage the required web/data payload."
    exit 1
fi
cp -R web/data "${DIST}/"

# Refuse to create an artifact that can compile but cannot boot.
for required_file in \
    "${DIST}/data/base/SECTORSD.DAT" \
    "${DIST}/data/base/SYSTEMSD.DAT" \
    "${DIST}/data/base/CAPSHPSD.DAT" \
    "${DIST}/data/base/FIGHTSD.DAT" \
    "${DIST}/data/base/TROOPSD.DAT" \
    "${DIST}/data/base/MJCHARSD.DAT" \
    "${DIST}/data/base/MNCHARSD.DAT" \
    "${DIST}/data/base/textstra.json" \
    "${DIST}/data/ui/bmp-manifest.json"
do
    if [ ! -f "${required_file}" ]; then
        echo "ERROR: required browser runtime file is missing: ${required_file}"
        exit 1
    fi
done

# Record hashes for every shipped runtime file so release and deployment
# verification can prove which data and code the browser loaded.
(
    cd "${DIST}"
    find . -type f ! -name SHA256SUMS -print \
        | LC_ALL=C sort \
        | while IFS= read -r file; do shasum -a 256 "${file}"; done \
        > SHA256SUMS
)

# Create zip
ZIP="${ROOT}/dist/open-rebellion-web-${VERSION}.zip"
rm -f "${ZIP}"
cd dist
zip -rq "open-rebellion-web-${VERSION}.zip" "open-rebellion-web-${VERSION}/"
cd ..

echo "Created: ${ZIP}"
ls -lh "${ZIP}"
