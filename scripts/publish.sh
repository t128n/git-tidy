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

WIN_ARCHIVE="dist/${PACKAGE_NAME}-${VERSION}-windows-x64.zip"
if [[ -f "${WIN_ARCHIVE}" ]]; then
  DESCRIPTION=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].description')
  if command -v sha256sum >/dev/null 2>&1; then
    WIN_SHA=$(sha256sum "${WIN_ARCHIVE}" | awk '{print toupper($1)}')
  elif command -v shasum >/dev/null 2>&1; then
    WIN_SHA=$(shasum -a 256 "${WIN_ARCHIVE}" | awk '{print toupper($1)}')
  else
    WIN_SHA=""
  fi

  if [[ -n "${WIN_SHA}" ]]; then
    cat <<EOF > "dist/winget.yaml"
# yaml-language-server: \$schema=https://aka.ms/winget-manifest.singleton.1.9.0.schema.json
PackageIdentifier: t128n.${PACKAGE_NAME}
PackageVersion: ${VERSION}
PackageName: ${PACKAGE_NAME}
Publisher: t128n
License: MIT
ShortDescription: ${DESCRIPTION}
PackageLocale: en-US
ManifestType: singleton
ManifestVersion: 1.9.0
InstallerType: zip
NestedInstallerType: portable
NestedInstallerFiles:
  - RelativeFilePath: ${PACKAGE_NAME}.exe
    PortableCommandAlias: ${PACKAGE_NAME}
Installers:
  - Architecture: x64
    InstallerUrl: https://github.com/t128n/${PACKAGE_NAME}/releases/download/v${VERSION}/${PACKAGE_NAME}-${VERSION}-windows-x64.zip
    InstallerSha256: ${WIN_SHA}
EOF
    echo "Generated dist/winget.yaml"
  fi
fi

echo "Build complete. Artifacts ready in dist/:"
ls -lh dist/

bunx changeset publish


