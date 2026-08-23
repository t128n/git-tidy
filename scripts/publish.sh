#!/usr/bin/env bash
set -euo pipefail

PACKAGE_NAME=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].name')
VERSION=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')

echo "Building release artifacts for ${PACKAGE_NAME} v${VERSION}..."

TARGETS=(
  "x86_64-unknown-linux-gnu:tar.gz:linux-x64"
  "aarch64-unknown-linux-gnu:tar.gz:linux-arm64"
  "x86_64-pc-windows-gnu:zip:windows-x64"
)

mkdir -p dist

for entry in "${TARGETS[@]}"; do
  IFS=":" read -r target format label <<< "${entry}"
  echo "==> Building ${PACKAGE_NAME} for ${target} (${label})..."

  cross build --release --target "${target}" --package "${PACKAGE_NAME}"

  if [[ "${format}" == "zip" ]]; then
    BINARY="target/${target}/release/${PACKAGE_NAME}.exe"
    ARCHIVE="dist/${PACKAGE_NAME}-${VERSION}-${label}.zip"
    if [[ -f "${BINARY}" ]]; then
      zip -j "${ARCHIVE}" "${BINARY}"
      echo "Created ${ARCHIVE}"
    else
      echo "Error: Binary not found at ${BINARY}" >&2
      exit 1
    fi
  else
    BINARY="target/${target}/release/${PACKAGE_NAME}"
    ARCHIVE="dist/${PACKAGE_NAME}-${VERSION}-${label}.tar.gz"
    if [[ -f "${BINARY}" ]]; then
      tar -czf "${ARCHIVE}" -C "target/${target}/release" "${PACKAGE_NAME}"
      echo "Created ${ARCHIVE}"
    else
      echo "Error: Binary not found at ${BINARY}" >&2
      exit 1
    fi
  fi
done

echo "Build complete. Artifacts ready in dist/:"
ls -lh dist/

bunx changeset publish

