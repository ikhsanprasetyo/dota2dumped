// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-04 12:25:34.881424700 +07:00

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
            REQUIREMENT_PASS: 0x8,
            REQUIREMENT_FAIL: 0x10,
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
            PVAL_OPAQUE_HANDLE: 0x13,
            PVAL_TYPESAFE_INT: 0x14,
            PVAL_MODEL_MATERIAL_GROUP: 0x15,
            PVAL_CURSOR_FLOW: 0x16,
            PVAL_VARIANT: 0x17,
            PVAL_UNKNOWN: 0x18,
            PVAL_SCHEMA_ENUM: 0x19,
            PVAL_PANORAMA_PANEL_HANDLE: 0x1A,
            PVAL_TEST_HANDLE: 0x1B,
            PVAL_ARRAY: 0x1C,
            PVAL_TYPESAFE_INT64: 0x1D,
            PVAL_PARTICLE_EHANDLE: 0x1E,
            PVAL_ANIM_SEQUENCE: 0x1F,
            PVAL_VDATA_CHOICE: 0x20,
            PVAL_COUNT: 0x21,
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
            NOP: 0x4,
            JUMP: 0x5,
            JUMP_COND: 0x6,
            CHUNK_LEAP: 0x7,
            CHUNK_LEAP_COND: 0x8,
            PULSE_CALL_SYNC: 0x9,
            PULSE_CALL_ASYNC_FIRE: 0xA,
            CREATE_CHILD_CURSOR_OUTFLOW: 0xB,
            CELL_INVOKE: 0xC,
            LIBRARY_INVOKE: 0xD,
            SET_VAR: 0xE,
            GET_VAR: 0xF,
            GET_VAR_DETACH: 0x10,
            DETACH_REGISTER: 0x11,
            SET_VAR_ARRAY_ELEMENT_1D: 0x12,
            SET_VAR_OBSERVABLE: 0x13,
            GET_CONST: 0x14,
            GET_ARRAY_ELEMENT: 0x15,
            GET_DOMAIN_VALUE: 0x16,
            COPY: 0x17,
            NOT: 0x18,
            NEGATE: 0x19,
            ADD: 0x1A,
            SUB: 0x1B,
            MUL: 0x1C,
            DIV: 0x1D,
            MOD: 0x1E,
            LT: 0x1F,
            LTE: 0x20,
            EQ: 0x21,
            NE: 0x22,
            AND: 0x23,
            OR: 0x24,
            SCALE: 0x25,
            SCALE_INV: 0x26,
            ELEMENT_ACCESS: 0x27,
            CONVERT_VALUE: 0x28,
            REINTERPRET_INSTANCE: 0x29,
            GET_BLACKBOARD_REFERENCE: 0x2A,
            SET_BLACKBOARD_REFERENCE: 0x2B,
            LAST_SERIALIZED_CODE: 0x2C,
            NEGATE_INT: 0x2D,
            NEGATE_FLOAT: 0x2E,
            NEGATE_VEC2: 0x2F,
            NEGATE_VEC3: 0x30,
            NEGATE_VEC4: 0x31,
            ADD_INT: 0x32,
            ADD_FLOAT: 0x33,
            ADD_STRING: 0x34,
            ADD_VEC2: 0x35,
            ADD_VEC3: 0x36,
            ADD_VEC3WS_VEC3: 0x37,
            ADD_VEC3_VEC3WS: 0x38,
            ADD_VEC4: 0x39,
            ADD_GAMETIME_FLOAT: 0x3A,
            ADD_FLOAT_GAMETIME: 0x3B,
            SUB_INT: 0x3C,
            SUB_FLOAT: 0x3D,
            SUB_VEC2: 0x3E,
            SUB_VEC3: 0x3F,
            SUB_VEC3WS_VEC3: 0x40,
            SUB_VEC3WS_VEC3WS: 0x41,
            SUB_VEC4: 0x42,
            SUB_GAMETIME_FLOAT: 0x43,
            SUB_GAMETIME: 0x44,
            MUL_INT: 0x45,
            MUL_FLOAT: 0x46,
            DIV_FLOAT: 0x47,
            MOD_INT: 0x48,
            MOD_FLOAT: 0x49,
            LT_INT: 0x4A,
            LT_FLOAT: 0x4B,
            LT_GAMETIME: 0x4C,
            LTE_INT: 0x4D,
            LTE_FLOAT: 0x4E,
            LTE_GAMETIME: 0x4F,
            EQ_BOOL: 0x50,
            EQ_INT: 0x51,
            EQ_FLOAT: 0x52,
            EQ_VEC2: 0x53,
            EQ_VEC3: 0x54,
            EQ_VEC3WS: 0x55,
            EQ_VEC4: 0x56,
            EQ_STRING: 0x57,
            EQ_ENTITY_NAME: 0x58,
            EQ_SCHEMA_ENUM: 0x59,
            EQ_EHANDLE: 0x5A,
            EQ_PANEL_HANDLE: 0x5B,
            EQ_OPAQUE_HANDLE: 0x5C,
            EQ_TEST_HANDLE: 0x5D,
            EQ_COLOR_RGB: 0x5E,
            EQ_ARRAY: 0x5F,
            EQ_GAMETIME: 0x60,
            NE_BOOL: 0x61,
            NE_INT: 0x62,
            NE_FLOAT: 0x63,
            NE_VEC2: 0x64,
            NE_VEC3: 0x65,
            NE_VEC3WS: 0x66,
            NE_VEC4: 0x67,
            NE_STRING: 0x68,
            NE_ENTITY_NAME: 0x69,
            NE_SCHEMA_ENUM: 0x6A,
            NE_EHANDLE: 0x6B,
            NE_PANEL_HANDLE: 0x6C,
            NE_OPAQUE_HANDLE: 0x6D,
            NE_TEST_HANDLE: 0x6E,
            NE_COLOR_RGB: 0x6F,
            NE_ARRAY: 0x70,
            NE_GAMETIME: 0x71,
            SCALE_VEC3: 0x72,
            SCALE_VEC2: 0x73,
            SCALE_VEC4: 0x74,
            SCALE_INV_VEC3: 0x75,
            SCALE_INV_VEC2: 0x76,
            SCALE_INV_VEC4: 0x77,
            ELEMENT_ACCESS_VEC2: 0x78,
            ELEMENT_ACCESS_VEC3: 0x79,
            ELEMENT_ACCESS_VEC3WS: 0x7A,
            ELEMENT_ACCESS_VEC4: 0x7B,
            ELEMENT_ACCESS_COLOR_RGB: 0x7C,
            GET_CONST_INLINE_STORAGE: 0x7D,
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
        CPulseCell_Test_MultiInflow_NoDefault: {
        },
        CPulseCell_Test_MultiOutflow_WithParams_Yielding: {
            m_Out1: 0xD8, // SignatureOutflow_Continue
            m_AsyncChild1: 0x120, // SignatureOutflow_Continue
            m_AsyncChild2: 0x168, // SignatureOutflow_Continue
            m_YieldResume1: 0x1B0, // SignatureOutflow_Resume
            m_YieldResume2: 0x1F8, // SignatureOutflow_Resume
        },
        CPulseGraphInstance_TestDomain_Derived: {
            m_nInstanceValueX: 0x158, // int32
        },
        CPulseGraphInstance_TestDomain: {
            m_bIsRunningUnitTests: 0x128, // bool
            m_bExplicitTimeStepping: 0x129, // bool
            m_bExpectingToDestroyWithYieldedCursors: 0x12A, // bool
            m_bQuietTracepoints: 0x12B, // bool
            m_bExpectingCursorTerminatedDueToMaxInstructions: 0x12C, // bool
            m_nCursorsTerminatedDueToMaxInstructions: 0x130, // int32
            m_nNextValidateIndex: 0x134, // int32
            m_Tracepoints: 0x138, // CUtlVector<CUtlString>
            m_bTestYesOrNoPath: 0x150, // bool
        },
        SignatureOutflow_Continue: {
        },
        CPulseCell_Outflow_TestExplicitYesNo: {
            m_Yes: 0x48, // CPulse_OutflowConnection
            m_No: 0x90, // CPulse_OutflowConnection
            m_Out1: 0xD8, // SignatureOutflow_Continue
            m_AsyncChild1: 0x120, // SignatureOutflow_Continue
        },
        CPulseCell_IsRequirementValid__Criteria_t: {
            m_bIsValid: 0x0, // bool
        },
        CPulseGraphInstance_TurtleGraphics: {
        },
        CPulseCell_TestWaitWithCursorState: {
            m_WakeResume: 0xD8, // CPulse_ResumePoint
            m_WakeFail: 0x120, // CPulse_ResumePoint
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
