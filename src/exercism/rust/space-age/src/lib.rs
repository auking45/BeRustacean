// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_SECS: f64 = 31557600.0;

#[derive(Debug)]
pub struct Duration {
    secs: f64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self {
            secs: s as f64
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        unimplemented!(
            "convert a duration ({d:?}) to the number of years on this planet for that duration"
        );
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

macro_rules! impl_trait2 {
    ($(($t:ty, $p:literal)),+) => {
        $(impl Planet for $t {
            fn years_during(d: &Duration) -> f64 {
                d.secs / EARTH_SECS / $p
            }
        })*
    }
}

impl_trait2!(
    (Mercury, 0.2408467),
    (Venus, 0.61519726),
    (Earth, 1.0),
    (Mars, 1.8808158),
    (Jupiter, 11.862615),
    (Saturn, 29.447498),
    (Uranus, 84.016846),
    (Neptune, 164.79132)
);
