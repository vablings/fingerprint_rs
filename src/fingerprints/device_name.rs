use crate::fingerprints::{Browser, Os};
use rand::prelude::IndexedRandom;
use serde::Serialize;
static DEVICE_NAME_CHROME: &[&str] = &["Chrome V136.0.0.0 (__OS__)", "Chrome V135.0.0.0"];

static DEVICE_NAME_FIREFOX: &[&str] = &["Firefox V123.0 (__OS__)"];

static DEVICE_NAME_EDGE: &[&str] = &["Edge V136.0.0.0 (__OS__)"];

#[derive(Serialize, Debug)]
pub struct DeviceName(String);

impl DeviceName {
    pub fn random(browser: &Browser, os: &Os) -> Result<Self, &'static str> {
        Ok(match browser {
            Browser::Chrome => Self::random_chrome(&os)?,
            Browser::Firefox => Self::random_firefox(&os)?,
            Browser::Edge => Self::random_edge(&os)?,
        })
    }

    fn random_chrome(os: &Os) -> Result<Self, &'static str> {
        let mut rng = rand::rng();
        let res = DEVICE_NAME_CHROME.choose(&mut rng).unwrap();
        if res.contains("__OS__") {
            return Ok(DeviceName(res.replace("__OS__", os.to_string().as_str())));
        }
        Ok(DeviceName(res.to_string()))
    }
    fn random_firefox(os: &Os) -> Result<Self, &'static str> {
        let mut rng = rand::rng();
        let res = DEVICE_NAME_FIREFOX.choose(&mut rng).unwrap();
        if res.contains("__OS__") {
            return Ok(DeviceName(res.replace("__OS__", os.to_string().as_str())));
        }
        Ok(DeviceName(res.to_string()))
    }
    fn random_edge(os: &Os) -> Result<Self, &'static str> {
        let mut rng = rand::rng();
        let res = DEVICE_NAME_EDGE.choose(&mut rng).unwrap();
        if res.contains("__OS__") {
            return Ok(DeviceName(res.replace("__OS__", os.to_string().as_str())));
        }
        Ok(DeviceName(res.to_string()))
    }
}
