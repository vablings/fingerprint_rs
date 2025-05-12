pub mod audio;
pub mod brand_models;
pub mod canvas_codes;
pub mod device_name;
pub mod fingerprint;
pub mod list_plugin;
pub mod screen_resolution;
pub mod system_lang;
pub mod system_version;
pub mod timezones;
pub mod user_agent;
pub mod web_timezones;
pub mod webgl_renderers;
pub mod webgl_vendor;

pub enum Browser {
    Chrome,
    Firefox,
    Edge,
}

pub enum Os {
    Windows,
    Ubuntu,
}
impl ToString for Os {
    fn to_string(&self) -> String {
        match self {
            Os::Windows => "Windows".to_string(),
            Os::Ubuntu => "Ubuntu".to_string(),
        }
    }
}
