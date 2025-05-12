use crate::fingerprints::Browser;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ListPlugin(&'static str);

impl ListPlugin {
    pub fn from_browser(browser: &Browser) -> Result<Self, &'static str> {
        Ok(match browser {
            Browser::Chrome => ListPlugin("PDF Viewer,Chrome PDF Viewer,Chromium PDF Viewer"),
            Browser::Firefox => ListPlugin("PDF Viewer,Chrome PDF Viewer"),
            Browser::Edge => ListPlugin("PDF Viewer,Chrome PDF Viewer,Microsoft Edge PDF Viewer"),
        })
    }
}
