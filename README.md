# rust-all
Just coding nice staf in rust lang

## install

- here: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- documentation: [https://rust-lang.github.io/rustup/installation/windows.html](https://rust-lang.github.io/rustup/installation/windows.html)

### install path

- path: `c:/users/<thebestwindowsuserintheworldxd>/.cargo/bin`

### community

- [https://rustaceans.org/](https://rustaceans.org/)

### Notes

- memory safe, null pointer exception
- deterministic memory (no garbage collector)
- WASM, (binaries in the browser)

#### tools made with rust

1. Dusk [https://dusk.network] (blockchain enteros)
2. Serum [https://portal.projectserum.com/] (es una Dapps -descentralized application- (realiza transacciones en el bc -block chain- de Solana))
3. Penumbra [https://penumbra.zone/] (this is the second layer, transacciones de segunda capa)

### WASM (WebAssembly)

- basically: bin exec in browser
- web3.0 conects with blockchain
- WASM site: [https://webassembly.org/](https://webassembly.org/)
    - dev site: [https://webassembly.org/getting-started/developers-guide/](https://webassembly.org/getting-started/developers-guide/)
- wasm by example [https://wasmbyexample.dev/home.en-us.html](https://wasmbyexample.dev/home.en-us.html)



## notes and code

- Install el pack de wasm
    - `cargo install wasm-pack`
- To create a project
    - `cargo new rust-wasm`
- To create a library
    - `cargo new --lib rust-wasm`
- build
    - `wasm-pack build --target web `
    - the `rust_wasm.js` file can be included in a html, as module, like this:
        ```
        <script type="module">
            import init, {saludar} from "./pkg/rust_wasm.js";
            init().then( () => {
                saludar("jamecho");
            });
        </script>
        ```
    - then, serve using python http, like this:
        - `python3 -m http.server 8091`
- the rust code in this wasm example is:
    ```
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern {
        pub fn alert(s: &str);
    }

    #[wasm_bindgen]
    pub fn saludar(name: &str) {
        alert(&format!("Hola, {}, how u doing?", name))
    }
    ```
    - it is a lib, and is into the `src` folder

