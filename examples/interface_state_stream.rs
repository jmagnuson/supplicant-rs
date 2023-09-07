use futures_util::stream::StreamExt;
use std::error::Error;
use supplicant::{Interface, Supplicant, SupplicantError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let supplicant = Supplicant::connect().await?;

    let iface_rm_stream = supplicant.receive_interface_removed().await;
    tokio::pin!(iface_rm_stream);
    let iface_add_stream = supplicant.receive_interface_added().await;
    tokio::pin!(iface_add_stream);

    let mut wlan_interface = find_interface(&supplicant, "wlan0")
        .await
        .ok_or("Failed to find wlan0")??;

    let state_stream = wlan_interface.receive_state_changed().await;
    tokio::pin!(state_stream);

    loop {
        tokio::select! {
            state_res_opt = state_stream.next() => {
                match state_res_opt {
                    Some(state_res) => print_state_info(&wlan_interface, state_res).await,
                    None => break,
                }
            }
            rm_iface = iface_rm_stream.next() => {
                println!("Interface removed: {:?}", rm_iface);
            }
            add_iface = iface_add_stream.next() => {
                let iface = add_iface.unwrap()?;
                let ifname = iface.ifname().await?;
                println!("Interface added: {:?}", ifname);

                if ifname == "wlan0" {
                    wlan_interface = iface;
                    let new_state_stream = wlan_interface.receive_state_changed().await;
                    state_stream.set(new_state_stream);
                }
            }
        }
    }
    println!("Exiting");
    Ok(())
}

async fn print_state_info<'a>(
    wlan_interface: &'a Interface<'_>,
    state_res: supplicant::Result<supplicant::InterfaceState>,
) {
    println!("new state: {:?}", state_res);
    let status_code = wlan_interface.association_status().await;
    println!("  status code: {:?}", status_code);
    if !matches!(status_code, Ok(supplicant::ieee80211::StatusCode::Success))
        || matches!(state_res, Ok(supplicant::InterfaceState::Disconnected))
    {
        let disconnect_reason = wlan_interface.disconnect_reason().await;
        println!("  disconnect reason: {:?}", disconnect_reason);
        let auth_status = wlan_interface.authentication_status().await;
        println!("  auth status: {:?}", auth_status);
    }
}

async fn find_interface<'a>(
    supplicant: &'a Supplicant<'_>,
    iface_name: impl AsRef<str>,
) -> Option<Result<Interface<'a>, SupplicantError>> {
    let mut iface_res: Option<Result<_, SupplicantError>> = None;
    for iface in supplicant.interfaces().await.unwrap().into_iter() {
        let ifname = iface.ifname().await;

        match ifname {
            Ok(name) if name == iface_name.as_ref() => {
                println!("found iface: {:?}", name);
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
