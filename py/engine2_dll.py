# Generated using https://github.com/ikhsanprasetyo/source2-dumper
# 2026-09-30 23:53:47.474156300 +07:00

class Schemas:
    # Module: engine2.dll
    class Engine2Dll:
        class EntityDormancyType_t:
            ENTITY_NOT_DORMANT = 0x0
            ENTITY_DORMANT = 0x1
            ENTITY_SUSPENDED = 0x2
        class EntityIOTargetType_t:
            ENTITY_IO_TARGET_INVALID = 0xFFFFFFFFFFFFFFFF
            ENTITY_IO_TARGET_ENTITYNAME = 0x2
            ENTITY_IO_TARGET_EHANDLE = 0x6
            ENTITY_IO_TARGET_ENTITYNAME_OR_CLASSNAME = 0x7
        class CEntityInstance:
            m_iszPrivateVScripts = 0x8 # CUtlSymbolLarge
            m_pEntity = 0x10 # CEntityIdentity*
            m_CScriptComponent = 0x28 # CScriptComponent*
        class CEntityComponent:
            pass
        class EventClientPostSimulate_t:
            pass
        class EventSimpleLoopFrameUpdate_t:
            m_LoopState = 0x0 # EngineLoopState_t
            m_flRealTime = 0x28 # float32
            m_flFrameTime = 0x2C # float32
        class EventPostAdvanceTick_t:
            m_nCurrentTick = 0x30 # int32
            m_nCurrentTickThisFrame = 0x34 # int32
            m_nTotalTicksThisFrame = 0x38 # int32
            m_nTotalTicks = 0x3C # int32
        class CEntityIOOutput:
            pass
        class EventClientSceneSystemThreadStateChange_t:
            m_bThreadsActive = 0x0 # bool
        class EventClientOutput_t:
            m_LoopState = 0x0 # EngineLoopState_t
            m_flRenderTime = 0x28 # float32
            m_flRealTime = 0x2C # float32
            m_flRenderFrameTimeUnbounded = 0x30 # float32
            m_bRenderOnly = 0x34 # bool
        class EventServerPostSimulate_t:
            m_bLastTickBeforeClientUpdate = 0x30 # bool
        class CEntityComponentHelper:
            m_flags = 0x8 # uint32
            m_pInfo = 0x10 # EntComponentInfo_t*
            m_nPriority = 0x18 # int32
            m_pNext = 0x20 # CEntityComponentHelper*
        class EventServerBeginSimulate_t:
            pass
        class EventServerEndAsyncPostTickWork_t:
            pass
        class EventClientAdvanceTick_t:
            pass
        class EntInput_t:
            pass
        class CNetworkVarChainer:
            m_PathIndex = 0x20 # ChangeAccessorFieldPathIndex_t
        class EventClientSimulate_t:
            pass
        class EventClientPostOutput_t:
            m_LoopState = 0x0 # EngineLoopState_t
            m_flRenderTime = 0x28 # float64
            m_flRenderFrameTime = 0x30 # float32
            m_flRenderFrameTimeUnbounded = 0x34 # float32
            m_bRenderOnly = 0x38 # bool
        class EventClientPollInput_t:
            m_LoopState = 0x0 # EngineLoopState_t
            m_flRealTime = 0x28 # float32
        class EventPreDataUpdate_t:
            m_nCount = 0x0 # int32
        class EventClientProcessGameInput_t:
            m_LoopState = 0x0 # EngineLoopState_t
            m_flRealTime = 0x28 # float32
            m_flFrameTime = 0x2C # float32
        class EventFrameBoundary_t:
            m_flFrameTime = 0x0 # float32
        class EventAppShutdown_t:
            m_nDummy0 = 0x0 # int32
        class EventServerPostAdvanceTick_t:
            m_bLastTickBeforeClientUpdate = 0x40 # bool
        class EventProfileStorageAvailable_t:
            m_nSplitScreenSlot = 0x0 # CSplitScreenSlot
        class EventPostDataUpdate_t:
            m_nCount = 0x0 # int32
        class EventClientPreSimulate_t:
            pass
        class EventClientPauseSimulate_t:
            pass
        class EventClientProcessNetworking_t:
            m_nTickCount = 0x0 # int32
        class CEntityAttributeTable:
            m_Attributes = 0x0 # CUtlOrderedMap<CUtlStringTokenNoRegistration,Attribute_t>
            m_Names = 0x28 # CUtlOrderedMap<CUtlStringTokenNoRegistration,CUtlString>
        class EventClientPreOutputParallelWithServer_t:
            pass
        class EventAdvanceTick_t:
            m_nCurrentTick = 0x30 # int32
            m_nCurrentTickThisFrame = 0x34 # int32
            m_nTotalTicksThisFrame = 0x38 # int32
            m_nTotalTicks = 0x3C # int32
        class EventSplitScreenStateChanged_t:
            pass
        class EventClientPostAdvanceTick_t:
            pass
        class EventBugBug_t:
            pass
        class CVariantDefaultAllocator:
            pass
        class EventBugBugComplete_t:
            m_pPayload = 0x0 # EventBugBug_t*
        class EventModInitialized_t:
            pass
        class EventClientPreOutput_t:
            m_LoopState = 0x0 # EngineLoopState_t
            m_flRenderTime = 0x28 # float64
            m_flRenderFrameTime = 0x30 # float64
            m_flRenderFrameTimeUnbounded = 0x38 # float64
            m_flRealTime = 0x40 # float32
            m_bRenderOnly = 0x44 # bool
        class EventClientFrameSimulate_t:
            m_LoopState = 0x0 # EngineLoopState_t
            m_flRealTime = 0x28 # float32
            m_flFrameTime = 0x2C # float32
            m_bScheduleSendTickPacket = 0x30 # bool
        class EventServerAdvanceTick_t:
            pass
        class EventSetTime_t:
            m_LoopState = 0x0 # EngineLoopState_t
            m_nClientOutputFrames = 0x28 # int32
            m_flRealTime = 0x30 # float64
            m_flRenderTime = 0x38 # float64
            m_flRenderFrameTime = 0x40 # float64
            m_flRenderFrameTimeUnbounded = 0x48 # float64
            m_flRenderFrameTimeUnscaled = 0x50 # float64
            m_flTickRemainder = 0x58 # float64
        class EventSimulate_t:
            m_LoopState = 0x0 # EngineLoopState_t
            m_bFirstTick = 0x28 # bool
            m_bLastTick = 0x29 # bool
        class CEntityKeyValues:
            pass
        class EventClientAdvanceNonRenderedFrame_t:
            pass
        class EventServerProcessNetworking_t:
            pass
        class CEmptyEntityInstance:
            pass
        class EntComponentInfo_t:
            m_pName = 0x0 # char*
            m_pCPPClassname = 0x8 # char*
            m_pNetworkDataReferencedDescription = 0x10 # char*
            m_pNetworkDataReferencedPtrPropDescription = 0x18 # char*
            m_nRuntimeIndex = 0x20 # int32
            m_nFlags = 0x24 # uint32
            m_pBaseClassComponentHelper = 0x60 # CEntityComponentHelper*
        class EngineLoopState_t:
            m_nPlatWindowWidth = 0x18 # int32
            m_nPlatWindowHeight = 0x1C # int32
            m_nRenderWidth = 0x20 # int32
            m_nRenderHeight = 0x24 # int32
        class EventClientPollNetworking_t:
            m_nTickCount = 0x0 # int32
        class EventServerBeginAsyncPostTickWork_t:
            m_bIsOncePerFrameAsyncWorkPhase = 0x0 # bool
        class EventClientProcessInput_t:
            m_LoopState = 0x0 # EngineLoopState_t
            m_flRealTime = 0x28 # float32
            m_flTickInterval = 0x2C # float32
            m_flTickStartTime = 0x30 # float64
        class EventServerEndSimulate_t:
            m_bLastTick = 0x0 # bool
        class EventServerPollNetworking_t:
            pass
