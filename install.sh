#!/usr/bin/env bash

version="latest"
os=$(uname -s | tr '[:upper:]' '[:lower:]')
arch=$(uname -m)
host="NotAJocke"
repo="whisper-keys-engine"
release_url="https://github.com/$host/$repo/releases/"
download_url="$release_url/$version/download/$repo-$os-$arch.zip"

if [ "$os" != "linux" ] && [ "$os" != "darwin" ]; then
	echo "Unsupported operating system: $os."
	exit 1
fi

if [ "$os" == "darwin" ] && [ "$arch" != "arm64" ]; then
	echo "Unsupported architecture: $arch."
	exit 1
fi

echo "Downloading whisper-keys $version for $os-$arch..."

temp_dir=$(mktemp -d -q)

if ! curl -sL "$download_url" -o "$temp_dir/release.zip"; then
	echo "Failed to download the archive."
	exit 1
fi
echo "Archive downloaded successfully."

if ! unzip -q "$temp_dir/release.zip" -d "$temp_dir"; then
	echo "Failed to extract the archive."
	exit 1
fi
echo "Archive extracted successfully."

chmod +x "$temp_dir/$repo-$os-$arch/whisper-keys-engine"

if [ ! -d "/usr/local/bin" ]; then
	sudo mkdir -p /usr/local/bin
fi

if ! sudo mv "$temp_dir/$repo-$os-$arch/whisper-keys-engine" /usr/local/bin/; then
	echo "Failed to move the binary to /usr/local/bin."
	exit 1
fi

rm -rf "$temp_dir"

echo
echo "Installation completed successfully."
