// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-09-24 17:35:37.601127800 +07:00

#pragma once

#include <cstddef>
#include <cstdint>

namespace source2_dumper {
    namespace schemas {
        // Module: pulse_system.dll
        // Class count: 26
        // Enum count: 15
        namespace pulse_system_dll {
            // Alignment: 4
            // Member count: 2
            enum class PulseBestOutflowRules_t : uint32_t {
                SORT_BY_NUMBER_OF_VALID_CRITERIA = 0x0,
                SORT_BY_OUTFLOW_INDEX = 0x1
            };
            // Alignment: 4
            // Member count: 4
            enum class PulseTestEnumFlags_t : uint32_t {
                NONE = 0x0,
                FIRST = 0x1,
                SECOND = 0x2,
                THIRD = 0x4
            };
            // Alignment: 4
            // Member count: 3
            enum class PulseTestEnumShape_t : uint32_t {
                CIRCLE = 0x64,
                SQUARE = 0xC8,
                TRIANGLE = 0x12C
            };
            // Alignment: 4
            // Member count: 4
            enum class PulseCursorCancelPriority_t : uint32_t {
                None = 0x0,
                CancelOnSucceeded = 0x1,
                SoftCancel = 0x2,
                HardCancel = 0x3
            };
            // Alignment: 4
            // Member count: 2
            enum class PulseTestEnumFlagsAlt_t : uint32_t {
                NONE = 0x0,
                FIRST = 0x1
            };
            // Alignment: 4
            // Member count: 2
            enum class PulseMethodCallMode_t : uint32_t {
                SYNC_WAIT_FOR_COMPLETION = 0x0,
                ASYNC_FIRE_AND_FORGET = 0x1
            };
            // Alignment: 4
            // Member count: 5
            enum class PulseTestEnumColor_t : uint32_t {
                BLACK = 0x0,
                WHITE = 0x1,
                RED = 0x2,
                GREEN = 0x3,
                BLUE = 0x4
            };
            // Alignment: 4
            // Member count: 2
            enum class PulseCursorWakePriority_t : uint32_t {
                WakeElegantly = 0x0,
                WakeImmediate = 0x1
            };
            // Alignment: 4
            // Member count: 7
            enum class PulseVariableKeysSource_t : uint32_t {
                PRIVATE = 0x0,
                CPP = 0x1,
                VMAP = 0x2,
                VMDL = 0x3,
                XML = 0x4,
                VDATA = 0x5,
                COUNT = 0x6
            };
            // Alignment: 4
            // Member count: 1
            enum class PulseDurationStringFormat_t : uint32_t {
                MM_SS_LEADING_ZERO = 0x0
            };
            // Alignment: 4
            // Member count: 6
            enum class EPulseGraphExecutionHistoryFlag : uint32_t {
                NO_FLAGS = 0x0,
                CURSOR_ADD_TAG = 0x1,
                CURSOR_REMOVE_TAG = 0x2,
                CURSOR_RETIRED = 0x4,
                REQUIREMENT_PASS = 0x8,
                REQUIREMENT_FAIL = 0x10
            };
            // Alignment: 4
            // Member count: 35
            enum class PulseValueType_t : uint32_t {
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
                PVAL_ANIM_SEQUENCE = 0x1F,
                PVAL_VDATA_CHOICE = 0x20,
                PVAL_COUNT = 0x21
            };
            // Alignment: 4
            // Member count: 6
            enum class PulseApiFeature_t : uint32_t {
                AF_NONE = 0x0,
                AF_ENTITIES = 0x1,
                AF_PANORAMA = 0x2,
                AF_PARTICLES = 0x8,
                AF_FAKE_ENTITIES = 0x10,
                AF_SELECTORS_WITHOUT_REQUIREMENTS = 0x20
            };
            // Alignment: 2
            // Member count: 126
            enum class PulseInstructionCode_t : uint16_t {
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
            };
            // Alignment: 4
            // Member count: 4
            enum class PulseDomainValueType_t : uint32_t {
                INVALID = 0xFFFFFFFF,
                ENTITY_NAME = 0x0,
                PANEL_ID = 0x1,
                COUNT = 0x2
            };
            // Parent: None
            // Field count: 0
            namespace CPulseGraphInstance_TestDomain_FakeEntityOwner {
            }
            // Parent: None
            // Field count: 0
            namespace CPulse_ResumePoint {
            }
            // Parent: None
            // Field count: 2
            namespace CTestDomainDerived_Cursor {
                constexpr std::ptrdiff_t m_nCursorValueA = 0xD8; // int32
                constexpr std::ptrdiff_t m_nCursorValueB = 0xDC; // int32
            }
            // Parent: None
            // Field count: 4
            namespace CPulse_OutflowConnection {
                constexpr std::ptrdiff_t m_SourceOutflowName = 0x0; // PulseSymbol_t
                constexpr std::ptrdiff_t m_nDestChunk = 0x10; // PulseRuntimeChunkIndex_t
                constexpr std::ptrdiff_t m_nInstruction = 0x14; // int32
                constexpr std::ptrdiff_t m_OutflowRegisterMap = 0x18; // PulseRegisterMap_t
            }
            // Parent: None
            // Field count: 0
            namespace CPulseGraphInstance_TestDomain_UseReadOnlyBlackboardView {
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyDescription
            namespace CPulseCell_TestWaitWithCursorState__InstanceState_t {
                constexpr std::ptrdiff_t m_nDummy = 0x0; // int32
            }
            // Parent: None
            // Field count: 0
            namespace CBasePulseGraphInstance {
            }
            // Parent: None
            // Field count: 0
            namespace SignatureOutflow_Resume {
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MPropertyFriendlyName
            // MPulseEditorHeaderText
            namespace CPulseCell_Test_MultiOutflow_WithParams_Yielding__CursorState_t {
                constexpr std::ptrdiff_t nTestStep = 0x0; // int32
            }
            // Parent: None
            // Field count: 4
            namespace CPulseTurtleGraphicsCursor {
                constexpr std::ptrdiff_t m_Color = 0xD8; // Color
                constexpr std::ptrdiff_t m_vPos = 0xDC; // Vector2D
                constexpr std::ptrdiff_t m_flHeadingDeg = 0xE4; // float32
                constexpr std::ptrdiff_t m_bPenUp = 0xE8; // bool
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPulseCell_TestWaitWithCursorState__CursorState_t {
                constexpr std::ptrdiff_t flWaitValue = 0x0; // float32
                constexpr std::ptrdiff_t bFail = 0x4; // bool
                constexpr std::ptrdiff_t m_hSelfCursor = 0x8; // HYieldedCursor
                constexpr std::ptrdiff_t m_hSelfCellInstanceUntyped = 0x14; // HPulseCellBase
                constexpr std::ptrdiff_t m_hSelfCellInstance = 0x1C; // HPulseCell<CPulseCell_TestWaitWithCursorState>
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
            namespace CPulseCell_Test_MultiInflow_NoDefault {
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
            namespace CPulseCell_Test_MultiOutflow_WithParams_Yielding {
                constexpr std::ptrdiff_t m_Out1 = 0xD8; // SignatureOutflow_Continue
                constexpr std::ptrdiff_t m_AsyncChild1 = 0x120; // SignatureOutflow_Continue
                constexpr std::ptrdiff_t m_AsyncChild2 = 0x168; // SignatureOutflow_Continue
                constexpr std::ptrdiff_t m_YieldResume1 = 0x1B0; // SignatureOutflow_Resume
                constexpr std::ptrdiff_t m_YieldResume2 = 0x1F8; // SignatureOutflow_Resume
            }
            // Parent: None
            // Field count: 1
            namespace CPulseGraphInstance_TestDomain_Derived {
                constexpr std::ptrdiff_t m_nInstanceValueX = 0x158; // int32
            }
            // Parent: None
            // Field count: 9
            namespace CPulseGraphInstance_TestDomain {
                constexpr std::ptrdiff_t m_bIsRunningUnitTests = 0x128; // bool
                constexpr std::ptrdiff_t m_bExplicitTimeStepping = 0x129; // bool
                constexpr std::ptrdiff_t m_bExpectingToDestroyWithYieldedCursors = 0x12A; // bool
                constexpr std::ptrdiff_t m_bQuietTracepoints = 0x12B; // bool
                constexpr std::ptrdiff_t m_bExpectingCursorTerminatedDueToMaxInstructions = 0x12C; // bool
                constexpr std::ptrdiff_t m_nCursorsTerminatedDueToMaxInstructions = 0x130; // int32
                constexpr std::ptrdiff_t m_nNextValidateIndex = 0x134; // int32
                constexpr std::ptrdiff_t m_Tracepoints = 0x138; // CUtlVector<CUtlString>
                constexpr std::ptrdiff_t m_bTestYesOrNoPath = 0x150; // bool
            }
            // Parent: None
            // Field count: 0
            namespace SignatureOutflow_Continue {
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
            namespace CPulseCell_Outflow_TestExplicitYesNo {
                constexpr std::ptrdiff_t m_Yes = 0x48; // CPulse_OutflowConnection
                constexpr std::ptrdiff_t m_No = 0x90; // CPulse_OutflowConnection
                constexpr std::ptrdiff_t m_Out1 = 0xD8; // SignatureOutflow_Continue
                constexpr std::ptrdiff_t m_AsyncChild1 = 0x120; // SignatureOutflow_Continue
            }
            // Parent: None
            // Field count: 1
            namespace CPulseCell_IsRequirementValid__Criteria_t {
                constexpr std::ptrdiff_t m_bIsValid = 0x0; // bool
            }
            // Parent: None
            // Field count: 0
            namespace CPulseGraphInstance_TurtleGraphics {
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
            namespace CPulseCell_TestWaitWithCursorState {
                constexpr std::ptrdiff_t m_WakeResume = 0xD8; // CPulse_ResumePoint
                constexpr std::ptrdiff_t m_WakeFail = 0x120; // CPulse_ResumePoint
            }
            // Parent: None
            // Field count: 1
            namespace CPulseCell_Unknown {
                constexpr std::ptrdiff_t m_UnknownKeys = 0x48; // KeyValues3
            }
            // Parent: None
            // Field count: 3
            namespace CPulseCell_ExampleCriteria__Criteria_t {
                constexpr std::ptrdiff_t m_flFloatValue1 = 0x0; // float32
                constexpr std::ptrdiff_t m_flFloatValue2 = 0x4; // float32
                constexpr std::ptrdiff_t m_bMyBool = 0x8; // bool
            }
            // Parent: None
            // Field count: 1
            namespace CPulseCell_LimitCount__Criteria_t {
                constexpr std::ptrdiff_t m_bLimitCountPasses = 0x0; // bool
            }
            // Parent: None
            // Field count: 0
            namespace CPulseExecCursor {
            }
            // Parent: None
            // Field count: 1
            namespace TestComponent_t {
                constexpr std::ptrdiff_t m_ComponentData = 0x8; // CUtlString
            }
            // Parent: None
            // Field count: 3
            namespace PulseRegisterMap_t {
                constexpr std::ptrdiff_t m_Inparams = 0x0; // KeyValues3
                constexpr std::ptrdiff_t m_InparamsWhichCanBeMoved = 0x10; // CKV3MemberNameSet
                constexpr std::ptrdiff_t m_Outparams = 0x20; // KeyValues3
            }
        }
    }
}
