#!/usr/bin/env bash

cargo clean 

rm -rf target Cargo.lock

PLATFORM=$1

if [ "$PLATFORM" == "android" ]; then
    export CC_aarch64_linux_android=~/Tools/android-ndk-r24/AndroidNDK8215888.app/Contents/NDK/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android21-clang

    export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=$CC_aarch64_linux_android

    cargo build --release --target aarch64-linux-android

    adb shell rm /data/local/tmp/libRustAndroid.so

    adb shell rm /data/local/tmp/RustAndroid

    adb push ./target/aarch64-linux-android/release/RustAndroid /data/local/tmp/

    adb push ./target/aarch64-linux-android/release/*.so /data/local/tmp/

    bin_path="/data/local/tmp"

    adb shell "chmod +x ${bin_path}/RustAndroid"

    adb shell "cd ${bin_path} \
            && export LD_LIBRARY_PATH=${bin_path}:${LD_LIBRARY_PATH} \
            && ./RustAndroid "

elif [ "$PLATFORM" == "macos" ]; then
    
    cargo build --release
    
    cargo run --release
else
    echo "Unknown platform: $PLATFORM"
fi




