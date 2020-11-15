#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

const EARTH_SECONDS: f64 = 31_557_600.0;

pub trait Planet {
    const PLANETARY_YEAR: f64;

    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / (EARTH_SECONDS * (Self::PLANETARY_YEAR))
    }
}

macro_rules! impl_planets {
    ($(($planet: ident, $constant: expr)),*) => {$(
        pub struct $planet;

        impl Planet for $planet{
            const PLANETARY_YEAR: f64 = $constant;
        }
    )*};
}

impl_planets!{(Mercury,0.2408467),(Venus,0.61519726),(Earth,1.0),(Mars,1.8808158),(Jupiter,11.862615),(Saturn,29.447498),(Uranus,84.016846),(Neptune,164.79132)}