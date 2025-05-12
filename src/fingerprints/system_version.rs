use crate::fingerprints::Os;
use rand::seq::IndexedRandom;
use serde::Serialize;

static SYSTEM_VERSION_WINDOWS: &[&str] = &["Windows 10", "Windows 11"];

static SYSTEM_VERSION_UBUNTU: &[&str] = &["Ubuntu 24.04", "Ubuntu 22.04"];

#[derive(Serialize, Debug)]
pub struct SystemVersion(&'static str);

impl SystemVersion {
    pub fn random(os: &Os) -> Result<Self, &'static str> {
        match os {
            Os::Windows => Self::random_windows(),
            Os::Ubuntu => Self::random_ubuntu(),
        }
    }

    pub fn random_windows() -> Result<Self, &'static str> {
        let mut rng = rand::rng();
        let res = SYSTEM_VERSION_WINDOWS.choose(&mut rng).unwrap();
        Ok(SystemVersion(res))
    }
    pub fn random_ubuntu() -> Result<Self, &'static str> {
        let mut rng = rand::rng();
        let res = SYSTEM_VERSION_UBUNTU.choose(&mut rng).unwrap();
        Ok(SystemVersion(res))
    }
}
