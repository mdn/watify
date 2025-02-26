# Watify

Compile WAT to WASM with WASM 🙇

## Requirements

To build, install the rust toolchain and `wasm-pack`.

## Running

```bash
# Optional: install wasm-pack if not already installed
cargo install wasm-pack

# build
wasm-pack build --release --target web

# Serve this directory for testing. (python server used in the example, 
# but any web server will do).
# This will server the index.html file in the root of the directory.
# There will only be output in the console if you load the page.
python3 -m http.server
```

Go to [localhost:8000](http://localhost:8000/) and look at he console output.
