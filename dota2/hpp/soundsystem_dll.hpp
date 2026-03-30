// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-03-30 13:31:39.632920200 +07:00

#pragma once

#include <cstddef>
#include <cstdint>

namespace source2_dumper {
    namespace schemas {
        // Module: soundsystem.dll
        // Class count: 136
        // Enum count: 26
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
            // Member count: 39
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
                CMD_SUBMIX_DEBUG = 0x19,
                CMD_SUBMIX_MIX2x1 = 0x1A,
                CMD_SUBMIX_OUTPUT = 0x1B,
                CMD_SUBMIX_OUTPUTx2 = 0x1C,
                CMD_SUBMIX_COPY = 0x1D,
                CMD_SUBMIX_ACCUMULATE = 0x1E,
                CMD_SUBMIX_METER = 0x1F,
                CMD_SUBMIX_METER_SPECTRUM = 0x20,
                CMD_IMPULSERESPONSE_INPUT_STORE = 0x21,
                CMD_PROCESSOR_SET_IMPULSERESPONSE_VALUE = 0x22,
                CMD_REMAP_VSND_TO_IMPULSERESPONSE = 0x23,
                CMD_IMPULSERESPONSE_RESET = 0x24,
                CMD_BLEND_VSNDS_TO_IMPULSERESPONSE = 0x25,
                CMD_IMPULSERESPONSE_DELAY = 0x26
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
            // Member count: 3
            enum class SosActionStopType_t : uint32_t {
                SOS_STOPTYPE_NONE = 0x0,
                SOS_STOPTYPE_TIME = 0x1,
                SOS_STOPTYPE_OPVAR = 0x2
            };
            // Alignment: 4
            // Member count: 2
            enum class SndSeqTrackPlaybackType_t : uint32_t {
                eSndSeqTrackPlaybackTypeStep = 0x0,
                eSndSeqTrackPlaybackTypeFwd = 0x1
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
            // Member count: 7
            enum class SndSeqQuantizeType_t : uint32_t {
                eSndSeqQuantizeInvalid = 0xFFFFFFFF,
                eSndSeqQuantizeNone = 0x0,
                eSndSeqQuantizeBeat = 0x1,
                eSndSeqQuantizeBar = 0x2,
                eSndSeqQuantizeSequence = 0x3,
                eSndSeqQuantizeSeek = 0x4,
                eSndSeqQuantizeReset = 0x5
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
            // Member count: 3
            enum class SosGroupFieldBehavior_t : uint32_t {
                kIgnore = 0x0,
                kBranch = 0x1,
                kMatch = 0x2
            };
            // Alignment: 4
            // Member count: 3
            enum class SndSeqSyncType_t : uint32_t {
                eSndSeqSyncTypeNone = 0x0,
                eSndSeqSyncTypeWait = 0x1,
                eSndSeqSyncTypeSeek = 0x2
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
            // Alignment: 4
            // Member count: 3
            enum class SndSeqRegionType_t : uint32_t {
                eSndSeqRegionTypeNull = 0x0,
                eSndSeqRegionTypeSndEvt = 0x1,
                eSndSeqRegionTypeMidiSeq = 0x2
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
            // Member count: 3
            enum class SndSeqPlayerType_t : uint32_t {
                eSndSeqPlayerNull = 0x0,
                eSndSeqPlayerSndEvt = 0x1,
                eSndSeqPlayerMidiSeq = 0x2
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
            // Member count: 2
            enum class SosActionSetParamSortType_t : uint32_t {
                SOS_SETPARAM_SORTTYPE_HIGHEST = 0x0,
                SOS_SETPARAM_SORTTYPE_LOWEST = 0x1
            };
            // Alignment: 4
            // Member count: 7
            enum class SndSeqMidiStatusType_t : uint32_t {
                SndSeqMidiStatusNoteOff = 0x8,
                SndSeqMidiStatusNoteOn = 0x9,
                SndSeqMidiStatusKeyPressure = 0xA,
                SndSeqMidiStatusCtrlChange = 0xB,
                SndSeqMidiStatusProgramChange = 0xC,
                SndSeqMidiStatusChannelPressure = 0xD,
                SndSeqMidiStatusPitchBend = 0xE
            };
            // Parent: None
            // Field count: 48
            namespace CVMixInputBase {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace CVoiceContainerBlender {
            }
            // Parent: soundsystem_lowlevel
            // Field count: 0
            namespace CVMixPitchShiftProcessorDesc {
            }
            // Parent: None
            // Field count: 48
            namespace VMixFreeverbDesc_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            namespace CVoiceContainerStaticAdditiveSynth__CHarmonic {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            namespace CVoiceContainerStaticAdditiveSynth__CTone {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: ___Y__
            // Field count: 0
            namespace CSosGroupActionOcclusionSchema {
            }
            // Parent: None
            // Field count: 0
            namespace CVoiceContainerRandomSampler {
            }
            // Parent: None
            // Field count: 0
            namespace CVMixSteamAudioDirectProcessorDesc {
            }
            // Parent: None
            // Field count: 0
            namespace CVMixSteamAudioHRTFProcessorDesc {
            }
            // Parent: soundsystem_voicecontainers
            // Field count: 0
            namespace CVoiceContainerDefault {
            }
            // Parent: None
            // Field count: 48
            namespace CVSound {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            namespace CDSPPresetMixgroupModifierTable {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace CSosGroupActionSoundeventClusterSchema {
            }
            // Parent: None
            // Field count: 0
            namespace CSosGroupActionSetSoundeventParameterSchema {
            }
            // Parent: None
            // Field count: 48
            namespace CSoundContainerReference {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: soundsystem_voicecontainers
            // Field count: 0
            namespace CVoiceContainerNull {
            }
            // Parent: None
            // Field count: 48
            namespace VMixSubgraphSwitchDesc_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            namespace CVoiceContainerAnalysisBase {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace CSosGroupActionSoundeventMinMaxValuesSchema {
            }
            // Parent: soundsystem_lowlevel
            // Field count: 0
            namespace CVMixEnvelopeProcessorDesc {
            }
            // Parent: None
            // Field count: 48
            namespace CVMixCommand {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 45
            namespace SamplerVoice_t {
                constexpr std::ptrdiff_t  = 0x10110FF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x45; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1032000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x61007400; // 
                constexpr std::ptrdiff_t  = 0x6F006400; // 
                constexpr std::ptrdiff_t  = 0x74006100; // 
                constexpr std::ptrdiff_t  = 0x2E003000; // 
                constexpr std::ptrdiff_t  = 0x3A004300; // 
                constexpr std::ptrdiff_t  = 0x63006900; // 
                constexpr std::ptrdiff_t  = 0x44002000; // 
                constexpr std::ptrdiff_t  = 0x6D007200; // 
                constexpr std::ptrdiff_t  = 0x5C003000; // 
                constexpr std::ptrdiff_t  = 0x66006E00; // 
                constexpr std::ptrdiff_t  = 0x61004400; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x6C005000; // 
                constexpr std::ptrdiff_t  = 0x32003000; // 
                constexpr std::ptrdiff_t  = 0x6E006500; // 
                constexpr std::ptrdiff_t  = 0x7FF98B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x69006400; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x44005C00; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7700; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D004D00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100F003; // 
                constexpr std::ptrdiff_t  = 0x1100000; // 
            }
            // Parent: None
            // Field count: 0
            namespace CVMixControlInput {
            }
            // Parent: ______
            // Field count: 0
            namespace CVMixBoxverbProcessorDesc {
            }
            // Parent: soundsystem_lowlevel
            // Field count: 0
            namespace CVMixDynamicsCompressorProcessorDesc {
            }
            // Parent: None
            // Field count: 48
            namespace VMixPannerDesc_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            namespace CSosGroupActionSoundeventPrioritySchema {
            }
            // Parent: None
            // Field count: 0
            namespace CVoiceContainerRealtimeFMSineWave {
            }
            // Parent: None
            // Field count: 48
            namespace SelectedEditItemInfo_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            namespace VMixModDelayDesc_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: soundsystem
            // Field count: 0
            namespace CSndSeqInstSndEvtSchema {
            }
            // Parent: soundsystem_lowlevel
            // Field count: 0
            namespace CVMixNameInputMeter {
            }
            // Parent: soundsystem
            // Field count: 0
            namespace CSndSeqInstruments {
            }
            // Parent: None
            // Field count: 0
            namespace CVMixDynamics3BandProcessorDesc {
            }
            // Parent: None
            // Field count: 48
            namespace VMixBoxverbDesc_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 45
            namespace CSosGroupActionSchema {
                constexpr std::ptrdiff_t  = 0x10110FF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x45; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1032000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x61007400; // 
                constexpr std::ptrdiff_t  = 0x6F006400; // 
                constexpr std::ptrdiff_t  = 0x74006100; // 
                constexpr std::ptrdiff_t  = 0x2E003000; // 
                constexpr std::ptrdiff_t  = 0x3A004300; // 
                constexpr std::ptrdiff_t  = 0x63006900; // 
                constexpr std::ptrdiff_t  = 0x44002000; // 
                constexpr std::ptrdiff_t  = 0x6D007200; // 
                constexpr std::ptrdiff_t  = 0x5C003000; // 
                constexpr std::ptrdiff_t  = 0x66006E00; // 
                constexpr std::ptrdiff_t  = 0x61004400; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x6C005000; // 
                constexpr std::ptrdiff_t  = 0x32003000; // 
                constexpr std::ptrdiff_t  = 0x6E006500; // 
                constexpr std::ptrdiff_t  = 0x7FF98B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x69006400; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x44005C00; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7700; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D004D00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100F003; // 
                constexpr std::ptrdiff_t  = 0x1100000; // 
            }
            // Parent: None
            // Field count: 48
            namespace CSosSoundEventGroupSchema {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: ___Y__
            // Field count: 0
            namespace CSndSeqInstMidiSampler {
            }
            // Parent: None
            // Field count: 45
            namespace CSndSeqInstBaseSchema {
                constexpr std::ptrdiff_t  = 0x10110FF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x45; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1032000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x61007400; // 
                constexpr std::ptrdiff_t  = 0x6F006400; // 
                constexpr std::ptrdiff_t  = 0x74006100; // 
                constexpr std::ptrdiff_t  = 0x2E003000; // 
                constexpr std::ptrdiff_t  = 0x3A004300; // 
                constexpr std::ptrdiff_t  = 0x63006900; // 
                constexpr std::ptrdiff_t  = 0x44002000; // 
                constexpr std::ptrdiff_t  = 0x6D007200; // 
                constexpr std::ptrdiff_t  = 0x5C003000; // 
                constexpr std::ptrdiff_t  = 0x66006E00; // 
                constexpr std::ptrdiff_t  = 0x61004400; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x6C005000; // 
                constexpr std::ptrdiff_t  = 0x32003000; // 
                constexpr std::ptrdiff_t  = 0x6E006500; // 
                constexpr std::ptrdiff_t  = 0x7FF98B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x69006400; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x44005C00; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7700; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D004D00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100F003; // 
                constexpr std::ptrdiff_t  = 0x1100000; // 
            }
            // Parent: None
            // Field count: 48
            namespace VMixDynamics3BandDesc_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: soundsystem_lowlevel
            // Field count: 0
            namespace CVMixEQ8ProcessorDesc {
            }
            // Parent: None
            // Field count: 48
            namespace CDSPMixgroupModifier {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            namespace CAudioMorphData {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            namespace VMixDualCompressorDesc_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            namespace CVoiceContainerStaticAdditiveSynth {
            }
            // Parent: None
            // Field count: 0
            namespace CVoiceContainerShapedNoise {
            }
            // Parent: None
            // Field count: 48
            namespace CDspPresetModifierList {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 45
            namespace CVoiceContainerBase {
                constexpr std::ptrdiff_t  = 0x10110FF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x45; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1032000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x61007400; // 
                constexpr std::ptrdiff_t  = 0x6F006400; // 
                constexpr std::ptrdiff_t  = 0x74006100; // 
                constexpr std::ptrdiff_t  = 0x2E003000; // 
                constexpr std::ptrdiff_t  = 0x3A004300; // 
                constexpr std::ptrdiff_t  = 0x63006900; // 
                constexpr std::ptrdiff_t  = 0x44002000; // 
                constexpr std::ptrdiff_t  = 0x6D007200; // 
                constexpr std::ptrdiff_t  = 0x5C003000; // 
                constexpr std::ptrdiff_t  = 0x66006E00; // 
                constexpr std::ptrdiff_t  = 0x61004400; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x6C005000; // 
                constexpr std::ptrdiff_t  = 0x32003000; // 
                constexpr std::ptrdiff_t  = 0x6E006500; // 
                constexpr std::ptrdiff_t  = 0x7FF98B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x69006400; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x44005C00; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7700; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D004D00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100F003; // 
                constexpr std::ptrdiff_t  = 0x1100000; // 
            }
            // Parent: soundsystem_lowlevel
            // Field count: 0
            namespace CVMixDiffusorProcessorDesc {
            }
            // Parent: None
            // Field count: 0
            namespace CVMixUtilityProcessorDesc {
            }
            // Parent: None
            // Field count: 0
            namespace CVoiceContainerGranulator {
            }
            // Parent: soundsystem_lowlevel
            // Field count: 0
            namespace CVMixPresetDSPProcessorDesc {
            }
            // Parent: None
            // Field count: 48
            namespace VMixDelayDesc_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            namespace VMixEQ8Desc_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            namespace CVMixDynamicsProcessorDesc {
            }
            // Parent: None
            // Field count: 48
            namespace CVMixGraphDescData {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace CVoiceContainerLoopXFade {
            }
            // Parent: None
            // Field count: 48
            namespace VMixPresetDSPDesc_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            namespace CAudioPhonemeTag {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: soundsystem_lowlevel
            // Field count: 0
            namespace CVMixControlInputArray {
            }
            // Parent: soundsystem_lowlevel
            // Field count: 0
            namespace CVMixNameInput {
            }
            // Parent: ___Y__
            // Field count: 0
            namespace CSosGroupActionSoundeventCountSchema {
            }
            // Parent: None
            // Field count: 0
            namespace CVoiceContainerEnvelopeAnalyzer {
            }
            // Parent: None
            // Field count: 45
            namespace CVMixBaseProcessorDesc {
                constexpr std::ptrdiff_t  = 0x10110FF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x45; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1032000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x61007400; // 
                constexpr std::ptrdiff_t  = 0x6F006400; // 
                constexpr std::ptrdiff_t  = 0x74006100; // 
                constexpr std::ptrdiff_t  = 0x2E003000; // 
                constexpr std::ptrdiff_t  = 0x3A004300; // 
                constexpr std::ptrdiff_t  = 0x63006900; // 
                constexpr std::ptrdiff_t  = 0x44002000; // 
                constexpr std::ptrdiff_t  = 0x6D007200; // 
                constexpr std::ptrdiff_t  = 0x5C003000; // 
                constexpr std::ptrdiff_t  = 0x66006E00; // 
                constexpr std::ptrdiff_t  = 0x61004400; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x6C005000; // 
                constexpr std::ptrdiff_t  = 0x32003000; // 
                constexpr std::ptrdiff_t  = 0x6E006500; // 
                constexpr std::ptrdiff_t  = 0x7FF98B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x69006400; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x44005C00; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7700; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D004D00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100F003; // 
                constexpr std::ptrdiff_t  = 0x1100000; // 
            }
            // Parent: soundsystem_lowlevel
            // Field count: 0
            namespace CVMixImpulseResponseInput {
            }
            // Parent: None
            // Field count: 0
            namespace CVMixSteamAudioHybridReverbProcessorDesc {
            }
            // Parent: None
            // Field count: 48
            namespace CSoundEventMetaData {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            namespace VMixPitchShiftDesc_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            namespace CVMixControlOutput {
            }
            // Parent: None
            // Field count: 0
            namespace CVMixModDelayProcessorDesc {
            }
            // Parent: ______
            // Field count: 0
            namespace CVMixShaperProcessorDesc {
            }
            // Parent: None
            // Field count: 48
            namespace CVMixAutomaticControlInput {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            namespace CAudioEmphasisSample {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: ______
            // Field count: 0
            namespace CVMixOscProcessorDesc {
            }
            // Parent: None
            // Field count: 48
            namespace CVMixCurveHeader {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: soundsystem_voicecontainers
            // Field count: 0
            namespace CVoiceContainerGenerator {
            }
            // Parent: None
            // Field count: 0
            namespace CVoiceContainerSet {
            }
            // Parent: None
            // Field count: 48
            namespace VMixConvolutionDesc_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            namespace CVoiceContainerSetElement {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: soundsystem_voicecontainers
            // Field count: 0
            namespace CVoiceContainerAsyncGenerator {
            }
            // Parent: None
            // Field count: 57
            namespace CSoundInfoHeader {
                constexpr std::ptrdiff_t  = 0x1FFEEFF; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x1000000; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x2C000; // 
                constexpr std::ptrdiff_t  = 0x10F; // 
                constexpr std::ptrdiff_t  = 0x101; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x107; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x103; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x101; // 
                constexpr std::ptrdiff_t  = 0xE02FA7; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C006100; // 
                constexpr std::ptrdiff_t  = 0x77006F00; // 
                constexpr std::ptrdiff_t  = 0x66007400; // 
                constexpr std::ptrdiff_t  = 0x36002E00; // 
                constexpr std::ptrdiff_t  = 0x5C003A00; // 
                constexpr std::ptrdiff_t  = 0x72006300; // 
                constexpr std::ptrdiff_t  = 0x65004400; // 
                constexpr std::ptrdiff_t  = 0x5C006D00; // 
                constexpr std::ptrdiff_t  = 0x4D005C00; // 
                constexpr std::ptrdiff_t  = 0x69006600; // 
                constexpr std::ptrdiff_t  = 0x74006100; // 
                constexpr std::ptrdiff_t  = 0x64006E00; // 
                constexpr std::ptrdiff_t  = 0x61006C00; // 
                constexpr std::ptrdiff_t  = 0x30003200; // 
                constexpr std::ptrdiff_t  = 0x6E00; // 
                constexpr std::ptrdiff_t  = 0x7F; // 
                constexpr std::ptrdiff_t  = 0x40000000; // 
                constexpr std::ptrdiff_t  = 0x73006900; // 
                constexpr std::ptrdiff_t  = 0x101; // 
                constexpr std::ptrdiff_t  = 0x69004400; // 
                constexpr std::ptrdiff_t  = 0x65006E00; // 
                constexpr std::ptrdiff_t  = 0x8E000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE000000; // 
                constexpr std::ptrdiff_t  = 0x80000000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D004D00; // 
                constexpr std::ptrdiff_t  = 0x2000000; // 
                constexpr std::ptrdiff_t  = 0xFFFF00; // 
                constexpr std::ptrdiff_t  = 0x15050000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100; // 
                constexpr std::ptrdiff_t  = 0x110; // 
            }
            // Parent: None
            // Field count: 48
            namespace SosEditItemInfo_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: soundsystem_lowlevel
            // Field count: 0
            namespace CVMixFlangerProcessorDesc {
            }
            // Parent: soundsystem_lowlevel
            // Field count: 0
            namespace CVMixEffectChainProcessorDesc {
            }
            // Parent: None
            // Field count: 45
            namespace KeyGroup_t {
                constexpr std::ptrdiff_t  = 0x10110FF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x45; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1032000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x61007400; // 
                constexpr std::ptrdiff_t  = 0x6F006400; // 
                constexpr std::ptrdiff_t  = 0x74006100; // 
                constexpr std::ptrdiff_t  = 0x2E003000; // 
                constexpr std::ptrdiff_t  = 0x3A004300; // 
                constexpr std::ptrdiff_t  = 0x63006900; // 
                constexpr std::ptrdiff_t  = 0x44002000; // 
                constexpr std::ptrdiff_t  = 0x6D007200; // 
                constexpr std::ptrdiff_t  = 0x5C003000; // 
                constexpr std::ptrdiff_t  = 0x66006E00; // 
                constexpr std::ptrdiff_t  = 0x61004400; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x6C005000; // 
                constexpr std::ptrdiff_t  = 0x32003000; // 
                constexpr std::ptrdiff_t  = 0x6E006500; // 
                constexpr std::ptrdiff_t  = 0x7FF98B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x69006400; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x44005C00; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7700; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D004D00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100F003; // 
                constexpr std::ptrdiff_t  = 0x1100000; // 
            }
            // Parent: None
            // Field count: 0
            namespace CVMixFreeverbProcessorDesc {
            }
            // Parent: None
            // Field count: 48
            namespace VMixPlateverbDesc_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: soundsystem_lowlevel
            // Field count: 0
            namespace CVMixConvolutionProcessorDesc {
            }
            // Parent: None
            // Field count: 48
            namespace CSoundContainerReferenceArray {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: ______
            // Field count: 0
            namespace CVMixFilterProcessorDesc {
            }
            // Parent: None
            // Field count: 48
            namespace CVMixAdditionalOutput {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace CVoiceContainerTapePlayer {
            }
            // Parent: ______
            // Field count: 0
            namespace CVMixSubgraphSwitchProcessorDesc {
            }
            // Parent: None
            // Field count: 48
            namespace VMixDiffusorDesc_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            namespace CVMixSteamAudioPathingProcessorDesc {
            }
            // Parent: soundsystem_lowlevel
            // Field count: 0
            namespace CVMixPannerProcessorDesc {
            }
            // Parent: None
            // Field count: 48
            namespace VMixDynamicsCompressorDesc_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: soundsystem_lowlevel
            // Field count: 0
            namespace CVMixStereoDelayProcessorDesc {
            }
            // Parent: None
            // Field count: 48
            namespace VMixShaperDesc_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            namespace VMixEnvelopeDesc_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            namespace CAudioSentence {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace CVoiceContainerParameterBlender {
            }
            // Parent: None
            // Field count: 48
            namespace CVMixAudioMeter {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: soundsystem_lowlevel
            // Field count: 0
            namespace CVMixVocoderProcessorDesc {
            }
            // Parent: ___Y__
            // Field count: 0
            namespace CSosGroupActionLimitSchema {
            }
            // Parent: soundsystem_voicecontainers
            // Field count: 0
            namespace CVoiceContainerAmpedDecayingSineWave {
            }
            // Parent: None
            // Field count: 0
            namespace CVoiceContainerEnvelope {
            }
            // Parent: None
            // Field count: 48
            namespace VMixAutoFilterDesc_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            namespace VMixDynamicsBand_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            namespace VMixEffectChainDesc_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: ______
            // Field count: 0
            namespace CVMixPlateReverbProcessorDesc {
            }
            // Parent: None
            // Field count: 0
            namespace CVoiceContainerMultiBlender {
            }
            // Parent: None
            // Field count: 0
            namespace CVMixVsndInput {
            }
            // Parent: None
            // Field count: 48
            namespace CVoiceContainerStaticAdditiveSynth__CGainScalePerInstance {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 45
            namespace VelocityZone_t {
                constexpr std::ptrdiff_t  = 0x10110FF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x45; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1032000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x61007400; // 
                constexpr std::ptrdiff_t  = 0x6F006400; // 
                constexpr std::ptrdiff_t  = 0x74006100; // 
                constexpr std::ptrdiff_t  = 0x2E003000; // 
                constexpr std::ptrdiff_t  = 0x3A004300; // 
                constexpr std::ptrdiff_t  = 0x63006900; // 
                constexpr std::ptrdiff_t  = 0x44002000; // 
                constexpr std::ptrdiff_t  = 0x6D007200; // 
                constexpr std::ptrdiff_t  = 0x5C003000; // 
                constexpr std::ptrdiff_t  = 0x66006E00; // 
                constexpr std::ptrdiff_t  = 0x61004400; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x6C005000; // 
                constexpr std::ptrdiff_t  = 0x32003000; // 
                constexpr std::ptrdiff_t  = 0x6E006500; // 
                constexpr std::ptrdiff_t  = 0x7FF98B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x69006400; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x44005C00; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7700; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D004D00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100F003; // 
                constexpr std::ptrdiff_t  = 0x1100000; // 
            }
            // Parent: None
            // Field count: 0
            namespace CVoiceContainerSelector {
            }
            // Parent: None
            // Field count: 0
            namespace CSosGroupActionTimeBlockLimitSchema {
            }
            // Parent: None
            // Field count: 0
            namespace CSosGroupActionMemberCountEnvelopeSchema {
            }
            // Parent: None
            // Field count: 0
            namespace CVMixDualCompressorProcessorDesc {
            }
            // Parent: None
            // Field count: 0
            namespace CVoiceContainerSwitch {
            }
            // Parent: soundsystem_lowlevel
            // Field count: 0
            namespace CVMixControlMeter {
            }
            // Parent: None
            // Field count: 0
            namespace CVoiceContainerEnum {
            }
            // Parent: ___Y__
            // Field count: 0
            namespace CSosGroupActionTimeLimitSchema {
            }
            // Parent: None
            // Field count: 48
            namespace VMixVocoderDesc_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            namespace VMixUtilityDesc_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            namespace CVoiceContainerLoopTrigger {
            }
            // Parent: None
            // Field count: 0
            namespace CVoiceContainerDecayingSineWave {
            }
            // Parent: ___Y__
            // Field count: 0
            namespace CVMixDelayProcessorDesc {
            }
            // Parent: None
            // Field count: 0
            namespace CVMixBoxverb2ProcessorDesc {
            }
            // Parent: None
            // Field count: 48
            namespace VMixFilterDesc_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            namespace VMixOscDesc_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            namespace CVMixAutoFilterProcessorDesc {
            }
            // Parent: None
            // Field count: 45
            namespace ISndSeqInstruments {
                constexpr std::ptrdiff_t  = 0x10110FF; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x45; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x400; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1032000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x61007400; // 
                constexpr std::ptrdiff_t  = 0x6F006400; // 
                constexpr std::ptrdiff_t  = 0x74006100; // 
                constexpr std::ptrdiff_t  = 0x2E003000; // 
                constexpr std::ptrdiff_t  = 0x3A004300; // 
                constexpr std::ptrdiff_t  = 0x63006900; // 
                constexpr std::ptrdiff_t  = 0x44002000; // 
                constexpr std::ptrdiff_t  = 0x6D007200; // 
                constexpr std::ptrdiff_t  = 0x5C003000; // 
                constexpr std::ptrdiff_t  = 0x66006E00; // 
                constexpr std::ptrdiff_t  = 0x61004400; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x6C005000; // 
                constexpr std::ptrdiff_t  = 0x32003000; // 
                constexpr std::ptrdiff_t  = 0x6E006500; // 
                constexpr std::ptrdiff_t  = 0x7FF98B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x69006400; // 
                constexpr std::ptrdiff_t  = 0x1015000; // 
                constexpr std::ptrdiff_t  = 0x44005C00; // 
                constexpr std::ptrdiff_t  = 0x6E006900; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x7700; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D004D00; // 
                constexpr std::ptrdiff_t  = 0x600; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x100F003; // 
                constexpr std::ptrdiff_t  = 0x1100000; // 
            }
            // Parent: None
            // Field count: 48
            namespace VMixFlangerDesc_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            namespace VMixDynamicsDesc_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x50005C; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x660065; // 
                constexpr std::ptrdiff_t  = 0x34005C; // 
                constexpr std::ptrdiff_t  = 0x70004D; // 
                constexpr std::ptrdiff_t  = 0x670069; // 
                constexpr std::ptrdiff_t  = 0x610074; // 
                constexpr std::ptrdiff_t  = 0x6F0064; // 
                constexpr std::ptrdiff_t  = 0x740061; // 
                constexpr std::ptrdiff_t  = 0x2E0030; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xE02FB640; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
        }
    }
}
