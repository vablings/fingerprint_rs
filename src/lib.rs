use crate::fingerprints::audio::Audios;
use crate::fingerprints::brand_models::BrandModels;
use crate::fingerprints::canvas_codes::CanvasCode;
use crate::fingerprints::device_name::DeviceName;
use crate::fingerprints::fingerprint::Fingerprint;
use crate::fingerprints::list_plugin::ListPlugin;
use crate::fingerprints::screen_resolution::ScreenResolution;
use crate::fingerprints::system_lang::SystemLang;
use crate::fingerprints::system_version::SystemVersion;
use crate::fingerprints::timezones::Timezone;
use crate::fingerprints::user_agent::UserAgent;
use crate::fingerprints::web_timezones::WebTimezones;
use crate::fingerprints::webgl_renderers::WebglRenderers;
use crate::fingerprints::{Browser, Os};
use serde::Serialize;
use serde_json::Value;
use crate::fingerprints::webgl_vendor::WebglVendors;

pub mod fingerprints;

#[derive(Serialize, Debug)]
#[serde(bound(deserialize = "'de: 'static"))]
struct FingerprintData {
    audios: Audios,
    available_screen_resolution: ScreenResolution,
    brand_models: BrandModels,
    canvas_code: CanvasCode,
    device_id: String,
    device_name: DeviceName,
    fingerprint: Fingerprint,
    list_plugin: ListPlugin,
    related_device_ids: String,
    screen_resolution: ScreenResolution,
    system_lang: SystemLang,
    system_version: SystemVersion,
    timezone: Timezone,
    timezone_offset: i32,
    user_agent: UserAgent,
    web_timezone: WebTimezones,
    webgl_renderers: WebglRenderers,
    webgl_vendor: WebglVendors,
}


impl FingerprintData {
    fn new_random(browser: Browser, os: Os) -> Result<Self, &'static str> {
        //todo; add a sensible helper interface to give logical results for location, timezone and language
        Ok(FingerprintData {
            audios: Audios::random(&browser)?,
            available_screen_resolution: ScreenResolution::random()?,
            brand_models: BrandModels::random()?,
            canvas_code: CanvasCode::random(),
            device_id: "".to_string(),
            device_name: DeviceName::random(&browser, &os)?,
            fingerprint: Fingerprint::random(),
            list_plugin: ListPlugin::from_browser(&browser)?,
            related_device_ids: "".to_string(),
            screen_resolution: ScreenResolution::random()?,
            system_lang: SystemLang::random()?,
            system_version: SystemVersion::random(&os)?,
            timezone: Timezone::random()?,
            timezone_offset: 0,
            user_agent: UserAgent::random()?,
            web_timezone: WebTimezones::random()?,
            webgl_renderers: WebglRenderers::random()?,
            webgl_vendor: WebglVendors::random()?,
        })
    }
    fn json(&self) -> Value {
        serde_json::to_value(self).expect("Seralizsation failed")
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn just_print_for_now() {
        let random_fingerprint = FingerprintData::new_random(Browser::Chrome, Os::Windows).unwrap();
        println!("{}", serde_json::to_string_pretty(&random_fingerprint.json()).unwrap());
    }
}


