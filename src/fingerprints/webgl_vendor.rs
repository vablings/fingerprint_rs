use rand::seq::IndexedRandom;
use serde::Serialize;
static WEBGL_VENDORS: &[&str] = &[
    "Google Inc.",
    "Google Inc. (0x05404C42)",
    "Google Inc. (AMD)",
    "Google Inc. (Apple)",
    "Google Inc. (Google)",
    "Google Inc. (Microsoft Corporation)",
    "Google Inc. (Microsoft)",
    "Google Inc. (NVIDIA Corporation)",
    "Google Inc. (NVIDIA Corporation) #7pz9yksmUm",
    "Google Inc. (NVIDIA Corporation) #NbhL24LYfk",
    "Google Inc. (NVIDIA Corporation) #SoJg7htQt4",
    "Google Inc. (NVIDIA Corporation) #udEpwF086L",
    "Google Inc. (NVIDIA)",
    "Google Inc.(NVIDIA)",
];
#[derive(Serialize, Debug)]
pub struct WebglVendors(&'static str);

impl WebglVendors {
    pub fn random() -> Result<Self, &'static str> {
        let mut rng = rand::rng();
        let res = WEBGL_VENDORS.choose(&mut rng).unwrap();
        Ok(WebglVendors(res))
    }
}
