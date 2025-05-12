use rand::seq::IndexedRandom;
use serde::Serialize;

static WEB_TIMEZONES: &[&str] = &[
    "America/Cambridge_Bay",
    "America/Campo_Grande",
    "America/Montevideo",
    "America/Montserrat",
    "America/Nassau",
    "America/New_York",
    "America/Nome",
    "UTC",
];

#[derive(Serialize, Debug)]
pub struct WebTimezones(&'static str);

impl WebTimezones {
    pub fn random() -> Result<Self, &'static str> {
        let mut rng = rand::rng();
        let res = WEB_TIMEZONES.choose(&mut rng).unwrap();
        Ok(WebTimezones(res))
    }
}
