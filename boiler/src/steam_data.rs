//! TODO: Auto-generate

use std::io::{Cursor, Read, Write};
use byteorder::{WriteBytesExt, ReadBytesExt, LittleEndian};
use num::FromPrimitive;
use boiler_generated::ProtoMessage;
use boiler_generated::steammessages_base::CMsgProtoBufHeader;

const PROTO_MASK: u32 = 0x80000000;

enum_from_primitive! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum EMsg {
        Invalid = 0,
        Multi = 1,
        GenericReply = 100,
        DestJobFailed = 113,
        Alert = 115,
        ScidRequest = 120,
        ScidResponse = 121,
        JobHeartbeat = 123,
        HubConnect = 124,
        Subscribe = 126,
        RouteMessage = 127,
        RemoteSysID = 128,
        AmCreateAccountResponse = 129,
        WGRequest = 130,
        WGResponse = 131,
        KeepAlive = 132,
        WebApiJobRequest = 133,
        WebApiJobResponse = 134,
        ClientSessionStart = 135,
        ClientSessionEnd = 136,
        ClientSessionUpdateAuthTicket = 137,
        ClientHeartBeat = 703,
        ClientVACResponse = 704,
        ClientLogOff = 706,
        ClientNoUDPConnectivity = 707,
        ClientInformOfCreateAccount = 708,
        ClientAckVACBan = 709,
        ClientConnectionStats = 710,
        ClientInitPurchase = 711,
        ClientPingResponse = 712,
        ClientRemoveFriend = 714,
        ClientGamesPlayedNoDataBlob = 715,
        ClientChangeStatus = 716,
        ClientVacStatusResponse = 717,
        ClientFriendMsg = 718,
        ClientLogOnResponse = 751,
		ClientVACChallenge = 753,
		ClientSetHeartbeatRate = 755,
        ClientLoggedOff = 757,
        GSApprove = 758,
        GSDeny = 759,
        GSKick = 760,
        ClientCreateAcctResponse = 761,
        ClientPurchaseResponse = 763,
        ClientPing = 764,
        ClientNOP = 765,
        ClientPersonaState = 766,
        ClientFriendsList = 767,
        ClientAccountInfo = 768,
        ClientVacStatusQuery = 770,
        ClientNewsUpdate = 771,
        ClientGameConnectDeny = 773,
        GSStatusReply = 774,
        ClientGetFinalPriceResponse = 775,
        ClientGameConnectTokens = 779,
        ClientLicenseList = 780,
        ClientVACBanStatus = 782,
        ClientCMList = 783,
        ClientEncryptPct = 784,
        ClientGetLegacyGameKeyResponse = 785,
        ClientFavoritesList = 786,
        ClientAckGuestPassResponse = 796,
        ClientRedeemGuestPassResponse = 797,
        ClientUpdateGuestPassesList = 798,
        ClientChatMsg = 799,
        ClientChatInvite = 800,
        ClientJoinChat = 801,
        ClientChatMemberInfo = 802,
        ClientPasswordChangeResponse = 805,
        ClientChatEnter = 807,
        ClientFriendRemovedFromSource = 808,
        ClientCreateChat = 809,
        ClientCreateChatResponse = 810,
        ClientUpdateChatMetadata = 811,
        ClientP2PIntroducerMessage = 813,
        ClientChatActionResult = 814,
        ClientRequestFriendData = 815,
        ClientGetUserStats = 818,
        ClientGetUserStatsResponse = 819,
        ClientStoreUserStats = 820,
        ClientStoreUserStatsResponse = 821,
        ClientClanState = 822,
        ClientServiceModule = 830,
        ClientServiceCall = 831,
        ClientServiceCallResponse = 832,
        ClientPackageInfoRequest = 833,
        ClientPackageInfoResponse = 834,
        ClientNatTraversalStatEvent = 839,
        ClientAppInfoRequest = 840,
        ClientAppInfoResponse = 841,
        ClientSteamUsageEvent = 842,
        ClientCheckPassword = 845,
        ClientResetPassword = 846,
        ClientCheckPasswordResponse = 848,
        ClientResetPasswordResponse = 849,
        ClientSessionToken = 850,
        ClientDRMProblemReport = 851,
        ClientSetIgnoreFriend = 855,
        ClientSetIgnoreFriendResponse = 856,
        ClientGetAppOwnershipTicket = 857,
        ClientGetAppOwnershipTicketResponse = 858,
        ClientGetLobbyListResponse = 860,
        ClientGetLobbyMetadata = 861,
        ClientGetLobbyMetadataResponse = 862,
        ClientVTTCert = 863,
        ClientAppInfoUpdate = 866,
        ClientAppInfoChanges = 867,
        ClientServerList = 880,
        ClientEmailChangeResponse = 891,
        ClientSecretQAChangeResponse = 892,
        ClientDRMBlobRequest = 896,
        ClientDRMBlobResponse = 897,
        ClientLookupKey = 898,
        ClientLookupKeyResponse = 899,
        BaseGameServer = 900,
        GSDisconnectNotice = 901,
        GSStatus = 903,
        GSUserPlaying = 905,
        GSStatus2 = 906,
        GSServerType = 908,
        GSPlayerList = 909,
        GSGetUserAchievementStatus = 910,
        GSGetUserAchievementStatusResponse = 911,
        GSGetPlayStats = 918,
        GSGetPlayStatsResponse = 919,
        GSGetUserGroupStatus = 920,
        AMGetUserGroupStatus = 921,
        AMGetUserGroupStatusResponse = 922,
        GSGetUserGroupStatusResponse = 923,
        GSGetReputation = 936,
        GSGetReputationResponse = 937,
        GSAssociateWithClan = 938,
        GSAssociateWithClanResponse = 939,
        GSComputeNewPlayerCompatibility = 940,
        GSComputeNewPlayerCompatibilityResponse = 941,
        AdminCmd = 1000,
        AdminCmdResponse = 1004,
        AdminLogListenRequest = 1005,
        AdminLogEvent = 1006,
        LogSearchRequest = 1007,
        LogSearchResponse = 1008,
        LogSearchCancel = 1009,
        UniverseData = 1010,
        RequestStatHistory = 1014,
        StatHistory = 1015,
        AdminPwLogon = 1017,
        AdminPwLogonResponse = 1018,
        AdminSpew = 1019,
        AdminConsoleTitle = 1020,
        AdminGCSpew = 1023,
        AdminGCCommand = 1024,
        AdminGCGetCommandList = 1025,
        AdminGCGetCommandListResponse = 1026,
        FBSConnectionData = 1027,
        AdminMsgSpew = 1028,
        FBSReqVersion = 1100,
        FBSVersionInfo = 1101,
        FBSForceRefresh = 1102,
        FBSForceBounce = 1103,
        FBSDeployPackage = 1104,
        FBSDeployResponse = 1105,
        FBSUpdateBootstrapper = 1106,
        FBSSetState = 1107,
        FBSApplyOSUpdates = 1108,
        FBSRunCMDScript = 1109,
        FBSRebootBox = 1110,
        FBSSetBigBrotherMode = 1111,
        FBSMinidumpServer = 1112,
        ChannelEncryptRequest = 1303,
    	ChannelEncryptResponse = 1304,
    	ChannelEncryptResult = 1305,
        ClientTicketAuthComplete = 5429,
		ClientIsLimitedAccount = 5430,
		ClientRequestAuthList = 5431,
		ClientAuthList = 5432,
		ClientStat = 5433,
		ClientP2PConnectionInfo = 5434,
		ClientP2PConnectionFailInfo = 5435,
		ClientGetNumberOfCurrentPlayers = 5436,
		ClientGetNumberOfCurrentPlayersResponse = 5437,
		ClientGetDepotDecryptionKey = 5438,
		ClientGetDepotDecryptionKeyResponse = 5439,
		GSPerformHardwareSurvey = 5440,
		ClientGetAppBetaPasswords = 5441,
		ClientGetAppBetaPasswordsResponse = 5442,
		ClientEnableTestLicense = 5443,
		ClientEnableTestLicenseResponse = 5444,
		ClientDisableTestLicense = 5445,
		ClientDisableTestLicenseResponse = 5446,
		ClientRequestValidationMail = 5448,
		ClientRequestValidationMailResponse = 5449,
		ClientCheckAppBetaPassword = 5450,
		ClientCheckAppBetaPasswordResponse = 5451,
		ClientToGC = 5452,
		ClientFromGC = 5453,
		ClientRequestChangeMail = 5454,
		ClientRequestChangeMailResponse = 5455,
		ClientEmailAddrInfo = 5456,
		ClientPasswordChange3 = 5457,
		ClientEmailChange3 = 5458,
		ClientPersonalQAChange3 = 5459,
		ClientResetForgottenPassword3 = 5460,
		ClientRequestForgottenPasswordEmail3 = 5461,
        ClientNewLoginKey = 5463,
        ClientNewLoginKeyAccepted = 5464,
        ClientStoreUserStats2 = 5466,
        ClientStatsUpdated = 5467,
        ClientActivateOEMLicense = 5468,
        ClientRegisterOEMMachine = 5469,
        ClientRegisterOEMMachineResponse = 5470,
        ClientRequestedClientStats = 5480,
        ClientStat2Int32 = 5481,
        ClientStat2 = 5482,
        ClientVerifyPassword = 5483,
        ClientVerifyPasswordResponse = 5484,
        ClientDRMDownloadRequest = 5485,
        ClientDRMDownloadResponse = 5486,
        ClientDRMFinalResult = 5487,
        ClientGetFriendsWhoPlayGame = 5488,
        ClientGetFriendsWhoPlayGameResponse = 5489,
        ClientOGSBeginSession = 5490,
        ClientOGSBeginSessionResponse = 5491,
        ClientOGSEndSession = 5492,
        ClientOGSEndSessionResponse = 5493,
        ClientOGSWriteRow = 5494,
        ClientDRMTest = 5495,
        ClientDRMTestResult = 5496,
        ClientServerUnavailable = 5500,
		ClientServersAvailable = 5501,
		ClientRegisterAuthTicketWithCM = 5502,
		ClientGCMsgFailed = 5503,
		ClientMicroTxnAuthRequest = 5504,
		ClientMicroTxnAuthorize = 5505,
		ClientMicroTxnAuthorizeResponse = 5506,
		ClientAppMinutesPlayedData = 5507,
		ClientGetMicroTxnInfo = 5508,
		ClientGetMicroTxnInfoResponse = 5509,
		ClientMarketingMessageUpdate2 = 5510,
		ClientDeregisterWithServer = 5511,
		ClientSubscribeToPersonaFeed = 5512,
		ClientLogon = 5514,
        ClientGetClientDetails = 5515,
        ClientGetClientDetailsResponse = 5516,
        ClientReportOverlayDetourFailure = 5517,
        ClientGetClientAppList = 5518,
        ClientGetClientAppListResponse = 5519,
        ClientInstallClientApp = 5520,
        ClientInstallClientAppResponse = 5521,
        ClientUninstallClientApp = 5522,
        ClientUninstallClientAppResponse = 5523,
        ClientSetClientAppUpdateState = 5524,
        ClientSetClientAppUpdateStateResponse = 5525,
        ClientRequestEncryptedAppTicket = 5526,
        ClientRequestEncryptedAppTicketResponse = 5527,
        ClientWalletInfoUpdate = 5528,
        ClientLBSSetUGC = 5529,
        ClientLBSSetUGCResponse = 5530,
        ClientAMGetClanOfficers = 5531,
        ClientAMGetClanOfficersResponse = 5532,
        ClientCheckFileSignature = 5533,
        ClientCheckFileSignatureResponse = 5534,
        ClientFriendProfileInfo = 5535,
        ClientFriendProfileInfoResponse = 5536,
        ClientUpdateMachineAuth = 5537,
        ClientUpdateMachineAuthResponse = 5538,
        ClientReadMachineAuth = 5539,
        ClientReadMachineAuthResponse = 5540,
        ClientRequestMachineAuth = 5541,
        ClientRequestMachineAuthResponse = 5542,
        ClientScreenshotsChanged = 5543,
        ClientEmailChange4 = 5544,
        ClientEmailChangeResponse4 = 5545,
        ClientGetCDNAuthToken = 5546,
        ClientGetCDNAuthTokenResponse = 5547,
        ClientDownloadRateStatistics = 5548,
        ClientRequestAccountData = 5549,
        ClientRequestAccountDataResponse = 5550,
        ClientResetForgottenPassword4 = 5551,
        ClientHideFriend = 5552,
        ClientFriendsGroupsList = 5553,
        ClientGetClanActivityCounts = 5554,
        ClientGetClanActivityCountsResponse = 5555,
        ClientOGSReportString = 5556,
        ClientOGSReportBug = 5557,
        ClientSentLogs = 5558,
        ClientLogonGameServer = 5559,
        AMClientCreateFriendsGroup = 5560,
		AMClientCreateFriendsGroupResponse = 5561,
		AMClientDeleteFriendsGroup = 5562,
		AMClientDeleteFriendsGroupResponse = 5563,
		AMClientRenameFriendsGroup = 5564,
		AMClientRenameFriendsGroupResponse = 5565,
		AMClientAddFriendToGroup = 5566,
		AMClientAddFriendToGroupResponse = 5567,
		AMClientRemoveFriendFromGroup = 5568,
		AMClientRemoveFriendFromGroupResponse = 5569,
		ClientAMGetPersonaNameHistory = 5570,
		ClientAMGetPersonaNameHistoryResponse = 5571,
		ClientRequestFreeLicense = 5572,
		ClientRequestFreeLicenseResponse = 5573,
		ClientDRMDownloadRequestWithCrashData = 5574,
		ClientAuthListAck = 5575,
		ClientItemAnnouncements = 5576,
		ClientRequestItemAnnouncements = 5577,
		ClientFriendMsgEchoToSender = 5578,
        ClientOGSGameServerPingSample = 5581,
        ClientCommentNotifications = 5582,
        ClientRequestCommentNotifications = 5583,
        ClientPersonaChangeResponse = 5584,
        ClientRequestWebAPIAuthenticateUserNonce = 5585,
        ClientRequestWebAPIAuthenticateUserNonceResponse = 5586,
        ClientPlayerNicknameList = 5587,
        AMClientSetPlayerNickname = 5588,
        AMClientSetPlayerNicknameResponse = 5589,
    }
}

enum_from_primitive! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum EPersonaState {
		Offline = 0,
		Online = 1,
		Busy = 2,
		Away = 3,
		Snooze = 4,
		LookingToTrade = 5,
		LookingToPlay = 6
    }
}

impl EMsg {
    pub fn parse(data: &mut Cursor<&Vec<u8>>) -> Self {
        let raw = Self::parse_to_raw(data);
        Self::from_raw(raw)
    }

    pub fn parse_to_raw(data: &mut Cursor<&Vec<u8>>) -> u32 {
        data.read_u32::<LittleEndian>().unwrap()
    }

    pub fn from_raw(raw: u32) -> Self {
        let protoless_raw = raw & !PROTO_MASK;
        EMsg::from_u32(protoless_raw)
            .expect(&format!("Can't parse EMsg {}, unknown!", protoless_raw))
    }
}

#[derive(Debug)]
pub struct MsgHdr {
    pub msg: EMsg,
    pub target_job_id: u64,
    pub source_job_id: u64
}

impl MsgHdr {
    pub fn parse(data: &mut Cursor<&Vec<u8>>) -> Self {
        trace!("Decoding MsgHdr type header");

        MsgHdr {
            msg: EMsg::parse(data),
            target_job_id: data.read_u64::<LittleEndian>().unwrap(),
            source_job_id: data.read_u64::<LittleEndian>().unwrap()
        }
    }

    pub fn write_to(&self, data: &mut Cursor<Vec<u8>>) {
        data.write_u32::<LittleEndian>(self.msg as u32).unwrap();
        data.write_u64::<LittleEndian>(self.target_job_id).unwrap();
        data.write_u64::<LittleEndian>(self.source_job_id).unwrap();
    }
}

#[derive(Debug)]
pub struct MsgHdrProtoBuf {
    pub msg: EMsg,
    pub proto: CMsgProtoBufHeader,
}

impl MsgHdrProtoBuf {
    pub fn parse(data: &mut Cursor<&Vec<u8>>) -> Self {
        trace!("Decoding MsgHdrProtoBuf type header");

        // Read in data
        let msg = EMsg::parse(data);
        let len = data.read_u32::<LittleEndian>().unwrap() as usize;
        let mut bytes = vec![0u8; len];
        data.read(&mut bytes).unwrap();

        // Convert protobuf data to the header type
        let mut proto = CMsgProtoBufHeader::new();
        proto.merge_from_bytes(&bytes).unwrap();

        MsgHdrProtoBuf {
            msg: msg,
            proto: proto
        }
    }


    pub fn write_to(&self, data: &mut Cursor<Vec<u8>>) {
        // Turn the protobuf data into bytes
        let bytes = self.proto.write_to_bytes().unwrap();

        // Flag the msg as protobuf
        let msg = self.msg as u32 | PROTO_MASK;

        // Actually write in the header data
        data.write_u32::<LittleEndian>(msg).unwrap();
        data.write_u32::<LittleEndian>(bytes.len() as u32).unwrap();
        data.write(&bytes).unwrap();
    }
}

#[derive(Debug)]
pub struct ExtendedClientMsgHdr {
    pub msg: EMsg,
    pub header_size: u8,
    pub header_version: u16,
    pub target_job_id: u64,
    pub source_job_id: u64,
    pub header_canary: u8,
    pub steam_id: u64,
    pub session_id: i32,
}

impl ExtendedClientMsgHdr {
    pub fn parse(data: &mut Cursor<&Vec<u8>>) -> Self {
        trace!("Decoding ExtendedClientMsgHdr type header");

        ExtendedClientMsgHdr {
            msg: EMsg::parse(data),
            header_size: data.read_u8().unwrap(),
            header_version: data.read_u16::<LittleEndian>().unwrap(),
            target_job_id: data.read_u64::<LittleEndian>().unwrap(),
            source_job_id: data.read_u64::<LittleEndian>().unwrap(),
            header_canary: data.read_u8().unwrap(),
            steam_id: data.read_u64::<LittleEndian>().unwrap(),
            session_id: data.read_i32::<LittleEndian>().unwrap(),
        }
    }

    pub fn write_to(&self, data: &mut Cursor<Vec<u8>>) {
        // TODO: We don't need a cursor for this
        data.write_u32::<LittleEndian>(self.msg as u32).unwrap();
        data.write_u8(self.header_size).unwrap();
        data.write_u16::<LittleEndian>(self.header_version).unwrap();
        data.write_u64::<LittleEndian>(self.target_job_id).unwrap();
        data.write_u64::<LittleEndian>(self.source_job_id).unwrap();
        data.write_u8(self.header_canary).unwrap();
        data.write_u64::<LittleEndian>(self.steam_id).unwrap();
        data.write_i32::<LittleEndian>(self.session_id).unwrap();
    }
}

/// A header of a message to be sent to or received from a server. Can be one of three header types.
#[derive(Debug)]
pub enum MessageHeader {
    MsgHdr(MsgHdr),
    MsgHdrProtoBuf(MsgHdrProtoBuf),
    ExtendedClientMsgHdr(ExtendedClientMsgHdr)
}

impl MessageHeader {
    pub fn parse(data: &mut Cursor<&Vec<u8>>) -> Self {
        // Temporarily store the position so we can peek
        let original_pos = data.position();

        // Peek the event type we received
        let emsg_raw = EMsg::parse_to_raw(data);
        let emsg = EMsg::from_raw(emsg_raw);
        trace!("Parsing message for EMsg: {:?}", emsg);

        // Reset the cursor again, the header needs it to be at the start
        data.set_position(original_pos);

        // Handle the header of the message, which can be different things
        if emsg == EMsg::ChannelEncryptRequest ||
           emsg == EMsg::ChannelEncryptResult {
            MessageHeader::MsgHdr(MsgHdr::parse(data))
        }
        else if (emsg_raw & PROTO_MASK) != 0 {
            MessageHeader::MsgHdrProtoBuf(MsgHdrProtoBuf::parse(data))
        }
        else {
            MessageHeader::ExtendedClientMsgHdr(ExtendedClientMsgHdr::parse(data))
        }
    }

    pub fn write_to(&self, data: &mut Cursor<Vec<u8>>) {
        match *self {
            MessageHeader::MsgHdr(ref h) => h.write_to(data),
            MessageHeader::MsgHdrProtoBuf(ref h) => h.write_to(data),
            MessageHeader::ExtendedClientMsgHdr(ref h) => h.write_to(data),
        }
    }

    /// Gets the EMsg of the inner header type.
    pub fn emsg(&self) -> EMsg {
        match *self {
            MessageHeader::MsgHdr(ref h) => h.msg,
            MessageHeader::MsgHdrProtoBuf(ref h) => h.msg,
            MessageHeader::ExtendedClientMsgHdr(ref h) => h.msg,
        }
    }
}

/// A message to be sent to or received from a steam server.
#[derive(Debug)]
pub struct Message {
    pub header: MessageHeader,
    pub body: Vec<u8>
}

impl Message {
    pub fn parse(data: &mut Cursor<&Vec<u8>>) -> Self {
        // Parse in the header
        let header = MessageHeader::parse(data);

        // Get the remaining data
        let mut body = Vec::new();
        data.read_to_end(&mut body).unwrap();

        // Create the actual message
        Message {
            header: header,
            body: body
        }
    }

    pub fn write_to(&self, data: &mut Cursor<Vec<u8>>) {
        self.header.write_to(data);
        data.write(&self.body).unwrap();
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        let mut data_c = Cursor::new(Vec::new());
        self.write_to(&mut data_c);
        data_c.into_inner()
    }
}
