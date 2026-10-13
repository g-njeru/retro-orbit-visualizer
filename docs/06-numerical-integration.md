# 06 — Numerical Integration (Euler vs. Verlet)

Models 1–4, 6, and 7 place bodies using closed-form formulas (the ellipse
equations). But sometimes there is no formula for position as a function of time —
Model 5 integrates the actual force of gravity, and any real simulation must.
"Integrating" means stepping the motion forward in small time increments.
**How** you take that step turns out to matter enormously.

## The problem in equations

For a body in gravity, Newton gives a pair of ordinary differential equations
(ODEs):

$$\frac{d\vec{r}}{dt} = \vec{v}, \qquad
\frac{d\vec{v}}{dt} = \vec{a}(\vec{r}) = -\mu\,\frac{\vec{r}}{\lvert\vec{r}\rvert^3}.$$

We know the acceleration at any instant, but not the future position. So we break
time into small steps $h$ ("the timestep") and approximate each step.

## Euler's method (1768) — the simplest

Advance using the state *we already have*:

$$\vec{v}_{n+1} = \vec{v}_n + \vec{a}(\vec{r}_n)\,h, \qquad
\vec{r}_{n+1} = \vec{r}_n + \vec{v}_n\,h \quad\text{(explicit Euler)}.$$

The name **explicit** means the new position uses the *old* velocity. This is
first-order accurate: the error in one step is $O(h^2)$, so over a whole orbit the
error is $O(h)$.

### Why plain Euler fails: energy grows

Use the harmonic oscillator as a clean test case ($\ddot x = -\omega^2 x$, the
"mass on a spring", the same mathematics as a small orbit). Explicit Euler
multiplies the amplitude by $\sqrt{1+(\omega h)^2}$ **every step**, which is
greater than 1. The motion therefore **spirals outward and gains energy forever**.
Applied to an orbit, explicit Euler makes the planet drift away from the Sun.

### Semi-implicit (symplectic) Euler — what the code uses today

A tiny change fixes the worst of it: update the velocity *first*, then move using
the **new** velocity:

$$\vec{v}_{n+1} = \vec{v}_n + \vec{a}(\vec{r}_n)\,h, \qquad
\vec{r}_{n+1} = \vec{r}_n + \vec{v}_{n+1}\,h.$$

This is **semi-implicit** (also called *symplectic*) Euler, and it is what
`draw_energy` does (`src/main.rs:590`):

```rust
let acc = -pos * (mu / (r2 * r));   // a = -mu * r_hat / r^2
vel += acc * h;                     // update velocity
pos += vel * h;                     // then move with the NEW velocity
```

Semi-implicit Euler keeps the energy **bounded** (it no longer runs away), but it
is still only **first-order** accurate, so it produces a slow *phase error*: the
ellipse **precesses** (rotates slightly each orbit) instead of closing exactly.
The variable timestep and the `r.max(1.0)` clamp in the code also break the
symmetry that makes symplectic Euler behave well.

## Velocity Verlet (1967) — second order and symmetric

The Verlet method uses the acceleration at **both** ends of the step and treats
the midpoint consistently:

$$\vec{r}_{n+1} = \vec{r}_n + \vec{v}_n\,h + \tfrac12\,\vec{a}(\vec{r}_n)\,h^2,$$
$$\vec{a}_{n+1} = \vec{a}(\vec{r}_{n+1}),$$
$$\vec{v}_{n+1} = \vec{v}_n + \tfrac12\big(\vec{a}_n + \vec{a}_{n+1}\big)\,h.$$

Properties that make it the standard choice for orbital mechanics:

- **Second order**: the one-step error is $O(h^3)$, so the global error is
  $O(h^2)$ — one power of $h$ better than Euler.
- **Symmetric / time-reversible**: running it backwards recovers the start.
- **Symplectic**: it conserves a nearby "shadow" energy essentially exactly, so
  the **energy does not drift** over long runs — the orbit stays closed.

## Comparing the methods

| Method | Order | Energy over long run | Time-reversible |
|---|---|---|---|
| Explicit Euler | $O(h)$ | grows without bound | no |
| Semi-implicit Euler | $O(h)$ | bounded but precesses | no |
| **Velocity Verlet** | $O(h^2)$ | bounded, no drift | yes |

Smaller $h$ helps every method, but it cannot rescue first-order Euler: its error
is *systematic*, not random, so it accumulates.

## Seeing it in Model 5

Model 5 starts three bodies at $r=4$ around a mass with $\mu=100$ and integrates
their motion with the semi-implicit Euler step above, using 6 substeps per frame
with $h = \min(\text{dt}/6,\ 0.016)$ (`src/main.rs:585`). The **LIVE ENERGY**
readout `E = v²/2 − GM/r` (`src/main.rs:611`) should stay constant for each body.
Under Euler it does not: it slowly wobbles and the trails visibly precess instead
of closing. Switching to velocity Verlet makes the energy hold steady and the
closed orbit stay closed.

## The idea to take away

Numerical integration error is not just "rounding". For a *conservative* system
like gravity, the wrong integrator steadily injects or removes energy. Choosing a
**structure-preserving (symplectic)** method such as Verlet keeps the physics
honest over long simulations — which is why it is used in real orbital and
molecular dynamics software.

Next: [07 — Camera and 3D Math](07-camera-and-3d-math.md).
