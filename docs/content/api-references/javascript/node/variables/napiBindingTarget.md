[@comfit/node](../index.md) / \_\_napiBindingTarget

# Variable: \_\_napiBindingTarget

> `const` **\_\_napiBindingTarget**: `"native"` \| `"wasm32-wasi"` \| `"wasm32-wasip1"`

Defined in: index.d.ts:10

Which binding artifact the generated loader actually loaded: `'native'` for
a native addon, otherwise the `platformArchABI` of the WASI flavor. Every
flavor napi-rs can build is listed, because `NAPI_RS_NATIVE_LIBRARY_PATH`
can point the loader at a WASI artifact this package does not build itself.
