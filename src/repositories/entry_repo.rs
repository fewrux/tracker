use crate::models::entry_model::Entry;
use anyhow::{anyhow, Result};
use futures::TryStreamExt;

use mongodb::{bson::doc, options::FindOptions, results::InsertOneResult, Client, Collection};

pub struct EntryRepo {
    col: Collection<Entry>,
}

impl EntryRepo {
    pub async fn init(uri: String) -> Result<Self, anyhow::Error> {
        let client = Client::with_uri_str(uri).await.map_err(|e| {
            log::error!("\n{e:?}");
            anyhow!("failed to connect to database: {}", e)
        })?;
        let db = client.database("tracker_mongo");
        let col = db.collection("Entry");
        Ok(EntryRepo { col })
    }

    pub async fn create_entry(&self, timestamp: String) -> Result<InsertOneResult, anyhow::Error> {
        let new_doc = Entry {
            id: None,
            timestamp,
        };

        let entry = self.col.insert_one(new_doc, None).await.map_err(|e| {
            log::error!("\n{e:?}");
            anyhow!("error adding new entry: {}", e)
        })?;
        Ok(entry)
    }

    pub async fn get_entries(&self) -> Result<Vec<Entry>, anyhow::Error> {
        let find_options = FindOptions::builder()
            .sort(doc! { "timestamp": -1 })
            .build();

        let mut cursors = self.col.find(None, find_options).await.map_err(|e| {
            log::error!("\n{e:?}");
            anyhow!("error getting list of entries: {}", e)
        })?;

        let mut entries: Vec<Entry> = Vec::new();

        while let Some(entry) = cursors.try_next().await.map_err(|e| {
            log::error!("\n{e:?}");
            anyhow!("error mapping through cursor: {}", e)
        })? {
            entries.push(entry)
        }

        Ok(entries)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn should_connect_to_the_database() {
        dotenv().ok();

        let db_uri = std::env::var("MONGO_URI").unwrap();
        let db = EntryRepo::init(db_uri).await.unwrap();

        assert_eq!(db.col.name(), "Entry");
    }

    #[tokio::test]

    async fn should_create_a_new_entry() {
        dotenv().ok();

        let db_uri = std::env::var("MONGO_URI").unwrap();
        let db = EntryRepo::init(db_uri).await.unwrap();

        let timestamp = "2021-01-01T00:00:00.000Z".to_string();
        db.create_entry(timestamp.clone()).await.unwrap();

        let entries = db.get_entries().await.unwrap();

        let last_entry = entries.last().unwrap();

        assert_eq!(last_entry.timestamp == timestamp, true);
    }

    #[tokio::test]
    async fn should_get_all_entries() {
        dotenv().ok();

        let db_uri = std::env::var("MONGO_URI").unwrap();
        let db = EntryRepo::init(db_uri).await.unwrap();

        let entries = db.get_entries().await.unwrap();

        assert_eq!(entries.len() > 0, true);
    }
}
