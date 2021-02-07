use termion::event::Key;

use tokio::{
    sync::mpsc::{self, error::SendError, Receiver, Sender},
};

pub struct Events<T> {
    id: usize,
    name: String,
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> Events<T> {
    pub fn new(id: usize, name: String, cap: usize) -> Events<T> {
        let (sender, receiver) = mpsc::channel::<T>(cap);
        Events {
            id,
            name,
            sender,
            receiver,
        }
    }

    pub fn sender(&self) -> Sender<T> {
        self.sender.clone()
    }

    pub async fn send(&self, value: T) -> Result<(), SendError<T>> {
        self.sender.send(value).await
    }

    pub async fn receive(&mut self) -> Option<T> {
        self.receiver.recv().await
    }
}

pub enum Event {
    Key(Key),
    Frame,
}
