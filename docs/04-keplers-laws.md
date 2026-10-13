# 04 — Kepler's Laws

Johannes Kepler, working from Tycho Brahe's naked-eye data, found three rules that
describe planetary motion. Newton later showed all three follow from a single
inverse-square force law. These three rules are the backbone of the visualizer.

## The shape of orbits — an ellipse

An **ellipse** is the set of points whose distances to two fixed points (the
**foci**, singular *focus*) add up to a constant. Its size is set by the
semi-major axis $a$ and its shape by the eccentricity $e$.

| Quantity | Formula | Meaning |
|---|---|---|
| Eccentricity | $e = \dfrac{c}{a}$ | $0$ = circle, $0<e<1$ = ellipse, $1$ = parabola, $>1$ = hyperbola |
| Semi-minor axis | $b = a\sqrt{1-e^2}$ | the short radius |
| Focus distance | $c = ae$ | distance from centre to focus |
| Perihelion | $r_p = a(1-e)$ | closest approach |
| Aphelion | $r_a = a(1+e)$ | farthest distance |

**Law 1.** Each planet moves on an ellipse with the Sun at one focus. In the code
the Sun (or the central body) sits at $\vec{r}_{\text{focus}} = (ae, 0)$ — see
Model 4 (`src/main.rs:544`) and Model 7 (`src/main.rs:685`).

The distance from the focus as a function of the **true anomaly** $\nu$ (the angle
from perihelion) is the *orbit equation*

$$r(\nu) = \frac{a(1-e^2)}{1 + e\cos\nu}.$$

For example, at $\nu = 0$, $r = a(1-e)$ (perihelion); at $\nu = \pi$,
$r = a(1+e)$ (aphelion). ✔

## Equal areas in equal times

**Law 2.** A line from the Sun to a planet sweeps out equal areas in equal times.
In a small time $dt$ the line sweeps a thin triangle of area
$dA = \tfrac12 r^2\,d\nu$, so

$$\frac{dA}{dt} = \tfrac12 r^2 \dot\nu = \frac{L}{2m} = \text{constant},$$

where $L = m r^2\dot\nu$ is the angular momentum about the focus. So Kepler's
second law *is* conservation of angular momentum. Its visible consequence: a
planet moves **faster near perihelion, slower near aphelion**. That is the whole
point of Model 4's equal-area wedges.

The **specific angular momentum** (angular momentum per unit mass) is

$$h = \lvert\vec{r}\times\vec{v}\rvert = \sqrt{\mu\,a(1-e^2)}.$$

## The period — Kepler's third law

**Law 3.** The square of the period is proportional to the cube of the semi-major
axis:

$$T^2 = \frac{4\pi^2}{G(m_1+m_2)}\,a^3
\quad\text{or, for a planet around the Sun,}\quad
T^2 \propto a^3.$$

Written with $\mu = G(m_1+m_2)$: $T = 2\pi\sqrt{a^3/\mu}$. This is why Model 3's
planets at radii $4, 9, 16$ have periods in the ratio $8:27:64$ (doc 02).

## The vis-viva equation — speed anywhere on the orbit

Combining energy conservation with the geometry of the ellipse gives the **vis-viva
equation** ("living force"), the single most useful formula in the whole program:

$$v^2 = \mu\left(\frac{2}{r} - \frac{1}{a}\right).$$

It lets you compute speed at any distance without integrating anything.

Two special cases worth memorising:

- **Circular orbit** (radius $r = a$): $\displaystyle v_{\text{circ}} = \sqrt{\frac{\mu}{r}}$.
- **Escape (parabolic)** ($a \to \infty$): $\displaystyle v_{\text{esc}} = \sqrt{\frac{2\mu}{r}} = \sqrt{2}\,v_{\text{circ}}$.

## Energy and orbit type

The specific orbital energy

$$\varepsilon = \frac{v^2}{2} - \frac{\mu}{r}$$

is constant and, for a bound orbit, equals $-\dfrac{\mu}{2a}$. Its sign classifies
the trajectory:

| $\varepsilon$ | Orbit | Shape |
|---|---|---|
| $\varepsilon < 0$ | bound | ellipse / circle |
| $\varepsilon = 0$ | marginal escape | parabola |
| $\varepsilon > 0$ | unbound | hyperbola |

Model 5 tests all three at once: at $r=4$, $\mu=100$, the circular speed is
$v_{\text{circ}} = 5.0$ and the escape speed is $v_{\text{esc}} = 7.07$. The three
bodies are launched at $5.8$ (bound), $7.07$ (parabolic), and $9.2$ (escaping).

## Speed ratio at the apsides

At perihelion and aphelion the velocity is perpendicular to the radius, so
$h = r v$ is the same at both. Dividing gives

$$\frac{v_p}{v_a} = \frac{r_a}{r_p} = \frac{1+e}{1-e}.$$

For Model 7 ($e = 0.85$) that ratio is $12.33$ — the comet is over twelve times
faster at perihelion than at aphelion.

## Where these appear in the code

- `kepler_point(a, e, m)` (`src/main.rs:162`) returns a point on an ellipse.
- `orbit_path(a, e, segments)` (`src/main.rs:177`) traces the full curve.
- `energy_readout` (`src/main.rs:611`) evaluates $\varepsilon$ live.

Next: [05 — Anomalies and Kepler's Equation](05-anomalies-and-keplers-equation.md).
