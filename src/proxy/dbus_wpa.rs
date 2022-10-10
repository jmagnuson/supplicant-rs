//! # DBus interface proxy for: `fi.w1.wpa_supplicant1`
//!
//! This code was generated by `zbus-xmlgen` `3.0.0` from DBus introspection data.
//! Source: `DBusWpa.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus::dbus_proxy;

// #[dbus_proxy(interface = "fi.w1.wpa_supplicant1")]
#[dbus_proxy(
    interface = "fi.w1.wpa_supplicant1",
    default_service = "fi.w1.wpa_supplicant1",
    default_path = "/fi/w1/wpa_supplicant1"
)]
trait wpa_supplicant1 {
    /// CreateInterface method
    fn create_interface(
        &self,
        args: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// ExpectDisconnect method
    fn expect_disconnect(&self) -> zbus::Result<()>;

    /// GetInterface method
    fn get_interface(&self, ifname: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// RemoveInterface method
    fn remove_interface(&self, path: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// InterfaceAdded signal
    #[dbus_proxy(signal)]
    fn interface_added(
        &self,
        path: zbus::zvariant::ObjectPath<'_>,
        properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// InterfaceRemoved signal
    #[dbus_proxy(signal)]
    fn interface_removed(&self, path: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// PropertiesChanged signal
    #[dbus_proxy(signal)]
    fn properties_changed(
        &self,
        properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Capabilities property
    #[dbus_proxy(property)]
    fn capabilities(&self) -> zbus::Result<Vec<String>>;

    /// DebugLevel property
    #[dbus_proxy(property)]
    fn debug_level(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_debug_level(&self, value: &str) -> zbus::Result<()>;

    /// DebugShowKeys property
    #[dbus_proxy(property)]
    fn debug_show_keys(&self) -> zbus::Result<bool>;
    #[dbus_proxy(property)]
    fn set_debug_show_keys(&self, value: bool) -> zbus::Result<()>;

    /// DebugTimestamp property
    #[dbus_proxy(property)]
    fn debug_timestamp(&self) -> zbus::Result<bool>;
    #[dbus_proxy(property)]
    fn set_debug_timestamp(&self, value: bool) -> zbus::Result<()>;

    /// EapMethods property
    #[dbus_proxy(property)]
    fn eap_methods(&self) -> zbus::Result<Vec<String>>;

    /// Interfaces property
    #[dbus_proxy(property)]
    fn interfaces(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// WFDIEs property
    #[dbus_proxy(property)]
    fn wfdies(&self) -> zbus::Result<Vec<u8>>;
    #[dbus_proxy(property)]
    fn set_wfdies(&self, value: &[u8]) -> zbus::Result<()>;
}