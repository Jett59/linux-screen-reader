use std::error::Error;

use atspi::{
    identify::{
        document::DocumentEvents,
        focus::FocusEvent,
        object::ObjectEvents,
        window::{ActivateEvent, WindowEvents},
    },
    signify::Signified,
    zbus::{export::futures_util::StreamExt, zvariant::Value},
    AccessibilityConnection,
};
use tts::Tts;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut tts = Tts::default()?;
    let connection = AccessibilityConnection::open().await?;
    connection.register_event::<ObjectEvents>().await?;
    connection.register_event::<WindowEvents>().await?;
    connection.register_event::<DocumentEvents>().await?;
    let mut event_stream = connection.event_stream();
    while let Some(event) = event_stream.next().await {
        let event = event?;
        if let Ok(event) = ActivateEvent::try_from(event) {
            let title = String::try_from(event.inner().any_data() as &Value)?;
            tts.stop();
            tts.speak(title, false)?;
        }
    }
    Ok(())
}
