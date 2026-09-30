// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-09-30 23:53:47.474156300 +07:00

#pragma once

#include <cstddef>
#include <cstdint>

namespace source2_dumper {
    namespace schemas {
        // Module: soundsystem.dll
        // Class count: 16
        // Enum count: 28
        namespace soundsystem_dll {
            // Alignment: 4
            // Member count: 3
            enum class SndSeqInstrumentType_t : uint32_t {
                eSndSeqInstNull = 0x0,
                eSndSeqInstSndEvt = 0x1,
                eSndSeqInstMidiSampler = 0x2
            };
            // Alignment: 4
            // Member count: 2
            enum class EMode_t : uint32_t {
                Peak = 0x0,
                RMS = 0x1
            };
            // Alignment: 4
            // Member count: 7
            enum class SndBeatMidiStatusType_t : uint32_t {
                SndSeqMidiStatusNoteOff = 0x8,
                SndSeqMidiStatusNoteOn = 0x9,
                SndSeqMidiStatusKeyPressure = 0xA,
                SndSeqMidiStatusCtrlChange = 0xB,
                SndSeqMidiStatusProgramChange = 0xC,
                SndSeqMidiStatusChannelPressure = 0xD,
                SndSeqMidiStatusPitchBend = 0xE
            };
            // Alignment: 4
            // Member count: 40
            enum class VMixGraphCommandID_t : uint32_t {
                CMD_INVALID = 0xFFFFFFFF,
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
            };
            // Alignment: 1
            // Member count: 5
            enum class EWaveform : uint8_t {
                Sine = 0x0,
                Square = 0x1,
                Saw = 0x2,
                Triangle = 0x3,
                Noise = 0x4
            };
            // Alignment: 4
            // Member count: 5
            enum class VMixLFOShape_t : uint32_t {
                LFO_SHAPE_SINE = 0x0,
                LFO_SHAPE_SQUARE = 0x1,
                LFO_SHAPE_TRI = 0x2,
                LFO_SHAPE_SAW = 0x3,
                LFO_SHAPE_NOISE = 0x4
            };
            // Alignment: 2
            // Member count: 10
            enum class VMixFilterType_t : uint16_t {
                FILTER_UNKNOWN = 0xFFFF,
                FILTER_LOWPASS = 0x0,
                FILTER_HIGHPASS = 0x1,
                FILTER_BANDPASS = 0x2,
                FILTER_NOTCH = 0x3,
                FILTER_PEAKING_EQ = 0x4,
                FILTER_LOW_SHELF = 0x5,
                FILTER_HIGH_SHELF = 0x6,
                FILTER_ALLPASS = 0x7,
                FILTER_PASSTHROUGH = 0x8
            };
            // Alignment: 4
            // Member count: 2
            enum class SndBeatTrackPlaybackType_t : uint32_t {
                eSndBeatTrackPlaybackTypeStep = 0x0,
                eSndBeatTrackPlaybackTypeFwd = 0x1
            };
            // Alignment: 4
            // Member count: 6
            enum class SndBeatEventType_t : uint32_t {
                eSndBeatEventTypeInvalid = 0x0,
                eSndBeatEventTypeBeat = 0x1,
                eSndBeatEventTypeBar = 0x2,
                eSndBeatEventTypePhrase = 0x3,
                eSndBeatEventTypeLength = 0x4,
                eSndBeatEventTypeKeys = 0x5
            };
            // Alignment: 4
            // Member count: 3
            enum class SosActionStopType_t : uint32_t {
                SOS_STOPTYPE_NONE = 0x0,
                SOS_STOPTYPE_TIME = 0x1,
                SOS_STOPTYPE_OPVAR = 0x2
            };
            // Alignment: 4
            // Member count: 5
            enum class SndBeatKeyType_t : uint32_t {
                eSndBeatPatternTypeNone = 0x0,
                eSndBeatPatternTypeKeys = 0x1,
                eSndBeatPatternTypeKeyedFloats = 0x2,
                eSndBeatPatternTypeKeyedSndEvts = 0x3,
                eSndBeatPatternTypeKeyedMidi = 0x4
            };
            // Alignment: 4
            // Member count: 6
            enum class SosEditItemType_t : uint32_t {
                SOS_EDIT_ITEM_TYPE_SOUNDEVENTS = 0x0,
                SOS_EDIT_ITEM_TYPE_SOUNDEVENT = 0x1,
                SOS_EDIT_ITEM_TYPE_LIBRARYSTACKS = 0x2,
                SOS_EDIT_ITEM_TYPE_STACK = 0x3,
                SOS_EDIT_ITEM_TYPE_OPERATOR = 0x4,
                SOS_EDIT_ITEM_TYPE_FIELD = 0x5
            };
            // Alignment: 4
            // Member count: 3
            enum class SndBeatSyncType_t : uint32_t {
                eSndBeatSyncTypeInvalid = 0x0,
                eSndBeatSyncTypeReset = 0x1,
                eSndBeatSyncTypeSeekImmediate = 0x2
            };
            // Alignment: 4
            // Member count: 5
            enum class PlayBackMode_t : uint32_t {
                Random = 0x0,
                RandomNoRepeats = 0x1,
                RandomAvoidLast = 0x2,
                Sequential = 0x3,
                RandomWeights = 0x4
            };
            // Alignment: 4
            // Member count: 2
            enum class EVsndTriggerMode : uint32_t {
                Trigger = 0x0,
                Gate = 0x1
            };
            // Alignment: 4
            // Member count: 3
            enum class SosGroupFieldBehavior_t : uint32_t {
                kIgnore = 0x0,
                kBranch = 0x1,
                kMatch = 0x2
            };
            // Alignment: 4
            // Member count: 30
            enum class soundlevel_t : uint32_t {
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
            };
            // Alignment: 4
            // Member count: 2
            enum class VMixPannerType_t : uint32_t {
                PANNER_TYPE_LINEAR = 0x0,
                PANNER_TYPE_EQUAL_POWER = 0x1
            };
            // Alignment: 4
            // Member count: 6
            enum class VMixChannelOperation_t : uint32_t {
                VMIX_CHAN_STEREO = 0x0,
                VMIX_CHAN_LEFT = 0x1,
                VMIX_CHAN_RIGHT = 0x2,
                VMIX_CHAN_SWAP = 0x3,
                VMIX_CHAN_MONO = 0x4,
                VMIX_CHAN_MID_SIDE = 0x5
            };
            // Alignment: 1
            // Member count: 13
            enum class EMidiNote : uint8_t {
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
            };
            // Alignment: 1
            // Member count: 4
            enum class CVSoundFormat_t : uint8_t {
                PCM16 = 0x0,
                PCM8 = 0x1,
                MP3 = 0x2,
                ADPCM = 0x3
            };
            // Alignment: 1
            // Member count: 9
            enum class VMixFilterSlope_t : uint8_t {
                FILTER_SLOPE_1POLE_6dB = 0x0,
                FILTER_SLOPE_1POLE_12dB = 0x1,
                FILTER_SLOPE_1POLE_18dB = 0x2,
                FILTER_SLOPE_1POLE_24dB = 0x3,
                FILTER_SLOPE_12dB = 0x4,
                FILTER_SLOPE_24dB = 0x5,
                FILTER_SLOPE_36dB = 0x6,
                FILTER_SLOPE_48dB = 0x7,
                FILTER_SLOPE_MAX = 0x7
            };
            // Alignment: 4
            // Member count: 2
            enum class SosActionLimitSortType_t : uint32_t {
                SOS_LIMIT_SORTTYPE_HIGHEST = 0x0,
                SOS_LIMIT_SORTTYPE_LOWEST = 0x1
            };
            // Alignment: 4
            // Member count: 3
            enum class VMixSubgraphSwitchInterpolationType_t : uint32_t {
                SUBGRAPH_INTERPOLATION_TEMPORAL_CROSSFADE = 0x0,
                SUBGRAPH_INTERPOLATION_TEMPORAL_FADE_OUT = 0x1,
                SUBGRAPH_INTERPOLATION_KEEP_LAST_SUBGRAPH_RUNNING = 0x2
            };
            // Alignment: 4
            // Member count: 2
            enum class SosGroupType_t : uint32_t {
                SOS_GROUPTYPE_DYNAMIC = 0x0,
                SOS_GROUPTYPE_STATIC = 0x1
            };
            // Alignment: 4
            // Member count: 3
            enum class SndBeatSyncStartType_t : uint32_t {
                eSndBeatSyncStartTypeInvalid = 0x0,
                eSndBeatSyncStartTypeImmediate = 0x1,
                eSndBeatSyncStartTypeQueue = 0x2
            };
            // Alignment: 4
            // Member count: 2
            enum class SosActionSetParamSortType_t : uint32_t {
                SOS_SETPARAM_SORTTYPE_HIGHEST = 0x0,
                SOS_SETPARAM_SORTTYPE_LOWEST = 0x1
            };
            // Alignment: 4
            // Member count: 2
            enum class EVsndPlaybackMode : uint32_t {
                Trigger = 0x0,
                Gate = 0x1
            };
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
            namespace CVsndTriggerSlot {
                constexpr std::ptrdiff_t m_bEnableVsnd = 0x0; // bool
                constexpr std::ptrdiff_t m_vsnd = 0x8; // CSoundContainerReference
                constexpr std::ptrdiff_t m_bEnableEndcap = 0x28; // bool
                constexpr std::ptrdiff_t m_endcapVsnd = 0x30; // CSoundContainerReference
                constexpr std::ptrdiff_t m_bEnableLoopcap = 0x50; // bool
                constexpr std::ptrdiff_t m_loopcapVsnd = 0x58; // CSoundContainerReference
                constexpr std::ptrdiff_t m_volume = 0x78; // float32
                constexpr std::ptrdiff_t m_fadeOut = 0x7C; // float32
                constexpr std::ptrdiff_t m_mode = 0x80; // EVsndTriggerMode
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
            namespace CDSPPresetMixgroupModifierTable {
                constexpr std::ptrdiff_t m_table = 0x0; // CUtlVector<CDspPresetModifierList>
            }
            // Parent: None
            // Field count: 1
            namespace SamplerVoice_t {
                constexpr std::ptrdiff_t nNoteNum = 0x0; // uint8
            }
            // Parent: None
            // Field count: 0
            namespace CSndSeqInstruments {
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
            namespace CSosSoundEventGroupSchema {
                constexpr std::ptrdiff_t m_nGroupType = 0x8; // SosGroupType_t
                constexpr std::ptrdiff_t m_bBlocksEvents = 0xC; // bool
                constexpr std::ptrdiff_t m_nBlockMaxCount = 0x10; // int32
                constexpr std::ptrdiff_t m_flMemberLifespanTime = 0x14; // float32
                constexpr std::ptrdiff_t m_bInvertMatch = 0x18; // bool
                constexpr std::ptrdiff_t m_Behavior_EventName = 0x1C; // SosGroupFieldBehavior_t
                constexpr std::ptrdiff_t m_matchSoundEventName = 0x20; // CUtlString
                constexpr std::ptrdiff_t m_bMatchEventSubString = 0x28; // bool
                constexpr std::ptrdiff_t m_matchSoundEventSubString = 0x30; // CUtlString
                constexpr std::ptrdiff_t m_Behavior_EntIndex = 0x38; // SosGroupFieldBehavior_t
                constexpr std::ptrdiff_t m_flEntIndex = 0x3C; // float32
                constexpr std::ptrdiff_t m_Behavior_Opvar = 0x40; // SosGroupFieldBehavior_t
                constexpr std::ptrdiff_t m_flOpvar = 0x44; // float32
                constexpr std::ptrdiff_t m_Behavior_String = 0x48; // SosGroupFieldBehavior_t
                constexpr std::ptrdiff_t m_opvarString = 0x50; // CUtlString
                constexpr std::ptrdiff_t m_vActions = 0x58; // CUtlVector<CSosGroupActionSchema*>
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
            namespace CDSPMixgroupModifier {
                constexpr std::ptrdiff_t m_mixgroup = 0x0; // CUtlString
                constexpr std::ptrdiff_t m_flModifier = 0x8; // float32
                constexpr std::ptrdiff_t m_flModifierMin = 0xC; // float32
                constexpr std::ptrdiff_t m_flSourceModifier = 0x10; // float32
                constexpr std::ptrdiff_t m_flSourceModifierMin = 0x14; // float32
                constexpr std::ptrdiff_t m_flListenerReverbModifierWhenSourceReverbIsActive = 0x18; // float32
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
            namespace CSndBeatTrack {
                constexpr std::ptrdiff_t m_name = 0x0; // CUtlString
                constexpr std::ptrdiff_t m_playbackType = 0x20; // SndBeatTrackPlaybackType_t
                constexpr std::ptrdiff_t m_nTranspose = 0x24; // int32
                constexpr std::ptrdiff_t m_bSyncToVoice = 0x28; // bool
                constexpr std::ptrdiff_t m_flBPM = 0x2C; // float32
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
            namespace CSndBeatPatternManager {
                constexpr std::ptrdiff_t m_vecPatterns = 0x38; // CUtlVector<CSndBeatPattern>
                constexpr std::ptrdiff_t m_vecActiveTracks = 0x70; // CUtlVector<CSndBeatTrack>
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CSoundInfoHeader {
            }
            // Parent: None
            // Field count: 6
            namespace CVMixSubmix {
                constexpr std::ptrdiff_t m_name = 0x0; // CUtlString
                constexpr std::ptrdiff_t m_sendOperator = 0x8; // CUtlString
                constexpr std::ptrdiff_t m_SendNames = 0x10; // CUtlString[4]
                constexpr std::ptrdiff_t m_nSoloNameHash = 0x30; // uint32
                constexpr std::ptrdiff_t m_nChannels = 0x34; // int32
                constexpr std::ptrdiff_t m_nMixDownRule = 0x38; // int32
            }
            // Parent: None
            // Field count: 5
            namespace KeyGroup_t {
                constexpr std::ptrdiff_t nCenterNote = 0x0; // uint8
                constexpr std::ptrdiff_t nMinNote = 0x1; // uint8
                constexpr std::ptrdiff_t nMaxNote = 0x2; // uint8
                constexpr std::ptrdiff_t nNumVelocityZones = 0x3; // uint8
                constexpr std::ptrdiff_t pVelocityZones = 0x8; // VelocityZone_t*
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace SndBeatTimeSignature_t {
                constexpr std::ptrdiff_t nNumerator = 0x0; // uint8
                constexpr std::ptrdiff_t nDenominator = 0x1; // uint8
            }
            // Parent: None
            // Field count: 4
            namespace VelocityZone_t {
                constexpr std::ptrdiff_t nMaxVel = 0x0; // uint8
                constexpr std::ptrdiff_t nNextSelection = 0x1; // uint8
                constexpr std::ptrdiff_t nNumSamples = 0x2; // uint8
                constexpr std::ptrdiff_t pSamples = 0x4; // uint32[4]
            }
            // Parent: None
            // Field count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace VMixVocoderDesc_t {
                constexpr std::ptrdiff_t m_nBandCount = 0x0; // int32
                constexpr std::ptrdiff_t m_flBandwidth = 0x4; // float32
                constexpr std::ptrdiff_t m_fldBModGain = 0x8; // float32
                constexpr std::ptrdiff_t m_flFreqRangeStart = 0xC; // float32
                constexpr std::ptrdiff_t m_flFreqRangeEnd = 0x10; // float32
                constexpr std::ptrdiff_t m_fldBUnvoicedGain = 0x14; // float32
                constexpr std::ptrdiff_t m_flAttackTimeMS = 0x18; // float32
                constexpr std::ptrdiff_t m_flReleaseTimeMS = 0x1C; // float32
                constexpr std::ptrdiff_t m_nDebugBand = 0x20; // int32
                constexpr std::ptrdiff_t m_bPeakMode = 0x24; // bool
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MVDataNodeType
            namespace SndBeatEventKeys_t {
                constexpr std::ptrdiff_t m_flKey = 0x8; // float32
            }
            // Parent: None
            // Field count: 0
            namespace ISndSeqInstruments {
            }
        }
    }
}
