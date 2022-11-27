use futures_util::stream::StreamExt;
use std::error::Error;
use supplicant::{Interface, Supplicant};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let supplicant = Supplicant::connect().await?;

    let wlan_interface = find_interface(&supplicant, "wlan0")
        .await
        .ok_or("Failed to find wlan0")??;

    let state_stream = wlan_interface.state_stream().await;
    tokio::pin!(state_stream);
    while let Some(state_res) = state_stream.next().await {
        println!("new state: {:?}", state_res);
    }
    Ok(())
}

async fn find_interface<'a>(
    supplicant: &'a Supplicant<'_>,
    iface_name: impl AsRef<str>,
) -> Option<Result<Interface<'a>, zbus::Error>> {
    let mut iface_res: Option<Result<_, zbus::Error>> = None;
    for iface in supplicant.interfaces().await.unwrap().into_iter() {
        let ifname = iface.ifname().await;

        match ifname {
            Ok(name) if &name == iface_name.as_ref() => {
                iface_res = Some(Ok(iface));
                break;
            }
            // Store the last err to return at the end
            Err(e) => {
                iface_res = Some(Err(e));
            }
            // Ignore other ifaces
            Ok(_) => {}
        }
    }
    iface_res
}