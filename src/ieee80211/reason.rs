#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Reason {
    code: ReasonCode,
    locally_generated: bool,
}

impl From<i32> for Reason {
    fn from(value: i32) -> Self {
        let code = ReasonCode::from(value.unsigned_abs() as u16);
        let locally_generated = value.is_negative();
        Self {
            code,
            locally_generated,
        }
    }
}

impl From<Reason> for i32 {
    fn from(value: Reason) -> Self {
        if value.locally_generated {
            -(u16::from(value.code) as i32)
        } else {
            u16::from(value.code) as i32
        }
    }
}

impl Reason {
    pub fn code(&self) -> ReasonCode {
        self.code
    }

    pub fn locally_generated(&self) -> bool {
        self.locally_generated
    }
}

/* Reason codes (IEEE Std 802.11-2016, 9.4.1.7, Table 9-45) */
const WLAN_REASON_UNSPECIFIED: u16 = 1;
const WLAN_REASON_PREV_AUTH_NOT_VALID: u16 = 2;
const WLAN_REASON_DEAUTH_LEAVING: u16 = 3;
const WLAN_REASON_DISASSOC_DUE_TO_INACTIVITY: u16 = 4;
const WLAN_REASON_DISASSOC_AP_BUSY: u16 = 5;
const WLAN_REASON_CLASS2_FRAME_FROM_NONAUTH_STA: u16 = 6;
const WLAN_REASON_CLASS3_FRAME_FROM_NONASSOC_STA: u16 = 7;
const WLAN_REASON_DISASSOC_STA_HAS_LEFT: u16 = 8;
const WLAN_REASON_STA_REQ_ASSOC_WITHOUT_AUTH: u16 = 9;
const WLAN_REASON_PWR_CAPABILITY_NOT_VALID: u16 = 10;
const WLAN_REASON_SUPPORTED_CHANNEL_NOT_VALID: u16 = 11;
const WLAN_REASON_BSS_TRANSITION_DISASSOC: u16 = 12;
const WLAN_REASON_INVALID_IE: u16 = 13;
const WLAN_REASON_MICHAEL_MIC_FAILURE: u16 = 14;
const WLAN_REASON_4WAY_HANDSHAKE_TIMEOUT: u16 = 15;
const WLAN_REASON_GROUP_KEY_UPDATE_TIMEOUT: u16 = 16;
const WLAN_REASON_IE_IN_4WAY_DIFFERS: u16 = 17;
const WLAN_REASON_GROUP_CIPHER_NOT_VALID: u16 = 18;
const WLAN_REASON_PAIRWISE_CIPHER_NOT_VALID: u16 = 19;
const WLAN_REASON_AKMP_NOT_VALID: u16 = 20;
const WLAN_REASON_UNSUPPORTED_RSN_IE_VERSION: u16 = 21;
const WLAN_REASON_INVALID_RSN_IE_CAPAB: u16 = 22;
const WLAN_REASON_IEEE_802_1X_AUTH_FAILED: u16 = 23;
const WLAN_REASON_CIPHER_SUITE_REJECTED: u16 = 24;
const WLAN_REASON_TDLS_TEARDOWN_UNREACHABLE: u16 = 25;
const WLAN_REASON_TDLS_TEARDOWN_UNSPECIFIED: u16 = 26;
const WLAN_REASON_SSP_REQUESTED_DISASSOC: u16 = 27;
const WLAN_REASON_NO_SSP_ROAMING_AGREEMENT: u16 = 28;
const WLAN_REASON_BAD_CIPHER_OR_AKM: u16 = 29;
const WLAN_REASON_NOT_AUTHORIZED_THIS_LOCATION: u16 = 30;
const WLAN_REASON_SERVICE_CHANGE_PRECLUDES_TS: u16 = 31;
const WLAN_REASON_UNSPECIFIED_QOS_REASON: u16 = 32;
const WLAN_REASON_NOT_ENOUGH_BANDWIDTH: u16 = 33;
const WLAN_REASON_DISASSOC_LOW_ACK: u16 = 34;
const WLAN_REASON_EXCEEDED_TXOP: u16 = 35;
const WLAN_REASON_STA_LEAVING: u16 = 36;
const WLAN_REASON_END_TS_BA_DLS: u16 = 37;
const WLAN_REASON_UNKNOWN_TS_BA: u16 = 38;
const WLAN_REASON_TIMEOUT: u16 = 39;
const WLAN_REASON_PEERKEY_MISMATCH: u16 = 45;
const WLAN_REASON_AUTHORIZED_ACCESS_LIMIT_REACHED: u16 = 46;
const WLAN_REASON_EXTERNAL_SERVICE_REQUIREMENTS: u16 = 47;
const WLAN_REASON_INVALID_FT_ACTION_FRAME_COUNT: u16 = 48;
const WLAN_REASON_INVALID_PMKID: u16 = 49;
const WLAN_REASON_INVALID_MDE: u16 = 50;
const WLAN_REASON_INVALID_FTE: u16 = 51;
const WLAN_REASON_MESH_PEERING_CANCELLED: u16 = 52;
const WLAN_REASON_MESH_MAX_PEERS: u16 = 53;
const WLAN_REASON_MESH_CONFIG_POLICY_VIOLATION: u16 = 54;
const WLAN_REASON_MESH_CLOSE_RCVD: u16 = 55;
const WLAN_REASON_MESH_MAX_RETRIES: u16 = 56;
const WLAN_REASON_MESH_CONFIRM_TIMEOUT: u16 = 57;
const WLAN_REASON_MESH_INVALID_GTK: u16 = 58;
const WLAN_REASON_MESH_INCONSISTENT_PARAMS: u16 = 59;
const WLAN_REASON_MESH_INVALID_SECURITY_CAP: u16 = 60;
const WLAN_REASON_MESH_PATH_ERROR_NO_PROXY_INFO: u16 = 61;
const WLAN_REASON_MESH_PATH_ERROR_NO_FORWARDING_INFO: u16 = 62;
const WLAN_REASON_MESH_PATH_ERROR_DEST_UNREACHABLE: u16 = 63;
const WLAN_REASON_MAC_ADDRESS_ALREADY_EXISTS_IN_MBSS: u16 = 64;
const WLAN_REASON_MESH_CHANNEL_SWITCH_REGULATORY_REQ: u16 = 65;
const WLAN_REASON_MESH_CHANNEL_SWITCH_UNSPECIFIED: u16 = 66;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ReasonCode {
    Unspecified,
    PrevAuthNotValid,
    DeauthLeaving,
    DisassocDueToInactivity,
    DisassocApBusy,
    Class2FrameFromNonauthSta,
    Class3FrameFromNonassocSta,
    DisassocStaHasLeft,
    StaReqAssocWithoutAuth,
    PwrCapabilityNotValid,
    SupportedChannelNotValid,
    BssTransitionDisassoc,
    InvalidIe,
    MichaelMicFailure,
    FourwayHandshakeTimeout,
    GroupKeyUpdateTimeout,
    IeIn4WayDiffers,
    GroupCipherNotValid,
    PairwiseCipherNotValid,
    AkmpNotValid,
    UnsupportedRsnIeVersion,
    InvalidRsnIeCapab,
    Ieee8021XAuthFailed,
    CipherSuiteRejected,
    TdlsTeardownUnreachable,
    TdlsTeardownUnspecified,
    SspRequestedDisassoc,
    NoSspRoamingAgreement,
    BadCipherOrAkm,
    NotAuthorizedThisLocation,
    ServiceChangePrecludesTs,
    UnspecifiedQosReason,
    NotEnoughBandwidth,
    DisassocLowAck,
    ExceededTxop,
    StaLeaving,
    EndTsBaDls,
    UnknownTsBa,
    Timeout,
    PeerkeyMismatch,
    AuthorizedAccessLimitReached,
    ExternalServiceRequirements,
    InvalidFtActionFrameCount,
    InvalidPmkid,
    InvalidMde,
    InvalidFte,
    MeshPeeringCancelled,
    MeshMaxPeers,
    MeshConfigPolicyViolation,
    MeshCloseRcvd,
    MeshMaxRetries,
    MeshConfirmTimeout,
    MeshInvalidGtk,
    MeshInconsistentParams,
    MeshInvalidSecurityCap,
    MeshPathErrorNoProxyInfo,
    MeshPathErrorNoForwardingInfo,
    MeshPathErrorDestUnreachable,
    MacAddressAlreadyExistsInMbss,
    MeshChannelSwitchRegulatoryReq,
    MeshChannelSwitchUnspecified,

    Unknown(u16),
}

impl From<u16> for ReasonCode {
    fn from(value: u16) -> Self {
        use ReasonCode::*;
        match value {
            WLAN_REASON_UNSPECIFIED => Unspecified,
            WLAN_REASON_PREV_AUTH_NOT_VALID => PrevAuthNotValid,
            WLAN_REASON_DEAUTH_LEAVING => DeauthLeaving,
            WLAN_REASON_DISASSOC_DUE_TO_INACTIVITY => DisassocDueToInactivity,
            WLAN_REASON_DISASSOC_AP_BUSY => DisassocApBusy,
            WLAN_REASON_CLASS2_FRAME_FROM_NONAUTH_STA => Class2FrameFromNonauthSta,
            WLAN_REASON_CLASS3_FRAME_FROM_NONASSOC_STA => Class3FrameFromNonassocSta,
            WLAN_REASON_DISASSOC_STA_HAS_LEFT => DisassocStaHasLeft,
            WLAN_REASON_STA_REQ_ASSOC_WITHOUT_AUTH => StaReqAssocWithoutAuth,
            WLAN_REASON_PWR_CAPABILITY_NOT_VALID => PwrCapabilityNotValid,
            WLAN_REASON_SUPPORTED_CHANNEL_NOT_VALID => SupportedChannelNotValid,
            WLAN_REASON_BSS_TRANSITION_DISASSOC => BssTransitionDisassoc,
            WLAN_REASON_INVALID_IE => InvalidIe,
            WLAN_REASON_MICHAEL_MIC_FAILURE => MichaelMicFailure,
            WLAN_REASON_4WAY_HANDSHAKE_TIMEOUT => FourwayHandshakeTimeout,
            WLAN_REASON_GROUP_KEY_UPDATE_TIMEOUT => GroupKeyUpdateTimeout,
            WLAN_REASON_IE_IN_4WAY_DIFFERS => IeIn4WayDiffers,
            WLAN_REASON_GROUP_CIPHER_NOT_VALID => GroupCipherNotValid,
            WLAN_REASON_PAIRWISE_CIPHER_NOT_VALID => PairwiseCipherNotValid,
            WLAN_REASON_AKMP_NOT_VALID => AkmpNotValid,
            WLAN_REASON_UNSUPPORTED_RSN_IE_VERSION => UnsupportedRsnIeVersion,
            WLAN_REASON_INVALID_RSN_IE_CAPAB => InvalidRsnIeCapab,
            WLAN_REASON_IEEE_802_1X_AUTH_FAILED => Ieee8021XAuthFailed,
            WLAN_REASON_CIPHER_SUITE_REJECTED => CipherSuiteRejected,
            WLAN_REASON_TDLS_TEARDOWN_UNREACHABLE => TdlsTeardownUnreachable,
            WLAN_REASON_TDLS_TEARDOWN_UNSPECIFIED => TdlsTeardownUnspecified,
            WLAN_REASON_SSP_REQUESTED_DISASSOC => SspRequestedDisassoc,
            WLAN_REASON_NO_SSP_ROAMING_AGREEMENT => NoSspRoamingAgreement,
            WLAN_REASON_BAD_CIPHER_OR_AKM => BadCipherOrAkm,
            WLAN_REASON_NOT_AUTHORIZED_THIS_LOCATION => NotAuthorizedThisLocation,
            WLAN_REASON_SERVICE_CHANGE_PRECLUDES_TS => ServiceChangePrecludesTs,
            WLAN_REASON_UNSPECIFIED_QOS_REASON => UnspecifiedQosReason,
            WLAN_REASON_NOT_ENOUGH_BANDWIDTH => NotEnoughBandwidth,
            WLAN_REASON_DISASSOC_LOW_ACK => DisassocLowAck,
            WLAN_REASON_EXCEEDED_TXOP => ExceededTxop,
            WLAN_REASON_STA_LEAVING => StaLeaving,
            WLAN_REASON_END_TS_BA_DLS => EndTsBaDls,
            WLAN_REASON_UNKNOWN_TS_BA => UnknownTsBa,
            WLAN_REASON_TIMEOUT => Timeout,
            WLAN_REASON_PEERKEY_MISMATCH => PeerkeyMismatch,
            WLAN_REASON_AUTHORIZED_ACCESS_LIMIT_REACHED => AuthorizedAccessLimitReached,
            WLAN_REASON_EXTERNAL_SERVICE_REQUIREMENTS => ExternalServiceRequirements,
            WLAN_REASON_INVALID_FT_ACTION_FRAME_COUNT => InvalidFtActionFrameCount,
            WLAN_REASON_INVALID_PMKID => InvalidPmkid,
            WLAN_REASON_INVALID_MDE => InvalidMde,
            WLAN_REASON_INVALID_FTE => InvalidFte,
            WLAN_REASON_MESH_PEERING_CANCELLED => MeshPeeringCancelled,
            WLAN_REASON_MESH_MAX_PEERS => MeshMaxPeers,
            WLAN_REASON_MESH_CONFIG_POLICY_VIOLATION => MeshConfigPolicyViolation,
            WLAN_REASON_MESH_CLOSE_RCVD => MeshCloseRcvd,
            WLAN_REASON_MESH_MAX_RETRIES => MeshMaxRetries,
            WLAN_REASON_MESH_CONFIRM_TIMEOUT => MeshConfirmTimeout,
            WLAN_REASON_MESH_INVALID_GTK => MeshInvalidGtk,
            WLAN_REASON_MESH_INCONSISTENT_PARAMS => MeshInconsistentParams,
            WLAN_REASON_MESH_INVALID_SECURITY_CAP => MeshInvalidSecurityCap,
            WLAN_REASON_MESH_PATH_ERROR_NO_PROXY_INFO => MeshPathErrorNoProxyInfo,
            WLAN_REASON_MESH_PATH_ERROR_NO_FORWARDING_INFO => MeshPathErrorNoForwardingInfo,
            WLAN_REASON_MESH_PATH_ERROR_DEST_UNREACHABLE => MeshPathErrorDestUnreachable,
            WLAN_REASON_MAC_ADDRESS_ALREADY_EXISTS_IN_MBSS => MacAddressAlreadyExistsInMbss,
            WLAN_REASON_MESH_CHANNEL_SWITCH_REGULATORY_REQ => MeshChannelSwitchRegulatoryReq,
            WLAN_REASON_MESH_CHANNEL_SWITCH_UNSPECIFIED => MeshChannelSwitchUnspecified,
            val => Unknown(val),
        }
    }
}

impl From<ReasonCode> for u16 {
    fn from(value: ReasonCode) -> Self {
        use ReasonCode::*;
        match value {
            Unspecified => WLAN_REASON_UNSPECIFIED,
            PrevAuthNotValid => WLAN_REASON_PREV_AUTH_NOT_VALID,
            DeauthLeaving => WLAN_REASON_DEAUTH_LEAVING,
            DisassocDueToInactivity => WLAN_REASON_DISASSOC_DUE_TO_INACTIVITY,
            DisassocApBusy => WLAN_REASON_DISASSOC_AP_BUSY,
            Class2FrameFromNonauthSta => WLAN_REASON_CLASS2_FRAME_FROM_NONAUTH_STA,
            Class3FrameFromNonassocSta => WLAN_REASON_CLASS3_FRAME_FROM_NONASSOC_STA,
            DisassocStaHasLeft => WLAN_REASON_DISASSOC_STA_HAS_LEFT,
            StaReqAssocWithoutAuth => WLAN_REASON_STA_REQ_ASSOC_WITHOUT_AUTH,
            PwrCapabilityNotValid => WLAN_REASON_PWR_CAPABILITY_NOT_VALID,
            SupportedChannelNotValid => WLAN_REASON_SUPPORTED_CHANNEL_NOT_VALID,
            BssTransitionDisassoc => WLAN_REASON_BSS_TRANSITION_DISASSOC,
            InvalidIe => WLAN_REASON_INVALID_IE,
            MichaelMicFailure => WLAN_REASON_MICHAEL_MIC_FAILURE,
            FourwayHandshakeTimeout => WLAN_REASON_4WAY_HANDSHAKE_TIMEOUT,
            GroupKeyUpdateTimeout => WLAN_REASON_GROUP_KEY_UPDATE_TIMEOUT,
            IeIn4WayDiffers => WLAN_REASON_IE_IN_4WAY_DIFFERS,
            GroupCipherNotValid => WLAN_REASON_GROUP_CIPHER_NOT_VALID,
            PairwiseCipherNotValid => WLAN_REASON_PAIRWISE_CIPHER_NOT_VALID,
            AkmpNotValid => WLAN_REASON_AKMP_NOT_VALID,
            UnsupportedRsnIeVersion => WLAN_REASON_UNSUPPORTED_RSN_IE_VERSION,
            InvalidRsnIeCapab => WLAN_REASON_INVALID_RSN_IE_CAPAB,
            Ieee8021XAuthFailed => WLAN_REASON_IEEE_802_1X_AUTH_FAILED,
            CipherSuiteRejected => WLAN_REASON_CIPHER_SUITE_REJECTED,
            TdlsTeardownUnreachable => WLAN_REASON_TDLS_TEARDOWN_UNREACHABLE,
            TdlsTeardownUnspecified => WLAN_REASON_TDLS_TEARDOWN_UNSPECIFIED,
            SspRequestedDisassoc => WLAN_REASON_SSP_REQUESTED_DISASSOC,
            NoSspRoamingAgreement => WLAN_REASON_NO_SSP_ROAMING_AGREEMENT,
            BadCipherOrAkm => WLAN_REASON_BAD_CIPHER_OR_AKM,
            NotAuthorizedThisLocation => WLAN_REASON_NOT_AUTHORIZED_THIS_LOCATION,
            ServiceChangePrecludesTs => WLAN_REASON_SERVICE_CHANGE_PRECLUDES_TS,
            UnspecifiedQosReason => WLAN_REASON_UNSPECIFIED_QOS_REASON,
            NotEnoughBandwidth => WLAN_REASON_NOT_ENOUGH_BANDWIDTH,
            DisassocLowAck => WLAN_REASON_DISASSOC_LOW_ACK,
            ExceededTxop => WLAN_REASON_EXCEEDED_TXOP,
            StaLeaving => WLAN_REASON_STA_LEAVING,
            EndTsBaDls => WLAN_REASON_END_TS_BA_DLS,
            UnknownTsBa => WLAN_REASON_UNKNOWN_TS_BA,
            Timeout => WLAN_REASON_TIMEOUT,
            PeerkeyMismatch => WLAN_REASON_PEERKEY_MISMATCH,
            AuthorizedAccessLimitReached => WLAN_REASON_AUTHORIZED_ACCESS_LIMIT_REACHED,
            ExternalServiceRequirements => WLAN_REASON_EXTERNAL_SERVICE_REQUIREMENTS,
            InvalidFtActionFrameCount => WLAN_REASON_INVALID_FT_ACTION_FRAME_COUNT,
            InvalidPmkid => WLAN_REASON_INVALID_PMKID,
            InvalidMde => WLAN_REASON_INVALID_MDE,
            InvalidFte => WLAN_REASON_INVALID_FTE,
            MeshPeeringCancelled => WLAN_REASON_MESH_PEERING_CANCELLED,
            MeshMaxPeers => WLAN_REASON_MESH_MAX_PEERS,
            MeshConfigPolicyViolation => WLAN_REASON_MESH_CONFIG_POLICY_VIOLATION,
            MeshCloseRcvd => WLAN_REASON_MESH_CLOSE_RCVD,
            MeshMaxRetries => WLAN_REASON_MESH_MAX_RETRIES,
            MeshConfirmTimeout => WLAN_REASON_MESH_CONFIRM_TIMEOUT,
            MeshInvalidGtk => WLAN_REASON_MESH_INVALID_GTK,
            MeshInconsistentParams => WLAN_REASON_MESH_INCONSISTENT_PARAMS,
            MeshInvalidSecurityCap => WLAN_REASON_MESH_INVALID_SECURITY_CAP,
            MeshPathErrorNoProxyInfo => WLAN_REASON_MESH_PATH_ERROR_NO_PROXY_INFO,
            MeshPathErrorNoForwardingInfo => WLAN_REASON_MESH_PATH_ERROR_NO_FORWARDING_INFO,
            MeshPathErrorDestUnreachable => WLAN_REASON_MESH_PATH_ERROR_DEST_UNREACHABLE,
            MacAddressAlreadyExistsInMbss => WLAN_REASON_MAC_ADDRESS_ALREADY_EXISTS_IN_MBSS,
            MeshChannelSwitchRegulatoryReq => WLAN_REASON_MESH_CHANNEL_SWITCH_REGULATORY_REQ,
            MeshChannelSwitchUnspecified => WLAN_REASON_MESH_CHANNEL_SWITCH_UNSPECIFIED,
            Unknown(val) => val,
        }
    }
}
