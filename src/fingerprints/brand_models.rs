use rand::seq::IndexedRandom;
use serde::Serialize;

static BRAND_MODELS: &[&str] = &["unknown", "Dell XPS 13", "MacBook Pro"];
#[derive(Serialize, Debug)]
pub struct BrandModels(&'static str);

impl BrandModels {
    pub fn random() -> Result<Self, &'static str> {
        let mut rng = rand::rng();
        let res = BRAND_MODELS.choose(&mut rng).unwrap();
        Ok(BrandModels(res))
    }
}
