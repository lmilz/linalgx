use image::{ImageBuffer, Rgb};
use linalgx::vector::Vec3;

type Color = (u8, u8, u8);

#[derive(Clone, Copy)]
struct Ray {
    origin: Vec3<f64>,
    direction: Vec3<f64>,
}

#[derive(Clone, Copy)]
enum Object {
    Sphere {
        center: Vec3<f64>,
        radius: f64,
        color: Color,
    },
    Square {
        center: Vec3<f64>,
        size: f64,
        color: Color,
    },
}

impl Object {
    fn intersect(&self, ray: &Ray) -> Option<(f64, Color)> {
        match self {
            Object::Sphere {
                center,
                radius,
                color,
            } => {
                let oc = ray.origin - *center;
                let a = ray.direction.dot(&ray.direction);
                let b = oc.dot(&ray.direction) * 2.0;
                let c = oc.dot(&oc) - radius * radius;
                let disc = b * b - 4.0 * a * c;

                (disc >= 0.0)
                    .then(|| {
                        let t = (-b - disc.sqrt()) / (2.0 * a);
                        (t, *color)
                    })
                    .filter(|(t, _)| *t > 0.0)
            }

            Object::Square {
                center,
                size,
                color,
            } => {
                let z_dir = ray.direction.data[2];
                if z_dir.abs() < 1e-6 {
                    return None;
                }

                let t = (center.data[2] - ray.origin.data[2]) / z_dir;
                let hit = ray.origin + ray.direction * t;
                let half = size / 2.0;

                (t > 0.0
                    && (center.data[0] - half..=center.data[0] + half).contains(&hit.data[0])
                    && (center.data[1] - half..=center.data[1] + half).contains(&hit.data[1]))
                .then_some((t, *color))
            }
        }
    }
}

fn cast_ray(ray: &Ray, scene: &[Object], background: Color) -> Color {
    scene
        .iter()
        .filter_map(|object| object.intersect(ray))
        .min_by(|(t1, _), (t2, _)| t1.partial_cmp(t2).unwrap())
        .map(|(_, color)| color)
        .unwrap_or(background)
}

fn render(width: u32, height: u32, scene: &[Object]) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let camera = Vec3::new([0.0, 0.0, -1.0]);
    let background = (30, 30, 40);

    ImageBuffer::from_fn(width, height, |x, y| {
        let u = (x as f64 / width as f64) * 2.0 - 1.0;
        let v = 1.0 - (y as f64 / height as f64) * 2.0;
        let direction = Vec3::new([u, v, 1.0]);
        let ray = Ray {
            origin: camera,
            direction,
        };
        let (r, g, b) = cast_ray(&ray, scene, background);
        Rgb([r, g, b])
    })
}

fn main() {
    let scene = vec![
        Object::Sphere {
            center: Vec3::new([0.3, 0.0, 2.5]),
            radius: 0.4,
            color: (255, 0, 0),
        },
        Object::Square {
            center: Vec3::new([-0.5, 0.0, 2.0]),
            size: 0.8,
            color: (0, 255, 0),
        },
    ];

    let image = render(400, 400, &scene);
    image.save("output.png").expect("Failed to save image");
}
