#!/bin/bash
wasm-pack build
cd www
sh run.sh
cd ..
