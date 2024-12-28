#!/bin/bash
cd ./engine
rm -rf ../assets/engine
wasm-pack build -t web -d ../assets/engine --no-pack --no-typescript --release
rm ../assets/engine/.gitignore
terser ../assets/engine/engine.js > ../assets/engine/engine.min.js
mv -f ../assets/engine/engine.min.js ../assets/engine/engine.js