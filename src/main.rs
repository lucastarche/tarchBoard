#[macro_use]
extern crate diesel;
extern crate dotenv;

mod schema;

mod app;
mod clock;
mod kanban;
mod load_image;
mod message;
mod utility_widgets;
mod view;
mod weather;

use anyhow::Result;
use diesel::{Connection, SqliteConnection};
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
        }
    }

    Ok(())
}

fn main() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let connection = SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to: {}", database_url));

    let (tx, rx) = mpsc::unbounded_channel();

    thread::spawn(move || {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async_thread(rx))
    });

    println!("Started!");
    let mut app = App::new(tx, connection);
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
