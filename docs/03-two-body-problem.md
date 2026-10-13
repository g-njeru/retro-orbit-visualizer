# 03 — The Two-Body Problem

Model 2 shows two stars orbiting a point *between* them. Why the centre of mass,
and why can we describe the pair with a single ellipse? This page derives it.

## Setup

Two point masses $m_1$ and $m_2$ at positions $\vec{r}_1$ and $\vec{r}_2$. Gravity
pulls them together (Newton's law of gravitation, and Newton's third law makes the
two forces equal and opposite):

$$\vec{F}_1 = -\vec{F}_2 = -G\frac{m_1 m_2}{\lvert\vec{r}_2-\vec{r}_1\rvert^3}
(\vec{r}_1-\vec{r}_2).$$

Define the **relative separation**
$\vec{r} = \vec{r}_2 - \vec{r}_1$, so $\lvert\vec{r}\rvert$ is the distance
between the bodies.

## Centre of mass

The **centre of mass** is the mass-weighted average position

$$\vec{R} = \frac{m_1\vec{r}_1 + m_2\vec{r}_2}{m_1 + m_2}.$$

Adding the two equations of motion, the forces cancel (Newton's third law), so

$$(m_1+m_2)\ddot{\vec{R}} = 0 \quad\Rightarrow\quad \vec{R} = \text{constant}
\text{ velocity}.$$

There is no outside force, so the centre of mass drifts at a constant velocity.
It is convenient (and exactly what the visualizer does) to sit in the frame where
$\vec{R} = 0$: **the centre of mass is fixed at the origin.** This is the little
`+` crosshair in Model 2.

With the centre of mass at the origin we can solve for each body in terms of the
separation:

$$\vec{r}_1 = -\frac{m_2}{m_1+m_2}\,\vec{r}, \qquad
\vec{r}_2 = +\frac{m_1}{m_1+m_2}\,\vec{r}.$$

The standing result:

> Each body orbits the centre of mass, and the **heavier body has the smaller
> orbit**, in the inverse mass ratio $r_1/r_2 = m_2/m_1$.

In Model 2, $m_1:m_2 = 3:1$, so $r_1:r_2 = 1:3$ — the $2.5$ and $7.5$ you see.

## Reducing two bodies to one

Subtract the two equations of motion (or just differentiate $\vec{r}$ twice) to
get a single equation for the separation:

$$\ddot{\vec{r}} = -\,\mu\,\frac{\vec{r}}{\lvert\vec{r}\rvert^3},
\qquad \mu = G(m_1+m_2).$$

This is the **one-body problem**: a single particle moving in the gravitational
field of a fixed point with mass parameter $\mu$. It is the entire reason we can
reuse Kepler's results for every model — from a satellite around a planet to a
comet around the Sun. The quantity $\mu = GM$ is called the **gravitational
parameter** (standard gravitational parameter).

## What is conserved

For this central force two quantities never change:

- **Specific angular momentum** $\vec{h} = \vec{r}\times\vec{v}$. Because the
  force points along $\vec{r}$, there is no torque, so $\vec{h}$ is constant.
  Its constancy forces the motion to lie in a **plane** and gives Kepler's second
  law (doc 04).
- **Specific energy** $\varepsilon = \tfrac12 v^2 - \mu/r$. Its sign sets the
  orbit shape: negative = ellipse, zero = parabola, positive = hyperbola
  (Model 5).

## Period

For a closed orbit the period depends only on the size, not the shape:

$$T = 2\pi\sqrt{\frac{a^3}{\mu}}, \qquad \mu = G(m_1+m_2),$$

which is Kepler's third law written for the general two-body case. In Model 2
both stars complete one revolution in the same time $T$: their positions share a
single angle, $\vec{r}_2$ just sits three times farther out than $\vec{r}_1$.

## Check the numbers (Model 2)

- Masses $600$ and $200$, so $r_1 = r\cdot\frac{200}{800} = 2.5$,
  $r_2 = r\cdot\frac{600}{800} = 7.5$ for a separation $r = 10$. ✔
- Both share one angular rate, so one period. ✔

Next: [04 — Kepler's Laws](04-keplers-laws.md).
