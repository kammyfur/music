#!/bin/bash
cd ./engine
rm -rf ../dist
wasm-pack build -t web -d ../dist --no-pack --no-typescript --release
rm ../dist/.gitignore
terser ../dist/engine.js > ../dist/engine.min.js
mv -f ../dist/engine.min.js ../dist/engine.js