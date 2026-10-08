use macroquad::prelude::*;

const FOV_Y: f32 = 50.0_f32.to_radians();

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    BinaryStars,
    MultiPlanet,
    KeplerAreas,
    Energy,
    GeoLeo,
    Comet,
}

struct ModeCfg {
    name: &'static str,
    eq: &'static str,
    base_radius: f32,
}

fn mode_cfg(mode: Mode) -> ModeCfg {
    match mode {
        Mode::BinaryStars => ModeCfg {
            name: "MODEL 1: BINARY STAR SYSTEM (MUTUAL GRAVITATION)",
            eq: "F = G M1 M2 / r^2   T^2 = 4PI^2 a^3 / (G(M1+M2))",
            base_radius: 9.0,
        },
        Mode::MultiPlanet => ModeCfg {
            name: "MODEL 2: MULTI-PLANET SYSTEM (KEPLER HARMONIC LAW)",
            eq: "v = sqrt(GM/r)   T ~ r^(3/2)",
            base_radius: 17.5,
        },
        Mode::KeplerAreas => ModeCfg {
            name: "MODEL 3: KEPLER 2ND LAW (EQUAL AREAS)",
            eq: "dA/dt = L/(2m) = const   r(t) = a(1-e^2)/(1+e cos t)",
            base_radius: 16.5,
        },
        Mode::Energy => ModeCfg {
            name: "MODEL 4: ORBITAL ENERGY & ESCAPE TRAJECTORIES",
            eq: "E = v^2/2 - GM/r   v_esc = sqrt(2) v_circ",
            base_radius: 15.0,
        },
        Mode::GeoLeo => ModeCfg {
            name: "MODEL 5: GEOSTATIONARY vs LOW EARTH ORBIT",
            eq: "T = 2PI sqrt(r^3/(GM))   GEO: T_orbit = T_earth",
            base_radius: 12.0,
        },
        Mode::Comet => ModeCfg {
            name: "MODEL 6: HIGHLY ECCENTRIC COMET",
            eq: "v_p/v_a = (1+e)/(1-e) = 12.3 for e = 0.85",
            base_radius: 24.0,
        },
    }
}

struct Cam {
    pos: Vec3,
    right: Vec3,
    up: Vec3,
    fwd: Vec3,
}

struct State {
    mode: Mode,
    yaw: f32,
    pitch: f32,
    zoom: f32,
    bin_angle: f32,
    spin1: f32,
    spin2: f32,
    planet_angles: [f32; 3],
    kep_angle: f32,
    wedge_timer: f32,
    wedges: Vec<(Vec3, Vec3)>,
    e_pos: [Vec3; 3],
    e_vel: [Vec3; 3],
    e_trails: [Vec<Vec3>; 3],
    earth_rot: f32,
    leo_angle: f32,
    geo_angle: f32,
    comet_angle: f32,
    prev_pinch: Option<f32>,
}

fn reset_mode(state: &mut State, mode: Mode) {
    state.mode = mode;
    match mode {
        Mode::BinaryStars => {
            state.bin_angle = 0.0;
            state.spin1 = 0.0;
            state.spin2 = 0.0;
        }
        Mode::MultiPlanet => {
            state.planet_angles = [0.0, 0.0, 0.0];
        }
        Mode::KeplerAreas => {
            state.kep_angle = 0.0;
            state.wedge_timer = 0.0;
            state.wedges.clear();
        }
        Mode::Energy => {
            for i in 0..3 {
                state.e_pos[i] = vec3(0.0, 0.0, 4.0);
                state.e_trails[i].clear();
            }
            state.e_vel[0] = vec3(5.8, 0.0, 0.0);
            state.e_vel[1] = vec3(7.07, 0.0, 0.0);
            state.e_vel[2] = vec3(9.2, 0.0, 0.0);
        }
        Mode::GeoLeo => {
            state.earth_rot = 0.0;
            state.leo_angle = 0.0;
            state.geo_angle = 0.0;
        }
        Mode::Comet => {
            state.comet_angle = 0.0;
        }
    }
}

fn rotate_y(v: Vec3, a: f32) -> Vec3 {
    let (s, c) = a.sin_cos();
    vec3(v.x * c + v.z * s, v.y, -v.x * s + v.z * c)
}

fn solve_kepler(m: f32, e: f32) -> f32 {
    let mut e_anomaly = m;
    for _ in 0..24 {
        let delta = (e_anomaly - e * e_anomaly.sin() - m) / (1.0 - e * e_anomaly.cos());
        e_anomaly -= delta;
        if delta.abs() < 1e-6 {
            break;
        }
    }
    e_anomaly
}

fn kepler_point(a: f32, e: f32, m: f32) -> Vec3 {
    let e_anomaly = solve_kepler(m, e);
    vec3(
        a * (e_anomaly.cos() - e),
        0.0,
        a * (1.0 - e * e).sqrt() * e_anomaly.sin(),
    )
}

fn draw_polyline(points: &[Vec3], color: Color) {
    for pair in points.windows(2) {
        draw_line_3d(pair[0], pair[1], color);
    }
}

fn orbit_path(a: f32, e: f32, segments: usize) -> Vec<Vec3> {
    let mut points = Vec::with_capacity(segments + 1);
    for i in 0..=segments {
        let m = i as f32 / segments as f32 * std::f32::consts::TAU;
        points.push(kepler_point(a, e, m));
    }
    points
}

fn draw_wireframe_sphere(center: Vec3, radius: f32, rot_y: f32, lat_steps: usize, lon_steps: usize, color: Color) {
    let segments = 40;
    for i in 1..lat_steps {
        let lat = -std::f32::consts::FRAC_PI_2 + i as f32 * std::f32::consts::PI / lat_steps as f32;
        let mut ring = Vec::with_capacity(segments + 1);
        for j in 0..=segments {
            let az = j as f32 / segments as f32 * std::f32::consts::TAU;
            let p = center + vec3(
                radius * lat.cos() * az.cos(),
                radius * lat.sin(),
                radius * lat.cos() * az.sin(),
            );
            ring.push(rotate_y(p, rot_y));
        }
        draw_polyline(&ring, color);
    }
    for i in 0..lon_steps {
        let az = i as f32 / lon_steps as f32 * std::f32::consts::PI;
        let mut meridian = Vec::with_capacity(segments + 1);
        for j in 0..=segments {
            let elev = -std::f32::consts::FRAC_PI_2 + j as f32 / segments as f32 * std::f32::consts::PI;
            let p = center + vec3(
                radius * elev.cos() * az.cos(),
                radius * elev.sin(),
                radius * elev.cos() * az.sin(),
            );
            meridian.push(rotate_y(p, rot_y));
        }
        draw_polyline(&meridian, color);
    }
}

fn draw_wireframe_box(center: Vec3, forward: Vec3, half: Vec3, color: Color) {
    let f = forward.normalize();
    let world_up = vec3(0.0, 1.0, 0.0);
    let r = if f.cross(world_up).length_squared() < 1e-6 {
        vec3(1.0, 0.0, 0.0)
    } else {
        f.cross(world_up).normalize()
    };
    let u = r.cross(f).normalize();
    let mut corners = [Vec3::ZERO; 8];
    for (i, corner) in corners.iter_mut().enumerate() {
        let sx = if i & 1 == 0 { -1.0 } else { 1.0 };
        let sy = if i & 2 == 0 { -1.0 } else { 1.0 };
        let sz = if i & 4 == 0 { -1.0 } else { 1.0 };
        *corner = center + r * (sx * half.x) + u * (sy * half.y) + f * (sz * half.z);
    }
    let edges: [(usize, usize); 12] = [
        (0, 1), (1, 3), (3, 2), (2, 0),
        (4, 5), (5, 7), (7, 6), (6, 4),
        (0, 4), (1, 5), (2, 6), (3, 7),
    ];
    for (a, b) in edges {
        draw_line_3d(corners[a], corners[b], color);
    }
}

fn draw_orbit_ring(radius: f32, color: Color) {
    let segments = 96;
    let mut ring = Vec::with_capacity(segments + 1);
    for j in 0..=segments {
        let az = j as f32 / segments as f32 * std::f32::consts::TAU;
        ring.push(vec3(radius * az.cos(), 0.0, radius * az.sin()));
    }
    draw_polyline(&ring, color);
}

fn draw_dashed_line_3d(a: Vec3, b: Vec3, dashes: usize, color: Color) {
    for i in 0..dashes {
        let t0 = i as f32 / dashes as f32;
        let t1 = (i as f32 + 0.5) / dashes as f32;
        draw_line_3d(a.lerp(b, t0), a.lerp(b, t1), color);
    }
}

fn fit_distance(radius: f32) -> f32 {
    let aspect = (screen_width() / screen_height().max(1.0)).max(0.1);
    let fov_h = 2.0 * ((FOV_Y * 0.5).tan() * aspect).atan();
    let fov_min = FOV_Y.min(fov_h);
    radius / (fov_min * 0.5).sin() * 1.15
}

fn build_cam(state: &State, cfg: &ModeCfg) -> Cam {
    let dist = fit_distance(cfg.base_radius) / state.zoom;
    let horiz = dist * state.pitch.cos();
    let pos = vec3(
        horiz * state.yaw.cos(),
        dist * state.pitch.sin(),
        horiz * state.yaw.sin(),
    );
    let target = vec3(0.0, 0.0, 0.0);
    let fwd = (target - pos).normalize();
    let up = vec3(0.0, 1.0, 0.0);
    let right = fwd.cross(up).normalize();
    let cam_up = right.cross(fwd).normalize();
    Cam {
        pos,
        right,
        up: cam_up,
        fwd,
    }
}

fn project(cam: &Cam, p: Vec3) -> Option<(f32, f32)> {
    let rel = p - cam.pos;
    let cz = rel.dot(cam.fwd);
    if cz <= 0.01 {
        return None;
    }
    let cx = rel.dot(cam.right);
    let cy = rel.dot(cam.up);
    let scale = screen_height() * 0.5 / (FOV_Y * 0.5).tan();
    Some((screen_width() * 0.5 + cx * scale / cz, screen_height() * 0.5 - cy * scale / cz))
}

fn draw_3d_label(cam: &Cam, p: Vec3, text: &str, color: Color) {
    if let Some((x, y)) = project(cam, p) {
        let dims = measure_text(text, None, 20, 1.0);
        draw_text(text, x - dims.width * 0.5, y, 20.0, color);
    }
}

fn fit_camera(dt: f32, state: &mut State) {
    let w = screen_width();
    let h = screen_height();
    let s = 56.0;
    let m = 14.0;
    let plus = Rect::new(w - s - m, h - 2.0 * s - 3.0 * m, s, s);
    let minus = Rect::new(w - s - m, h - s - m, s, s);

    if !plus.contains(mouse_position_local()) && !minus.contains(mouse_position_local()) {
        if is_mouse_button_down(MouseButton::Left) {
            let delta = mouse_delta_position();
            state.yaw -= delta.x * 1.5;
            state.pitch = (state.pitch + delta.y * 1.5).clamp(-1.2, 1.2);
        }
    } else {
        if is_mouse_button_pressed(MouseButton::Left) {
            if plus.contains(mouse_position_local()) {
                state.zoom = (state.zoom * 1.25).clamp(0.25, 6.0);
            } else {
                state.zoom = (state.zoom / 1.25).clamp(0.25, 6.0);
            }
        }
    }

    let (_, wy) = mouse_wheel();
    if wy.abs() > 0.0 {
        state.zoom = (state.zoom * (1.0 + wy * 0.12)).clamp(0.25, 6.0);
    }

    let ts = touches();
    if ts.len() == 2 {
        let dist = ts[0].position.distance(ts[1].position);
        if let Some(prev) = state.prev_pinch {
            state.zoom = (state.zoom * (1.0 + (prev - dist) * 0.01)).clamp(0.25, 6.0);
        }
        state.prev_pinch = Some(dist);
    } else {
        state.prev_pinch = None;
    }

    if is_key_pressed(KeyCode::Z) {
        state.zoom = (state.zoom * 1.25).clamp(0.25, 6.0);
    }
    if is_key_pressed(KeyCode::X) {
        state.zoom = (state.zoom / 1.25).clamp(0.25, 6.0);
    }
    if is_key_pressed(KeyCode::R) {
        state.yaw = 0.0;
        state.pitch = 0.0;
        state.zoom = 1.0;
    }

    if !is_mouse_button_down(MouseButton::Left) {
        state.yaw += dt * 0.08;
    }

    draw_rectangle_lines(plus.x, plus.y, plus.w, plus.h, 2.0, LIGHTGRAY);
    draw_rectangle_lines(minus.x, minus.y, minus.w, minus.h, 2.0, LIGHTGRAY);
    draw_text("+", plus.x + plus.w * 0.5 - 12.0, plus.y + plus.h * 0.5 + 8.0, 30.0, WHITE);
    draw_text("-", minus.x + minus.w * 0.5 - 10.0, minus.y + minus.h * 0.5 + 8.0, 30.0, WHITE);
}

fn hud(state: &State) {
    let cfg = mode_cfg(state.mode);
    let dim = Color::new(0.45, 0.45, 0.45, 1.0);
    draw_text(cfg.name, 14.0, 26.0, 22.0, WHITE);
    draw_text(cfg.eq, 14.0, 48.0, 18.0, LIGHTGRAY);
    draw_text(
        "KEYS 1-6 MODELS   R RESET   Z/X ZOOM   DRAG TILT   WHEEL/PINCH ZOOM   BUTTONS ZOOM",
        14.0,
        screen_height() - 14.0,
        16.0,
        dim,
    );
}

fn draw_binary(state: &mut State, dt: f32, cam_dim: Color) {
    let a1 = 2.5;
    let a2 = 7.5;
    draw_orbit_ring(a1, cam_dim);
    draw_orbit_ring(a2, cam_dim);

    let p1 = vec3(a1 * state.bin_angle.cos(), 0.0, a1 * state.bin_angle.sin());
    let p2 = vec3(a2 * state.bin_angle.cos(), 0.0, a2 * state.bin_angle.sin());

    let c = 0.55;
    let cross = Color::new(c, c, c, 1.0);
    draw_line_3d(vec3(-0.6, 0.0, 0.0), vec3(0.6, 0.0, 0.0), cross);
    draw_line_3d(vec3(0.0, -0.6, 0.0), vec3(0.0, 0.6, 0.0), cross);
    draw_line_3d(vec3(0.0, 0.0, -0.6), vec3(0.0, 0.0, 0.6), cross);

    draw_line_3d(p1, p2, LIGHTGRAY);
    draw_wireframe_sphere(p1, 2.2, state.spin1, 6, 6, WHITE);
    draw_wireframe_sphere(p2, 1.4, state.spin2, 5, 5, WHITE);

    state.spin1 += dt * 0.5;
    state.spin2 += dt * 0.9;
    state.bin_angle += dt;
}

fn draw_multi_planet(state: &mut State, dt: f32, cam: &Cam, cam_dim: Color) {
    let radii = [4.0, 9.0, 16.0];
    let speeds = [3.5, 2.3, 1.7];
    let mu = 5000.0;

    for r in radii.iter() {
        draw_orbit_ring(*r, cam_dim);
    }
    draw_wireframe_sphere(vec3(0.0, 0.0, 0.0), 2.5, state.planet_angles[1], 6, 6, WHITE);

    for i in 0..3 {
        let omega = speeds[i] / radii[i];
        state.planet_angles[i] += omega * dt;
        let az = state.planet_angles[i];
        let pos = vec3(radii[i] * az.cos(), 0.0, radii[i] * az.sin());
        let v = speeds[i];
        draw_wireframe_box(pos, vec3(0.0, 1.0, 0.0), vec3(0.3, 0.3, 0.3), WHITE);
        let label = format!("v = {:.1}  (sqrt({:.0}/{:.0}) = {:.1})", v, mu, radii[i], (mu / radii[i]).sqrt());
        draw_3d_label(cam, pos + vec3(0.0, 1.0, 0.0), &label, LIGHTGRAY);
    }
}

fn draw_areas(state: &mut State, dt: f32, cam_dim: Color) {
    let a = 9.0;
    let e = 0.65;
    let focus = vec3(a * e, 0.0, 0.0);

    let path = orbit_path(a, e, 160);
    let path_color = Color::new(0.35, 0.35, 0.35, 1.0);
    draw_polyline(&path, path_color);

    draw_wireframe_sphere(focus, 0.9, state.kep_angle, 4, 4, WHITE);

    state.kep_angle += dt * 0.5;
    let pos = kepler_point(a, e, state.kep_angle);

    let p1 = kepler_point(a, e, state.kep_angle - dt * 0.5);
    state.wedge_timer += dt;
    if state.wedge_timer >= 1.5 {
        state.wedge_timer = 0.0;
        state.wedges.push((p1, pos));
        if state.wedges.len() > 20 {
            state.wedges.remove(0);
        }
    }

    for (ws, we) in state.wedges.iter() {
        draw_line_3d(focus, *ws, LIGHTGRAY);
        draw_line_3d(focus, *we, LIGHTGRAY);
        for i in 0..6 {
            let t0 = i as f32 / 6.0;
            let t1 = (i as f32 + 1.0) / 6.0;
            let mid0 = ws.lerp(*we, t0);
            let mid1 = ws.lerp(*we, t1);
            draw_line_3d(mid0, mid1, cam_dim);
        }
    }

    draw_wireframe_box(pos, -pos.normalize(), vec3(0.25, 0.25, 0.45), WHITE);
}

fn draw_energy(state: &mut State, dt: f32, cam_dim: Color) {
    let mu = 100.0;
    draw_wireframe_sphere(vec3(0.0, 0.0, 0.0), 3.0, 0.0, 6, 6, LIGHTGRAY);

    let substeps = 6;
    let h = (dt / substeps as f32).min(0.016);
    for i in 0..3 {
        for _ in 0..substeps {
            let r2 = state.e_pos[i].length_squared();
            let r = r2.sqrt().max(1.0);
            let acc = -state.e_pos[i] * (mu / (r2 * r));
            state.e_vel[i] += acc * h;
            state.e_pos[i] += state.e_vel[i] * h;
        }
        state.e_trails[i].push(state.e_pos[i]);
        if state.e_trails[i].len() > 2400 {
            state.e_trails[i].remove(0);
        }
    }

    let trail_colors = [
        Color::new(0.5, 0.5, 0.5, 1.0),
        Color::new(0.75, 0.75, 0.75, 1.0),
        WHITE,
    ];
    for i in 0..3 {
        draw_polyline(&state.e_trails[i], trail_colors[i]);
    }
    draw_line_3d(state.e_pos[0], vec3(0.0, 0.0, 0.0), cam_dim);
}

fn energy_readout(state: &State) {
    let mu = 100.0;
    let labels = ["A E < 0", "B E = 0", "C E > 0"];
    let dim = Color::new(0.5, 0.5, 0.5, 1.0);
    draw_text("LIVE ENERGY (E = v^2/2 - GM/r):", 14.0, 78.0, 18.0, LIGHTGRAY);
    for i in 0..3 {
        let r = state.e_pos[i].length();
        let e = state.e_vel[i].length_squared() * 0.5 - mu / r;
        let color = if e < -0.01 { dim } else if e > 0.01 { WHITE } else { LIGHTGRAY };
        let line = format!("{}   E = {:.3}   r = {:.1}", labels[i], e, r);
        draw_text(&line, 14.0, 100.0 + 22.0 * i as f32, 18.0, color);
    }
}

fn draw_geo_leo(state: &mut State, dt: f32, cam: &Cam, cam_dim: Color) {
    let earth_r = 3.0;
    let omega_earth = std::f32::consts::TAU / 10.0;
    let omega_leo = std::f32::consts::TAU / 1.5;
    let r_leo = 4.2;
    let r_geo = 10.5;

    state.earth_rot += omega_earth * dt;
    state.leo_angle += omega_leo * dt;
    state.geo_angle += omega_earth * dt;

    draw_wireframe_sphere(vec3(0.0, 0.0, 0.0), earth_r, state.earth_rot, 8, 8, LIGHTGRAY);

    let meridian_dir = vec3(state.earth_rot.sin(), 0.0, state.earth_rot.cos());
    draw_line_3d(vec3(0.0, 0.0, 0.0), meridian_dir * earth_r, WHITE);
    draw_line_3d(meridian_dir * earth_r, meridian_dir * (earth_r + 2.0), cam_dim);

    let leo_local = rotate_y(vec3(0.0, r_leo, 0.0), state.leo_angle);
    let leo_pos = rotate_x(leo_local, 1.15);
    draw_wireframe_box(leo_pos, -leo_pos.normalize(), vec3(0.22, 0.22, 0.32), WHITE);
    draw_line_3d(vec3(0.0, 0.0, 0.0), leo_pos, cam_dim);
    draw_3d_label(cam, leo_pos + vec3(0.0, 0.8, 0.0), "LEO  T = 1.5 s", LIGHTGRAY);

    let geo_pos = meridian_dir * r_geo;
    draw_wireframe_box(geo_pos, -geo_pos.normalize(), vec3(0.3, 0.3, 0.4), WHITE);
    draw_dashed_line_3d(meridian_dir * earth_r, geo_pos, 12, WHITE);
    draw_3d_label(cam, geo_pos + vec3(0.0, 0.9, 0.0), "GEO  T = 10 s", WHITE);
}

fn rotate_x(v: Vec3, a: f32) -> Vec3 {
    let (s, c) = a.sin_cos();
    vec3(v.x, v.y * c - v.z * s, v.y * s + v.z * c)
}

fn draw_comet(state: &mut State, dt: f32, cam: &Cam, cam_dim: Color) {
    let a = 12.0;
    let e = 0.85;
    let focus = vec3(a * e, 0.0, 0.0);

    let path = orbit_path(a, e, 192);
    let path_color = Color::new(0.35, 0.35, 0.35, 1.0);
    draw_polyline(&path, path_color);

    draw_wireframe_sphere(focus, 1.2, state.comet_angle, 4, 4, WHITE);

    state.comet_angle += dt * 0.35;
    let pos = kepler_point(a, e, state.comet_angle);

    let r = kepler_point(a, e, state.comet_angle).length();
    let v_mag = (a * (1.0 - e * e)).sqrt() * (2.0 / r - 1.0 / a).sqrt();
    let tail_dir = (pos - focus).normalize();
    let tail_len = 2.0 + v_mag * 0.35;
    let tail_end = pos - tail_dir * tail_len;
    draw_line_3d(pos, tail_end, LIGHTGRAY);
    let t = tail_dir.cross(vec3(0.0, 1.0, 0.0)).normalize();
    for side in [-1.0f32, 1.0] {
        draw_line_3d(
            pos + t * (side * 0.25),
            tail_end + t * (side * 0.18),
            cam_dim,
        );
    }

    draw_wireframe_box(pos, tail_dir, vec3(0.2, 0.2, 0.3), WHITE);
    draw_3d_label(cam, tail_end + vec3(0.0, 0.7, 0.0), "COMET", LIGHTGRAY);
}

fn hud_model_4(state: &State) {
    energy_readout(state);
}

fn window_conf() -> Conf {
    Conf {
        window_title: "1963 Retro Orbit Visualizer".to_string(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = State {
        mode: Mode::BinaryStars,
        yaw: 0.4,
        pitch: 0.35,
        zoom: 1.0,
        bin_angle: 0.0,
        spin1: 0.0,
        spin2: 0.0,
        planet_angles: [0.0, 0.0, 0.0],
        kep_angle: 0.0,
        wedge_timer: 0.0,
        wedges: Vec::new(),
        e_pos: [Vec3::ZERO; 3],
        e_vel: [Vec3::ZERO; 3],
        e_trails: [Vec::new(), Vec::new(), Vec::new()],
        earth_rot: 0.0,
        leo_angle: 0.0,
        geo_angle: 0.0,
        comet_angle: 0.0,
        prev_pinch: None,
    };
    reset_mode(&mut state, Mode::BinaryStars);

    let cam_dim = Color::new(0.4, 0.4, 0.4, 1.0);

    loop {
        clear_background(BLACK);

        let dt = get_frame_time().min(0.1);

        if is_key_pressed(KeyCode::Key1) {
            reset_mode(&mut state, Mode::BinaryStars);
        }
        if is_key_pressed(KeyCode::Key2) {
            reset_mode(&mut state, Mode::MultiPlanet);
        }
        if is_key_pressed(KeyCode::Key3) {
            reset_mode(&mut state, Mode::KeplerAreas);
        }
        if is_key_pressed(KeyCode::Key4) {
            reset_mode(&mut state, Mode::Energy);
        }
        if is_key_pressed(KeyCode::Key5) {
            reset_mode(&mut state, Mode::GeoLeo);
        }
        if is_key_pressed(KeyCode::Key6) {
            reset_mode(&mut state, Mode::Comet);
        }

        fit_camera(dt, &mut state);

        let cfg = mode_cfg(state.mode);
        let cam = build_cam(&state, &cfg);
        set_camera(&Camera3D {
            position: cam.pos,
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            fovy: FOV_Y.to_degrees(),
            ..Default::default()
        });

        match state.mode {
            Mode::BinaryStars => draw_binary(&mut state, dt, cam_dim),
            Mode::MultiPlanet => draw_multi_planet(&mut state, dt, &cam, cam_dim),
            Mode::KeplerAreas => draw_areas(&mut state, dt, cam_dim),
            Mode::Energy => draw_energy(&mut state, dt, cam_dim),
            Mode::GeoLeo => draw_geo_leo(&mut state, dt, &cam, cam_dim),
            Mode::Comet => draw_comet(&mut state, dt, &cam, cam_dim),
        }

        set_default_camera();

        hud(&state);
        if state.mode == Mode::Energy {
            hud_model_4(&state);
        }

        next_frame().await
    }
}