#!/bin/bash


PI_IP=$1
TARGET=armv7-unknown-linux-gnueabihf

# build binary
cargo build --target $TARGET

if [ $? -ne 0 ]; then
    exit 1
fi

if [[ $# -eq 0 ]] ; then
    echo 'No Pi ip address specified, run ./install.sh {IP_ADDRESS} to deploy and run'
    exit 0
fi

trap ctrl_c INT
function ctrl_c() {
        echo "Exiting"
        ssh pi@$PI_IP 'sudo pkill scoreboard'
}

ssh pi@$PI_IP 'sudo pkill scoreboard-rust'
rsync -avzhe ssh target/$TARGET/debug/scoreboard-rust pi@$PI_IP:/home/pi/rust-scoreboard/
ssh pi@$PI_IP 'sudo RUST_LOG="debug, rocket= error" ./rust-scoreboard/scoreboard-rust --skip_update'
