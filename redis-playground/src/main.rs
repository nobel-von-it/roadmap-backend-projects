use anyhow::Result;
use redis::{AsyncCommands, Commands};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Huis {
    name: String,
    address: String,
}

impl Huis {
    fn new<S: AsRef<str>>(name: S, address: S) -> Huis {
        Huis {
            name: name.as_ref().to_string(),
            address: address.as_ref().to_string(),
        }
    }
    fn to_string(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }
    fn from_string<S: AsRef<str>>(s: S) -> Result<Huis> {
        Ok(serde_json::from_str(s.as_ref())?)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_multiplexed_tokio_connection().await?;

    let huis = Huis::new("huis", "adres");

    let _: () = con.set(1, huis.to_string()?).await?;

    let prevalue: String = con.get(1).await?;
    let value = Huis::from_string(prevalue)?;

    println!("value: {:?}", value);

    Ok(())
}
