# 1963 Retro Orbit Visualizer

Monochrome 3D wireframe celestial mechanics visualization in Rust (Macroquad), inspired by Edward Zajac's 1963 Bell Labs computer animation *Two-Gyro Gravity-Gradient Satellite Attitude Simulation*.

## Six orbital mechanics models

| Key | Model |
|---|---|
| `1` | Binary star system (mutual gravitation, center of mass) |
| `2` | Multi-planet solar system (Kepler harmonic law, speed labels) |
| `3` | Kepler's second law (equal swept areas, wedge shading) |
| `4` | Orbital energy & escape trajectories (bound / parabolic / hyperbolic) |
| `5` | Geostationary vs low Earth orbit (rotating Earth grid) |
| `6` | Highly eccentric cometary trajectory (perihelion whip + tail) |

Controls: `1`-`6` switch models, `R` reset view, `Z`/`X` zoom, drag to tilt, mouse wheel / pinch / on-screen `+`/`-` buttons to zoom.

Visual standard: white/light-grey wireframe lines on pitch-black, viewport auto-fits any aspect (desktop or phone).

## Run on desktop

```sh
cargo run --release
```

Requires Rust + OpenGL.

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