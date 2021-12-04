use poem::{error::InternalServerError, handler, Error};
use rfd::AsyncFileDialog;

#[handler]
pub async fn pick_module_handler() -> Result<String, Error> {
    let file = AsyncFileDialog::new()
        .pick_file()
        .await
        .ok_or(InternalServerError("No file chosen"))?;
    let path = file
        .path()
        .to_str()
        .ok_or(InternalServerError("No file chosen"))?;
    Ok(path.to_string())
}
