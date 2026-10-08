// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-08 20:27:02.884074100 +07:00

namespace Source2Dumper.Schemas {
    // Module: scenesystem.dll
    // Class count: 17
    // Enum count: 10
    public static class ScenesystemDll {
        // Alignment: 4
        // Member count: 3
        public enum ESceneObjectMeshletVisualization : uint {
            SCENEOBJECT_MESHLET_VIS_NONE = 0x0,
            SCENEOBJECT_MESHLET_VIS_MESHLET = 0x1,
            SCENEOBJECT_MESHLET_VIS_CULLED = 0x2
        }
        // Alignment: 1
        // Member count: 2
        public enum RTProxyInstanceFlags_t : byte {
            RTPROXY_INSTANCE_FLAG_NONE = 0x0,
            RTPROXY_INSTANCE_UNIQUE_MESH = 0x1
        }
        // Alignment: 4
        // Member count: 11
        public enum SceneStatsSections_t : uint {
            SCENE_STATS_NONE = 0x0,
            SCENE_STATS_FRAME = 0x1,
            SCENE_STATS_GEOMETRY = 0x2,
            SCENE_STATS_CULLING = 0x4,
            SCENE_STATS_MATERIALS = 0x8,
            SCENE_STATS_LIGHTING = 0x10,
            SCENE_STATS_RAYTRACING = 0x20,
            SCENE_STATS_INTERNALS = 0x40,
            SCENE_STATS_RENDERDEVICE = 0x80,
            SCENE_STATS_ALL = 0xFF,
            SCENE_STATS_DEFAULT = 0x7F
        }
        // Alignment: 4
        // Member count: 18
        public enum ObjectTypeFlags_t : uint {
            OBJECT_TYPE_NONE = 0x0,
            OBJECT_TYPE_MODEL = 0x8,
            OBJECT_TYPE_BLOCK_LIGHT = 0x10,
            OBJECT_TYPE_NO_SHADOWS = 0x20,
            OBJECT_TYPE_WORLDSPACE_TEXURE_BLEND = 0x40,
            OBJECT_TYPE_DISABLED_IN_LOW_QUALITY = 0x80,
            OBJECT_TYPE_RENDER_WITH_DYNAMIC = 0x200,
            OBJECT_TYPE_RENDER_TO_CUBEMAPS = 0x400,
            OBJECT_TYPE_MODEL_HAS_LODS = 0x800,
            OBJECT_TYPE_OVERLAY = 0x2000,
            OBJECT_TYPE_PRECOMPUTED_VISMEMBERS = 0x4000,
            OBJECT_TYPE_STATIC_CUBE_MAP = 0x8000,
            OBJECT_TYPE_DISABLE_VIS_CULLING = 0x10000,
            OBJECT_TYPE_BAKED_GEOMETRY = 0x20000,
            OBJECT_TYPE_NEEDS_DYNAMIC_SHADOWS = 0x40000,
            OBJECT_TYPE_HAS_AGGREGATE_RTPROXY = 0x80000,
            OBJECT_TYPE_HAS_EMISSIVE_GI = 0x100000,
            OBJECT_TYPE_RT_DYNAMIC_MATERIALS = 0x200000
        }
        // Alignment: 4
        // Member count: 7
        public enum ESceneViewDebugOverlaysListenerDataType_t : uint {
            k_ESceneViewDebugOverlaysListenerDataType_Unknown = 0x0,
            k_ESceneViewDebugOverlaysListenerDataType_Sphere = 0x1,
            k_ESceneViewDebugOverlaysListenerDataType_Capsule = 0x2,
            k_ESceneViewDebugOverlaysListenerDataType_BoxAngles = 0x3,
            k_ESceneViewDebugOverlaysListenerDataType_Line = 0x4,
            k_ESceneViewDebugOverlaysListenerDataType_SolidBoxAngles = 0x5,
            k_ESceneViewDebugOverlaysListenerDataType_Text3D = 0x6
        }
        // Alignment: 4
        // Member count: 4
        public enum ESilhouetteType_t : uint {
            SILHOUETTE_NONE = 0x0,
            SILHOUETTE_LIGHT = 0x1,
            SILHOUETTE_ENVMAP = 0x2,
            SILHOUETTE_LPV = 0x4
        }
        // Alignment: 1
        // Member count: 5
        public enum DisableShadows_t : byte {
            kDisableShadows_None = 0x0,
            kDisableShadows_All = 0x1,
            kDisableShadows_Baked = 0x2,
            kDisableShadows_Realtime = 0x3,
            kDisableShadows_ReallyNone = 0x4
        }
        // Alignment: 1
        // Member count: 5
        public enum AggregateInstanceStream_t : byte {
            AGGREGATE_INSTANCE_STREAM_NONE = 0x0,
            AGGREGATE_INSTANCE_STREAM_LIGHTMAPUV_UNORM16 = 0x1,
            AGGREGATE_INSTANCE_STREAM_VERTEXTINT_UNORM8 = 0x2,
            AGGREGATE_INSTANCE_STREAM_VERTEXBLEND_UNORM8 = 0x4,
            AGGREGATE_INSTANCE_STREAM_COLOR_UNORM8 = 0x8
        }
        // Alignment: 1
        // Member count: 6
        public enum DecalRtEncoding_t : byte {
            kDecalInvalid = 0xFF,
            kDecalMin = 0x0,
            kDecalBlood = 0x0,
            kDecalCloak = 0x1,
            kDecalMax = 0x2,
            kDecalDefault = 0x0
        }
        // Alignment: 4
        // Member count: 6
        public enum ESceneObjectVisualization : uint {
            SCENEOBJECT_VIS_NONE = 0x0,
            SCENEOBJECT_VIS_OBJECT = 0x1,
            SCENEOBJECT_VIS_MATERIAL = 0x2,
            SCENEOBJECT_VIS_TEXTURE_SIZE = 0x3,
            SCENEOBJECT_VIS_LOD = 0x4,
            SCENEOBJECT_VIS_INSTANCING = 0x5
        }
        // Parent: None
        // Field count: 10
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class RTProxyInstanceInfo_t {
            public const nint m_nFlags = 0x0; // RTProxyInstanceFlags_t
            public const nint m_albedoFormat = 0x1; // VertexAlbedoFormat_t
            public const nint m_emissiveFormat = 0x2; // VertexAlbedoFormat_t
            public const nint m_nBLASCount = 0x4; // uint16
            public const nint m_nBLASIndex = 0x8; // uint32
            public const nint m_nVertexAlbedoByteOffset = 0xC; // uint32
            public const nint m_nVertexEmissiveByteOffset = 0x10; // uint32
            public const nint m_fEmissiveFactor = 0x14; // float32
            public const nint m_mWorldFromLocal = 0x18; // matrix3x4_t
            public const nint m_vTintColorSRGB = 0x48; // Color
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
        public static class AggregateVertexAlbedoStreamOnDiskData_t {
            public const nint m_BufferData = 0x0; // CUtlBinaryBlock
        }
        // Parent: None
        // Field count: 15
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class SceneObject_t {
            public const nint m_nObjectID = 0x0; // uint32
            public const nint m_vTransform = 0x4; // Vector4D[3]
            public const nint m_flFadeStartDistance = 0x34; // float32
            public const nint m_flFadeEndDistance = 0x38; // float32
            public const nint m_vTintColor = 0x3C; // Vector4D
            public const nint m_skin = 0x50; // CUtlString
            public const nint m_nObjectTypeFlags = 0x58; // ObjectTypeFlags_t
            public const nint m_vLightingOrigin = 0x5C; // Vector
            public const nint m_nOverlayRenderOrder = 0x68; // int16
            public const nint m_nLODOverride = 0x6A; // int16
            public const nint m_nCubeMapPrecomputedHandshake = 0x6C; // int32
            public const nint m_nLightProbeVolumePrecomputedHandshake = 0x70; // int32
            public const nint m_flEmissiveLightingBoost = 0x74; // float32
            public const nint m_renderableModel = 0x80; // CStrongHandle<InfoForResourceTypeCModel>
            public const nint m_renderable = 0x88; // CStrongHandle<InfoForResourceTypeCRenderMesh>
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
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class AggregateLODSetup_t {
            public const nint m_vLODOrigin = 0x0; // Vector
            public const nint m_fMaxObjectScale = 0xC; // float32
            public const nint m_fSwitchDistances = 0x10; // CUtlVector<float32>
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class ClutterTile_t {
            public const nint m_nFirstInstance = 0x0; // uint32
            public const nint m_nLastInstance = 0x4; // uint32
            public const nint m_BoundsWs = 0x8; // AABB_t
        }
        // Parent: None
        // Field count: 11
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
        public static class AggregateSceneObject_t {
            public const nint m_allFlags = 0x0; // ObjectTypeFlags_t
            public const nint m_anyFlags = 0x4; // ObjectTypeFlags_t
            public const nint m_nLayer = 0x8; // int16
            public const nint m_instanceStream = 0xA; // int16
            public const nint m_vertexAlbedoStream = 0xC; // int16
            public const nint m_vertexEmissiveStream = 0xE; // int16
            public const nint m_aggregateMeshes = 0x10; // CUtlVector<AggregateMeshInfo_t>
            public const nint m_lodSetups = 0x28; // CUtlVector<AggregateLODSetup_t>
            public const nint m_visClusterMembership = 0x40; // CUtlVector<uint16>
            public const nint m_fragmentTransforms = 0x58; // CUtlVector<matrix3x4_t>
            public const nint m_renderableModel = 0x70; // CStrongHandle<InfoForResourceTypeCModel>
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
        public static class AggregateInstanceStreamOnDiskData_t {
            public const nint m_DecodedSize = 0x0; // uint32
            public const nint m_BufferData = 0x8; // CUtlBinaryBlock
        }
        // Parent: None
        // Field count: 9
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class RTProxyBLAS_t {
            public const nint m_nFirstIndex = 0x0; // uint32
            public const nint m_nIndexCount = 0x4; // uint32
            public const nint m_nVBByteOffset = 0x8; // uint32
            public const nint m_nBaseVertex = 0xC; // uint32
            public const nint m_nVertexCount = 0x10; // uint16
            public const nint m_albedoFormat = 0x12; // VertexAlbedoFormat_t
            public const nint m_boundLs = 0x14; // AABB_t
            public const nint m_vVertexOriginLs = 0x2C; // Vector
            public const nint m_vVertexExtentLs = 0x38; // Vector
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
        public static class AggregateVertexEmissiveStreamOnDiskData_t {
            public const nint m_BufferData = 0x0; // CUtlBinaryBlock
        }
        // Parent: None
        // Field count: 11
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
        public static class ClutterSceneObject_t {
            public const nint m_Bounds = 0x0; // AABB_t
            public const nint m_flags = 0x18; // ObjectTypeFlags_t
            public const nint m_nLayer = 0x1C; // int16
            public const nint m_instancePositions = 0x20; // CUtlVector<Vector>
            public const nint m_instanceScales = 0x50; // CUtlVector<float32>
            public const nint m_instanceTintSrgb = 0x68; // CUtlVector<Color>
            public const nint m_tiles = 0x80; // CUtlVector<ClutterTile_t>
            public const nint m_renderableModel = 0x98; // CStrongHandle<InfoForResourceTypeCModel>
            public const nint m_materialGroup = 0xA0; // CUtlStringToken
            public const nint m_flBeginCullSize = 0xA4; // float32
            public const nint m_flEndCullSize = 0xA8; // float32
        }
        // Parent: None
        // Field count: 16
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
        public static class WorldNode_t {
            public const nint m_sceneObjects = 0x0; // CUtlVector<SceneObject_t>
            public const nint m_visClusterMembership = 0x18; // CUtlVector<uint16>
            public const nint m_aggregateSceneObjects = 0x30; // CUtlVector<AggregateSceneObject_t>
            public const nint m_clutterSceneObjects = 0x48; // CUtlVector<ClutterSceneObject_t>
            public const nint m_rtProxies = 0x60; // CUtlVector<AggregateRTProxySceneObject_t>
            public const nint m_extraVertexStreamOverrides = 0x78; // CUtlVector<ExtraVertexStreamOverride_t>
            public const nint m_materialOverrides = 0x90; // CUtlVector<MaterialOverride_t>
            public const nint m_extraVertexStreams = 0xA8; // CUtlVector<WorldNodeOnDiskBufferData_t>
            public const nint m_aggregateInstanceStreams = 0xC0; // CUtlVector<AggregateInstanceStreamOnDiskData_t>
            public const nint m_vertexAlbedoStreams = 0xD8; // CUtlVector<AggregateVertexAlbedoStreamOnDiskData_t>
            public const nint m_vertexEmissiveStreams = 0xF0; // CUtlVector<AggregateVertexEmissiveStreamOnDiskData_t>
            public const nint m_layerNames = 0x108; // CUtlVector<CUtlString>
            public const nint m_sceneObjectLayerIndices = 0x120; // CUtlVector<uint8>
            public const nint m_grassFileName = 0x138; // CUtlString
            public const nint m_nodeLightingInfo = 0x140; // BakedLightingInfo_t
            public const nint m_bHasBakedGeometryFlag = 0x188; // bool
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class BaseSceneObjectOverride_t {
            public const nint m_nSceneObjectIndex = 0x0; // uint32
        }
        // Parent: None
        // Field count: 12
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class BakedLightingInfo_t {
            public const nint m_nLightmapVersionNumber = 0x0; // uint32
            public const nint m_nLightmapGameVersionNumber = 0x4; // uint32
            public const nint m_vLightmapUvScale = 0x8; // Vector2D
            public const nint m_bHasLightmaps = 0x10; // bool
            public const nint m_bBakedShadowsGamma20 = 0x11; // bool
            public const nint m_bCompressionEnabled = 0x12; // bool
            public const nint m_nLPVEncoding = 0x13; // int8
            public const nint m_nLightmapEncoding = 0x14; // int8
            public const nint m_nChartPackIterations = 0x15; // uint8
            public const nint m_nVradQuality = 0x16; // uint8
            public const nint m_lightMaps = 0x18; // CUtlVector<CStrongHandle<InfoForResourceTypeCTextureBase>>
            public const nint m_bakedShadows = 0x30; // CUtlVector<BakedLightingInfo_t::BakedShadowAssignment_t>
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class WorldNodeOnDiskBufferData_t {
            public const nint m_nElementCount = 0x0; // int32
            public const nint m_nElementSizeInBytes = 0x4; // int32
            public const nint m_inputLayoutFields = 0x8; // CUtlVector<RenderInputLayoutField_t>
            public const nint m_pData = 0x20; // CUtlVector<uint8>
        }
        // Parent: None
        // Field count: 14
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class AggregateMeshInfo_t {
            public const nint m_nVisClusterMemberOffset = 0x0; // uint32
            public const nint m_nVisClusterMemberCount = 0x4; // uint8
            public const nint m_bHasTransform = 0x5; // bool
            public const nint m_nLODGroupMask = 0x6; // uint8
            public const nint m_nDrawCallIndex = 0x8; // int16
            public const nint m_nLODSetupIndex = 0xA; // int16
            public const nint m_vTintColor = 0xC; // Color
            public const nint m_objectFlags = 0x10; // ObjectTypeFlags_t
            public const nint m_nLightProbeVolumePrecomputedHandshake = 0x14; // int32
            public const nint m_nInstanceStreamOffset = 0x18; // uint32
            public const nint m_nVertexAlbedoStreamOffset = 0x1C; // uint32
            public const nint m_nVertexEmissiveStreamOffset = 0x20; // uint32
            public const nint m_instanceStreams = 0x24; // AggregateInstanceStream_t
            public const nint m_fEmissiveFactor = 0x28; // float32
        }
        // Parent: None
        // Field count: 3
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class BakedLightingInfo_t__BakedShadowAssignment_t {
            public const nint m_nLightHash = 0x0; // uint32
            public const nint m_nMapHash = 0x4; // uint32
            public const nint m_nShadowChannel = 0x8; // int8
        }
        // Parent: None
        // Field count: 7
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
        public static class AggregateRTProxySceneObject_t {
            public const nint m_nLayer = 0x0; // int16
            public const nint m_BLASes = 0x8; // CUtlVector<RTProxyBLAS_t>
            public const nint m_Instances = 0x20; // CUtlVector<RTProxyInstanceInfo_t>
            public const nint m_VBData = 0x38; // CUtlBinaryBlock
            public const nint m_IBData = 0x48; // CUtlBinaryBlock
            public const nint m_InstanceAlbedoData = 0x58; // CUtlBinaryBlock
            public const nint m_InstanceEmissiveData = 0x68; // CUtlBinaryBlock
        }
    }
}
