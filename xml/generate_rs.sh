#!/bin/sh

zbus-xmlgen file fi.w1.wpa_supplicant1.xml -o dbus_wpa.rs
zbus-xmlgen file fi.w1.wpa_supplicant1.bss.xml -o dbus_wpa_bss.rs
zbus-xmlgen file fi.w1.wpa_supplicant1.interface.xml -o dbus_wpa_interface.rs
zbus-xmlgen file fi.w1.wpa_supplicant1.network.xml -o dbus_wpa_network.rs
