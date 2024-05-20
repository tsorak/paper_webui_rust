use std::time::Duration;

use lunatic::{Mailbox, MailboxError, Process};
use serde::{Deserialize, Serialize};
use submillisecond::{router, static_router, Application};
use submillisecond_live_view::prelude::*;

fn main() -> std::io::Result<()> {
    Application::new(router! {
        "/" => Counter::handler("index.html", "#app")
        "/static" => static_router!("./static")
    })
    .serve("127.0.0.1:3000")
}

#[derive(Clone, Serialize, Deserialize)]
struct Counter {
    count: i32,
    ticker: Option<Process<i32>>,
}

impl LiveView for Counter {
    type Events = (Increment, Decrement);

    fn mount(_uri: Uri, socket: Option<Socket>) -> Self {
        let ticker = match socket.clone() {
            None => None,
            Some(socket) => {
                let ticker =
                    Process::spawn_link(socket, |mut socket, mailbox: Mailbox<i32>| loop {
                        match mailbox.receive_timeout(Duration::from_millis(500)) {
                            Ok(_ms) => (),
                            Err(MailboxError::TimedOut) => {
                                socket.send_event(Tick {}).unwrap();
                            }
                            err => panic!("{err:?}"),
                        }
                    });

                Some(ticker)
            }
        };

        Self { count: 0, ticker }
    }

    fn render(&self) -> Rendered {
        html! {
            button @click=(Increment) { "Increment" }
            button @click=(Decrement) { "Decrement" }
            p { "Count is " (self.count) }
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Tick {}

impl LiveViewEvent<Tick> for Counter {
    fn handle(state: &mut Self, _event: Tick) {
        state.count += 1;
    }
}

#[derive(Deserialize)]
struct Increment {}

impl LiveViewEvent<Increment> for Counter {
    fn handle(state: &mut Self, _event: Increment) {
        state.count += 1;
    }
}

#[derive(Deserialize)]
struct Decrement {}

impl LiveViewEvent<Decrement> for Counter {
    fn handle(state: &mut Self, _event: Decrement) {
        state.count -= 1;
    }
}
