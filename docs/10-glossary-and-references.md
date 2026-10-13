# 10 — Glossary and References

## Symbols

| Symbol | Name | Meaning |
|---|---|---|
| $a$ | semi-major axis | half the long axis of an ellipse; sets the orbit size |
| $b$ | semi-minor axis | half the short axis; $b = a\sqrt{1-e^2}$ |
| $e$ | eccentricity | shape: $0$ circle → $1$ parabola → $>1$ hyperbola |
| $E$ | eccentric anomaly | auxiliary angle at the ellipse centre |
| $M$ | mean anomaly | uniform "clock" angle |
| $\nu$ | true anomaly | real angle seen from the focus |
| $\mu$ | gravitational parameter | $\mu = GM$; standard for a given body/system |
| $\varepsilon$ | specific orbital energy | $v^2/2 - \mu/r$ |
| $h$ | specific angular momentum | $\lvert\vec{r}\times\vec{v}\rvert$ |
| $h$ | (integration) timestep | the small time step; also written $dt$ |
| $T$ | period | time for one orbit; $T = 2\pi\sqrt{a^3/\mu}$ |
| $r_p, r_a$ | perihelion, aphelion | closest / farthest distances |
| $\text{FOV}$ | field of view | angular size the camera sees |

## Terms A–Z

- **Anomaly** — an angle describing a position on an orbit (mean, eccentric, or
  true). See doc 05.
- **Aphelion / perihelion** — the farthest / nearest points of an orbit around the
  Sun (generic terms: *apoapsis* / *periapsis*).
- **Conventional Commits** — a commit-message convention; prefixes like `feat:`
  and `fix:` drive automatic versioning.
- **Eccentricity** — how far an ellipse is from a circle.
- **Escape velocity** — the speed needed to leave on an unbound (parabolic)
  trajectory: $\sqrt{2\mu/r}$.
- **`f32` / `f64`** — IEEE-754 floating-point numbers with about 7 / 16 decimal
  digits of precision. The renderer uses `f32`.
- **Field of view (FOV)** — the angular extent captured by the camera.
- **Focus (plural foci)** — one of the two special points of an ellipse; a central
  body sits at one focus.
- **Gravity gradient** — the small variation of gravity across a large object,
  which torques it into alignment (Model 1).
- **Gravitational parameter** $\mu = GM$ — the standard "strength" of gravity for
  a system.
- **Integrator** — a numerical method that advances a system in time step by step
  (e.g. Euler, Verlet). See doc 06.
- **Kepler's equation** — $M = E - e\sin E$, relating mean and eccentric anomaly.
- **LTO (link-time optimization)** — a build setting (`Cargo.toml`) that makes the
  release binary smaller/faster at the cost of compile time.
- **Macroquad** — the small Rust game/graphics library this project uses.
- **Mean anomaly** — a uniform angle proportional to time.
- **MSRV (minimum supported Rust version)** — the oldest compiler the crate
  supports; here **1.85**, enforced by CI.
- **Normalize** — make a vector unit length (divide by its length).
- **Numerical drift** — the slow accumulation of error in a simulation; for
  integrators, often seen as energy gain/loss. See doc 06.
- **ODE (ordinary differential equation)** — an equation giving rates of change
  (like acceleration) rather than the value directly.
- **Orbit ellipse / orbit equation** — the curve a body follows; its radius is
  $r(\nu) = a(1-e^2)/(1+e\cos\nu)$.
- **Parabola / hyperbola** — the open-trajectory shapes for energy zero / positive.
- **Perspective projection** — turning 3D points into 2D by dividing by depth.
- **Reduced mass** — the effective mass $\mu = G(m_1+m_2)$ in the two-body
  reduction (doc 03).
- **Rotational matrix** — a matrix that rotates a vector; see doc 07.
- **Semi-major axis** — half the long axis; the orbit's size.
- **Semi-implicit (symplectic) Euler** — a first-order integrator; current choice in
  Model 5.
- **Specific** quantity — "per unit mass" (e.g. specific energy, specific angular
  momentum).
- **Symplectic integrator** — one that preserves the geometric structure of
  Hamiltonian mechanics, so energy stays bounded. Verlet is one.
- **Two-body problem** — two masses under mutual gravity; reducible to a single
  body in a central field (doc 03).
- **Velocity Verlet** — a second-order, time-reversible symplectic integrator (doc
  06).
- **Vis-viva equation** — $v^2 = \mu(2/r - 1/a)$, the speed anywhere on an orbit.
- **WASM (WebAssembly)** — a compact binary that runs in web browsers; the Rust
  code is compiled to it for the online version.
- **WebGL2** — the browser 3D graphics API Macroquad targets.
- **World space** — the 3D coordinate system the models are defined in, before
  the camera projects it to the screen.

## References and further reading

- **Zajac, E.** — *Two-Gyro Gravity-Gradient Satellite Attitude Simulation*, Bell
  Laboratories, 1963 (the film that inspired the visual style).
- **Kepler, J.** — *Astronomia Nova* (1609) and *Harmonices Mundi* (1619): the
  three laws.
- **Newton, I.** — *Philosophiæ Naturalis Principia Mathematica* (1687): universal
  gravitation.
- **Bate, R., Mueller, D., White, J.** — *Fundamentals of Astrodynamics* (Dover).
  A classic, gentle introduction.
- **Vallado, D.** — *Fundamentals of Astrodynamics and Applications*. The standard
  modern reference.
- **Hairer, E., Lubich, C., Wanner, G.** — *Geometric Numerical Integration*.
  The definitive treatment of symplectic integrators like Verlet.
- **Macroquad** — https://macroquad.rs
- **Conventional Commits** — https://www.conventionalcommits.org
- **MDN WebAssembly** — https://developer.mozilla.org/docs/WebAssembly

Back to the [index](README.md).
