use error_chain::error_chain;
use std::io::copy;
use std::fs::File;
use tempfile::Builder;

error_chain!{
    foreign_links{
        Io(std::io::Error);
        HttRequest(reqwest::Error);
    }          
}

#[tokio::main]
fn main() -> Result<()> {
let tmp_dir = Builder::new().prefix("example").tempdir();
let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
let response = reqwest::get(target).await?;
Ok(())
}