// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-05-08 10:01:14.691628500 +07:00

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]

pub mod source2_dumper {
    pub mod schemas {
        // Module: pulse_system.dll
        // Class count: 130
        // Enum count: 12
        pub mod pulse_system_dll {
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum PulseBestOutflowRules_t {
                SORT_BY_NUMBER_OF_VALID_CRITERIA = 0x0,
                SORT_BY_OUTFLOW_INDEX = 0x1
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum PulseTestEnumShape_t {
                CIRCLE = 0x64,
                SQUARE = 0xC8,
                TRIANGLE = 0x12C
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum PulseCursorCancelPriority_t {
                None = 0x0,
                CancelOnSucceeded = 0x1,
                SoftCancel = 0x2,
                HardCancel = 0x3
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum PulseMethodCallMode_t {
                SYNC_WAIT_FOR_COMPLETION = 0x0,
                ASYNC_FIRE_AND_FORGET = 0x1
            }
            // Alignment: 4
            // Member count: 5
            #[repr(u32)]
            pub enum PulseTestEnumColor_t {
                BLACK = 0x0,
                WHITE = 0x1,
                RED = 0x2,
                GREEN = 0x3,
                BLUE = 0x4
            }
            // Alignment: 4
            // Member count: 6
            #[repr(u32)]
            pub enum PulseVariableKeysSource_t {
                PRIVATE = 0x0,
                CPP = 0x1,
                VMAP = 0x2,
                VMDL = 0x3,
                XML = 0x4,
                COUNT = 0x5
            }
            // Alignment: 4
            // Member count: 6
            #[repr(u32)]
            pub enum EPulseGraphExecutionHistoryFlag {
                NO_FLAGS = 0x0,
                CURSOR_ADD_TAG = 0x1,
                CURSOR_REMOVE_TAG = 0x2,
                CURSOR_RETIRED = 0x4,
                REQUIREMENT_PASS = 0x8,
                REQUIREMENT_FAIL = 0x10
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum PulseCursorExecResult_t {
                Succeeded = 0x0,
                Canceled = 0x1,
                Failed = 0x2,
                OngoingNotify = 0x3
            }
            // Alignment: 4
            // Member count: 33
            #[repr(u32)]
            pub enum PulseValueType_t {
                PVAL_VOID = u32::MAX,
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
            #[repr(u32)]
            pub enum PulseApiFeature_t {
                AF_NONE = 0x0,
                AF_ENTITIES = 0x1,
                AF_PANORAMA = 0x2,
                AF_PARTICLES = 0x8,
                AF_FAKE_ENTITIES = 0x10,
                AF_SELECTORS_WITHOUT_REQUIREMENTS = 0x20
            }
            // Alignment: 2
            // Member count: 125
            #[repr(u16)]
            pub enum PulseInstructionCode_t {
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
            #[repr(u32)]
            pub enum PulseDomainValueType_t {
                INVALID = u32::MAX,
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
            pub mod CPulseCell_Step_TestDomainDestroyFakeEntity {
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
            pub mod CPulseCell_WaitForCursorsWithTag {
                pub const m_bTagSelfWhenComplete: usize = 0x98; // bool
                pub const m_nDesiredKillPriority: usize = 0x9C; // PulseCursorCancelPriority_t
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
            pub mod CPulseCell_Test_NoInflow {
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseGraphInstance_TestDomain_FakeEntityOwner {
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CPulseCell_Base {
                pub const m_nEditorNodeID: usize = 0x8; // PulseDocNodeID_t
            }
            // Parent: None
            // Field count: 0
            pub mod CPulse_ResumePoint {
            }
            // Parent: None
            // Field count: 2
            pub mod CTestDomainDerived_Cursor {
                pub const m_nCursorValueA: usize = 0xD8; // int32
                pub const m_nCursorValueB: usize = 0xDC; // int32
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
            pub mod CPulseCell_PickBestOutflowSelector {
                pub const m_nCheckType: usize = 0x48; // PulseBestOutflowRules_t
                pub const m_OutflowList: usize = 0x50; // PulseSelectorOutflowList_t
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
            pub mod CPulseTestFuncs_LibraryA {
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
            pub mod CPulseCell_WaitForObservable {
                pub const m_Condition: usize = 0x48; // PulseObservableBoolExpression_t
                pub const m_OnTrue: usize = 0xC0; // CPulse_ResumePoint
            }
            // Parent: None
            // Field count: 4
            pub mod CPulse_OutflowConnection {
                pub const m_SourceOutflowName: usize = 0x0; // PulseSymbol_t
                pub const m_nDestChunk: usize = 0x10; // PulseRuntimeChunkIndex_t
                pub const m_nInstruction: usize = 0x14; // int32
                pub const m_OutflowRegisterMap: usize = 0x18; // PulseRegisterMap_t
            }
            // Parent: None
            // Field count: 14
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CPulseGraphDef {
                pub const m_DomainIdentifier: usize = 0x8; // PulseSymbol_t
                pub const m_DomainSubType: usize = 0x18; // CPulseValueFullType
                pub const m_ParentMapName: usize = 0x30; // PulseSymbol_t
                pub const m_ParentXmlName: usize = 0x40; // PulseSymbol_t
                pub const m_Chunks: usize = 0x50; // CUtlVector<CPulse_Chunk*>
                pub const m_Cells: usize = 0x68; // CUtlVector<CPulseCell_Base*>
                pub const m_Vars: usize = 0x80; // CUtlVector<CPulse_Variable>
                pub const m_PublicOutputs: usize = 0x98; // CUtlVector<CPulse_PublicOutput>
                pub const m_InvokeBindings: usize = 0xB0; // CUtlVector<CPulse_InvokeBinding*>
                pub const m_CallInfos: usize = 0xC8; // CUtlVector<CPulse_CallInfo*>
                pub const m_Constants: usize = 0xE0; // CUtlVector<CPulse_Constant>
                pub const m_DomainValues: usize = 0xF8; // CUtlVector<CPulse_DomainValue>
                pub const m_BlackboardReferences: usize = 0x110; // CUtlVector<CPulse_BlackboardReference>
                pub const m_OutputConnections: usize = 0x128; // CUtlVector<CPulse_OutputConnection*>
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseGraphInstance_TestDomain_UseReadOnlyBlackboardView {
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // \570
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
            pub mod CPulseCell_FireCursors {
                pub const m_Outflows: usize = 0x48; // CUtlVector<CPulse_OutflowConnection>
                pub const m_bWaitForChildOutflows: usize = 0x60; // bool
                pub const m_OnFinished: usize = 0x68; // CPulse_ResumePoint
                pub const m_OnCanceled: usize = 0xB0; // CPulse_ResumePoint
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CPulseCell_Timeline__TimelineEvent_t {
                pub const m_flTimeFromPrevious: usize = 0x0; // float32
                pub const m_EventOutflow: usize = 0x8; // CPulse_OutflowConnection
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CPulseCell_IntervalTimer__CursorState_t {
                pub const m_StartTime: usize = 0x0; // GameTime_t
                pub const m_EndTime: usize = 0x4; // GameTime_t
                pub const m_flWaitInterval: usize = 0x8; // float32
                pub const m_flWaitIntervalHigh: usize = 0xC; // float32
                pub const m_bCompleteOnNextWake: usize = 0x10; // bool
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
            // u
            // MGetKV3ClassDefaults
            pub mod CPulseCell_BaseRequirement {
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
            pub mod CPulseCell_BaseState {
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod OutflowWithRequirements_t {
                pub const m_Connection: usize = 0x0; // CPulse_OutflowConnection
                pub const m_DestinationFlowNodeID: usize = 0x48; // PulseDocNodeID_t
                pub const m_RequirementNodeIDs: usize = 0x50; // CUtlVector<PulseDocNodeID_t>
                pub const m_nCursorStateBlockIndex: usize = 0x68; // CUtlVector<int32>
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
            pub mod CPulseCell_IsRequirementValid {
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
            pub mod CPulseCell_Value_Gradient {
                pub const m_Gradient: usize = 0x48; // CColorGradient
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
            pub mod CPulseCursorFuncs {
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod PulseNodeDynamicOutflows_t__DynamicOutflow_t {
                pub const m_OutflowID: usize = 0x0; // CGlobalSymbol
                pub const m_Connection: usize = 0x8; // CPulse_OutflowConnection
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
            pub mod CPulseCell_Test_MultiOutflow_WithParams {
                pub const m_Out1: usize = 0x48; // SignatureOutflow_Continue
                pub const m_Out2: usize = 0x90; // SignatureOutflow_Continue
            }
            // Parent: None
            // Field count: 0
            pub mod CBasePulseGraphInstance {
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
            // MGetKV3ClassDefaults
            pub mod CPulseCell_Inflow_GraphHook {
                pub const m_HookName: usize = 0x80; // PulseSymbol_t
            }
            // Parent: None
            // Field count: 0
            pub mod SignatureOutflow_Resume {
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPulseEditorHeaderText
            pub mod CPulseCell_Test_MultiOutflow_WithParams_Yielding__CursorState_t {
                pub const nTestStep: usize = 0x0; // int32
            }
            // Parent: None
            // Field count: 4
            pub mod CPulseTurtleGraphicsCursor {
                pub const m_Color: usize = 0xD8; // Color
                pub const m_vPos: usize = 0xDC; // Vector2D
                pub const m_flHeadingDeg: usize = 0xE4; // float32
                pub const m_bPenUp: usize = 0xE8; // bool
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CPulseCell_TestWaitWithCursorState__CursorState_t {
                pub const flWaitValue: usize = 0x0; // float32
                pub const bFailOnCancel: usize = 0x4; // bool
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
            // u
            // MGetKV3ClassDefaults
            pub mod CPulseCell_Inflow_BaseEntrypoint {
                pub const m_EntryChunk: usize = 0x48; // PulseRuntimeChunkIndex_t
                pub const m_RegisterMap: usize = 0x50; // PulseRegisterMap_t
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
            pub mod CPulseCell_Test_MultiInflow_NoDefault {
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
            pub mod CPulseCell_WaitForCursorsWithTagBase {
                pub const m_nCursorsAllowedToWait: usize = 0x48; // int32
                pub const m_WaitComplete: usize = 0x50; // CPulse_ResumePoint
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CPulse_InvokeBinding {
                pub const m_RegisterMap: usize = 0x0; // PulseRegisterMap_t
                pub const m_FuncName: usize = 0x30; // PulseSymbol_t
                pub const m_nCellIndex: usize = 0x40; // PulseRuntimeCellIndex_t
                pub const m_nSrcChunk: usize = 0x44; // PulseRuntimeChunkIndex_t
                pub const m_nSrcInstruction: usize = 0x48; // int32
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
            pub mod CPulseCell_IntervalTimer {
                pub const m_Completed: usize = 0x48; // CPulse_ResumePoint
                pub const m_OnInterval: usize = 0x90; // SignatureOutflow_Continue
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
            pub mod CPulseTestScriptLib {
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
            // u
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPropertyFriendlyName
            // MPropertyDescription
            pub mod CPulseCell_BaseLerp {
                pub const m_WakeResume: usize = 0x48; // CPulse_ResumePoint
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
            pub mod CPulseCell_Value_TestValue50 {
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
            // ameId=570
            pub mod CPulseCell_Test_MultiOutflow_WithParams_Yielding {
                pub const m_Out1: usize = 0x48; // SignatureOutflow_Continue
                pub const m_AsyncChild1: usize = 0x90; // SignatureOutflow_Continue
                pub const m_AsyncChild2: usize = 0xD8; // SignatureOutflow_Continue
                pub const m_YieldResume1: usize = 0x120; // SignatureOutflow_Resume
                pub const m_YieldResume2: usize = 0x168; // SignatureOutflow_Resume
            }
            // Parent: None
            // Field count: 0
            pub mod TestComponent_tAPI {
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // \570
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
            pub mod CPulseCell_Value_Curve {
                pub const m_Curve: usize = 0x48; // CPiecewiseCurve
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
            // MGetKV3ClassDefaults
            pub mod CPulseCell_Inflow_EventHandler {
                pub const m_EventName: usize = 0x80; // PulseSymbol_t
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
            pub mod CPulseCell_BaseFlow {
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
            pub mod CPulseCell_Step_TestDomainTracepoint {
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CPulseCell_Outflow_CycleShuffled__InstanceState_t {
                pub const m_Shuffle: usize = 0x0; // CUtlVectorFixedGrowable<uint8,8>
                pub const m_nNextShuffle: usize = 0x20; // int32
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CPulseCell_BaseLerp__CursorState_t {
                pub const m_StartTime: usize = 0x0; // GameTime_t
                pub const m_EndTime: usize = 0x4; // GameTime_t
            }
            // Parent: None
            // Field count: 1
            pub mod CPulseGraphInstance_TestDomain_Derived {
                pub const m_nInstanceValueX: usize = 0x160; // int32
            }
            // Parent: None
            // Field count: 1
            pub mod CPulseCell_WaitForCursorsWithTagBase__CursorState_t {
                pub const m_TagName: usize = 0x0; // PulseSymbol_t
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
            pub mod CPulseArraylib {
            }
            // Parent: None
            // Field count: 9
            pub mod CPulseGraphInstance_TestDomain {
                pub const m_bIsRunningUnitTests: usize = 0x130; // bool
                pub const m_bExplicitTimeStepping: usize = 0x131; // bool
                pub const m_bExpectingToDestroyWithYieldedCursors: usize = 0x132; // bool
                pub const m_bQuietTracepoints: usize = 0x133; // bool
                pub const m_bExpectingCursorTerminatedDueToMaxInstructions: usize = 0x134; // bool
                pub const m_nCursorsTerminatedDueToMaxInstructions: usize = 0x138; // int32
                pub const m_nNextValidateIndex: usize = 0x13C; // int32
                pub const m_Tracepoints: usize = 0x140; // CUtlVector<CUtlString>
                pub const m_bTestYesOrNoPath: usize = 0x158; // bool
            }
            // Parent: None
            // Field count: 0
            pub mod SignatureOutflow_Continue {
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
            pub mod CPulseCell_Timeline {
                pub const m_TimelineEvents: usize = 0x48; // CUtlVector<CPulseCell_Timeline::TimelineEvent_t>
                pub const m_bWaitForChildOutflows: usize = 0x60; // bool
                pub const m_OnFinished: usize = 0x68; // CPulse_ResumePoint
                pub const m_OnCanceled: usize = 0xB0; // CPulse_ResumePoint
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
            // MGetKV3ClassDefaults
            pub mod CPulseCell_Inflow_EntOutputHandler {
                pub const m_SourceEntity: usize = 0x80; // PulseSymbol_t
                pub const m_SourceOutput: usize = 0x90; // PulseSymbol_t
                pub const m_ExpectedParamType: usize = 0xA0; // CPulseValueFullType
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
            pub mod CPulseCell_Outflow_TestExplicitYesNo {
                pub const m_Yes: usize = 0x48; // CPulse_OutflowConnection
                pub const m_No: usize = 0x90; // CPulse_OutflowConnection
                pub const m_Out1: usize = 0x48; // SignatureOutflow_Continue
                pub const m_AsyncChild1: usize = 0x90; // SignatureOutflow_Continue
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
            pub mod CPulseCell_Outflow_TestRandomYesNo {
                pub const m_Yes: usize = 0x48; // CPulse_OutflowConnection
                pub const m_No: usize = 0x90; // CPulse_OutflowConnection
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CPulseCell_Outflow_CycleOrdered__InstanceState_t {
                pub const m_nNextIndex: usize = 0x0; // int32
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CPulseCell_LimitCount__InstanceState_t {
                pub const m_nCurrentCount: usize = 0x0; // int32
            }
            // Parent: None
            // Field count: 0
            pub mod FakeEntity_tAPI {
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CPulseCell_Test_MultiInflow_WithDefault {
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CPulseCell_Step_DebugLog {
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
            // u
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPropertyDescription
            // MPropertyFriendlyName
            // MPropertyDescription
            pub mod CPulseCell_BaseYieldingInflow {
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod PulseNodeDynamicOutflows_t {
                pub const m_Outflows: usize = 0x0; // CUtlVector<PulseNodeDynamicOutflows_t::DynamicOutflow_t>
            }
            // Parent: None
            // Field count: 1
            pub mod CPulseCell_IsRequirementValid__Criteria_t {
                pub const m_bIsValid: usize = 0x0; // bool
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
            // u
            // MGetKV3ClassDefaults
            pub mod CPulseCell_Inflow_ObservableVariableListener {
                pub const m_nBlackboardReference: usize = 0x80; // PulseRuntimeBlackboardReferenceIndex_t
                pub const m_bSelfReference: usize = 0x82; // bool
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
            pub mod CPulseCell_Outflow_CycleOrdered {
                pub const m_Outputs: usize = 0x48; // CUtlVector<CPulse_OutflowConnection>
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod PulseSelectorOutflowList_t {
                pub const m_Outflows: usize = 0x0; // CUtlVector<OutflowWithRequirements_t>
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseGraphInstance_TurtleGraphics {
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // ameId=570
            pub mod CPulseCell_Val_TestDomainGetEntityName {
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
            // MGetKV3ClassDefaults
            pub mod CPulseCell_Inflow_Wait {
                pub const m_WakeResume: usize = 0x48; // CPulse_ResumePoint
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
            pub mod CPulseCell_TestWaitWithCursorState {
                pub const m_WakeResume: usize = 0x48; // CPulse_ResumePoint
                pub const m_WakeCancel: usize = 0x90; // CPulse_ResumePoint
                pub const m_WakeFail: usize = 0xD8; // CPulse_ResumePoint
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
            // MGetKV3ClassDefaults
            pub mod CPulseCell_Outflow_CycleShuffled {
                pub const m_Outputs: usize = 0x48; // CUtlVector<CPulse_OutflowConnection>
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
            // u
            // MGetKV3ClassDefaults
            pub mod CPulseCell_Inflow_Method {
                pub const m_MethodName: usize = 0x80; // PulseSymbol_t
                pub const m_Description: usize = 0x90; // CUtlString
                pub const m_bIsPublic: usize = 0x98; // bool
                pub const m_ReturnType: usize = 0xA0; // CPulseValueFullType
                pub const m_Args: usize = 0xB8; // CUtlLeanVector<CPulseRuntimeMethodArg>
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // u
            // MGetKV3ClassDefaults
            pub mod CPulseCell_BaseValue {
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
            pub mod CPulseCell_BooleanSwitchState {
                pub const m_Condition: usize = 0x48; // PulseObservableBoolExpression_t
                pub const m_Always: usize = 0xC0; // CPulse_OutflowConnection
                pub const m_WhenTrue: usize = 0x108; // CPulse_OutflowConnection
                pub const m_WhenFalse: usize = 0x150; // CPulse_OutflowConnection
            }
            // Parent: None
            // Field count: 0
            pub mod FakeEntityDerivedB_tAPI {
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
            // MGetKV3ClassDefaults
            pub mod CPulseCell_Inflow_Yield {
                pub const m_UnyieldResume: usize = 0x48; // CPulse_ResumePoint
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MPropertyDescription
            pub mod CPulseMathlib {
            }
            // Parent: None
            // Field count: 1
            pub mod CPulseCell_Unknown {
                pub const m_UnknownKeys: usize = 0x48; // KeyValues3
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // u
            // MGetKV3ClassDefaults
            pub mod CPulseCell_Outflow_CycleRandom {
                pub const m_Outputs: usize = 0x48; // CUtlVector<CPulse_OutflowConnection>
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // u
            // MGetKV3ClassDefaults
            pub mod CPulseCell_Step_PublicOutput {
                pub const m_OutputIndex: usize = 0x48; // PulseRuntimeOutputIndex_t
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // ameId=570
            pub mod CPulseCell_Val_TestDomainFindEntityByName {
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CPulse_BlackboardReference {
                pub const m_hBlackboardResource: usize = 0x0; // CStrongHandle<InfoForResourceTypeIPulseGraphDef>
                pub const m_BlackboardResource: usize = 0x8; // PulseSymbol_t
                pub const m_nNodeID: usize = 0x18; // PulseDocNodeID_t
                pub const m_NodeName: usize = 0x20; // CGlobalSymbol
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
            pub mod CPulseCell_Value_RandomInt {
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
            // ameId=570
            pub mod CPulseCell_Step_TestDomainEntFire {
                pub const m_Input: usize = 0x48; // CUtlString
                pub const flWaitValue: usize = 0x0; // float32
                pub const bFailOnCancel: usize = 0x4; // bool
            }
            // Parent: None
            // Field count: 0
            pub mod FakeEntityDerivedA_tAPI {
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
            pub mod CPulseCell_ExampleSelector {
                pub const m_OutflowList: usize = 0x48; // PulseSelectorOutflowList_t
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CPulse_CallInfo {
                pub const m_PortName: usize = 0x0; // PulseSymbol_t
                pub const m_nEditorNodeID: usize = 0x10; // PulseDocNodeID_t
                pub const m_RegisterMap: usize = 0x18; // PulseRegisterMap_t
                pub const m_CallMethodID: usize = 0x48; // PulseDocNodeID_t
                pub const m_nSrcChunk: usize = 0x4C; // PulseRuntimeChunkIndex_t
                pub const m_nSrcInstruction: usize = 0x50; // int32
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
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CPulseCell_InlineNodeSkipSelector {
                pub const m_nFlowNodeID: usize = 0x48; // PulseDocNodeID_t
                pub const m_bAnd: usize = 0x4C; // bool
                pub const m_PassOutflow: usize = 0x50; // PulseSelectorOutflowList_t
                pub const m_FailOutflow: usize = 0x68; // CPulse_OutflowConnection
            }
            // Parent: None
            // Field count: 3
            pub mod CPulseCell_ExampleCriteria__Criteria_t {
                pub const m_flFloatValue1: usize = 0x0; // float32
                pub const m_flFloatValue2: usize = 0x4; // float32
                pub const m_bMyBool: usize = 0x8; // bool
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
            pub mod CPulseCell_ExampleCriteria {
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
            pub mod CPulseCell_LimitCount {
                pub const m_nLimitCount: usize = 0x48; // int32
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CPulseCell_Step_CallExternalMethod {
                pub const m_MethodName: usize = 0x48; // PulseSymbol_t
                pub const m_nBlackboardIndex: usize = 0x58; // PulseRuntimeBlackboardReferenceIndex_t
                pub const m_ExpectedArgs: usize = 0x60; // CUtlLeanVector<CPulseRuntimeMethodArg>
                pub const m_nAsyncCallMode: usize = 0x70; // PulseMethodCallMode_t
                pub const m_OnFinished: usize = 0x78; // CPulse_ResumePoint
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod PulseObservableBoolExpression_t {
                pub const m_EvaluateConnection: usize = 0x0; // CPulse_OutflowConnection
                pub const m_DependentObservableVars: usize = 0x48; // CUtlVector<PulseRuntimeVarIndex_t>
                pub const m_DependentObservableBlackboardReferences: usize = 0x60; // CUtlVector<PulseRuntimeBlackboardReferenceIndex_t>
            }
            // Parent: None
            // Field count: 1
            pub mod CPulseCell_LimitCount__Criteria_t {
                pub const m_bLimitCountPasses: usize = 0x0; // bool
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
            pub mod CPulseCell_Step_TestDomainCreateFakeEntity {
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
            pub mod CPulseCell_CursorQueue {
                pub const m_nCursorsAllowedToRunParallel: usize = 0x98; // int32
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
            pub mod CPulseCell_Value_RandomFloat {
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseExecCursor {
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // 3,0x0583/0xb031,0x0583/0xa009,0x25f0/0x83c4,0x25f0/0x83c2,0x18d1/0x9400,0x2563/0x0526,0x0925/0x1700,0x1a34/0x0203,0x0f0d/0x0049,
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod PulseRuntimeVarIndex_t {
                pub const m_Value: usize = 0x0; // int32
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
            pub mod PulseRuntimeEntrypointIndex_t {
                pub const m_Value: usize = 0x0; // int32
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // 3,0x0583/0xb031,0x0583/0xa009,0x25f0/0x83c4,0x25f0/0x83c2,0x18d1/0x9400,0x2563/0x0526,0x0925/0x1700,0x1a34/0x0203,0x0f0d/0x0049,
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod PulseRuntimeDomainValueIndex_t {
                pub const m_Value: usize = 0x0; // int16
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CPulse_Chunk {
                pub const m_Instructions: usize = 0x0; // CUtlLeanVector<PGDInstruction_t>
                pub const m_Registers: usize = 0x10; // CUtlLeanVector<CPulse_RegisterInfo>
                pub const m_InstructionDebugInfos: usize = 0x20; // CUtlLeanVector<CPulse_InstructionDebug>
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // 3,0x0583/0xb031,0x0583/0xa009,0x25f0/0x83c4,0x25f0/0x83c2,0x18d1/0x9400,0x2563/0x0526,0x0925/0x1700,0x1a34/0x0203,0x0f0d/0x0049,
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod PulseRuntimeInvokeIndex_t {
                pub const m_Value: usize = 0x0; // int32
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // 3,0x0583/0xb031,0x0583/0xa009,0x25f0/0x83c4,0x25f0/0x83c2,0x18d1/0x9400,0x2563/0x0526,0x0925/0x1700,0x1a34/0x0203,0x0f0d/0x0049,
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod PulseRuntimeRegisterIndex_t {
                pub const m_Value: usize = 0x0; // int16
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
            pub mod PulseCursorID_t {
                pub const m_Value: usize = 0x0; // int32
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod PulseGraphExecutionHistoryCursorDesc_t {
                pub const vecAncestorCursorIDs: usize = 0x0; // CUtlVector<PulseCursorID_t>
                pub const nSpawnNodeID: usize = 0x18; // PulseDocNodeID_t
                pub const nRetiredAtNodeID: usize = 0x1C; // PulseDocNodeID_t
                pub const flLastReferenced: usize = 0x20; // float32
                pub const nLastValidEntryIdx: usize = 0x24; // int32
                pub const bWasAnObservableComputation: usize = 0x28; // bool
            }
            // Parent: None
            // Field count: 1
            pub mod TestComponent_t {
                pub const m_ComponentData: usize = 0x8; // CUtlString
            }
            // Parent: None
            // Field count: 3
            pub mod PulseRegisterMap_t {
                pub const m_Inparams: usize = 0x0; // KeyValues3
                pub const m_InparamsWhichCanBeMoved: usize = 0x10; // CKV3MemberNameSet
                pub const m_Outparams: usize = 0x20; // KeyValues3
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CPulse_PublicOutput {
                pub const m_Name: usize = 0x0; // PulseSymbol_t
                pub const m_Description: usize = 0x10; // CUtlString
                pub const m_Args: usize = 0x18; // CUtlLeanVector<CPulseRuntimeMethodArg>
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CPulse_OutputConnection {
                pub const m_SourceOutput: usize = 0x0; // PulseSymbol_t
                pub const m_TargetEntity: usize = 0x10; // PulseSymbol_t
                pub const m_TargetInput: usize = 0x20; // PulseSymbol_t
                pub const m_Param: usize = 0x30; // PulseSymbol_t
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod PulseGraphExecutionHistoryNodeDesc_t {
                pub const strCellDesc: usize = 0x0; // CBufferString
                pub const strBindingName: usize = 0x10; // PulseSymbol_t
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod PulseGraphExecutionHistoryEntry_t {
                pub const nCursorID: usize = 0x0; // PulseCursorID_t
                pub const nEditorID: usize = 0x4; // PulseDocNodeID_t
                pub const flExecTime: usize = 0x8; // float32
                pub const unFlags: usize = 0xC; // uint32
                pub const tagName: usize = 0x10; // PulseSymbol_t
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
            pub mod PulseCursorYieldToken_t {
                pub const m_Value: usize = 0x0; // int32
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CPulseGraphExecutionHistory {
                pub const m_nInstanceID: usize = 0x0; // PulseGraphInstanceID_t
                pub const m_strFileName: usize = 0x8; // CUtlString
                pub const m_vecHistory: usize = 0x10; // CUtlVector<PulseGraphExecutionHistoryEntry_t*>
                pub const m_mapCellDesc: usize = 0x28; // CUtlOrderedMap<PulseDocNodeID_t,PulseGraphExecutionHistoryNodeDesc_t*>
                pub const m_mapCursorDesc: usize = 0x50; // CUtlOrderedMap<PulseCursorID_t,PulseGraphExecutionHistoryCursorDesc_t*>
            }
            // Parent: None
            // Field count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod PGDInstruction_t {
                pub const m_nCode: usize = 0x0; // PulseInstructionCode_t
                pub const m_nVar: usize = 0x4; // PulseRuntimeVarIndex_t
                pub const m_nReg0: usize = 0x8; // PulseRuntimeRegisterIndex_t
                pub const m_nReg1: usize = 0xA; // PulseRuntimeRegisterIndex_t
                pub const m_nReg2: usize = 0xC; // PulseRuntimeRegisterIndex_t
                pub const m_nInvokeBindingIndex: usize = 0x10; // PulseRuntimeInvokeIndex_t
                pub const m_nChunk: usize = 0x14; // PulseRuntimeChunkIndex_t
                pub const m_nDestInstruction: usize = 0x18; // int32
                pub const m_nCallInfoIndex: usize = 0x1C; // PulseRuntimeCallInfoIndex_t
                pub const m_nConstIdx: usize = 0x20; // PulseRuntimeConstantIndex_t
                pub const m_nDomainValueIdx: usize = 0x22; // PulseRuntimeDomainValueIndex_t
                pub const m_nBlackboardReferenceIdx: usize = 0x24; // PulseRuntimeBlackboardReferenceIndex_t
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CPulse_Variable {
                pub const m_Name: usize = 0x0; // PulseSymbol_t
                pub const m_Description: usize = 0x10; // CUtlString
                pub const m_Type: usize = 0x18; // CPulseValueFullType
                pub const m_DefaultValue: usize = 0x30; // KeyValues3
                pub const m_nKeysSource: usize = 0x44; // PulseVariableKeysSource_t
                pub const m_bIsPublicBlackboardVariable: usize = 0x48; // bool
                pub const m_bIsObservable: usize = 0x49; // bool
                pub const m_nEditorNodeID: usize = 0x4C; // PulseDocNodeID_t
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // 3,0x0583/0xb031,0x0583/0xa009,0x25f0/0x83c4,0x25f0/0x83c2,0x18d1/0x9400,0x2563/0x0526,0x0925/0x1700,0x1a34/0x0203,0x0f0d/0x0049,
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod PulseRuntimeOutputIndex_t {
                pub const m_Value: usize = 0x0; // int32
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CPulse_DomainValue {
                pub const m_nType: usize = 0x0; // PulseDomainValueType_t
                pub const m_Value: usize = 0x8; // CGlobalSymbolCaseSensitive
                pub const m_RequiredRuntimeType: usize = 0x10; // CPulseValueFullType
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // 3,0x0583/0xb031,0x0583/0xa009,0x25f0/0x83c4,0x25f0/0x83c2,0x18d1/0x9400,0x2563/0x0526,0x0925/0x1700,0x1a34/0x0203,0x0f0d/0x0049,
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod PulseRuntimeBlackboardReferenceIndex_t {
                pub const m_Value: usize = 0x0; // int16
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // 3,0x0583/0xb031,0x0583/0xa009,0x25f0/0x83c4,0x25f0/0x83c2,0x18d1/0x9400,0x2563/0x0526,0x0925/0x1700,0x1a34/0x0203,0x0f0d/0x0049,
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod PulseRuntimeConstantIndex_t {
                pub const m_Value: usize = 0x0; // int16
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
            pub mod PulseDocNodeID_t {
                pub const m_Value: usize = 0x0; // int32
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CPulse_RegisterInfo {
                pub const m_nReg: usize = 0x0; // PulseRuntimeRegisterIndex_t
                pub const m_Type: usize = 0x8; // CPulseValueFullType
                pub const m_OriginName: usize = 0x20; // CKV3MemberNameWithStorage
                pub const m_nWrittenByInstruction: usize = 0x58; // int32
                pub const m_nLastReadByInstruction: usize = 0x5C; // int32
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // 3,0x0583/0xb031,0x0583/0xa009,0x25f0/0x83c4,0x25f0/0x83c2,0x18d1/0x9400,0x2563/0x0526,0x0925/0x1700,0x1a34/0x0203,0x0f0d/0x0049,
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod PulseRuntimeCellIndex_t {
                pub const m_Value: usize = 0x0; // int32
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
            pub mod PulseGraphInstanceID_t {
                pub const m_Value: usize = 0x0; // uint32
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CPulse_Constant {
                pub const m_Type: usize = 0x0; // CPulseValueFullType
                pub const m_Value: usize = 0x18; // KeyValues3
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // 3,0x0583/0xb031,0x0583/0xa009,0x25f0/0x83c4,0x25f0/0x83c2,0x18d1/0x9400,0x2563/0x0526,0x0925/0x1700,0x1a34/0x0203,0x0f0d/0x0049,
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod PulseRuntimeCallInfoIndex_t {
                pub const m_Value: usize = 0x0; // int32
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // 3,0x0583/0xb031,0x0583/0xa009,0x25f0/0x83c4,0x25f0/0x83c2,0x18d1/0x9400,0x2563/0x0526,0x0925/0x1700,0x1a34/0x0203,0x0f0d/0x0049,
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod PulseRuntimeChunkIndex_t {
                pub const m_Value: usize = 0x0; // int32
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CPulseRuntimeMethodArg {
                pub const m_Name: usize = 0x0; // CKV3MemberNameWithStorage
                pub const m_Description: usize = 0x38; // CUtlString
                pub const m_Type: usize = 0x40; // CPulseValueFullType
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // 3,0x0583/0xb031,0x0583/0xa009,0x25f0/0x83c4,0x25f0/0x83c2,0x18d1/0x9400,0x2563/0x0526,0x0925/0x1700,0x1a34/0x0203,0x0f0d/0x0049,
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod PulseRuntimeStateOffset_t {
                pub const m_Value: usize = 0x0; // uint16
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CPulse_InstructionDebug {
                pub const m_nFlowNodeID: usize = 0x0; // PulseDocNodeID_t
                pub const m_nValueNodeID: usize = 0x4; // PulseDocNodeID_t
                pub const m_SequencePointName: usize = 0x8; // CGlobalSymbol
            }
        }
    }
}
