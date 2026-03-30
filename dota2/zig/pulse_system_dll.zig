// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-03-30 13:31:39.632920200 +07:00

pub const source2_dumper = struct {
    pub const schemas = struct {
        // Module: pulse_system.dll
        // Class count: 130
        // Enum count: 12
        pub const pulse_system_dll = struct {
            // Alignment: 4
            // Member count: 2
            pub const PulseBestOutflowRules_t = enum(u32) {
                SORT_BY_NUMBER_OF_VALID_CRITERIA = 0x0,
                SORT_BY_OUTFLOW_INDEX = 0x1
            };
            // Alignment: 4
            // Member count: 3
            pub const PulseTestEnumShape_t = enum(u32) {
                CIRCLE = 0x64,
                SQUARE = 0xC8,
                TRIANGLE = 0x12C
            };
            // Alignment: 4
            // Member count: 4
            pub const PulseCursorCancelPriority_t = enum(u32) {
                None = 0x0,
                CancelOnSucceeded = 0x1,
                SoftCancel = 0x2,
                HardCancel = 0x3
            };
            // Alignment: 4
            // Member count: 2
            pub const PulseMethodCallMode_t = enum(u32) {
                SYNC_WAIT_FOR_COMPLETION = 0x0,
                ASYNC_FIRE_AND_FORGET = 0x1
            };
            // Alignment: 4
            // Member count: 5
            pub const PulseTestEnumColor_t = enum(u32) {
                BLACK = 0x0,
                WHITE = 0x1,
                RED = 0x2,
                GREEN = 0x3,
                BLUE = 0x4
            };
            // Alignment: 4
            // Member count: 6
            pub const PulseVariableKeysSource_t = enum(u32) {
                PRIVATE = 0x0,
                CPP = 0x1,
                VMAP = 0x2,
                VMDL = 0x3,
                XML = 0x4,
                COUNT = 0x5
            };
            // Alignment: 4
            // Member count: 6
            pub const EPulseGraphExecutionHistoryFlag = enum(u32) {
                NO_FLAGS = 0x0,
                CURSOR_ADD_TAG = 0x1,
                CURSOR_REMOVE_TAG = 0x2,
                CURSOR_RETIRED = 0x4,
                REQUIREMENT_PASS = 0x8,
                REQUIREMENT_FAIL = 0x10
            };
            // Alignment: 4
            // Member count: 4
            pub const PulseCursorExecResult_t = enum(u32) {
                Succeeded = 0x0,
                Canceled = 0x1,
                Failed = 0x2,
                OngoingNotify = 0x3
            };
            // Alignment: 4
            // Member count: 33
            pub const PulseValueType_t = enum(u32) {
                PVAL_VOID = 0xFFFFFFFF,
                PVAL_BOOL = 0x0,
                PVAL_INT = 0x1,
                PVAL_FLOAT = 0x2,
                PVAL_STRING = 0x3,
                PVAL_VEC2 = 0x4,
                PVAL_VEC3 = 0x5,
                PVAL_QANGLE = 0x6,
                PVAL_VEC3_WORLDSPACE = 0x7,
                PVAL_VEC4 = 0x8,
                PVAL_TRANSFORM = 0x9,
                PVAL_TRANSFORM_WORLDSPACE = 0xA,
                PVAL_COLOR_RGB = 0xB,
                PVAL_GAMETIME = 0xC,
                PVAL_EHANDLE = 0xD,
                PVAL_RESOURCE = 0xE,
                PVAL_RESOURCE_NAME = 0xF,
                PVAL_SNDEVT_GUID = 0x10,
                PVAL_SNDEVT_NAME = 0x11,
                PVAL_ENTITY_NAME = 0x12,
                PVAL_OPAQUE_HANDLE = 0x13,
                PVAL_TYPESAFE_INT = 0x14,
                PVAL_MODEL_MATERIAL_GROUP = 0x15,
                PVAL_CURSOR_FLOW = 0x16,
                PVAL_VARIANT = 0x17,
                PVAL_UNKNOWN = 0x18,
                PVAL_SCHEMA_ENUM = 0x19,
                PVAL_PANORAMA_PANEL_HANDLE = 0x1A,
                PVAL_TEST_HANDLE = 0x1B,
                PVAL_ARRAY = 0x1C,
                PVAL_TYPESAFE_INT64 = 0x1D,
                PVAL_PARTICLE_EHANDLE = 0x1E,
                PVAL_COUNT = 0x1F
            };
            // Alignment: 4
            // Member count: 6
            pub const PulseApiFeature_t = enum(u32) {
                AF_NONE = 0x0,
                AF_ENTITIES = 0x1,
                AF_PANORAMA = 0x2,
                AF_PARTICLES = 0x8,
                AF_FAKE_ENTITIES = 0x10,
                AF_SELECTORS_WITHOUT_REQUIREMENTS = 0x20
            };
            // Alignment: 2
            // Member count: 125
            pub const PulseInstructionCode_t = enum(u16) {
                INVALID = 0x0,
                IMMEDIATE_HALT = 0x1,
                RETURN_VOID = 0x2,
                RETURN_VALUE = 0x3,
                NOP = 0x4,
                JUMP = 0x5,
                JUMP_COND = 0x6,
                CHUNK_LEAP = 0x7,
                CHUNK_LEAP_COND = 0x8,
                PULSE_CALL_SYNC = 0x9,
                PULSE_CALL_ASYNC_FIRE = 0xA,
                CELL_INVOKE = 0xB,
                LIBRARY_INVOKE = 0xC,
                SET_VAR = 0xD,
                GET_VAR = 0xE,
                GET_VAR_DETACH = 0xF,
                DETACH_REGISTER = 0x10,
                SET_VAR_ARRAY_ELEMENT_1D = 0x11,
                SET_VAR_OBSERVABLE = 0x12,
                GET_CONST = 0x13,
                GET_ARRAY_ELEMENT = 0x14,
                GET_DOMAIN_VALUE = 0x15,
                COPY = 0x16,
                NOT = 0x17,
                NEGATE = 0x18,
                ADD = 0x19,
                SUB = 0x1A,
                MUL = 0x1B,
                DIV = 0x1C,
                MOD = 0x1D,
                LT = 0x1E,
                LTE = 0x1F,
                EQ = 0x20,
                NE = 0x21,
                AND = 0x22,
                OR = 0x23,
                SCALE = 0x24,
                SCALE_INV = 0x25,
                ELEMENT_ACCESS = 0x26,
                CONVERT_VALUE = 0x27,
                REINTERPRET_INSTANCE = 0x28,
                GET_BLACKBOARD_REFERENCE = 0x29,
                SET_BLACKBOARD_REFERENCE = 0x2A,
                LAST_SERIALIZED_CODE = 0x2B,
                NEGATE_INT = 0x2C,
                NEGATE_FLOAT = 0x2D,
                NEGATE_VEC2 = 0x2E,
                NEGATE_VEC3 = 0x2F,
                NEGATE_VEC4 = 0x30,
                ADD_INT = 0x31,
                ADD_FLOAT = 0x32,
                ADD_STRING = 0x33,
                ADD_VEC2 = 0x34,
                ADD_VEC3 = 0x35,
                ADD_VEC3WS_VEC3 = 0x36,
                ADD_VEC3_VEC3WS = 0x37,
                ADD_VEC4 = 0x38,
                ADD_GAMETIME_FLOAT = 0x39,
                ADD_FLOAT_GAMETIME = 0x3A,
                SUB_INT = 0x3B,
                SUB_FLOAT = 0x3C,
                SUB_VEC2 = 0x3D,
                SUB_VEC3 = 0x3E,
                SUB_VEC3WS_VEC3 = 0x3F,
                SUB_VEC3WS_VEC3WS = 0x40,
                SUB_VEC4 = 0x41,
                SUB_GAMETIME_FLOAT = 0x42,
                SUB_GAMETIME = 0x43,
                MUL_INT = 0x44,
                MUL_FLOAT = 0x45,
                DIV_FLOAT = 0x46,
                MOD_INT = 0x47,
                MOD_FLOAT = 0x48,
                LT_INT = 0x49,
                LT_FLOAT = 0x4A,
                LT_GAMETIME = 0x4B,
                LTE_INT = 0x4C,
                LTE_FLOAT = 0x4D,
                LTE_GAMETIME = 0x4E,
                EQ_BOOL = 0x4F,
                EQ_INT = 0x50,
                EQ_FLOAT = 0x51,
                EQ_VEC2 = 0x52,
                EQ_VEC3 = 0x53,
                EQ_VEC3WS = 0x54,
                EQ_VEC4 = 0x55,
                EQ_STRING = 0x56,
                EQ_ENTITY_NAME = 0x57,
                EQ_SCHEMA_ENUM = 0x58,
                EQ_EHANDLE = 0x59,
                EQ_PANEL_HANDLE = 0x5A,
                EQ_OPAQUE_HANDLE = 0x5B,
                EQ_TEST_HANDLE = 0x5C,
                EQ_COLOR_RGB = 0x5D,
                EQ_ARRAY = 0x5E,
                EQ_GAMETIME = 0x5F,
                NE_BOOL = 0x60,
                NE_INT = 0x61,
                NE_FLOAT = 0x62,
                NE_VEC2 = 0x63,
                NE_VEC3 = 0x64,
                NE_VEC3WS = 0x65,
                NE_VEC4 = 0x66,
                NE_STRING = 0x67,
                NE_ENTITY_NAME = 0x68,
                NE_SCHEMA_ENUM = 0x69,
                NE_EHANDLE = 0x6A,
                NE_PANEL_HANDLE = 0x6B,
                NE_OPAQUE_HANDLE = 0x6C,
                NE_TEST_HANDLE = 0x6D,
                NE_COLOR_RGB = 0x6E,
                NE_ARRAY = 0x6F,
                NE_GAMETIME = 0x70,
                SCALE_VEC3 = 0x71,
                SCALE_VEC2 = 0x72,
                SCALE_VEC4 = 0x73,
                SCALE_INV_VEC3 = 0x74,
                SCALE_INV_VEC2 = 0x75,
                SCALE_INV_VEC4 = 0x76,
                ELEMENT_ACCESS_VEC2 = 0x77,
                ELEMENT_ACCESS_VEC3 = 0x78,
                ELEMENT_ACCESS_VEC3WS = 0x79,
                ELEMENT_ACCESS_VEC4 = 0x7A,
                ELEMENT_ACCESS_COLOR_RGB = 0x7B,
                GET_CONST_INLINE_STORAGE = 0x7C
            };
            // Alignment: 4
            // Member count: 4
            pub const PulseDomainValueType_t = enum(u32) {
                INVALID = 0xFFFFFFFF,
                ENTITY_NAME = 0x0,
                PANEL_ID = 0x1,
                COUNT = 0x2
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Step_TestDomainDestroyFakeEntity = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_WaitForCursorsWithTag = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Test_NoInflow = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseGraphInstance_TestDomain_FakeEntityOwner = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CPulseCell_Base = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPulse_ResumePoint = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CTestDomainDerived_Cursor = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_PickBestOutflowSelector = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CPulseTestFuncs_LibraryA = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_WaitForObservable = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CPulse_OutflowConnection = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CPulseGraphDef = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPulseGraphInstance_TestDomain_UseReadOnlyBlackboardView = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_FireCursors = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CPulseCell_Timeline__TimelineEvent_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CPulseCell_IntervalTimer__CursorState_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_BaseRequirement = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_BaseState = struct {
            };
            // Parent: None
            // Field count: 48
            pub const OutflowWithRequirements_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_IsRequirementValid = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Value_Gradient = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CPulseCursorFuncs = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const PulseNodeDynamicOutflows_t__DynamicOutflow_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Test_MultiOutflow_WithParams = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CBasePulseGraphInstance = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_Inflow_GraphHook = struct {
            };
            // Parent: None
            // Field count: 0
            pub const SignatureOutflow_Resume = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CPulseCell_Test_MultiOutflow_WithParams_Yielding__CursorState_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPulseTurtleGraphicsCursor = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CPulseCell_TestWaitWithCursorState__CursorState_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Inflow_BaseEntrypoint = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Test_MultiInflow_NoDefault = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_WaitForCursorsWithTagBase = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CPulse_InvokeBinding = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_IntervalTimer = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CPulseTestScriptLib = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_BaseLerp = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Value_TestValue50 = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_Test_MultiOutflow_WithParams_Yielding = struct {
            };
            // Parent: None
            // Field count: 45
            pub const TestComponent_tAPI = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Value_Curve = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_Inflow_EventHandler = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_BaseFlow = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Step_TestDomainTracepoint = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CPulseCell_Outflow_CycleShuffled__InstanceState_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CPulseCell_BaseLerp__CursorState_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseGraphInstance_TestDomain_Derived = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CPulseCell_WaitForCursorsWithTagBase__CursorState_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 45
            pub const CPulseArraylib = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPulseGraphInstance_TestDomain = struct {
            };
            // Parent: None
            // Field count: 0
            pub const SignatureOutflow_Continue = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Timeline = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Inflow_EntOutputHandler = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Outflow_TestExplicitYesNo = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Outflow_TestRandomYesNo = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CPulseCell_Outflow_CycleOrdered__InstanceState_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CPulseCell_LimitCount__InstanceState_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 45
            pub const FakeEntity_tAPI = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: pulse_system
            // Field count: 0
            pub const CPulseCell_Test_MultiInflow_WithDefault = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_Step_DebugLog = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_BaseYieldingInflow = struct {
            };
            // Parent: None
            // Field count: 48
            pub const PulseNodeDynamicOutflows_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 45
            pub const CPulseCell_IsRequirementValid__Criteria_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Inflow_ObservableVariableListener = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_Outflow_CycleOrdered = struct {
            };
            // Parent: None
            // Field count: 48
            pub const PulseSelectorOutflowList_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseGraphInstance_TurtleGraphics = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Val_TestDomainGetEntityName = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_Inflow_Wait = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_TestWaitWithCursorState = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_Outflow_CycleShuffled = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Inflow_Method = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_BaseValue = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_BooleanSwitchState = struct {
            };
            // Parent: None
            // Field count: 45
            pub const FakeEntityDerivedB_tAPI = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_Inflow_Yield = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CPulseMathlib = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Unknown = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_Outflow_CycleRandom = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_Step_PublicOutput = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_Val_TestDomainFindEntityByName = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CPulse_BlackboardReference = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Value_RandomInt = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Step_TestDomainEntFire = struct {
            };
            // Parent: None
            // Field count: 45
            pub const FakeEntityDerivedA_tAPI = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_ExampleSelector = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CPulse_CallInfo = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_InlineNodeSkipSelector = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CPulseCell_ExampleCriteria__Criteria_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_ExampleCriteria = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_LimitCount = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Step_CallExternalMethod = struct {
            };
            // Parent: None
            // Field count: 48
            pub const PulseObservableBoolExpression_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 45
            pub const CPulseCell_LimitCount__Criteria_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Step_TestDomainCreateFakeEntity = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_CursorQueue = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Value_RandomFloat = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CPulseExecCursor = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 45
            pub const PulseRuntimeVarIndex_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 45
            pub const PulseRuntimeEntrypointIndex_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 45
            pub const PulseRuntimeDomainValueIndex_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CPulse_Chunk = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 45
            pub const PulseRuntimeInvokeIndex_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 45
            pub const PulseRuntimeRegisterIndex_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 45
            pub const PulseCursorID_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const PulseGraphExecutionHistoryCursorDesc_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 45
            pub const TestComponent_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 45
            pub const PulseRegisterMap_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CPulse_PublicOutput = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CPulse_OutputConnection = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const PulseGraphExecutionHistoryNodeDesc_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const PulseGraphExecutionHistoryEntry_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 45
            pub const PulseCursorYieldToken_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CPulseGraphExecutionHistory = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const PGDInstruction_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CPulse_Variable = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 45
            pub const PulseRuntimeOutputIndex_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CPulse_DomainValue = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 45
            pub const PulseRuntimeBlackboardReferenceIndex_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 45
            pub const PulseRuntimeConstantIndex_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 45
            pub const PulseDocNodeID_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CPulse_RegisterInfo = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 45
            pub const PulseRuntimeCellIndex_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 45
            pub const PulseGraphInstanceID_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CPulse_Constant = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 45
            pub const PulseRuntimeCallInfoIndex_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 45
            pub const PulseRuntimeChunkIndex_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 54
            pub const CPulseRuntimeMethodArg = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 45
            pub const PulseRuntimeStateOffset_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CPulse_InstructionDebug = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
        };
    };
};
