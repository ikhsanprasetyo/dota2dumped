# Generated using https://github.com/ikhsanprasetyo/source2-dumper
# 2026-08-06 20:23:55.050139500 +07:00

class Schemas:
    # Module: pulse_system.dll
    class PulseSystemDll:
        class PulseBestOutflowRules_t:
            SORT_BY_NUMBER_OF_VALID_CRITERIA = 0x0
            SORT_BY_OUTFLOW_INDEX = 0x1
        class PulseTestEnumFlags_t:
            NONE = 0x0
            FIRST = 0x1
            SECOND = 0x2
            THIRD = 0x4
        class PulseTestEnumShape_t:
            CIRCLE = 0x64
            SQUARE = 0xC8
            TRIANGLE = 0x12C
        class PulseCursorCancelPriority_t:
            None_ = 0x0
            CancelOnSucceeded = 0x1
            SoftCancel = 0x2
            HardCancel = 0x3
        class PulseTestEnumFlagsAlt_t:
            NONE = 0x0
            FIRST = 0x1
        class PulseMethodCallMode_t:
            SYNC_WAIT_FOR_COMPLETION = 0x0
            ASYNC_FIRE_AND_FORGET = 0x1
        class PulseTestEnumColor_t:
            BLACK = 0x0
            WHITE = 0x1
            RED = 0x2
            GREEN = 0x3
            BLUE = 0x4
        class PulseCursorWakePriority_t:
            WakeElegantly = 0x0
            WakeImmediate = 0x1
        class PulseVariableKeysSource_t:
            PRIVATE = 0x0
            CPP = 0x1
            VMAP = 0x2
            VMDL = 0x3
            XML = 0x4
            VDATA = 0x5
            COUNT = 0x6
        class PulseDurationStringFormat_t:
            MM_SS_LEADING_ZERO = 0x0
        class EPulseGraphExecutionHistoryFlag:
            NO_FLAGS = 0x0
            CURSOR_ADD_TAG = 0x1
            CURSOR_REMOVE_TAG = 0x2
            CURSOR_RETIRED = 0x4
            REQUIREMENT_PASS = 0x8
            REQUIREMENT_FAIL = 0x10
        class PulseValueType_t:
            PVAL_VOID = 0xFFFFFFFFFFFFFFFF
            PVAL_BOOL = 0x0
            PVAL_INT = 0x1
            PVAL_FLOAT = 0x2
            PVAL_STRING = 0x3
            PVAL_VEC2 = 0x4
            PVAL_VEC3 = 0x5
            PVAL_QANGLE = 0x6
            PVAL_VEC3_WORLDSPACE = 0x7
            PVAL_VEC4 = 0x8
            PVAL_TRANSFORM = 0x9
            PVAL_TRANSFORM_WORLDSPACE = 0xA
            PVAL_COLOR_RGB = 0xB
            PVAL_GAMETIME = 0xC
            PVAL_EHANDLE = 0xD
            PVAL_RESOURCE = 0xE
            PVAL_RESOURCE_NAME = 0xF
            PVAL_SNDEVT_GUID = 0x10
            PVAL_SNDEVT_NAME = 0x11
            PVAL_ENTITY_NAME = 0x12
            PVAL_OPAQUE_HANDLE = 0x13
            PVAL_TYPESAFE_INT = 0x14
            PVAL_MODEL_MATERIAL_GROUP = 0x15
            PVAL_CURSOR_FLOW = 0x16
            PVAL_VARIANT = 0x17
            PVAL_UNKNOWN = 0x18
            PVAL_SCHEMA_ENUM = 0x19
            PVAL_PANORAMA_PANEL_HANDLE = 0x1A
            PVAL_TEST_HANDLE = 0x1B
            PVAL_ARRAY = 0x1C
            PVAL_TYPESAFE_INT64 = 0x1D
            PVAL_PARTICLE_EHANDLE = 0x1E
            PVAL_ANIM_SEQUENCE = 0x1F
            PVAL_VDATA_CHOICE = 0x20
            PVAL_COUNT = 0x21
        class PulseApiFeature_t:
            AF_NONE = 0x0
            AF_ENTITIES = 0x1
            AF_PANORAMA = 0x2
            AF_PARTICLES = 0x8
            AF_FAKE_ENTITIES = 0x10
            AF_SELECTORS_WITHOUT_REQUIREMENTS = 0x20
        class PulseInstructionCode_t:
            INVALID = 0x0
            IMMEDIATE_HALT = 0x1
            RETURN_VOID = 0x2
            RETURN_VALUE = 0x3
            NOP = 0x4
            JUMP = 0x5
            JUMP_COND = 0x6
            CHUNK_LEAP = 0x7
            CHUNK_LEAP_COND = 0x8
            PULSE_CALL_SYNC = 0x9
            PULSE_CALL_ASYNC_FIRE = 0xA
            CREATE_CHILD_CURSOR_OUTFLOW = 0xB
            CELL_INVOKE = 0xC
            LIBRARY_INVOKE = 0xD
            SET_VAR = 0xE
            GET_VAR = 0xF
            GET_VAR_DETACH = 0x10
            DETACH_REGISTER = 0x11
            SET_VAR_ARRAY_ELEMENT_1D = 0x12
            SET_VAR_OBSERVABLE = 0x13
            GET_CONST = 0x14
            GET_ARRAY_ELEMENT = 0x15
            GET_DOMAIN_VALUE = 0x16
            COPY = 0x17
            NOT = 0x18
            NEGATE = 0x19
            ADD = 0x1A
            SUB = 0x1B
            MUL = 0x1C
            DIV = 0x1D
            MOD = 0x1E
            LT = 0x1F
            LTE = 0x20
            EQ = 0x21
            NE = 0x22
            AND = 0x23
            OR = 0x24
            SCALE = 0x25
            SCALE_INV = 0x26
            ELEMENT_ACCESS = 0x27
            CONVERT_VALUE = 0x28
            REINTERPRET_INSTANCE = 0x29
            GET_BLACKBOARD_REFERENCE = 0x2A
            SET_BLACKBOARD_REFERENCE = 0x2B
            LAST_SERIALIZED_CODE = 0x2C
            NEGATE_INT = 0x2D
            NEGATE_FLOAT = 0x2E
            NEGATE_VEC2 = 0x2F
            NEGATE_VEC3 = 0x30
            NEGATE_VEC4 = 0x31
            ADD_INT = 0x32
            ADD_FLOAT = 0x33
            ADD_STRING = 0x34
            ADD_VEC2 = 0x35
            ADD_VEC3 = 0x36
            ADD_VEC3WS_VEC3 = 0x37
            ADD_VEC3_VEC3WS = 0x38
            ADD_VEC4 = 0x39
            ADD_GAMETIME_FLOAT = 0x3A
            ADD_FLOAT_GAMETIME = 0x3B
            SUB_INT = 0x3C
            SUB_FLOAT = 0x3D
            SUB_VEC2 = 0x3E
            SUB_VEC3 = 0x3F
            SUB_VEC3WS_VEC3 = 0x40
            SUB_VEC3WS_VEC3WS = 0x41
            SUB_VEC4 = 0x42
            SUB_GAMETIME_FLOAT = 0x43
            SUB_GAMETIME = 0x44
            MUL_INT = 0x45
            MUL_FLOAT = 0x46
            DIV_FLOAT = 0x47
            MOD_INT = 0x48
            MOD_FLOAT = 0x49
            LT_INT = 0x4A
            LT_FLOAT = 0x4B
            LT_GAMETIME = 0x4C
            LTE_INT = 0x4D
            LTE_FLOAT = 0x4E
            LTE_GAMETIME = 0x4F
            EQ_BOOL = 0x50
            EQ_INT = 0x51
            EQ_FLOAT = 0x52
            EQ_VEC2 = 0x53
            EQ_VEC3 = 0x54
            EQ_VEC3WS = 0x55
            EQ_VEC4 = 0x56
            EQ_STRING = 0x57
            EQ_ENTITY_NAME = 0x58
            EQ_SCHEMA_ENUM = 0x59
            EQ_EHANDLE = 0x5A
            EQ_PANEL_HANDLE = 0x5B
            EQ_OPAQUE_HANDLE = 0x5C
            EQ_TEST_HANDLE = 0x5D
            EQ_COLOR_RGB = 0x5E
            EQ_ARRAY = 0x5F
            EQ_GAMETIME = 0x60
            NE_BOOL = 0x61
            NE_INT = 0x62
            NE_FLOAT = 0x63
            NE_VEC2 = 0x64
            NE_VEC3 = 0x65
            NE_VEC3WS = 0x66
            NE_VEC4 = 0x67
            NE_STRING = 0x68
            NE_ENTITY_NAME = 0x69
            NE_SCHEMA_ENUM = 0x6A
            NE_EHANDLE = 0x6B
            NE_PANEL_HANDLE = 0x6C
            NE_OPAQUE_HANDLE = 0x6D
            NE_TEST_HANDLE = 0x6E
            NE_COLOR_RGB = 0x6F
            NE_ARRAY = 0x70
            NE_GAMETIME = 0x71
            SCALE_VEC3 = 0x72
            SCALE_VEC2 = 0x73
            SCALE_VEC4 = 0x74
            SCALE_INV_VEC3 = 0x75
            SCALE_INV_VEC2 = 0x76
            SCALE_INV_VEC4 = 0x77
            ELEMENT_ACCESS_VEC2 = 0x78
            ELEMENT_ACCESS_VEC3 = 0x79
            ELEMENT_ACCESS_VEC3WS = 0x7A
            ELEMENT_ACCESS_VEC4 = 0x7B
            ELEMENT_ACCESS_COLOR_RGB = 0x7C
            GET_CONST_INLINE_STORAGE = 0x7D
        class PulseDomainValueType_t:
            INVALID = 0xFFFFFFFFFFFFFFFF
            ENTITY_NAME = 0x0
            PANEL_ID = 0x1
            COUNT = 0x2
        class CPulseCell_Step_TestDomainDestroyFakeEntity:
            pass
        class CPulseCell_WaitForCursorsWithTag:
            m_bTagSelfWhenComplete = 0x128 # bool
            m_nDesiredKillPriority = 0x12C # PulseCursorCancelPriority_t
        class CPulseCell_Test_NoInflow:
            pass
        class CPulseGraphInstance_TestDomain_FakeEntityOwner:
            pass
        class CPulseCell_Base:
            m_nEditorNodeID = 0x8 # PulseDocNodeID_t
        class CPulse_ResumePoint:
            pass
        class CTestDomainDerived_Cursor:
            m_nCursorValueA = 0xD8 # int32
            m_nCursorValueB = 0xDC # int32
        class CPulseCell_PickBestOutflowSelector:
            m_nCheckType = 0x48 # PulseBestOutflowRules_t
            m_OutflowList = 0x50 # PulseSelectorOutflowList_t
        class CPulseCell_WaitForObservable:
            m_Condition = 0xD8 # CPulseObservableExpression<bool>
            m_OnTrue = 0x150 # CPulse_ResumePoint
        class CPulse_OutflowConnection:
            m_SourceOutflowName = 0x0 # PulseSymbol_t
            m_nDestChunk = 0x10 # PulseRuntimeChunkIndex_t
            m_nInstruction = 0x14 # int32
            m_OutflowRegisterMap = 0x18 # PulseRegisterMap_t
        class CPulseGraphDef:
            m_DomainIdentifier = 0x8 # PulseSymbol_t
            m_DomainSubType = 0x18 # CPulseValueFullType
            m_ParentMapName = 0x30 # PulseSymbol_t
            m_ParentXmlName = 0x40 # PulseSymbol_t
            m_Chunks = 0x50 # CUtlVector<CPulse_Chunk*>
            m_Cells = 0x68 # CUtlVector<CPulseCell_Base*>
            m_Vars = 0x80 # CUtlVector<CPulse_Variable>
            m_PublicOutputs = 0x98 # CUtlVector<CPulse_PublicOutput>
            m_InvokeBindings = 0xB0 # CUtlVector<CPulse_InvokeBinding*>
            m_CallInfos = 0xC8 # CUtlVector<CPulse_CallInfo*>
            m_Constants = 0xE0 # CUtlVector<CPulse_Constant>
            m_DomainValues = 0xF8 # CUtlVector<CPulse_DomainValue>
            m_BlackboardReferences = 0x110 # CUtlVector<CPulse_BlackboardReference>
            m_OutputConnections = 0x128 # CUtlVector<CPulse_OutputConnection*>
        class CPulseCell_TestYieldForever:
            pass
        class CPulseGraphInstance_TestDomain_UseReadOnlyBlackboardView:
            pass
        class CPulseCell_TestWaitWithCursorState__InstanceState_t:
            m_nDummy = 0x0 # int32
        class CPulseCell_FireCursors:
            m_Outflows = 0xD8 # CUtlVector<CPulse_OutflowConnection>
            m_bWaitForChildOutflows = 0xF0 # bool
            m_OnFinished = 0xF8 # CPulse_ResumePoint
        class CPulseCell_Timeline__TimelineEvent_t:
            m_flTimeFromPrevious = 0x0 # float32
            m_EventOutflow = 0x8 # CPulse_OutflowConnection
        class CPulseCell_IntervalTimer__CursorState_t:
            m_StartTime = 0x0 # GameTime_t
            m_EndTime = 0x4 # GameTime_t
            m_flWaitInterval = 0x8 # float32
            m_flWaitIntervalHigh = 0xC # float32
            m_bCompleteOnNextWake = 0x10 # bool
        class CPulseCell_BaseRequirement:
            pass
        class CPulseCell_BaseState:
            pass
        class OutflowWithRequirements_t:
            m_Connection = 0x0 # CPulse_OutflowConnection
            m_DestinationFlowNodeID = 0x48 # PulseDocNodeID_t
            m_RequirementNodeIDs = 0x50 # CUtlVector<PulseDocNodeID_t>
            m_nCursorStateBlockIndex = 0x68 # CUtlVector<int32>
        class CPulseCell_IsRequirementValid:
            pass
        class CPulseCell_Value_Gradient:
            m_Gradient = 0x48 # CColorGradient
        class CPulseCell_TestWaitWithAutoTracepoints:
            m_TracePrefix = 0xD8 # CUtlString
            m_WakeResume = 0xE0 # CPulse_ResumePoint
        class PulseNodeDynamicOutflows_t__DynamicOutflow_t:
            m_OutflowID = 0x0 # CGlobalSymbol
            m_Connection = 0x8 # CPulse_OutflowConnection
        class CPulseCell_Test_MultiOutflow_WithParams:
            m_Out1 = 0x48 # SignatureOutflow_Continue
            m_Out2 = 0x90 # SignatureOutflow_Continue
        class CBasePulseGraphInstance:
            pass
        class CPulseCell_Inflow_GraphHook:
            m_HookName = 0x80 # PulseSymbol_t
        class SignatureOutflow_Resume:
            pass
        class CPulseCell_Test_MultiOutflow_WithParams_Yielding__CursorState_t:
            nTestStep = 0x0 # int32
        class CPulseTurtleGraphicsCursor:
            m_Color = 0xD8 # Color
            m_vPos = 0xDC # Vector2D
            m_flHeadingDeg = 0xE4 # float32
            m_bPenUp = 0xE8 # bool
        class CPulseCell_TestWaitWithCursorState__CursorState_t:
            flWaitValue = 0x0 # float32
            bFail = 0x4 # bool
            m_hSelfCursor = 0x8 # HYieldedCursor
            m_hSelfCellInstanceUntyped = 0x14 # HPulseCellBase
            m_hSelfCellInstance = 0x1C # HPulseCell<CPulseCell_TestWaitWithCursorState>
        class CPulseCell_Inflow_BaseEntrypoint:
            m_EntryChunk = 0x48 # PulseRuntimeChunkIndex_t
            m_RegisterMap = 0x50 # PulseRegisterMap_t
        class CPulseCell_Test_MultiInflow_NoDefault:
            pass
        class CPulseCell_WaitForCursorsWithTagBase:
            m_nCursorsAllowedToWait = 0xD8 # int32
            m_WaitComplete = 0xE0 # CPulse_ResumePoint
        class CPulse_InvokeBinding:
            m_RegisterMap = 0x0 # PulseRegisterMap_t
            m_FuncName = 0x30 # PulseSymbol_t
            m_nCellIndex = 0x40 # PulseRuntimeCellIndex_t
            m_nSrcChunk = 0x44 # PulseRuntimeChunkIndex_t
            m_nSrcInstruction = 0x48 # int32
        class CPulseCell_IntervalTimer:
            m_Completed = 0xD8 # CPulse_ResumePoint
            m_OnInterval = 0x120 # SignatureOutflow_Continue
        class CPulseCell_BaseLerp:
            m_WakeResume = 0xD8 # CPulse_ResumePoint
        class CPulseCell_Value_TestValue50:
            pass
        class CPulseCell_Test_MultiOutflow_WithParams_Yielding:
            m_Out1 = 0xD8 # SignatureOutflow_Continue
            m_AsyncChild1 = 0x120 # SignatureOutflow_Continue
            m_AsyncChild2 = 0x168 # SignatureOutflow_Continue
            m_YieldResume1 = 0x1B0 # SignatureOutflow_Resume
            m_YieldResume2 = 0x1F8 # SignatureOutflow_Resume
        class CPulseCell_Value_Curve:
            m_Curve = 0x48 # CPiecewiseCurve
        class CPulseCell_Inflow_EventHandler:
            m_EventName = 0x80 # PulseSymbol_t
        class CPulseCell_BaseFlow:
            pass
        class CPulseCell_Step_TestDomainTracepoint:
            pass
        class CPulseCell_Outflow_CycleShuffled__InstanceState_t:
            m_Shuffle = 0x0 # CUtlVectorFixedGrowable<uint8,8>
            m_nNextShuffle = 0x20 # int32
        class CPulseCell_BaseLerp__CursorState_t:
            m_StartTime = 0x0 # GameTime_t
            m_EndTime = 0x4 # GameTime_t
        class CPulseGraphInstance_TestDomain_Derived:
            m_nInstanceValueX = 0x158 # int32
        class CPulseGraphInstance_TestDomain:
            m_bIsRunningUnitTests = 0x128 # bool
            m_bExplicitTimeStepping = 0x129 # bool
            m_bExpectingToDestroyWithYieldedCursors = 0x12A # bool
            m_bQuietTracepoints = 0x12B # bool
            m_bExpectingCursorTerminatedDueToMaxInstructions = 0x12C # bool
            m_nCursorsTerminatedDueToMaxInstructions = 0x130 # int32
            m_nNextValidateIndex = 0x134 # int32
            m_Tracepoints = 0x138 # CUtlVector<CUtlString>
            m_bTestYesOrNoPath = 0x150 # bool
        class SignatureOutflow_Continue:
            pass
        class CPulseCell_Timeline:
            m_TimelineEvents = 0xD8 # CUtlVector<CPulseCell_Timeline::TimelineEvent_t>
            m_bWaitForChildOutflows = 0xF0 # bool
            m_OnFinished = 0xF8 # CPulse_ResumePoint
        class CPulseCell_Inflow_EntOutputHandler:
            m_SourceEntity = 0x80 # PulseSymbol_t
            m_SourceOutput = 0x90 # PulseSymbol_t
            m_ExpectedParamType = 0xA0 # CPulseValueFullType
        class CPulseCell_Outflow_TestExplicitYesNo:
            m_Yes = 0x48 # CPulse_OutflowConnection
            m_No = 0x90 # CPulse_OutflowConnection
            m_Out1 = 0xD8 # SignatureOutflow_Continue
            m_AsyncChild1 = 0x120 # SignatureOutflow_Continue
        class CPulseCell_Outflow_TestRandomYesNo:
            m_Yes = 0x48 # CPulse_OutflowConnection
            m_No = 0x90 # CPulse_OutflowConnection
        class CPulseCell_Outflow_CycleOrdered__InstanceState_t:
            m_nNextIndex = 0x0 # int32
        class CPulseCell_LimitCount__InstanceState_t:
            m_nCurrentCount = 0x0 # int32
        class CPulseCell_Test_MultiInflow_WithDefault:
            pass
        class CPulseCell_Step_DebugLog:
            pass
        class CPulseCell_BaseYieldingInflow:
            m_BaseFlow_OnAfterCancel = 0x48 # CPulse_ResumePoint
            m_BaseFlow_WhileActive = 0x90 # CPulse_ResumePoint
        class PulseNodeDynamicOutflows_t:
            m_Outflows = 0x0 # CUtlVector<PulseNodeDynamicOutflows_t::DynamicOutflow_t>
        class CPulseCell_IsRequirementValid__Criteria_t:
            m_bIsValid = 0x0 # bool
        class CPulseCell_Inflow_ObservableVariableListener:
            m_nBlackboardReference = 0x80 # PulseRuntimeBlackboardReferenceIndex_t
            m_bSelfReference = 0x82 # bool
        class CPulseCell_Outflow_CycleOrdered:
            m_Outputs = 0x48 # CUtlVector<CPulse_OutflowConnection>
        class PulseSelectorOutflowList_t:
            m_Outflows = 0x0 # CUtlVector<OutflowWithRequirements_t>
        class CPulseGraphInstance_TurtleGraphics:
            pass
        class CPulseCell_Val_TestDomainGetEntityName:
            pass
        class CPulseCell_Inflow_Wait:
            m_WakeResume = 0xD8 # CPulse_ResumePoint
        class CPulseCell_TestWaitWithCursorState:
            m_WakeResume = 0xD8 # CPulse_ResumePoint
            m_WakeFail = 0x120 # CPulse_ResumePoint
        class CPulseCell_Outflow_CycleShuffled:
            m_Outputs = 0x48 # CUtlVector<CPulse_OutflowConnection>
        class CPulseCell_Inflow_Method:
            m_MethodName = 0x80 # PulseSymbol_t
            m_Description = 0x90 # CUtlString
            m_bIsPublic = 0x98 # bool
            m_ReturnType = 0xA0 # CPulseValueFullType
            m_Args = 0xB8 # CUtlLeanVector<CPulseRuntimeMethodArg>
        class CPulseCell_BaseValue:
            pass
        class CPulseCell_BooleanSwitchState:
            m_Condition = 0xD8 # CPulseObservableExpression<bool>
            m_WhenTrue = 0x150 # CPulse_OutflowConnection
            m_WhenFalse = 0x198 # CPulse_OutflowConnection
        class CPulseCell_Inflow_Yield:
            m_UnyieldResume = 0xD8 # CPulse_ResumePoint
        class CPulseCell_Unknown:
            m_UnknownKeys = 0x48 # KeyValues3
        class CPulseCell_Outflow_CycleRandom:
            m_Outputs = 0x48 # CUtlVector<CPulse_OutflowConnection>
        class CPulseCell_Step_PublicOutput:
            m_OutputIndex = 0x48 # PulseRuntimeOutputIndex_t
        class CPulseCell_Val_TestDomainFindEntityByName:
            pass
        class CPulse_BlackboardReference:
            m_hBlackboardResource = 0x0 # CStrongHandle<InfoForResourceTypeIPulseGraphDef>
            m_BlackboardResource = 0x8 # PulseSymbol_t
            m_nNodeID = 0x18 # PulseDocNodeID_t
            m_NodeName = 0x20 # CGlobalSymbol
        class CPulseCell_Value_RandomInt:
            pass
        class CPulseCell_TestEnums:
            m_nReferenceColor = 0x48 # PulseTestEnumColor_t
            m_nReferenceFlags = 0x4C # PulseTestEnumFlags_t
        class CPulseCell_Step_TestDomainEntFire:
            m_Input = 0x48 # CUtlString
            flWaitValue = 0x0 # float32
            bFail = 0x4 # bool
        class CPulseCell_ExampleSelector:
            m_OutflowList = 0x48 # PulseSelectorOutflowList_t
        class CPulse_CallInfo:
            m_PortName = 0x0 # PulseSymbol_t
            m_nEditorNodeID = 0x10 # PulseDocNodeID_t
            m_RegisterMap = 0x18 # PulseRegisterMap_t
            m_CallMethodID = 0x48 # PulseDocNodeID_t
            m_nSrcChunk = 0x4C # PulseRuntimeChunkIndex_t
            m_nSrcInstruction = 0x50 # int32
        class CPulseCell_InlineNodeSkipSelector:
            m_nFlowNodeID = 0x48 # PulseDocNodeID_t
            m_bAnd = 0x4C # bool
            m_PassOutflow = 0x50 # PulseSelectorOutflowList_t
            m_FailOutflow = 0x68 # CPulse_OutflowConnection
        class CPulseCell_ExampleCriteria__Criteria_t:
            m_flFloatValue1 = 0x0 # float32
            m_flFloatValue2 = 0x4 # float32
            m_bMyBool = 0x8 # bool
        class CPulseCell_ExampleCriteria:
            pass
        class CPulseCell_LimitCount:
            m_nLimitCount = 0x48 # int32
        class CPulseCell_Step_CallExternalMethod:
            m_MethodName = 0xD8 # PulseSymbol_t
            m_nBlackboardIndex = 0xE8 # PulseRuntimeBlackboardReferenceIndex_t
            m_ExpectedArgs = 0xF0 # CUtlLeanVector<CPulseRuntimeMethodArg>
            m_nAsyncCallMode = 0x100 # PulseMethodCallMode_t
            m_OnFinished = 0x108 # CPulse_ResumePoint
        class CPulseCell_LimitCount__Criteria_t:
            m_bLimitCountPasses = 0x0 # bool
        class CPulseCell_Step_TestDomainCreateFakeEntity:
            pass
        class CPulseCell_TestYieldWithObservables:
            m_flWatchForFloatValue = 0xD8 # float32
            m_LiveFloatValue = 0xE0 # CPulseObservableExpression<float32>
            m_WatchForStringValue = 0x158 # CUtlString
            m_LiveStringValue = 0x160 # CPulseObservableExpression<CUtlString>
            m_WakeResume = 0x1D8 # CPulse_ResumePoint
        class CPulseCell_CursorQueue:
            m_nCursorsAllowedToRunParallel = 0x128 # int32
        class CPulseCell_Value_RandomFloat:
            pass
        class CPulseExecCursor:
            pass
        class PulseRuntimeVarIndex_t:
            m_Value = 0x0 # int32
        class PulseRuntimeEntrypointIndex_t:
            m_Value = 0x0 # int32
        class PulseRuntimeDomainValueIndex_t:
            m_Value = 0x0 # int16
        class CPulse_Chunk:
            m_Instructions = 0x0 # CUtlLeanVector<PGDInstruction_t>
            m_Registers = 0x10 # CUtlLeanVector<CPulse_RegisterInfo>
            m_InstructionDebugInfos = 0x20 # CUtlLeanVector<CPulse_InstructionDebug>
        class CPulseBreakpointLocation:
            m_NodeID = 0x0 # PulseDocNodeID_t
            m_SequencePoint = 0x8 # PulseSymbol_t
            m_PortName = 0x18 # PulseSymbol_t
        class PulseRuntimeInvokeIndex_t:
            m_Value = 0x0 # int32
        class PulseRuntimeRegisterIndex_t:
            m_Value = 0x0 # int16
        class PulseCursorID_t:
            m_Value = 0x0 # int32
        class PulseGraphExecutionHistoryCursorDesc_t:
            vecAncestorCursorIDs = 0x0 # CUtlVector<PulseCursorID_t>
            nSpawnNodeID = 0x18 # PulseDocNodeID_t
            nRetiredAtNodeID = 0x1C # PulseDocNodeID_t
            flLastReferenced = 0x20 # float32
            nLastValidEntryIdx = 0x24 # int32
            bWasAnObservableComputation = 0x28 # bool
        class TestComponent_t:
            m_ComponentData = 0x8 # CUtlString
        class PulseRegisterMap_t:
            m_Inparams = 0x0 # KeyValues3
            m_InparamsWhichCanBeMoved = 0x10 # CKV3MemberNameSet
            m_Outparams = 0x20 # KeyValues3
        class CPulse_PublicOutput:
            m_Name = 0x0 # PulseSymbol_t
            m_Description = 0x10 # CUtlString
            m_Args = 0x18 # CUtlLeanVector<CPulseRuntimeMethodArg>
        class CPulse_OutputConnection:
            m_SourceOutput = 0x0 # PulseSymbol_t
            m_TargetEntity = 0x10 # PulseSymbol_t
            m_TargetInput = 0x20 # PulseSymbol_t
            m_Param = 0x30 # PulseSymbol_t
        class PulseGraphExecutionHistoryNodeDesc_t:
            strCellDesc = 0x0 # CBufferString
            strBindingName = 0x10 # PulseSymbol_t
        class PulseGraphExecutionHistoryEntry_t:
            nCursorID = 0x0 # PulseCursorID_t
            nEditorID = 0x4 # PulseDocNodeID_t
            flExecTime = 0x8 # float32
            unFlags = 0xC # uint32
            tagName = 0x10 # PulseSymbol_t
        class PulseCursorYieldToken_t:
            m_Value = 0x0 # int32
        class CPulseGraphExecutionHistory:
            m_nInstanceID = 0x0 # PulseGraphInstanceID_t
            m_strFileName = 0x8 # CUtlString
            m_vecHistory = 0x10 # CUtlVector<PulseGraphExecutionHistoryEntry_t*>
            m_mapCellDesc = 0x28 # CUtlOrderedMap<PulseDocNodeID_t,PulseGraphExecutionHistoryNodeDesc_t*>
            m_mapCursorDesc = 0x50 # CUtlOrderedMap<PulseCursorID_t,PulseGraphExecutionHistoryCursorDesc_t*>
        class PGDInstruction_t:
            m_nCode = 0x0 # PulseInstructionCode_t
            m_nVar = 0x4 # PulseRuntimeVarIndex_t
            m_nReg0 = 0x8 # PulseRuntimeRegisterIndex_t
            m_nReg1 = 0xA # PulseRuntimeRegisterIndex_t
            m_nReg2 = 0xC # PulseRuntimeRegisterIndex_t
            m_nInvokeBindingIndex = 0x10 # PulseRuntimeInvokeIndex_t
            m_nChunk = 0x14 # PulseRuntimeChunkIndex_t
            m_nDestInstruction = 0x18 # int32
            m_nCallInfoIndex = 0x1C # PulseRuntimeCallInfoIndex_t
            m_nConstIdx = 0x20 # PulseRuntimeConstantIndex_t
            m_nDomainValueIdx = 0x22 # PulseRuntimeDomainValueIndex_t
            m_nBlackboardReferenceIdx = 0x24 # PulseRuntimeBlackboardReferenceIndex_t
        class CPulse_Variable:
            m_Name = 0x0 # PulseSymbol_t
            m_Description = 0x10 # CUtlString
            m_Type = 0x18 # CPulseValueFullType
            m_DefaultValue = 0x30 # KeyValues3
            m_nKeysSource = 0x44 # PulseVariableKeysSource_t
            m_bIsPublicBlackboardVariable = 0x48 # bool
            m_bIsObservable = 0x49 # bool
            m_nEditorNodeID = 0x4C # PulseDocNodeID_t
            m_Metadata = 0x50 # KeyValues3
        class PulseRuntimeOutputIndex_t:
            m_Value = 0x0 # int32
        class CPulse_DomainValue:
            m_nType = 0x0 # PulseDomainValueType_t
            m_Value = 0x8 # CGlobalSymbolCaseSensitive
            m_RequiredRuntimeType = 0x10 # CPulseValueFullType
        class PulseRuntimeBlackboardReferenceIndex_t:
            m_Value = 0x0 # int16
        class PulseRuntimeConstantIndex_t:
            m_Value = 0x0 # int16
        class PulseDocNodeID_t:
            m_Value = 0x0 # int32
        class CPulse_RegisterInfo:
            m_nReg = 0x0 # PulseRuntimeRegisterIndex_t
            m_Type = 0x8 # CPulseValueFullType
            m_OriginName = 0x20 # CKV3MemberNameWithStorage
            m_nWrittenByInstruction = 0x58 # int32
            m_nLastReadByInstruction = 0x5C # int32
        class PulseRuntimeCellIndex_t:
            m_Value = 0x0 # int32
        class PulseGraphInstanceID_t:
            m_Value = 0x0 # uint32
        class CPulse_Constant:
            m_Type = 0x0 # CPulseValueFullType
            m_Value = 0x18 # KeyValues3
        class PulseRuntimeCallInfoIndex_t:
            m_Value = 0x0 # int32
        class PulseRuntimeChunkIndex_t:
            m_Value = 0x0 # int32
        class CPulseRuntimeMethodArg:
            m_Name = 0x0 # CKV3MemberNameWithStorage
            m_Description = 0x38 # CUtlString
            m_Type = 0x40 # CPulseValueFullType
        class PulseRuntimeStateOffset_t:
            m_Value = 0x0 # uint16
        class CPulse_InstructionDebug:
            m_nFlowNodeID = 0x0 # PulseDocNodeID_t
            m_nValueNodeID = 0x4 # PulseDocNodeID_t
            m_SequencePointName = 0x8 # PulseSymbol_t
