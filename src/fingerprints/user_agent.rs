use rand::seq::IndexedRandom;
use serde::Serialize;
static USER_AGENT: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) Gecko/20100101 Firefox/123.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) Edg/135.0.0.0",
];
#[derive(Serialize, Debug)]
pub struct UserAgent(String);

impl UserAgent {
    pub fn random() -> Result<Self, &'static str> {
        let mut rng = rand::rng();
        let res = USER_AGENT.choose(&mut rng).unwrap();
        Ok(UserAgent(res.to_string()))
    }
    pub fn user_agent<S: AsRef<str>>(user_agent: S) -> Self {
        UserAgent(user_agent.as_ref().to_string())
    }
}
