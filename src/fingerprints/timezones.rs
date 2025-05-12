use rand::seq::IndexedRandom;
use serde::Serialize;
static TIMEZONES: &[&str] = &["GMT-05:00", "GMT+00:00", "GMT+01:00"];
#[derive(Serialize, Debug)]
pub struct Timezone(&'static str);

impl Timezone {
    pub fn random() -> Result<Self, &'static str> {
        let mut rng = rand::rng();
        let res = TIMEZONES.choose(&mut rng).unwrap();
        Ok(Timezone(res))
    }
}
