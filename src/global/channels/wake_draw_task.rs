use tokio::sync::watch::{self, Receiver, Sender};

struct WakeDrawTaskType;

#[derive(Clone)]
pub struct WakeDrawTaskReceiver(Receiver<WakeDrawTaskType>);

impl WakeDrawTaskReceiver {
    pub async fn wake(&mut self) {
        self.0.borrow_and_update();
        _ = self.0.changed().await;
    }
}

#[derive(Clone)]
pub struct WakeDrawTask(Sender<WakeDrawTaskType>);

impl WakeDrawTask {
    pub fn wake(&self) {
        _ = self.0.send(WakeDrawTaskType);
    }
}

pub fn make() -> (WakeDrawTask, WakeDrawTaskReceiver) {
    let (sender, receiver) = watch::channel(WakeDrawTaskType);

    (WakeDrawTask(sender), WakeDrawTaskReceiver(receiver))
}
