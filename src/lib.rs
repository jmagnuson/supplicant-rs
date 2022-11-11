mod proxy;
use proxy::{
    dbus_wpa::wpa_supplicant1Proxy, dbus_wpa_bss::BSSProxy, dbus_wpa_interface::InterfaceProxy,
};

use zbus::zvariant::OwnedObjectPath;
use zbus::{CacheProperties, Connection, Result};

pub const SUPPLICANT_DBUS_NAME: &str = "fi.w1.wpa_supplicant1";

// TODO: replace zbus errors with this
pub struct SupplicantError {}

#[derive(Clone, Debug)]
pub struct Supplicant<'a> {
    conn: Connection,
    proxy: wpa_supplicant1Proxy<'a>,
}
impl<'a> Supplicant<'a> {
    /// Create a new `Supplicant` instance.
    pub async fn connect() -> Result<Supplicant<'a>> {
        let conn = Connection::system().await?;

        let proxy = wpa_supplicant1Proxy::new(&conn).await?;

        Ok(Supplicant { conn, proxy })
    }

    pub async fn interfaces(&'a self) -> Result<Vec<Interface<'a>>> {
        let interfaces = self.proxy.interfaces().await?;

        futures_util::future::join_all(
            interfaces
                .into_iter()
                .map(|object_path| Interface::new(self.conn.clone(), &self.proxy, object_path)),
        )
        .await
        .into_iter()
        .collect::<Result<_>>()
    }
}

#[derive(Clone, Debug)]
pub struct Interface<'a> {
    conn: Connection,
    _path: OwnedObjectPath,
    proxy: InterfaceProxy<'a>,
    supplicant_proxy: &'a wpa_supplicant1Proxy<'a>,
}

impl<'a> Interface<'a> {
    pub(crate) async fn new(
        conn: Connection,
        supplicant_proxy: &'a wpa_supplicant1Proxy<'_>,
        interface_path: OwnedObjectPath,
    ) -> Result<Interface<'a>> {
        let proxy = InterfaceProxy::builder(&conn)
            .destination(SUPPLICANT_DBUS_NAME)?
            .path(interface_path.clone())?
            .cache_properties(CacheProperties::No)
            .build()
            .await?;

        Ok(Interface {
            conn,
            _path: interface_path,
            proxy,
            supplicant_proxy,
        })
    }

    pub async fn scan(&'a self) -> Result<()> {
        use std::collections::HashMap;
        let mut args: HashMap<&str, zbus::zvariant::Value<'_>> = HashMap::new();

        args.insert("Type", "active".into());

        self.proxy.scan(args).await
    }

    pub async fn list_networks(&'a self) -> Result<Vec<Bss<'a>>> {
        let bsss = self.proxy.bsss().await?;
        futures_util::future::join_all(
            bsss.into_iter()
                .map(|object_path| Bss::new(self.conn.clone(), self.supplicant_proxy, object_path)),
        )
        .await
        .into_iter()
        .collect::<Result<_>>()
    }

    pub async fn ifname(&'a self) -> Result<String> {
        self.proxy.ifname().await
    }
}

#[derive(Clone, Debug)]
pub struct Bss<'a> {
    _conn: Connection,
    _path: OwnedObjectPath,
    proxy: BSSProxy<'a>,
    _supplicant_proxy: &'a wpa_supplicant1Proxy<'a>,
}

impl<'a> Bss<'a> {
    pub(crate) async fn new(
        conn: Connection,
        supplicant_proxy: &'a wpa_supplicant1Proxy<'_>,
        bss_path: OwnedObjectPath,
    ) -> Result<Bss<'a>> {
        let proxy = BSSProxy::builder(&conn)
            .destination(SUPPLICANT_DBUS_NAME)?
            .path(bss_path.clone())?
            .cache_properties(CacheProperties::No)
            .build()
            .await?;

        Ok(Bss {
            _conn: conn,
            _path: bss_path,
            proxy,
            _supplicant_proxy: supplicant_proxy,
        })
    }

    pub async fn bssid(&'a self) -> Result<Vec<u8>> {
        self.proxy.bssid().await
    }

    pub async fn frequency(&'a self) -> Result<u16> {
        self.proxy.frequency().await
    }

    pub async fn ssid(&'a self) -> Result<Vec<u8>> {
        self.proxy.ssid().await
    }

    pub async fn signal(&'a self) -> Result<i16> {
        self.proxy.signal().await
    }

    pub async fn wpa(&'a self) -> Result<Wpa> {
        let mut wpa_value = self.proxy.wpa().await?;
        let key_mgmt = wpa_value
            .remove("KeyMgmt")
            .map(|v| v.try_into())
            .transpose()?;
        let pairwise = wpa_value
            .remove("Pairwise")
            .map(|v| v.try_into())
            .transpose()?;
        let group = wpa_value
            .remove("Group")
            .map(|v| v.try_into())
            .transpose()?;

        Ok(Wpa {
            key_mgmt,
            pairwise,
            group,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Wpa {
    pub key_mgmt: Option<Vec<String>>,
    pub pairwise: Option<Vec<String>>,
    pub group: Option<String>,
}
