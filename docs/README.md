# How This Visualizer Works — Documentation

These notes explain the **1963 Retro Orbit Visualizer** from the ground up: the
physics, the math, the graphics, and the engineering. They are written for
students who know some programming and high-school/early-university physics,
but they define every term and symbol before using it.

Math is written in LaTeX, so it renders nicely on GitHub (for example
$E = \tfrac12 m v^2 - \tfrac{GMm}{r}$).

## Suggested reading order

| # | Document | What you learn |
|---|---|---|
| 01 | [Overview](01-overview.md) | What the program is, and what you see on screen |
| 02 | [The Seven Models](02-the-seven-models.md) | Every model, its physics, its equations, and its screen scaling |
| 03 | [The Two-Body Problem](03-two-body-problem.md) | Newtonian gravity, centre of mass, reduced mass |
| 04 | [Kepler's Laws](04-keplers-laws.md) | Ellipses, eccentricity, the vis-viva equation, escape |
| 05 | [Anomalies and Kepler's Equation](05-anomalies-and-keplers-equation.md) | Mean/true/eccentric anomaly and how the solver works |
| 06 | [Numerical Integration](06-numerical-integration.md) | Euler vs. Verlet, error, and energy drift |
| 07 | [Camera and 3D Math](07-camera-and-3d-math.md) | Vectors, rotations, the camera, and screen projection |
| 08 | [Rendering Wireframes](08-rendering-wireframes.md) | How the monochrome line art is drawn |
| 09 | [Build and Deploy](09-build-and-deploy.md) | Rust, WASM, GitHub Pages, CI, and releases |
| 10 | [Glossary and References](10-glossary-and-references.md) | Symbol/term lookup and further reading |

If you only have ten minutes, read **01**, **02**, and **06**.

## Where things live in the code

Almost everything is one file, `src/main.rs`. Handy anchors:

| Concept | Function / location |
|---|---|
| Model descriptions, titles, base radius | `mode_cfg` (`src/main.rs:36`) |
| Camera fit distance | `fit_distance` (`src/main.rs:274`) |
| Camera construction | `build_cam` (`src/main.rs:281`) |
| Perspective projection | `project_with` (`src/main.rs:302`) |
| Kepler solver (Newton–Raphson) | `solve_kepler` (`src/main.rs:150`) |
| Position on an orbit | `kepler_point` (`src/main.rs:162`) |
| Rotation helpers | `rotate_x/y/z`, `to_world` (`src/main.rs:145`, `451`, `456`, `461`) |
| Gravity integrator (Model 5) | `draw_energy` (`src/main.rs:580`) |
| Wireframe sphere | `draw_wireframe_sphere` (`src/main.rs:186`) |
| Wireframe box | `draw_wireframe_box` (`src/main.rs:221`) |
| HUD (titles, buttons) | `hud` (`src/main.rs:396`) |
| Main loop | `main` (`src/main.rs:727`) |
| Unit tests | `mod tests` (`src/main.rs:808`) |

> **A note on the original specification.** The file
> `astronomical_visualization_models.txt` is the *first draft* of the concept and
> describes **six** models in a different order. The shipped program has **seven**
> models and uses the numbering you see in the app. This documentation follows
> the shipped program, not the draft.
