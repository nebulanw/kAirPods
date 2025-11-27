//! Media control module for sending play/pause commands via MPRIS.
//!
//! This module provides functionality to control media playback using the
//! MPRIS (Media Player Remote Interfacing Specification) D-Bus interface.

use log::{debug, warn};
use zbus::Connection;

/// Sends a play command to the active media player via MPRIS.
pub async fn send_play() {
   if let Err(e) = send_mpris_command("Play").await {
      warn!("Failed to send play command: {e}");
   } else {
      debug!("Sent play command to media player");
   }
}

/// Sends a pause command to the active media player via MPRIS.
pub async fn send_pause() {
   if let Err(e) = send_mpris_command("Pause").await {
      warn!("Failed to send pause command: {e}");
   } else {
      debug!("Sent pause command to media player");
   }
}

/// Sends a play/pause toggle command to the active media player via MPRIS.
pub async fn send_play_pause() {
   if let Err(e) = send_mpris_command("PlayPause").await {
      warn!("Failed to send play/pause command: {e}");
   } else {
      debug!("Sent play/pause command to media player");
   }
}

async fn send_mpris_command(method: &str) -> Result<(), Box<dyn std::error::Error>> {
   // Connect to the session bus
   let connection = Connection::session().await?;

   // List all MPRIS services
   let dbus_proxy = zbus::fdo::DBusProxy::new(&connection).await?;
   let names = dbus_proxy.list_names().await?;

   // Find the first active MPRIS media player
   let mpris_service = names
      .iter()
      .find(|name| name.starts_with("org.mpris.MediaPlayer2."));

   let Some(service_name) = mpris_service else {
      debug!("No active MPRIS media player found");
      return Ok(()); // Not an error if no player is active
   };

   // Call the method using zbus's call API
   let path = zbus::zvariant::ObjectPath::from_str_unchecked("/org/mpris/MediaPlayer2");
   let interface = "org.mpris.MediaPlayer2.Player";
   
   connection
      .call_method(
         Some(service_name.as_str()),
         &path,
         Some(interface),
         method,
         &(),
      )
      .await?;

   Ok(())
}

