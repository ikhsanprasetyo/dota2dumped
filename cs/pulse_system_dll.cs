// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-07-16 16:42:29.142708300 +07:00

namespace Source2Dumper.Schemas {
    // Module: pulse_system.dll
    // Class count: 125
    // Enum count: 15
    public static class PulseSystemDll {
        // Alignment: 4
        // Member count: 2
        public enum PulseBestOutflowRules_t : uint {
            SORT_BY_NUMBER_OF_VALID_CRITERIA = 0x0,
            SORT_BY_OUTFLOW_INDEX = 0x1
        }
        // Alignment: 4
        // Member count: 4
        public enum PulseTestEnumFlags_t : uint {
            NONE = 0x0,
            FIRST = 0x1,
            SECOND = 0x2,
            THIRD = 0x4
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
        public enum PulseTestEnumFlagsAlt_t : uint {
            NONE = 0x0,
            FIRST = 0x1
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
        // Member count: 2
        public enum PulseCursorWakePriority_t : uint {
            WakeElegantly = 0x0,
            WakeImmediate = 0x1
        }
        // Alignment: 4
        // Member count: 7
        public enum PulseVariableKeysSource_t : uint {
            PRIVATE = 0x0,
            CPP = 0x1,
            VMAP = 0x2,
            VMDL = 0x3,
            XML = 0x4,
            VDATA = 0x5,
            COUNT = 0x6
        }
        // Alignment: 4
        // Member count: 1
        public enum PulseDurationStringFormat_t : uint {
            MM_SS_LEADING_ZERO = 0x0
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
        // Member count: 35
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
            PVAL_ANIM_SEQUENCE = 0x1F,
            PVAL_VDATA_CHOICE = 0x20,
            PVAL_COUNT = 0x21
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
        // Member count: 126
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
            CREATE_CHILD_CURSOR_OUTFLOW = 0xB,
            CELL_INVOKE = 0xC,
            LIBRARY_INVOKE = 0xD,
            SET_VAR = 0xE,
            GET_VAR = 0xF,
            GET_VAR_DETACH = 0x10,
            DETACH_REGISTER = 0x11,
            SET_VAR_ARRAY_ELEMENT_1D = 0x12,
            SET_VAR_OBSERVABLE = 0x13,
            GET_CONST = 0x14,
            GET_ARRAY_ELEMENT = 0x15,
            GET_DOMAIN_VALUE = 0x16,
            COPY = 0x17,
            NOT = 0x18,
            NEGATE = 0x19,
            ADD = 0x1A,
            SUB = 0x1B,
            MUL = 0x1C,
            DIV = 0x1D,
            MOD = 0x1E,
            LT = 0x1F,
            LTE = 0x20,
            EQ = 0x21,
            NE = 0x22,
            AND = 0x23,
            OR = 0x24,
            SCALE = 0x25,
            SCALE_INV = 0x26,
            ELEMENT_ACCESS = 0x27,
            CONVERT_VALUE = 0x28,
            REINTERPRET_INSTANCE = 0x29,
            GET_BLACKBOARD_REFERENCE = 0x2A,
            SET_BLACKBOARD_REFERENCE = 0x2B,
            LAST_SERIALIZED_CODE = 0x2C,
            NEGATE_INT = 0x2D,
            NEGATE_FLOAT = 0x2E,
            NEGATE_VEC2 = 0x2F,
            NEGATE_VEC3 = 0x30,
            NEGATE_VEC4 = 0x31,
            ADD_INT = 0x32,
            ADD_FLOAT = 0x33,
            ADD_STRING = 0x34,
            ADD_VEC2 = 0x35,
            ADD_VEC3 = 0x36,
            ADD_VEC3WS_VEC3 = 0x37,
            ADD_VEC3_VEC3WS = 0x38,
            ADD_VEC4 = 0x39,
            ADD_GAMETIME_FLOAT = 0x3A,
            ADD_FLOAT_GAMETIME = 0x3B,
            SUB_INT = 0x3C,
            SUB_FLOAT = 0x3D,
            SUB_VEC2 = 0x3E,
            SUB_VEC3 = 0x3F,
            SUB_VEC3WS_VEC3 = 0x40,
            SUB_VEC3WS_VEC3WS = 0x41,
            SUB_VEC4 = 0x42,
            SUB_GAMETIME_FLOAT = 0x43,
            SUB_GAMETIME = 0x44,
            MUL_INT = 0x45,
            MUL_FLOAT = 0x46,
            DIV_FLOAT = 0x47,
            MOD_INT = 0x48,
            MOD_FLOAT = 0x49,
            LT_INT = 0x4A,
            LT_FLOAT = 0x4B,
            LT_GAMETIME = 0x4C,
            LTE_INT = 0x4D,
            LTE_FLOAT = 0x4E,
            LTE_GAMETIME = 0x4F,
            EQ_BOOL = 0x50,
            EQ_INT = 0x51,
            EQ_FLOAT = 0x52,
            EQ_VEC2 = 0x53,
            EQ_VEC3 = 0x54,
            EQ_VEC3WS = 0x55,
            EQ_VEC4 = 0x56,
            EQ_STRING = 0x57,
            EQ_ENTITY_NAME = 0x58,
            EQ_SCHEMA_ENUM = 0x59,
            EQ_EHANDLE = 0x5A,
            EQ_PANEL_HANDLE = 0x5B,
            EQ_OPAQUE_HANDLE = 0x5C,
            EQ_TEST_HANDLE = 0x5D,
            EQ_COLOR_RGB = 0x5E,
            EQ_ARRAY = 0x5F,
            EQ_GAMETIME = 0x60,
            NE_BOOL = 0x61,
            NE_INT = 0x62,
            NE_FLOAT = 0x63,
            NE_VEC2 = 0x64,
            NE_VEC3 = 0x65,
            NE_VEC3WS = 0x66,
            NE_VEC4 = 0x67,
            NE_STRING = 0x68,
            NE_ENTITY_NAME = 0x69,
            NE_SCHEMA_ENUM = 0x6A,
            NE_EHANDLE = 0x6B,
            NE_PANEL_HANDLE = 0x6C,
            NE_OPAQUE_HANDLE = 0x6D,
            NE_TEST_HANDLE = 0x6E,
            NE_COLOR_RGB = 0x6F,
            NE_ARRAY = 0x70,
            NE_GAMETIME = 0x71,
            SCALE_VEC3 = 0x72,
            SCALE_VEC2 = 0x73,
            SCALE_VEC4 = 0x74,
            SCALE_INV_VEC3 = 0x75,
            SCALE_INV_VEC2 = 0x76,
            SCALE_INV_VEC4 = 0x77,
            ELEMENT_ACCESS_VEC2 = 0x78,
            ELEMENT_ACCESS_VEC3 = 0x79,
            ELEMENT_ACCESS_VEC3WS = 0x7A,
            ELEMENT_ACCESS_VEC4 = 0x7B,
            ELEMENT_ACCESS_COLOR_RGB = 0x7C,
            GET_CONST_INLINE_STORAGE = 0x7D
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPulseExpressionAlias
        // MPulseLegacyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseLegacyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyLeafSuggestionProviderFn
        // MPulseExpressionAlias
        // MGetKV3ClassDefaults
        // MPropertyDescription
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
        // MPulsePolymorphicDependentReturn
        // MPulsePolymorphicDependentReturn
        // MPulsePolymorphicDependentArg
        // MPulsePolymorphicDependentReturn
        // MPulsePolymorphicDependentArg
        // MPulsePolymorphicDependentReturn
        // MPulsePolymorphicDependentReturn
        // MPulsePolymorphicDependentArg
        // MPulsePolymorphicDependentReturn
        // MPulsePolymorphicDependentReturn
        // MPulsePolymorphicDependentArg
        // MPulsePolymorphicDependentReturn
        // MPulsePolymorphicDependentReturn
        // MPulsePolymorphicDependentArg
        public static class CPulseCell_WaitForCursorsWithTag {
            public const nint m_bTagSelfWhenComplete = 0x128; // bool
            public const nint m_nDesiredKillPriority = 0x12C; // PulseCursorCancelPriority_t
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPulseExpressionAlias
        // MPulseLegacyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseLegacyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyLeafSuggestionProviderFn
        // MPulseExpressionAlias
        // MGetKV3ClassDefaults
        // MPropertyDescription
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
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_PickBestOutflowSelector {
            public const nint m_nCheckType = 0x48; // PulseBestOutflowRules_t
            public const nint m_OutflowList = 0x50; // PulseSelectorOutflowList_t
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPulseEditorHeaderIcon
        // MPropertyFriendlyName
        // MPropertyDescription
        // CURSOR_ADD_TAG
        // CURSOR_REMOVE_TAG
        // CURSOR_RETIRED
        // REQUIREMENT_PASS
        // REQUIREMENT_FAIL
        public static class CPulseCell_WaitForObservable {
            public const nint m_Condition = 0xD8; // CPulseObservableExpression<bool>
            public const nint m_OnTrue = 0x150; // CPulse_ResumePoint
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
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyDescription
        // MGetKV3ClassDefaults
        public static class CPulseCell_TestYieldForever {
        }
        // Parent: None
        // Field count: 0
        public static class CPulseGraphInstance_TestDomain_UseReadOnlyBlackboardView {
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyDescription
        public static class CPulseCell_TestWaitWithCursorState__InstanceState_t {
            public const nint m_nDummy = 0x0; // int32
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPropertyDescription
        // MPropertyDescription
        // V
        // MGetKV3ClassDefaults
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPulseEditorCanvasItemSpecKV3
        public static class CPulseCell_FireCursors {
            public const nint m_Outflows = 0xD8; // CUtlVector<CPulse_OutflowConnection>
            public const nint m_bWaitForChildOutflows = 0xF0; // bool
            public const nint m_OnFinished = 0xF8; // CPulse_ResumePoint
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
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
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
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_BaseState {
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // SORT_BY_OUTFLOW_INDEX
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MPropertyDescription
        // MPropertyDescription
        // V
        // MGetKV3ClassDefaults
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPulseEditorCanvasItemSpecKV3
        public static class CPulseCell_Value_Gradient {
            public const nint m_Gradient = 0x48; // CColorGradient
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyLeafSuggestionProviderFn
        // MPulseExpressionAlias
        // MGetKV3ClassDefaults
        // MPropertyDescription
        // MGetKV3ClassDefaults
        public static class CPulseCell_TestWaitWithAutoTracepoints {
            public const nint m_TracePrefix = 0xD8; // CUtlString
            public const nint m_WakeResume = 0xE0; // CPulse_ResumePoint
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
        // FIRST
        // SECOND
        // THIRD
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPulseEditorHeaderText
        // MPropertyFriendlyName
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // MPropertyDescription
        // MPropertyAttributeSuggestionName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
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
        // MCustomFGDMetadata
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
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
        // Field count: 5
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CPulseCell_TestWaitWithCursorState__CursorState_t {
            public const nint flWaitValue = 0x0; // float32
            public const nint bFail = 0x4; // bool
            public const nint m_hSelfCursor = 0x8; // HYieldedCursor
            public const nint m_hSelfCellInstanceUntyped = 0x14; // HPulseCellBase
            public const nint m_hSelfCellInstance = 0x1C; // HPulseCell<CPulseCell_TestWaitWithCursorState>
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
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
        // FIRST
        // WHITE
        // RED
        // GREEN
        // BLUE
        // MPropertyFriendlyName
        // MPulseSignatureForOutflow
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // FIRST
        // SECOND
        // THIRD
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPulseEditorHeaderText
        // MPropertyFriendlyName
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // MPropertyDescription
        public static class CPulseCell_Test_MultiInflow_NoDefault {
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPulseEditorCanvasItemSpecKV3
        // MPropertyDescription
        // MPropertyDescription
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        public static class CPulseCell_WaitForCursorsWithTagBase {
            public const nint m_nCursorsAllowedToWait = 0xD8; // int32
            public const nint m_WaitComplete = 0xE0; // CPulse_ResumePoint
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
        // MPulseEditorCanvasItemSpecKV3
        // MGetKV3ClassDefaults
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
        public static class CPulseCell_IntervalTimer {
            public const nint m_Completed = 0xD8; // CPulse_ResumePoint
            public const nint m_OnInterval = 0x120; // SignatureOutflow_Continue
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
        public static class CPulseCell_BaseLerp {
            public const nint m_WakeResume = 0xD8; // CPulse_ResumePoint
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPulseExpressionAlias
        // MPulseLegacyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseLegacyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyLeafSuggestionProviderFn
        // MPulseExpressionAlias
        // MGetKV3ClassDefaults
        // MPropertyDescription
        // MGetKV3ClassDefaults
        public static class CPulseCell_Value_TestValue50 {
        }
        // Parent: None
        // Field count: 5
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPulseSignatureForOutflow
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyDescription
        // MPulseSignatureForOutflow
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPropertyFriendlyName
        // MPulseSignatureForOutflow
        // MPropertyFriendlyName
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPulseExpressionAlias
        // MPulseLegacyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseLegacyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyLeafSuggestionProviderFn
        // MPulseExpressionAlias
        // MGetKV3ClassDefaults
        // MPropertyDescription
        // MGetKV3ClassDefaults
        public static class CPulseCell_Test_MultiOutflow_WithParams_Yielding {
            public const nint m_Out1 = 0xD8; // SignatureOutflow_Continue
            public const nint m_AsyncChild1 = 0x120; // SignatureOutflow_Continue
            public const nint m_AsyncChild2 = 0x168; // SignatureOutflow_Continue
            public const nint m_YieldResume1 = 0x1B0; // SignatureOutflow_Resume
            public const nint m_YieldResume2 = 0x1F8; // SignatureOutflow_Resume
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // MGetKV3ClassDefaults
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
        // MCustomFGDMetadata
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
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
        // FIRST
        // SECOND
        // THIRD
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPulseEditorHeaderText
        // MPropertyFriendlyName
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // MPropertyDescription
        // MPropertyAttributeSuggestionName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
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
            public const nint m_nInstanceValueX = 0x158; // int32
        }
        // Parent: None
        // Field count: 9
        public static class CPulseGraphInstance_TestDomain {
            public const nint m_bIsRunningUnitTests = 0x128; // bool
            public const nint m_bExplicitTimeStepping = 0x129; // bool
            public const nint m_bExpectingToDestroyWithYieldedCursors = 0x12A; // bool
            public const nint m_bQuietTracepoints = 0x12B; // bool
            public const nint m_bExpectingCursorTerminatedDueToMaxInstructions = 0x12C; // bool
            public const nint m_nCursorsTerminatedDueToMaxInstructions = 0x130; // int32
            public const nint m_nNextValidateIndex = 0x134; // int32
            public const nint m_Tracepoints = 0x138; // CUtlVector<CUtlString>
            public const nint m_bTestYesOrNoPath = 0x150; // bool
        }
        // Parent: None
        // Field count: 0
        public static class SignatureOutflow_Continue {
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPulseEditorCanvasItemSpecKV3
        // MPropertyDescription
        // MPropertyDescription
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorHeaderIcon
        // MPropertyDescription
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        public static class CPulseCell_Timeline {
            public const nint m_TimelineEvents = 0xD8; // CUtlVector<CPulseCell_Timeline::TimelineEvent_t>
            public const nint m_bWaitForChildOutflows = 0xF0; // bool
            public const nint m_OnFinished = 0xF8; // CPulse_ResumePoint
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MCustomFGDMetadata
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
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
        // FIRST
        // WHITE
        // RED
        // GREEN
        // BLUE
        // MPropertyFriendlyName
        // MPulseSignatureForOutflow
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // FIRST
        // SECOND
        // THIRD
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPulseEditorHeaderText
        // MPropertyFriendlyName
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // MPropertyDescription
        public static class CPulseCell_Outflow_TestExplicitYesNo {
            public const nint m_Yes = 0x48; // CPulse_OutflowConnection
            public const nint m_No = 0x90; // CPulse_OutflowConnection
            public const nint m_Out1 = 0xD8; // SignatureOutflow_Continue
            public const nint m_AsyncChild1 = 0x120; // SignatureOutflow_Continue
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // MPropertyDescription
        // MPropertyAttributeSuggestionName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class CPulseCell_Outflow_TestRandomYesNo {
            public const nint m_Yes = 0x48; // CPulse_OutflowConnection
            public const nint m_No = 0x90; // CPulse_OutflowConnection
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CPulseCell_Outflow_CycleOrdered__InstanceState_t {
            public const nint m_nNextIndex = 0x0; // int32
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_LimitCount__InstanceState_t {
            public const nint m_nCurrentCount = 0x0; // int32
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
        // MPropertyDescription
        // MPropertyAttributeSuggestionName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class CPulseCell_Test_MultiInflow_WithDefault {
        }
        // Parent: None
        // Field count: 0
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        //  
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MCustomFGDMetadata
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_Step_DebugLog {
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MCustomFGDMetadata
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
            public const nint m_BaseFlow_OnAfterCancel = 0x48; // CPulse_ResumePoint
            public const nint m_BaseFlow_WhileActive = 0x90; // CPulse_ResumePoint
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
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
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
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
        // MPulseExpressionAlias
        // MPulseLegacyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseLegacyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyLeafSuggestionProviderFn
        // MPulseExpressionAlias
        // MGetKV3ClassDefaults
        // MPropertyDescription
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_Inflow_Wait {
            public const nint m_WakeResume = 0xD8; // CPulse_ResumePoint
        }
        // Parent: None
        // Field count: 2
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
        // FIRST
        // WHITE
        // RED
        // GREEN
        // BLUE
        // MPropertyFriendlyName
        // MPulseSignatureForOutflow
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyFriendlyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // FIRST
        // SECOND
        // THIRD
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPulseEditorHeaderText
        // MPropertyFriendlyName
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // MPropertyDescription
        public static class CPulseCell_TestWaitWithCursorState {
            public const nint m_WakeResume = 0xD8; // CPulse_ResumePoint
            public const nint m_WakeFail = 0x120; // CPulse_ResumePoint
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        //  
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MCustomFGDMetadata
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
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
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
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
        public static class CPulseCell_BaseValue {
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorCanvasItemSpecKV3
        // MPropertyDescription
        // MPropertyFriendlyName
        // MGetKV3ClassDefaults
        // MPulseEditorHeaderIcon
        // MPropertyFriendlyName
        // MPropertyDescription
        // CURSOR_ADD_TAG
        // CURSOR_REMOVE_TAG
        // CURSOR_RETIRED
        // REQUIREMENT_PASS
        // REQUIREMENT_FAIL
        public static class CPulseCell_BooleanSwitchState {
            public const nint m_Condition = 0xD8; // CPulseObservableExpression<bool>
            public const nint m_WhenTrue = 0x150; // CPulse_OutflowConnection
            public const nint m_WhenFalse = 0x198; // CPulse_OutflowConnection
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
        // MPropertyFriendlyName
        // MPropertyDescription
        public static class CPulseCell_Inflow_Yield {
            public const nint m_UnyieldResume = 0xD8; // CPulse_ResumePoint
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
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
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
        // MPulseExpressionAlias
        // MPulseLegacyName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseLegacyName
        // MPropertyDescription
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyLeafSuggestionProviderFn
        // MPulseExpressionAlias
        // MGetKV3ClassDefaults
        // MPropertyDescription
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MGetKV3ClassDefaults
        public static class CPulseCell_Value_RandomInt {
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class CPulseCell_TestEnums {
            public const nint m_nReferenceColor = 0x48; // PulseTestEnumColor_t
            public const nint m_nReferenceFlags = 0x4C; // PulseTestEnumFlags_t
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPulseEditorHeaderText
        // MPropertyFriendlyName
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // MPropertyDescription
        // MPropertyAttributeSuggestionName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class CPulseCell_Step_TestDomainEntFire {
            public const nint m_Input = 0x48; // CUtlString
            public const nint flWaitValue = 0x0; // float32
            public const nint bFail = 0x4; // bool
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
        // MPropertyAttributeSuggestionName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class CPulseCell_ExampleSelector {
            public const nint m_OutflowList = 0x48; // PulseSelectorOutflowList_t
        }
        // Parent: None
        // Field count: 6
        //
        // Metadata:
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // SORT_BY_OUTFLOW_INDEX
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyLeafSuggestionProviderFn
        // MPulseExpressionAlias
        // MGetKV3ClassDefaults
        // MPropertyDescription
        // MGetKV3ClassDefaults
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
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // SORT_BY_OUTFLOW_INDEX
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_LimitCount {
            public const nint m_nLimitCount = 0x48; // int32
        }
        // Parent: None
        // Field count: 5
        //
        // Metadata:
        // MGetKV3ClassDefaults
        //  
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MCustomFGDMetadata
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseCell_Step_CallExternalMethod {
            public const nint m_MethodName = 0xD8; // PulseSymbol_t
            public const nint m_nBlackboardIndex = 0xE8; // PulseRuntimeBlackboardReferenceIndex_t
            public const nint m_ExpectedArgs = 0xF0; // CUtlLeanVector<CPulseRuntimeMethodArg>
            public const nint m_nAsyncCallMode = 0x100; // PulseMethodCallMode_t
            public const nint m_OnFinished = 0x108; // CPulse_ResumePoint
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
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
        // MPropertyDescription
        // MPropertyAttributeSuggestionName
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class CPulseCell_Step_TestDomainCreateFakeEntity {
        }
        // Parent: None
        // Field count: 5
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CPulseCell_TestYieldWithObservables {
            public const nint m_flWatchForFloatValue = 0xD8; // float32
            public const nint m_LiveFloatValue = 0xE0; // CPulseObservableExpression<float32>
            public const nint m_WatchForStringValue = 0x158; // CUtlString
            public const nint m_LiveStringValue = 0x160; // CPulseObservableExpression<CUtlString>
            public const nint m_WakeResume = 0x1D8; // CPulse_ResumePoint
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
        // MPulsePolymorphicDependentReturn
        // MPulsePolymorphicDependentReturn
        // MPulsePolymorphicDependentArg
        // MPulsePolymorphicDependentReturn
        // MPulsePolymorphicDependentArg
        // MPulsePolymorphicDependentReturn
        // MPulsePolymorphicDependentReturn
        // MPulsePolymorphicDependentArg
        // MPulsePolymorphicDependentReturn
        // MPulsePolymorphicDependentReturn
        // MPulsePolymorphicDependentArg
        // MPulsePolymorphicDependentReturn
        // MPulsePolymorphicDependentReturn
        // MPulsePolymorphicDependentArg
        public static class CPulseCell_CursorQueue {
            public const nint m_nCursorsAllowedToRunParallel = 0x128; // int32
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
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MPulseLegacyName
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MPulseEditorCanvasItemSpecKV3
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
        public static class PulseRuntimeVarIndex_t {
            public const nint m_Value = 0x0; // int32
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // generic
        // MGetKV3ClassDefaults
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
        public static class PulseRuntimeEntrypointIndex_t {
            public const nint m_Value = 0x0; // int32
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // generic
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
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
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CPulseBreakpointLocation {
            public const nint m_NodeID = 0x0; // PulseDocNodeID_t
            public const nint m_SequencePoint = 0x8; // PulseSymbol_t
            public const nint m_PortName = 0x18; // PulseSymbol_t
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // generic
        // MGetKV3ClassDefaults
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
        public static class PulseRuntimeInvokeIndex_t {
            public const nint m_Value = 0x0; // int32
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // generic
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        public static class PulseRuntimeRegisterIndex_t {
            public const nint m_Value = 0x0; // int16
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
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
        // MGetKV3ClassDefaults
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
        // Field count: 9
        //
        // Metadata:
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
            public const nint m_Metadata = 0x50; // KeyValues3
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // generic
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
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
        // generic
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        public static class PulseRuntimeBlackboardReferenceIndex_t {
            public const nint m_Value = 0x0; // int16
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // generic
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        public static class PulseRuntimeConstantIndex_t {
            public const nint m_Value = 0x0; // int16
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // generic
        // MGetKV3ClassDefaults
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
        // generic
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        public static class PulseRuntimeCellIndex_t {
            public const nint m_Value = 0x0; // int32
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // generic
        // MGetKV3ClassDefaults
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
        // generic
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        public static class PulseRuntimeCallInfoIndex_t {
            public const nint m_Value = 0x0; // int32
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
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
        // generic
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MPropertyFriendlyName
        // MPropertyDescription
        // MPulseEditorHeaderIcon
        public static class PulseRuntimeStateOffset_t {
            public const nint m_Value = 0x0; // uint16
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CPulse_InstructionDebug {
            public const nint m_nFlowNodeID = 0x0; // PulseDocNodeID_t
            public const nint m_nValueNodeID = 0x4; // PulseDocNodeID_t
            public const nint m_SequencePointName = 0x8; // PulseSymbol_t
        }
    }
}
