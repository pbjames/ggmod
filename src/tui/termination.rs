use tokio::sync::broadcast::{self, Receiver, Sender};

#[derive(Clone)]
pub struct Termination {
    tx_terminate: Sender<usize>,
}

impl Termination {
    pub fn new() -> (Termination, Receiver<usize>) {
        let (tx, rx) = broadcast::channel::<usize>(32);
        (Termination { tx_terminate: tx }, rx)
    }

    pub fn exit(&self) {
        let _ = self.tx_terminate.send(1);
    }
}
