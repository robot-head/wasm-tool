use crate::App;
use dominator::{clone, events, html, with_node, Dom};
use std::sync::Arc;

#[derive(Default, Debug, Clone, Copy)]
pub struct Home;

impl Home {
    pub fn render(app: Arc<App>) -> Dom {
        html!("div", {
          .style("margin", "6rem 0")
          .children(&mut [
            html!("h1", {
              .text("Hello")
            })
          ])
        })
    }
}
