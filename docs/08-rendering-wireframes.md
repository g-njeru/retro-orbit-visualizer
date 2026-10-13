# 08 — Rendering Wireframes

The whole visual style is **lines on black**. There are no filled surfaces, no
lighting, and no textures. Everything is built from one primitive —
`draw_line_3d(a, b, color)` — combined into reusable shapes.

## The one primitive: `draw_polyline`

`draw_polyline(points, color)` (`src/main.rs:171`) connects a list of points with
straight segments by drawing a line for each neighbouring pair. Every curve
(orbits, rings, sphere grids, trails) is just a dense polyline: enough short
segments look smooth.

## Wireframe sphere

A globe is a **grid of latitude and longitude lines**:

- **Latitude rings** are horizontal circles. At latitude $\varphi$ the ring has
  radius $R\cos\varphi$ and sits at height $R\sin\varphi$.
- **Longitude meridians** are half-circles from pole to pole; the code draws them
  as **full great circles** through the poles, each rotated by a different
  longitude.

A point on the sphere is parametrised as

$$\text{local} = R\big(\cos\varphi\cos\theta,\ \sin\varphi,\ \cos\varphi\sin\theta\big),$$

then rotated by the body's spin:

$$\vec{p} = \text{center} + R_y(\text{spin} + \text{lon})\cdot\text{local}.$$

Using full great circles (rather than half-meridians) is what makes the grid look
correct from every angle. This is `draw_wireframe_sphere` (`src/main.rs:186`);
Model 6 uses a finer grid ($8 \times 8$) than the small bodies ($7 \times 6$).

## Wireframe box

A box (satellite, planet marker) is eight corners joined by twelve edges. Given a
centre, a **forward** direction, and half-widths, the code builds an orthonormal
frame:

$$\vec{r} = \widehat{\vec{f}\times\hat{y}}, \qquad
  \vec{u} = \vec{r}\times\vec{f},$$

falls back to a default if $\vec{f}$ is parallel to $\hat y$ (to avoid a
degenerate cross product), and places the corners at
$\text{centre} \pm \vec{r}\,h_x \pm \vec{u}\,h_y \pm \vec{f}\,h_z$. All twelve
edges are then drawn. This is `draw_wireframe_box` (`src/main.rs:221`); the box is
often oriented with `forward` pointing along the orbit or at the central body.

## Orbit rings and dashed lines

- `draw_orbit_ring` (`src/main.rs:256`) is just a circle polyline — used for the
  circular guides in Models 2 and 3.
- `draw_dashed_line_3d` (`src/main.rs:266`) splits a segment into short drawn
  pieces, e.g. the GEO tether in Model 6.

## Trails

To leave a trail, a model stores its recent positions in a list and draws them as
a polyline (for example `e_trails` in Model 5, `src/main.rs:594`). Old points are
dropped once the list is long enough, so the trail has a fixed length.

## The monochrome palette

There is one hue — grey — used at three brightness levels, a classic trick for
depth and hierarchy in line art:

| Colour | Use |
|---|---|
| `WHITE` | the main subject (stars, planets, the active body) |
| `LIGHTGRAY` | secondary geometry (globes, guides, labels) |
| dim grey (`0.4`–`0.5`) | orbit paths and less important lines |

The background is pure black (`clear_background(BLACK)`), which makes the lines
read like the film-recorder output of the original 1963 animation.

## The HUD (2D overlay)

After the 3D scene is drawn, `set_default_camera()` switches to flat 2D and `hud`
(`src/main.rs:396`) draws the title, the governing equation, the control legend,
the four zoom/navigation buttons, and — in a dev build — a `DEV BUILD` watermark
and the FPS counter. Model-specific readouts (the energy panel in Model 5) are
drawn by `energy_readout` (`src/main.rs:611`).

The HUD buttons are plain rectangles; `ui_rects` (`src/main.rs:325`) computes their
positions from the current screen size, and `fit_camera` checks whether a click
landed inside one of them using the **pixel** mouse position.

## Text labels in 3D

To label a point in space (e.g. "GEO T = 10 s"), the code projects the 3D point to
screen coordinates with `project` and then draws ordinary 2D text there
(`draw_3d_label`, `src/main.rs:318`). The projection math is in
[07 — Camera and 3D Math](07-camera-and-3d-math.md).

Next: [09 — Build and Deploy](09-build-and-deploy.md).
