use rand::Rng;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Fingerprint(String);

impl Fingerprint {
    //todo; make canvas code here something specific for now its very site specific
    pub fn new<S: Into<String>>(canvas_code: S) -> Self {
        Fingerprint(canvas_code.into())
    }
    pub fn random() -> Self {
        let mut rng = rand::rng();
        let mut bytes = [0u8; 32];
        rng.fill(&mut bytes);
        Fingerprint(hex::encode(bytes))
    }
}
