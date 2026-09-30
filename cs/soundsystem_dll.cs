// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-09-30 23:53:47.474156300 +07:00

namespace Source2Dumper.Schemas {
    // Module: soundsystem.dll
    // Class count: 16
    // Enum count: 28
    public static class SoundsystemDll {
        // Alignment: 4
        // Member count: 3
        public enum SndSeqInstrumentType_t : uint {
            eSndSeqInstNull = 0x0,
            eSndSeqInstSndEvt = 0x1,
            eSndSeqInstMidiSampler = 0x2
        }
        // Alignment: 4
        // Member count: 2
        public enum EMode_t : uint {
            Peak = 0x0,
            RMS = 0x1
        }
        // Alignment: 4
        // Member count: 7
        public enum SndBeatMidiStatusType_t : uint {
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
        public enum VMixGraphCommandID_t : uint {
            CMD_INVALID = unchecked((uint)-1),
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
        public enum EWaveform : byte {
            Sine = 0x0,
            Square = 0x1,
            Saw = 0x2,
            Triangle = 0x3,
            Noise = 0x4
        }
        // Alignment: 4
        // Member count: 5
        public enum VMixLFOShape_t : uint {
            LFO_SHAPE_SINE = 0x0,
            LFO_SHAPE_SQUARE = 0x1,
            LFO_SHAPE_TRI = 0x2,
            LFO_SHAPE_SAW = 0x3,
            LFO_SHAPE_NOISE = 0x4
        }
        // Alignment: 2
        // Member count: 10
        public enum VMixFilterType_t : ushort {
            FILTER_UNKNOWN = unchecked((ushort)-1),
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
        public enum SndBeatTrackPlaybackType_t : uint {
            eSndBeatTrackPlaybackTypeStep = 0x0,
            eSndBeatTrackPlaybackTypeFwd = 0x1
        }
        // Alignment: 4
        // Member count: 6
        public enum SndBeatEventType_t : uint {
            eSndBeatEventTypeInvalid = 0x0,
            eSndBeatEventTypeBeat = 0x1,
            eSndBeatEventTypeBar = 0x2,
            eSndBeatEventTypePhrase = 0x3,
            eSndBeatEventTypeLength = 0x4,
            eSndBeatEventTypeKeys = 0x5
        }
        // Alignment: 4
        // Member count: 3
        public enum SosActionStopType_t : uint {
            SOS_STOPTYPE_NONE = 0x0,
            SOS_STOPTYPE_TIME = 0x1,
            SOS_STOPTYPE_OPVAR = 0x2
        }
        // Alignment: 4
        // Member count: 5
        public enum SndBeatKeyType_t : uint {
            eSndBeatPatternTypeNone = 0x0,
            eSndBeatPatternTypeKeys = 0x1,
            eSndBeatPatternTypeKeyedFloats = 0x2,
            eSndBeatPatternTypeKeyedSndEvts = 0x3,
            eSndBeatPatternTypeKeyedMidi = 0x4
        }
        // Alignment: 4
        // Member count: 6
        public enum SosEditItemType_t : uint {
            SOS_EDIT_ITEM_TYPE_SOUNDEVENTS = 0x0,
            SOS_EDIT_ITEM_TYPE_SOUNDEVENT = 0x1,
            SOS_EDIT_ITEM_TYPE_LIBRARYSTACKS = 0x2,
            SOS_EDIT_ITEM_TYPE_STACK = 0x3,
            SOS_EDIT_ITEM_TYPE_OPERATOR = 0x4,
            SOS_EDIT_ITEM_TYPE_FIELD = 0x5
        }
        // Alignment: 4
        // Member count: 3
        public enum SndBeatSyncType_t : uint {
            eSndBeatSyncTypeInvalid = 0x0,
            eSndBeatSyncTypeReset = 0x1,
            eSndBeatSyncTypeSeekImmediate = 0x2
        }
        // Alignment: 4
        // Member count: 5
        public enum PlayBackMode_t : uint {
            Random = 0x0,
            RandomNoRepeats = 0x1,
            RandomAvoidLast = 0x2,
            Sequential = 0x3,
            RandomWeights = 0x4
        }
        // Alignment: 4
        // Member count: 2
        public enum EVsndTriggerMode : uint {
            Trigger = 0x0,
            Gate = 0x1
        }
        // Alignment: 4
        // Member count: 3
        public enum SosGroupFieldBehavior_t : uint {
            kIgnore = 0x0,
            kBranch = 0x1,
            kMatch = 0x2
        }
        // Alignment: 4
        // Member count: 30
        public enum soundlevel_t : uint {
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
            SNDLVL_60dB = 0x3C,
            SNDLVL_65dB = 0x41,
            SNDLVL_STATIC = 0x42,
            SNDLVL_70dB = 0x46,
            SNDLVL_NORM = 0x4B,
            SNDLVL_75dB = 0x4B,
            SNDLVL_80dB = 0x50,
            SNDLVL_TALKING = 0x50,
            SNDLVL_85dB = 0x55,
            SNDLVL_90dB = 0x5A,
            SNDLVL_95dB = 0x5F,
            SNDLVL_100dB = 0x64,
            SNDLVL_105dB = 0x69,
            SNDLVL_110dB = 0x6E,
            SNDLVL_120dB = 0x78,
            SNDLVL_130dB = 0x82,
            SNDLVL_GUNFIRE = 0x8C,
            SNDLVL_140dB = 0x8C,
            SNDLVL_150dB = 0x96,
            SNDLVL_180dB = 0xB4
        }
        // Alignment: 4
        // Member count: 2
        public enum VMixPannerType_t : uint {
            PANNER_TYPE_LINEAR = 0x0,
            PANNER_TYPE_EQUAL_POWER = 0x1
        }
        // Alignment: 4
        // Member count: 6
        public enum VMixChannelOperation_t : uint {
            VMIX_CHAN_STEREO = 0x0,
            VMIX_CHAN_LEFT = 0x1,
            VMIX_CHAN_RIGHT = 0x2,
            VMIX_CHAN_SWAP = 0x3,
            VMIX_CHAN_MONO = 0x4,
            VMIX_CHAN_MID_SIDE = 0x5
        }
        // Alignment: 1
        // Member count: 13
        public enum EMidiNote : byte {
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
        public enum CVSoundFormat_t : byte {
            PCM16 = 0x0,
            PCM8 = 0x1,
            MP3 = 0x2,
            ADPCM = 0x3
        }
        // Alignment: 1
        // Member count: 9
        public enum VMixFilterSlope_t : byte {
            FILTER_SLOPE_1POLE_6dB = 0x0,
            FILTER_SLOPE_1POLE_12dB = 0x1,
            FILTER_SLOPE_1POLE_18dB = 0x2,
            FILTER_SLOPE_1POLE_24dB = 0x3,
            FILTER_SLOPE_12dB = 0x4,
            FILTER_SLOPE_24dB = 0x5,
            FILTER_SLOPE_36dB = 0x6,
            FILTER_SLOPE_48dB = 0x7,
            FILTER_SLOPE_MAX = 0x7
        }
        // Alignment: 4
        // Member count: 2
        public enum SosActionLimitSortType_t : uint {
            SOS_LIMIT_SORTTYPE_HIGHEST = 0x0,
            SOS_LIMIT_SORTTYPE_LOWEST = 0x1
        }
        // Alignment: 4
        // Member count: 3
        public enum VMixSubgraphSwitchInterpolationType_t : uint {
            SUBGRAPH_INTERPOLATION_TEMPORAL_CROSSFADE = 0x0,
            SUBGRAPH_INTERPOLATION_TEMPORAL_FADE_OUT = 0x1,
            SUBGRAPH_INTERPOLATION_KEEP_LAST_SUBGRAPH_RUNNING = 0x2
        }
        // Alignment: 4
        // Member count: 2
        public enum SosGroupType_t : uint {
            SOS_GROUPTYPE_DYNAMIC = 0x0,
            SOS_GROUPTYPE_STATIC = 0x1
        }
        // Alignment: 4
        // Member count: 3
        public enum SndBeatSyncStartType_t : uint {
            eSndBeatSyncStartTypeInvalid = 0x0,
            eSndBeatSyncStartTypeImmediate = 0x1,
            eSndBeatSyncStartTypeQueue = 0x2
        }
        // Alignment: 4
        // Member count: 2
        public enum SosActionSetParamSortType_t : uint {
            SOS_SETPARAM_SORTTYPE_HIGHEST = 0x0,
            SOS_SETPARAM_SORTTYPE_LOWEST = 0x1
        }
        // Alignment: 4
        // Member count: 2
        public enum EVsndPlaybackMode : uint {
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
        public static class CVsndTriggerSlot {
            public const nint m_bEnableVsnd = 0x0; // bool
            public const nint m_vsnd = 0x8; // CSoundContainerReference
            public const nint m_bEnableEndcap = 0x28; // bool
            public const nint m_endcapVsnd = 0x30; // CSoundContainerReference
            public const nint m_bEnableLoopcap = 0x50; // bool
            public const nint m_loopcapVsnd = 0x58; // CSoundContainerReference
            public const nint m_volume = 0x78; // float32
            public const nint m_fadeOut = 0x7C; // float32
            public const nint m_mode = 0x80; // EVsndTriggerMode
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
        public static class CDSPPresetMixgroupModifierTable {
            public const nint m_table = 0x0; // CUtlVector<CDspPresetModifierList>
        }
        // Parent: None
        // Field count: 1
        public static class SamplerVoice_t {
            public const nint nNoteNum = 0x0; // uint8
        }
        // Parent: None
        // Field count: 0
        public static class CSndSeqInstruments {
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
        public static class CSosSoundEventGroupSchema {
            public const nint m_nGroupType = 0x8; // SosGroupType_t
            public const nint m_bBlocksEvents = 0xC; // bool
            public const nint m_nBlockMaxCount = 0x10; // int32
            public const nint m_flMemberLifespanTime = 0x14; // float32
            public const nint m_bInvertMatch = 0x18; // bool
            public const nint m_Behavior_EventName = 0x1C; // SosGroupFieldBehavior_t
            public const nint m_matchSoundEventName = 0x20; // CUtlString
            public const nint m_bMatchEventSubString = 0x28; // bool
            public const nint m_matchSoundEventSubString = 0x30; // CUtlString
            public const nint m_Behavior_EntIndex = 0x38; // SosGroupFieldBehavior_t
            public const nint m_flEntIndex = 0x3C; // float32
            public const nint m_Behavior_Opvar = 0x40; // SosGroupFieldBehavior_t
            public const nint m_flOpvar = 0x44; // float32
            public const nint m_Behavior_String = 0x48; // SosGroupFieldBehavior_t
            public const nint m_opvarString = 0x50; // CUtlString
            public const nint m_vActions = 0x58; // CUtlVector<CSosGroupActionSchema*>
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
        public static class CDSPMixgroupModifier {
            public const nint m_mixgroup = 0x0; // CUtlString
            public const nint m_flModifier = 0x8; // float32
            public const nint m_flModifierMin = 0xC; // float32
            public const nint m_flSourceModifier = 0x10; // float32
            public const nint m_flSourceModifierMin = 0x14; // float32
            public const nint m_flListenerReverbModifierWhenSourceReverbIsActive = 0x18; // float32
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
        public static class CSndBeatTrack {
            public const nint m_name = 0x0; // CUtlString
            public const nint m_playbackType = 0x20; // SndBeatTrackPlaybackType_t
            public const nint m_nTranspose = 0x24; // int32
            public const nint m_bSyncToVoice = 0x28; // bool
            public const nint m_flBPM = 0x2C; // float32
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
        public static class CSndBeatPatternManager {
            public const nint m_vecPatterns = 0x38; // CUtlVector<CSndBeatPattern>
            public const nint m_vecActiveTracks = 0x70; // CUtlVector<CSndBeatTrack>
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CSoundInfoHeader {
        }
        // Parent: None
        // Field count: 6
        public static class CVMixSubmix {
            public const nint m_name = 0x0; // CUtlString
            public const nint m_sendOperator = 0x8; // CUtlString
            public const nint m_SendNames = 0x10; // CUtlString[4]
            public const nint m_nSoloNameHash = 0x30; // uint32
            public const nint m_nChannels = 0x34; // int32
            public const nint m_nMixDownRule = 0x38; // int32
        }
        // Parent: None
        // Field count: 5
        public static class KeyGroup_t {
            public const nint nCenterNote = 0x0; // uint8
            public const nint nMinNote = 0x1; // uint8
            public const nint nMaxNote = 0x2; // uint8
            public const nint nNumVelocityZones = 0x3; // uint8
            public const nint pVelocityZones = 0x8; // VelocityZone_t*
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class SndBeatTimeSignature_t {
            public const nint nNumerator = 0x0; // uint8
            public const nint nDenominator = 0x1; // uint8
        }
        // Parent: None
        // Field count: 4
        public static class VelocityZone_t {
            public const nint nMaxVel = 0x0; // uint8
            public const nint nNextSelection = 0x1; // uint8
            public const nint nNumSamples = 0x2; // uint8
            public const nint pSamples = 0x4; // uint32[4]
        }
        // Parent: None
        // Field count: 10
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class VMixVocoderDesc_t {
            public const nint m_nBandCount = 0x0; // int32
            public const nint m_flBandwidth = 0x4; // float32
            public const nint m_fldBModGain = 0x8; // float32
            public const nint m_flFreqRangeStart = 0xC; // float32
            public const nint m_flFreqRangeEnd = 0x10; // float32
            public const nint m_fldBUnvoicedGain = 0x14; // float32
            public const nint m_flAttackTimeMS = 0x18; // float32
            public const nint m_flReleaseTimeMS = 0x1C; // float32
            public const nint m_nDebugBand = 0x20; // int32
            public const nint m_bPeakMode = 0x24; // bool
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MVDataNodeType
        public static class SndBeatEventKeys_t {
            public const nint m_flKey = 0x8; // float32
        }
        // Parent: None
        // Field count: 0
        public static class ISndSeqInstruments {
        }
    }
}
