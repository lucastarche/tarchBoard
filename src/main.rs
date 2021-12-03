mod app;
mod clock;
mod load_image;
mod message;
mod view;
mod weather;

use anyhow::Result;
use std::thread;
use tokio::sync::mpsc;

use app::App;
use message::{Message, MessageReceiver};
use weather::fetch_weather;

async fn async_thread(mut rx: MessageReceiver) -> Result<()> {
    while let Some(message) = rx.recv().await {
        match message {
            Message::FetchNewResource { url, send } => {
                fetch_weather(send, url).await?;
            }
            _ => todo!(),
        }
    }

    Ok(())
}

fn main() {
    let (tx, rx) = mpsc::unbounded_channel();

    thread::spawn(move || {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async_thread(rx))
    });

    let mut app = App::new(tx);
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
