// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-09 13:46:49.828411900 +07:00

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
        SndBeatMidiStatusType_t: {
            SndSeqMidiStatusNoteOff: 0x8,
            SndSeqMidiStatusNoteOn: 0x9,
            SndSeqMidiStatusKeyPressure: 0xA,
            SndSeqMidiStatusCtrlChange: 0xB,
            SndSeqMidiStatusProgramChange: 0xC,
            SndSeqMidiStatusChannelPressure: 0xD,
            SndSeqMidiStatusPitchBend: 0xE,
        },
        VMixGraphCommandID_t: {
            CMD_INVALID: 0xFFFFFFFFFFFFFFFF,
            CMD_CONTROL_CONVERT_DB_TO_GAIN: 0x1,
            CMD_CONTROL_TRANSIENT_INPUT_STORE: 0x2,
            CMD_CONTROL_TRANSIENT_INPUT_RESET: 0x3,
            CMD_CONTROL_OUTPUT_STORE: 0x4,
            CMD_CONTROL_EVALUATE_CURVE: 0x5,
            CMD_CONTROL_COPY: 0x6,
            CMD_CONTROL_COND_COPY_IF_NEGATIVE: 0x7,
            CMD_CONTROL_REMAP_LINEAR: 0x8,
            CMD_CONTROL_REMAP_SINE: 0x9,
            CMD_CONTROL_REMAP_LOGLINEAR: 0xA,
            CMD_CONTROL_MAX: 0xB,
            CMD_CONTROL_RESET_TIMER: 0xC,
            CMD_CONTROL_INCREMENT_TIMER: 0xD,
            CMD_CONTROL_EVAL_ENVELOPE: 0xE,
            CMD_CONTROL_SINE_BLEND: 0xF,
            CMD_SUBMIX_PROCESS: 0x10,
            CMD_SUBMIX_GENERATE: 0x11,
            CMD_SUBMIX_GENERATE_SIDECHAIN: 0x12,
            CMD_SUBMIX_EXTRACTCONTAINER: 0x13,
            CMD_SUBMIX_DEBUG: 0x14,
            CMD_SUBMIX_MIX2x1: 0x15,
            CMD_SUBMIX_OUTPUT: 0x16,
            CMD_SUBMIX_OUTPUTx2: 0x17,
            CMD_SUBMIX_COPY: 0x18,
            CMD_SUBMIX_ACCUMULATE: 0x19,
            CMD_SUBMIX_METER: 0x1A,
            CMD_SUBMIX_METER_SPECTRUM: 0x1B,
            CMD_IMPULSERESPONSE_INPUT_STORE: 0x1C,
            CMD_PROCESSOR_SET_IMPULSERESPONSE_VALUE: 0x1D,
            CMD_REMAP_VSND_TO_IMPULSERESPONSE: 0x1E,
            CMD_IMPULSERESPONSE_RESET: 0x1F,
            CMD_BLEND_VSNDS_TO_IMPULSERESPONSE: 0x20,
            CMD_IMPULSERESPONSE_DELAY: 0x21,
        },
        EWaveform: {
            Sine: 0x0,
            Square: 0x1,
            Saw: 0x2,
            Triangle: 0x3,
            Noise: 0x4,
        },
        VMixFilterChannelSet_t: {
            FILTER_ALL_CHANNELS: 0x0,
            FILTER_LEFT_ONLY: 0x1,
            FILTER_RIGHT_ONLY: 0x2,
            FILTER_MID_ONLY: 0x3,
            FILTER_SIDE_ONLY: 0x4,
            FILTER_CHANNEL_SET_MAX: 0x5,
        },
        VMixLFOShape_t: {
            LFO_SHAPE_SINE: 0x0,
            LFO_SHAPE_SQUARE: 0x1,
            LFO_SHAPE_TRI: 0x2,
            LFO_SHAPE_SAW: 0x3,
            LFO_SHAPE_NOISE: 0x4,
        },
        VMixOffsetType_t: {
            VO_CHAR: 0x0,
            VO_ARRAY: 0x1,
            VO_BOOL: 0x2,
            VO_FLOAT: 0x3,
            VO_UINT32: 0x4,
            VO_INT32: 0x5,
            VO_VECTOR: 0x6,
            VO_QUATERNION: 0x7,
            VO_CUBIC_SPLINE: 0x8,
            VO_VSND_INPUT: 0x9,
            VO_FLOAT_UTLVECTOR: 0xA,
            VO_SHAREDPTR_IR: 0xB,
            VO_TYPE_COUNT: 0xC,
        },
        VMixMixDownRule_t: {
            SUM: 0x0,
            LEFT: 0x1,
            RIGHT: 0x2,
            MID: 0x3,
            SIDE: 0x4,
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
        SndBeatTrackPlaybackType_t: {
            eSndBeatTrackPlaybackTypeStep: 0x0,
            eSndBeatTrackPlaybackTypeFwd: 0x1,
        },
        VMixSendOperator_t: {
            NO_VOICES: 0xFFFFFFFFFFFFFFFF,
            ALL_VOICES: 0x0,
            ROOM_VOICES: 0x1,
            FACING_VOICES: 0x2,
            MIXGROUP_VOICES: 0x3,
            NAMED_SEND: 0x4,
            INVERSE_NAMED_SENDS: 0x5,
            INVERSE_TOTAL_SEND: 0x6,
            ALL_MAX_SEND: 0x7,
            TRACK: 0x8,
        },
        SndBeatEventType_t: {
            eSndBeatEventTypeInvalid: 0x0,
            eSndBeatEventTypeBeat: 0x1,
            eSndBeatEventTypeBar: 0x2,
            eSndBeatEventTypePhrase: 0x3,
            eSndBeatEventTypeLength: 0x4,
            eSndBeatEventTypeKeys: 0x5,
        },
        SosActionStopType_t: {
            SOS_STOPTYPE_NONE: 0x0,
            SOS_STOPTYPE_TIME: 0x1,
            SOS_STOPTYPE_OPVAR: 0x2,
        },
        SndBeatKeyType_t: {
            eSndBeatPatternTypeNone: 0x0,
            eSndBeatPatternTypeKeys: 0x1,
            eSndBeatPatternTypeKeyedFloats: 0x2,
            eSndBeatPatternTypeKeyedSndEvts: 0x3,
            eSndBeatPatternTypeKeyedMidi: 0x4,
        },
        SosEditItemType_t: {
            SOS_EDIT_ITEM_TYPE_SOUNDEVENTS: 0x0,
            SOS_EDIT_ITEM_TYPE_SOUNDEVENT: 0x1,
            SOS_EDIT_ITEM_TYPE_LIBRARYSTACKS: 0x2,
            SOS_EDIT_ITEM_TYPE_STACK: 0x3,
            SOS_EDIT_ITEM_TYPE_OPERATOR: 0x4,
            SOS_EDIT_ITEM_TYPE_FIELD: 0x5,
        },
        SndBeatSyncType_t: {
            eSndBeatSyncTypeInvalid: 0x0,
            eSndBeatSyncTypeReset: 0x1,
            eSndBeatSyncTypeSeekImmediate: 0x2,
        },
        PlayBackMode_t: {
            Random: 0x0,
            RandomNoRepeats: 0x1,
            RandomAvoidLast: 0x2,
            Sequential: 0x3,
            RandomWeights: 0x4,
        },
        EVsndTriggerMode: {
            Trigger: 0x0,
            Gate: 0x1,
        },
        SosGroupFieldBehavior_t: {
            kIgnore: 0x0,
            kBranch: 0x1,
            kMatch: 0x2,
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
        VMixAutoControlType_t: {
            VMIX_AUTO_SEND_LEVEL: 0x0,
            VMIX_AUTO_STACK_VAR: 0x1,
            VMIX_AUTO_PLAYTIME: 0x2,
            VMIX_AUTO_DISTANCE: 0x3,
            VMIX_AUTO_POSITION_X: 0x4,
            VMIX_AUTO_POSITION_Y: 0x5,
            VMIX_AUTO_POSITION_Z: 0x6,
            VMIX_AUTO_POSITION_VECTOR: 0x7,
            VMIX_AUTO_LISTENER_YAW_SIN: 0x8,
            VMIX_AUTO_LISTENER_YAW_COS: 0x9,
            VMIX_AUTO_LISTENER_PITCH_SIN: 0xA,
            VMIX_AUTO_LISTENER_PITCH_COS: 0xB,
            VMIX_AUTO_LISTENER_ROLL_SIN: 0xC,
            VMIX_AUTO_LISTENER_ROLL_COS: 0xD,
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
        VMixOffsetCategory_t: {
            NULL_POINTER: 0x0,
            HEAP_OFFSET: 0x1,
            INPUT_INDEX: 0x2,
            SUBMIX_INDEX: 0x3,
        },
        SndBeatSyncStartType_t: {
            eSndBeatSyncStartTypeInvalid: 0x0,
            eSndBeatSyncStartTypeImmediate: 0x1,
            eSndBeatSyncStartTypeQueue: 0x2,
        },
        SosActionSetParamSortType_t: {
            SOS_SETPARAM_SORTTYPE_HIGHEST: 0x0,
            SOS_SETPARAM_SORTTYPE_LOWEST: 0x1,
        },
        EVsndPlaybackMode: {
            Trigger: 0x0,
            Gate: 0x1,
        },
        CVsndTriggerSlot: {
            m_bEnableVsnd: 0x0, // bool
            m_vsnd: 0x8, // CSoundContainerReference
            m_bEnableEndcap: 0x28, // bool
            m_endcapVsnd: 0x30, // CSoundContainerReference
            m_bEnableLoopcap: 0x50, // bool
            m_loopcapVsnd: 0x58, // CSoundContainerReference
            m_volume: 0x78, // float32
            m_fadeOut: 0x7C, // float32
            m_mode: 0x80, // EVsndTriggerMode
        },
        CDSPPresetMixgroupModifierTable: {
            m_table: 0x0, // CUtlVector<CDspPresetModifierList>
        },
        SamplerVoice_t: {
            nNoteNum: 0x0, // uint8
        },
        VMixPannerDesc_t: {
            m_type: 0x0, // VMixPannerType_t
            m_flStrength: 0x4, // float32
        },
        CSndSeqInstruments: {
        },
        CSosSoundEventGroupSchema: {
            m_nGroupType: 0x8, // SosGroupType_t
            m_bBlocksEvents: 0xC, // bool
            m_nBlockMaxCount: 0x10, // int32
            m_flMemberLifespanTime: 0x14, // float32
            m_bInvertMatch: 0x18, // bool
            m_Behavior_EventName: 0x1C, // SosGroupFieldBehavior_t
            m_matchSoundEventName: 0x20, // CUtlString
            m_bMatchEventSubString: 0x28, // bool
            m_matchSoundEventSubString: 0x30, // CUtlString
            m_Behavior_EntIndex: 0x38, // SosGroupFieldBehavior_t
            m_flEntIndex: 0x3C, // float32
            m_Behavior_Opvar: 0x40, // SosGroupFieldBehavior_t
            m_flOpvar: 0x44, // float32
            m_Behavior_String: 0x48, // SosGroupFieldBehavior_t
            m_opvarString: 0x50, // CUtlString
            m_vActions: 0x58, // CUtlVector<CSosGroupActionSchema*>
        },
        VMixPointerFixupEntry_t: {
            m_nIndex: 0x0, // uint32
            m_offset: 0x4, // CVMixDataOffset
        },
        CSndBeatTrack: {
            m_name: 0x0, // CUtlString
            m_playbackType: 0x20, // SndBeatTrackPlaybackType_t
            m_nTranspose: 0x24, // int32
            m_bSyncToVoice: 0x28, // bool
            m_flBPM: 0x2C, // float32
        },
        CSoundInfoHeader: {
        },
        KeyGroup_t: {
            nCenterNote: 0x0, // uint8
            nMinNote: 0x1, // uint8
            nMaxNote: 0x2, // uint8
            nNumVelocityZones: 0x3, // uint8
            pVelocityZones: 0x8, // VelocityZone_t*
        },
        CVMixDataOffset: {
            m_nOffset: 0x0, // uint32
        },
        SndBeatTimeSignature_t: {
            nNumerator: 0x0, // uint8
            nDenominator: 0x1, // uint8
        },
        VelocityZone_t: {
            nMaxVel: 0x0, // uint8
            nNextSelection: 0x1, // uint8
            nNumSamples: 0x2, // uint8
            pSamples: 0x4, // uint32[4]
        },
        SndBeatEventKeys_t: {
            m_flKey: 0x8, // float32
        },
        ISndSeqInstruments: {
        },
    },
};
