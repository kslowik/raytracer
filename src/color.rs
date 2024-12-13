use crate::vec3::Vec3;
use std::io;

pub type Color = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color(buffer: &mut Vec<u8>, pixel_color: Color) -> io::Result<()> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    r = r.clamp(0.0, 0.999);
    g = g.clamp(0.0, 0.999);
    b = b.clamp(0.0, 0.999);

    let rbyte = (256.0 * r) as u8;
    let gbyte = (256.0 * g) as u8;
    let bbyte = (256.0 * b) as u8;

    // Append the pixel color components to the buffer.
    buffer.push(rbyte);
    buffer.push(gbyte);
    buffer.push(bbyte);

    Ok(())
}

#[test]
fn test_linear_to_gamma() {
    assert_eq!(linear_to_gamma(0.0), 0.0);
    assert_eq!(linear_to_gamma(1.0), 1.0);
    assert_eq!(linear_to_gamma(0.25), 0.5);
}

#[test]
fn test_write_color() {
    let mut buffer = Vec::new();
    let pixel_color = Color::new(0.5, 0.25, 0.75);
    write_color(&mut buffer, pixel_color).unwrap();
    assert_eq!(buffer, vec![181, 128, 221]);
}
