# Demo WebAssembly > Node.js

Esta es una demo de cómo usar Samuel desde una página web.

## Requisitos

* Python: Para poder ejecutar un servidor web ligero en el que servir la página web. Pero se puede usar cualquier otra forma de servir esa página.

## Uso

Primero deben copiarse los archivos `output/wasm/web/samuel.js` y `output/wasm/web/samuel_bg.wasm` en el directorio `demo/wasm/web/lib`.

Una vez copiados, ejecutar:

```bash
demo/wasm/web ~ python3 -m http.server
```

Abrir en el navegador la URL: [http://0.0.0.0:8000/](http://0.0.0.0:8000/)
