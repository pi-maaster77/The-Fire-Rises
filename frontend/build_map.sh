# frontend/build_map.sh

#!/bin/bash
# Compilar Rust a WASM
cd map
wasm-pack build --target web --out-dir ../web-ui/src/wasm/

# Limpiar archivos innecesarios que genera wasm-pack
rm ../web-ui/src/wasm/.gitignore
rm ../web-ui/src/wasm/package.json