# Watify

## Requirements

To build, a working rust toolchain and `wasm-pack` are needed.

## Build

```bash
# Optional: install wasm-pack if not already installed
cargo install wasm-pack

# use wasm-pack to build artifact
wasm-pack build --release --target web
```

## Test

```bash
# Serve this directory for testing. (python server used in the example, 
# but any web server will do).
# This will server the index.html file in the root of the directory.
# There will only be output in the console if you load the page.
python3 -m http.server
```
