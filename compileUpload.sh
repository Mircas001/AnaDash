!#/bin/bash

echo "Welcome!"
echo "Make sure you connected the Raspberry Pico to your computer and it is in BOOTSEL mode."
cd ./drivers/
cargo build --release