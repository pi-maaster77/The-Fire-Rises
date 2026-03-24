# frontend/build_map.sh

#!/bin/bash
# Compilar Rust a WASM
cd map
wasm-pack build --target web --out-dir ../web-ui/wasm-map

# Limpiar archivos innecesarios que genera wasm-pack
rm ../web-ui/public/wasm-map/.gitignore
rm ../web-ui/public/wasm-map/package.json