#!/bin/sh

zbus-xmlgen fi.w1.wpa_supplicant1.xml > dbus_wpa.rs
zbus-xmlgen fi.w1.wpa_supplicant1.bss.xml > dbus_wpa_bss.rs
zbus-xmlgen fi.w1.wpa_supplicant1.interface.xml > dbus_wpa_interface.rs
zbus-xmlgen fi.w1.wpa_supplicant1.network.xml > dbus_wpa_network.rs
