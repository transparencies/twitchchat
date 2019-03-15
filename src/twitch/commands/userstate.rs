use super::*;

/// Identifies a user's chat settings or properties (e.g., chat color)..
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UserState {
    pub tags: Tags,
    pub channel: String,
}

impl UserState {
    pub fn badges(&self) -> Vec<Badge> {
        badges(self.get("badges").unwrap_or_default())
    }
    pub fn color(&self) -> Option<TwitchColor> {
        self.get("color").map(RGB::from_hex).map(Into::into)
    }
    pub fn display_name(&self) -> Option<&str> {
        self.get("display-name")
    }
    pub fn emotes(&self) -> Vec<Emotes> {
        emotes(self.get("emotes").unwrap_or_default())
    }
    pub fn moderator(&self) -> bool {
        self.get_as_bool("mod")
    }
}

impl Tag for UserState {
    fn get(&self, key: &str) -> Option<&str> {
        self.tags.get(key).map(AsRef::as_ref)
    }
}
