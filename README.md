# 1963 Retro Orbit Visualizer

Monochrome 3D wireframe celestial mechanics visualization in Rust (Macroquad), inspired by Edward Zajac's 1963 Bell Labs computer animation *Two-Gyro Gravity-Gradient Satellite Attitude Simulation*.

## Seven orbital mechanics models

| Key | Model |
|---|---|
| `1` | Gravity-gradient satellite (original 1963 Zajac aesthetic) |
| `2` | Binary star system (mutual gravitation, center of mass) |
| `3` | Multi-planet solar system (Kepler harmonic law, speed labels) |
| `4` | Kepler's second law (equal swept areas, wedge shading) |
| `5` | Orbital energy & escape trajectories (bound / parabolic / hyperbolic) |
| `6` | Geostationary vs low Earth orbit (rotating Earth grid) |
| `7` | Highly eccentric cometary trajectory (perihelion whip + tail) |

Controls: keys `1`-`7` switch models, on-screen `‹` `›` buttons navigate models (touch), `R` reset view, `Z`/`X` zoom, drag to tilt, mouse wheel / pinch / on-screen `+`/`-` buttons to zoom.

Visual standard: white/light-grey wireframe lines on pitch-black, viewport auto-fits any aspect (desktop or phone).

## Branches & releases

- `dev` — working branch, live preview at `/dev/`, built with the `dev-overlay` feature (DEV BUILD watermark). Push `dev` → auto-deploy preview.
- `main` — production, the version presented publicly. Merge `dev`→`main` → auto-deploy root + auto-release.
- Conventional Commits enforced on PRs (`feat:`, `fix:`, `docs:`, `chore:` …). On `main` merges, `release-plz` bumps the version, updates `CHANGELOG.md`, creates a GitHub Release and `vX.Y.Z` tag (`feat:` → minor, `fix:`/`chore:` → patch, breaking → major).

## Run on desktop

```sh
cargo run --release            # production build
cargo run --features dev-overlay   # dev build (watermark + debug readout)
```

Requires Rust + OpenGL.

## Play in the browser (WASM)

GitHub Pages:

- Production: https://g-njeru.github.io/retro-orbit-visualizer/
- Dev preview: https://g-njeru.github.io/retro-orbit-visualizer/dev/

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

Push to `main` or `dev` → GitHub Actions builds the WASM and publishes to Pages automatically (`main` → root, `dev` → `/dev/`).