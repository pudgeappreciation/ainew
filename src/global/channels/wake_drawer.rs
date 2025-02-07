use tokio::sync::watch::{self, Receiver, Sender};

struct WakeDrawerType;

#[derive(Clone)]
pub struct WakeDrawerReceiver(Receiver<WakeDrawerType>);

impl WakeDrawerReceiver {
    pub async fn wake(&mut self) {
        self.0.borrow_and_update();
        _ = self.0.changed().await;
    }
}

#[derive(Clone)]
pub struct WakeDrawer(Sender<WakeDrawerType>);

impl WakeDrawer {
    pub fn wake(&self) {
        _ = self.0.send(WakeDrawerType);
    }
}

pub fn make() -> (WakeDrawer, WakeDrawerReceiver) {
    let (sender, receiver) = watch::channel(WakeDrawerType);

    (WakeDrawer(sender), WakeDrawerReceiver(receiver))
}
