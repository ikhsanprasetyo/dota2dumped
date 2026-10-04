// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-04 12:25:34.881424700 +07:00

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]

pub mod source2_dumper {
    pub mod schemas {
        // Module: soundsystem.dll
        // Class count: 16
        // Enum count: 28
        pub mod soundsystem_dll {
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum SndSeqInstrumentType_t {
                eSndSeqInstNull = 0x0,
                eSndSeqInstSndEvt = 0x1,
                eSndSeqInstMidiSampler = 0x2
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum EMode_t {
                Peak = 0x0,
                RMS = 0x1
            }
            // Alignment: 4
            // Member count: 7
            #[repr(u32)]
            pub enum SndBeatMidiStatusType_t {
                SndSeqMidiStatusNoteOff = 0x8,
                SndSeqMidiStatusNoteOn = 0x9,
                SndSeqMidiStatusKeyPressure = 0xA,
                SndSeqMidiStatusCtrlChange = 0xB,
                SndSeqMidiStatusProgramChange = 0xC,
                SndSeqMidiStatusChannelPressure = 0xD,
                SndSeqMidiStatusPitchBend = 0xE
            }
            // Alignment: 4
            // Member count: 40
            #[repr(u32)]
            pub enum VMixGraphCommandID_t {
                CMD_INVALID = u32::MAX,
                CMD_CONTROL_INPUT_STORE = 0x1,
                CMD_CONTROL_INPUT_STORE_DB = 0x2,
                CMD_CONTROL_TRANSIENT_INPUT_STORE = 0x3,
                CMD_CONTROL_TRANSIENT_INPUT_RESET = 0x4,
                CMD_CONTROL_OUTPUT_STORE = 0x5,
                CMD_CONTROL_EVALUATE_CURVE = 0x6,
                CMD_CONTROL_COPY = 0x7,
                CMD_CONTROL_COND_COPY_IF_NEGATIVE = 0x8,
                CMD_CONTROL_REMAP_LINEAR = 0x9,
                CMD_CONTROL_REMAP_SINE = 0xA,
                CMD_CONTROL_REMAP_LOGLINEAR = 0xB,
                CMD_CONTROL_MAX = 0xC,
                CMD_CONTROL_RESET_TIMER = 0xD,
                CMD_CONTROL_INCREMENT_TIMER = 0xE,
                CMD_CONTROL_EVAL_ENVELOPE = 0xF,
                CMD_CONTROL_SINE_BLEND = 0x10,
                CMD_PROCESSOR_SET_CONTROL_VALUE = 0x11,
                CMD_PROCESSOR_SET_NAME_INPUT = 0x12,
                CMD_PROCESSOR_SET_CONTROL_ARRAYVALUE = 0x13,
                CMD_PROCESSOR_STORE_CONTROL_VALUE = 0x14,
                CMD_PROCESSOR_SET_VSND_VALUE = 0x15,
                CMD_SUBMIX_PROCESS = 0x16,
                CMD_SUBMIX_GENERATE = 0x17,
                CMD_SUBMIX_GENERATE_SIDECHAIN = 0x18,
                CMD_SUBMIX_EXTRACTCONTAINER = 0x19,
                CMD_SUBMIX_DEBUG = 0x1A,
                CMD_SUBMIX_MIX2x1 = 0x1B,
                CMD_SUBMIX_OUTPUT = 0x1C,
                CMD_SUBMIX_OUTPUTx2 = 0x1D,
                CMD_SUBMIX_COPY = 0x1E,
                CMD_SUBMIX_ACCUMULATE = 0x1F,
                CMD_SUBMIX_METER = 0x20,
                CMD_SUBMIX_METER_SPECTRUM = 0x21,
                CMD_IMPULSERESPONSE_INPUT_STORE = 0x22,
                CMD_PROCESSOR_SET_IMPULSERESPONSE_VALUE = 0x23,
                CMD_REMAP_VSND_TO_IMPULSERESPONSE = 0x24,
                CMD_IMPULSERESPONSE_RESET = 0x25,
                CMD_BLEND_VSNDS_TO_IMPULSERESPONSE = 0x26,
                CMD_IMPULSERESPONSE_DELAY = 0x27
            }
            // Alignment: 1
            // Member count: 5
            #[repr(u8)]
            pub enum EWaveform {
                Sine = 0x0,
                Square = 0x1,
                Saw = 0x2,
                Triangle = 0x3,
                Noise = 0x4
            }
            // Alignment: 4
            // Member count: 5
            #[repr(u32)]
            pub enum VMixLFOShape_t {
                LFO_SHAPE_SINE = 0x0,
                LFO_SHAPE_SQUARE = 0x1,
                LFO_SHAPE_TRI = 0x2,
                LFO_SHAPE_SAW = 0x3,
                LFO_SHAPE_NOISE = 0x4
            }
            // Alignment: 2
            // Member count: 10
            #[repr(u16)]
            pub enum VMixFilterType_t {
                FILTER_UNKNOWN = u16::MAX,
                FILTER_LOWPASS = 0x0,
                FILTER_HIGHPASS = 0x1,
                FILTER_BANDPASS = 0x2,
                FILTER_NOTCH = 0x3,
                FILTER_PEAKING_EQ = 0x4,
                FILTER_LOW_SHELF = 0x5,
                FILTER_HIGH_SHELF = 0x6,
                FILTER_ALLPASS = 0x7,
                FILTER_PASSTHROUGH = 0x8
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum SndBeatTrackPlaybackType_t {
                eSndBeatTrackPlaybackTypeStep = 0x0,
                eSndBeatTrackPlaybackTypeFwd = 0x1
            }
            // Alignment: 4
            // Member count: 6
            #[repr(u32)]
            pub enum SndBeatEventType_t {
                eSndBeatEventTypeInvalid = 0x0,
                eSndBeatEventTypeBeat = 0x1,
                eSndBeatEventTypeBar = 0x2,
                eSndBeatEventTypePhrase = 0x3,
                eSndBeatEventTypeLength = 0x4,
                eSndBeatEventTypeKeys = 0x5
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum SosActionStopType_t {
                SOS_STOPTYPE_NONE = 0x0,
                SOS_STOPTYPE_TIME = 0x1,
                SOS_STOPTYPE_OPVAR = 0x2
            }
            // Alignment: 4
            // Member count: 5
            #[repr(u32)]
            pub enum SndBeatKeyType_t {
                eSndBeatPatternTypeNone = 0x0,
                eSndBeatPatternTypeKeys = 0x1,
                eSndBeatPatternTypeKeyedFloats = 0x2,
                eSndBeatPatternTypeKeyedSndEvts = 0x3,
                eSndBeatPatternTypeKeyedMidi = 0x4
            }
            // Alignment: 4
            // Member count: 6
            #[repr(u32)]
            pub enum SosEditItemType_t {
                SOS_EDIT_ITEM_TYPE_SOUNDEVENTS = 0x0,
                SOS_EDIT_ITEM_TYPE_SOUNDEVENT = 0x1,
                SOS_EDIT_ITEM_TYPE_LIBRARYSTACKS = 0x2,
                SOS_EDIT_ITEM_TYPE_STACK = 0x3,
                SOS_EDIT_ITEM_TYPE_OPERATOR = 0x4,
                SOS_EDIT_ITEM_TYPE_FIELD = 0x5
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum SndBeatSyncType_t {
                eSndBeatSyncTypeInvalid = 0x0,
                eSndBeatSyncTypeReset = 0x1,
                eSndBeatSyncTypeSeekImmediate = 0x2
            }
            // Alignment: 4
            // Member count: 5
            #[repr(u32)]
            pub enum PlayBackMode_t {
                Random = 0x0,
                RandomNoRepeats = 0x1,
                RandomAvoidLast = 0x2,
                Sequential = 0x3,
                RandomWeights = 0x4
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum EVsndTriggerMode {
                Trigger = 0x0,
                Gate = 0x1
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum SosGroupFieldBehavior_t {
                kIgnore = 0x0,
                kBranch = 0x1,
                kMatch = 0x2
            }
            // Alignment: 4
            // Member count: 30
            #[repr(u32)]
            pub enum soundlevel_t {
                SNDLVL_NONE = 0x0,
                SNDLVL_20dB = 0x14,
                SNDLVL_25dB = 0x19,
                SNDLVL_30dB = 0x1E,
                SNDLVL_35dB = 0x23,
                SNDLVL_40dB = 0x28,
                SNDLVL_45dB = 0x2D,
                SNDLVL_50dB = 0x32,
                SNDLVL_55dB = 0x37,
                SNDLVL_IDLE = 0x3C,
                SNDLVL_65dB = 0x41,
                SNDLVL_STATIC = 0x42,
                SNDLVL_70dB = 0x46,
                SNDLVL_NORM = 0x4B,
                SNDLVL_80dB = 0x50,
                SNDLVL_85dB = 0x55,
                SNDLVL_90dB = 0x5A,
                SNDLVL_95dB = 0x5F,
                SNDLVL_100dB = 0x64,
                SNDLVL_105dB = 0x69,
                SNDLVL_110dB = 0x6E,
                SNDLVL_120dB = 0x78,
                SNDLVL_130dB = 0x82,
                SNDLVL_GUNFIRE = 0x8C,
                SNDLVL_150dB = 0x96,
                SNDLVL_180dB = 0xB4
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum VMixPannerType_t {
                PANNER_TYPE_LINEAR = 0x0,
                PANNER_TYPE_EQUAL_POWER = 0x1
            }
            // Alignment: 4
            // Member count: 6
            #[repr(u32)]
            pub enum VMixChannelOperation_t {
                VMIX_CHAN_STEREO = 0x0,
                VMIX_CHAN_LEFT = 0x1,
                VMIX_CHAN_RIGHT = 0x2,
                VMIX_CHAN_SWAP = 0x3,
                VMIX_CHAN_MONO = 0x4,
                VMIX_CHAN_MID_SIDE = 0x5
            }
            // Alignment: 1
            // Member count: 13
            #[repr(u8)]
            pub enum EMidiNote {
                C = 0x0,
                C_Sharp = 0x1,
                D = 0x2,
                D_Sharp = 0x3,
                E = 0x4,
                F = 0x5,
                F_Sharp = 0x6,
                G = 0x7,
                G_Sharp = 0x8,
                A = 0x9,
                A_Sharp = 0xA,
                B = 0xB,
                Count = 0xC
            }
            // Alignment: 1
            // Member count: 4
            #[repr(u8)]
            pub enum CVSoundFormat_t {
                PCM16 = 0x0,
                PCM8 = 0x1,
                MP3 = 0x2,
                ADPCM = 0x3
            }
            // Alignment: 1
            // Member count: 9
            #[repr(u8)]
            pub enum VMixFilterSlope_t {
                FILTER_SLOPE_1POLE_6dB = 0x0,
                FILTER_SLOPE_1POLE_12dB = 0x1,
                FILTER_SLOPE_1POLE_18dB = 0x2,
                FILTER_SLOPE_1POLE_24dB = 0x3,
                FILTER_SLOPE_12dB = 0x4,
                FILTER_SLOPE_24dB = 0x5,
                FILTER_SLOPE_36dB = 0x6,
                FILTER_SLOPE_48dB = 0x7
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum SosActionLimitSortType_t {
                SOS_LIMIT_SORTTYPE_HIGHEST = 0x0,
                SOS_LIMIT_SORTTYPE_LOWEST = 0x1
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum VMixSubgraphSwitchInterpolationType_t {
                SUBGRAPH_INTERPOLATION_TEMPORAL_CROSSFADE = 0x0,
                SUBGRAPH_INTERPOLATION_TEMPORAL_FADE_OUT = 0x1,
                SUBGRAPH_INTERPOLATION_KEEP_LAST_SUBGRAPH_RUNNING = 0x2
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum SosGroupType_t {
                SOS_GROUPTYPE_DYNAMIC = 0x0,
                SOS_GROUPTYPE_STATIC = 0x1
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum SndBeatSyncStartType_t {
                eSndBeatSyncStartTypeInvalid = 0x0,
                eSndBeatSyncStartTypeImmediate = 0x1,
                eSndBeatSyncStartTypeQueue = 0x2
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum SosActionSetParamSortType_t {
                SOS_SETPARAM_SORTTYPE_HIGHEST = 0x0,
                SOS_SETPARAM_SORTTYPE_LOWEST = 0x1
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum EVsndPlaybackMode {
                Trigger = 0x0,
                Gate = 0x1
            }
            // Parent: None
            // Field count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            pub mod CVsndTriggerSlot {
                pub const m_bEnableVsnd: usize = 0x0; // bool
                pub const m_vsnd: usize = 0x8; // CSoundContainerReference
                pub const m_bEnableEndcap: usize = 0x28; // bool
                pub const m_endcapVsnd: usize = 0x30; // CSoundContainerReference
                pub const m_bEnableLoopcap: usize = 0x50; // bool
                pub const m_loopcapVsnd: usize = 0x58; // CSoundContainerReference
                pub const m_volume: usize = 0x78; // float32
                pub const m_fadeOut: usize = 0x7C; // float32
                pub const m_mode: usize = 0x80; // EVsndTriggerMode
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MVDataNodeType
            // MPropertyDescription
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            pub mod CDSPPresetMixgroupModifierTable {
                pub const m_table: usize = 0x0; // CUtlVector<CDspPresetModifierList>
            }
            // Parent: None
            // Field count: 1
            pub mod SamplerVoice_t {
                pub const nNoteNum: usize = 0x0; // uint8
            }
            // Parent: None
            // Field count: 0
            pub mod CSndSeqInstruments {
            }
            // Parent: None
            // Field count: 16
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyStartGroup
            // MPropertySuppressExpr
            // MPropertyAttributeEditor
            // MPropertyReadonlyExpr
            // MPropertySuppressExpr
            // MPropertyReadonlyExpr
            pub mod CSosSoundEventGroupSchema {
                pub const m_nGroupType: usize = 0x8; // SosGroupType_t
                pub const m_bBlocksEvents: usize = 0xC; // bool
                pub const m_nBlockMaxCount: usize = 0x10; // int32
                pub const m_flMemberLifespanTime: usize = 0x14; // float32
                pub const m_bInvertMatch: usize = 0x18; // bool
                pub const m_Behavior_EventName: usize = 0x1C; // SosGroupFieldBehavior_t
                pub const m_matchSoundEventName: usize = 0x20; // CUtlString
                pub const m_bMatchEventSubString: usize = 0x28; // bool
                pub const m_matchSoundEventSubString: usize = 0x30; // CUtlString
                pub const m_Behavior_EntIndex: usize = 0x38; // SosGroupFieldBehavior_t
                pub const m_flEntIndex: usize = 0x3C; // float32
                pub const m_Behavior_Opvar: usize = 0x40; // SosGroupFieldBehavior_t
                pub const m_flOpvar: usize = 0x44; // float32
                pub const m_Behavior_String: usize = 0x48; // SosGroupFieldBehavior_t
                pub const m_opvarString: usize = 0x50; // CUtlString
                pub const m_vActions: usize = 0x58; // CUtlVector<CSosGroupActionSchema*>
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyDescription
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPropertyFriendlyName
            // MPropertyDescription
            pub mod CDSPMixgroupModifier {
                pub const m_mixgroup: usize = 0x0; // CUtlString
                pub const m_flModifier: usize = 0x8; // float32
                pub const m_flModifierMin: usize = 0xC; // float32
                pub const m_flSourceModifier: usize = 0x10; // float32
                pub const m_flSourceModifierMin: usize = 0x14; // float32
                pub const m_flListenerReverbModifierWhenSourceReverbIsActive: usize = 0x18; // float32
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MPropertyArrayElementNameKey
            // MVDataOutlinerNameExpr
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            // MPropertyFriendlyName
            pub mod CSndBeatTrack {
                pub const m_name: usize = 0x0; // CUtlString
                pub const m_playbackType: usize = 0x20; // SndBeatTrackPlaybackType_t
                pub const m_nTranspose: usize = 0x24; // int32
                pub const m_bSyncToVoice: usize = 0x28; // bool
                pub const m_flBPM: usize = 0x2C; // float32
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MVDataPromoteField
            // MPropertyFriendlyName
            // MVDataPromoteField
            pub mod CSndBeatPatternManager {
                pub const m_vecPatterns: usize = 0x38; // CUtlVector<CSndBeatPattern>
                pub const m_vecActiveTracks: usize = 0x70; // CUtlVector<CSndBeatTrack>
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CSoundInfoHeader {
            }
            // Parent: None
            // Field count: 6
            pub mod CVMixSubmix {
                pub const m_name: usize = 0x0; // CUtlString
                pub const m_sendOperator: usize = 0x8; // CUtlString
                pub const m_SendNames: usize = 0x10; // CUtlString[4]
                pub const m_nSoloNameHash: usize = 0x30; // uint32
                pub const m_nChannels: usize = 0x34; // int32
                pub const m_nMixDownRule: usize = 0x38; // int32
            }
            // Parent: None
            // Field count: 5
            pub mod KeyGroup_t {
                pub const nCenterNote: usize = 0x0; // uint8
                pub const nMinNote: usize = 0x1; // uint8
                pub const nMaxNote: usize = 0x2; // uint8
                pub const nNumVelocityZones: usize = 0x3; // uint8
                pub const pVelocityZones: usize = 0x8; // VelocityZone_t*
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod SndBeatTimeSignature_t {
                pub const nNumerator: usize = 0x0; // uint8
                pub const nDenominator: usize = 0x1; // uint8
            }
            // Parent: None
            // Field count: 4
            pub mod VelocityZone_t {
                pub const nMaxVel: usize = 0x0; // uint8
                pub const nNextSelection: usize = 0x1; // uint8
                pub const nNumSamples: usize = 0x2; // uint8
                pub const pSamples: usize = 0x4; // uint32[4]
            }
            // Parent: None
            // Field count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod VMixVocoderDesc_t {
                pub const m_nBandCount: usize = 0x0; // int32
                pub const m_flBandwidth: usize = 0x4; // float32
                pub const m_fldBModGain: usize = 0x8; // float32
                pub const m_flFreqRangeStart: usize = 0xC; // float32
                pub const m_flFreqRangeEnd: usize = 0x10; // float32
                pub const m_fldBUnvoicedGain: usize = 0x14; // float32
                pub const m_flAttackTimeMS: usize = 0x18; // float32
                pub const m_flReleaseTimeMS: usize = 0x1C; // float32
                pub const m_nDebugBand: usize = 0x20; // int32
                pub const m_bPeakMode: usize = 0x24; // bool
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MVDataNodeType
            pub mod SndBeatEventKeys_t {
                pub const m_flKey: usize = 0x8; // float32
            }
            // Parent: None
            // Field count: 0
            pub mod ISndSeqInstruments {
            }
        }
    }
}
