use crate::{models::entry_model::Entry, repositories::entry_repo::EntryRepo};
use actix_web::{get, web::Data, HttpResponse};
use chrono::{DateTime, Utc};

#[get("/entries/add")]
pub async fn add_entry(db: Data<EntryRepo>) -> HttpResponse {
    let now: DateTime<Utc> = Utc::now();
    let timestamp = now.to_rfc2822();

    let data = Entry {
        id: None,
        timestamp,
    };

    let entry_detail = db.create_entry(data).await;

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
