mod api;
mod models;
mod repositories;

use actix_web::web::{Data, ServiceConfig};
use anyhow::anyhow;
use api::entry_api::{add_entry, get_entries};
use repositories::entry_repo::EntryRepo;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_secrets::SecretStore;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let db_uri = if let Some(secret) = secret_store.get("MONGO_URI") {
        secret
    } else {
        return Err(anyhow!("Error loading database uri").into());
    };

    let db = EntryRepo::init(db_uri).await;
    let db_data = Data::new(db);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(db_data.clone())
            .service(add_entry)
            .service(get_entries);
    };

    Ok(config.into())
}
