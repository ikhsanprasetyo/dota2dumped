// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-09 13:46:49.828411900 +07:00

export const Schemas = {
    scenesystem_dll: {
        ESceneObjectMeshletVisualization: {
            SCENEOBJECT_MESHLET_VIS_NONE: 0x0,
            SCENEOBJECT_MESHLET_VIS_MESHLET: 0x1,
            SCENEOBJECT_MESHLET_VIS_CULLED: 0x2,
        },
        RTProxyInstanceFlags_t: {
            RTPROXY_INSTANCE_FLAG_NONE: 0x0,
            RTPROXY_INSTANCE_UNIQUE_MESH: 0x1,
        },
        SceneStatsSections_t: {
            SCENE_STATS_NONE: 0x0,
            SCENE_STATS_FRAME: 0x1,
            SCENE_STATS_GEOMETRY: 0x2,
            SCENE_STATS_CULLING: 0x4,
            SCENE_STATS_MATERIALS: 0x8,
            SCENE_STATS_LIGHTING: 0x10,
            SCENE_STATS_RAYTRACING: 0x20,
            SCENE_STATS_INTERNALS: 0x40,
            SCENE_STATS_RENDERDEVICE: 0x80,
            SCENE_STATS_ALL: 0xFF,
            SCENE_STATS_DEFAULT: 0x7F,
        },
        ObjectTypeFlags_t: {
            OBJECT_TYPE_NONE: 0x0,
            OBJECT_TYPE_MODEL: 0x8,
            OBJECT_TYPE_BLOCK_LIGHT: 0x10,
            OBJECT_TYPE_NO_SHADOWS: 0x20,
            OBJECT_TYPE_WORLDSPACE_TEXURE_BLEND: 0x40,
            OBJECT_TYPE_DISABLED_IN_LOW_QUALITY: 0x80,
            OBJECT_TYPE_RENDER_WITH_DYNAMIC: 0x200,
            OBJECT_TYPE_RENDER_TO_CUBEMAPS: 0x400,
            OBJECT_TYPE_MODEL_HAS_LODS: 0x800,
            OBJECT_TYPE_OVERLAY: 0x2000,
            OBJECT_TYPE_PRECOMPUTED_VISMEMBERS: 0x4000,
            OBJECT_TYPE_STATIC_CUBE_MAP: 0x8000,
            OBJECT_TYPE_DISABLE_VIS_CULLING: 0x10000,
            OBJECT_TYPE_BAKED_GEOMETRY: 0x20000,
            OBJECT_TYPE_NEEDS_DYNAMIC_SHADOWS: 0x40000,
            OBJECT_TYPE_HAS_AGGREGATE_RTPROXY: 0x80000,
            OBJECT_TYPE_HAS_EMISSIVE_GI: 0x100000,
            OBJECT_TYPE_RT_DYNAMIC_MATERIALS: 0x200000,
        },
        ESceneViewDebugOverlaysListenerDataType_t: {
            k_ESceneViewDebugOverlaysListenerDataType_Unknown: 0x0,
            k_ESceneViewDebugOverlaysListenerDataType_Sphere: 0x1,
            k_ESceneViewDebugOverlaysListenerDataType_Capsule: 0x2,
            k_ESceneViewDebugOverlaysListenerDataType_BoxAngles: 0x3,
            k_ESceneViewDebugOverlaysListenerDataType_Line: 0x4,
            k_ESceneViewDebugOverlaysListenerDataType_SolidBoxAngles: 0x5,
            k_ESceneViewDebugOverlaysListenerDataType_Text3D: 0x6,
        },
        ESilhouetteType_t: {
            SILHOUETTE_NONE: 0x0,
            SILHOUETTE_LIGHT: 0x1,
            SILHOUETTE_ENVMAP: 0x2,
            SILHOUETTE_LPV: 0x4,
        },
        DisableShadows_t: {
            kDisableShadows_None: 0x0,
            kDisableShadows_All: 0x1,
            kDisableShadows_Baked: 0x2,
            kDisableShadows_Realtime: 0x3,
            kDisableShadows_ReallyNone: 0x4,
        },
        AggregateInstanceStream_t: {
            AGGREGATE_INSTANCE_STREAM_NONE: 0x0,
            AGGREGATE_INSTANCE_STREAM_LIGHTMAPUV_UNORM16: 0x1,
            AGGREGATE_INSTANCE_STREAM_VERTEXTINT_UNORM8: 0x2,
            AGGREGATE_INSTANCE_STREAM_VERTEXBLEND_UNORM8: 0x4,
            AGGREGATE_INSTANCE_STREAM_COLOR_UNORM8: 0x8,
        },
        DecalRtEncoding_t: {
            kDecalInvalid: 0xFF,
            kDecalMin: 0x0,
            kDecalCloak: 0x1,
            kDecalMax: 0x2,
        },
        ESceneObjectVisualization: {
            SCENEOBJECT_VIS_NONE: 0x0,
            SCENEOBJECT_VIS_OBJECT: 0x1,
            SCENEOBJECT_VIS_MATERIAL: 0x2,
            SCENEOBJECT_VIS_TEXTURE_SIZE: 0x3,
            SCENEOBJECT_VIS_LOD: 0x4,
            SCENEOBJECT_VIS_INSTANCING: 0x5,
        },
        RTProxyInstanceInfo_t: {
            m_nFlags: 0x0, // RTProxyInstanceFlags_t
            m_albedoFormat: 0x1, // VertexAlbedoFormat_t
            m_emissiveFormat: 0x2, // VertexAlbedoFormat_t
            m_nBLASCount: 0x4, // uint16
            m_nBLASIndex: 0x8, // uint32
            m_nVertexAlbedoByteOffset: 0xC, // uint32
            m_nVertexEmissiveByteOffset: 0x10, // uint32
            m_fEmissiveFactor: 0x14, // float32
            m_mWorldFromLocal: 0x18, // matrix3x4_t
            m_vTintColorSRGB: 0x48, // Color
        },
        AggregateVertexAlbedoStreamOnDiskData_t: {
            m_BufferData: 0x0, // CUtlBinaryBlock
        },
        SceneObject_t: {
            m_nObjectID: 0x0, // uint32
            m_vTransform: 0x4, // Vector4D[3]
            m_flFadeStartDistance: 0x34, // float32
            m_flFadeEndDistance: 0x38, // float32
            m_vTintColor: 0x3C, // Vector4D
            m_skin: 0x50, // CUtlString
            m_nObjectTypeFlags: 0x58, // ObjectTypeFlags_t
            m_vLightingOrigin: 0x5C, // Vector
            m_nOverlayRenderOrder: 0x68, // int16
            m_nLODOverride: 0x6A, // int16
            m_nCubeMapPrecomputedHandshake: 0x6C, // int32
            m_nLightProbeVolumePrecomputedHandshake: 0x70, // int32
            m_flEmissiveLightingBoost: 0x74, // float32
            m_renderableModel: 0x80, // CStrongHandle<InfoForResourceTypeCModel>
            m_renderable: 0x88, // CStrongHandle<InfoForResourceTypeCRenderMesh>
        },
        AggregateLODSetup_t: {
            m_vLODOrigin: 0x0, // Vector
            m_fMaxObjectScale: 0xC, // float32
            m_fSwitchDistances: 0x10, // CUtlVector<float32>
        },
        ClutterTile_t: {
            m_nFirstInstance: 0x0, // uint32
            m_nLastInstance: 0x4, // uint32
            m_BoundsWs: 0x8, // AABB_t
        },
        AggregateSceneObject_t: {
            m_allFlags: 0x0, // ObjectTypeFlags_t
            m_anyFlags: 0x4, // ObjectTypeFlags_t
            m_nLayer: 0x8, // int16
            m_instanceStream: 0xA, // int16
            m_vertexAlbedoStream: 0xC, // int16
            m_vertexEmissiveStream: 0xE, // int16
            m_aggregateMeshes: 0x10, // CUtlVector<AggregateMeshInfo_t>
            m_lodSetups: 0x28, // CUtlVector<AggregateLODSetup_t>
            m_visClusterMembership: 0x40, // CUtlVector<uint16>
            m_fragmentTransforms: 0x58, // CUtlVector<matrix3x4_t>
            m_renderableModel: 0x70, // CStrongHandle<InfoForResourceTypeCModel>
        },
        AggregateInstanceStreamOnDiskData_t: {
            m_DecodedSize: 0x0, // uint32
            m_BufferData: 0x8, // CUtlBinaryBlock
        },
        RTProxyBLAS_t: {
            m_nFirstIndex: 0x0, // uint32
            m_nIndexCount: 0x4, // uint32
            m_nVBByteOffset: 0x8, // uint32
            m_nBaseVertex: 0xC, // uint32
            m_nVertexCount: 0x10, // uint16
            m_albedoFormat: 0x12, // VertexAlbedoFormat_t
            m_boundLs: 0x14, // AABB_t
            m_vVertexOriginLs: 0x2C, // Vector
            m_vVertexExtentLs: 0x38, // Vector
        },
        AggregateVertexEmissiveStreamOnDiskData_t: {
            m_BufferData: 0x0, // CUtlBinaryBlock
        },
        ClutterSceneObject_t: {
            m_Bounds: 0x0, // AABB_t
            m_flags: 0x18, // ObjectTypeFlags_t
            m_nLayer: 0x1C, // int16
            m_instancePositions: 0x20, // CUtlVector<Vector>
            m_instanceScales: 0x50, // CUtlVector<float32>
            m_instanceTintSrgb: 0x68, // CUtlVector<Color>
            m_tiles: 0x80, // CUtlVector<ClutterTile_t>
            m_renderableModel: 0x98, // CStrongHandle<InfoForResourceTypeCModel>
            m_materialGroup: 0xA0, // CUtlStringToken
            m_flBeginCullSize: 0xA4, // float32
            m_flEndCullSize: 0xA8, // float32
        },
        WorldNode_t: {
            m_sceneObjects: 0x0, // CUtlVector<SceneObject_t>
            m_visClusterMembership: 0x18, // CUtlVector<uint16>
            m_aggregateSceneObjects: 0x30, // CUtlVector<AggregateSceneObject_t>
            m_clutterSceneObjects: 0x48, // CUtlVector<ClutterSceneObject_t>
            m_rtProxies: 0x60, // CUtlVector<AggregateRTProxySceneObject_t>
            m_extraVertexStreamOverrides: 0x78, // CUtlVector<ExtraVertexStreamOverride_t>
            m_materialOverrides: 0x90, // CUtlVector<MaterialOverride_t>
            m_extraVertexStreams: 0xA8, // CUtlVector<WorldNodeOnDiskBufferData_t>
            m_aggregateInstanceStreams: 0xC0, // CUtlVector<AggregateInstanceStreamOnDiskData_t>
            m_vertexAlbedoStreams: 0xD8, // CUtlVector<AggregateVertexAlbedoStreamOnDiskData_t>
            m_vertexEmissiveStreams: 0xF0, // CUtlVector<AggregateVertexEmissiveStreamOnDiskData_t>
            m_layerNames: 0x108, // CUtlVector<CUtlString>
            m_sceneObjectLayerIndices: 0x120, // CUtlVector<uint8>
            m_grassFileName: 0x138, // CUtlString
            m_nodeLightingInfo: 0x140, // BakedLightingInfo_t
            m_bHasBakedGeometryFlag: 0x188, // bool
        },
        BaseSceneObjectOverride_t: {
            m_nSceneObjectIndex: 0x0, // uint32
        },
        BakedLightingInfo_t: {
            m_nLightmapVersionNumber: 0x0, // uint32
            m_nLightmapGameVersionNumber: 0x4, // uint32
            m_vLightmapUvScale: 0x8, // Vector2D
            m_bHasLightmaps: 0x10, // bool
            m_bBakedShadowsGamma20: 0x11, // bool
            m_bCompressionEnabled: 0x12, // bool
            m_nLPVEncoding: 0x13, // int8
            m_nLightmapEncoding: 0x14, // int8
            m_nChartPackIterations: 0x15, // uint8
            m_nVradQuality: 0x16, // uint8
            m_lightMaps: 0x18, // CUtlVector<CStrongHandle<InfoForResourceTypeCTextureBase>>
            m_bakedShadows: 0x30, // CUtlVector<BakedLightingInfo_t::BakedShadowAssignment_t>
        },
        WorldNodeOnDiskBufferData_t: {
            m_nElementCount: 0x0, // int32
            m_nElementSizeInBytes: 0x4, // int32
            m_inputLayoutFields: 0x8, // CUtlVector<RenderInputLayoutField_t>
            m_pData: 0x20, // CUtlVector<uint8>
        },
        AggregateMeshInfo_t: {
            m_nVisClusterMemberOffset: 0x0, // uint32
            m_nVisClusterMemberCount: 0x4, // uint8
            m_bHasTransform: 0x5, // bool
            m_nLODGroupMask: 0x6, // uint8
            m_nDrawCallIndex: 0x8, // int16
            m_nLODSetupIndex: 0xA, // int16
            m_vTintColor: 0xC, // Color
            m_objectFlags: 0x10, // ObjectTypeFlags_t
            m_nLightProbeVolumePrecomputedHandshake: 0x14, // int32
            m_nInstanceStreamOffset: 0x18, // uint32
            m_nVertexAlbedoStreamOffset: 0x1C, // uint32
            m_nVertexEmissiveStreamOffset: 0x20, // uint32
            m_instanceStreams: 0x24, // AggregateInstanceStream_t
            m_fEmissiveFactor: 0x28, // float32
        },
        BakedLightingInfo_t__BakedShadowAssignment_t: {
            m_nLightHash: 0x0, // uint32
            m_nMapHash: 0x4, // uint32
            m_nShadowChannel: 0x8, // int8
        },
        AggregateRTProxySceneObject_t: {
            m_nLayer: 0x0, // int16
            m_BLASes: 0x8, // CUtlVector<RTProxyBLAS_t>
            m_Instances: 0x20, // CUtlVector<RTProxyInstanceInfo_t>
            m_VBData: 0x38, // CUtlBinaryBlock
            m_IBData: 0x48, // CUtlBinaryBlock
            m_InstanceAlbedoData: 0x58, // CUtlBinaryBlock
            m_InstanceEmissiveData: 0x68, // CUtlBinaryBlock
        },
    },
};
