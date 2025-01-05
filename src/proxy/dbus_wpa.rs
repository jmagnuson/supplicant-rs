//! # D-Bus interface proxy for: `fi.w1.wpa_supplicant1`
//!
//! This code was generated by `zbus-xmlgen` `5.0.1` from D-Bus introspection data.
//! Source: `fi.w1.wpa_supplicant1.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the [Writing a client proxy] section of the zbus
//! documentation.
//!
//! This type implements the [D-Bus standard interfaces], (`org.freedesktop.DBus.*`) for which the
//! following zbus API can be used:
//!
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PropertiesProxy`]
//!
//! Consequently `zbus-xmlgen` did not generate code for the above interfaces.
//!
//! [Writing a client proxy]: https://dbus2.github.io/zbus/client.html
//! [D-Bus standard interfaces]: https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces,
use zbus::proxy;
#[proxy(
    interface = "fi.w1.wpa_supplicant1",
    default_service = "fi.w1.wpa_supplicant1",
    default_path = "/fi/w1/wpa_supplicant1"
)]
pub trait wpa_supplicant1 {
    /// CreateInterface method
    fn create_interface(
        &self,
        args: std::collections::HashMap<&str, &zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// ExpectDisconnect method
    fn expect_disconnect(&self) -> zbus::Result<()>;

    /// GetInterface method
    fn get_interface(&self, ifname: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// RemoveInterface method
    fn remove_interface(&self, path: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// InterfaceAdded signal
    #[zbus(signal)]
    fn interface_added(
        &self,
        path: zbus::zvariant::ObjectPath<'_>,
        properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// InterfaceRemoved signal
    #[zbus(signal)]
    fn interface_removed(&self, path: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// PropertiesChanged signal
    #[zbus(signal)]
    fn properties_changed(
        &self,
        properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Capabilities property
    #[zbus(property)]
    fn capabilities(&self) -> zbus::Result<Vec<String>>;

    /// DebugLevel property
    #[zbus(property)]
    fn debug_level(&self) -> zbus::Result<String>;
    #[zbus(property)]
    fn set_debug_level(&self, value: &str) -> zbus::Result<()>;

    /// DebugShowKeys property
    #[zbus(property)]
    fn debug_show_keys(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn set_debug_show_keys(&self, value: bool) -> zbus::Result<()>;

    /// DebugTimestamp property
    #[zbus(property)]
    fn debug_timestamp(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn set_debug_timestamp(&self, value: bool) -> zbus::Result<()>;

    /// EapMethods property
    #[zbus(property)]
    fn eap_methods(&self) -> zbus::Result<Vec<String>>;

    /// Interfaces property
    #[zbus(property)]
    fn interfaces(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
}
