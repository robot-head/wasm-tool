use crate::App;
use dominator::{clone, events, html, with_node, Dom};
use futures_signals::signal::Mutable;
use std::sync::Arc;

#[derive(Default, Debug, Clone)]
pub struct Home {
    path: Mutable<String>,
}

impl Home {
    pub fn render(home: Arc<Self>, app: Arc<App>) -> Dom {
        html!("div", {
          .style("margin", "6rem 0")
          .children(&mut [
            html!("h3", {
              .text("Welcome to WasmTool.")
            }),
            html!("div", {
              .text_signal(home.path.signal_cloned())
            }),
            html!("bx-btn", {
              .text("Choose WASM module")
              .event(clone!(home => move |_: events::Click| {
                home.path.replace_with(|_| "/some/path".to_string());
              }))
            })
          ])
        })
    }
}
