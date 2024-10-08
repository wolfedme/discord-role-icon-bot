use serenity::all::Http;
use tracing::{debug, error};

use crate::model::feature::LogToChannel;

pub async fn log_to_channel(
    http: &Http,
    log_feature: &LogToChannel,
    message: &str,
    message_type: LogType,
) {
    if !log_feature.enabled {
        debug!("log_to_channel.enabled == false");
        return;
    }

    let msg = match message_type {
        LogType::Info => format!("--- ℹ️ INFO ---\n{}", message),
        LogType::Warn => format!("--- ⚠️ WARN ---\n{}", message),
        LogType::Error => format!("--- ‼️ ERROR ---\n{}", message),
    };

    let payload = serenity::builder::CreateMessage::default().content(msg);

    match http
        .send_message(log_feature.channel_id.into(), Vec::default(), &payload)
        .await
    {
        Ok(_) => (),
        Err(why) => {
            error!("Could not send log message due to: {:?}", why);
            ()
        }
    }
}

pub enum LogType {
    Info,
    Warn,
    Error,
}
