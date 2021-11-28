use dominator::routing;
use wasm_bindgen::prelude::*;
use web_sys::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    PickWasmModule,
    Completed,
    Home,
}

impl Route {
    // This could use more advanced URL parsing, but it isn't needed
    pub fn from_url(url: &str) -> Self {
        let url = Url::new(&url).unwrap_throw();
        match url.hash().as_str() {
            "#/pick" => Route::PickWasmModule,
            "#/completed" => Route::Completed,
            _ => Route::Home,
        }
    }

    pub fn to_url(&self) -> &'static str {
        match self {
            Route::PickWasmModule => "#/pick",
            Route::Completed => "#/completed",
            Route::Home => "#/",
        }
    }
}

impl Default for Route {
    fn default() -> Self {
        // Create the Route based on the current URL
        Self::from_url(&routing::url().lock_ref())
    }
}
