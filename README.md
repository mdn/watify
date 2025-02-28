# Watify

Compile WAT to WASM with WASM 🙇

## Usage

For local testing, build the package, then server the project root 
by running an http server. THe `index.html` file has this example.
The output can be seen in the browser's console.

```javascript
import init, { watify } from "watify";
const w = `
(module
  (func $fac (export "fac") (param $x i64) (result i64)
    (return_call $fac-aux (local.get $x) (i64.const 1))
  )

  (func $fac-aux (param $x i64) (param $r i64) (result i64)
    (if (result i64) (i64.eqz (local.get $x))
      (then (return (local.get $r)))
      (else
        (return_call $fac-aux
          (i64.sub (local.get $x) (i64.const 1))
          (i64.mul (local.get $x) (local.get $r))
        )
      )
    )
  )
)`;
init().then(() => {
  const wasm = watify(w);
  console.log(wasm);
  WebAssembly.instantiate(wasm, {}).then((result) => {
    const fac = result.instance.exports.fac;
    console.log(fac(5n));
  });
});
```
