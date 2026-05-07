// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-05-07 15:06:58.093220400 +07:00

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
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseSignatureForOutflow
        // MPulseSignatureForOutflow
        // CIRCLE
        // SQUARE
        // TRIANGLE
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_Step_TestDomainDestroyFakeEntity {
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // MPropertyDescription
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyDescription
        public static class CPulseCell_WaitForCursorsWithTag {
            public const nint m_bTagSelfWhenComplete = 0x98; // bool
            public const nint m_nDesiredKillPriority = 0x9C; // PulseCursorCancelPriority_t
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseSignatureForOutflow
        // MPulseSignatureForOutflow
        // CIRCLE
        // SQUARE
        // TRIANGLE
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_Test_NoInflow {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseGraphInstance_TestDomain_FakeEntityOwner {
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_Base {
            public const nint m_nEditorNodeID = 0x8; // PulseDocNodeID_t
        }
        // Parent: None
        // Field count: 0
        public static class CPulse_ResumePoint {
        }
        // Parent: None
        // Field count: 2
        public static class CTestDomainDerived_Cursor {
            public const nint m_nCursorValueA = 0xD8; // int32
            public const nint m_nCursorValueB = 0xDC; // int32
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // SORT_BY_OUTFLOW_INDEX
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_PickBestOutflowSelector {
            public const nint m_nCheckType = 0x48; // PulseBestOutflowRules_t
            public const nint m_OutflowList = 0x50; // PulseSelectorOutflowList_t
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // MPropertyDescription
        // MPulseExpressionAlias
        // MPulseLegacyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseLegacyName
        // MPropertyDescription
        // MPropertyLeafSuggestionProviderFn
        // MPulseExpressionAlias
        // 2
        public static class CPulseTestFuncs_LibraryA {
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPulseEditorHeaderIcon
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyDescription
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // MPropertyDescription
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyDescription
        public static class CPulseCell_WaitForObservable {
            public const nint m_Condition = 0x48; // PulseObservableBoolExpression_t
            public const nint m_OnTrue = 0xC0; // CPulse_ResumePoint
        }
        // Parent: None
        // Field count: 4
        public static class CPulse_OutflowConnection {
            public const nint m_SourceOutflowName = 0x0; // PulseSymbol_t
            public const nint m_nDestChunk = 0x10; // PulseRuntimeChunkIndex_t
            public const nint m_nInstruction = 0x14; // int32
            public const nint m_OutflowRegisterMap = 0x18; // PulseRegisterMap_t
        }
        // Parent: None
        // Field count: 14
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CPulseGraphDef {
            public const nint m_DomainIdentifier = 0x8; // PulseSymbol_t
            public const nint m_DomainSubType = 0x18; // CPulseValueFullType
            public const nint m_ParentMapName = 0x30; // PulseSymbol_t
            public const nint m_ParentXmlName = 0x40; // PulseSymbol_t
            public const nint m_Chunks = 0x50; // CUtlVector<CPulse_Chunk*>
            public const nint m_Cells = 0x68; // CUtlVector<CPulseCell_Base*>
            public const nint m_Vars = 0x80; // CUtlVector<CPulse_Variable>
            public const nint m_PublicOutputs = 0x98; // CUtlVector<CPulse_PublicOutput>
            public const nint m_InvokeBindings = 0xB0; // CUtlVector<CPulse_InvokeBinding*>
            public const nint m_CallInfos = 0xC8; // CUtlVector<CPulse_CallInfo*>
            public const nint m_Constants = 0xE0; // CUtlVector<CPulse_Constant>
            public const nint m_DomainValues = 0xF8; // CUtlVector<CPulse_DomainValue>
            public const nint m_BlackboardReferences = 0x110; // CUtlVector<CPulse_BlackboardReference>
            public const nint m_OutputConnections = 0x128; // CUtlVector<CPulse_OutputConnection*>
        }
        // Parent: None
        // Field count: 0
        public static class CPulseGraphInstance_TestDomain_UseReadOnlyBlackboardView {
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyDescription
        // MPulseSignatureForOutflow
        // MPropertyDescription
        // MPropertyDescription
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyDescription
        // MPropertyDescription
        public static class CPulseCell_FireCursors {
            public const nint m_Outflows = 0x48; // CUtlVector<CPulse_OutflowConnection>
            public const nint m_bWaitForChildOutflows = 0x60; // bool
            public const nint m_OnFinished = 0x68; // CPulse_ResumePoint
            public const nint m_OnCanceled = 0xB0; // CPulse_ResumePoint
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CPulseCell_Timeline__TimelineEvent_t {
            public const nint m_flTimeFromPrevious = 0x0; // float32
            public const nint m_EventOutflow = 0x8; // CPulse_OutflowConnection
        }
        // Parent: None
        // Field count: 5
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CPulseCell_IntervalTimer__CursorState_t {
            public const nint m_StartTime = 0x0; // GameTime_t
            public const nint m_EndTime = 0x4; // GameTime_t
            public const nint m_flWaitInterval = 0x8; // float32
            public const nint m_flWaitIntervalHigh = 0xC; // float32
            public const nint m_bCompleteOnNextWake = 0x10; // bool
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_BaseRequirement {
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyDescription
        // MPropertyDescription
        // MPropertyDescription
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyDescription
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPulseEditorCanvasItemSpecKV3
        // null
        // int
        // double
        // binary_blob
        // table
        public static class CPulseCell_BaseState {
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class OutflowWithRequirements_t {
            public const nint m_Connection = 0x0; // CPulse_OutflowConnection
            public const nint m_DestinationFlowNodeID = 0x48; // PulseDocNodeID_t
            public const nint m_RequirementNodeIDs = 0x50; // CUtlVector<PulseDocNodeID_t>
            public const nint m_nCursorStateBlockIndex = 0x68; // CUtlVector<int32>
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // SORT_BY_OUTFLOW_INDEX
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_IsRequirementValid {
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyDescription
        // MPulseSignatureForOutflow
        // MPropertyDescription
        // MPropertyDescription
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyDescription
        // MPropertyDescription
        // MPropertyDescription
        // MPropertyFriendlyName
        public static class CPulseCell_Value_Gradient {
            public const nint m_Gradient = 0x48; // CColorGradient
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MPropertyDescription
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // MPropertyDescription
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyDescription
        public static class CPulseCursorFuncs {
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class PulseNodeDynamicOutflows_t__DynamicOutflow_t {
            public const nint m_OutflowID = 0x0; // CGlobalSymbol
            public const nint m_Connection = 0x8; // CPulse_OutflowConnection
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyFriendlyName
        // MPulseSignatureForOutflow
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseSignatureForOutflow
        // MPulseSignatureForOutflow
        // CIRCLE
        // SQUARE
        // TRIANGLE
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_Test_MultiOutflow_WithParams {
            public const nint m_Out1 = 0x48; // SignatureOutflow_Continue
            public const nint m_Out2 = 0x90; // SignatureOutflow_Continue
        }
        // Parent: None
        // Field count: 0
        public static class CBasePulseGraphInstance {
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // ;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0\;C:\Windows\System32\OpenSSH\;C:\Program Files (x86)\Window
        // MGetKV3ClassDefaults
        public static class CPulseCell_Inflow_GraphHook {
            public const nint m_HookName = 0x80; // PulseSymbol_t
        }
        // Parent: None
        // Field count: 0
        public static class SignatureOutflow_Resume {
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPulseEditorHeaderText
        public static class CPulseCell_Test_MultiOutflow_WithParams_Yielding__CursorState_t {
            public const nint nTestStep = 0x0; // int32
        }
        // Parent: None
        // Field count: 4
        public static class CPulseTurtleGraphicsCursor {
            public const nint m_Color = 0xD8; // Color
            public const nint m_vPos = 0xDC; // Vector2D
            public const nint m_flHeadingDeg = 0xE4; // float32
            public const nint m_bPenUp = 0xE8; // bool
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CPulseCell_TestWaitWithCursorState__CursorState_t {
            public const nint flWaitValue = 0x0; // float32
            public const nint bFailOnCancel = 0x4; // bool
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_Inflow_BaseEntrypoint {
            public const nint m_EntryChunk = 0x48; // PulseRuntimeChunkIndex_t
            public const nint m_RegisterMap = 0x50; // PulseRegisterMap_t
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // WHITE
        // RED
        // GREEN
        // BLUE
        // MPulseSignatureForOutflow
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyFriendlyName
        // MPulseSignatureForOutflow
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseSignatureForOutflow
        // MPulseSignatureForOutflow
        // CIRCLE
        // SQUARE
        // TRIANGLE
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_Test_MultiInflow_NoDefault {
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPulseEditorCanvasItemSpecKV3
        // null
        // int
        // double
        // binary_blob
        // table
        public static class CPulseCell_WaitForCursorsWithTagBase {
            public const nint m_nCursorsAllowedToWait = 0x48; // int32
            public const nint m_WaitComplete = 0x50; // CPulse_ResumePoint
        }
        // Parent: None
        // Field count: 5
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CPulse_InvokeBinding {
            public const nint m_RegisterMap = 0x0; // PulseRegisterMap_t
            public const nint m_FuncName = 0x30; // PulseSymbol_t
            public const nint m_nCellIndex = 0x40; // PulseRuntimeCellIndex_t
            public const nint m_nSrcChunk = 0x44; // PulseRuntimeChunkIndex_t
            public const nint m_nSrcInstruction = 0x48; // int32
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorCanvasItemSpecKV3
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPulseEditorHeaderIcon
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyDescription
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyDescription
        public static class CPulseCell_IntervalTimer {
            public const nint m_Completed = 0x48; // CPulse_ResumePoint
            public const nint m_OnInterval = 0x90; // SignatureOutflow_Continue
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MPropertyDescription
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MGetKV3ClassDefaults
        public static class CPulseTestScriptLib {
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyFriendlyName
        // MPropertyDescription
        public static class CPulseCell_BaseLerp {
            public const nint m_WakeResume = 0x48; // CPulse_ResumePoint
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseSignatureForOutflow
        // MPulseSignatureForOutflow
        // CIRCLE
        // SQUARE
        // TRIANGLE
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_Value_TestValue50 {
        }
        // Parent: None
        // Field count: 5
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPulseSignatureForOutflow
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseSignatureForOutflow
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPulseEditorHeaderText
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class CPulseCell_Test_MultiOutflow_WithParams_Yielding {
            public const nint m_Out1 = 0x48; // SignatureOutflow_Continue
            public const nint m_AsyncChild1 = 0x90; // SignatureOutflow_Continue
            public const nint m_AsyncChild2 = 0xD8; // SignatureOutflow_Continue
            public const nint m_YieldResume1 = 0x120; // SignatureOutflow_Resume
            public const nint m_YieldResume2 = 0x168; // SignatureOutflow_Resume
        }
        // Parent: None
        // Field count: 0
        public static class TestComponent_tAPI {
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyDescription
        // MPulseSignatureForOutflow
        // MPropertyDescription
        // MPropertyDescription
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyDescription
        // MPropertyDescription
        public static class CPulseCell_Value_Curve {
            public const nint m_Curve = 0x48; // CPiecewiseCurve
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // ;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0\;C:\Windows\System32\OpenSSH\;C:\Program Files (x86)\Window
        // MGetKV3ClassDefaults
        public static class CPulseCell_Inflow_EventHandler {
            public const nint m_EventName = 0x80; // PulseSymbol_t
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        public static class CPulseCell_BaseFlow {
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyFriendlyName
        // MPulseSignatureForOutflow
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseSignatureForOutflow
        // MPulseSignatureForOutflow
        // CIRCLE
        // SQUARE
        // TRIANGLE
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_Step_TestDomainTracepoint {
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CPulseCell_Outflow_CycleShuffled__InstanceState_t {
            public const nint m_Shuffle = 0x0; // CUtlVectorFixedGrowable<uint8,8>
            public const nint m_nNextShuffle = 0x20; // int32
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_BaseLerp__CursorState_t {
            public const nint m_StartTime = 0x0; // GameTime_t
            public const nint m_EndTime = 0x4; // GameTime_t
        }
        // Parent: None
        // Field count: 1
        public static class CPulseGraphInstance_TestDomain_Derived {
            public const nint m_nInstanceValueX = 0x160; // int32
        }
        // Parent: None
        // Field count: 1
        public static class CPulseCell_WaitForCursorsWithTagBase__CursorState_t {
            public const nint m_TagName = 0x0; // PulseSymbol_t
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MPropertyDescription
        // MPulseExpressionAlias
        // MPulseExpressionAlias
        // MPulseExpressionAlias
        // MPulseExpressionAlias
        // MPulseExpressionAlias
        // MPulseExpressionAlias
        // MPulseExpressionAlias
        // MPulseExpressionAlias
        // MPulseExpressionAlias
        // MPulseExpressionAlias
        // MPulseExpressionAlias
        // MPulseExpressionAlias
        // MPulseExpressionAlias
        // MPulseExpressionAlias
        // MPulseExpressionAlias
        // MPulseExpressionAlias
        // MPulseExpressionAlias
        // MPulseExpressionAlias
        // MPulseExpressionAlias
        // MPulseExpressionAlias
        // MPulseExpressionAlias
        // MPropertyDescription
        public static class CPulseArraylib {
        }
        // Parent: None
        // Field count: 9
        public static class CPulseGraphInstance_TestDomain {
            public const nint m_bIsRunningUnitTests = 0x130; // bool
            public const nint m_bExplicitTimeStepping = 0x131; // bool
            public const nint m_bExpectingToDestroyWithYieldedCursors = 0x132; // bool
            public const nint m_bQuietTracepoints = 0x133; // bool
            public const nint m_bExpectingCursorTerminatedDueToMaxInstructions = 0x134; // bool
            public const nint m_nCursorsTerminatedDueToMaxInstructions = 0x138; // int32
            public const nint m_nNextValidateIndex = 0x13C; // int32
            public const nint m_Tracepoints = 0x140; // CUtlVector<CUtlString>
            public const nint m_bTestYesOrNoPath = 0x158; // bool
        }
        // Parent: None
        // Field count: 0
        public static class SignatureOutflow_Continue {
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyDescription
        // MPropertyDescription
        // MPropertyDescription
        // MPropertyFriendlyName
        // MPropertyDescription
        public static class CPulseCell_Timeline {
            public const nint m_TimelineEvents = 0x48; // CUtlVector<CPulseCell_Timeline::TimelineEvent_t>
            public const nint m_bWaitForChildOutflows = 0x60; // bool
            public const nint m_OnFinished = 0x68; // CPulse_ResumePoint
            public const nint m_OnCanceled = 0xB0; // CPulse_ResumePoint
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // ;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0\;C:\Windows\System32\OpenSSH\;C:\Program Files (x86)\Window
        // MGetKV3ClassDefaults
        public static class CPulseCell_Inflow_EntOutputHandler {
            public const nint m_SourceEntity = 0x80; // PulseSymbol_t
            public const nint m_SourceOutput = 0x90; // PulseSymbol_t
            public const nint m_ExpectedParamType = 0xA0; // CPulseValueFullType
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyDescription
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // WHITE
        // RED
        // GREEN
        // BLUE
        // MPulseSignatureForOutflow
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyFriendlyName
        // MPulseSignatureForOutflow
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseSignatureForOutflow
        // MPulseSignatureForOutflow
        // CIRCLE
        // SQUARE
        // TRIANGLE
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_Outflow_TestExplicitYesNo {
            public const nint m_Yes = 0x48; // CPulse_OutflowConnection
            public const nint m_No = 0x90; // CPulse_OutflowConnection
            public const nint m_Out1 = 0x48; // SignatureOutflow_Continue
            public const nint m_AsyncChild1 = 0x90; // SignatureOutflow_Continue
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_Outflow_TestRandomYesNo {
            public const nint m_Yes = 0x48; // CPulse_OutflowConnection
            public const nint m_No = 0x90; // CPulse_OutflowConnection
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_Outflow_CycleOrdered__InstanceState_t {
            public const nint m_nNextIndex = 0x0; // int32
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CPulseCell_LimitCount__InstanceState_t {
            public const nint m_nCurrentCount = 0x0; // int32
        }
        // Parent: None
        // Field count: 0
        public static class FakeEntity_tAPI {
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_Test_MultiInflow_WithDefault {
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // ;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0\;C:\Windows\System32\OpenSSH\;C:\Program Files (x86)\Window
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_Step_DebugLog {
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyFriendlyName
        // MPropertyDescription
        public static class CPulseCell_BaseYieldingInflow {
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class PulseNodeDynamicOutflows_t {
            public const nint m_Outflows = 0x0; // CUtlVector<PulseNodeDynamicOutflows_t::DynamicOutflow_t>
        }
        // Parent: None
        // Field count: 1
        public static class CPulseCell_IsRequirementValid__Criteria_t {
            public const nint m_bIsValid = 0x0; // bool
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_Inflow_ObservableVariableListener {
            public const nint m_nBlackboardReference = 0x80; // PulseRuntimeBlackboardReferenceIndex_t
            public const nint m_bSelfReference = 0x82; // bool
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // ;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0\;C:\Windows\System32\OpenSSH\;C:\Program Files (x86)\Window
        // MGetKV3ClassDefaults
        public static class CPulseCell_Outflow_CycleOrdered {
            public const nint m_Outputs = 0x48; // CUtlVector<CPulse_OutflowConnection>
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class PulseSelectorOutflowList_t {
            public const nint m_Outflows = 0x0; // CUtlVector<OutflowWithRequirements_t>
        }
        // Parent: None
        // Field count: 0
        public static class CPulseGraphInstance_TurtleGraphics {
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class CPulseCell_Val_TestDomainGetEntityName {
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // ;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0\;C:\Windows\System32\OpenSSH\;C:\Program Files (x86)\Window
        // MGetKV3ClassDefaults
        public static class CPulseCell_Inflow_Wait {
            public const nint m_WakeResume = 0x48; // CPulse_ResumePoint
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyDescription
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // WHITE
        // RED
        // GREEN
        // BLUE
        // MPulseSignatureForOutflow
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyFriendlyName
        // MPulseSignatureForOutflow
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseSignatureForOutflow
        // MPulseSignatureForOutflow
        // CIRCLE
        // SQUARE
        // TRIANGLE
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_TestWaitWithCursorState {
            public const nint m_WakeResume = 0x48; // CPulse_ResumePoint
            public const nint m_WakeCancel = 0x90; // CPulse_ResumePoint
            public const nint m_WakeFail = 0xD8; // CPulse_ResumePoint
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // ;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0\;C:\Windows\System32\OpenSSH\;C:\Program Files (x86)\Window
        // MGetKV3ClassDefaults
        public static class CPulseCell_Outflow_CycleShuffled {
            public const nint m_Outputs = 0x48; // CUtlVector<CPulse_OutflowConnection>
        }
        // Parent: None
        // Field count: 5
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_Inflow_Method {
            public const nint m_MethodName = 0x80; // PulseSymbol_t
            public const nint m_Description = 0x90; // CUtlString
            public const nint m_bIsPublic = 0x98; // bool
            public const nint m_ReturnType = 0xA0; // CPulseValueFullType
            public const nint m_Args = 0xB8; // CUtlLeanVector<CPulseRuntimeMethodArg>
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_BaseValue {
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorCanvasItemSpecKV3
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPulseEditorHeaderIcon
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyDescription
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // MPropertyDescription
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyDescription
        public static class CPulseCell_BooleanSwitchState {
            public const nint m_Condition = 0x48; // PulseObservableBoolExpression_t
            public const nint m_Always = 0xC0; // CPulse_OutflowConnection
            public const nint m_WhenTrue = 0x108; // CPulse_OutflowConnection
            public const nint m_WhenFalse = 0x150; // CPulse_OutflowConnection
        }
        // Parent: None
        // Field count: 0
        public static class FakeEntityDerivedB_tAPI {
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // ;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0\;C:\Windows\System32\OpenSSH\;C:\Program Files (x86)\Window
        // MGetKV3ClassDefaults
        public static class CPulseCell_Inflow_Yield {
            public const nint m_UnyieldResume = 0x48; // CPulse_ResumePoint
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MPropertyDescription
        public static class CPulseMathlib {
        }
        // Parent: None
        // Field count: 1
        public static class CPulseCell_Unknown {
            public const nint m_UnknownKeys = 0x48; // KeyValues3
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_Outflow_CycleRandom {
            public const nint m_Outputs = 0x48; // CUtlVector<CPulse_OutflowConnection>
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_Step_PublicOutput {
            public const nint m_OutputIndex = 0x48; // PulseRuntimeOutputIndex_t
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class CPulseCell_Val_TestDomainFindEntityByName {
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulse_BlackboardReference {
            public const nint m_hBlackboardResource = 0x0; // CStrongHandle<InfoForResourceTypeIPulseGraphDef>
            public const nint m_BlackboardResource = 0x8; // PulseSymbol_t
            public const nint m_nNodeID = 0x18; // PulseDocNodeID_t
            public const nint m_NodeName = 0x20; // CGlobalSymbol
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // ;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0\;C:\Windows\System32\OpenSSH\;C:\Program Files (x86)\Window
        // MGetKV3ClassDefaults
        public static class CPulseCell_Value_RandomInt {
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPulseEditorHeaderText
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class CPulseCell_Step_TestDomainEntFire {
            public const nint m_Input = 0x48; // CUtlString
            public const nint flWaitValue = 0x0; // float32
            public const nint bFailOnCancel = 0x4; // bool
        }
        // Parent: None
        // Field count: 0
        public static class FakeEntityDerivedA_tAPI {
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // MPropertyDescription
        // MPulseExpressionAlias
        // MPulseLegacyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseLegacyName
        // MPropertyDescription
        // MPropertyLeafSuggestionProviderFn
        // MPulseExpressionAlias
        // 2
        public static class CPulseCell_ExampleSelector {
            public const nint m_OutflowList = 0x48; // PulseSelectorOutflowList_t
        }
        // Parent: None
        // Field count: 6
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulse_CallInfo {
            public const nint m_PortName = 0x0; // PulseSymbol_t
            public const nint m_nEditorNodeID = 0x10; // PulseDocNodeID_t
            public const nint m_RegisterMap = 0x18; // PulseRegisterMap_t
            public const nint m_CallMethodID = 0x48; // PulseDocNodeID_t
            public const nint m_nSrcChunk = 0x4C; // PulseRuntimeChunkIndex_t
            public const nint m_nSrcInstruction = 0x50; // int32
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyFriendlyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // Files\Cloudflare\Cloudflare WARP\;C:\Program Files\CMake\bin;C:\Users\ByteseekerPC\AppData\Local\Programs\Python\Python312\Scrip
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_InlineNodeSkipSelector {
            public const nint m_nFlowNodeID = 0x48; // PulseDocNodeID_t
            public const nint m_bAnd = 0x4C; // bool
            public const nint m_PassOutflow = 0x50; // PulseSelectorOutflowList_t
            public const nint m_FailOutflow = 0x68; // CPulse_OutflowConnection
        }
        // Parent: None
        // Field count: 3
        public static class CPulseCell_ExampleCriteria__Criteria_t {
            public const nint m_flFloatValue1 = 0x0; // float32
            public const nint m_flFloatValue2 = 0x4; // float32
            public const nint m_bMyBool = 0x8; // bool
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseLegacyName
        // MPropertyDescription
        // MPropertyLeafSuggestionProviderFn
        // MPulseExpressionAlias
        // 2
        public static class CPulseCell_ExampleCriteria {
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // CURSOR_ADD_TAG
        // CURSOR_REMOVE_TAG
        // CURSOR_RETIRED
        // REQUIREMENT_PASS
        public static class CPulseCell_LimitCount {
            public const nint m_nLimitCount = 0x48; // int32
        }
        // Parent: None
        // Field count: 5
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // ;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0\;C:\Windows\System32\OpenSSH\;C:\Program Files (x86)\Window
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_Step_CallExternalMethod {
            public const nint m_MethodName = 0x48; // PulseSymbol_t
            public const nint m_nBlackboardIndex = 0x58; // PulseRuntimeBlackboardReferenceIndex_t
            public const nint m_ExpectedArgs = 0x60; // CUtlLeanVector<CPulseRuntimeMethodArg>
            public const nint m_nAsyncCallMode = 0x70; // PulseMethodCallMode_t
            public const nint m_OnFinished = 0x78; // CPulse_ResumePoint
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class PulseObservableBoolExpression_t {
            public const nint m_EvaluateConnection = 0x0; // CPulse_OutflowConnection
            public const nint m_DependentObservableVars = 0x48; // CUtlVector<PulseRuntimeVarIndex_t>
            public const nint m_DependentObservableBlackboardReferences = 0x60; // CUtlVector<PulseRuntimeBlackboardReferenceIndex_t>
        }
        // Parent: None
        // Field count: 1
        public static class CPulseCell_LimitCount__Criteria_t {
            public const nint m_bLimitCountPasses = 0x0; // bool
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_Step_TestDomainCreateFakeEntity {
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // MPropertyDescription
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyDescription
        public static class CPulseCell_CursorQueue {
            public const nint m_nCursorsAllowedToRunParallel = 0x98; // int32
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // ;C:\Windows\System32\Wbem;C:\Windows\System32\WindowsPowerShell\v1.0\;C:\Windows\System32\OpenSSH\;C:\Program Files (x86)\Window
        // MGetKV3ClassDefaults
        public static class CPulseCell_Value_RandomFloat {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseExecCursor {
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class PulseRuntimeVarIndex_t {
            public const nint m_Value = 0x0; // int32
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // generic
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class PulseRuntimeEntrypointIndex_t {
            public const nint m_Value = 0x0; // int32
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class PulseRuntimeDomainValueIndex_t {
            public const nint m_Value = 0x0; // int16
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CPulse_Chunk {
            public const nint m_Instructions = 0x0; // CUtlLeanVector<PGDInstruction_t>
            public const nint m_Registers = 0x10; // CUtlLeanVector<CPulse_RegisterInfo>
            public const nint m_InstructionDebugInfos = 0x20; // CUtlLeanVector<CPulse_InstructionDebug>
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class PulseRuntimeInvokeIndex_t {
            public const nint m_Value = 0x0; // int32
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class PulseRuntimeRegisterIndex_t {
            public const nint m_Value = 0x0; // int16
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // generic
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class PulseCursorID_t {
            public const nint m_Value = 0x0; // int32
        }
        // Parent: None
        // Field count: 6
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class PulseGraphExecutionHistoryCursorDesc_t {
            public const nint vecAncestorCursorIDs = 0x0; // CUtlVector<PulseCursorID_t>
            public const nint nSpawnNodeID = 0x18; // PulseDocNodeID_t
            public const nint nRetiredAtNodeID = 0x1C; // PulseDocNodeID_t
            public const nint flLastReferenced = 0x20; // float32
            public const nint nLastValidEntryIdx = 0x24; // int32
            public const nint bWasAnObservableComputation = 0x28; // bool
        }
        // Parent: None
        // Field count: 1
        public static class TestComponent_t {
            public const nint m_ComponentData = 0x8; // CUtlString
        }
        // Parent: None
        // Field count: 3
        public static class PulseRegisterMap_t {
            public const nint m_Inparams = 0x0; // KeyValues3
            public const nint m_InparamsWhichCanBeMoved = 0x10; // CKV3MemberNameSet
            public const nint m_Outparams = 0x20; // KeyValues3
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulse_PublicOutput {
            public const nint m_Name = 0x0; // PulseSymbol_t
            public const nint m_Description = 0x10; // CUtlString
            public const nint m_Args = 0x18; // CUtlLeanVector<CPulseRuntimeMethodArg>
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // Files\Cloudflare\Cloudflare WARP\;C:\Program Files\CMake\bin;C:\Users\ByteseekerPC\AppData\Local\Programs\Python\Python312\Scrip
        public static class CPulse_OutputConnection {
            public const nint m_SourceOutput = 0x0; // PulseSymbol_t
            public const nint m_TargetEntity = 0x10; // PulseSymbol_t
            public const nint m_TargetInput = 0x20; // PulseSymbol_t
            public const nint m_Param = 0x30; // PulseSymbol_t
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class PulseGraphExecutionHistoryNodeDesc_t {
            public const nint strCellDesc = 0x0; // CBufferString
            public const nint strBindingName = 0x10; // PulseSymbol_t
        }
        // Parent: None
        // Field count: 5
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class PulseGraphExecutionHistoryEntry_t {
            public const nint nCursorID = 0x0; // PulseCursorID_t
            public const nint nEditorID = 0x4; // PulseDocNodeID_t
            public const nint flExecTime = 0x8; // float32
            public const nint unFlags = 0xC; // uint32
            public const nint tagName = 0x10; // PulseSymbol_t
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // generic
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class PulseCursorYieldToken_t {
            public const nint m_Value = 0x0; // int32
        }
        // Parent: None
        // Field count: 5
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CPulseGraphExecutionHistory {
            public const nint m_nInstanceID = 0x0; // PulseGraphInstanceID_t
            public const nint m_strFileName = 0x8; // CUtlString
            public const nint m_vecHistory = 0x10; // CUtlVector<PulseGraphExecutionHistoryEntry_t*>
            public const nint m_mapCellDesc = 0x28; // CUtlOrderedMap<PulseDocNodeID_t,PulseGraphExecutionHistoryNodeDesc_t*>
            public const nint m_mapCursorDesc = 0x50; // CUtlOrderedMap<PulseCursorID_t,PulseGraphExecutionHistoryCursorDesc_t*>
        }
        // Parent: None
        // Field count: 12
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class PGDInstruction_t {
            public const nint m_nCode = 0x0; // PulseInstructionCode_t
            public const nint m_nVar = 0x4; // PulseRuntimeVarIndex_t
            public const nint m_nReg0 = 0x8; // PulseRuntimeRegisterIndex_t
            public const nint m_nReg1 = 0xA; // PulseRuntimeRegisterIndex_t
            public const nint m_nReg2 = 0xC; // PulseRuntimeRegisterIndex_t
            public const nint m_nInvokeBindingIndex = 0x10; // PulseRuntimeInvokeIndex_t
            public const nint m_nChunk = 0x14; // PulseRuntimeChunkIndex_t
            public const nint m_nDestInstruction = 0x18; // int32
            public const nint m_nCallInfoIndex = 0x1C; // PulseRuntimeCallInfoIndex_t
            public const nint m_nConstIdx = 0x20; // PulseRuntimeConstantIndex_t
            public const nint m_nDomainValueIdx = 0x22; // PulseRuntimeDomainValueIndex_t
            public const nint m_nBlackboardReferenceIdx = 0x24; // PulseRuntimeBlackboardReferenceIndex_t
        }
        // Parent: None
        // Field count: 8
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulse_Variable {
            public const nint m_Name = 0x0; // PulseSymbol_t
            public const nint m_Description = 0x10; // CUtlString
            public const nint m_Type = 0x18; // CPulseValueFullType
            public const nint m_DefaultValue = 0x30; // KeyValues3
            public const nint m_nKeysSource = 0x44; // PulseVariableKeysSource_t
            public const nint m_bIsPublicBlackboardVariable = 0x48; // bool
            public const nint m_bIsObservable = 0x49; // bool
            public const nint m_nEditorNodeID = 0x4C; // PulseDocNodeID_t
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class PulseRuntimeOutputIndex_t {
            public const nint m_Value = 0x0; // int32
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulse_DomainValue {
            public const nint m_nType = 0x0; // PulseDomainValueType_t
            public const nint m_Value = 0x8; // CGlobalSymbolCaseSensitive
            public const nint m_RequiredRuntimeType = 0x10; // CPulseValueFullType
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class PulseRuntimeBlackboardReferenceIndex_t {
            public const nint m_Value = 0x0; // int16
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class PulseRuntimeConstantIndex_t {
            public const nint m_Value = 0x0; // int16
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // generic
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class PulseDocNodeID_t {
            public const nint m_Value = 0x0; // int32
        }
        // Parent: None
        // Field count: 5
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CPulse_RegisterInfo {
            public const nint m_nReg = 0x0; // PulseRuntimeRegisterIndex_t
            public const nint m_Type = 0x8; // CPulseValueFullType
            public const nint m_OriginName = 0x20; // CKV3MemberNameWithStorage
            public const nint m_nWrittenByInstruction = 0x58; // int32
            public const nint m_nLastReadByInstruction = 0x5C; // int32
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class PulseRuntimeCellIndex_t {
            public const nint m_Value = 0x0; // int32
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // generic
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class PulseGraphInstanceID_t {
            public const nint m_Value = 0x0; // uint32
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulse_Constant {
            public const nint m_Type = 0x0; // CPulseValueFullType
            public const nint m_Value = 0x18; // KeyValues3
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class PulseRuntimeCallInfoIndex_t {
            public const nint m_Value = 0x0; // int32
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class PulseRuntimeChunkIndex_t {
            public const nint m_Value = 0x0; // int32
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // Files\Cloudflare\Cloudflare WARP\;C:\Program Files\CMake\bin;C:\Users\ByteseekerPC\AppData\Local\Programs\Python\Python312\Scrip
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseRuntimeMethodArg {
            public const nint m_Name = 0x0; // CKV3MemberNameWithStorage
            public const nint m_Description = 0x38; // CUtlString
            public const nint m_Type = 0x40; // CPulseValueFullType
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class PulseRuntimeStateOffset_t {
            public const nint m_Value = 0x0; // uint16
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulse_InstructionDebug {
            public const nint m_nFlowNodeID = 0x0; // PulseDocNodeID_t
            public const nint m_nValueNodeID = 0x4; // PulseDocNodeID_t
            public const nint m_SequencePointName = 0x8; // CGlobalSymbol
        }
    }
}
