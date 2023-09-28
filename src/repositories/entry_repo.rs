use crate::models::entry_model::Entry;
use anyhow::{anyhow, Result};
use futures::TryStreamExt;

use mongodb::{bson::doc, options::FindOptions, results::InsertOneResult, Client, Collection};

/// Repository for managing `Entry` documents in the database.
pub struct EntryRepo {
    col: Collection<Entry>,
}

impl EntryRepo {
    /// Initializes a new instance of `EntryRepo` connected to the MongoDB database.
    ///
    /// # Arguments
    ///
    /// * `uri` - A string containing the MongoDB connection URI.
    ///
    /// # Returns
    ///
    /// A `Result` containing the initialized `EntryRepo` instance if successful, or an error if the connection fails.
    pub async fn init(uri: String) -> Result<Self, anyhow::Error> {
        // Create a MongoDB client using the provided URI.
        let client = Client::with_uri_str(uri).await.map_err(|e| {
            log::error!("\n{e:?}");
            anyhow!("failed to connect to database: {}", e)
        })?;

        // Get the database and collection handles.
        let db = client.database("tracker_mongo");
        let col = db.collection("Entry");

        // Return the initialized EntryRepo.
        Ok(EntryRepo { col })
    }

    /// Creates a new entry in the database with the given timestamp.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - A string representing the timestamp for the new entry.
    ///
    /// # Returns
    ///
    /// A `Result` containing the result of the insertion operation if successful, or an error if it fails.
    pub async fn create_entry(&self, timestamp: String) -> Result<InsertOneResult, anyhow::Error> {
        // Create a new Entry document.
        let new_doc = Entry {
            id: None,
            timestamp,
        };

        // Insert the new Entry document into the collection
        let entry = self.col.insert_one(new_doc, None).await.map_err(|e| {
            log::error!("\n{e:?}");
            anyhow!("error adding new entry: {}", e)
        })?;

        // Return the result of the insertion.
        Ok(entry)
    }

    /// Retrieves a list of entries from the database, sorted by timestamp in descending order.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `Entry` instances if successful, or an error if the retrieval fails.
    pub async fn get_entries(&self) -> Result<Vec<Entry>, anyhow::Error> {
        // Define options for the find operation to sort by timestamp in descending order.
        let find_options = FindOptions::builder().sort(doc! { "_id": -1 }).build();

        // Perform the find operation on the database.
        let mut cursors = self.col.find(None, find_options).await.map_err(|e| {
            log::error!("\n{e:?}");
            anyhow!("error getting list of entries: {}", e)
        })?;

        let mut entries: Vec<Entry> = Vec::new();

        // Iterate through the cursor and collect entries.
        while let Some(entry) = cursors.try_next().await.map_err(|e| {
            log::error!("\n{e:?}");
            anyhow!("error mapping through cursor: {}", e)
        })? {
            entries.push(entry)
        }

        // Return the list of retrieved entries.
        Ok(entries)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use dotenvy;

    #[actix_web::test]
    async fn should_connect_to_the_database() {
        dotenvy::from_filename("Secrets.toml").unwrap();

        let db_uri = std::env::var("MONGO_URI").unwrap();
        let db = EntryRepo::init(db_uri).await.unwrap();

        assert_eq!(db.col.name(), "Entry");
    }

    #[actix_web::test]
    async fn should_create_a_new_entry() {
        dotenvy::from_filename("Secrets.toml").unwrap();

        let db_uri = std::env::var("MONGO_URI").unwrap();
        let db = EntryRepo::init(db_uri).await.unwrap();

        let timestamp = "2021-01-01T00:00:00.000Z".to_string();
        db.create_entry(timestamp.clone()).await.unwrap();

        let entries = db.get_entries().await.unwrap();

        let last_entry = entries.last().unwrap();

        assert_eq!(last_entry.timestamp == timestamp, true);
    }

    #[actix_web::test]
    async fn should_get_all_entries() {
        dotenvy::from_filename("Secrets.toml").unwrap();

        let db_uri = std::env::var("MONGO_URI").unwrap();
        let db = EntryRepo::init(db_uri).await.unwrap();

        let entries = db.get_entries().await.unwrap();

        assert_eq!(entries.len() > 0, true);
    }
}
