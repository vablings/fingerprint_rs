use crate::fingerprints::Browser;
use rand::prelude::IndexedRandom;
use serde::Serialize;
static PXI_OUTPUT_CHROME: &[f64] = &[
    124.04347527516074,
    124.08072766105033,
    124.04347657808103,
    124.04346607114712,
    124.03942285600351,
    124.0434496849557,
    124.0434485301812,
];
static PXI_OUTPUT_FIREFOX: &[f64] = &[
    124.04347527516074,
    124.08072766105033,
    124.04347657808103,
    124.04346607114712,
    124.03942285600351,
    124.0434496849557,
    124.0434485301812,
];
static PXI_OUTPUT_EDGE: &[f64] = &[
    124.04347527516074,
    124.08072766105033,
    124.04347657808103,
    124.04346607114712,
    124.03942285600351,
    124.0434496849557,
    124.0434485301812,
];
#[derive(Serialize, Debug)]
pub struct Audios(&'static f64);

impl Audios {
    pub fn random(browser: &Browser) -> Result<Self, &'static str> {
        Ok(match browser {
            Browser::Chrome => Self::random_chrome()?,
            Browser::Firefox => Self::random_firefox()?,
            Browser::Edge => Self::random_edge()?,
        })
    }

    fn random_chrome() -> Result<Self, &'static str> {
        let mut rng = rand::rng();
        let res = PXI_OUTPUT_CHROME.choose(&mut rng).unwrap();
        Ok(Audios(res))
    }
    fn random_firefox() -> Result<Self, &'static str> {
        let mut rng = rand::rng();
        let res = PXI_OUTPUT_FIREFOX.choose(&mut rng).unwrap();
        Ok(Audios(res))
    }
    fn random_edge() -> Result<Self, &'static str> {
        let mut rng = rand::rng();
        let res = PXI_OUTPUT_EDGE.choose(&mut rng).unwrap();
        Ok(Audios(res))
    }
}
