#[derive(Debug)]
pub struct Duration(u64);

const EARTH_SECONDS: u64 = 31_557_600;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        // Mercury: orbital period 0.2408467 Earth years
        d.0 as f64 / (0.2408467 * EARTH_SECONDS as f64)
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        // Venus: orbital period 0.61519726 Earth years
        d.0 as f64 / (0.61519726 * EARTH_SECONDS as f64)
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        // Earth: orbital period 1.0 Earth years, 365.25 Earth days, or 31557600 seconds
        d.0 as f64 /1.0* EARTH_SECONDS as f64
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        // Mars: orbital period 1.8808158 Earth years
        d.0 as f64 / (1.8808158 * EARTH_SECONDS as f64)
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        // Jupiter: orbital period 11.862615 Earth years
        d.0 as f64 / (11.862615 * EARTH_SECONDS as f64)
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        // Saturn: orbital period 29.447498 Earth years
        d.0 as f64 / (29.447498 * EARTH_SECONDS as f64)
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        // Uranus: orbital period 84.016846 Earth years
        d.0 as f64 / (84.016846 * EARTH_SECONDS as f64)
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        // Neptune: orbital period 164.79132 Earth years
        d.0 as f64 / (164.79132 * EARTH_SECONDS as f64)
    }
}