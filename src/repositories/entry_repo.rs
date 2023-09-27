use crate::models::entry_model::Entry;
use anyhow::{anyhow, Result};
use futures::TryStreamExt;

use mongodb::{bson::doc, options::FindOptions, results::InsertOneResult, Client, Collection};

pub struct EntryRepo {
    col: Collection<Entry>,
}

impl EntryRepo {
    pub async fn init(uri: String) -> Result<Self, anyhow::Error> {
        let client = Client::with_uri_str(uri)
            .await
            .map_err(|e| anyhow!("failed to connect to database: {}", e))?;
        let db = client.database("tracker_mongo");
        let col = db.collection("Entry");
        Ok(EntryRepo { col })
    }

    pub async fn create_entry(&self, new_entry: Entry) -> Result<InsertOneResult, anyhow::Error> {
        let new_doc = Entry {
            id: None,
            timestamp: new_entry.timestamp,
        };

        let entry = self
            .col
            .insert_one(new_doc, None)
            .await
            .map_err(|e| anyhow!("error adding new entry: {}", e))?;
        Ok(entry)
    }

    pub async fn get_entries(&self) -> Result<Vec<Entry>, anyhow::Error> {
        let find_options = FindOptions::builder()
            .sort(doc! { "timestamp": -1 })
            .build();

        let mut cursors = self
            .col
            .find(None, find_options)
            .await
            .map_err(|e| anyhow!("error getting list of entries: {}", e))?;

        let mut entries: Vec<Entry> = Vec::new();

        while let Some(entry) = cursors
            .try_next()
            .await
            .map_err(|e| anyhow!("error mapping through cursor: {}", e))?
        {
            entries.push(entry)
        }

        Ok(entries)
    }
}
