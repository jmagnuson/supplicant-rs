pub mod ieee80211;
mod proxy;

use futures_util::StreamExt;
use proxy::{
    dbus_wpa::wpa_supplicant1Proxy, dbus_wpa_bss::BSSProxy, dbus_wpa_interface::InterfaceProxy,
};
use std::str::FromStr;
use thiserror::Error;

use crate::ieee80211::{Reason, StatusCode};
use zbus::zvariant::OwnedObjectPath;
use zbus::{CacheProperties, Connection, Error as ZbusError};

pub const SUPPLICANT_DBUS_NAME: &str = "fi.w1.wpa_supplicant1";

pub type Result<T> = std::result::Result<T, SupplicantError>;

// TODO: replace zbus errors with this
#[derive(Error, Debug)]
pub enum SupplicantError {
    #[error(transparent)]
    Dbus(DbusError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Error, Debug)]
#[error(transparent)]
pub struct DbusError {
    inner: ZbusError,
}

impl From<ZbusError> for SupplicantError {
    fn from(zbus_err: ZbusError) -> Self {
        match zbus_err {
            #[allow(deprecated)]
            ZbusError::Io(io_err) => io_err.into(),
            ZbusError::InputOutput(io_err) => std::io::Error::new(io_err.kind(), io_err).into(),
            _ => SupplicantError::Dbus(DbusError { inner: zbus_err }),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Supplicant<'a> {
    conn: Connection,
    proxy: wpa_supplicant1Proxy<'a>,
}
impl<'a> Supplicant<'a> {
    /// Create a new `Supplicant` instance.
    #[tracing::instrument]
    pub async fn connect() -> Result<Supplicant<'a>> {
        let conn = Connection::system().await?;

        let proxy = wpa_supplicant1Proxy::new(&conn).await?;

        Ok(Supplicant { conn, proxy })
    }

    #[tracing::instrument]
    pub async fn interfaces(&self) -> Result<Vec<Interface<'_>>> {
        let interfaces = self.proxy.interfaces().await?;

        futures_util::future::join_all(
            interfaces.into_iter().map(|object_path| {
                Interface::new(self.conn.clone(), self.proxy.clone(), object_path)
            }),
        )
        .await
        .into_iter()
        .collect::<Result<_>>()
    }

    #[tracing::instrument]
    pub async fn receive_interface_added(
        &self,
    ) -> impl futures_util::Stream<Item = Result<Interface<'_>>> + '_ {
        let iface_added_stream = self.proxy.receive_interface_added().await.unwrap();
        iface_added_stream.then(move |iface_added| {
            let proxy = self.proxy.clone();
            async move {
                let object_path = iface_added.args()?.path;
                Interface::new(self.conn.clone(), proxy, object_path.into()).await
            }
        })
    }

    #[tracing::instrument]
    pub async fn receive_interface_removed(
        &self,
    ) -> impl futures_util::Stream<Item = Result<OwnedObjectPath>> + '_ {
        let iface_removed_stream = self.proxy.receive_interface_removed().await.unwrap();
        iface_removed_stream.then(move |iface_removed| async move {
            let args = iface_removed.args()?;
            let object_path = args.path;
            Ok(object_path.into())
        })
    }
}

#[derive(Clone, Debug)]
pub struct Interface<'a> {
    conn: Connection,
    _path: OwnedObjectPath,
    proxy: InterfaceProxy<'a>,
    supplicant_proxy: wpa_supplicant1Proxy<'a>,
}

impl<'a> Interface<'a> {
    #[tracing::instrument]
    pub(crate) async fn new(
        conn: Connection,
        supplicant_proxy: wpa_supplicant1Proxy<'a>,
        interface_path: OwnedObjectPath,
    ) -> Result<Interface<'a>> {
        let proxy = InterfaceProxy::builder(&conn)
            .destination(SUPPLICANT_DBUS_NAME)?
            .path(interface_path.clone())?
            .cache_properties(CacheProperties::Lazily)
            .build()
            .await?;

        Ok(Interface {
            conn,
            _path: interface_path,
            proxy,
            supplicant_proxy,
        })
    }

    #[tracing::instrument]
    pub async fn scan(&self) -> Result<()> {
        use std::collections::HashMap;
        let mut args: HashMap<&str, zbus::zvariant::Value<'_>> = HashMap::new();

        args.insert("Type", "active".into());

        self.proxy.scan(args).await.map_err(From::from)
    }

    #[tracing::instrument]
    pub async fn receive_scan_done(&self) -> impl futures_util::Stream<Item = Result<bool>> + 'a {
        // TODO: no unwrap
        let scan_done_stream = self.proxy.receive_scan_done().await.unwrap();
        scan_done_stream.then(|signal| async move {
            tracing::trace!("signal: {:?}", &signal);
            Ok(signal.args()?.success)
        })
    }

    #[tracing::instrument]
    pub async fn list_networks(&self) -> Result<Vec<Bss<'_>>> {
        let bsss = self.proxy.bsss().await?;
        futures_util::future::join_all(bsss.into_iter().map(|object_path| {
            Bss::new(
                self.conn.clone(),
                self.supplicant_proxy.clone(),
                object_path,
            )
        }))
        .await
        .into_iter()
        .collect::<Result<_>>()
    }

    #[tracing::instrument]
    pub async fn ifname(&self) -> Result<String> {
        self.proxy.ifname().await.map_err(From::from)
    }

    #[tracing::instrument]
    pub async fn state(&self) -> Result<InterfaceState> {
        self.proxy.state().await?.parse().map_err(From::from)
    }

    #[tracing::instrument]
    pub async fn receive_state_changed(
        &self,
    ) -> impl futures_util::Stream<Item = Result<InterfaceState>> + 'a {
        let proxy = self.proxy.clone();
        proxy
            .receive_state_changed()
            .await
            .then(|state_prop| async move {
                state_prop.get().await.map_err(From::from).and_then(|val| {
                    tracing::trace!("state: {:?}", &val);
                    val.parse().map_err(From::from)
                })
            })
    }

    #[tracing::instrument]
    pub async fn disconnect_reason(&self) -> Result<Reason> {
        Ok(self.proxy.disconnect_reason().await?.into())
    }

    #[tracing::instrument]
    pub async fn association_status(&self) -> Result<StatusCode> {
        Ok((self.proxy.assoc_status_code().await? as u16).into())
    }

    #[tracing::instrument]
    pub async fn authentication_status(&self) -> Result<StatusCode> {
        Ok((self.proxy.auth_status_code().await? as u16).into())
    }
}

#[derive(Clone, Debug)]
pub struct Bss<'a> {
    _conn: Connection,
    _path: OwnedObjectPath,
    proxy: BSSProxy<'a>,
    _supplicant_proxy: wpa_supplicant1Proxy<'a>,
}

impl<'a> Bss<'a> {
    #[tracing::instrument]
    pub(crate) async fn new(
        conn: Connection,
        supplicant_proxy: wpa_supplicant1Proxy<'a>,
        bss_path: OwnedObjectPath,
    ) -> Result<Bss<'a>> {
        let proxy = BSSProxy::builder(&conn)
            .destination(SUPPLICANT_DBUS_NAME)?
            .path(bss_path.clone())?
            .cache_properties(CacheProperties::Lazily)
            .build()
            .await?;

        Ok(Bss {
            _conn: conn,
            _path: bss_path,
            proxy,
            _supplicant_proxy: supplicant_proxy,
        })
    }

    pub async fn bssid(&self) -> Result<Vec<u8>> {
        self.proxy.bssid().await.map_err(From::from)
    }

    pub async fn frequency(&self) -> Result<u16> {
        self.proxy.frequency().await.map_err(From::from)
    }

    pub async fn ssid(&self) -> Result<Vec<u8>> {
        self.proxy.ssid().await.map_err(From::from)
    }

    pub async fn signal(&self) -> Result<i16> {
        self.proxy.signal().await.map_err(From::from)
    }

    pub async fn wpa(&self) -> Result<Wpa> {
        let mut wpa_value = self.proxy.wpa().await?;
        let key_mgmt = wpa_value
            .remove("KeyMgmt")
            .map(|v| v.try_into())
            .transpose()
            .map_err(ZbusError::from)?;
        let pairwise = wpa_value
            .remove("Pairwise")
            .map(|v| v.try_into())
            .transpose()
            .map_err(ZbusError::from)?;
        let group = wpa_value
            .remove("Group")
            .map(|v| v.try_into())
            .transpose()
            .map_err(ZbusError::from)?;

        Ok(Wpa {
            key_mgmt,
            pairwise,
            group,
        })
    }

    pub async fn rsn(&self) -> Result<Rsn> {
        let mut wpa_value = self.proxy.rsn().await?;
        let key_mgmt = wpa_value
            .remove("KeyMgmt")
            .map(|v| v.try_into())
            .transpose()
            .map_err(ZbusError::from)?;
        let pairwise = wpa_value
            .remove("Pairwise")
            .map(|v| v.try_into())
            .transpose()
            .map_err(ZbusError::from)?;
        let group = wpa_value
            .remove("Group")
            .map(|v| v.try_into())
            .transpose()
            .map_err(ZbusError::from)?;
        let mgmt_group = wpa_value
            .remove("MgmtGroup")
            .map(|v| v.try_into())
            .transpose()
            .map_err(ZbusError::from)?;

        Ok(Rsn {
            key_mgmt,
            pairwise,
            group,
            mgmt_group,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Wpa {
    pub key_mgmt: Option<Vec<String>>,
    pub pairwise: Option<Vec<String>>,
    pub group: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Rsn {
    pub key_mgmt: Option<Vec<String>>,
    pub pairwise: Option<Vec<String>>,
    pub group: Option<String>,
    pub mgmt_group: Option<String>,
}

/// "disconnected", "inactive", "scanning", "authenticating", "associating", "associated", "4way_handshake", "group_handshake", "completed","unknown".
#[derive(Clone, Debug)]
pub enum InterfaceState {
    Disconnected,
    Inactive,
    Scanning,
    Authenticating,
    Associating,
    Associated,
    FourwayHandshake,
    GroupHandshake,
    Completed,
    Unknown,
    InterfaceDisabled,
}

impl FromStr for InterfaceState {
    type Err = zbus::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use InterfaceState::*;
        let val = match s {
            "disconnected" => Disconnected,
            "inactive" => Inactive,
            "scanning" => Scanning,
            "authenticating" => Authenticating,
            "associating" => Associating,
            "associated" => Associated,
            "4way_handshake" => FourwayHandshake,
            "group_handshake" => GroupHandshake,
            "completed" => Completed,
            "unknown" => Unknown,
            "interface_disabled" => InterfaceDisabled,
            _ => Err(zbus::Error::Variant(zbus::zvariant::Error::Message(
                format!("Failed to parse State value '{}'", s),
            )))?,
        };

        Ok(val)
    }
}
