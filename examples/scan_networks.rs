use std::error::Error;
use supplicant::{Interface, Supplicant};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let supplicant = Supplicant::connect().await?;

    let wlan_interface = find_interface(&supplicant, "wlan0")
        .await
        .ok_or("Failed to find wlan0")??;

    wlan_interface.scan().await?;
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    for network in wlan_interface.list_networks().await?.into_iter() {
        print_network_info(network).await?;
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

async fn print_network_info(network: supplicant::Bss<'_>) -> Result<(), zbus::Error> {
    let b = network.bssid().await?;
    let bssid = macaddr::MacAddr6::new(b[0], b[1], b[2], b[3], b[4], b[5]);
    let frequency = network.frequency().await?;
    let ssid = String::from_utf8_lossy(&network.ssid().await?).to_string();
    let signal = network.signal().await?;
    let wpa = network.wpa().await?;

    println!(
        "\nSSID:\t{}\nBSSID:\t{}\nFREQ:\t{}\nSIGNAL:\t{}\nWPA:\t{:?}",
        ssid, bssid, frequency, signal, wpa,
    );

    Ok(())
}
