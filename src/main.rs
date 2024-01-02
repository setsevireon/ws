use anyhow::Result;
use reqwest::blocking::ClientBuilder;

fn main() -> Result<()> {
    let client = ClientBuilder::new().unwrap()..danger_accept_invalid_certs(true).build().unwrap();
    let body = client.get("https://lpw6001tws03.unix.fidelidademundial.com:31116/twsd")?.text()?;

    println!("body = {:?}", body);

    Ok(())
}
