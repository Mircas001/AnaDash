use anyhow::Result;
use futures_util::stream::StreamExt;
use shared::HostTransmission;
use shared::NotificationData;
use std::collections::HashMap;
use tokio::sync::mpsc;
use zbus::zvariant::Value;
use zbus::{Connection, MessageStream};

/*
 * My understanding of this:
 * tx does nothing, rx is a yapping notifications thing
*/

pub fn spawn_notification_monitor() -> mpsc::Receiver<HostTransmission> {
    let (tx, rx) = mpsc::channel(32); // * Serial is not something i was expecting

    tokio::spawn(async move {
        if let Err(e) = monitor(tx).await {
            eprintln!("Error with the notification monitor: {e}");
        }
    });

    rx
}

#[cfg(target_family = "unix")]
async fn monitor(tx: mpsc::Sender<HostTransmission>) -> Result<()> {
    let connection = Connection::session().await?;

    connection
        .call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus.Monitoring"),
            "BecomeMonitor",
            &(
                vec!["interface='org.freedesktop.Notifications',member='Notify'"],
                0u32,
            ),
        )
        .await?;

    let mut stream = MessageStream::from(&connection);
    while let Some(msg) = stream.next().await {
        let msg = msg?;

        if msg.header().member().map(|m| m.as_str()) != Some("Notify") {
            continue;
        }

        /*
         * The definition of an notification:
         * Notify(STRING app_name, UINT32 replaces_id, STRING app_icon,
         *     STRING summary, STRING body, ARRAY of STRING actions,
         *     DICT of {STRING, VARIANT} hints, INT32 expire_timeout)
         */

        let (app, _, _, summary, body, _, _, _): (
            String,
            u32,
            String,
            String,
            String,
            Vec<String>,
            HashMap<String, Value>,
            i32,
        ) = msg.body().deserialize()?;

        let app: heapless::String<16> =
            heapless::String::try_from(app.as_str()).unwrap_or_default();
        let summary: heapless::String<128> =
            heapless::String::try_from(summary.as_str()).unwrap_or_default();
        let body: heapless::String<256> =
            heapless::String::try_from(body.as_str()).unwrap_or_default();

        // * Kill itself if main dies
        if tx
            .send(HostTransmission::Notification(NotificationData {
                app,
                summary,
                body,
            }))
            .await
            .is_err()
        {
            break;
        }
    }

    Ok(())
}
