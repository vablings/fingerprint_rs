use rand::seq::IndexedRandom;
use serde::Serialize;
static SCREEN_RESOLUTION: &[&str] = &[
    "1920,1080",
    "2560,1440",
    "3840,2160",
    "1280,720",
    "1600,900",
    "2560,1080",
];
#[derive(Serialize, Debug)]
pub struct ScreenResolution(&'static str);

impl ScreenResolution {
    pub fn random() -> Result<Self, &'static str> {
        let mut rng = rand::rng();
        let res = SCREEN_RESOLUTION.choose(&mut rng).unwrap();
        Ok(ScreenResolution(res))
    }
}
