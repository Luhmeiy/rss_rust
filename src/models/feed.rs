use feed_rs::model::Entry;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Feed {
    pub title: String,
    pub url: String,
}

#[derive(Clone, PartialEq)]
pub struct FeedEntry {
    pub feed: String,
    pub entry: Entry,
}

impl FeedEntry {
    pub fn title(&self) -> String {
        self.entry
            .title
            .as_ref()
            .map_or("Untitled", |t| &t.content)
            .to_string()
    }

    pub fn feed(&self) -> String {
        self.feed.clone()
    }

    pub fn date(&self) -> String {
        self.entry.published.or(self.entry.updated).map_or_else(
            || "N/A".to_string(),
            |d| d.format("%d/%m/%Y - %H:%M").to_string(),
        )
    }

    pub fn summary(&self) -> String {
        self.entry
            .summary
            .as_ref()
            .map(|s| s.content.to_string())
            .unwrap_or_default()
    }

    pub fn content(&self) -> String {
        if let Some(body) = self.entry.content.as_ref().and_then(|c| c.body.as_ref()) {
            return body.to_string();
        }
        self.summary()
    }
}
