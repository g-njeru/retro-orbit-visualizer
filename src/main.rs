use macroquad::prelude::*;

const PLANET_RADIUS: f32 = 1.0;
const SEMI_MAJOR: f32 = 4.0;
const ECCENTRICITY: f32 = 0.5;
const INCLINATION: f32 = 0.5;
const ARG_PERIAPSIS: f32 = 0.7;
const MEAN_MOTION: f32 = 0.6;
const SEGMENTS: usize = 128;

fn rotate_x(v: Vec3, a: f32) -> Vec3 {
    let (s, c) = a.sin_cos();
    vec3(v.x, v.y * c - v.z * s, v.y * s + v.z * c)
}

fn rotate_y(v: Vec3, a: f32) -> Vec3 {
    let (s, c) = a.sin_cos();
    vec3(v.x * c + v.z * s, v.y, -v.x * s + v.z * c)
}

fn rotate_z(v: Vec3, a: f32) -> Vec3 {
    let (s, c) = a.sin_cos();
    vec3(v.x * c - v.y * s, v.x * s + v.y * c, v.z)
}

fn to_world(perifocal: Vec3) -> Vec3 {
    let p = rotate_z(perifocal, ARG_PERIAPSIS);
    rotate_x(p, INCLINATION)
}

fn solve_kepler(mean_anomaly: f32) -> f32 {
    let mut e_anomaly = mean_anomaly;
    for _ in 0..16 {
        let f = e_anomaly - ECCENTRICITY * e_anomaly.sin() - mean_anomaly;
        let fp = 1.0 - ECCENTRICITY * e_anomaly.cos();
        let delta = f / fp;
        e_anomaly -= delta;
        if delta.abs() < 1e-6 {
            break;
        }
    }
    e_anomaly
}

fn satellite_position(mean_anomaly: f32) -> Vec3 {
    let e_anomaly = solve_kepler(mean_anomaly);
    let a = SEMI_MAJOR;
    let e = ECCENTRICITY;
    let x = a * (e_anomaly.cos() - e);
    let y = a * (1.0 - e * e).sqrt() * e_anomaly.sin();
    to_world(vec3(x, y, 0.0))
}

fn orbit_path() -> Vec<Vec3> {
    let mut points = Vec::with_capacity(SEGMENTS + 1);
    for i in 0..=SEGMENTS {
        let e_anomaly = i as f32 / SEGMENTS as f32 * std::f32::consts::TAU;
        let x = SEMI_MAJOR * (e_anomaly.cos() - ECCENTRICITY);
        let y = SEMI_MAJOR * (1.0 - ECCENTRICITY * ECCENTRICITY).sqrt() * e_anomaly.sin();
        points.push(to_world(vec3(x, y, 0.0)));
    }
    points
}

fn draw_polyline(points: &[Vec3], color: Color) {
    for pair in points.windows(2) {
        draw_line_3d(pair[0], pair[1], color);
    }
}

fn draw_wire_planet(color: Color) {
    let lat_steps = 7;
    let lon_steps = 6;
    let ring_segments = 48;
    for i in 1..lat_steps {
        let lat = -std::f32::consts::FRAC_PI_2 + i as f32 * std::f32::consts::PI / lat_steps as f32;
        let ring_radius = PLANET_RADIUS * lat.cos();
        let height = PLANET_RADIUS * lat.sin();
        let mut ring = Vec::with_capacity(ring_segments + 1);
        for j in 0..=ring_segments {
            let angle = j as f32 / ring_segments as f32 * std::f32::consts::TAU;
            ring.push(vec3(ring_radius * angle.cos(), height, ring_radius * angle.sin()));
        }
        draw_polyline(&ring, color);
    }
    for i in 0..lon_steps {
        let lon = i as f32 / lon_steps as f32 * std::f32::consts::PI;
        let mut meridian = Vec::with_capacity(ring_segments + 1);
        for j in 0..=ring_segments {
            let angle = j as f32 / ring_segments as f32 * std::f32::consts::TAU;
            let p = vec3(PLANET_RADIUS * angle.cos(), PLANET_RADIUS * angle.sin(), 0.0);
            meridian.push(rotate_y(p, lon));
        }
        draw_polyline(&meridian, color);
    }
}

fn draw_oriented_box(center: Vec3, forward: Vec3, half: Vec3, color: Color) {
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
        (0, 1),
        (1, 3),
        (3, 2),
        (2, 0),
        (4, 5),
        (5, 7),
        (7, 6),
        (6, 4),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];
    for (a, b) in edges {
        draw_line_3d(corners[a], corners[b], color);
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "1963 Retro Orbit Visualizer".to_string(),
        high_dpi: true,
        ..Default::default()
    }
}

fn camera_distance() -> f32 {
    let scene_radius = SEMI_MAJOR * (1.0 + ECCENTRICITY) + 0.5;
    let fov_y = 50f32.to_radians();
    let aspect = (screen_width() / screen_height().max(1.0)).max(0.1);
    let fov_h = 2.0 * ((fov_y / 2.0).tan() * aspect).atan();
    let fov_min = fov_y.min(fov_h);
    scene_radius / (fov_min * 0.5).sin() * 1.15
}

#[macroquad::main(window_conf)]
async fn main() {
    let orbit = orbit_path();
    let mut mean_anomaly = 0.0_f32;
    let mut camera_angle = 0.0_f32;

    loop {
        clear_background(BLACK);

        let dt = get_frame_time();
        mean_anomaly = (mean_anomaly + MEAN_MOTION * dt) % std::f32::consts::TAU;
        camera_angle += dt * 0.1;

        let cam_radius = camera_distance();
        set_camera(&Camera3D {
            position: vec3(
                cam_radius * camera_angle.cos(),
                cam_radius * 0.35,
                cam_radius * camera_angle.sin(),
            ),
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            fovy: 50.0,
            ..Default::default()
        });

        draw_wire_planet(LIGHTGRAY);

        let path_color = Color::new(0.4, 0.4, 0.4, 1.0);
        draw_polyline(&orbit, path_color);

        let sat_pos = satellite_position(mean_anomaly);
        let gradient_dir = -sat_pos.normalize();
        draw_line_3d(sat_pos, vec3(0.0, 0.0, 0.0), path_color);
        draw_oriented_box(sat_pos, gradient_dir, vec3(0.12, 0.12, 0.35), WHITE);

        set_default_camera();

        draw_text(
            &format!("ECCENTRICITY {:.2}", ECCENTRICITY),
            12.0,
            24.0,
            24.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
