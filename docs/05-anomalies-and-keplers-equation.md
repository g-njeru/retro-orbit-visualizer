# 05 — Anomalies and Kepler's Equation

`orbit_path` draws a full ellipse by stepping an angle from $0$ to $2\pi$. But the
planet does **not** move at a constant angle — it speeds up near perihelion. So we
cannot simply advance the real angle at a constant rate. Kepler's equation is the
bridge between "time" and "position on the ellipse".

## Three angles for one point

For a point on an ellipse we use three different angles (called **anomalies**;
"anomaly" here just means "angle"):

| Anomaly | Symbol | What it is |
|---|---|---|
| **Mean** anomaly | $M$ | A fictitious *uniform* angle, $M = 2\pi\,(t-t_p)/T$. Runs at a constant rate like clockwork. |
| **Eccentric** anomaly | $E$ | An auxiliary angle at the *centre* of the ellipse. The convenient middle-man. |
| **True** anomaly | $\nu$ | The *real* angle of the body as seen from the focus. |

Time is easy in $M$ ($M$ grows steadily), position is easy in $E$ and $\nu$, and
Kepler's equation links $M$ and $E$.

## From eccentric anomaly to position

Draw a circle of radius $a$ around the ellipse's centre (the "auxiliary circle")
and let $E$ be the angle at the centre for the corresponding circle point. Then
the body's coordinates in the orbital plane are

$$x = a(\cos E - e), \qquad y = a\sqrt{1-e^2}\,\sin E,$$

and, using $x^2 + y^2$,

$$r = a\,(1 - e\cos E).$$

The code returns exactly this: `kepler_point` gives
$\big(a(\cos E - e),\; 0,\; a\sqrt{1-e^2}\,\sin E\big)$ (`src/main.rs:162`) — the
$y$ component is zero and the orbit lies in the $x$–$z$ plane.

## Kepler's equation

The relationship between $M$ and $E$ is

$$\boxed{\,M = E - e\sin E\,}$$

### Where it comes from (area argument)

Kepler's second law says area is swept at a constant rate, so the mean anomaly is
just the swept area as a fraction of the whole ellipse, scaled to $2\pi$. The
area swept between perihelion and eccentric anomaly $E$ is, by geometry,

$$A(E) = \frac{ab}{2}\,(E - e\sin E),$$

where $a b$ is the full ellipse's area $\pi a b$ divided appropriately. Setting
$M = 2\pi \dfrac{A(E)}{\pi a b}$ gives $M = E - e\sin E$. That is the origin of the
equation.

## Why we must solve it numerically

Kepler's equation is **transcendental**: $E$ appears both alone and inside a sine,
and there is no formula that isolates $E$. So for a given $M$ and $e$ we solve for
$E$ by iteration. This is the `solve_kepler` function.

## Newton–Raphson in the code

Newton–Raphson finds a root of $f(E) = E - e\sin E - M$. Since
$f'(E) = 1 - e\cos E$, each step is

$$E_{n+1} = E_n - \frac{E_n - e\sin E_n - M}{1 - e\cos E_n}.$$

The code (`src/main.rs:150`) starts at $E_0 = M$, repeats this up to 24 times, and
stops early when the correction $\lvert \Delta E \rvert < 10^{-6}$:

```rust
fn solve_kepler(m: f32, e: f32) -> f32 {
    let mut e_anomaly = m;
    for _ in 0..24 {
        let delta = (e_anomaly - e * e_anomaly.sin() - m)
                  / (1.0 - e * e_anomaly.cos());
        e_anomaly -= delta;
        if delta.abs() < 1e-6 { break; }
    }
    e_anomaly
}
```

### Convergence notes

- For small eccentricity ($e$ near 0) Newton converges in a couple of steps.
- As $e \to 1$ the derivative $1 - e\cos E$ can approach $0$ near perihelion, so
  convergence slows and the initial guess $E_0 = M$ becomes poor. A common
  improvement is to start closer, e.g. $E_0 = \pi$ for high $e$ (the code uses the
  simple $E_0 = M$; see doc 06 for the numerical-precision discussion).
- The existing unit test `kepler_solver_converges` (`src/main.rs:812`) checks the
  residual $\lvert E - e\sin E - M\rvert$ stays small over a range of $e$ and $M$.

## Why this gives the right *speed*

`orbit_path` sweeps the **mean** anomaly uniformly and converts each value to a
position. Because the conversion compresses points where the body is far and
spreads them where it is near, the drawn point automatically speeds up at
perihelion — Kepler's second law falls out for free, with no extra code.

Next: [06 — Numerical Integration](06-numerical-integration.md).
