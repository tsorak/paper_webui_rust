use tokio::{sync::mpsc, task::JoinHandle};

use super::instance::InstanceConfig;

pub struct Runner {
    tx: mpsc::UnboundedSender<Req>,
    handle: JoinHandle<()>,
}

impl Runner {
    pub(in crate::core) fn new() -> Self {
        let ch = mpsc::unbounded_channel();

        let handle = event_loop::RunnerEventLoop::spawn(ch.1);

        Self { tx: ch.0, handle }
    }

    pub fn new_instance(&self, cfg: InstanceConfig) {
        let _ = self.tx.send(Req::NewInstance(cfg));
    }
}

pub enum Req {
    NewInstance(InstanceConfig),
}

mod event_loop {
    use tokio::{sync::mpsc, task::JoinHandle};

    use crate::core::instance::Instance;

    use super::Req;

    pub(super) struct RunnerEventLoop;

    impl RunnerEventLoop {
        pub(super) fn spawn(mut rx: mpsc::UnboundedReceiver<Req>) -> JoinHandle<()> {
            tokio::task::spawn(async move {
                while let Some(req) = rx.recv().await {
                    let mut instances: Vec<Instance> = vec![];

                    match req {
                        Req::NewInstance(cfg) => {
                            if let Ok(inst) = Instance::new(cfg) {
                                instances.push(inst)
                            }
                        }
                    }
                }
            })
        }
    }
}
