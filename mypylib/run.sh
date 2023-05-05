#!/bin/bash
cargo build
cp ../target/debug/libmypylib.so mypylib.so
python3 test.py
