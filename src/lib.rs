use crate::components::home::Home;
use dominator::{class, clone, events, html, routing, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use routes::Route;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
mod components;
mod routes;

#[derive(Debug)]
pub struct App {
    message: Mutable<String>,
    route: Mutable<Route>,
    home: Arc<Home>,
}

impl App {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            message: Mutable::new("Hello world!".to_string()),
            route: Mutable::new(Route::default()),
            home: Arc::new(Home::default()),
        })
    }

    fn render(app: Arc<Self>) -> Dom {
        // Define CSS styles

        // Create the DOM nodes
        html!("div", {
                    .children(&mut [
                        html!("bx-header", {
                            .attr("aria-label", "foo")
                            .child(html!("bx-header-name", {
                                .attr("prefix", "WasmTool")
                                .text("GUI")
                            }))
                        }),
                        match app.route.read_only().get() {

                            Route::PickWasmModule => Home::render(app.clone()),
                            Route::Completed => Home::render(app.clone()),
                            Route::Home => Home::render(app.clone()),
        }
                    ])
                    .future(routing::url()
                        .signal_ref(|url| Route::from_url(url))
                        .for_each(clone!(app => move |route| {
                            app.route.set_neq(route);
                            async {}
                        })))
                    })
    }

    pub fn route(&self) -> impl Signal<Item = Route> {
        self.route.signal()
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let app = App::new();
    dominator::append_dom(&dominator::body(), App::render(app));

    Ok(())
}
