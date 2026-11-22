// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-11-22 15:00:19.948492600 +07:00

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
            BROADCAST_INVOKE: 0xF,
            SET_VAR: 0x10,
            GET_VAR: 0x11,
            GET_VAR_DETACH: 0x12,
            DETACH_REGISTER: 0x13,
            SET_VAR_ARRAY_ELEMENT_1D: 0x14,
            SET_VAR_OBSERVABLE: 0x15,
            GET_CONST: 0x16,
            GET_ARRAY_ELEMENT: 0x17,
            GET_DOMAIN_VALUE: 0x18,
            COPY: 0x19,
            NOT: 0x1A,
            NEGATE: 0x1B,
            ADD: 0x1C,
            SUB: 0x1D,
            MUL: 0x1E,
            DIV: 0x1F,
            MOD: 0x20,
            LT: 0x21,
            LTE: 0x22,
            EQ: 0x23,
            NE: 0x24,
            AND: 0x25,
            OR: 0x26,
            SCALE: 0x27,
            SCALE_INV: 0x28,
            ELEMENT_ACCESS: 0x29,
            CONVERT_VALUE: 0x2A,
            REINTERPRET_INSTANCE: 0x2B,
            GET_BLACKBOARD_REFERENCE: 0x2C,
            SET_BLACKBOARD_REFERENCE: 0x2D,
            GET_TEMPVAR: 0x2E,
            SET_TEMPVAR: 0x2F,
            SET_TEMPVAR_OBSERVABLE: 0x30,
            BIND_LAMBDA: 0x31,
            LAST_SERIALIZED_CODE: 0x32,
            NEGATE_INT: 0x33,
            NEGATE_FLOAT: 0x34,
            NEGATE_VEC2: 0x35,
            NEGATE_VEC3: 0x36,
            NEGATE_VEC4: 0x37,
            ADD_INT: 0x38,
            ADD_FLOAT: 0x39,
            ADD_STRING: 0x3A,
            ADD_VEC2: 0x3B,
            ADD_VEC3: 0x3C,
            ADD_VEC3WS_VEC3: 0x3D,
            ADD_VEC3_VEC3WS: 0x3E,
            ADD_VEC4: 0x3F,
            ADD_GAMETIME_FLOAT: 0x40,
            ADD_FLOAT_GAMETIME: 0x41,
            SUB_INT: 0x42,
            SUB_FLOAT: 0x43,
            SUB_VEC2: 0x44,
            SUB_VEC3: 0x45,
            SUB_VEC3WS_VEC3: 0x46,
            SUB_VEC3WS_VEC3WS: 0x47,
            SUB_VEC4: 0x48,
            SUB_GAMETIME_FLOAT: 0x49,
            SUB_GAMETIME: 0x4A,
            MUL_INT: 0x4B,
            MUL_FLOAT: 0x4C,
            DIV_FLOAT: 0x4D,
            MOD_INT: 0x4E,
            MOD_FLOAT: 0x4F,
            LT_INT: 0x50,
            LT_FLOAT: 0x51,
            LT_GAMETIME: 0x52,
            LTE_INT: 0x53,
            LTE_FLOAT: 0x54,
            LTE_GAMETIME: 0x55,
            EQ_BOOL: 0x56,
            NE_BOOL: 0x57,
            EQ_INT: 0x58,
            NE_INT: 0x59,
            EQ_FLOAT: 0x5A,
            NE_FLOAT: 0x5B,
            EQ_STRING: 0x5C,
            NE_STRING: 0x5D,
            EQ_VEC2: 0x5E,
            NE_VEC2: 0x5F,
            EQ_VEC3: 0x60,
            NE_VEC3: 0x61,
            EQ_QANGLE: 0x62,
            NE_QANGLE: 0x63,
            EQ_VEC3WS: 0x64,
            NE_VEC3WS: 0x65,
            EQ_VEC4: 0x66,
            NE_VEC4: 0x67,
            EQ_TRANSFORM: 0x68,
            NE_TRANSFORM: 0x69,
            EQ_TRANSFORMWS: 0x6A,
            NE_TRANSFORMWS: 0x6B,
            EQ_COLOR_RGB: 0x6C,
            NE_COLOR_RGB: 0x6D,
            EQ_GAMETIME: 0x6E,
            NE_GAMETIME: 0x6F,
            EQ_EHANDLE: 0x70,
            NE_EHANDLE: 0x71,
            EQ_RESOURCE: 0x72,
            NE_RESOURCE: 0x73,
            EQ_RESOURCE_NAME: 0x74,
            NE_RESOURCE_NAME: 0x75,
            EQ_SNDEVT_GUID: 0x76,
            NE_SNDEVT_GUID: 0x77,
            EQ_SNDEVT_NAME: 0x78,
            NE_SNDEVT_NAME: 0x79,
            EQ_ENTITY_NAME: 0x7A,
            NE_ENTITY_NAME: 0x7B,
            EQ_LEAF: 0x7C,
            NE_LEAF: 0x7D,
            EQ_MODEL_MATERIAL_GROUP: 0x7E,
            NE_MODEL_MATERIAL_GROUP: 0x7F,
            EQ_VARIANT: 0x80,
            NE_VARIANT: 0x81,
            EQ_ENUM: 0x82,
            NE_ENUM: 0x83,
            EQ_PANEL_HANDLE: 0x84,
            NE_PANEL_HANDLE: 0x85,
            EQ_ARRAY: 0x86,
            NE_ARRAY: 0x87,
            EQ_PARTICLE_EHANDLE: 0x88,
            NE_PARTICLE_EHANDLE: 0x89,
            EQ_ANIM_SEQUENCE: 0x8A,
            NE_ANIM_SEQUENCE: 0x8B,
            EQ_VDATA_CHOICE: 0x8C,
            NE_VDATA_CHOICE: 0x8D,
            SCALE_VEC3: 0x8E,
            SCALE_VEC2: 0x8F,
            SCALE_VEC4: 0x90,
            SCALE_INV_VEC3: 0x91,
            SCALE_INV_VEC2: 0x92,
            SCALE_INV_VEC4: 0x93,
            ELEMENT_ACCESS_VEC2: 0x94,
            ELEMENT_ACCESS_VEC3: 0x95,
            ELEMENT_ACCESS_VEC3WS: 0x96,
            ELEMENT_ACCESS_VEC4: 0x97,
            ELEMENT_ACCESS_COLOR_RGB: 0x98,
            GET_CONST_INLINE_STORAGE: 0x99,
            BROADCAST_INVOKE_DOMAIN: 0x9A,
            BROADCAST_INVOKE_ARRAY_SIMPLE: 0x9B,
            BROADCAST_INVOKE_ARRAY_DOMAIN: 0x9C,
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
            VMAP_AND_VDATA: 0x6,
            COUNT: 0x7,
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
            INTERNAL_CALL_TO_PULSE: 0x80,
            RETURN: 0x100,
            RECORD_NEXT_SEQ_POINT: 0x200,
            CURSOR_MAX_INSTRUCTIONS: 0x400,
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
            PVAL_LAMBDA: 0x1E,
            PVAL_COUNT: 0x1F,
        },
        PulseApiFeature_t: {
            AF_NONE: 0x0,
            AF_ENTITIES: 0x1,
            AF_PANORAMA: 0x2,
            AF_PARTICLES: 0x8,
            AF_FAKE_ENTITIES: 0x10,
            AF_SELECTORS_WITHOUT_REQUIREMENTS: 0x20,
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
        CPulseCell_TestYieldWithObservables__CursorState_t: {
            m_LiveFloatValue: 0x0, // HPulseObservableLambda<float32>
            m_LiveStringValue: 0xC, // HPulseObservableLambda<CUtlString>
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
