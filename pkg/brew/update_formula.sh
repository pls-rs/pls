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

LINUX_URL=$(echo "$RELEASE" | jq -r '.assets[] | select(.name | contains("unknown-linux-musl")).browser_download_url')
echo "Downloading Linux asset from $LINUX_URL."

curl -sL "$LINUX_URL" -o /tmp/linux_asset
LINUX_SHA=$(shasum -a 256 /tmp/linux_asset | awk '{ print $1 }')
echo "SHA256 for Linux asset is $LINUX_SHA."

sed -e "s|{{ VERSION }}|$VERSION|g" \
    -e "s|{{ MAC_URL }}|$MAC_URL|g" \
    -e "s|{{ MAC_SHA }}|$MAC_SHA|g" \
    -e "s|{{ LINUX_URL }}|$LINUX_URL|g" \
    -e "s|{{ LINUX_SHA }}|$LINUX_SHA|g" "$1" > "$2"

echo "Formula written!"
