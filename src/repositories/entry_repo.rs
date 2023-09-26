use crate::models::entry_model::Entry;
use futures::TryStreamExt;

use mongodb::{
    bson::doc, bson::extjson::de::Error, options::FindOptions, results::InsertOneResult, Client,
    Collection,
};

pub struct EntryRepo {
    col: Collection<Entry>,
}

impl EntryRepo {
    pub async fn init(uri: String) -> Self {
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("tracker_mongo");
        let col = db.collection("Entry");
        EntryRepo { col }
    }

    pub async fn create_entry(&self, new_entry: Entry) -> Result<InsertOneResult, Error> {
        let new_doc = Entry {
            id: None,
            timestamp: new_entry.timestamp,
        };

        let entry = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error adding new entry");
        Ok(entry)
    }

    pub async fn get_entries(&self) -> Result<Vec<Entry>, Error> {
        let find_options = FindOptions::builder()
            .sort(doc! { "timestamp": -1 })
            .build();

        let mut cursors = self
            .col
            .find(None, find_options)
            .await
            .ok()
            .expect("Error getting list of entries");

        let mut entries: Vec<Entry> = Vec::new();

        while let Some(entry) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            entries.push(entry)
        }

        Ok(entries)
    }
}
