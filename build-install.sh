#!/bin/bash

cargo build --release
cp ./target/release/cgpt ~/.local/bin/
