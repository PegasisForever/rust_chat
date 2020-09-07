use std::path::Path;
use tokio::{fs, io};
use log::info;

pub async fn ensure_file_exists(file_path: &str, init_text: &str) -> io::Result<()> {
    let path = Path::new(file_path);
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::write(path, init_text).await?;
        info!("Created file {}.", file_path);
    }
    Ok(())
}
