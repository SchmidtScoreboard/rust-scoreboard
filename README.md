# Schmidt Scoreboard

## Introduction

This repo contains two parts:

1. The `rust-scoreboard` program, which runs on the Scoreboard, fetches scores,
   displays them, and handles setup and state changes. This program is written
in Rust
2. The `server` directory, which contains all the Python code that fetches
   scores from remote sources, parses, caches, and handles errors. This code
nominally runs inside of an AWS Lambda function. Any pushes to `main` branch
will automatically deploy to AWS.

## Setting up for Development

Building `rust-scoreboard` requires
[Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) and
[rustup](https://rustup.rs)

It requires the gcc cross compiler for RaspberryPi. You can adapt
[this](https://gist.github.com/jmptable/1ae6e6df1ed84aba136f102cd813bd37)
installation for your local development setup.

### Matrix Library

`rust-scoreboard` depends on the
[RGBMatrix](https://github.com/hzeller/rpi-rgb-led-matrix) library. 

In general, I’ve had difficulty building this library with the cross compiler.
I’ve had good luck cloning the repo on my RaspberryPi, building it there, then
copying it back to my development machine.

You must specify the environment variable `MATRIX_LIB` to point to the
`rpi-rgb-led-matrix/lib` directory.

### Installing

Building `rust-scoreboard` relies on the `install.sh` script. This script has
several options:

  -i : Raspberry PI IP address                                                                                                                                              
  -r : Should build release                                                                                                                                                 
  -n : Should run after install                                                                                                                                             
  -e : specify additional enviroment variables

This script will build the binary and deploy if successful.

### Server/AWS

All the code for fetching data from remote sources and caching locally lives in
the `server` directory. A user can run 

```
  python3 server/server.py
```

in order to run a “fake” server and force a scoreboard to connect to it with:

```
  ./install.sh -r -i scoreboard.local -e
‘V2_URL=“http://{DEV_MACHINE_IP}:5000/“
```
  
## Code Overview

Below is a rough overview of the most important parts of the code base. 

* `matrix.rs` contains the main loop of the Scoreboard. In the `run` function,
  the main thread will check a MPSC queue for any commands, execute the
commands, which include displaying an image, disabling or enabling power and
processing settings changes. After processing the command, it will return to
the command queue and wait for another.
* The `Matrix` struct maintains a list of all the available `ScreenProvider`s.
  Any struct that implements this trait will be able to draw an image.
Currently, there are 4 implementors of `ScreenProvider`:
  * `AWSScreen` in `sport.rs` which fetches sport data from AWS, filters, and
    displays it.
  * `SetupScreen` in `setup_screen.rs` which handles displaying setup info and
    animations.
  * `Clock` in `clock.rs`, the simplest screen that displays the current time. 
  * `AnimationTestScreen` in `animation.rs`, a dev only scratchpad screen for
    testing animations.
