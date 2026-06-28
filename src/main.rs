use futures::StreamExt;
use tokio::signal;

use crate::bluetooth::hfp::{self, HFP};

mod bluetooth;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::Config::load()?;
    println!("{:#?}", config);

    let (session, adapter) = bluetooth::Bluetooth::setup_bluetooth(&config.bluetooth).await?;
    let (mut hfp_handle, mut map_handle, mut pbap_handle) =
        bluetooth::Bluetooth::register_profiles(&session).await?;
    let agent_handle = bluetooth::Bluetooth::register_agent(&session).await?;

    let mut hfp: HFP = HFP {
        hfp_handle: hfp_handle,
    };

    tokio::spawn(async move { HFP::handle_hfp_profile(hfp).await });

    tokio::spawn(async move {
        while let Some(req) = map_handle.next().await {
            println!("{:#?}", req);
        }
    });
    tokio::spawn(async move {
        while let Some(req) = pbap_handle.next().await {
            println!("{:#?}", req);
        }
    });

    signal::ctrl_c().await?;
    Ok(())
}
