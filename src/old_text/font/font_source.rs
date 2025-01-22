use std::path::PathBuf;

pub const ROBOTO_FAMILY: &str = "Roboto";

#[derive(PartialEq)]
pub enum FontSource {
    Url(String),
    Raw(Vec<u8>),
    Local(PathBuf),
}

impl FontSource {
    pub fn roboto_local() -> Self {
        let roboto_regular = include_bytes!("./fonts/roboto/Roboto-Regular.ttf");

        FontSource::Raw(roboto_regular.to_vec())
    }
}
