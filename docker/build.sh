#!/bin/sh
cd `dirname $0`/../
cargo build --release
cp target/release/imagecleanup docker
cd docker
sudo docker build -t willwill/imagecleanup .
