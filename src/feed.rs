use std::collections::HashSet;
use std::error::Error;
use std::fs;

use feed_rs::model::Entry;
use tokio::task::JoinSet;

const MAX_ITEMS: usize = 20;

pub struct FeedEntry {
    pub source: String,
    pub entry: Entry,
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

pub async fn run() -> Result<Vec<FeedEntry>, Box<dyn Error>> {
    let urls = load_urls();

    let mut tasks = JoinSet::new();
    for url in urls {
        tasks.spawn(fetch_feed(url));
    }

    let mut all_entries: Vec<FeedEntry> = Vec::new();
    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(Ok((source, entries))) => {
                for entry in entries {
                    all_entries.push(FeedEntry {
                        source: source.clone(),
                        entry,
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

    Ok(all_entries)
}
