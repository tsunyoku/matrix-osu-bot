use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use matrix_sdk::{Client, ClientBuildError, LoopCtrl};
use matrix_sdk::config::SyncSettings;
use matrix_sdk::ruma::UserId;
use matrix_sdk::ruma::api::client::filter::FilterDefinition;
use matrix_sdk::ruma::events::key::verification::request::ToDeviceKeyVerificationRequestEvent;
use tokio::fs;
use tracing::{info, warn};
use crate::error::Result;
use crate::matrix::session::FullSession;
use crate::matrix::settings::MatrixSettings;
use crate::matrix::verification::PendingVerification;

mod matrix;
mod error;
mod events;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let matrix_settings = MatrixSettings::new()?;

    if !matrix_settings.data_directory.exists() {
        fs::create_dir(&matrix_settings.data_directory).await?;
    }

    let session_file: &'static Path = Box::leak(matrix_settings.session_file().into_boxed_path());

    let (client, sync_token) = if session_file.exists() {
        info!("Restoring existing session");
        restore_session(&matrix_settings).await?
    } else {
        info!("No session found, logging in");
        (login(&matrix_settings).await?, None)
    };

    let admin_user_id = UserId::parse(&matrix_settings.admin_user_id)?;
    let pending_verification: PendingVerification = Arc::new(Mutex::new(None));

    client.add_event_handler_context(pending_verification);
    client.add_event_handler_context(admin_user_id);

    client.add_event_handler(events::room_message::on_room_message);
    client.add_event_handler(events::verification::on_device_key_verification_request);

    let sync_settings = sync(&client, sync_token, session_file).await?;

    info!("Starting sync loop");
    client
        .sync_with_result_callback(sync_settings, |sync_result| async move {
            let response = sync_result?;

            persist_sync_token(session_file, response.next_batch)
                .await
                .map_err(|err| matrix_sdk::Error::UnknownError(err.into()))?;

            Ok(LoopCtrl::Continue)
        })
        .await?;

    Ok(())
}

async fn sync(client: &Client, initial_sync_token: Option<String>, session_file: &Path) -> Result<SyncSettings> {
    let filter = FilterDefinition::with_lazy_loading();

    let mut sync_settings = SyncSettings::default()
        .filter(filter.into());

    if let Some(sync_token) = initial_sync_token {
        sync_settings = sync_settings.token(sync_token);
    }

    info!("Performing initial sync");
    loop {
        match client.sync_once(sync_settings.clone()).await {
            Ok(response) => {
                sync_settings = sync_settings.token(response.next_batch.clone());
                persist_sync_token(session_file, response.next_batch).await?;
                break;
            }
            Err(error) => {
                warn!("Initial sync failed, retrying in 5s: {error}");
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }

    Ok(sync_settings)
}

async fn persist_sync_token(session_file: &Path, sync_token: String) -> Result<()> {
    let serialized_session = fs::read_to_string(session_file).await?;

    let mut full_session: FullSession = serde_json::from_str(&serialized_session)?;
    full_session.sync_token = Some(sync_token);

    let serialized_session = serde_json::to_string(&full_session)?;
    fs::write(session_file, serialized_session).await?;

    Ok(())
}

async fn restore_session(matrix_settings: &MatrixSettings) -> Result<(Client, Option<String>)> {
    let serialised_session = fs::read_to_string(matrix_settings.session_file()).await?;

    let FullSession { user_session, sync_token } =
        serde_json::from_str(&serialised_session)?;

    let client = build_client(matrix_settings).await?;

    client.restore_session(user_session).await?;

    Ok((client, sync_token))
}

async fn login(matrix_settings: &MatrixSettings) -> Result<Client> {
    let client = build_client(matrix_settings).await?;

    let matrix_auth = client.matrix_auth();

    matrix_auth
        .login_username(&matrix_settings.username, &matrix_settings.password)
        .initial_device_display_name("matrix-osu-bot")
        .await?;

    let user_session = matrix_auth
        .session()
        .expect("A logged-in client should have a session");

    let full_session = FullSession {
        user_session,
        sync_token: None,
    };

    let serialised_session = serde_json::to_string(&full_session)?;

    fs::write(matrix_settings.session_file(), serialised_session).await?;

    info!("Login successful, session saved");
    Ok(client)
}

async fn build_client(matrix_settings: &MatrixSettings) -> Result<Client, ClientBuildError> {
    Client::builder()
        .homeserver_url(&matrix_settings.homeserver)
        .sqlite_store(matrix_settings.database_directory(), Some(&matrix_settings.database_passphrase))
        .build()
        .await
}
