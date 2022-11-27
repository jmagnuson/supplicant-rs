//! # DBus interface proxies for: `fi.w1.wpa_supplicant1.Interface`, `fi.w1.wpa_supplicant1.Interface.WPS`
//!
//! This code was generated by `zbus-xmlgen` `3.0.0` from DBus introspection data.
//! Source: `fi.w1.wpa_supplicant1.interface.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!
//! This DBus object implements
//! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
//! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
//!
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PropertiesProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.

use zbus::dbus_proxy;

#[dbus_proxy(interface = "fi.w1.wpa_supplicant1.Interface")]
trait Interface {
    /// AbortScan method
    fn abort_scan(&self) -> zbus::Result<()>;

    /// AddBlob method
    fn add_blob(&self, name: &str, data: &[u8]) -> zbus::Result<()>;

    /// AddNetwork method
    fn add_network(
        &self,
        args: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// AutoScan method
    fn auto_scan(&self, arg: &str) -> zbus::Result<()>;

    /// Disconnect method
    fn disconnect(&self) -> zbus::Result<()>;

    /// EAPLogoff method
    fn eaplogoff(&self) -> zbus::Result<()>;

    /// EAPLogon method
    fn eaplogon(&self) -> zbus::Result<()>;

    /// FlushBSS method
    fn flush_bss(&self, age: u32) -> zbus::Result<()>;

    /// GetBlob method
    fn get_blob(&self, name: &str) -> zbus::Result<Vec<u8>>;

    /// NetworkReply method
    fn network_reply(
        &self,
        path: &zbus::zvariant::ObjectPath<'_>,
        field: &str,
        value: &str,
    ) -> zbus::Result<()>;

    /// Reassociate method
    fn reassociate(&self) -> zbus::Result<()>;

    /// Reattach method
    fn reattach(&self) -> zbus::Result<()>;

    /// Reconnect method
    fn reconnect(&self) -> zbus::Result<()>;

    /// RemoveAllNetworks method
    fn remove_all_networks(&self) -> zbus::Result<()>;

    /// RemoveBlob method
    fn remove_blob(&self, name: &str) -> zbus::Result<()>;

    /// RemoveNetwork method
    fn remove_network(&self, path: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// Roam method
    fn roam(&self, addr: &str) -> zbus::Result<()>;

    /// SaveConfig method
    fn save_config(&self) -> zbus::Result<()>;

    /// Scan method
    fn scan(
        &self,
        args: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// SelectNetwork method
    fn select_network(&self, path: &zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// SetPKCS11EngineAndModulePath method
    fn set_pkcs11engine_and_module_path(
        &self,
        pkcs11_engine_path: &str,
        pkcs11_module_path: &str,
    ) -> zbus::Result<()>;

    /// SignalPoll method
    fn signal_poll(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// SubscribeProbeReq method
    fn subscribe_probe_req(&self) -> zbus::Result<()>;

    /// UnsubscribeProbeReq method
    fn unsubscribe_probe_req(&self) -> zbus::Result<()>;

    /// VendorElemAdd method
    fn vendor_elem_add(&self, frame_id: i32, ielems: &[u8]) -> zbus::Result<()>;

    /// VendorElemGet method
    fn vendor_elem_get(&self, frame_id: i32) -> zbus::Result<Vec<u8>>;

    /// VendorElemRem method
    fn vendor_elem_rem(&self, frame_id: i32, ielems: &[u8]) -> zbus::Result<()>;

    /// BSSAdded signal
    #[dbus_proxy(signal)]
    fn bssadded(
        &self,
        path: zbus::zvariant::ObjectPath<'_>,
        properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// BSSRemoved signal
    #[dbus_proxy(signal)]
    fn bssremoved(&self, path: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// BlobAdded signal
    #[dbus_proxy(signal)]
    fn blob_added(&self, name: &str) -> zbus::Result<()>;

    /// BlobRemoved signal
    #[dbus_proxy(signal)]
    fn blob_removed(&self, name: &str) -> zbus::Result<()>;

    /// Certification signal
    #[dbus_proxy(signal)]
    fn certification(
        &self,
        certification: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// EAP signal
    #[dbus_proxy(signal)]
    fn eap(&self, status: &str, parameter: &str) -> zbus::Result<()>;

    /// NetworkAdded signal
    #[dbus_proxy(signal)]
    fn network_added(
        &self,
        path: zbus::zvariant::ObjectPath<'_>,
        properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// NetworkRemoved signal
    #[dbus_proxy(signal)]
    fn network_removed(&self, path: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// NetworkRequest signal
    #[dbus_proxy(signal)]
    fn network_request(
        &self,
        path: zbus::zvariant::ObjectPath<'_>,
        field: &str,
        text: &str,
    ) -> zbus::Result<()>;

    /// NetworkSelected signal
    #[dbus_proxy(signal)]
    fn network_selected(&self, path: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// ProbeRequest signal
    #[dbus_proxy(signal)]
    fn probe_request(
        &self,
        args: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// PropertiesChanged signal
    #[dbus_proxy(signal)]
    fn properties_changed(
        &self,
        properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// ScanDone signal
    #[dbus_proxy(signal)]
    fn scan_done(&self, success: bool) -> zbus::Result<()>;

    /// StaAuthorized signal
    #[dbus_proxy(signal)]
    fn sta_authorized(&self, name: &str) -> zbus::Result<()>;

    /// StaDeauthorized signal
    #[dbus_proxy(signal)]
    fn sta_deauthorized(&self, name: &str) -> zbus::Result<()>;

    /// StationAdded signal
    #[dbus_proxy(signal)]
    fn station_added(
        &self,
        path: zbus::zvariant::ObjectPath<'_>,
        properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// StationRemoved signal
    #[dbus_proxy(signal)]
    fn station_removed(&self, path: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// AccessNetworkType property
    #[dbus_proxy(property)]
    fn access_network_type(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_access_network_type(&self, value: &str) -> zbus::Result<()>;

    /// ApAssocrespElements property
    #[dbus_proxy(property)]
    fn ap_assocresp_elements(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_ap_assocresp_elements(&self, value: &str) -> zbus::Result<()>;

    /// ApIsolate property
    #[dbus_proxy(property)]
    fn ap_isolate(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_ap_isolate(&self, value: &str) -> zbus::Result<()>;

    /// ApScan property
    #[dbus_proxy(property)]
    fn ap_scan(&self) -> zbus::Result<u32>;
    #[dbus_proxy(property)]
    fn set_ap_scan(&self, value: u32) -> zbus::Result<()>;

    /// ApVendorElements property
    #[dbus_proxy(property)]
    fn ap_vendor_elements(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_ap_vendor_elements(&self, value: &str) -> zbus::Result<()>;

    /// AssocStatusCode property
    #[dbus_proxy(property)]
    fn assoc_status_code(&self) -> zbus::Result<i32>;

    /// AuthStatusCode property
    #[dbus_proxy(property)]
    fn auth_status_code(&self) -> zbus::Result<i32>;

    /// AutoInterworking property
    #[dbus_proxy(property)]
    fn auto_interworking(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_auto_interworking(&self, value: &str) -> zbus::Result<()>;

    /// AutoUuid property
    #[dbus_proxy(property)]
    fn auto_uuid(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_auto_uuid(&self, value: &str) -> zbus::Result<()>;

    /// Autoscan property
    #[dbus_proxy(property)]
    fn autoscan(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_autoscan(&self, value: &str) -> zbus::Result<()>;

    /// BSSExpireAge property
    #[dbus_proxy(property)]
    fn bssexpire_age(&self) -> zbus::Result<u32>;
    #[dbus_proxy(property)]
    fn set_bssexpire_age(&self, value: u32) -> zbus::Result<()>;

    /// BSSExpireCount property
    #[dbus_proxy(property)]
    fn bssexpire_count(&self) -> zbus::Result<u32>;
    #[dbus_proxy(property)]
    fn set_bssexpire_count(&self, value: u32) -> zbus::Result<()>;

    /// BSSTMStatus property
    #[dbus_proxy(property)]
    fn bsstmstatus(&self) -> zbus::Result<u32>;

    /// BSSs property
    #[dbus_proxy(property, name = "BSSs")]
    fn bsss(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// BeaconInt property
    #[dbus_proxy(property)]
    fn beacon_int(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_beacon_int(&self, value: &str) -> zbus::Result<()>;

    /// Bgscan property
    #[dbus_proxy(property)]
    fn bgscan(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_bgscan(&self, value: &str) -> zbus::Result<()>;

    /// Blobs property
    #[dbus_proxy(property)]
    fn blobs(&self) -> zbus::Result<std::collections::HashMap<String, Vec<u8>>>;

    /// BridgeIfname property
    #[dbus_proxy(property)]
    fn bridge_ifname(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_bridge_ifname(&self, value: &str) -> zbus::Result<()>;

    /// BssMaxCount property
    #[dbus_proxy(property)]
    fn bss_max_count(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_bss_max_count(&self, value: &str) -> zbus::Result<()>;

    /// Capabilities property
    #[dbus_proxy(property)]
    fn capabilities(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// CertInCb property
    #[dbus_proxy(property)]
    fn cert_in_cb(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_cert_in_cb(&self, value: &str) -> zbus::Result<()>;

    /// ColocIntfReporting property
    #[dbus_proxy(property)]
    fn coloc_intf_reporting(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_coloc_intf_reporting(&self, value: &str) -> zbus::Result<()>;

    /// ConfigFile property
    #[dbus_proxy(property)]
    fn config_file(&self) -> zbus::Result<String>;

    /// ConfigMethods property
    #[dbus_proxy(property)]
    fn config_methods(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_config_methods(&self, value: &str) -> zbus::Result<()>;

    /// Country property
    #[dbus_proxy(property)]
    fn country(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_country(&self, value: &str) -> zbus::Result<()>;

    /// CtrlInterface property
    #[dbus_proxy(property)]
    fn ctrl_interface(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_ctrl_interface(&self, value: &str) -> zbus::Result<()>;

    /// CtrlInterfaceGroup property
    #[dbus_proxy(property)]
    fn ctrl_interface_group(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_ctrl_interface_group(&self, value: &str) -> zbus::Result<()>;

    /// CurrentAuthMode property
    #[dbus_proxy(property)]
    fn current_auth_mode(&self) -> zbus::Result<String>;

    /// CurrentBSS property
    #[dbus_proxy(property)]
    fn current_bss(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// CurrentNetwork property
    #[dbus_proxy(property)]
    fn current_network(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// DeviceName property
    #[dbus_proxy(property)]
    fn device_name(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_device_name(&self, value: &str) -> zbus::Result<()>;

    /// DeviceType property
    #[dbus_proxy(property)]
    fn device_type(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_device_type(&self, value: &str) -> zbus::Result<()>;

    /// DisableScanOffload property
    #[dbus_proxy(property)]
    fn disable_scan_offload(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_disable_scan_offload(&self, value: &str) -> zbus::Result<()>;

    /// DisassocLowAck property
    #[dbus_proxy(property)]
    fn disassoc_low_ack(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_disassoc_low_ack(&self, value: &str) -> zbus::Result<()>;

    /// DisconnectReason property
    #[dbus_proxy(property)]
    fn disconnect_reason(&self) -> zbus::Result<i32>;

    /// Dot11RSNAConfigPMKLifetime property
    #[dbus_proxy(property)]
    fn dot11_rsnaconfig_pmklifetime(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_dot11_rsnaconfig_pmklifetime(&self, value: &str) -> zbus::Result<()>;

    /// Dot11RSNAConfigPMKReauthThreshold property
    #[dbus_proxy(property)]
    fn dot11_rsnaconfig_pmkreauth_threshold(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_dot11_rsnaconfig_pmkreauth_threshold(&self, value: &str) -> zbus::Result<()>;

    /// Dot11RSNAConfigSATimeout property
    #[dbus_proxy(property)]
    fn dot11_rsnaconfig_satimeout(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_dot11_rsnaconfig_satimeout(&self, value: &str) -> zbus::Result<()>;

    /// Driver property
    #[dbus_proxy(property)]
    fn driver(&self) -> zbus::Result<String>;

    /// DriverParam property
    #[dbus_proxy(property)]
    fn driver_param(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_driver_param(&self, value: &str) -> zbus::Result<()>;

    /// DtimPeriod property
    #[dbus_proxy(property)]
    fn dtim_period(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_dtim_period(&self, value: &str) -> zbus::Result<()>;

    /// EapolVersion property
    #[dbus_proxy(property)]
    fn eapol_version(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_eapol_version(&self, value: &str) -> zbus::Result<()>;

    /// ExtPasswordBackend property
    #[dbus_proxy(property)]
    fn ext_password_backend(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_ext_password_backend(&self, value: &str) -> zbus::Result<()>;

    /// ExternalSim property
    #[dbus_proxy(property)]
    fn external_sim(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_external_sim(&self, value: &str) -> zbus::Result<()>;

    /// FastReauth property
    #[dbus_proxy(property)]
    fn fast_reauth(&self) -> zbus::Result<bool>;
    #[dbus_proxy(property)]
    fn set_fast_reauth(&self, value: bool) -> zbus::Result<()>;

    /// FilterRssi property
    #[dbus_proxy(property)]
    fn filter_rssi(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_filter_rssi(&self, value: &str) -> zbus::Result<()>;

    /// FilterSsids property
    #[dbus_proxy(property)]
    fn filter_ssids(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_filter_ssids(&self, value: &str) -> zbus::Result<()>;

    /// FreqList property
    #[dbus_proxy(property)]
    fn freq_list(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_freq_list(&self, value: &str) -> zbus::Result<()>;

    /// FtmInitiator property
    #[dbus_proxy(property)]
    fn ftm_initiator(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_ftm_initiator(&self, value: &str) -> zbus::Result<()>;

    /// FtmResponder property
    #[dbus_proxy(property)]
    fn ftm_responder(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_ftm_responder(&self, value: &str) -> zbus::Result<()>;

    /// GasAddress3 property
    #[dbus_proxy(property)]
    fn gas_address3(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_gas_address3(&self, value: &str) -> zbus::Result<()>;

    /// GasRandAddrLifetime property
    #[dbus_proxy(property)]
    fn gas_rand_addr_lifetime(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_gas_rand_addr_lifetime(&self, value: &str) -> zbus::Result<()>;

    /// GasRandMacAddr property
    #[dbus_proxy(property)]
    fn gas_rand_mac_addr(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_gas_rand_mac_addr(&self, value: &str) -> zbus::Result<()>;

    /// GoAccessNetworkType property
    #[dbus_proxy(property)]
    fn go_access_network_type(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_go_access_network_type(&self, value: &str) -> zbus::Result<()>;

    /// GoInternet property
    #[dbus_proxy(property)]
    fn go_internet(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_go_internet(&self, value: &str) -> zbus::Result<()>;

    /// GoInterworking property
    #[dbus_proxy(property)]
    fn go_interworking(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_go_interworking(&self, value: &str) -> zbus::Result<()>;

    /// GoVenueGroup property
    #[dbus_proxy(property)]
    fn go_venue_group(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_go_venue_group(&self, value: &str) -> zbus::Result<()>;

    /// GoVenueType property
    #[dbus_proxy(property)]
    fn go_venue_type(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_go_venue_type(&self, value: &str) -> zbus::Result<()>;

    /// Hessid property
    #[dbus_proxy(property)]
    fn hessid(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_hessid(&self, value: &str) -> zbus::Result<()>;

    /// Ifname property
    #[dbus_proxy(property)]
    fn ifname(&self) -> zbus::Result<String>;

    /// IgnoreOldScanRes property
    #[dbus_proxy(property)]
    fn ignore_old_scan_res(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_ignore_old_scan_res(&self, value: &str) -> zbus::Result<()>;

    /// InitialFreqList property
    #[dbus_proxy(property)]
    fn initial_freq_list(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_initial_freq_list(&self, value: &str) -> zbus::Result<()>;

    /// Interworking property
    #[dbus_proxy(property)]
    fn interworking(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_interworking(&self, value: &str) -> zbus::Result<()>;

    /// KeyMgmtOffload property
    #[dbus_proxy(property)]
    fn key_mgmt_offload(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_key_mgmt_offload(&self, value: &str) -> zbus::Result<()>;

    /// MACAddressRandomizationMask property
    #[dbus_proxy(property)]
    fn macaddress_randomization_mask(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, Vec<u8>>>;
    #[dbus_proxy(property)]
    fn set_macaddress_randomization_mask(
        &self,
        value: std::collections::HashMap<&str, &[u8]>,
    ) -> zbus::Result<()>;

    /// MacAddr property
    #[dbus_proxy(property)]
    fn mac_addr(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_mac_addr(&self, value: &str) -> zbus::Result<()>;

    /// Manufacturer property
    #[dbus_proxy(property)]
    fn manufacturer(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_manufacturer(&self, value: &str) -> zbus::Result<()>;

    /// MaxNumSta property
    #[dbus_proxy(property)]
    fn max_num_sta(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_max_num_sta(&self, value: &str) -> zbus::Result<()>;

    /// ModelName property
    #[dbus_proxy(property)]
    fn model_name(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_model_name(&self, value: &str) -> zbus::Result<()>;

    /// ModelNumber property
    #[dbus_proxy(property)]
    fn model_number(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_model_number(&self, value: &str) -> zbus::Result<()>;

    /// Networks property
    #[dbus_proxy(property)]
    fn networks(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Okc property
    #[dbus_proxy(property)]
    fn okc(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_okc(&self, value: &str) -> zbus::Result<()>;

    /// OpenscEnginePath property
    #[dbus_proxy(property)]
    fn opensc_engine_path(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_opensc_engine_path(&self, value: &str) -> zbus::Result<()>;

    /// OpensslCiphers property
    #[dbus_proxy(property)]
    fn openssl_ciphers(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_openssl_ciphers(&self, value: &str) -> zbus::Result<()>;

    /// OsVersion property
    #[dbus_proxy(property)]
    fn os_version(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_os_version(&self, value: &str) -> zbus::Result<()>;

    /// OsuDir property
    #[dbus_proxy(property)]
    fn osu_dir(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_osu_dir(&self, value: &str) -> zbus::Result<()>;

    /// P2pGoMaxInactivity property
    #[dbus_proxy(property)]
    fn p2p_go_max_inactivity(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_p2p_go_max_inactivity(&self, value: &str) -> zbus::Result<()>;

    /// P2pSearchDelay property
    #[dbus_proxy(property)]
    fn p2p_search_delay(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_p2p_search_delay(&self, value: &str) -> zbus::Result<()>;

    /// PKCS11EnginePath property
    #[dbus_proxy(property)]
    fn pkcs11engine_path(&self) -> zbus::Result<String>;

    /// PKCS11ModulePath property
    #[dbus_proxy(property)]
    fn pkcs11module_path(&self) -> zbus::Result<String>;

    /// PassiveScan property
    #[dbus_proxy(property)]
    fn passive_scan(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_passive_scan(&self, value: &str) -> zbus::Result<()>;

    /// PbcInM1 property
    #[dbus_proxy(property)]
    fn pbc_in_m1(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_pbc_in_m1(&self, value: &str) -> zbus::Result<()>;

    /// PcscPin property
    #[dbus_proxy(property)]
    fn pcsc_pin(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_pcsc_pin(&self, value: &str) -> zbus::Result<()>;

    /// PcscReader property
    #[dbus_proxy(property)]
    fn pcsc_reader(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_pcsc_reader(&self, value: &str) -> zbus::Result<()>;

    /// Pmf property
    #[dbus_proxy(property)]
    fn pmf(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_pmf(&self, value: &str) -> zbus::Result<()>;

    /// PreassocMacAddr property
    #[dbus_proxy(property)]
    fn preassoc_mac_addr(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_preassoc_mac_addr(&self, value: &str) -> zbus::Result<()>;

    /// RandAddrLifetime property
    #[dbus_proxy(property)]
    fn rand_addr_lifetime(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_rand_addr_lifetime(&self, value: &str) -> zbus::Result<()>;

    /// ReassocSameBssOptim property
    #[dbus_proxy(property)]
    fn reassoc_same_bss_optim(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_reassoc_same_bss_optim(&self, value: &str) -> zbus::Result<()>;

    /// RoamComplete property
    #[dbus_proxy(property)]
    fn roam_complete(&self) -> zbus::Result<bool>;

    /// RoamTime property
    #[dbus_proxy(property)]
    fn roam_time(&self) -> zbus::Result<u32>;

    /// SaeGroups property
    #[dbus_proxy(property)]
    fn sae_groups(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_sae_groups(&self, value: &str) -> zbus::Result<()>;

    /// SaePmkidInAssoc property
    #[dbus_proxy(property)]
    fn sae_pmkid_in_assoc(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_sae_pmkid_in_assoc(&self, value: &str) -> zbus::Result<()>;

    /// SaePwe property
    #[dbus_proxy(property)]
    fn sae_pwe(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_sae_pwe(&self, value: &str) -> zbus::Result<()>;

    /// ScanCurFreq property
    #[dbus_proxy(property)]
    fn scan_cur_freq(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_scan_cur_freq(&self, value: &str) -> zbus::Result<()>;

    /// ScanInterval property
    #[dbus_proxy(property)]
    fn scan_interval(&self) -> zbus::Result<i32>;
    #[dbus_proxy(property)]
    fn set_scan_interval(&self, value: i32) -> zbus::Result<()>;

    /// ScanResValidForConnect property
    #[dbus_proxy(property)]
    fn scan_res_valid_for_connect(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_scan_res_valid_for_connect(&self, value: &str) -> zbus::Result<()>;

    /// Scanning property
    #[dbus_proxy(property)]
    fn scanning(&self) -> zbus::Result<bool>;

    /// SchedScanInterval property
    #[dbus_proxy(property)]
    fn sched_scan_interval(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_sched_scan_interval(&self, value: &str) -> zbus::Result<()>;

    /// SchedScanPlans property
    #[dbus_proxy(property)]
    fn sched_scan_plans(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_sched_scan_plans(&self, value: &str) -> zbus::Result<()>;

    /// SchedScanStartDelay property
    #[dbus_proxy(property)]
    fn sched_scan_start_delay(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_sched_scan_start_delay(&self, value: &str) -> zbus::Result<()>;

    /// SerialNumber property
    #[dbus_proxy(property)]
    fn serial_number(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_serial_number(&self, value: &str) -> zbus::Result<()>;

    /// SessionLength property
    #[dbus_proxy(property)]
    fn session_length(&self) -> zbus::Result<u32>;

    /// State property
    #[dbus_proxy(property)]
    fn state(&self) -> zbus::Result<String>;

    /// Stations property
    #[dbus_proxy(property)]
    fn stations(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// TdlsExternalControl property
    #[dbus_proxy(property)]
    fn tdls_external_control(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_tdls_external_control(&self, value: &str) -> zbus::Result<()>;

    /// UpdateConfig property
    #[dbus_proxy(property)]
    fn update_config(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_update_config(&self, value: &str) -> zbus::Result<()>;

    /// Uuid property
    #[dbus_proxy(property)]
    fn uuid(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_uuid(&self, value: &str) -> zbus::Result<()>;

    /// WowlanDisconnectOnDeinit property
    #[dbus_proxy(property)]
    fn wowlan_disconnect_on_deinit(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_wowlan_disconnect_on_deinit(&self, value: &str) -> zbus::Result<()>;

    /// WowlanTriggers property
    #[dbus_proxy(property)]
    fn wowlan_triggers(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_wowlan_triggers(&self, value: &str) -> zbus::Result<()>;

    /// WpaRscRelaxation property
    #[dbus_proxy(property)]
    fn wpa_rsc_relaxation(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_wpa_rsc_relaxation(&self, value: &str) -> zbus::Result<()>;

    /// WpsCredAddSae property
    #[dbus_proxy(property)]
    fn wps_cred_add_sae(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_wps_cred_add_sae(&self, value: &str) -> zbus::Result<()>;

    /// WpsCredProcessing property
    #[dbus_proxy(property)]
    fn wps_cred_processing(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_wps_cred_processing(&self, value: &str) -> zbus::Result<()>;

    /// WpsNfcDevPw property
    #[dbus_proxy(property)]
    fn wps_nfc_dev_pw(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_wps_nfc_dev_pw(&self, value: &str) -> zbus::Result<()>;

    /// WpsNfcDevPwId property
    #[dbus_proxy(property)]
    fn wps_nfc_dev_pw_id(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_wps_nfc_dev_pw_id(&self, value: &str) -> zbus::Result<()>;

    /// WpsNfcDhPrivkey property
    #[dbus_proxy(property)]
    fn wps_nfc_dh_privkey(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_wps_nfc_dh_privkey(&self, value: &str) -> zbus::Result<()>;

    /// WpsNfcDhPubkey property
    #[dbus_proxy(property)]
    fn wps_nfc_dh_pubkey(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_wps_nfc_dh_pubkey(&self, value: &str) -> zbus::Result<()>;

    /// WpsPriority property
    #[dbus_proxy(property)]
    fn wps_priority(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_wps_priority(&self, value: &str) -> zbus::Result<()>;

    /// WpsVendorExtM1 property
    #[dbus_proxy(property)]
    fn wps_vendor_ext_m1(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_wps_vendor_ext_m1(&self, value: &str) -> zbus::Result<()>;
}

/*
#[dbus_proxy(interface = "fi.w1.wpa_supplicant1.Interface.WPS")]
trait WPS {
    /// Cancel method
    fn cancel(&self) -> zbus::Result<()>;

    /// Start method
    fn start(
        &self,
        args: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;

    /// Credentials signal
    #[dbus_proxy(signal)]
    fn credentials(
        &self,
        credentials: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// Event signal
    #[dbus_proxy(signal)]
    fn event(
        &self,
        name: &str,
        args: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// PropertiesChanged signal
    #[dbus_proxy(signal)]
    fn properties_changed(
        &self,
        properties: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// ConfigMethods property
    #[dbus_proxy(property)]
    fn config_methods(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_config_methods(&self, value: &str) -> zbus::Result<()>;

    /// DeviceName property
    #[dbus_proxy(property)]
    fn device_name(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_device_name(&self, value: &str) -> zbus::Result<()>;

    /// DeviceType property
    #[dbus_proxy(property)]
    fn device_type(&self) -> zbus::Result<Vec<u8>>;
    #[dbus_proxy(property)]
    fn set_device_type(&self, value: &[u8]) -> zbus::Result<()>;

    /// Manufacturer property
    #[dbus_proxy(property)]
    fn manufacturer(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_manufacturer(&self, value: &str) -> zbus::Result<()>;

    /// ModelName property
    #[dbus_proxy(property)]
    fn model_name(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_model_name(&self, value: &str) -> zbus::Result<()>;

    /// ModelNumber property
    #[dbus_proxy(property)]
    fn model_number(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_model_number(&self, value: &str) -> zbus::Result<()>;

    /// ProcessCredentials property
    #[dbus_proxy(property)]
    fn process_credentials(&self) -> zbus::Result<bool>;
    #[dbus_proxy(property)]
    fn set_process_credentials(&self, value: bool) -> zbus::Result<()>;

    /// SerialNumber property
    #[dbus_proxy(property)]
    fn serial_number(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn set_serial_number(&self, value: &str) -> zbus::Result<()>;
}
*/
