pub mod app_info {
    pub const NAME: &str = "Oxiflow";
    pub const VERSION: &str = "1.1.8";
    pub const AUTHOR: &str = "Artem Oliinyk <azure.email@gmail.com";
    pub const DESCRIPTION: &str = "Small fast HTTP load-testing app";
}

pub const EXIT_ERROR_PARSING_ARGS: u8 = 3;
pub const EXIT_UNKNOWN_METHOD: u8 = 4;
pub const EXIT_NO_URLS_FOUND: u8 = 5;

// modules
pub mod components {
    pub mod cli;
    pub mod file_processor;
    pub mod http;
    pub mod progressbar;
    pub mod report;
    pub mod worker;
}
