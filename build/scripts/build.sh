#!/bin/bash

echo "Building decentralized DNS + DHCP system..."

cd ..

cargo build --release

echo "Build complete."
