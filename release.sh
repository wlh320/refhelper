#!/bin/sh
cargo build --release --target x86_64-unknown-linux-musl
cargo build --release --target x86_64-pc-windows-gnu
cd target/x86_64-unknown-linux-musl/release
strip refhelper
upx -9 refhelper
tar -czvf refhelper_x86_64-unknown-linux-musl.tar.gz refhelper
cp refhelper_x86_64-unknown-linux-musl.tar.gz ../../
cd ../../x86_64-pc-windows-gnu/release
strip refhelper.exe
upx -9 --force refhelper.exe
zip refhelper_x86_64-pc-windows-gnu.zip refhelper.exe
cp refhelper_x86_64-pc-windows-gnu.zip ../../
