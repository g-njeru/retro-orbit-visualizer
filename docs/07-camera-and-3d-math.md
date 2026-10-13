# 07 — Camera and 3D Math

The models are defined in a 3D **world space**, but the screen is 2D. This page
explains the small toolkit that turns one into the other: vectors, rotations, the
camera, and the perspective projection.

## Vectors

A `Vec3` is three numbers $(x, y, z)$ — a position or a direction. The operations
used throughout the code:

| Operation | Meaning | Code |
|---|---|---|
| $\lvert\vec{v}\rvert$ | length: $\sqrt{x^2+y^2+z^2}$ | `v.length()` |
| $\hat{v}$ | unit vector: $\vec{v}/\lvert\vec{v}\rvert$ | `v.normalize()` |
| $\vec{a}\cdot\vec{b}$ | dot product: $a_xb_x+a_yb_y+a_zb_z$ | `a.dot(b)` |
| $\vec{a}\times\vec{b}$ | cross product (a perpendicular vector) | `a.cross(b)` |

The **dot product** measures how much two vectors point the same way; the **cross
product** gives a vector perpendicular to both. Both are the workhorses of the
camera code.

In this world, $y$ points **up**.

## Rotations

Rotating a point about each axis uses standard rotation matrices. With
$s = \sin a$ and $c = \cos a$:

$$
R_x(a):\ (x,\ y\,c - z\,s,\ y\,s + z\,c), \qquad
R_y(a):\ (x\,c + z\,s,\ y,\ -x\,s + z\,c),
$$
$$
R_z(a):\ (x\,c - y\,s,\ x\,s + y\,c,\ z).
$$

These are `rotate_x`, `rotate_y`, `rotate_z` (`src/main.rs:451`, `145`, `456`). A
rotation never changes a vector's length — a fact the unit tests check
(`rotations_preserve_length_and_identity`, `src/main.rs:854`).

`to_world` (`src/main.rs:461`) tilts an orbit into a 3D viewing plane by applying
$R_x(0.5)$ after $R_z(0.7)$, so the ellipse is not seen edge-on.

## The camera as a frame

A camera is a position plus three unit vectors: which way is **right**, **up**,
and **forward**. `build_cam` (`src/main.rs:281`) places the camera on a sphere
around the origin using two angles and a distance:

$$\text{pos} = \big(d\cos\phi\cos\theta,\ d\sin\phi,\ d\cos\phi\sin\theta\big),$$

where $\theta$ is **yaw**, $\phi$ is **pitch**, and $d$ is the distance. The camera
always looks at the origin. The on-screen `right`/`up`/`forward` vectors are then

$$\vec{f} = \widehat{\text{origin} - \text{pos}}, \quad
\vec{r} = \widehat{\vec{f}\times\vec{y}}, \quad
\vec{u} = \widehat{\vec{r}\times\vec{f}}.$$

## Fitting the view

How far back must the camera sit so a sphere of radius $R$ fits on screen? If the
vertical field of view is $\text{FOV}_y$, the angular radius of the sphere is
$\arcsin(R/d)$, so we need $\arcsin(R/d) \le \text{FOV}_{\min}/2$, giving

$$d = \frac{R}{\sin(\text{FOV}_{\min}/2)} \times 1.15,$$

with a 15 % margin. The horizontal FOV depends on the screen aspect ratio $\alpha
= w/h$:

$$\text{FOV}_h = 2\arctan\!\big(\tan(\text{FOV}_y/2)\cdot\alpha\big), \qquad
\text{FOV}_{\min} = \min(\text{FOV}_y, \text{FOV}_h).$$

Taking the *minimum* guarantees the object fits whether the window is wide or
tall. This is `fit_distance` (`src/main.rs:274`), and it is why the app looks the
same on a phone and a desktop monitor. Zooming simply divides $d$ by a factor.

The vertical field of view is `FOV_Y = 50°` (`src/main.rs:3`).

## Perspective projection (world → screen)

A perspective camera makes far things smaller. First move the point into the
camera's frame by projecting it onto the three basis vectors:

$$ c_z = (\vec{p}-\vec{p}_{\text{cam}})\cdot\vec{f}, \quad
   c_x = (\vec{p}-\vec{p}_{\text{cam}})\cdot\vec{r}, \quad
   c_y = (\vec{p}-\vec{p}_{\text{cam}})\cdot\vec{u}. $$

$c_z$ is the depth along the view direction. If $c_z \le 0.01$ the point is behind
(or too close to) the camera and is culled. Otherwise the screen position is

$$ x_{\text{screen}} = \frac{w}{2} + \frac{c_x}{c_z}\cdot\frac{h/2}{\tan(\text{FOV}_y/2)},
\qquad
 y_{\text{screen}} = \frac{h}{2} - \frac{c_y}{c_z}\cdot\frac{h/2}{\tan(\text{FOV}_y/2)}. $$

The factor $\frac{h/2}{\tan(\text{FOV}_y/2)}$ converts an angle into pixels. The
$y$ subtraction flips the axis because screen coordinates grow *downward* while
world $y$ grows *upward*. This is `project_with` (`src/main.rs:302`).

Projection is used for two things:

1. Macroquad draws all the 3D lines using this same perspective automatically
   (via `set_camera(Camera3D)`).
2. `project_with` is called directly to place **2D text labels** at 3D points
   (`draw_3d_label`, `src/main.rs:318`).

## Putting it together (per frame)

1. `fit_camera` handles input and updates yaw / pitch / zoom (`src/main.rs:337`).
2. `build_cam` computes the camera position and basis (`src/main.rs:281`).
3. `set_camera` installs it; the model's `draw_*` function draws the 3D scene.
4. `set_default_camera` returns to 2D; the HUD and labels are drawn on top.

Next: [08 — Rendering Wireframes](08-rendering-wireframes.md).
