use axum_live_view::{event_data::EventData, html, live_view::Updated, Html, LiveView};
use serde::{Deserialize, Serialize};

// Our live view is just a regular Rust struct...
#[derive(Default)]
pub struct Counter {
    count: u64,
}

// ...that implements the `LiveView` trait.
impl LiveView for Counter {
    // This is the type of update messages our HTML contains. They will be sent
    // to the view in the `update` method
    type Message = Msg;

    // Update the view based on which message it receives.
    //
    // `EventData` contains data from the event that happened in the
    // browser. This might be values of input fields or which key was pressed in
    // a keyboard event.
    fn update(mut self, msg: Msg, _data: Option<EventData>) -> Updated<Self> {
        match msg {
            Msg::Increment => {
                print!("{} += 1 => ", self.count);
                self.count += 1;
                println!("{}", self.count);
            }
            Msg::Decrement => {
                print!("{} -= 1 => ", self.count);
                if self.count > 0 {
                    self.count -= 1;
                }
                println!("{}", self.count);
            }
        }

        Updated::new(self)
    }

    // Render the live view into an HTML template. This function is called during
    // the initial render in `LiveViewManager::embed` and for each subsequent
    // update.
    //
    // The HTML is diff'ed on the server and only minimal deltas are sent over
    // the wire. The browser then builds the full HTML template and efficiently
    // updates the DOM.
    fn render(&self) -> Html<Self::Message> {
        html! {
            <div>
                "Counter value: "
                // Embed dynamic Rust values into the HTML.
                //
                // `if`, `for`, and `match` are also supported.
                { self.count }
            </div>

            <div>
                // Elements with the `axm-click` attribute will send an update message
                // to the view which calls `update` after which the view is
                // re-rendered.
                <button axm-click={ Msg::Increment }>"+"</button>
                <button axm-click={ Msg::Decrement }>"-"</button>
            </div>
        }
    }

    // The `LiveView` trait also has a `mount` method that is called when a new
    // WebSocket connects. This can be used to perform auth, load data that
    // isn't needed for the first response, and spawn a task that can send
    // messages to the view itself from other parts of the application.
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Msg {
    Increment,
    Decrement,
}
