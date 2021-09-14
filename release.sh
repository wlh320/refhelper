#!/bin/sh
cargo build --release --target x86_64-unknown-linux-musl
cargo build --release --target x86_64-pc-windows-gnu
cd target/x86_64-unknown-linux-musl/release
tar -czvf refhelper_x86_64-unknown-linux-musl.tar.gz refhelper
cp refhelper_x86_64-unknown-linux-musl.tar.gz ../../
cd ../../x86_64-pc-windows-gnu/release
zip refhelper_x86_64-pc-windows-gnu.zip refhelper.exe
cp refhelper_x86_64-pc-windows-gnu.zip ../../
