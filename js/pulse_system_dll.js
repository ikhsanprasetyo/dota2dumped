// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-08 20:27:02.884074100 +07:00

export const Schemas = {
    pulse_system_dll: {
        PulseBestOutflowRules_t: {
            SORT_BY_NUMBER_OF_VALID_CRITERIA: 0x0,
            SORT_BY_OUTFLOW_INDEX: 0x1,
        },
        PulseTestEnumFlags_t: {
            NONE: 0x0,
            FIRST: 0x1,
            SECOND: 0x2,
            THIRD: 0x4,
        },
        PulseTestEnumShape_t: {
            CIRCLE: 0x64,
            SQUARE: 0xC8,
            TRIANGLE: 0x12C,
        },
        PulseCursorCancelPriority_t: {
            None: 0x0,
            CancelOnSucceeded: 0x1,
            SoftCancel: 0x2,
            HardCancel: 0x3,
        },
        PulseTestEnumFlagsAlt_t: {
            NONE: 0x0,
            FIRST: 0x1,
        },
        PulseMethodCallMode_t: {
            SYNC_WAIT_FOR_COMPLETION: 0x0,
            ASYNC_FIRE_AND_FORGET: 0x1,
        },
        PulseTestEnumColor_t: {
            BLACK: 0x0,
            WHITE: 0x1,
            RED: 0x2,
            GREEN: 0x3,
            BLUE: 0x4,
        },
        PulseCursorWakePriority_t: {
            WakeElegantly: 0x0,
            WakeImmediate: 0x1,
        },
        PulseVariableKeysSource_t: {
            PRIVATE: 0x0,
            CPP: 0x1,
            VMAP: 0x2,
            VMDL: 0x3,
            XML: 0x4,
            VDATA: 0x5,
            COUNT: 0x6,
        },
        PulseDurationStringFormat_t: {
            MM_SS_LEADING_ZERO: 0x0,
        },
        EPulseGraphExecutionHistoryFlag: {
            NO_FLAGS: 0x0,
            CURSOR_ADD_TAG: 0x1,
            CURSOR_REMOVE_TAG: 0x2,
            CURSOR_RETIRED: 0x4,
            CURSOR_CREATE_CHILD: 0x8,
            REQUIREMENT_PASS: 0x10,
            REQUIREMENT_FAIL: 0x20,
            CALL_TO_PULSE: 0x40,
            RETURN: 0x80,
        },
        PulseValueType_t: {
            PVAL_VOID: 0xFFFFFFFFFFFFFFFF,
            PVAL_BOOL: 0x0,
            PVAL_INT: 0x1,
            PVAL_FLOAT: 0x2,
            PVAL_STRING: 0x3,
            PVAL_VEC2: 0x4,
            PVAL_VEC3: 0x5,
            PVAL_QANGLE: 0x6,
            PVAL_VEC3_WORLDSPACE: 0x7,
            PVAL_VEC4: 0x8,
            PVAL_TRANSFORM: 0x9,
            PVAL_TRANSFORM_WORLDSPACE: 0xA,
            PVAL_COLOR_RGB: 0xB,
            PVAL_GAMETIME: 0xC,
            PVAL_EHANDLE: 0xD,
            PVAL_RESOURCE: 0xE,
            PVAL_RESOURCE_NAME: 0xF,
            PVAL_SNDEVT_GUID: 0x10,
            PVAL_SNDEVT_NAME: 0x11,
            PVAL_ENTITY_NAME: 0x12,
            PVAL_LEAF: 0x13,
            PVAL_MODEL_MATERIAL_GROUP: 0x14,
            PVAL_CURSOR_FLOW: 0x15,
            PVAL_VARIANT: 0x16,
            PVAL_UNKNOWN: 0x17,
            PVAL_ENUM: 0x18,
            PVAL_PANORAMA_PANEL_HANDLE: 0x19,
            PVAL_ARRAY: 0x1A,
            PVAL_PARTICLE_EHANDLE: 0x1B,
            PVAL_ANIM_SEQUENCE: 0x1C,
            PVAL_VDATA_CHOICE: 0x1D,
            PVAL_COUNT: 0x1E,
        },
        PulseApiFeature_t: {
            AF_NONE: 0x0,
            AF_ENTITIES: 0x1,
            AF_PANORAMA: 0x2,
            AF_PARTICLES: 0x8,
            AF_FAKE_ENTITIES: 0x10,
            AF_SELECTORS_WITHOUT_REQUIREMENTS: 0x20,
        },
        PulseInstructionCode_t: {
            INVALID: 0x0,
            IMMEDIATE_HALT: 0x1,
            RETURN_VOID: 0x2,
            RETURN_VALUE: 0x3,
            LOOP_BREAK: 0x4,
            NOP: 0x5,
            JUMP: 0x6,
            JUMP_COND: 0x7,
            CHUNK_LEAP: 0x8,
            CHUNK_LEAP_COND: 0x9,
            PULSE_CALL_SYNC: 0xA,
            PULSE_CALL_ASYNC_FIRE: 0xB,
            CREATE_CHILD_CURSOR_OUTFLOW: 0xC,
            CELL_INVOKE: 0xD,
            LIBRARY_INVOKE: 0xE,
            SET_VAR: 0xF,
            GET_VAR: 0x10,
            GET_VAR_DETACH: 0x11,
            DETACH_REGISTER: 0x12,
            SET_VAR_ARRAY_ELEMENT_1D: 0x13,
            SET_VAR_OBSERVABLE: 0x14,
            GET_CONST: 0x15,
            GET_ARRAY_ELEMENT: 0x16,
            GET_DOMAIN_VALUE: 0x17,
            COPY: 0x18,
            NOT: 0x19,
            NEGATE: 0x1A,
            ADD: 0x1B,
            SUB: 0x1C,
            MUL: 0x1D,
            DIV: 0x1E,
            MOD: 0x1F,
            LT: 0x20,
            LTE: 0x21,
            EQ: 0x22,
            NE: 0x23,
            AND: 0x24,
            OR: 0x25,
            SCALE: 0x26,
            SCALE_INV: 0x27,
            ELEMENT_ACCESS: 0x28,
            CONVERT_VALUE: 0x29,
            REINTERPRET_INSTANCE: 0x2A,
            GET_BLACKBOARD_REFERENCE: 0x2B,
            SET_BLACKBOARD_REFERENCE: 0x2C,
            GET_TEMPVAR: 0x2D,
            SET_TEMPVAR: 0x2E,
            SET_TEMPVAR_OBSERVABLE: 0x2F,
            LAST_SERIALIZED_CODE: 0x30,
            NEGATE_INT: 0x31,
            NEGATE_FLOAT: 0x32,
            NEGATE_VEC2: 0x33,
            NEGATE_VEC3: 0x34,
            NEGATE_VEC4: 0x35,
            ADD_INT: 0x36,
            ADD_FLOAT: 0x37,
            ADD_STRING: 0x38,
            ADD_VEC2: 0x39,
            ADD_VEC3: 0x3A,
            ADD_VEC3WS_VEC3: 0x3B,
            ADD_VEC3_VEC3WS: 0x3C,
            ADD_VEC4: 0x3D,
            ADD_GAMETIME_FLOAT: 0x3E,
            ADD_FLOAT_GAMETIME: 0x3F,
            SUB_INT: 0x40,
            SUB_FLOAT: 0x41,
            SUB_VEC2: 0x42,
            SUB_VEC3: 0x43,
            SUB_VEC3WS_VEC3: 0x44,
            SUB_VEC3WS_VEC3WS: 0x45,
            SUB_VEC4: 0x46,
            SUB_GAMETIME_FLOAT: 0x47,
            SUB_GAMETIME: 0x48,
            MUL_INT: 0x49,
            MUL_FLOAT: 0x4A,
            DIV_FLOAT: 0x4B,
            MOD_INT: 0x4C,
            MOD_FLOAT: 0x4D,
            LT_INT: 0x4E,
            LT_FLOAT: 0x4F,
            LT_GAMETIME: 0x50,
            LTE_INT: 0x51,
            LTE_FLOAT: 0x52,
            LTE_GAMETIME: 0x53,
            EQ_BOOL: 0x54,
            EQ_INT: 0x55,
            EQ_FLOAT: 0x56,
            EQ_VEC2: 0x57,
            EQ_VEC3: 0x58,
            EQ_VEC3WS: 0x59,
            EQ_VEC4: 0x5A,
            EQ_STRING: 0x5B,
            EQ_ENTITY_NAME: 0x5C,
            EQ_ENUM: 0x5D,
            EQ_EHANDLE: 0x5E,
            EQ_PANEL_HANDLE: 0x5F,
            EQ_LEAF: 0x60,
            EQ_COLOR_RGB: 0x61,
            EQ_ARRAY: 0x62,
            EQ_GAMETIME: 0x63,
            NE_BOOL: 0x64,
            NE_INT: 0x65,
            NE_FLOAT: 0x66,
            NE_VEC2: 0x67,
            NE_VEC3: 0x68,
            NE_VEC3WS: 0x69,
            NE_VEC4: 0x6A,
            NE_STRING: 0x6B,
            NE_ENTITY_NAME: 0x6C,
            NE_ENUM: 0x6D,
            NE_EHANDLE: 0x6E,
            NE_PANEL_HANDLE: 0x6F,
            NE_LEAF: 0x70,
            NE_COLOR_RGB: 0x71,
            NE_ARRAY: 0x72,
            NE_GAMETIME: 0x73,
            SCALE_VEC3: 0x74,
            SCALE_VEC2: 0x75,
            SCALE_VEC4: 0x76,
            SCALE_INV_VEC3: 0x77,
            SCALE_INV_VEC2: 0x78,
            SCALE_INV_VEC4: 0x79,
            ELEMENT_ACCESS_VEC2: 0x7A,
            ELEMENT_ACCESS_VEC3: 0x7B,
            ELEMENT_ACCESS_VEC3WS: 0x7C,
            ELEMENT_ACCESS_VEC4: 0x7D,
            ELEMENT_ACCESS_COLOR_RGB: 0x7E,
            GET_CONST_INLINE_STORAGE: 0x7F,
        },
        PulseDomainValueType_t: {
            INVALID: 0xFFFFFFFFFFFFFFFF,
            ENTITY_NAME: 0x0,
            PANEL_ID: 0x1,
            COUNT: 0x2,
        },
        CPulseGraphInstance_TestDomain_FakeEntityOwner: {
        },
        CPulse_ResumePoint: {
        },
        CTestDomainDerived_Cursor: {
            m_nCursorValueA: 0xD8, // int32
            m_nCursorValueB: 0xDC, // int32
        },
        CPulse_OutflowConnection: {
            m_SourceOutflowName: 0x0, // PulseSymbol_t
            m_nDestChunk: 0x10, // PulseRuntimeChunkIndex_t
            m_nInstruction: 0x14, // int32
            m_OutflowRegisterMap: 0x18, // PulseRegisterMap_t
        },
        CPulseGraphInstance_TestDomain_UseReadOnlyBlackboardView: {
        },
        CPulseCell_TestWaitWithCursorState__InstanceState_t: {
            m_nDummy: 0x0, // int32
        },
        CBasePulseGraphInstance: {
        },
        SignatureOutflow_Resume: {
        },
        CPulseCell_Test_MultiOutflow_WithParams_Yielding__CursorState_t: {
            nTestStep: 0x0, // int32
        },
        CPulseTurtleGraphicsCursor: {
            m_Color: 0xD8, // Color
            m_vPos: 0xDC, // Vector2D
            m_flHeadingDeg: 0xE4, // float32
            m_bPenUp: 0xE8, // bool
        },
        CPulseCell_TestWaitWithCursorState__CursorState_t: {
            flWaitValue: 0x0, // float32
            bFail: 0x4, // bool
            m_hSelfCursor: 0x8, // HYieldedCursor
            m_hSelfCellInstanceUntyped: 0x14, // HPulseCellBase
            m_hSelfCellInstance: 0x1C, // HPulseCell<CPulseCell_TestWaitWithCursorState>
        },
        CPulseGraphInstance_TestDomain_Derived: {
            m_nInstanceValueX: 0xD8, // int32
        },
        CPulseGraphInstance_TestDomain: {
            m_bIsRunningUnitTests: 0xA8, // bool
            m_bExplicitTimeStepping: 0xA9, // bool
            m_bExpectingToDestroyWithYieldedCursors: 0xAA, // bool
            m_bQuietTracepoints: 0xAB, // bool
            m_bExpectingCursorTerminatedDueToMaxInstructions: 0xAC, // bool
            m_nCursorsTerminatedDueToMaxInstructions: 0xB0, // int32
            m_nNextValidateIndex: 0xB4, // int32
            m_Tracepoints: 0xB8, // CUtlVector<CUtlString>
            m_bTestYesOrNoPath: 0xD0, // bool
        },
        SignatureOutflow_Continue: {
        },
        CPulseCell_IsRequirementValid__Criteria_t: {
            m_bIsValid: 0x0, // bool
        },
        CPulseGraphInstance_TurtleGraphics: {
        },
        CPulseCell_Unknown: {
            m_UnknownKeys: 0x48, // KeyValues3
        },
        CPulseCell_ExampleCriteria__Criteria_t: {
            m_flFloatValue1: 0x0, // float32
            m_flFloatValue2: 0x4, // float32
            m_bMyBool: 0x8, // bool
        },
        CPulseCell_LimitCount__Criteria_t: {
            m_bLimitCountPasses: 0x0, // bool
        },
        CPulseExecCursor: {
        },
        TestComponent_t: {
            m_ComponentData: 0x8, // CUtlString
        },
        PulseRegisterMap_t: {
            m_Inparams: 0x0, // KeyValues3
            m_InparamsWhichCanBeMoved: 0x10, // CKV3MemberNameSet
            m_Outparams: 0x20, // KeyValues3
        },
    },
};
