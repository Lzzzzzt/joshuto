use allmytoes::ThumbSize;
use ratatui_image::picker::ProtocolType;
use serde::Deserialize;

pub const fn default_max_preview_size() -> u64 {
    2 * 1024 * 1024 // 2 MB
}

pub const fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PreviewProtocol {
    #[default]
    Auto,
    Disabled,
    #[serde(untagged)]
    ProtocolType(ProtocolType),
}

#[derive(Clone, Debug, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum XDGThumbSizes {
    Normal,
    Large,
    #[default]
    XLarge,
    XXLarge,
}

impl XDGThumbSizes {
    pub fn to_amt_size(&self) -> ThumbSize {
        match &self {
            XDGThumbSizes::Normal => ThumbSize::Normal,
            XDGThumbSizes::Large => ThumbSize::Large,
            XDGThumbSizes::XLarge => ThumbSize::XLarge,
            XDGThumbSizes::XXLarge => ThumbSize::XXLarge,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct PreviewOptionRaw {
    #[serde(default = "default_max_preview_size")]
    pub max_preview_size: u64,
    #[serde(default)]
    pub preview_protocol: PreviewProtocol,
    #[serde(default)]
    pub preview_script: Option<String>,
    #[serde(default = "default_true")]
    pub use_xdg_thumbs: bool,
    #[serde(default)]
    pub xdg_thumb_size: XDGThumbSizes,
    #[serde(default)]
    pub preview_shown_hook_script: Option<String>,
    #[serde(default)]
    pub preview_removed_hook_script: Option<String>,
}

// This should be deleted maybe. I don't see where it is invoked.
impl std::default::Default for PreviewOptionRaw {
    fn default() -> Self {
        Self {
            max_preview_size: default_max_preview_size(),
            preview_protocol: PreviewProtocol::Auto,
            preview_script: None,
            use_xdg_thumbs: true,
            xdg_thumb_size: XDGThumbSizes::XLarge,
            preview_shown_hook_script: None,
            preview_removed_hook_script: None,
        }
    }
}
