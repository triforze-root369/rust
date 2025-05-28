use rayon::prelude::*;
use std::{env, fs::File, io::Write, path::Path};

// Vector mathematics module
mod vec3 {
    use std::ops::{Add, Mul, Sub};

    #[derive(Debug, Clone, Copy)]
    pub struct Vec3 {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    impl Vec3 {
        pub fn new(x: f64, y: f64, z: f64) -> Self {
            Vec3 { x, y, z }
        }

        pub fn zero() -> Self {
            Vec3::new(0.0, 0.0, 0.0)
        }

        pub fn dot(&self, other: &Vec3) -> f64 {
            self.x * other.x + self.y * other.y + self.z * other.z
        }

        pub fn length(&self) -> f64 {
            self.dot(self).sqrt()
        }

        pub fn normalize(&self) -> Vec3 {
            let len = self.length();
            if len > 0.0 {
                Vec3::new(self.x / len, self.y / len, self.z / len)
            } else {
                *self
            }
        }

        pub fn scale(&self, t: f64) -> Vec3 {
            Vec3::new(self.x * t, self.y * t, self.z * t)
        }

        pub fn cross(&self, other: &Vec3) -> Vec3 {
            Vec3::new(
                self.y * other.z - self.z * other.y,
                self.z * other.x - self.x * other.z,
                self.x * other.y - self.y * other.x,
            )
        }
    }

    impl Add for Vec3 {
        type Output = Vec3;

        fn add(self, other: Vec3) -> Vec3 {
            Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
        }
    }

    impl Sub for Vec3 {
        type Output = Vec3;

        fn sub(self, other: Vec3) -> Vec3 {
            Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
        }
    }

    impl Mul<f64> for Vec3 {
        type Output = Vec3;

        fn mul(self, rhs: f64) -> Vec3 {
            self.scale(rhs)
        }
    }
}

// Ray structure
#[derive(Debug)]
struct Ray {
    origin: vec3::Vec3,
    direction: vec3::Vec3,
}

impl Ray {
    fn new(origin: vec3::Vec3, direction: vec3::Vec3) -> Self {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }

    fn point_at(&self, t: f64) -> vec3::Vec3 {
        self.origin + self.direction.scale(t)
    }
}

// Sphere structure
struct Sphere {
    center: vec3::Vec3,
    radius: f64,
    color: vec3::Vec3,
}

impl Sphere {
    fn new(center: vec3::Vec3, radius: f64, color: vec3::Vec3) -> Self {
        Sphere {
            center,
            radius,
            color,
        }
    }

    // Returns the intersection point distance if the ray hits the sphere
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t > 0.0 {
                Some(t)
            } else {
                let t = (-b + discriminant.sqrt()) / (2.0 * a);
                if t > 0.0 {
                    Some(t)
                } else {
                    None
                }
            }
        }
    }

    fn normal_at(&self, point: vec3::Vec3) -> vec3::Vec3 {
        (point - self.center).normalize()
    }
}

// Camera structure
struct Camera {
    position: vec3::Vec3,
    direction: vec3::Vec3,
    up: vec3::Vec3,
    fov: f64,
    aspect_ratio: f64,
}

impl Camera {
    fn new(position: vec3::Vec3, direction: vec3::Vec3, up: vec3::Vec3, fov: f64, aspect_ratio: f64) -> Self {
        Camera {
            position,
            direction: direction.normalize(),
            up: up.normalize(),
            fov,
            aspect_ratio,
        }
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray {
        let theta = self.fov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = self.aspect_ratio * half_height;

        let w = (self.direction * -1.0).normalize();
        let u_vec = self.up.cross(&w).normalize();
        let v_vec = w.cross(&u_vec);

        let lower_left = self.position - u_vec.scale(half_width) - v_vec.scale(half_height) - w;
        let horizontal = u_vec.scale(2.0 * half_width);
        let vertical = v_vec.scale(2.0 * half_height);

        let direction = lower_left - self.position + horizontal.scale(u) + vertical.scale(v);
        Ray::new(self.position, direction)
    }
}

// Scene structure
struct Scene {
    spheres: Vec<Sphere>,
    light_pos: vec3::Vec3,
    light_intensity: f64,
    ambient_intensity: f64,
}

impl Scene {
    fn new(spheres: Vec<Sphere>, light_pos: vec3::Vec3, light_intensity: f64, ambient_intensity: f64) -> Self {
        Scene {
            spheres,
            light_pos,
            light_intensity,
            ambient_intensity,
        }
    }

    fn trace(&self, ray: &Ray) -> vec3::Vec3 {
        let mut closest_intersection: Option<(f64, &Sphere)> = None;

        // Find the closest intersection
        for sphere in &self.spheres {
            if let Some(t) = sphere.intersect(ray) {
                match closest_intersection {
                    None => closest_intersection = Some((t, sphere)),
                    Some((closest_t, _)) if t < closest_t => closest_intersection = Some((t, sphere)),
                    _ => {}
                }
            }
        }

        // If we hit something, calculate the color
        if let Some((t, sphere)) = closest_intersection {
            let hit_point = ray.point_at(t);
            let normal = sphere.normal_at(hit_point);
            let to_light = (self.light_pos - hit_point).normalize();

            // Check for shadows
            let shadow_ray = Ray::new(hit_point + normal.scale(0.001), to_light);
            let in_shadow = self.spheres.iter().any(|s| s.intersect(&shadow_ray).is_some());

            if in_shadow {
                sphere.color.scale(self.ambient_intensity)
            } else {
                let diffuse = normal.dot(&to_light).max(0.0) * self.light_intensity;
                sphere.color.scale(diffuse + self.ambient_intensity)
            }
        } else {
            // Background color (black)
            vec3::Vec3::zero()
        }
    }
}

fn clamp(x: f64) -> u8 {
    (x.max(0.0).min(1.0) * 255.0) as u8
}

fn main() -> std::io::Result<()> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <width> <height>", args[0]);
        std::process::exit(1);
    }

    let width: usize = args[1].parse().expect("Invalid width");
    let height: usize = args[2].parse().expect("Invalid height");

    // Set up the scene
    let scene = Scene::new(
        vec![
            Sphere::new(
                vec3::Vec3::new(0.0, 0.0, -5.0),
                1.0,
                vec3::Vec3::new(1.0, 0.2, 0.2),
            ),
            Sphere::new(
                vec3::Vec3::new(2.0, 0.0, -6.0),
                1.0,
                vec3::Vec3::new(0.2, 1.0, 0.2),
            ),
            Sphere::new(
                vec3::Vec3::new(-2.0, 0.0, -4.0),
                1.0,
                vec3::Vec3::new(0.2, 0.2, 1.0),
            ),
        ],
        vec3::Vec3::new(5.0, 5.0, 5.0), // Light position
        1.0,                            // Light intensity
        0.1,                            // Ambient intensity
    );

    let camera = Camera::new(
        vec3::Vec3::new(0.0, 0.0, 0.0),
        vec3::Vec3::new(0.0, 0.0, -1.0),
        vec3::Vec3::new(0.0, 1.0, 0.0),
        90.0,
        width as f64 / height as f64,
    );

    // Create and open the output file
    let path = Path::new("output.ppm");
    let mut file = File::create(path)?;

    // Write PPM header
    writeln!(file, "P3\n{} {}\n255", width, height)?;

    // Render the image
    let pixels: Vec<Vec<vec3::Vec3>> = (0..height)
        .into_par_iter()
        .map(|j| {
            let mut row = Vec::with_capacity(width);
            for i in 0..width {
                let u = i as f64 / (width - 1) as f64;
                let v = 1.0 - (j as f64 / (height - 1) as f64);
                let ray = camera.get_ray(u, v);
                row.push(scene.trace(&ray));
            }
            println!("Rendering row {}/{}", j + 1, height);
            row
        })
        .collect();

    // Write pixels to file
    for row in pixels {
        for color in row {
            writeln!(
                file,
                "{} {} {}",
                clamp(color.x),
                clamp(color.y),
                clamp(color.z)
            )?;
        }
    }

    println!("Rendering complete! Output saved to output.ppm");
    Ok(())
} 