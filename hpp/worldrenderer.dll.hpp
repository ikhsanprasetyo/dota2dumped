// printed at 2024-02-04 18:54:53.405635700 +07:00
#pragma once
#include <cstdint>

namespace Offsets {
    namespace AggregateLODSetup_t {
        constexpr uint32_t m_vLODOrigin = 0x0; // Vector
        constexpr uint32_t m_fMaxObjectScale = 0xC; // float32
        constexpr uint32_t m_fSwitchDistances = 0x10; // CUtlVectorFixedGrowable< float32 >
    }

    namespace AggregateMeshInfo_t {
        constexpr uint32_t m_nVisClusterMemberOffset = 0x0; // uint32
        constexpr uint32_t m_nVisClusterMemberCount = 0x4; // uint8
        constexpr uint32_t m_bHasTransform = 0x5; // bool
        constexpr uint32_t m_nDrawCallIndex = 0x6; // int16
        constexpr uint32_t m_nLODSetupIndex = 0x8; // int16
        constexpr uint32_t m_nLODGroupMask = 0xA; // uint8
        constexpr uint32_t m_vTintColor = 0xB; // Color
        constexpr uint32_t m_objectFlags = 0x10; // ObjectTypeFlags_t
        constexpr uint32_t m_nLightProbeVolumePrecomputedHandshake = 0x14; // int32
    }

    namespace AggregateSceneObject_t {
        constexpr uint32_t m_allFlags = 0x0; // ObjectTypeFlags_t
        constexpr uint32_t m_anyFlags = 0x4; // ObjectTypeFlags_t
        constexpr uint32_t m_nLayer = 0x8; // int16
        constexpr uint32_t m_aggregateMeshes = 0x10; // CUtlVector< AggregateMeshInfo_t >
        constexpr uint32_t m_lodSetups = 0x28; // CUtlVector< AggregateLODSetup_t >
        constexpr uint32_t m_visClusterMembership = 0x40; // CUtlVector< uint16 >
        constexpr uint32_t m_fragmentTransforms = 0x58; // CUtlVector< matrix3x4_t >
        constexpr uint32_t m_renderableModel = 0x70; // CStrongHandle< InfoForResourceTypeCModel >
    }

    namespace BakedLightingInfo_t {
        constexpr uint32_t m_nLightmapVersionNumber = 0x0; // uint32
        constexpr uint32_t m_nLightmapGameVersionNumber = 0x4; // uint32
        constexpr uint32_t m_vLightmapUvScale = 0x8; // Vector2D
        constexpr uint32_t m_bHasLightmaps = 0x10; // bool
        constexpr uint32_t m_lightMaps = 0x18; // CUtlVector< CStrongHandle< InfoForResourceTypeCTextureBase > >
    }

    namespace BaseSceneObjectOverride_t {
        constexpr uint32_t m_nSceneObjectIndex = 0x0; // uint32
    }

    namespace CEntityIdentity {
        constexpr uint32_t m_nameStringableIndex = 0x14; // int32
        constexpr uint32_t m_name = 0x18; // CUtlSymbolLarge
        constexpr uint32_t m_designerName = 0x20; // CUtlSymbolLarge
        constexpr uint32_t m_flags = 0x30; // uint32
        constexpr uint32_t m_worldGroupId = 0x38; // WorldGroupId_t
        constexpr uint32_t m_fDataObjectTypes = 0x3C; // uint32
        constexpr uint32_t m_PathIndex = 0x40; // ChangeAccessorFieldPathIndex_t
        constexpr uint32_t m_pPrev = 0x58; // CEntityIdentity*
        constexpr uint32_t m_pNext = 0x60; // CEntityIdentity*
        constexpr uint32_t m_pPrevByClass = 0x68; // CEntityIdentity*
        constexpr uint32_t m_pNextByClass = 0x70; // CEntityIdentity*
    }

    namespace CEntityInstance {
        constexpr uint32_t m_iszPrivateVScripts = 0x8; // CUtlSymbolLarge
        constexpr uint32_t m_pEntity = 0x10; // CEntityIdentity*
        constexpr uint32_t m_CScriptComponent = 0x28; // CScriptComponent*
    }

    namespace CScriptComponent {
        constexpr uint32_t m_scriptClassName = 0x30; // CUtlSymbolLarge
    }

    namespace CVoxelVisibility {
        constexpr uint32_t m_nBaseClusterCount = 0x40; // uint32
        constexpr uint32_t m_nPVSBytesPerCluster = 0x44; // uint32
        constexpr uint32_t m_vMinBounds = 0x48; // Vector
        constexpr uint32_t m_vMaxBounds = 0x54; // Vector
        constexpr uint32_t m_flGridSize = 0x60; // float32
        constexpr uint32_t m_nSkyVisibilityCluster = 0x64; // uint32
        constexpr uint32_t m_nSunVisibilityCluster = 0x68; // uint32
        constexpr uint32_t m_NodeBlock = 0x6C; // VoxelVisBlockOffset_t
        constexpr uint32_t m_RegionBlock = 0x74; // VoxelVisBlockOffset_t
        constexpr uint32_t m_EnclosedClusterListBlock = 0x7C; // VoxelVisBlockOffset_t
        constexpr uint32_t m_EnclosedClustersBlock = 0x84; // VoxelVisBlockOffset_t
        constexpr uint32_t m_MasksBlock = 0x8C; // VoxelVisBlockOffset_t
        constexpr uint32_t m_nVisBlocks = 0x94; // VoxelVisBlockOffset_t
    }

    namespace ClutterSceneObject_t {
        constexpr uint32_t m_Bounds = 0x0; // AABB_t
        constexpr uint32_t m_flags = 0x18; // ObjectTypeFlags_t
        constexpr uint32_t m_nLayer = 0x1C; // int16
        constexpr uint32_t m_instancePositions = 0x20; // CUtlVector< Vector >
        constexpr uint32_t m_instanceScales = 0x50; // CUtlVector< float32 >
        constexpr uint32_t m_instanceTintSrgb = 0x68; // CUtlVector< Color >
        constexpr uint32_t m_tiles = 0x80; // CUtlVector< ClutterTile_t >
        constexpr uint32_t m_renderableModel = 0x98; // CStrongHandle< InfoForResourceTypeCModel >
    }

    namespace ClutterTile_t {
        constexpr uint32_t m_nFirstInstance = 0x0; // uint32
        constexpr uint32_t m_nLastInstance = 0x4; // uint32
        constexpr uint32_t m_BoundsWs = 0x8; // AABB_t
    }

    namespace EntityIOConnectionData_t {
        constexpr uint32_t m_outputName = 0x0; // CUtlString
        constexpr uint32_t m_targetType = 0x8; // uint32
        constexpr uint32_t m_targetName = 0x10; // CUtlString
        constexpr uint32_t m_inputName = 0x18; // CUtlString
        constexpr uint32_t m_overrideParam = 0x20; // CUtlString
        constexpr uint32_t m_flDelay = 0x28; // float32
        constexpr uint32_t m_nTimesToFire = 0x2C; // int32
    }

    namespace EntityKeyValueData_t {
        constexpr uint32_t m_connections = 0x8; // CUtlVector< EntityIOConnectionData_t >
        constexpr uint32_t m_keyValuesData = 0x20; // CUtlBinaryBlock
    }

    namespace ExtraVertexStreamOverride_t {
        constexpr uint32_t m_nSubSceneObject = 0x4; // uint32
        constexpr uint32_t m_nDrawCallIndex = 0x8; // uint32
        constexpr uint32_t m_nAdditionalMeshDrawPrimitiveFlags = 0xC; // MeshDrawPrimitiveFlags_t
        constexpr uint32_t m_extraBufferBinding = 0x10; // CRenderBufferBinding
    }

    namespace InfoOverlayData_t {
        constexpr uint32_t m_transform = 0x0; // matrix3x4_t
        constexpr uint32_t m_flWidth = 0x30; // float32
        constexpr uint32_t m_flHeight = 0x34; // float32
        constexpr uint32_t m_flDepth = 0x38; // float32
        constexpr uint32_t m_vUVStart = 0x3C; // Vector2D
        constexpr uint32_t m_vUVEnd = 0x44; // Vector2D
        constexpr uint32_t m_pMaterial = 0x50; // CStrongHandle< InfoForResourceTypeIMaterial2 >
        constexpr uint32_t m_nRenderOrder = 0x58; // int32
        constexpr uint32_t m_vTintColor = 0x5C; // Vector4D
        constexpr uint32_t m_nSequenceOverride = 0x6C; // int32
    }

    namespace MaterialOverride_t {
        constexpr uint32_t m_nSubSceneObject = 0x4; // uint32
        constexpr uint32_t m_nDrawCallIndex = 0x8; // uint32
        constexpr uint32_t m_pMaterial = 0x10; // CStrongHandle< InfoForResourceTypeIMaterial2 >
    }

    namespace NodeData_t {
        constexpr uint32_t m_nParent = 0x0; // int32
        constexpr uint32_t m_vOrigin = 0x4; // Vector
        constexpr uint32_t m_vMinBounds = 0x10; // Vector
        constexpr uint32_t m_vMaxBounds = 0x1C; // Vector
        constexpr uint32_t m_flMinimumDistance = 0x28; // float32
        constexpr uint32_t m_ChildNodeIndices = 0x30; // CUtlVector< int32 >
        constexpr uint32_t m_worldNodePrefix = 0x48; // CUtlString
    }

    namespace PermEntityLumpData_t {
        constexpr uint32_t m_name = 0x8; // CUtlString
        constexpr uint32_t m_hammerUniqueId = 0x10; // CUtlString
        constexpr uint32_t m_childLumps = 0x18; // CUtlVector< CStrongHandleCopyable< InfoForResourceTypeCEntityLump > >
        constexpr uint32_t m_entityKeyValues = 0x30; // CUtlLeanVector< EntityKeyValueData_t >
    }

    namespace SceneObject_t {
        constexpr uint32_t m_nObjectID = 0x0; // uint32
        constexpr uint32_t m_vTransform = 0x4; // Vector4D[3]
        constexpr uint32_t m_flFadeStartDistance = 0x34; // float32
        constexpr uint32_t m_flFadeEndDistance = 0x38; // float32
        constexpr uint32_t m_vTintColor = 0x3C; // Vector4D
        constexpr uint32_t m_skin = 0x50; // CUtlString
        constexpr uint32_t m_nObjectTypeFlags = 0x58; // ObjectTypeFlags_t
        constexpr uint32_t m_vLightingOrigin = 0x5C; // Vector
        constexpr uint32_t m_nLightGroup = 0x68; // uint32
        constexpr uint32_t m_nOverlayRenderOrder = 0x6C; // int16
        constexpr uint32_t m_nLODOverride = 0x6E; // int16
        constexpr uint32_t m_nCubeMapPrecomputedHandshake = 0x70; // int32
        constexpr uint32_t m_nLightProbeVolumePrecomputedHandshake = 0x74; // int32
        constexpr uint32_t m_renderableModel = 0x80; // CStrongHandle< InfoForResourceTypeCModel >
        constexpr uint32_t m_renderable = 0x88; // CStrongHandle< InfoForResourceTypeCRenderMesh >
    }

    namespace VoxelVisBlockOffset_t {
        constexpr uint32_t m_nOffset = 0x0; // uint32
        constexpr uint32_t m_nElementCount = 0x4; // uint32
    }

    namespace WorldBuilderParams_t {
        constexpr uint32_t m_flMinDrawVolumeSize = 0x0; // float32
        constexpr uint32_t m_bBuildBakedLighting = 0x4; // bool
        constexpr uint32_t m_vLightmapUvScale = 0x8; // Vector2D
        constexpr uint32_t m_nCompileTimestamp = 0x10; // uint64
        constexpr uint32_t m_nCompileFingerprint = 0x18; // uint64
    }

    namespace WorldNodeOnDiskBufferData_t {
        constexpr uint32_t m_nElementCount = 0x0; // int32
        constexpr uint32_t m_nElementSizeInBytes = 0x4; // int32
        constexpr uint32_t m_inputLayoutFields = 0x8; // CUtlVector< RenderInputLayoutField_t >
        constexpr uint32_t m_pData = 0x20; // CUtlVector< uint8 >
    }

    namespace WorldNode_t {
        constexpr uint32_t m_sceneObjects = 0x0; // CUtlVector< SceneObject_t >
        constexpr uint32_t m_infoOverlays = 0x18; // CUtlVector< InfoOverlayData_t >
        constexpr uint32_t m_visClusterMembership = 0x30; // CUtlVector< uint16 >
        constexpr uint32_t m_aggregateSceneObjects = 0x48; // CUtlVector< AggregateSceneObject_t >
        constexpr uint32_t m_clutterSceneObjects = 0x60; // CUtlVector< ClutterSceneObject_t >
        constexpr uint32_t m_extraVertexStreamOverrides = 0x78; // CUtlVector< ExtraVertexStreamOverride_t >
        constexpr uint32_t m_materialOverrides = 0x90; // CUtlVector< MaterialOverride_t >
        constexpr uint32_t m_extraVertexStreams = 0xA8; // CUtlVector< WorldNodeOnDiskBufferData_t >
        constexpr uint32_t m_layerNames = 0xC0; // CUtlVector< CUtlString >
        constexpr uint32_t m_sceneObjectLayerIndices = 0xD8; // CUtlVector< uint8 >
        constexpr uint32_t m_overlayLayerIndices = 0xF0; // CUtlVector< uint8 >
        constexpr uint32_t m_grassFileName = 0x108; // CUtlString
        constexpr uint32_t m_nodeLightingInfo = 0x110; // BakedLightingInfo_t
    }

    namespace World_t {
        constexpr uint32_t m_builderParams = 0x0; // WorldBuilderParams_t
        constexpr uint32_t m_worldNodes = 0x20; // CUtlVector< NodeData_t >
        constexpr uint32_t m_worldLightingInfo = 0x38; // BakedLightingInfo_t
        constexpr uint32_t m_entityLumps = 0x68; // CUtlVector< CStrongHandleCopyable< InfoForResourceTypeCEntityLump > >
    }
}