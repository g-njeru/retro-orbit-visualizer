# TASK SPECIFICATION: 1963 Retro Orbit Visualizer in Rust (Macroquad)

## 1. Overview
Build a standalone desktop and WebAssembly-compatible celestial mechanics visualization in Rust using the `macroquad` graphics library. The application replicates the visual aesthetic of Edward Zajac's historic 1963 Bell Labs computer animation ("Two-Gyro Gravity-Gradient Satellite Attitude Simulation"):
- Pure monochrome vector wireframe rendering (white/light-grey lines on a pitch-black background).
- Spherically gridded central planet (wireframe Earth with latitude and longitude rings).
- An orbiting box satellite undergoing an eccentric Keplerian trajectory.
- Clear visual cues: an orbit trajectory path and a gravity-gradient indicator connecting the satellite to the primary body's center.

---

## 2. Project Configuration

### `Cargo.toml`
Ensure the following dependencies and release optimizations are present:

```toml
[package]
name = "retro_orbit_visualizer"
version = "0.1.0"
edition = "2021"

[dependencies]
macroquad = "0.4"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
