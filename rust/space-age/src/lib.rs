#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64)
    }
}

pub trait Planet {
    const SECONDS_IN_EARTH_YEAR: f64 = 31557600.0;
    const ORBITAL_PERIOD: f64;

    fn years_during(d: &Duration) -> f64 {
        (d.0/Self::SECONDS_IN_EARTH_YEAR) / Self::ORBITAL_PERIOD
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

macro_rules! planet {
    ($name:ident, $period:literal) => {
       impl Planet for $name {
           const ORBITAL_PERIOD: f64 = $period;
       } 
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
