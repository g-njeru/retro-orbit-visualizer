# 01 — Overview

## What the program is

The **1963 Retro Orbit Visualizer** is a 3D, real-time simulation of classical
orbital mechanics drawn as **monochrome wireframes** on a black background. It
runs both as a normal desktop window and, from the same source code, inside a
web browser.

The look is a deliberate homage to a real piece of computing history: Edward
Zajac's 1963 Bell Labs film *Two-Gyro Gravity-Gradient Satellite Attitude
Simulation*, one of the first computer-animated films. Those early films were
drawn on film recorders as pure white vector lines on black — hence the
aesthetic here.

## What you see on screen

- A **wireframe sphere** (a "globe" of latitude rings and longitude meridians)
  standing in for the central body.
- One or more **wireframe boxes** representing satellites, planets, or stars.
- **Orbit paths**: lines showing the trajectories.
- **Labels** computed live from the physics (for example an orbital speed or an
  energy value).
- A **HUD** (heads-up display) with the model name, its governing equation, and
  on-screen zoom / navigation buttons.

The viewport automatically fits whatever window or phone screen you use.

## Controls

| Input | Action |
|---|---|
| Keys `1`–`7` | Switch between the seven models |
| On-screen `<` `>` buttons (left) | Previous / next model |
| On-screen `+` `−` buttons (bottom-right) | Zoom in / out |
| Drag with the mouse | Tilt the camera |
| Mouse wheel | Zoom |
| Pinch (touch) | Zoom |
| `Z` / `X` | Zoom in / out |
| `R` | Reset the view |

## Technology

| Part | Choice |
|---|---|
| Language | Rust |
| Graphics | [Macroquad](https://macroquad.rs) (a small cross-platform game library) |
| Desktop | Native window, OpenGL |
| Browser | Compiled to WebAssembly (WASM), runs in any WebGL2 browser |
| Numbers | 32-bit floating point (`f32`) for rendering; the math itself is small |

Because Macroquad compiles to both native code and WebAssembly, the *same*
program runs in a desktop window and in a browser tab.

## The seven models at a glance

1. **Gravity-gradient satellite** — a satellite held in a Kepler ellipse, with a
   stick pointing at the planet (the original 1963 look).
2. **Binary star system** — two stars orbiting their common centre of mass.
3. **Multi-planet system** — three planets on circular orbits, faster when
   closer in (Kepler's third law).
4. **Kepler's second law** — equal areas swept in equal times.
5. **Orbital energy** — bound, parabolic, and hyperbolic ("escape") trajectories.
6. **Geostationary vs. low Earth orbit** — one Earth, two extreme orbits.
7. **Highly eccentric comet** — an orbit that whips around perihelion.

Each is described in detail, with its equations, in
[02 — The Seven Models](02-the-seven-models.md).

## How to run it

See [09 — Build and Deploy](09-build-and-deploy.md).
