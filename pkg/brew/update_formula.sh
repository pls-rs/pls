#!/usr/bin/env bash
set -euo pipefail

REPO="pls-rs/pls"
RELEASE=$(curl -fsSL "https://api.github.com/repos/$REPO/releases" | jq -r '.[0]')

# Abort early if the API returned no release data instead of propagating bad values downstream.
if [[ -z "$RELEASE" || "$RELEASE" == "null" ]]; then
	echo "error: GitHub API returned no releases for $REPO" >&2
	exit 1
fi

VERSION=$(echo "$RELEASE" | jq -r '.name' | cut -c 2-)
if [[ -z "$VERSION" ]]; then
	echo "error: could not determine version from release data" >&2
	exit 1
fi
echo "Latest release is $VERSION."

asset_url() {
	local triple="$1"
	local name="pls-${triple}.zip"
	echo "$RELEASE" | jq -r --arg name "$name" '.assets[] | select(.name == $name) | .browser_download_url'
}

sha_for() {
	local url="$1"
	local dest="$2"
	curl -fsSL "$url" -o "$dest"
	shasum -a 256 "$dest" | awk '{ print $1 }'
}

MAC_ARM_URL=$(asset_url "aarch64-apple-darwin")
if [[ -z "$MAC_ARM_URL" ]]; then
	echo "error: missing asset pls-aarch64-apple-darwin.zip in release" >&2
	exit 1
fi
MAC_ARM_SHA=$(sha_for "$MAC_ARM_URL" /tmp/mac_arm_asset)
echo "macOS arm64 asset: $MAC_ARM_URL ($MAC_ARM_SHA)."

MAC_X86_URL=$(asset_url "x86_64-apple-darwin")
if [[ -z "$MAC_X86_URL" ]]; then
	echo "error: missing asset pls-x86_64-apple-darwin.zip in release" >&2
	exit 1
fi
MAC_X86_SHA=$(sha_for "$MAC_X86_URL" /tmp/mac_x86_asset)
echo "macOS x86_64 asset: $MAC_X86_URL ($MAC_X86_SHA)."

LINUX_X86_URL=$(asset_url "x86_64-unknown-linux-musl")
if [[ -z "$LINUX_X86_URL" ]]; then
	echo "error: missing asset pls-x86_64-unknown-linux-musl.zip in release" >&2
	exit 1
fi
LINUX_X86_SHA=$(sha_for "$LINUX_X86_URL" /tmp/linux_x86_asset)
echo "Linux x86_64 asset: $LINUX_X86_URL ($LINUX_X86_SHA)."

LINUX_ARM_URL=$(asset_url "aarch64-unknown-linux-musl")
if [[ -z "$LINUX_ARM_URL" ]]; then
	echo "error: missing asset pls-aarch64-unknown-linux-musl.zip in release" >&2
	exit 1
fi
LINUX_ARM_SHA=$(sha_for "$LINUX_ARM_URL" /tmp/linux_arm_asset)
echo "Linux aarch64 asset: $LINUX_ARM_URL ($LINUX_ARM_SHA)."

sed -e "s|{{ VERSION }}|$VERSION|g" \
	-e "s|{{ MAC_ARM_URL }}|$MAC_ARM_URL|g" \
	-e "s|{{ MAC_ARM_SHA }}|$MAC_ARM_SHA|g" \
	-e "s|{{ MAC_X86_URL }}|$MAC_X86_URL|g" \
	-e "s|{{ MAC_X86_SHA }}|$MAC_X86_SHA|g" \
	-e "s|{{ LINUX_X86_URL }}|$LINUX_X86_URL|g" \
	-e "s|{{ LINUX_X86_SHA }}|$LINUX_X86_SHA|g" \
	-e "s|{{ LINUX_ARM_URL }}|$LINUX_ARM_URL|g" \
	-e "s|{{ LINUX_ARM_SHA }}|$LINUX_ARM_SHA|g" "$1" > "$2"

echo "Formula written!"
