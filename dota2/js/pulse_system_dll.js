// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-03-30 13:31:39.632920200 +07:00

export const Schemas = {
    pulse_system_dll: {
        PulseBestOutflowRules_t: {
            SORT_BY_NUMBER_OF_VALID_CRITERIA: 0x0,
            SORT_BY_OUTFLOW_INDEX: 0x1,
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
        PulseVariableKeysSource_t: {
            PRIVATE: 0x0,
            CPP: 0x1,
            VMAP: 0x2,
            VMDL: 0x3,
            XML: 0x4,
            COUNT: 0x5,
        },
        EPulseGraphExecutionHistoryFlag: {
            NO_FLAGS: 0x0,
            CURSOR_ADD_TAG: 0x1,
            CURSOR_REMOVE_TAG: 0x2,
            CURSOR_RETIRED: 0x4,
            REQUIREMENT_PASS: 0x8,
            REQUIREMENT_FAIL: 0x10,
        },
        PulseCursorExecResult_t: {
            Succeeded: 0x0,
            Canceled: 0x1,
            Failed: 0x2,
            OngoingNotify: 0x3,
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
            CELL_INVOKE: 0xB,
            LIBRARY_INVOKE: 0xC,
            SET_VAR: 0xD,
            GET_VAR: 0xE,
            GET_VAR_DETACH: 0xF,
            DETACH_REGISTER: 0x10,
            SET_VAR_ARRAY_ELEMENT_1D: 0x11,
            SET_VAR_OBSERVABLE: 0x12,
            GET_CONST: 0x13,
            GET_ARRAY_ELEMENT: 0x14,
            GET_DOMAIN_VALUE: 0x15,
            COPY: 0x16,
            NOT: 0x17,
            NEGATE: 0x18,
            ADD: 0x19,
            SUB: 0x1A,
            MUL: 0x1B,
            DIV: 0x1C,
            MOD: 0x1D,
            LT: 0x1E,
            LTE: 0x1F,
            EQ: 0x20,
            NE: 0x21,
            AND: 0x22,
            OR: 0x23,
            SCALE: 0x24,
            SCALE_INV: 0x25,
            ELEMENT_ACCESS: 0x26,
            CONVERT_VALUE: 0x27,
            REINTERPRET_INSTANCE: 0x28,
            GET_BLACKBOARD_REFERENCE: 0x29,
            SET_BLACKBOARD_REFERENCE: 0x2A,
            LAST_SERIALIZED_CODE: 0x2B,
            NEGATE_INT: 0x2C,
            NEGATE_FLOAT: 0x2D,
            NEGATE_VEC2: 0x2E,
            NEGATE_VEC3: 0x2F,
            NEGATE_VEC4: 0x30,
            ADD_INT: 0x31,
            ADD_FLOAT: 0x32,
            ADD_STRING: 0x33,
            ADD_VEC2: 0x34,
            ADD_VEC3: 0x35,
            ADD_VEC3WS_VEC3: 0x36,
            ADD_VEC3_VEC3WS: 0x37,
            ADD_VEC4: 0x38,
            ADD_GAMETIME_FLOAT: 0x39,
            ADD_FLOAT_GAMETIME: 0x3A,
            SUB_INT: 0x3B,
            SUB_FLOAT: 0x3C,
            SUB_VEC2: 0x3D,
            SUB_VEC3: 0x3E,
            SUB_VEC3WS_VEC3: 0x3F,
            SUB_VEC3WS_VEC3WS: 0x40,
            SUB_VEC4: 0x41,
            SUB_GAMETIME_FLOAT: 0x42,
            SUB_GAMETIME: 0x43,
            MUL_INT: 0x44,
            MUL_FLOAT: 0x45,
            DIV_FLOAT: 0x46,
            MOD_INT: 0x47,
            MOD_FLOAT: 0x48,
            LT_INT: 0x49,
            LT_FLOAT: 0x4A,
            LT_GAMETIME: 0x4B,
            LTE_INT: 0x4C,
            LTE_FLOAT: 0x4D,
            LTE_GAMETIME: 0x4E,
            EQ_BOOL: 0x4F,
            EQ_INT: 0x50,
            EQ_FLOAT: 0x51,
            EQ_VEC2: 0x52,
            EQ_VEC3: 0x53,
            EQ_VEC3WS: 0x54,
            EQ_VEC4: 0x55,
            EQ_STRING: 0x56,
            EQ_ENTITY_NAME: 0x57,
            EQ_SCHEMA_ENUM: 0x58,
            EQ_EHANDLE: 0x59,
            EQ_PANEL_HANDLE: 0x5A,
            EQ_OPAQUE_HANDLE: 0x5B,
            EQ_TEST_HANDLE: 0x5C,
            EQ_COLOR_RGB: 0x5D,
            EQ_ARRAY: 0x5E,
            EQ_GAMETIME: 0x5F,
            NE_BOOL: 0x60,
            NE_INT: 0x61,
            NE_FLOAT: 0x62,
            NE_VEC2: 0x63,
            NE_VEC3: 0x64,
            NE_VEC3WS: 0x65,
            NE_VEC4: 0x66,
            NE_STRING: 0x67,
            NE_ENTITY_NAME: 0x68,
            NE_SCHEMA_ENUM: 0x69,
            NE_EHANDLE: 0x6A,
            NE_PANEL_HANDLE: 0x6B,
            NE_OPAQUE_HANDLE: 0x6C,
            NE_TEST_HANDLE: 0x6D,
            NE_COLOR_RGB: 0x6E,
            NE_ARRAY: 0x6F,
            NE_GAMETIME: 0x70,
            SCALE_VEC3: 0x71,
            SCALE_VEC2: 0x72,
            SCALE_VEC4: 0x73,
            SCALE_INV_VEC3: 0x74,
            SCALE_INV_VEC2: 0x75,
            SCALE_INV_VEC4: 0x76,
            ELEMENT_ACCESS_VEC2: 0x77,
            ELEMENT_ACCESS_VEC3: 0x78,
            ELEMENT_ACCESS_VEC3WS: 0x79,
            ELEMENT_ACCESS_VEC4: 0x7A,
            ELEMENT_ACCESS_COLOR_RGB: 0x7B,
            GET_CONST_INLINE_STORAGE: 0x7C,
        },
        PulseDomainValueType_t: {
            INVALID: 0xFFFFFFFFFFFFFFFF,
            ENTITY_NAME: 0x0,
            PANEL_ID: 0x1,
            COUNT: 0x2,
        },
        CPulseCell_Step_TestDomainDestroyFakeEntity: {
        },
        CPulseCell_WaitForCursorsWithTag: {
        },
        CPulseCell_Test_NoInflow: {
        },
        CPulseGraphInstance_TestDomain_FakeEntityOwner: {
        },
        CPulseCell_Base: {
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
        CPulse_ResumePoint: {
        },
        CTestDomainDerived_Cursor: {
        },
        CPulseCell_PickBestOutflowSelector: {
        },
        CPulseTestFuncs_LibraryA: {
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
        CPulseCell_WaitForObservable: {
        },
        CPulse_OutflowConnection: {
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
        CPulseGraphDef: {
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
        CPulseGraphInstance_TestDomain_UseReadOnlyBlackboardView: {
        },
        CPulseCell_FireCursors: {
        },
        CPulseCell_Timeline__TimelineEvent_t: {
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
        CPulseCell_IntervalTimer__CursorState_t: {
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
        CPulseCell_BaseRequirement: {
        },
        CPulseCell_BaseState: {
        },
        OutflowWithRequirements_t: {
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
        CPulseCell_IsRequirementValid: {
        },
        CPulseCell_Value_Gradient: {
        },
        CPulseCursorFuncs: {
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
        PulseNodeDynamicOutflows_t__DynamicOutflow_t: {
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
        CPulseCell_Test_MultiOutflow_WithParams: {
        },
        CBasePulseGraphInstance: {
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
        CPulseCell_Inflow_GraphHook: {
        },
        SignatureOutflow_Resume: {
        },
        CPulseCell_Test_MultiOutflow_WithParams_Yielding__CursorState_t: {
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
        CPulseTurtleGraphicsCursor: {
        },
        CPulseCell_TestWaitWithCursorState__CursorState_t: {
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
        CPulseCell_Inflow_BaseEntrypoint: {
        },
        CPulseCell_Test_MultiInflow_NoDefault: {
        },
        CPulseCell_WaitForCursorsWithTagBase: {
        },
        CPulse_InvokeBinding: {
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
        CPulseCell_IntervalTimer: {
        },
        CPulseTestScriptLib: {
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
        CPulseCell_BaseLerp: {
        },
        CPulseCell_Value_TestValue50: {
        },
        CPulseCell_Test_MultiOutflow_WithParams_Yielding: {
        },
        TestComponent_tAPI: {
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
        CPulseCell_Value_Curve: {
        },
        CPulseCell_Inflow_EventHandler: {
        },
        CPulseCell_BaseFlow: {
        },
        CPulseCell_Step_TestDomainTracepoint: {
        },
        CPulseCell_Outflow_CycleShuffled__InstanceState_t: {
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
        CPulseCell_BaseLerp__CursorState_t: {
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
        CPulseGraphInstance_TestDomain_Derived: {
        },
        CPulseCell_WaitForCursorsWithTagBase__CursorState_t: {
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
        CPulseArraylib: {
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
        CPulseGraphInstance_TestDomain: {
        },
        SignatureOutflow_Continue: {
        },
        CPulseCell_Timeline: {
        },
        CPulseCell_Inflow_EntOutputHandler: {
        },
        CPulseCell_Outflow_TestExplicitYesNo: {
        },
        CPulseCell_Outflow_TestRandomYesNo: {
        },
        CPulseCell_Outflow_CycleOrdered__InstanceState_t: {
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
        CPulseCell_LimitCount__InstanceState_t: {
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
        FakeEntity_tAPI: {
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
        CPulseCell_Test_MultiInflow_WithDefault: {
        },
        CPulseCell_Step_DebugLog: {
        },
        CPulseCell_BaseYieldingInflow: {
        },
        PulseNodeDynamicOutflows_t: {
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
        CPulseCell_IsRequirementValid__Criteria_t: {
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
        CPulseCell_Inflow_ObservableVariableListener: {
        },
        CPulseCell_Outflow_CycleOrdered: {
        },
        PulseSelectorOutflowList_t: {
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
        CPulseGraphInstance_TurtleGraphics: {
        },
        CPulseCell_Val_TestDomainGetEntityName: {
        },
        CPulseCell_Inflow_Wait: {
        },
        CPulseCell_TestWaitWithCursorState: {
        },
        CPulseCell_Outflow_CycleShuffled: {
        },
        CPulseCell_Inflow_Method: {
        },
        CPulseCell_BaseValue: {
        },
        CPulseCell_BooleanSwitchState: {
        },
        FakeEntityDerivedB_tAPI: {
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
        CPulseCell_Inflow_Yield: {
        },
        CPulseMathlib: {
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
        CPulseCell_Unknown: {
        },
        CPulseCell_Outflow_CycleRandom: {
        },
        CPulseCell_Step_PublicOutput: {
        },
        CPulseCell_Val_TestDomainFindEntityByName: {
        },
        CPulse_BlackboardReference: {
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
        CPulseCell_Value_RandomInt: {
        },
        CPulseCell_Step_TestDomainEntFire: {
        },
        FakeEntityDerivedA_tAPI: {
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
        CPulseCell_ExampleSelector: {
        },
        CPulse_CallInfo: {
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
        CPulseCell_InlineNodeSkipSelector: {
        },
        CPulseCell_ExampleCriteria__Criteria_t: {
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
        CPulseCell_ExampleCriteria: {
        },
        CPulseCell_LimitCount: {
        },
        CPulseCell_Step_CallExternalMethod: {
        },
        PulseObservableBoolExpression_t: {
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
        CPulseCell_LimitCount__Criteria_t: {
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
        CPulseCell_Step_TestDomainCreateFakeEntity: {
        },
        CPulseCell_CursorQueue: {
        },
        CPulseCell_Value_RandomFloat: {
        },
        CPulseExecCursor: {
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
        PulseRuntimeVarIndex_t: {
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
        PulseRuntimeEntrypointIndex_t: {
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
        PulseRuntimeDomainValueIndex_t: {
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
        CPulse_Chunk: {
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
        PulseRuntimeInvokeIndex_t: {
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
        PulseRuntimeRegisterIndex_t: {
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
        PulseCursorID_t: {
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
        PulseGraphExecutionHistoryCursorDesc_t: {
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
        TestComponent_t: {
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
        PulseRegisterMap_t: {
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
        CPulse_PublicOutput: {
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
        CPulse_OutputConnection: {
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
        PulseGraphExecutionHistoryNodeDesc_t: {
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
        PulseGraphExecutionHistoryEntry_t: {
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
        PulseCursorYieldToken_t: {
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
        CPulseGraphExecutionHistory: {
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
        PGDInstruction_t: {
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
        CPulse_Variable: {
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
        PulseRuntimeOutputIndex_t: {
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
        CPulse_DomainValue: {
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
        PulseRuntimeBlackboardReferenceIndex_t: {
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
        PulseRuntimeConstantIndex_t: {
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
        PulseDocNodeID_t: {
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
        CPulse_RegisterInfo: {
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
        PulseRuntimeCellIndex_t: {
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
        PulseGraphInstanceID_t: {
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
        CPulse_Constant: {
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
        PulseRuntimeCallInfoIndex_t: {
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
        PulseRuntimeChunkIndex_t: {
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
        CPulseRuntimeMethodArg: {
            : 0x10120, // 
            : 0x10740, // 
            : 0x0, // 
            : 0x100000, // 
            : 0x88, // 
            �: 0x1F, // �
            : 0x10018, // 
            : 0x0, // 
            P: 0x102C0, // P
            : 0x1000, // 
            : 0x0, // 
            : 0x8B7F8EA0, // 
            : 0x0, // 
            : 0x0, // 
            P: 0x4, // P
            : 0xE02FA840, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6D0061, // 
            : 0x57005C, // 
            : 0x5C0072, // 
            : 0x360032, // 
            : 0x6C0064, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x360000, // 
            : 0x1, // 
            : 0x0, // 
            : 0x96, // 
            : 0x3, // 
            : 0x10150, // 
            P: 0x310038, // P
            : 0x4E002E, // 
            : 0x55002D, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x20004D, // 
            : 0x10150, // 
            : 0x640064, // 
            : 0x0, // 
            : 0x20009, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x5467122E, // 
            : 0x10150, // 
            �: 0x10060, // �
            : 0x0, // 
        },
        PulseRuntimeStateOffset_t: {
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
        CPulse_InstructionDebug: {
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
    },
};
