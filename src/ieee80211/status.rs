/* Status codes (IEEE Std 802.11-2016, 9.4.1.9, Table 9-46) */
const WLAN_STATUS_SUCCESS: u16 = 0;
const WLAN_STATUS_UNSPECIFIED_FAILURE: u16 = 1;
const WLAN_STATUS_TDLS_WAKEUP_ALTERNATE: u16 = 2;
const WLAN_STATUS_TDLS_WAKEUP_REJECT: u16 = 3;
const WLAN_STATUS_SECURITY_DISABLED: u16 = 5;
const WLAN_STATUS_UNACCEPTABLE_LIFETIME: u16 = 6;
const WLAN_STATUS_NOT_IN_SAME_BSS: u16 = 7;
const WLAN_STATUS_CAPS_UNSUPPORTED: u16 = 10;
const WLAN_STATUS_REASSOC_NO_ASSOC: u16 = 11;
const WLAN_STATUS_ASSOC_DENIED_UNSPEC: u16 = 12;
const WLAN_STATUS_NOT_SUPPORTED_AUTH_ALG: u16 = 13;
const WLAN_STATUS_UNKNOWN_AUTH_TRANSACTION: u16 = 14;
const WLAN_STATUS_CHALLENGE_FAIL: u16 = 15;
const WLAN_STATUS_AUTH_TIMEOUT: u16 = 16;
const WLAN_STATUS_AP_UNABLE_TO_HANDLE_NEW_STA: u16 = 17;
const WLAN_STATUS_ASSOC_DENIED_RATES: u16 = 18;
const WLAN_STATUS_ASSOC_DENIED_NOSHORT: u16 = 19;
const WLAN_STATUS_SPEC_MGMT_REQUIRED: u16 = 22;
const WLAN_STATUS_PWR_CAPABILITY_NOT_VALID: u16 = 23;
const WLAN_STATUS_SUPPORTED_CHANNEL_NOT_VALID: u16 = 24;
const WLAN_STATUS_ASSOC_DENIED_NO_SHORT_SLOT_TIME: u16 = 25;
const WLAN_STATUS_ASSOC_DENIED_NO_HT: u16 = 27;
const WLAN_STATUS_R0KH_UNREACHABLE: u16 = 28;
const WLAN_STATUS_ASSOC_DENIED_NO_PCO: u16 = 29;
const WLAN_STATUS_ASSOC_REJECTED_TEMPORARILY: u16 = 30;
const WLAN_STATUS_ROBUST_MGMT_FRAME_POLICY_VIOLATION: u16 = 31;
const WLAN_STATUS_UNSPECIFIED_QOS_FAILURE: u16 = 32;
const WLAN_STATUS_DENIED_INSUFFICIENT_BANDWIDTH: u16 = 33;
const WLAN_STATUS_DENIED_POOR_CHANNEL_CONDITIONS: u16 = 34;
const WLAN_STATUS_DENIED_QOS_NOT_SUPPORTED: u16 = 35;
const WLAN_STATUS_REQUEST_DECLINED: u16 = 37;
const WLAN_STATUS_INVALID_PARAMETERS: u16 = 38;
const WLAN_STATUS_REJECTED_WITH_SUGGESTED_CHANGES: u16 = 39;
const WLAN_STATUS_INVALID_IE: u16 = 40;
const WLAN_STATUS_GROUP_CIPHER_NOT_VALID: u16 = 41;
const WLAN_STATUS_PAIRWISE_CIPHER_NOT_VALID: u16 = 42;
const WLAN_STATUS_AKMP_NOT_VALID: u16 = 43;
const WLAN_STATUS_UNSUPPORTED_RSN_IE_VERSION: u16 = 44;
const WLAN_STATUS_INVALID_RSN_IE_CAPAB: u16 = 45;
const WLAN_STATUS_CIPHER_REJECTED_PER_POLICY: u16 = 46;
const WLAN_STATUS_TS_NOT_CREATED: u16 = 47;
const WLAN_STATUS_DIRECT_LINK_NOT_ALLOWED: u16 = 48;
const WLAN_STATUS_DEST_STA_NOT_PRESENT: u16 = 49;
const WLAN_STATUS_DEST_STA_NOT_QOS_STA: u16 = 50;
const WLAN_STATUS_ASSOC_DENIED_LISTEN_INT_TOO_LARGE: u16 = 51;
const WLAN_STATUS_INVALID_FT_ACTION_FRAME_COUNT: u16 = 52;
const WLAN_STATUS_INVALID_PMKID: u16 = 53;
const WLAN_STATUS_INVALID_MDIE: u16 = 54;
const WLAN_STATUS_INVALID_FTIE: u16 = 55;
const WLAN_STATUS_REQUESTED_TCLAS_NOT_SUPPORTED: u16 = 56;
const WLAN_STATUS_INSUFFICIENT_TCLAS_PROCESSING_RESOURCES: u16 = 57;
const WLAN_STATUS_TRY_ANOTHER_BSS: u16 = 58;
const WLAN_STATUS_GAS_ADV_PROTO_NOT_SUPPORTED: u16 = 59;
const WLAN_STATUS_NO_OUTSTANDING_GAS_REQ: u16 = 60;
const WLAN_STATUS_GAS_RESP_NOT_RECEIVED: u16 = 61;
const WLAN_STATUS_STA_TIMED_OUT_WAITING_FOR_GAS_RESP: u16 = 62;
const WLAN_STATUS_GAS_RESP_LARGER_THAN_LIMIT: u16 = 63;
const WLAN_STATUS_REQ_REFUSED_HOME: u16 = 64;
const WLAN_STATUS_ADV_SRV_UNREACHABLE: u16 = 65;
const WLAN_STATUS_REQ_REFUSED_SSPN: u16 = 67;
const WLAN_STATUS_REQ_REFUSED_UNAUTH_ACCESS: u16 = 68;
const WLAN_STATUS_INVALID_RSNIE: u16 = 72;
const WLAN_STATUS_U_APSD_COEX_NOT_SUPPORTED: u16 = 73;
const WLAN_STATUS_U_APSD_COEX_MODE_NOT_SUPPORTED: u16 = 74;
const WLAN_STATUS_BAD_INTERVAL_WITH_U_APSD_COEX: u16 = 75;
const WLAN_STATUS_ANTI_CLOGGING_TOKEN_REQ: u16 = 76;
const WLAN_STATUS_FINITE_CYCLIC_GROUP_NOT_SUPPORTED: u16 = 77;
const WLAN_STATUS_CANNOT_FIND_ALT_TBTT: u16 = 78;
const WLAN_STATUS_TRANSMISSION_FAILURE: u16 = 79;
const WLAN_STATUS_REQ_TCLAS_NOT_SUPPORTED: u16 = 80;
const WLAN_STATUS_TCLAS_RESOURCES_EXCHAUSTED: u16 = 81;
const WLAN_STATUS_REJECTED_WITH_SUGGESTED_BSS_TRANSITION: u16 = 82;
const WLAN_STATUS_REJECT_WITH_SCHEDULE: u16 = 83;
const WLAN_STATUS_REJECT_NO_WAKEUP_SPECIFIED: u16 = 84;
const WLAN_STATUS_SUCCESS_POWER_SAVE_MODE: u16 = 85;
const WLAN_STATUS_PENDING_ADMITTING_FST_SESSION: u16 = 86;
const WLAN_STATUS_PERFORMING_FST_NOW: u16 = 87;
const WLAN_STATUS_PENDING_GAP_IN_BA_WINDOW: u16 = 88;
const WLAN_STATUS_REJECT_U_PID_SETTING: u16 = 89;
const WLAN_STATUS_REFUSED_EXTERNAL_REASON: u16 = 92;
const WLAN_STATUS_REFUSED_AP_OUT_OF_MEMORY: u16 = 93;
const WLAN_STATUS_REJECTED_EMERGENCY_SERVICE_NOT_SUPPORTED: u16 = 94;
const WLAN_STATUS_QUERY_RESP_OUTSTANDING: u16 = 95;
const WLAN_STATUS_REJECT_DSE_BAND: u16 = 96;
const WLAN_STATUS_TCLAS_PROCESSING_TERMINATED: u16 = 97;
const WLAN_STATUS_TS_SCHEDULE_CONFLICT: u16 = 98;
const WLAN_STATUS_DENIED_WITH_SUGGESTED_BAND_AND_CHANNEL: u16 = 99;
const WLAN_STATUS_MCCAOP_RESERVATION_CONFLICT: u16 = 100;
const WLAN_STATUS_MAF_LIMIT_EXCEEDED: u16 = 101;
const WLAN_STATUS_MCCA_TRACK_LIMIT_EXCEEDED: u16 = 102;
const WLAN_STATUS_DENIED_DUE_TO_SPECTRUM_MANAGEMENT: u16 = 103;
const WLAN_STATUS_ASSOC_DENIED_NO_VHT: u16 = 104;
const WLAN_STATUS_ENABLEMENT_DENIED: u16 = 105;
const WLAN_STATUS_RESTRICTION_FROM_AUTHORIZED_GDB: u16 = 106;
const WLAN_STATUS_AUTHORIZATION_DEENABLED: u16 = 107;
const WLAN_STATUS_FILS_AUTHENTICATION_FAILURE: u16 = 112;
const WLAN_STATUS_UNKNOWN_AUTHENTICATION_SERVER: u16 = 113;
const WLAN_STATUS_UNKNOWN_PASSWORD_IDENTIFIER: u16 = 123;
const WLAN_STATUS_DENIED_HE_NOT_SUPPORTED: u16 = 124;
const WLAN_STATUS_SAE_HASH_TO_ELEMENT: u16 = 126;
const WLAN_STATUS_SAE_PK: u16 = 127;

#[derive(Clone, Copy, Debug)]
pub enum StatusCode {
    Success,
    UnspecifiedFailure,
    TdlsWakeupAlternate,
    TdlsWakeupReject,
    SecurityDisabled,
    UnacceptableLifetime,
    NotInSameBss,
    CapsUnsupported,
    ReassocNoAssoc,
    AssocDeniedUnspec,
    NotSupportedAuthAlg,
    UnknownAuthTransaction,
    ChallengeFail,
    AuthTimeout,
    ApUnableToHandleNewSta,
    AssocDeniedRates,
    AssocDeniedNoshort,
    SpecMgmtRequired,
    PwrCapabilityNotValid,
    SupportedChannelNotValid,
    AssocDeniedNoShortSlotTime,
    AssocDeniedNoHt,
    R0KhUnreachable,
    AssocDeniedNoPco,
    AssocRejectedTemporarily,
    RobustMgmtFramePolicyViolation,
    UnspecifiedQosFailure,
    DeniedInsufficientBandwidth,
    DeniedPoorChannelConditions,
    DeniedQosNotSupported,
    RequestDeclined,
    InvalidParameters,
    RejectedWithSuggestedChanges,
    InvalidIe,
    GroupCipherNotValid,
    PairwiseCipherNotValid,
    AkmpNotValid,
    UnsupportedRsnIeVersion,
    InvalidRsnIeCapab,
    CipherRejectedPerPolicy,
    TsNotCreated,
    DirectLinkNotAllowed,
    DestStaNotPresent,
    DestStaNotQosSta,
    AssocDeniedListenIntTooLarge,
    InvalidFtActionFrameCount,
    InvalidPmkid,
    InvalidMdie,
    InvalidFtie,
    RequestedTclasNotSupported,
    InsufficientTclasProcessingResources,
    TryAnotherBss,
    GasAdvProtoNotSupported,
    NoOutstandingGasReq,
    GasRespNotReceived,
    StaTimedOutWaitingForGasResp,
    GasRespLargerThanLimit,
    ReqRefusedHome,
    AdvSrvUnreachable,
    ReqRefusedSspn,
    ReqRefusedUnauthAccess,
    InvalidRsnie,
    UApsdCoexNotSupported,
    UApsdCoexModeNotSupported,
    BadIntervalWithUApsdCoex,
    AntiCloggingTokenReq,
    FiniteCyclicGroupNotSupported,
    CannotFindAltTbtt,
    TransmissionFailure,
    ReqTclasNotSupported,
    TclasResourcesExchausted,
    RejectedWithSuggestedBssTransition,
    RejectWithSchedule,
    RejectNoWakeupSpecified,
    SuccessPowerSaveMode,
    PendingAdmittingFstSession,
    PerformingFstNow,
    PendingGapInBaWindow,
    RejectUPidSetting,
    RefusedExternalReason,
    RefusedApOutOfMemory,
    RejectedEmergencyServiceNotSupported,
    QueryRespOutstanding,
    RejectDseBand,
    TclasProcessingTerminated,
    TsScheduleConflict,
    DeniedWithSuggestedBandAndChannel,
    MccaopReservationConflict,
    MafLimitExceeded,
    MccaTrackLimitExceeded,
    DeniedDueToSpectrumManagement,
    AssocDeniedNoVht,
    EnablementDenied,
    RestrictionFromAuthorizedGdb,
    AuthorizationDeenabled,
    FilsAuthenticationFailure,
    UnknownAuthenticationServer,
    UnknownPasswordIdentifier,
    DeniedHeNotSupported,
    SaeHashToElement,
    SaePk,

    Unknown(u16),
}

impl From<u16> for StatusCode {
    fn from(value: u16) -> Self {
        use StatusCode::*;
        match value {
            WLAN_STATUS_SUCCESS => Success,
            WLAN_STATUS_UNSPECIFIED_FAILURE => UnspecifiedFailure,
            WLAN_STATUS_TDLS_WAKEUP_ALTERNATE => TdlsWakeupAlternate,
            WLAN_STATUS_TDLS_WAKEUP_REJECT => TdlsWakeupReject,
            WLAN_STATUS_SECURITY_DISABLED => SecurityDisabled,
            WLAN_STATUS_UNACCEPTABLE_LIFETIME => UnacceptableLifetime,
            WLAN_STATUS_NOT_IN_SAME_BSS => NotInSameBss,
            WLAN_STATUS_CAPS_UNSUPPORTED => CapsUnsupported,
            WLAN_STATUS_REASSOC_NO_ASSOC => ReassocNoAssoc,
            WLAN_STATUS_ASSOC_DENIED_UNSPEC => AssocDeniedUnspec,
            WLAN_STATUS_NOT_SUPPORTED_AUTH_ALG => NotSupportedAuthAlg,
            WLAN_STATUS_UNKNOWN_AUTH_TRANSACTION => UnknownAuthTransaction,
            WLAN_STATUS_CHALLENGE_FAIL => ChallengeFail,
            WLAN_STATUS_AUTH_TIMEOUT => AuthTimeout,
            WLAN_STATUS_AP_UNABLE_TO_HANDLE_NEW_STA => ApUnableToHandleNewSta,
            WLAN_STATUS_ASSOC_DENIED_RATES => AssocDeniedRates,
            WLAN_STATUS_ASSOC_DENIED_NOSHORT => AssocDeniedNoshort,
            WLAN_STATUS_SPEC_MGMT_REQUIRED => SpecMgmtRequired,
            WLAN_STATUS_PWR_CAPABILITY_NOT_VALID => PwrCapabilityNotValid,
            WLAN_STATUS_SUPPORTED_CHANNEL_NOT_VALID => SupportedChannelNotValid,
            WLAN_STATUS_ASSOC_DENIED_NO_SHORT_SLOT_TIME => AssocDeniedNoShortSlotTime,
            WLAN_STATUS_ASSOC_DENIED_NO_HT => AssocDeniedNoHt,
            WLAN_STATUS_R0KH_UNREACHABLE => R0KhUnreachable,
            WLAN_STATUS_ASSOC_DENIED_NO_PCO => AssocDeniedNoPco,
            WLAN_STATUS_ASSOC_REJECTED_TEMPORARILY => AssocRejectedTemporarily,
            WLAN_STATUS_ROBUST_MGMT_FRAME_POLICY_VIOLATION => RobustMgmtFramePolicyViolation,
            WLAN_STATUS_UNSPECIFIED_QOS_FAILURE => UnspecifiedQosFailure,
            WLAN_STATUS_DENIED_INSUFFICIENT_BANDWIDTH => DeniedInsufficientBandwidth,
            WLAN_STATUS_DENIED_POOR_CHANNEL_CONDITIONS => DeniedPoorChannelConditions,
            WLAN_STATUS_DENIED_QOS_NOT_SUPPORTED => DeniedQosNotSupported,
            WLAN_STATUS_REQUEST_DECLINED => RequestDeclined,
            WLAN_STATUS_INVALID_PARAMETERS => InvalidParameters,
            WLAN_STATUS_REJECTED_WITH_SUGGESTED_CHANGES => RejectedWithSuggestedChanges,
            WLAN_STATUS_INVALID_IE => InvalidIe,
            WLAN_STATUS_GROUP_CIPHER_NOT_VALID => GroupCipherNotValid,
            WLAN_STATUS_PAIRWISE_CIPHER_NOT_VALID => PairwiseCipherNotValid,
            WLAN_STATUS_AKMP_NOT_VALID => AkmpNotValid,
            WLAN_STATUS_UNSUPPORTED_RSN_IE_VERSION => UnsupportedRsnIeVersion,
            WLAN_STATUS_INVALID_RSN_IE_CAPAB => InvalidRsnIeCapab,
            WLAN_STATUS_CIPHER_REJECTED_PER_POLICY => CipherRejectedPerPolicy,
            WLAN_STATUS_TS_NOT_CREATED => TsNotCreated,
            WLAN_STATUS_DIRECT_LINK_NOT_ALLOWED => DirectLinkNotAllowed,
            WLAN_STATUS_DEST_STA_NOT_PRESENT => DestStaNotPresent,
            WLAN_STATUS_DEST_STA_NOT_QOS_STA => DestStaNotQosSta,
            WLAN_STATUS_ASSOC_DENIED_LISTEN_INT_TOO_LARGE => AssocDeniedListenIntTooLarge,
            WLAN_STATUS_INVALID_FT_ACTION_FRAME_COUNT => InvalidFtActionFrameCount,
            WLAN_STATUS_INVALID_PMKID => InvalidPmkid,
            WLAN_STATUS_INVALID_MDIE => InvalidMdie,
            WLAN_STATUS_INVALID_FTIE => InvalidFtie,
            WLAN_STATUS_REQUESTED_TCLAS_NOT_SUPPORTED => RequestedTclasNotSupported,
            WLAN_STATUS_INSUFFICIENT_TCLAS_PROCESSING_RESOURCES => InsufficientTclasProcessingResources,
            WLAN_STATUS_TRY_ANOTHER_BSS => TryAnotherBss,
            WLAN_STATUS_GAS_ADV_PROTO_NOT_SUPPORTED => GasAdvProtoNotSupported,
            WLAN_STATUS_NO_OUTSTANDING_GAS_REQ => NoOutstandingGasReq,
            WLAN_STATUS_GAS_RESP_NOT_RECEIVED => GasRespNotReceived,
            WLAN_STATUS_STA_TIMED_OUT_WAITING_FOR_GAS_RESP => StaTimedOutWaitingForGasResp,
            WLAN_STATUS_GAS_RESP_LARGER_THAN_LIMIT => GasRespLargerThanLimit,
            WLAN_STATUS_REQ_REFUSED_HOME => ReqRefusedHome,
            WLAN_STATUS_ADV_SRV_UNREACHABLE => AdvSrvUnreachable,
            WLAN_STATUS_REQ_REFUSED_SSPN => ReqRefusedSspn,
            WLAN_STATUS_REQ_REFUSED_UNAUTH_ACCESS => ReqRefusedUnauthAccess,
            WLAN_STATUS_INVALID_RSNIE => InvalidRsnie,
            WLAN_STATUS_U_APSD_COEX_NOT_SUPPORTED => UApsdCoexNotSupported,
            WLAN_STATUS_U_APSD_COEX_MODE_NOT_SUPPORTED => UApsdCoexModeNotSupported,
            WLAN_STATUS_BAD_INTERVAL_WITH_U_APSD_COEX => BadIntervalWithUApsdCoex,
            WLAN_STATUS_ANTI_CLOGGING_TOKEN_REQ => AntiCloggingTokenReq,
            WLAN_STATUS_FINITE_CYCLIC_GROUP_NOT_SUPPORTED => FiniteCyclicGroupNotSupported,
            WLAN_STATUS_CANNOT_FIND_ALT_TBTT => CannotFindAltTbtt,
            WLAN_STATUS_TRANSMISSION_FAILURE => TransmissionFailure,
            WLAN_STATUS_REQ_TCLAS_NOT_SUPPORTED => ReqTclasNotSupported,
            WLAN_STATUS_TCLAS_RESOURCES_EXCHAUSTED => TclasResourcesExchausted,
            WLAN_STATUS_REJECTED_WITH_SUGGESTED_BSS_TRANSITION => RejectedWithSuggestedBssTransition,
            WLAN_STATUS_REJECT_WITH_SCHEDULE => RejectWithSchedule,
            WLAN_STATUS_REJECT_NO_WAKEUP_SPECIFIED => RejectNoWakeupSpecified,
            WLAN_STATUS_SUCCESS_POWER_SAVE_MODE => SuccessPowerSaveMode,
            WLAN_STATUS_PENDING_ADMITTING_FST_SESSION => PendingAdmittingFstSession,
            WLAN_STATUS_PERFORMING_FST_NOW => PerformingFstNow,
            WLAN_STATUS_PENDING_GAP_IN_BA_WINDOW => PendingGapInBaWindow,
            WLAN_STATUS_REJECT_U_PID_SETTING => RejectUPidSetting,
            WLAN_STATUS_REFUSED_EXTERNAL_REASON => RefusedExternalReason,
            WLAN_STATUS_REFUSED_AP_OUT_OF_MEMORY => RefusedApOutOfMemory,
            WLAN_STATUS_REJECTED_EMERGENCY_SERVICE_NOT_SUPPORTED => RejectedEmergencyServiceNotSupported,
            WLAN_STATUS_QUERY_RESP_OUTSTANDING => QueryRespOutstanding,
            WLAN_STATUS_REJECT_DSE_BAND => RejectDseBand,
            WLAN_STATUS_TCLAS_PROCESSING_TERMINATED => TclasProcessingTerminated,
            WLAN_STATUS_TS_SCHEDULE_CONFLICT => TsScheduleConflict,
            WLAN_STATUS_DENIED_WITH_SUGGESTED_BAND_AND_CHANNEL => DeniedWithSuggestedBandAndChannel,
            WLAN_STATUS_MCCAOP_RESERVATION_CONFLICT => MccaopReservationConflict,
            WLAN_STATUS_MAF_LIMIT_EXCEEDED => MafLimitExceeded,
            WLAN_STATUS_MCCA_TRACK_LIMIT_EXCEEDED => MccaTrackLimitExceeded,
            WLAN_STATUS_DENIED_DUE_TO_SPECTRUM_MANAGEMENT => DeniedDueToSpectrumManagement,
            WLAN_STATUS_ASSOC_DENIED_NO_VHT => AssocDeniedNoVht,
            WLAN_STATUS_ENABLEMENT_DENIED => EnablementDenied,
            WLAN_STATUS_RESTRICTION_FROM_AUTHORIZED_GDB => RestrictionFromAuthorizedGdb,
            WLAN_STATUS_AUTHORIZATION_DEENABLED => AuthorizationDeenabled,
            WLAN_STATUS_FILS_AUTHENTICATION_FAILURE => FilsAuthenticationFailure,
            WLAN_STATUS_UNKNOWN_AUTHENTICATION_SERVER => UnknownAuthenticationServer,
            WLAN_STATUS_UNKNOWN_PASSWORD_IDENTIFIER => UnknownPasswordIdentifier,
            WLAN_STATUS_DENIED_HE_NOT_SUPPORTED => DeniedHeNotSupported,
            WLAN_STATUS_SAE_HASH_TO_ELEMENT => SaeHashToElement,
            WLAN_STATUS_SAE_PK => SaePk,

            val @ _ => Unknown(val),
        }
    }
}

impl From<StatusCode> for u16 {
    fn from(value: StatusCode) -> Self {
        use StatusCode::*;
        match value {
            Success => WLAN_STATUS_SUCCESS,
            UnspecifiedFailure => WLAN_STATUS_UNSPECIFIED_FAILURE,
            TdlsWakeupAlternate => WLAN_STATUS_TDLS_WAKEUP_ALTERNATE,
            TdlsWakeupReject => WLAN_STATUS_TDLS_WAKEUP_REJECT,
            SecurityDisabled => WLAN_STATUS_SECURITY_DISABLED,
            UnacceptableLifetime => WLAN_STATUS_UNACCEPTABLE_LIFETIME,
            NotInSameBss => WLAN_STATUS_NOT_IN_SAME_BSS,
            CapsUnsupported => WLAN_STATUS_CAPS_UNSUPPORTED,
            ReassocNoAssoc => WLAN_STATUS_REASSOC_NO_ASSOC,
            AssocDeniedUnspec => WLAN_STATUS_ASSOC_DENIED_UNSPEC,
            NotSupportedAuthAlg => WLAN_STATUS_NOT_SUPPORTED_AUTH_ALG,
            UnknownAuthTransaction => WLAN_STATUS_UNKNOWN_AUTH_TRANSACTION,
            ChallengeFail => WLAN_STATUS_CHALLENGE_FAIL,
            AuthTimeout => WLAN_STATUS_AUTH_TIMEOUT,
            ApUnableToHandleNewSta => WLAN_STATUS_AP_UNABLE_TO_HANDLE_NEW_STA,
            AssocDeniedRates => WLAN_STATUS_ASSOC_DENIED_RATES,
            AssocDeniedNoshort => WLAN_STATUS_ASSOC_DENIED_NOSHORT,
            SpecMgmtRequired => WLAN_STATUS_SPEC_MGMT_REQUIRED,
            PwrCapabilityNotValid => WLAN_STATUS_PWR_CAPABILITY_NOT_VALID,
            SupportedChannelNotValid => WLAN_STATUS_SUPPORTED_CHANNEL_NOT_VALID,
            AssocDeniedNoShortSlotTime => WLAN_STATUS_ASSOC_DENIED_NO_SHORT_SLOT_TIME,
            AssocDeniedNoHt => WLAN_STATUS_ASSOC_DENIED_NO_HT,
            R0KhUnreachable => WLAN_STATUS_R0KH_UNREACHABLE,
            AssocDeniedNoPco => WLAN_STATUS_ASSOC_DENIED_NO_PCO,
            AssocRejectedTemporarily => WLAN_STATUS_ASSOC_REJECTED_TEMPORARILY,
            RobustMgmtFramePolicyViolation => WLAN_STATUS_ROBUST_MGMT_FRAME_POLICY_VIOLATION,
            UnspecifiedQosFailure => WLAN_STATUS_UNSPECIFIED_QOS_FAILURE,
            DeniedInsufficientBandwidth => WLAN_STATUS_DENIED_INSUFFICIENT_BANDWIDTH,
            DeniedPoorChannelConditions => WLAN_STATUS_DENIED_POOR_CHANNEL_CONDITIONS,
            DeniedQosNotSupported => WLAN_STATUS_DENIED_QOS_NOT_SUPPORTED,
            RequestDeclined => WLAN_STATUS_REQUEST_DECLINED,
            InvalidParameters => WLAN_STATUS_INVALID_PARAMETERS,
            RejectedWithSuggestedChanges => WLAN_STATUS_REJECTED_WITH_SUGGESTED_CHANGES,
            InvalidIe => WLAN_STATUS_INVALID_IE,
            GroupCipherNotValid => WLAN_STATUS_GROUP_CIPHER_NOT_VALID,
            PairwiseCipherNotValid => WLAN_STATUS_PAIRWISE_CIPHER_NOT_VALID,
            AkmpNotValid => WLAN_STATUS_AKMP_NOT_VALID,
            UnsupportedRsnIeVersion => WLAN_STATUS_UNSUPPORTED_RSN_IE_VERSION,
            InvalidRsnIeCapab => WLAN_STATUS_INVALID_RSN_IE_CAPAB,
            CipherRejectedPerPolicy => WLAN_STATUS_CIPHER_REJECTED_PER_POLICY,
            TsNotCreated => WLAN_STATUS_TS_NOT_CREATED,
            DirectLinkNotAllowed => WLAN_STATUS_DIRECT_LINK_NOT_ALLOWED,
            DestStaNotPresent => WLAN_STATUS_DEST_STA_NOT_PRESENT,
            DestStaNotQosSta => WLAN_STATUS_DEST_STA_NOT_QOS_STA,
            AssocDeniedListenIntTooLarge => WLAN_STATUS_ASSOC_DENIED_LISTEN_INT_TOO_LARGE,
            InvalidFtActionFrameCount => WLAN_STATUS_INVALID_FT_ACTION_FRAME_COUNT,
            InvalidPmkid => WLAN_STATUS_INVALID_PMKID,
            InvalidMdie => WLAN_STATUS_INVALID_MDIE,
            InvalidFtie => WLAN_STATUS_INVALID_FTIE,
            RequestedTclasNotSupported => WLAN_STATUS_REQUESTED_TCLAS_NOT_SUPPORTED,
            InsufficientTclasProcessingResources => WLAN_STATUS_INSUFFICIENT_TCLAS_PROCESSING_RESOURCES,
            TryAnotherBss => WLAN_STATUS_TRY_ANOTHER_BSS,
            GasAdvProtoNotSupported => WLAN_STATUS_GAS_ADV_PROTO_NOT_SUPPORTED,
            NoOutstandingGasReq => WLAN_STATUS_NO_OUTSTANDING_GAS_REQ,
            GasRespNotReceived => WLAN_STATUS_GAS_RESP_NOT_RECEIVED,
            StaTimedOutWaitingForGasResp => WLAN_STATUS_STA_TIMED_OUT_WAITING_FOR_GAS_RESP,
            GasRespLargerThanLimit => WLAN_STATUS_GAS_RESP_LARGER_THAN_LIMIT,
            ReqRefusedHome => WLAN_STATUS_REQ_REFUSED_HOME,
            AdvSrvUnreachable => WLAN_STATUS_ADV_SRV_UNREACHABLE,
            ReqRefusedSspn => WLAN_STATUS_REQ_REFUSED_SSPN,
            ReqRefusedUnauthAccess => WLAN_STATUS_REQ_REFUSED_UNAUTH_ACCESS,
            InvalidRsnie => WLAN_STATUS_INVALID_RSNIE,
            UApsdCoexNotSupported => WLAN_STATUS_U_APSD_COEX_NOT_SUPPORTED,
            UApsdCoexModeNotSupported => WLAN_STATUS_U_APSD_COEX_MODE_NOT_SUPPORTED,
            BadIntervalWithUApsdCoex => WLAN_STATUS_BAD_INTERVAL_WITH_U_APSD_COEX,
            AntiCloggingTokenReq => WLAN_STATUS_ANTI_CLOGGING_TOKEN_REQ,
            FiniteCyclicGroupNotSupported => WLAN_STATUS_FINITE_CYCLIC_GROUP_NOT_SUPPORTED,
            CannotFindAltTbtt => WLAN_STATUS_CANNOT_FIND_ALT_TBTT,
            TransmissionFailure => WLAN_STATUS_TRANSMISSION_FAILURE,
            ReqTclasNotSupported => WLAN_STATUS_REQ_TCLAS_NOT_SUPPORTED,
            TclasResourcesExchausted => WLAN_STATUS_TCLAS_RESOURCES_EXCHAUSTED,
            RejectedWithSuggestedBssTransition => WLAN_STATUS_REJECTED_WITH_SUGGESTED_BSS_TRANSITION,
            RejectWithSchedule => WLAN_STATUS_REJECT_WITH_SCHEDULE,
            RejectNoWakeupSpecified => WLAN_STATUS_REJECT_NO_WAKEUP_SPECIFIED,
            SuccessPowerSaveMode => WLAN_STATUS_SUCCESS_POWER_SAVE_MODE,
            PendingAdmittingFstSession => WLAN_STATUS_PENDING_ADMITTING_FST_SESSION,
            PerformingFstNow => WLAN_STATUS_PERFORMING_FST_NOW,
            PendingGapInBaWindow => WLAN_STATUS_PENDING_GAP_IN_BA_WINDOW,
            RejectUPidSetting => WLAN_STATUS_REJECT_U_PID_SETTING,
            RefusedExternalReason => WLAN_STATUS_REFUSED_EXTERNAL_REASON,
            RefusedApOutOfMemory => WLAN_STATUS_REFUSED_AP_OUT_OF_MEMORY,
            RejectedEmergencyServiceNotSupported => WLAN_STATUS_REJECTED_EMERGENCY_SERVICE_NOT_SUPPORTED,
            QueryRespOutstanding => WLAN_STATUS_QUERY_RESP_OUTSTANDING,
            RejectDseBand => WLAN_STATUS_REJECT_DSE_BAND,
            TclasProcessingTerminated => WLAN_STATUS_TCLAS_PROCESSING_TERMINATED,
            TsScheduleConflict => WLAN_STATUS_TS_SCHEDULE_CONFLICT,
            DeniedWithSuggestedBandAndChannel => WLAN_STATUS_DENIED_WITH_SUGGESTED_BAND_AND_CHANNEL,
            MccaopReservationConflict => WLAN_STATUS_MCCAOP_RESERVATION_CONFLICT,
            MafLimitExceeded => WLAN_STATUS_MAF_LIMIT_EXCEEDED,
            MccaTrackLimitExceeded => WLAN_STATUS_MCCA_TRACK_LIMIT_EXCEEDED,
            DeniedDueToSpectrumManagement => WLAN_STATUS_DENIED_DUE_TO_SPECTRUM_MANAGEMENT,
            AssocDeniedNoVht => WLAN_STATUS_ASSOC_DENIED_NO_VHT,
            EnablementDenied => WLAN_STATUS_ENABLEMENT_DENIED,
            RestrictionFromAuthorizedGdb => WLAN_STATUS_RESTRICTION_FROM_AUTHORIZED_GDB,
            AuthorizationDeenabled => WLAN_STATUS_AUTHORIZATION_DEENABLED,
            FilsAuthenticationFailure => WLAN_STATUS_FILS_AUTHENTICATION_FAILURE,
            UnknownAuthenticationServer => WLAN_STATUS_UNKNOWN_AUTHENTICATION_SERVER,
            UnknownPasswordIdentifier => WLAN_STATUS_UNKNOWN_PASSWORD_IDENTIFIER,
            DeniedHeNotSupported => WLAN_STATUS_DENIED_HE_NOT_SUPPORTED,
            SaeHashToElement => WLAN_STATUS_SAE_HASH_TO_ELEMENT,
            SaePk => WLAN_STATUS_SAE_PK,

            Unknown(val) => val,
        }
    }
}

