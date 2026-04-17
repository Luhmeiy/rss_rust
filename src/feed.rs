use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::sync::mpsc;

use feed_rs::model::Entry;
use tokio::task::JoinSet;

const MAX_ITEMS: usize = 20;

#[derive(Clone, PartialEq)]
pub struct FeedEntry {
    pub feed_url: String,
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

    pub fn feed_url(&self) -> String {
        self.feed_url.clone()
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

fn load_urls() -> Vec<String> {
    fs::read_to_string("feeds.txt")
        .unwrap_or_default()
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

async fn fetch_feed(url: String) -> Result<(String, Vec<Entry>), Box<dyn Error + Send + Sync>> {
    let content = reqwest::get(&url).await?.bytes().await?;
    let feed = feed_rs::parser::parse(&content[..])?;
    let title = feed
        .title
        .as_ref()
        .map_or("Unknown Feed", |t| &t.content)
        .to_string();

    Ok((title, feed.entries))
}

pub fn run() -> (Vec<String>, Vec<FeedEntry>) {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(async_run());
        tx.send(result.expect("Failed to fetch feeds.")).unwrap();
    });

    rx.recv().unwrap()
}

async fn async_run() -> Result<(Vec<String>, Vec<FeedEntry>), Box<dyn Error>> {
    let urls = load_urls();

    let mut tasks = JoinSet::new();
    for url in urls {
        tasks.spawn(fetch_feed(url));
    }

    let mut feeds: Vec<String> = Vec::new();
    let mut all_entries: Vec<FeedEntry> = Vec::new();
    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(Ok((source, entries))) => {
                for (index, entry) in entries.iter().enumerate() {
                    if index == 0 {
                        feeds.push(source.clone());
                    }

                    all_entries.push(FeedEntry {
                        feed_url: source.clone(),
                        entry: entry.clone(),
                    });
                }
            }
            Ok(Err(e)) => eprintln!("Feed error: {}", e),
            Err(e) => eprintln!("Task error: {}", e),
        }
    }

    all_entries.sort_by(|a, b| {
        let da = a.entry.published.or(a.entry.updated);
        let db = b.entry.published.or(b.entry.updated);
        db.cmp(&da)
    });

    let mut seen = HashSet::new();
    let mut shown = 0;

    all_entries.retain(|item| {
        if shown >= MAX_ITEMS {
            return false;
        }

        let key = if !item.entry.id.is_empty() {
            item.entry.id.clone()
        } else {
            item.entry
                .links
                .first()
                .map_or(String::new(), |l| l.href.clone())
        };

        if !seen.insert(key) {
            return false;
        }

        shown += 1;
        true
    });

    Ok((feeds, all_entries))
}
