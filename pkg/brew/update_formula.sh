#!/usr/bin/env bash

REPO="pls-rs/pls"
RELEASE=$(curl -s "https://api.github.com/repos/$REPO/releases" | jq -r '.[0]')

VERSION=$(echo "$RELEASE" | jq -r '.name' | cut -c 2-)
echo "Latest release is $VERSION."

MAC_URL=$(echo "$RELEASE" | jq -r '.assets[] | select(.name | contains("apple-darwin")).browser_download_url')
echo "Downloading macOS asset from $MAC_URL."

curl -sL "$MAC_URL" -o /tmp/mac_asset
MAC_SHA=$(shasum -a 256 /tmp/mac_asset | awk '{ print $1 }')
echo "SHA256 for macOS asset is $MAC_SHA."

LINUX_X86_URL=$(echo "$RELEASE" | jq -r '.assets[] | select(.name | contains("x86_64-unknown-linux-musl")).browser_download_url')
echo "Downloading Linux x86_64 asset from $LINUX_X86_URL."

curl -sL "$LINUX_X86_URL" -o /tmp/linux_x86_asset
LINUX_X86_SHA=$(shasum -a 256 /tmp/linux_x86_asset | awk '{ print $1 }')
echo "SHA256 for Linux x86_64 asset is $LINUX_X86_SHA."

LINUX_ARM_URL=$(echo "$RELEASE" | jq -r '.assets[] | select(.name | contains("aarch64-unknown-linux-musl")).browser_download_url')
echo "Downloading Linux aarch64 asset from $LINUX_ARM_URL."

curl -sL "$LINUX_ARM_URL" -o /tmp/linux_arm_asset
LINUX_ARM_SHA=$(shasum -a 256 /tmp/linux_arm_asset | awk '{ print $1 }')
echo "SHA256 for Linux aarch64 asset is $LINUX_ARM_SHA."

sed -e "s|{{ VERSION }}|$VERSION|g" \
    -e "s|{{ MAC_URL }}|$MAC_URL|g" \
    -e "s|{{ MAC_SHA }}|$MAC_SHA|g" \
    -e "s|{{ LINUX_X86_URL }}|$LINUX_X86_URL|g" \
    -e "s|{{ LINUX_X86_SHA }}|$LINUX_X86_SHA|g" \
    -e "s|{{ LINUX_ARM_URL }}|$LINUX_ARM_URL|g" \
    -e "s|{{ LINUX_ARM_SHA }}|$LINUX_ARM_SHA|g" "$1" > "$2"

echo "Formula written!"
