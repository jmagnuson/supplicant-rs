#!/bin/sh

zbus-xmlgen DBusWpa.xml > dbus_wpa.rs
zbus-xmlgen DBusWpaBss.xml > dbus_wpa_bss.rs
zbus-xmlgen DBusWpaInterface.xml > dbus_wpa_interface.rs
zbus-xmlgen DBusWpaNetwork.xml > dbus_wpa_network.rs
