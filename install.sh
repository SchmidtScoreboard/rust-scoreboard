#!/bin/bash
# A POSIX variable
OPTIND=1         # Reset in case getopts has been used previously in the shell.

# Initialize our own variables:
release=1
should_run=0
build_args=""
environment=""

while getopts "h?i:e:rn" opt; do
    case "$opt" in
    h|\?)
        echo "-i : Raspberry PI IP address"
        echo "-r : Should build release"
        echo "-n : Should run after install"
        echo "-e : specify additional enviroment variables"
        exit 0
        ;;
    i)  pi_ip=$OPTARG
        ;;
    r)  release=0
        build_args="--release"
        ;;
    n)  should_run=1
        ;;
    e)  environment=$OPTARG 
        ;;
    esac
done

shift $((OPTIND-1))

[ "${1:-}" = "--" ] && shift


TARGET=armv7-unknown-linux-gnueabihf

# build binary
cargo build --target $TARGET $build_args

if [ $? -ne 0 ]; then
    exit 1
fi

if test -z "$pi_ip" 
then
    echo 'No Pi ip address specified, run ./install.sh -i {IP_ADDRESS} to deploy and run'
    exit 0
fi

trap ctrl_c INT
function ctrl_c() {
        echo "Exiting"
        ssh pi@$pi_ip 'sudo pkill scoreboard'
}

ssh pi@$pi_ip 'sudo systemctl stop scoreboard.service; sudo pkill scoreboard-rust'
if [[ $release -eq 0 ]] ; then
    echo "Sending release build"
    rsync -avzhe ssh target/$TARGET/release/scoreboard pi@$pi_ip:/var/lib/scoreboard/
else
    echo "Sending debug build"
    rsync -avzhe ssh target/$TARGET/debug/scoreboard pi@$pi_ip:/var/lib/scoreboard/
fi

if [[ $should_run -eq 0 ]] ; then
    echo "Running with environment ${environment}"
    ssh pi@$pi_ip "sudo RUST_BACKTRACE=1 RUST_LOG=\"debug\" ${environment} /var/lib/scoreboard/scoreboard --skip_update"
fi
