use rand::Rng;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct CanvasCode(String);

impl CanvasCode {
    //todo; make canvas code here something specific for now its very site specific
    pub fn new<S: Into<String>>(canvas_code: S) -> Self {
        CanvasCode(canvas_code.into())
    }
    pub fn random() -> Self {
        let mut rng = rand::rng();
        let mut bytes = [0u8; 16];
        rng.fill(&mut bytes);
        CanvasCode(hex::encode(bytes))
    }
}
