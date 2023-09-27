mod api;
mod models;
mod repositories;

use actix_web::web::{Data, ServiceConfig};
use anyhow::{anyhow, Context};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_secrets::SecretStore;

use api::entry_api::{add_entry, get_entries};
use repositories::entry_repo::EntryRepo;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    env_logger::init();

    let db_uri = secret_store.get("MONGO_URI").with_context(|| {
        log::error!("\nfailed to load database URI");
        anyhow!("failed to load database URI")
    })?;

    let db = EntryRepo::init(db_uri).await.map_err(|e| {
        log::error!("\n{e:?}");
        anyhow!("failed to initialize database: {}", e)
    })?;
    let db_data = Data::new(db);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(db_data.clone())
            .service(add_entry)
            .service(get_entries);
    };

    Ok(config.into())
}
