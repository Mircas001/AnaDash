use anyhow::Result;
use futures_util::stream::StreamExt;
use std::collections::HashMap;
use tokio::sync::mpsc;
use zbus::zvariant::Value;
use zbus::{Connection, MessageStream};

pub struct Notification {
    pub app: String,
    pub notification_icon: String, // * I should consider not using strings
    pub summary: String,
    pub body: String,
}

/*
 * My understanding of this:
 * tx does nothing, rx is a yapping notifications thing
*/

pub fn spawn_notification_monitor() -> mpsc::Receiver<Notification> {
    let (tx, rx) = mpsc::channel(32); // * Serial is not something i was expecting

    tokio::spawn(async move {
        if let Err(e) = monitor(tx).await {
            eprintln!("Error with the notification monitor: {e}");
        }
    });

    rx
}

#[cfg(target_family = "unix")]
async fn monitor(tx: mpsc::Sender<Notification>) -> Result<()> {
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

        let (app, _, notification_icon, summary, body, _, _, _): (
            String,
            u32,
            String,
            String,
            String,
            Vec<String>,
            HashMap<String, Value>,
            i32,
        ) = msg.body().deserialize()?;

        // * Kill itself if main dies
        if tx
            .send(Notification {
                app,
                notification_icon,
                summary,
                body,
            })
            .await
            .is_err()
        {
            break;
        }
    }

    Ok(())
}
