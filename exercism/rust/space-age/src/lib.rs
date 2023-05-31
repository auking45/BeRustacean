// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_SECS: f64 = 31557600.0;

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s as f64)
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

impl Mercury {
    fn seconds() -> f64 {
        0.2408467 * EARTH_SECS
    }
}

impl Venus {
    fn seconds() -> f64 {
        0.61519726 * EARTH_SECS
    }
}

impl Earth {
    fn seconds() -> f64 {
        EARTH_SECS
    }
}

impl Mars {
    fn seconds() -> f64 {
        1.8808158 * EARTH_SECS
    }
}

impl Jupiter {
    fn seconds() -> f64 {
        11.862615 * EARTH_SECS
    }
}

impl Saturn {
    fn seconds() -> f64 {
        29.447498 * EARTH_SECS
    }
}

impl Uranus {
    fn seconds() -> f64 {
        84.016846 * EARTH_SECS
    }
}

impl Neptune {
    fn seconds() -> f64 {
        164.79132 * EARTH_SECS
    }
}

macro_rules! impl_trait {
    ($($t:ty),+) => {
        $(impl Planet for $t {
            fn years_during(d: &Duration) -> f64 {
                d.0 / <$t>::seconds()
            }
        })*
    }
}

impl_trait!(Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, Neptune);
