use crate::handlers::pick_module_handler;

use poem::Route;
pub fn get_routes() -> Route {
    Route::new().at("/pick_module", pick_module_handler)
}
