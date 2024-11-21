#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s }
    }
}

const EARTH_YEAR_SECONDS: f64 = 31557600.0;

pub trait Planet {
    const PLANET_YEAR_SECONDS: f64;
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / Self::PLANET_YEAR_SECONDS
    }
}

macro_rules! planet {
    ($i:ident, $n:literal) => {
        pub struct $i;
        impl Planet for $i {
            const PLANET_YEAR_SECONDS: f64 = EARTH_YEAR_SECONDS * $n;
        }
    };
}

planet!(Earth, 1.0);
planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
