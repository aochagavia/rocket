use std::time::Duration;

use crate::{
    geometry::{Point, Vector},
    models::Particle,
};

/// Converts a duration to seconds
pub fn duration_to_seconds(d: Duration) -> f32 {
    d.as_secs() as f32 + d.subsec_nanos() as f32 * 1e-9
}

/// Optimized version of `Vec::retain`
///
/// We achieve better performance by renouncing to keep the original order of the `Vec`
pub fn fast_retain<T, F>(vec: &mut Vec<T>, mut f: F)
where
    F: FnMut(&T) -> bool,
{
    let mut i = 0;
    while i < vec.len() {
        if !f(&vec[i]) {
            vec.swap_remove(i);
        } else {
            i += 1;
        }
    }
}

/// Generates a new explosion of the given intensity at the given position.
/// This works best with values between 5 and 25
pub fn make_explosion(particles: &mut Vec<Particle>, position: &Point, intensity: u8) {
    for rotation in itertools_num::linspace(0.0, 2.0 * ::std::f32::consts::PI, 30) {
        for ttl in (1..intensity).map(|x| (x as f32) / 10.0) {
            particles.push(Particle::new(Vector::new(position.clone(), rotation), ttl));
        }
    }
}

#[test]
fn test_fast_retain() {
    let mut xs = vec![42; 100];
    fast_retain(&mut xs, |&x| x < 42);
    assert_eq!(xs.len(), 0);
}

#[test]
fn test_duration_to_seconds() {
    let d = Duration::from_millis(1500);
    assert_eq!(1.5, duration_to_seconds(d));
}
