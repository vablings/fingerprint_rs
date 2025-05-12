use rand::seq::IndexedRandom;
use serde::Serialize;
static SYSTEM_LANG: &[&str] = &["en-US", "en-GB", "fr-FR"];
#[derive(Serialize, Debug)]
pub struct SystemLang(&'static str);

impl SystemLang {
    pub fn random() -> Result<Self, &'static str> {
        let mut rng = rand::rng();
        let res = SYSTEM_LANG.choose(&mut rng).unwrap();
        Ok(SystemLang(res))
    }
}
