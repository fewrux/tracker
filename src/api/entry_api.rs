use crate::repositories::entry_repo::EntryRepo;
use actix_web::{get, web::Data, HttpResponse};
use chrono::{DateTime, Utc};

/// Adds a new entry to the database with the current timestamp.
///
/// This endpoint generates a timestamp in RFC2822 format for the new entry and
/// adds it to the database using the `EntryRepo`. If successful, it returns the
/// details of the added entry in JSON format.
///
/// # Example
///
/// ```
/// GET /entries/add
/// ```
#[get("/entries/add")]
pub async fn add_entry(db: Data<EntryRepo>) -> HttpResponse {
    let now: DateTime<Utc> = Utc::now();
    let timestamp = now.to_rfc2822();

    let entry_detail = db.create_entry(timestamp).await;

    match entry_detail {
        Ok(entry) => {
            log::info!("entry added: {:?}", &entry);
            HttpResponse::Ok().json(entry)
        }
        Err(err) => {
            log::error!("\n{err:?}");
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

/// Retrieves a list of all entries in the database.
///
/// This endpoint retrieves a list of entries from the database using the `EntryRepo`.
/// If successful, it returns the list of entries in JSON format.
///
/// # Example
///
/// ```
/// GET /entries
/// ```
#[get("/entries")]
pub async fn get_entries(db: Data<EntryRepo>) -> HttpResponse {
    let entries = db.get_entries().await;
    match entries {
        Ok(entries) => {
            log::info!("retrieved list of entries");
            HttpResponse::Ok().json(entries)
        }
        Err(err) => {
            log::error!("\n{err:?}");
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}
