#[derive(Debug)]
pub enum Topic {
    COMMENT_CREATE,
    NO_TOPIC_BAD,
}

impl Topic {
    pub fn as_str(&self) -> &'static str {
        match self {
            Topic::COMMENT_CREATE => "COMMENT_CREATE",
            Topic::NO_TOPIC_BAD => "NO_TOPIC_BAD",
        }
    }

    pub fn from_str(key: &str) -> Topic {
        match key {
            "COMMENT_CREATE" => Topic::COMMENT_CREATE,
            _ => Topic::NO_TOPIC_BAD,
        }
    }
}
