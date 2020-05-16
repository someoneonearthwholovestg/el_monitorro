use crate::sync::reader;
use crate::sync::reader::{FeedReaderError, FetchedFeed, FetchedFeedItem, ReadFeed};
use chrono::{DateTime, Utc};
use rss::Channel;

pub struct RssReader {
    pub url: String,
}

impl ReadFeed for RssReader {
    fn read(&self) -> Result<FetchedFeed, FeedReaderError> {
        match Channel::from_url(&self.url) {
            Ok(channel) => Ok(FetchedFeed::from(channel)),
            Err(err) => {
                let msg = format!("{}", err);
                Err(FeedReaderError { msg })
            }
        }
    }
}

impl From<Channel> for FetchedFeed {
    fn from(channel: Channel) -> Self {
        let mut items = channel
            .items()
            .into_iter()
            .filter(|item| item.link().is_some())
            .map(|item| {
                let pub_date: DateTime<Utc> = reader::parse_time(item.pub_date());
                FetchedFeedItem {
                    title: item.title().map(|s| s.to_string()),
                    description: item.description().map(|s| s.to_string()),
                    link: item.link().map(|s| s.to_string()),
                    author: item.author().map(|s| s.to_string()),
                    guid: item.guid().map(|s| s.value().to_string()),
                    publication_date: pub_date,
                }
            })
            .collect::<Vec<FetchedFeedItem>>();

        items.dedup_by(|a, b| a.link == b.link && a.title == b.title);

        FetchedFeed {
            title: channel.title().to_string(),
            link: channel.link().to_string(),
            description: channel.description().to_string(),
            items: items,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FetchedFeed;
    use rss::Channel;
    use std::fs;
    use std::str::FromStr;

    #[test]
    fn it_converts_rss_channel_to_fetched_feed() {
        let xml_feed = fs::read_to_string("./tests/support/rss_feed_example.xml").unwrap();
        let channel = Channel::from_str(&xml_feed).unwrap();

        let fetched_feed: FetchedFeed = channel.into();

        assert_eq!(fetched_feed.title, "FeedForAll Sample Feed".to_string());
    }
}