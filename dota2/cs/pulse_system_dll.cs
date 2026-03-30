// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-03-30 13:31:39.632920200 +07:00

namespace Source2Dumper.Schemas {
    // Module: pulse_system.dll
    // Class count: 130
    // Enum count: 12
    public static class PulseSystemDll {
        // Alignment: 4
        // Member count: 2
        public enum PulseBestOutflowRules_t : uint {
            SORT_BY_NUMBER_OF_VALID_CRITERIA = 0x0,
            SORT_BY_OUTFLOW_INDEX = 0x1
        }
        // Alignment: 4
        // Member count: 3
        public enum PulseTestEnumShape_t : uint {
            CIRCLE = 0x64,
            SQUARE = 0xC8,
            TRIANGLE = 0x12C
        }
        // Alignment: 4
        // Member count: 4
        public enum PulseCursorCancelPriority_t : uint {
            None = 0x0,
            CancelOnSucceeded = 0x1,
            SoftCancel = 0x2,
            HardCancel = 0x3
        }
        // Alignment: 4
        // Member count: 2
        public enum PulseMethodCallMode_t : uint {
            SYNC_WAIT_FOR_COMPLETION = 0x0,
            ASYNC_FIRE_AND_FORGET = 0x1
        }
        // Alignment: 4
        // Member count: 5
        public enum PulseTestEnumColor_t : uint {
            BLACK = 0x0,
            WHITE = 0x1,
            RED = 0x2,
            GREEN = 0x3,
            BLUE = 0x4
        }
        // Alignment: 4
        // Member count: 6
        public enum PulseVariableKeysSource_t : uint {
            PRIVATE = 0x0,
            CPP = 0x1,
            VMAP = 0x2,
            VMDL = 0x3,
            XML = 0x4,
            COUNT = 0x5
        }
        // Alignment: 4
        // Member count: 6
        public enum EPulseGraphExecutionHistoryFlag : uint {
            NO_FLAGS = 0x0,
            CURSOR_ADD_TAG = 0x1,
            CURSOR_REMOVE_TAG = 0x2,
            CURSOR_RETIRED = 0x4,
            REQUIREMENT_PASS = 0x8,
            REQUIREMENT_FAIL = 0x10
        }
        // Alignment: 4
        // Member count: 4
        public enum PulseCursorExecResult_t : uint {
            Succeeded = 0x0,
            Canceled = 0x1,
            Failed = 0x2,
            OngoingNotify = 0x3
        }
        // Alignment: 4
        // Member count: 33
        public enum PulseValueType_t : uint {
            PVAL_VOID = unchecked((uint)-1),
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
        }
        // Alignment: 4
        // Member count: 6
        public enum PulseApiFeature_t : uint {
            AF_NONE = 0x0,
            AF_ENTITIES = 0x1,
            AF_PANORAMA = 0x2,
            AF_PARTICLES = 0x8,
            AF_FAKE_ENTITIES = 0x10,
            AF_SELECTORS_WITHOUT_REQUIREMENTS = 0x20
        }
        // Alignment: 2
        // Member count: 125
        public enum PulseInstructionCode_t : ushort {
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
        }
        // Alignment: 4
        // Member count: 4
        public enum PulseDomainValueType_t : uint {
            INVALID = unchecked((uint)-1),
            ENTITY_NAME = 0x0,
            PANEL_ID = 0x1,
            COUNT = 0x2
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Step_TestDomainDestroyFakeEntity {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_WaitForCursorsWithTag {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Test_NoInflow {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseGraphInstance_TestDomain_FakeEntityOwner {
        }
        // Parent: None
        // Field count: 48
        public static class CPulseCell_Base {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulse_ResumePoint {
        }
        // Parent: None
        // Field count: 0
        public static class CTestDomainDerived_Cursor {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_PickBestOutflowSelector {
        }
        // Parent: None
        // Field count: 45
        public static class CPulseTestFuncs_LibraryA {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_WaitForObservable {
        }
        // Parent: None
        // Field count: 45
        public static class CPulse_OutflowConnection {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 48
        public static class CPulseGraphDef {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulseGraphInstance_TestDomain_UseReadOnlyBlackboardView {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_FireCursors {
        }
        // Parent: None
        // Field count: 48
        public static class CPulseCell_Timeline__TimelineEvent_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 48
        public static class CPulseCell_IntervalTimer__CursorState_t {
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x50005C; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x660065; // 
            public const nint  = 0x34005C; // 
            public const nint  = 0x70004D; // 
            public const nint  = 0x670069; // 
            public const nint  = 0x610074; // 
            public const nint  = 0x6F0064; // 
            public const nint  = 0x740061; // 
            public const nint  = 0x2E0030; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xE02FB640; // 
            public const nint  = 0x700073; // 
            public const nint  = 0x0; // 
            public const nint  = 0x730069; // 
            public const nint  = 0x720065; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x80; // 
            public const nint  = 0x20004D; // 
            public const nint  = 0x520002; // 
            public const nint  = 0x0; // 
            public const nint  = 0x15; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_BaseRequirement {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_BaseState {
        }
        // Parent: None
        // Field count: 48
        public static class OutflowWithRequirements_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_IsRequirementValid {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Value_Gradient {
        }
        // Parent: None
        // Field count: 45
        public static class CPulseCursorFuncs {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 48
        public static class PulseNodeDynamicOutflows_t__DynamicOutflow_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Test_MultiOutflow_WithParams {
        }
        // Parent: None
        // Field count: 45
        public static class CBasePulseGraphInstance {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_Inflow_GraphHook {
        }
        // Parent: None
        // Field count: 0
        public static class SignatureOutflow_Resume {
        }
        // Parent: None
        // Field count: 48
        public static class CPulseCell_Test_MultiOutflow_WithParams_Yielding__CursorState_t {
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x50005C; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x660065; // 
            public const nint  = 0x34005C; // 
            public const nint  = 0x70004D; // 
            public const nint  = 0x670069; // 
            public const nint  = 0x610074; // 
            public const nint  = 0x6F0064; // 
            public const nint  = 0x740061; // 
            public const nint  = 0x2E0030; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xE02FB640; // 
            public const nint  = 0x700073; // 
            public const nint  = 0x0; // 
            public const nint  = 0x730069; // 
            public const nint  = 0x720065; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x80; // 
            public const nint  = 0x20004D; // 
            public const nint  = 0x520002; // 
            public const nint  = 0x0; // 
            public const nint  = 0x15; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulseTurtleGraphicsCursor {
        }
        // Parent: None
        // Field count: 48
        public static class CPulseCell_TestWaitWithCursorState__CursorState_t {
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x50005C; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x660065; // 
            public const nint  = 0x34005C; // 
            public const nint  = 0x70004D; // 
            public const nint  = 0x670069; // 
            public const nint  = 0x610074; // 
            public const nint  = 0x6F0064; // 
            public const nint  = 0x740061; // 
            public const nint  = 0x2E0030; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xE02FB640; // 
            public const nint  = 0x700073; // 
            public const nint  = 0x0; // 
            public const nint  = 0x730069; // 
            public const nint  = 0x720065; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x80; // 
            public const nint  = 0x20004D; // 
            public const nint  = 0x520002; // 
            public const nint  = 0x0; // 
            public const nint  = 0x15; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Inflow_BaseEntrypoint {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Test_MultiInflow_NoDefault {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_WaitForCursorsWithTagBase {
        }
        // Parent: None
        // Field count: 48
        public static class CPulse_InvokeBinding {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_IntervalTimer {
        }
        // Parent: None
        // Field count: 45
        public static class CPulseTestScriptLib {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_BaseLerp {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Value_TestValue50 {
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_Test_MultiOutflow_WithParams_Yielding {
        }
        // Parent: None
        // Field count: 45
        public static class TestComponent_tAPI {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Value_Curve {
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_Inflow_EventHandler {
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_BaseFlow {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Step_TestDomainTracepoint {
        }
        // Parent: None
        // Field count: 48
        public static class CPulseCell_Outflow_CycleShuffled__InstanceState_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 48
        public static class CPulseCell_BaseLerp__CursorState_t {
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x50005C; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x660065; // 
            public const nint  = 0x34005C; // 
            public const nint  = 0x70004D; // 
            public const nint  = 0x670069; // 
            public const nint  = 0x610074; // 
            public const nint  = 0x6F0064; // 
            public const nint  = 0x740061; // 
            public const nint  = 0x2E0030; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xE02FB640; // 
            public const nint  = 0x700073; // 
            public const nint  = 0x0; // 
            public const nint  = 0x730069; // 
            public const nint  = 0x720065; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x80; // 
            public const nint  = 0x20004D; // 
            public const nint  = 0x520002; // 
            public const nint  = 0x0; // 
            public const nint  = 0x15; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseGraphInstance_TestDomain_Derived {
        }
        // Parent: None
        // Field count: 45
        public static class CPulseCell_WaitForCursorsWithTagBase__CursorState_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 45
        public static class CPulseArraylib {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulseGraphInstance_TestDomain {
        }
        // Parent: None
        // Field count: 0
        public static class SignatureOutflow_Continue {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Timeline {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Inflow_EntOutputHandler {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Outflow_TestExplicitYesNo {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Outflow_TestRandomYesNo {
        }
        // Parent: None
        // Field count: 48
        public static class CPulseCell_Outflow_CycleOrdered__InstanceState_t {
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x50005C; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x660065; // 
            public const nint  = 0x34005C; // 
            public const nint  = 0x70004D; // 
            public const nint  = 0x670069; // 
            public const nint  = 0x610074; // 
            public const nint  = 0x6F0064; // 
            public const nint  = 0x740061; // 
            public const nint  = 0x2E0030; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xE02FB640; // 
            public const nint  = 0x700073; // 
            public const nint  = 0x0; // 
            public const nint  = 0x730069; // 
            public const nint  = 0x720065; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x80; // 
            public const nint  = 0x20004D; // 
            public const nint  = 0x520002; // 
            public const nint  = 0x0; // 
            public const nint  = 0x15; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
        }
        // Parent: None
        // Field count: 48
        public static class CPulseCell_LimitCount__InstanceState_t {
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0xFF00; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x50005C; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x660065; // 
            public const nint  = 0x34005C; // 
            public const nint  = 0x70004D; // 
            public const nint  = 0x670069; // 
            public const nint  = 0x610074; // 
            public const nint  = 0x6F0064; // 
            public const nint  = 0x740061; // 
            public const nint  = 0x2E0030; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xE02FB640; // 
            public const nint  = 0x700073; // 
            public const nint  = 0x0; // 
            public const nint  = 0x730069; // 
            public const nint  = 0x720065; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x80; // 
            public const nint  = 0x20004D; // 
            public const nint  = 0x520002; // 
            public const nint  = 0x0; // 
            public const nint  = 0x15; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
        }
        // Parent: None
        // Field count: 45
        public static class FakeEntity_tAPI {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: pulse_system
        // Field count: 0
        public static class CPulseCell_Test_MultiInflow_WithDefault {
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_Step_DebugLog {
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_BaseYieldingInflow {
        }
        // Parent: None
        // Field count: 48
        public static class PulseNodeDynamicOutflows_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 45
        public static class CPulseCell_IsRequirementValid__Criteria_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Inflow_ObservableVariableListener {
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_Outflow_CycleOrdered {
        }
        // Parent: None
        // Field count: 48
        public static class PulseSelectorOutflowList_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseGraphInstance_TurtleGraphics {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Val_TestDomainGetEntityName {
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_Inflow_Wait {
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_TestWaitWithCursorState {
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_Outflow_CycleShuffled {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Inflow_Method {
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_BaseValue {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_BooleanSwitchState {
        }
        // Parent: None
        // Field count: 45
        public static class FakeEntityDerivedB_tAPI {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_Inflow_Yield {
        }
        // Parent: None
        // Field count: 45
        public static class CPulseMathlib {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Unknown {
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_Outflow_CycleRandom {
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_Step_PublicOutput {
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_Val_TestDomainFindEntityByName {
        }
        // Parent: None
        // Field count: 48
        public static class CPulse_BlackboardReference {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Value_RandomInt {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Step_TestDomainEntFire {
        }
        // Parent: None
        // Field count: 45
        public static class FakeEntityDerivedA_tAPI {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_ExampleSelector {
        }
        // Parent: None
        // Field count: 48
        public static class CPulse_CallInfo {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_InlineNodeSkipSelector {
        }
        // Parent: None
        // Field count: 45
        public static class CPulseCell_ExampleCriteria__Criteria_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_ExampleCriteria {
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_LimitCount {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Step_CallExternalMethod {
        }
        // Parent: None
        // Field count: 48
        public static class PulseObservableBoolExpression_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 45
        public static class CPulseCell_LimitCount__Criteria_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Step_TestDomainCreateFakeEntity {
        }
        // Parent: pulse_runtime_lib
        // Field count: 0
        public static class CPulseCell_CursorQueue {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseCell_Value_RandomFloat {
        }
        // Parent: None
        // Field count: 45
        public static class CPulseExecCursor {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 45
        public static class PulseRuntimeVarIndex_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 45
        public static class PulseRuntimeEntrypointIndex_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 45
        public static class PulseRuntimeDomainValueIndex_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 48
        public static class CPulse_Chunk {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 45
        public static class PulseRuntimeInvokeIndex_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 45
        public static class PulseRuntimeRegisterIndex_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 45
        public static class PulseCursorID_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 48
        public static class PulseGraphExecutionHistoryCursorDesc_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 45
        public static class TestComponent_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 45
        public static class PulseRegisterMap_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 48
        public static class CPulse_PublicOutput {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 48
        public static class CPulse_OutputConnection {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 48
        public static class PulseGraphExecutionHistoryNodeDesc_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 48
        public static class PulseGraphExecutionHistoryEntry_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 45
        public static class PulseCursorYieldToken_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 48
        public static class CPulseGraphExecutionHistory {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 48
        public static class PGDInstruction_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 48
        public static class CPulse_Variable {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 45
        public static class PulseRuntimeOutputIndex_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 48
        public static class CPulse_DomainValue {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 45
        public static class PulseRuntimeBlackboardReferenceIndex_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 45
        public static class PulseRuntimeConstantIndex_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 45
        public static class PulseDocNodeID_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 48
        public static class CPulse_RegisterInfo {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 45
        public static class PulseRuntimeCellIndex_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 45
        public static class PulseGraphInstanceID_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 48
        public static class CPulse_Constant {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 45
        public static class PulseRuntimeCallInfoIndex_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 45
        public static class PulseRuntimeChunkIndex_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 54
        public static class CPulseRuntimeMethodArg {
            public const nint  = 0x10120; // 
            public const nint  = 0x10740; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100000; // 
            public const nint  = 0x88; // 
            public const nint � = 0x1F; // �
            public const nint  = 0x10018; // 
            public const nint  = 0x0; // 
            public const nint P = 0x102C0; // P
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x8B7F8EA0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint P = 0x4; // P
            public const nint  = 0xE02FA840; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x57005C; // 
            public const nint  = 0x5C0072; // 
            public const nint  = 0x360032; // 
            public const nint  = 0x6C0064; // 
            public const nint  = 0x50005C; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x660065; // 
            public const nint  = 0x34005C; // 
            public const nint  = 0x360000; // 
            public const nint  = 0x1; // 
            public const nint  = 0x0; // 
            public const nint  = 0x96; // 
            public const nint  = 0x3; // 
            public const nint  = 0x10150; // 
            public const nint P = 0x310038; // P
            public const nint  = 0x4E002E; // 
            public const nint  = 0x55002D; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x20004D; // 
            public const nint  = 0x10150; // 
            public const nint  = 0x640064; // 
            public const nint  = 0x0; // 
            public const nint  = 0x20009; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5467122E; // 
            public const nint  = 0x10150; // 
            public const nint � = 0x10060; // �
            public const nint  = 0x0; // 
        }
        // Parent: None
        // Field count: 45
        public static class PulseRuntimeStateOffset_t {
            public const nint  = 0x10110FF; // 
            public const nint  = 0x0; // 
            public const nint  = 0x45; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x400; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1032000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x61007400; // 
            public const nint  = 0x6F006400; // 
            public const nint  = 0x74006100; // 
            public const nint  = 0x2E003000; // 
            public const nint  = 0x3A004300; // 
            public const nint  = 0x63006900; // 
            public const nint  = 0x44002000; // 
            public const nint  = 0x6D007200; // 
            public const nint  = 0x5C003000; // 
            public const nint  = 0x66006E00; // 
            public const nint  = 0x61004400; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x6C005000; // 
            public const nint  = 0x32003000; // 
            public const nint  = 0x6E006500; // 
            public const nint  = 0x7FF98B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x1; // 
            public const nint  = 0x69006400; // 
            public const nint  = 0x1015000; // 
            public const nint  = 0x44005C00; // 
            public const nint  = 0x6E006900; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x7700; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4D004D00; // 
            public const nint  = 0x600; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x100F003; // 
            public const nint  = 0x1100000; // 
        }
        // Parent: None
        // Field count: 48
        public static class CPulse_InstructionDebug {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0xE02FA810; // 
            public const nint  = 0x630069; // 
            public const nint  = 0x440020; // 
            public const nint  = 0x6D0072; // 
            public const nint  = 0x5C0030; // 
            public const nint  = 0x6F0072; // 
            public const nint  = 0x6F0073; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x31002E; // 
            public const nint  = 0x61004F; // 
            public const nint  = 0x6C0000; // 
            public const nint  = 0x4D005C; // 
            public const nint  = 0x730077; // 
            public const nint  = 0x6F0066; // 
            public const nint  = 0x2D0036; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0x4B; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
    }
}
