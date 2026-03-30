// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-03-30 13:31:39.632920200 +07:00

export const Schemas = {
    soundsystem_dll: {
        SndSeqInstrumentType_t: {
            eSndSeqInstNull: 0x0,
            eSndSeqInstSndEvt: 0x1,
            eSndSeqInstMidiSampler: 0x2,
        },
        EMode_t: {
            Peak: 0x0,
            RMS: 0x1,
        },
        VMixGraphCommandID_t: {
            CMD_INVALID: 0xFFFFFFFFFFFFFFFF,
            CMD_CONTROL_INPUT_STORE: 0x1,
            CMD_CONTROL_INPUT_STORE_DB: 0x2,
            CMD_CONTROL_TRANSIENT_INPUT_STORE: 0x3,
            CMD_CONTROL_TRANSIENT_INPUT_RESET: 0x4,
            CMD_CONTROL_OUTPUT_STORE: 0x5,
            CMD_CONTROL_EVALUATE_CURVE: 0x6,
            CMD_CONTROL_COPY: 0x7,
            CMD_CONTROL_COND_COPY_IF_NEGATIVE: 0x8,
            CMD_CONTROL_REMAP_LINEAR: 0x9,
            CMD_CONTROL_REMAP_SINE: 0xA,
            CMD_CONTROL_REMAP_LOGLINEAR: 0xB,
            CMD_CONTROL_MAX: 0xC,
            CMD_CONTROL_RESET_TIMER: 0xD,
            CMD_CONTROL_INCREMENT_TIMER: 0xE,
            CMD_CONTROL_EVAL_ENVELOPE: 0xF,
            CMD_CONTROL_SINE_BLEND: 0x10,
            CMD_PROCESSOR_SET_CONTROL_VALUE: 0x11,
            CMD_PROCESSOR_SET_NAME_INPUT: 0x12,
            CMD_PROCESSOR_SET_CONTROL_ARRAYVALUE: 0x13,
            CMD_PROCESSOR_STORE_CONTROL_VALUE: 0x14,
            CMD_PROCESSOR_SET_VSND_VALUE: 0x15,
            CMD_SUBMIX_PROCESS: 0x16,
            CMD_SUBMIX_GENERATE: 0x17,
            CMD_SUBMIX_GENERATE_SIDECHAIN: 0x18,
            CMD_SUBMIX_DEBUG: 0x19,
            CMD_SUBMIX_MIX2x1: 0x1A,
            CMD_SUBMIX_OUTPUT: 0x1B,
            CMD_SUBMIX_OUTPUTx2: 0x1C,
            CMD_SUBMIX_COPY: 0x1D,
            CMD_SUBMIX_ACCUMULATE: 0x1E,
            CMD_SUBMIX_METER: 0x1F,
            CMD_SUBMIX_METER_SPECTRUM: 0x20,
            CMD_IMPULSERESPONSE_INPUT_STORE: 0x21,
            CMD_PROCESSOR_SET_IMPULSERESPONSE_VALUE: 0x22,
            CMD_REMAP_VSND_TO_IMPULSERESPONSE: 0x23,
            CMD_IMPULSERESPONSE_RESET: 0x24,
            CMD_BLEND_VSNDS_TO_IMPULSERESPONSE: 0x25,
            CMD_IMPULSERESPONSE_DELAY: 0x26,
        },
        EWaveform: {
            Sine: 0x0,
            Square: 0x1,
            Saw: 0x2,
            Triangle: 0x3,
            Noise: 0x4,
        },
        VMixLFOShape_t: {
            LFO_SHAPE_SINE: 0x0,
            LFO_SHAPE_SQUARE: 0x1,
            LFO_SHAPE_TRI: 0x2,
            LFO_SHAPE_SAW: 0x3,
            LFO_SHAPE_NOISE: 0x4,
        },
        VMixFilterType_t: {
            FILTER_UNKNOWN: 0xFFFFFFFFFFFFFFFF,
            FILTER_LOWPASS: 0x0,
            FILTER_HIGHPASS: 0x1,
            FILTER_BANDPASS: 0x2,
            FILTER_NOTCH: 0x3,
            FILTER_PEAKING_EQ: 0x4,
            FILTER_LOW_SHELF: 0x5,
            FILTER_HIGH_SHELF: 0x6,
            FILTER_ALLPASS: 0x7,
            FILTER_PASSTHROUGH: 0x8,
        },
        SosActionStopType_t: {
            SOS_STOPTYPE_NONE: 0x0,
            SOS_STOPTYPE_TIME: 0x1,
            SOS_STOPTYPE_OPVAR: 0x2,
        },
        SndSeqTrackPlaybackType_t: {
            eSndSeqTrackPlaybackTypeStep: 0x0,
            eSndSeqTrackPlaybackTypeFwd: 0x1,
        },
        SosEditItemType_t: {
            SOS_EDIT_ITEM_TYPE_SOUNDEVENTS: 0x0,
            SOS_EDIT_ITEM_TYPE_SOUNDEVENT: 0x1,
            SOS_EDIT_ITEM_TYPE_LIBRARYSTACKS: 0x2,
            SOS_EDIT_ITEM_TYPE_STACK: 0x3,
            SOS_EDIT_ITEM_TYPE_OPERATOR: 0x4,
            SOS_EDIT_ITEM_TYPE_FIELD: 0x5,
        },
        SndSeqQuantizeType_t: {
            eSndSeqQuantizeInvalid: 0xFFFFFFFFFFFFFFFF,
            eSndSeqQuantizeNone: 0x0,
            eSndSeqQuantizeBeat: 0x1,
            eSndSeqQuantizeBar: 0x2,
            eSndSeqQuantizeSequence: 0x3,
            eSndSeqQuantizeSeek: 0x4,
            eSndSeqQuantizeReset: 0x5,
        },
        PlayBackMode_t: {
            Random: 0x0,
            RandomNoRepeats: 0x1,
            RandomAvoidLast: 0x2,
            Sequential: 0x3,
            RandomWeights: 0x4,
        },
        SosGroupFieldBehavior_t: {
            kIgnore: 0x0,
            kBranch: 0x1,
            kMatch: 0x2,
        },
        SndSeqSyncType_t: {
            eSndSeqSyncTypeNone: 0x0,
            eSndSeqSyncTypeWait: 0x1,
            eSndSeqSyncTypeSeek: 0x2,
        },
        soundlevel_t: {
            SNDLVL_NONE: 0x0,
            SNDLVL_20dB: 0x14,
            SNDLVL_25dB: 0x19,
            SNDLVL_30dB: 0x1E,
            SNDLVL_35dB: 0x23,
            SNDLVL_40dB: 0x28,
            SNDLVL_45dB: 0x2D,
            SNDLVL_50dB: 0x32,
            SNDLVL_55dB: 0x37,
            SNDLVL_IDLE: 0x3C,
            SNDLVL_65dB: 0x41,
            SNDLVL_STATIC: 0x42,
            SNDLVL_70dB: 0x46,
            SNDLVL_NORM: 0x4B,
            SNDLVL_80dB: 0x50,
            SNDLVL_85dB: 0x55,
            SNDLVL_90dB: 0x5A,
            SNDLVL_95dB: 0x5F,
            SNDLVL_100dB: 0x64,
            SNDLVL_105dB: 0x69,
            SNDLVL_110dB: 0x6E,
            SNDLVL_120dB: 0x78,
            SNDLVL_130dB: 0x82,
            SNDLVL_GUNFIRE: 0x8C,
            SNDLVL_150dB: 0x96,
            SNDLVL_180dB: 0xB4,
        },
        VMixPannerType_t: {
            PANNER_TYPE_LINEAR: 0x0,
            PANNER_TYPE_EQUAL_POWER: 0x1,
        },
        VMixChannelOperation_t: {
            VMIX_CHAN_STEREO: 0x0,
            VMIX_CHAN_LEFT: 0x1,
            VMIX_CHAN_RIGHT: 0x2,
            VMIX_CHAN_SWAP: 0x3,
            VMIX_CHAN_MONO: 0x4,
            VMIX_CHAN_MID_SIDE: 0x5,
        },
        EMidiNote: {
            C: 0x0,
            C_Sharp: 0x1,
            D: 0x2,
            D_Sharp: 0x3,
            E: 0x4,
            F: 0x5,
            F_Sharp: 0x6,
            G: 0x7,
            G_Sharp: 0x8,
            A: 0x9,
            A_Sharp: 0xA,
            B: 0xB,
            Count: 0xC,
        },
        SndSeqRegionType_t: {
            eSndSeqRegionTypeNull: 0x0,
            eSndSeqRegionTypeSndEvt: 0x1,
            eSndSeqRegionTypeMidiSeq: 0x2,
        },
        CVSoundFormat_t: {
            PCM16: 0x0,
            PCM8: 0x1,
            MP3: 0x2,
            ADPCM: 0x3,
        },
        VMixFilterSlope_t: {
            FILTER_SLOPE_1POLE_6dB: 0x0,
            FILTER_SLOPE_1POLE_12dB: 0x1,
            FILTER_SLOPE_1POLE_18dB: 0x2,
            FILTER_SLOPE_1POLE_24dB: 0x3,
            FILTER_SLOPE_12dB: 0x4,
            FILTER_SLOPE_24dB: 0x5,
            FILTER_SLOPE_36dB: 0x6,
            FILTER_SLOPE_48dB: 0x7,
        },
        SndSeqPlayerType_t: {
            eSndSeqPlayerNull: 0x0,
            eSndSeqPlayerSndEvt: 0x1,
            eSndSeqPlayerMidiSeq: 0x2,
        },
        SosActionLimitSortType_t: {
            SOS_LIMIT_SORTTYPE_HIGHEST: 0x0,
            SOS_LIMIT_SORTTYPE_LOWEST: 0x1,
        },
        VMixSubgraphSwitchInterpolationType_t: {
            SUBGRAPH_INTERPOLATION_TEMPORAL_CROSSFADE: 0x0,
            SUBGRAPH_INTERPOLATION_TEMPORAL_FADE_OUT: 0x1,
            SUBGRAPH_INTERPOLATION_KEEP_LAST_SUBGRAPH_RUNNING: 0x2,
        },
        SosGroupType_t: {
            SOS_GROUPTYPE_DYNAMIC: 0x0,
            SOS_GROUPTYPE_STATIC: 0x1,
        },
        SosActionSetParamSortType_t: {
            SOS_SETPARAM_SORTTYPE_HIGHEST: 0x0,
            SOS_SETPARAM_SORTTYPE_LOWEST: 0x1,
        },
        SndSeqMidiStatusType_t: {
            SndSeqMidiStatusNoteOff: 0x8,
            SndSeqMidiStatusNoteOn: 0x9,
            SndSeqMidiStatusKeyPressure: 0xA,
            SndSeqMidiStatusCtrlChange: 0xB,
            SndSeqMidiStatusProgramChange: 0xC,
            SndSeqMidiStatusChannelPressure: 0xD,
            SndSeqMidiStatusPitchBend: 0xE,
        },
        CVMixInputBase: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CVoiceContainerBlender: {
        },
        CVMixPitchShiftProcessorDesc: {
        },
        VMixFreeverbDesc_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CVoiceContainerStaticAdditiveSynth__CHarmonic: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CVoiceContainerStaticAdditiveSynth__CTone: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSosGroupActionOcclusionSchema: {
        },
        CVoiceContainerRandomSampler: {
        },
        CVMixSteamAudioDirectProcessorDesc: {
        },
        CVMixSteamAudioHRTFProcessorDesc: {
        },
        CVoiceContainerDefault: {
        },
        CVSound: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CDSPPresetMixgroupModifierTable: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSosGroupActionSoundeventClusterSchema: {
        },
        CSosGroupActionSetSoundeventParameterSchema: {
        },
        CSoundContainerReference: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CVoiceContainerNull: {
        },
        VMixSubgraphSwitchDesc_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CVoiceContainerAnalysisBase: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSosGroupActionSoundeventMinMaxValuesSchema: {
        },
        CVMixEnvelopeProcessorDesc: {
        },
        CVMixCommand: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        SamplerVoice_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CVMixControlInput: {
        },
        CVMixBoxverbProcessorDesc: {
        },
        CVMixDynamicsCompressorProcessorDesc: {
        },
        VMixPannerDesc_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CSosGroupActionSoundeventPrioritySchema: {
        },
        CVoiceContainerRealtimeFMSineWave: {
        },
        SelectedEditItemInfo_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        VMixModDelayDesc_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CSndSeqInstSndEvtSchema: {
        },
        CVMixNameInputMeter: {
        },
        CSndSeqInstruments: {
        },
        CVMixDynamics3BandProcessorDesc: {
        },
        VMixBoxverbDesc_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CSosGroupActionSchema: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CSosSoundEventGroupSchema: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSndSeqInstMidiSampler: {
        },
        CSndSeqInstBaseSchema: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        VMixDynamics3BandDesc_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CVMixEQ8ProcessorDesc: {
        },
        CDSPMixgroupModifier: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CAudioMorphData: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        VMixDualCompressorDesc_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CVoiceContainerStaticAdditiveSynth: {
        },
        CVoiceContainerShapedNoise: {
        },
        CDspPresetModifierList: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CVoiceContainerBase: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CVMixDiffusorProcessorDesc: {
        },
        CVMixUtilityProcessorDesc: {
        },
        CVoiceContainerGranulator: {
        },
        CVMixPresetDSPProcessorDesc: {
        },
        VMixDelayDesc_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        VMixEQ8Desc_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CVMixDynamicsProcessorDesc: {
        },
        CVMixGraphDescData: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CVoiceContainerLoopXFade: {
        },
        VMixPresetDSPDesc_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CAudioPhonemeTag: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CVMixControlInputArray: {
        },
        CVMixNameInput: {
        },
        CSosGroupActionSoundeventCountSchema: {
        },
        CVoiceContainerEnvelopeAnalyzer: {
        },
        CVMixBaseProcessorDesc: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CVMixImpulseResponseInput: {
        },
        CVMixSteamAudioHybridReverbProcessorDesc: {
        },
        CSoundEventMetaData: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        VMixPitchShiftDesc_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CVMixControlOutput: {
        },
        CVMixModDelayProcessorDesc: {
        },
        CVMixShaperProcessorDesc: {
        },
        CVMixAutomaticControlInput: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CAudioEmphasisSample: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CVMixOscProcessorDesc: {
        },
        CVMixCurveHeader: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CVoiceContainerGenerator: {
        },
        CVoiceContainerSet: {
        },
        VMixConvolutionDesc_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CVoiceContainerSetElement: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CVoiceContainerAsyncGenerator: {
        },
        CSoundInfoHeader: {
            : 0x1FFEEFF, // 
            : 0x100, // 
            : 0x1000000, // 
            : 0x80, // 
            : 0x0, // 
            : 0x1, // 
            : 0x2C000, // 
            : 0x10F, // 
            : 0x101, // 
            : 0x0, // 
            : 0x107, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x103, // 
            : 0x0, // 
            : 0x0, // 
            : 0x101, // 
            : 0xE02FA7, // 
            : 0x0, // 
            : 0x5C006100, // 
            : 0x77006F00, // 
            : 0x66007400, // 
            : 0x36002E00, // 
            : 0x5C003A00, // 
            : 0x72006300, // 
            : 0x65004400, // 
            : 0x5C006D00, // 
            : 0x4D005C00, // 
            : 0x69006600, // 
            : 0x74006100, // 
            : 0x64006E00, // 
            : 0x61006C00, // 
            : 0x30003200, // 
            : 0x6E00, // 
            : 0x7F, // 
            : 0x40000000, // 
            : 0x73006900, // 
            : 0x101, // 
            : 0x69004400, // 
            : 0x65006E00, // 
            : 0x8E000000, // 
            : 0x0, // 
            : 0xEE000000, // 
            : 0x80000000, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x2000000, // 
            : 0xFFFF00, // 
            : 0x15050000, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100, // 
            : 0x110, // 
        },
        SosEditItemInfo_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CVMixFlangerProcessorDesc: {
        },
        CVMixEffectChainProcessorDesc: {
        },
        KeyGroup_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CVMixFreeverbProcessorDesc: {
        },
        VMixPlateverbDesc_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CVMixConvolutionProcessorDesc: {
        },
        CSoundContainerReferenceArray: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CVMixFilterProcessorDesc: {
        },
        CVMixAdditionalOutput: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CVoiceContainerTapePlayer: {
        },
        CVMixSubgraphSwitchProcessorDesc: {
        },
        VMixDiffusorDesc_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CVMixSteamAudioPathingProcessorDesc: {
        },
        CVMixPannerProcessorDesc: {
        },
        VMixDynamicsCompressorDesc_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CVMixStereoDelayProcessorDesc: {
        },
        VMixShaperDesc_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        VMixEnvelopeDesc_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CAudioSentence: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CVoiceContainerParameterBlender: {
        },
        CVMixAudioMeter: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CVMixVocoderProcessorDesc: {
        },
        CSosGroupActionLimitSchema: {
        },
        CVoiceContainerAmpedDecayingSineWave: {
        },
        CVoiceContainerEnvelope: {
        },
        VMixAutoFilterDesc_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        VMixDynamicsBand_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        VMixEffectChainDesc_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CVMixPlateReverbProcessorDesc: {
        },
        CVoiceContainerMultiBlender: {
        },
        CVMixVsndInput: {
        },
        CVoiceContainerStaticAdditiveSynth__CGainScalePerInstance: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        VelocityZone_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CVoiceContainerSelector: {
        },
        CSosGroupActionTimeBlockLimitSchema: {
        },
        CSosGroupActionMemberCountEnvelopeSchema: {
        },
        CVMixDualCompressorProcessorDesc: {
        },
        CVoiceContainerSwitch: {
        },
        CVMixControlMeter: {
        },
        CVoiceContainerEnum: {
        },
        CSosGroupActionTimeLimitSchema: {
        },
        VMixVocoderDesc_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        VMixUtilityDesc_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CVoiceContainerLoopTrigger: {
        },
        CVoiceContainerDecayingSineWave: {
        },
        CVMixDelayProcessorDesc: {
        },
        CVMixBoxverb2ProcessorDesc: {
        },
        VMixFilterDesc_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        VMixOscDesc_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CVMixAutoFilterProcessorDesc: {
        },
        ISndSeqInstruments: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        VMixFlangerDesc_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        VMixDynamicsDesc_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
    },
};
