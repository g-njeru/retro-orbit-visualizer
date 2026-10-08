# 1963 Retro Orbit Visualizer

Monochrome 3D wireframe celestial mechanics visualization in Rust (Macroquad), inspired by Edward Zajac's 1963 Bell Labs computer animation *Two-Gyro Gravity-Gradient Satellite Attitude Simulation*.

- Spherically gridded wireframe planet (latitude rings + meridians)
- Box satellite on an eccentric Keplerian orbit (Newton-solved)
- Orbit path trail and gravity-gradient indicator line
- Slow orbiting camera, auto-fits any viewport aspect (desktop + phone)

## Run on desktop

```sh
cargo run --release
```

Requires Rust + OpenGL. Window is white/light-grey lines on black.

## Play in the browser (WASM)

GitHub Pages live demo:

https://g-njeru.github.io/retro-orbit-visualizer/

Same code, compiled to WebAssembly — works in any WebGL2 browser (desktop and mobile).

### Build manually

```sh
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
cp index.html target/wasm32-unknown-unknown/release/
cp web/mq_js_bundle.js target/wasm32-unknown-unknown/release/
# serve that directory over HTTP, e.g.
python3 -m http.server -d target/wasm32-unknown-unknown/release
```

### Deploy

Push to `main` → GitHub Actions builds the WASM and publishes to Pages automatically.