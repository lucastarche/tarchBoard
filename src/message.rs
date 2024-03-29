use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    oneshot,
};

use crate::weather::WeatherResponse;

pub type OneshotSender<T> = oneshot::Sender<T>;
pub type OneshotReceiver<T> = oneshot::Receiver<T>;
pub type MessageSender = UnboundedSender<Message>;
pub type MessageReceiver = UnboundedReceiver<Message>;

pub enum Message {
    FetchNewResource {
        url: String,
        send: OneshotSender<WeatherResponse>,
    },
}
