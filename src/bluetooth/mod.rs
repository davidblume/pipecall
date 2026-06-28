use anyhow::Result;
use bluer::{
    Adapter, Session, Uuid,
    agent::{Agent, AgentHandle, RequestConfirmation},
    rfcomm::{Profile, ProfileHandle, Role},
};
pub mod hfp;

use crate::config::BluetoothConfig;
pub struct Bluetooth {}

impl Bluetooth {
    pub async fn setup_bluetooth(config: &BluetoothConfig) -> Result<(Session, Adapter)> {
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        adapter.set_powered(true).await?;
        adapter.set_alias(config.device_name.clone()).await?;
        adapter.set_discoverable(true).await?;
        adapter.set_pairable(true).await?;
        Ok((session, adapter))
    }

    pub async fn register_profiles(
        session: &Session,
    ) -> Result<(ProfileHandle, ProfileHandle, ProfileHandle)> {
        let hfp_uuid: Uuid = bluer::Uuid::parse_str("0000111e-0000-1000-8000-00805f9b34fb")?;
        let map_uuid: Uuid = bluer::Uuid::parse_str("00001133-0000-1000-8000-00805f9b34fb")?;
        let pbap_uuid: Uuid = bluer::Uuid::parse_str("0000112f-0000-1000-8000-00805f9b34fb")?;

        let hfp_profile = Profile {
            uuid: hfp_uuid,
            name: Some(String::from("Hands-Free")),
            role: Some(Role::Server),
            channel: Some(1),
            require_authentication: Some(false),
            require_authorization: Some(false),
            ..Default::default()
        };

        let map_profile = Profile {
            uuid: map_uuid,
            name: Some(String::from("Message Access")),
            role: Some(Role::Server),
            channel: Some(2),
            require_authentication: Some(false),
            require_authorization: Some(false),
            ..Default::default()
        };

        let pbap_profile = Profile {
            uuid: pbap_uuid,
            name: Some(String::from("Phone Book")),
            role: Some(Role::Server),
            channel: Some(3),
            require_authentication: Some(false),
            require_authorization: Some(false),
            ..Default::default()
        };

        let hfp_profile_handle = session.register_profile(hfp_profile).await?;
        let map_profile_handle = session.register_profile(map_profile).await?;
        let pbap_profile_handle = session.register_profile(pbap_profile).await?;

        Ok((hfp_profile_handle, map_profile_handle, pbap_profile_handle))
    }

    pub async fn register_agent(session: &Session) -> Result<AgentHandle> {
        let agent = Agent {
            request_default: true,
            request_confirmation: Some(Box::new(|req: RequestConfirmation| {
                Box::pin(async move {
                    println!("Please verify this code on your phone: {:06}", req.passkey);

                    Ok(())
                })
            })),
            ..Default::default()
        };

        Ok(session.register_agent(agent).await?)
    }
}
