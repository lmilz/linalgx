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
    /// Returns the intersection distance and color if the ray hits the Object
    fn intersect(&self, ray: &Ray) -> Option<(f64, Color)> {
        match self {
            Object::Sphere {
                center,
                radius,
                color,
            } => {
                let vector_to_center = ray.origin - *center;

                // Solve the quadratic equation for ray-sphere intersection
                let a = ray.direction.dot(&ray.direction);
                let b = 2.0 * vector_to_center.dot(&ray.direction);
                let c = vector_to_center.dot(&vector_to_center) - radius * radius;

                let discriminant = b * b - 4.0 * a * c;

                // If discriminant is non-negative, compute intersection distance
                (discriminant >= 0.0)
                    .then(|| {
                        let distance = (-b - discriminant.sqrt()) / (2.0 * a);
                        (distance, *color)
                    })
                    .filter(|(distance, _)| *distance > 0.0)
            }

            Object::Square {
                center,
                size,
                color,
            } => {
                let ray_z = ray.direction.data[2];

                // Avoid intersection if ray is parallel to square's plane
                if ray_z.abs() < 1e-6 {
                    return None;
                }

                // Calculate intersection with square's plane
                let t = (center.data[2] - ray.origin.data[2]) / ray_z;
                let hit_position = ray.origin + ray.direction * t;
                let half_size = size / 2.0;

                // Check if intersection point is within square bounds
                (t > 0.0
                    && (center.data[0] - half_size..=center.data[0] + half_size)
                        .contains(&hit_position.data[0])
                    && (center.data[1] - half_size..=center.data[1] + half_size)
                        .contains(&hit_position.data[1]))
                .then_some((t, *color))
            }
        }
    }
}

/// Casts a ray into the scene and returns the color of the nearest hit object
fn cast_ray(ray: &Ray, scene_objects: &[Object], background_color: Color) -> Color {
    scene_objects
        .iter()
        .filter_map(|object| object.intersect(ray))
        .min_by(|(distance1, _), (distance2, _)| distance1.partial_cmp(distance2).unwrap())
        .map(|(_, color)| color)
        .unwrap_or(background_color)
}

/// Renders the scene to an image buffer
fn render(
    image_width: u32,
    image_height: u32,
    scene_objects: &[Object],
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let camera_origin = Vec3::new([0.0, 0.0, -1.0]);
    let background_color = (30, 30, 40);

    ImageBuffer::from_fn(image_width, image_height, |pixel_x, pixel_y| {
        // Convert pixel coordinate to viewport space (-1.0 to 1.0)
        let normalized_x = (pixel_x as f64 / image_width as f64) * 2.0 - 1.0;
        let normalized_y = 1.0 - (pixel_y as f64 / image_height as f64) * 2.0;

        // Generate ray direction through the viewport pixel
        let ray_direction = Vec3::new([normalized_x, normalized_y, 1.0]);
        let ray = Ray {
            origin: camera_origin,
            direction: ray_direction,
        };

        // Determine color seen along the ray
        let (red, green, blue) = cast_ray(&ray, scene_objects, background_color);
        Rgb([red, green, blue])
    })
}

/// Main entry point of the program
fn main() {
    let image_width = 400;
    let image_height = 400;
    let scene_objects = vec![
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

    let image = render(image_width, image_height, &scene_objects);
    image.save("output.png").expect("Failed to save image");
}
