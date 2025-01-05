#[rustfmt::skip]
pub mod ieee80211;
mod proxy;

use futures_util::StreamExt;
use proxy::{
    dbus_wpa::wpa_supplicant1Proxy, dbus_wpa_bss::BSSProxy, dbus_wpa_interface::InterfaceProxy,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::collections::HashSet;
use std::convert::Infallible;
use strum::ParseError;
use thiserror::Error;

use crate::ieee80211::{Reason, StatusCode};
use zbus::zvariant::OwnedObjectPath;
use zbus::{CacheProperties, Connection, Error as ZbusError};
use zvariant::{OwnedValue, Type};

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

impl From<strum::ParseError> for SupplicantError {
    fn from(value: ParseError) -> Self {
        ZbusError::Variant(zbus::zvariant::Error::Message(format!(
            "Failed to parse State value '{}'",
            value
        )))
        .into()
    }
}

impl From<zvariant::Error> for SupplicantError {
    fn from(value: zvariant::Error) -> Self {
        ZbusError::from(value).into()
    }
}
impl From<Infallible> for SupplicantError {
    fn from(value: Infallible) -> Self {
        ZbusError::from(value).into()
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
    pub async fn get_interface(&self, ifname: &str) -> Result<Interface<'_>> {
        let object_path = self.proxy.get_interface(ifname).await?;

        Interface::new(self.conn.clone(), self.proxy.clone(), object_path).await
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
            .map(|v| {
                let vs: Vec<OwnedValue> = v.try_into().map_err(ZbusError::from)?;
                let vval: Result<HashSet<wpa::KeyMgmt>> = vs
                    .into_iter()
                    .map(|s| wpa::KeyMgmt::try_from(s).map_err(From::from))
                    .collect();
                vval
            })
            .transpose()?;
        let pairwise = wpa_value
            .remove("Pairwise")
            .map(|v| {
                let vs: Vec<OwnedValue> = v.try_into().map_err(ZbusError::from)?;
                let vval: Result<HashSet<wpa::Pairwise>> = vs
                    .into_iter()
                    .map(|s| wpa::Pairwise::try_from(s).map_err(From::from))
                    .collect();
                vval
            })
            .transpose()?;
        let group: Option<wpa::Group> = wpa_value
            .remove("Group")
            .map(|v| {
                let maybe_ov: Option<OwnedValue> = v.try_into().map_err(ZbusError::from)?;
                let val: Option<wpa::Group> = maybe_ov.and_then(|s| wpa::Group::try_from(s).ok());
                Ok::<_, SupplicantError>(val)
            })
            .transpose()
            .map(Option::flatten)?;

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
            .map(|v| {
                let vs: Vec<OwnedValue> = v.try_into().map_err(ZbusError::from)?;
                let vval: Result<HashSet<rsn::KeyMgmt>> = vs
                    .into_iter()
                    .map(|s| rsn::KeyMgmt::try_from(s).map_err(From::from))
                    .collect();
                vval
            })
            .transpose()?;
        let pairwise = wpa_value
            .remove("Pairwise")
            .map(|v| {
                let vs: Vec<OwnedValue> = v.try_into().map_err(ZbusError::from)?;
                let vval: Result<HashSet<rsn::Pairwise>> = vs
                    .into_iter()
                    .map(|s| rsn::Pairwise::try_from(s).map_err(From::from))
                    .collect();
                vval
            })
            .transpose()?;
        let group: Option<rsn::Group> = wpa_value
            .remove("Group")
            .map(|v| {
                let maybe_ov: Option<OwnedValue> = v.try_into().map_err(ZbusError::from)?;
                let val: Option<rsn::Group> = maybe_ov.and_then(|s| rsn::Group::try_from(s).ok());
                Ok::<_, SupplicantError>(val)
            })
            .transpose()
            .map(Option::flatten)?;
        let mgmt_group: Option<rsn::MgmtGroup> = wpa_value
            .remove("MgmtGroup")
            .map(|v| {
                let maybe_ov: Option<OwnedValue> = v.try_into().map_err(ZbusError::from)?;
                let val: Option<rsn::MgmtGroup> =
                    maybe_ov.and_then(|s| rsn::MgmtGroup::try_from(s).ok());
                Ok::<_, SupplicantError>(val)
            })
            .transpose()
            .map(Option::flatten)?;

        Ok(Rsn {
            key_mgmt,
            pairwise,
            group,
            mgmt_group,
        })
    }
}

#[macro_export]
macro_rules! impl_traits_for_fromstr {
    ($ty:ident) => {
        impl TryFrom<zvariant::OwnedValue> for $ty {
            type Error = $crate::SupplicantError;

            fn try_from(value: zvariant::OwnedValue) -> std::result::Result<Self, Self::Error> {
                use std::str::FromStr;
                let value = String::try_from(value)?;
                Ok($ty::from_str(value.as_str())?)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use std::str::FromStr;
                let s = String::deserialize(deserializer)?;
                $ty::from_str(s.as_str()).map_err(serde::de::Error::custom)
            }
        }

        #[cfg(feature = "serde")]
        impl serde::Serialize for $ty {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }
    };
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Wpa {
    pub key_mgmt: Option<HashSet<wpa::KeyMgmt>>,
    pub pairwise: Option<HashSet<wpa::Pairwise>>,
    pub group: Option<wpa::Group>,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Rsn {
    pub key_mgmt: Option<HashSet<rsn::KeyMgmt>>,
    pub pairwise: Option<HashSet<rsn::Pairwise>>,
    pub group: Option<rsn::Group>,
    pub mgmt_group: Option<rsn::MgmtGroup>,
}

mod wpa {
    use strum::EnumString;
    use zvariant::Type;

    // Key management suite. Possible array elements: "wpa-psk", "wpa-eap", "wpa-none"
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, EnumString, Type, strum::Display)]
    #[strum(serialize_all = "kebab-case")]
    #[zvariant(signature = "s", rename_all = "kebab-case")]
    #[allow(clippy::enum_variant_names)]
    pub enum KeyMgmt {
        WpaPsk,
        WpaEap,
        WpaNone,
    }
    impl_traits_for_fromstr!(KeyMgmt);

    // Pairwise cipher suites. Possible array elements: "ccmp", "tkip"
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, EnumString, Type, strum::Display)]
    #[strum(serialize_all = "kebab-case")]
    #[zvariant(signature = "s", rename_all = "kebab-case")]
    pub enum Pairwise {
        Ccmp,
        Tkip,
    }
    impl_traits_for_fromstr!(Pairwise);

    // Group cipher suite. Possible values are: "ccmp", "tkip", "wep104", "wep40"
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, EnumString, Type, strum::Display)]
    #[strum(serialize_all = "kebab-case")]
    #[zvariant(signature = "s", rename_all = "kebab-case")]
    pub enum Group {
        Ccmp,
        Tkip,
        Wep104,
        Wep40,
    }
    impl_traits_for_fromstr!(Group);
}
mod rsn {
    use strum::EnumString;
    use zvariant::Type;

    // Key management suite. Possible array elements: "wpa-psk", "wpa-eap", "wpa-ft-psk", "wpa-ft-eap", "wpa-psk-sha256", "wpa-eap-sha256"
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, EnumString, Type, strum::Display)]
    #[strum(serialize_all = "kebab-case")]
    #[zvariant(signature = "s")]
    #[allow(clippy::enum_variant_names)]
    pub enum KeyMgmt {
        WpaPsk,
        WpaEap,
        WpaFtPsk,
        WpaFtEap,
        WpaPskSha256,
        WpaEapSha256,
    }
    impl_traits_for_fromstr!(KeyMgmt);

    // Pairwise cipher suites. Possible array elements: "ccmp", "tkip"
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, EnumString, Type, strum::Display)]
    #[strum(serialize_all = "kebab-case")]
    #[zvariant(signature = "s")]
    pub enum Pairwise {
        Ccmp,
        Tkip,
    }
    impl_traits_for_fromstr!(Pairwise);

    // Group cipher suite. Possible values are: "ccmp", "tkip", "wep104", "wep40"
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, EnumString, Type, strum::Display)]
    #[strum(serialize_all = "kebab-case")]
    #[zvariant(signature = "s")]
    pub enum Group {
        Ccmp,
        Tkip,
        Wep104,
        Wep40,
    }
    impl_traits_for_fromstr!(Group);

    // 	Mangement frames cipher suite. Possible values are: "aes128cmac"
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, EnumString, Type, strum::Display)]
    #[strum(serialize_all = "kebab-case")]
    #[zvariant(signature = "s")]
    pub enum MgmtGroup {
        Aes128cmac,
    }
    impl_traits_for_fromstr!(MgmtGroup);
}

/// "disconnected", "inactive", "scanning", "authenticating", "associating", "associated", "4way_handshake", "group_handshake", "completed","unknown".
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, strum::EnumString, Type, strum::Display)]
#[strum(serialize_all = "snake_case")]
#[zvariant(signature = "s")]
pub enum InterfaceState {
    Disconnected,
    Inactive,
    Scanning,
    Authenticating,
    Associating,
    Associated,
    #[strum(serialize = "4way_handshake")]
    FourwayHandshake,
    GroupHandshake,
    Completed,
    Unknown,
    InterfaceDisabled,
}
impl_traits_for_fromstr!(InterfaceState);
