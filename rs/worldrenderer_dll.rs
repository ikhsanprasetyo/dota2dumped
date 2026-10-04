// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-04 12:25:34.881424700 +07:00

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]

pub mod source2_dumper {
    pub mod schemas {
        // Module: worldrenderer.dll
        // Class count: 17
        // Enum count: 3
        pub mod worldrenderer_dll {
            // Alignment: 1
            // Member count: 2
            #[repr(u8)]
            pub enum RTProxyInstanceFlags_t {
                RTPROXY_INSTANCE_FLAG_NONE = 0x0,
                RTPROXY_INSTANCE_UNIQUE_MESH = 0x1
            }
            // Alignment: 4
            // Member count: 16
            #[repr(u32)]
            pub enum ObjectTypeFlags_t {
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
                OBJECT_TYPE_HAS_AGGREGATE_RTPROXY = 0x80000
            }
            // Alignment: 1
            // Member count: 4
            #[repr(u8)]
            pub enum AggregateInstanceStream_t {
                AGGREGATE_INSTANCE_STREAM_NONE = 0x0,
                AGGREGATE_INSTANCE_STREAM_LIGHTMAPUV_UNORM16 = 0x1,
                AGGREGATE_INSTANCE_STREAM_VERTEXTINT_UNORM8 = 0x2,
                AGGREGATE_INSTANCE_STREAM_VERTEXBLEND_UNORM8 = 0x4
            }
            // Parent: None
            // Field count: 3
            pub mod CEntityInstance {
                pub const m_iszPrivateVScripts: usize = 0x8; // CUtlSymbolLarge
                pub const m_pEntity: usize = 0x10; // CEntityIdentity*
                pub const m_CScriptComponent: usize = 0x28; // CScriptComponent*
            }
            // Parent: None
            // Field count: 0
            pub mod CEntityComponent {
            }
            // Parent: None
            // Field count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod RTProxyInstanceInfo_t {
                pub const m_nFlags: usize = 0x0; // RTProxyInstanceFlags_t
                pub const m_albedoFormat: usize = 0x1; // VertexAlbedoFormat_t
                pub const m_emissiveFormat: usize = 0x2; // VertexAlbedoFormat_t
                pub const m_nBLASCount: usize = 0x4; // uint16
                pub const m_nBLASIndex: usize = 0x8; // uint32
                pub const m_nVertexAlbedoByteOffset: usize = 0xC; // uint32
                pub const m_nVertexEmissiveByteOffset: usize = 0x10; // uint32
                pub const m_fEmissiveFactor: usize = 0x14; // float32
                pub const m_mWorldFromLocal: usize = 0x18; // matrix3x4_t
            }
            // Parent: None
            // Field count: 14
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod SceneObject_t {
                pub const m_nObjectID: usize = 0x0; // uint32
                pub const m_vTransform: usize = 0x4; // Vector4D[3]
                pub const m_flFadeStartDistance: usize = 0x34; // float32
                pub const m_flFadeEndDistance: usize = 0x38; // float32
                pub const m_vTintColor: usize = 0x3C; // Vector4D
                pub const m_skin: usize = 0x50; // CUtlString
                pub const m_nObjectTypeFlags: usize = 0x58; // ObjectTypeFlags_t
                pub const m_vLightingOrigin: usize = 0x5C; // Vector
                pub const m_nOverlayRenderOrder: usize = 0x68; // int16
                pub const m_nLODOverride: usize = 0x6A; // int16
                pub const m_nCubeMapPrecomputedHandshake: usize = 0x6C; // int32
                pub const m_nLightProbeVolumePrecomputedHandshake: usize = 0x70; // int32
                pub const m_renderableModel: usize = 0x78; // CStrongHandle<InfoForResourceTypeCModel>
                pub const m_renderable: usize = 0x80; // CStrongHandle<InfoForResourceTypeCRenderMesh>
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // OBJECT_TYPE_MODEL
            // OBJECT_TYPE_BLOCK_LIGHT
            pub mod AggregateLODSetup_t {
                pub const m_vLODOrigin: usize = 0x0; // Vector
                pub const m_fMaxObjectScale: usize = 0xC; // float32
                pub const m_fSwitchDistances: usize = 0x10; // CUtlVector<float32>
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
            pub mod AggregateSceneObject_t {
                pub const m_allFlags: usize = 0x0; // ObjectTypeFlags_t
                pub const m_anyFlags: usize = 0x4; // ObjectTypeFlags_t
                pub const m_nLayer: usize = 0x8; // int16
                pub const m_instanceStream: usize = 0xA; // int16
                pub const m_vertexAlbedoStream: usize = 0xC; // int16
                pub const m_vertexEmissiveStream: usize = 0xE; // int16
                pub const m_aggregateMeshes: usize = 0x10; // CUtlVector<AggregateMeshInfo_t>
                pub const m_lodSetups: usize = 0x28; // CUtlVector<AggregateLODSetup_t>
                pub const m_visClusterMembership: usize = 0x40; // CUtlVector<uint16>
                pub const m_fragmentTransforms: usize = 0x58; // CUtlVector<matrix3x4_t>
                pub const m_renderableModel: usize = 0x70; // CStrongHandle<InfoForResourceTypeCModel>
            }
            // Parent: None
            // Field count: 0
            pub mod VMapResourceData_t {
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // OBJECT_TYPE_MODEL
            pub mod AggregateInstanceStreamOnDiskData_t {
                pub const m_DecodedSize: usize = 0x0; // uint32
                pub const m_BufferData: usize = 0x8; // CUtlBinaryBlock
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
            // OBJECT_TYPE_MODEL
            pub mod ClutterSceneObject_t {
                pub const m_Bounds: usize = 0x0; // AABB_t
                pub const m_flags: usize = 0x18; // ObjectTypeFlags_t
                pub const m_nLayer: usize = 0x1C; // int16
                pub const m_instancePositions: usize = 0x20; // CUtlVector<Vector>
                pub const m_instanceScales: usize = 0x50; // CUtlVector<float32>
                pub const m_instanceTintSrgb: usize = 0x68; // CUtlVector<Color>
                pub const m_tiles: usize = 0x80; // CUtlVector<ClutterTile_t>
                pub const m_renderableModel: usize = 0x98; // CStrongHandle<InfoForResourceTypeCModel>
                pub const m_materialGroup: usize = 0xA0; // CUtlStringToken
                pub const m_flBeginCullSize: usize = 0xA4; // float32
                pub const m_flEndCullSize: usize = 0xA8; // float32
            }
            // Parent: None
            // Field count: 16
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // AGGREGATE_INSTANCE_STREAM_LIGHTMAPUV_UNORM16
            pub mod WorldNode_t {
                pub const m_sceneObjects: usize = 0x0; // CUtlVector<SceneObject_t>
                pub const m_visClusterMembership: usize = 0x18; // CUtlVector<uint16>
                pub const m_aggregateSceneObjects: usize = 0x30; // CUtlVector<AggregateSceneObject_t>
                pub const m_clutterSceneObjects: usize = 0x48; // CUtlVector<ClutterSceneObject_t>
                pub const m_rtProxies: usize = 0x60; // CUtlVector<AggregateRTProxySceneObject_t>
                pub const m_extraVertexStreamOverrides: usize = 0x78; // CUtlVector<ExtraVertexStreamOverride_t>
                pub const m_materialOverrides: usize = 0x90; // CUtlVector<MaterialOverride_t>
                pub const m_extraVertexStreams: usize = 0xA8; // CUtlVector<WorldNodeOnDiskBufferData_t>
                pub const m_aggregateInstanceStreams: usize = 0xC0; // CUtlVector<AggregateInstanceStreamOnDiskData_t>
                pub const m_vertexAlbedoStreams: usize = 0xD8; // CUtlVector<AggregateVertexAlbedoStreamOnDiskData_t>
                pub const m_vertexEmissiveStreams: usize = 0xF0; // CUtlVector<AggregateVertexEmissiveStreamOnDiskData_t>
                pub const m_layerNames: usize = 0x108; // CUtlVector<CUtlString>
                pub const m_sceneObjectLayerIndices: usize = 0x120; // CUtlVector<uint8>
                pub const m_grassFileName: usize = 0x138; // CUtlString
                pub const m_nodeLightingInfo: usize = 0x140; // BakedLightingInfo_t
                pub const m_bHasBakedGeometryFlag: usize = 0x188; // bool
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod BaseSceneObjectOverride_t {
                pub const m_nSceneObjectIndex: usize = 0x0; // uint32
            }
            // Parent: None
            // Field count: 11
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // AGGREGATE_INSTANCE_STREAM_LIGHTMAPUV_UNORM16
            // AGGREGATE_INSTANCE_STREAM_VERTEXTINT_UNORM8
            pub mod BakedLightingInfo_t {
                pub const m_nLightmapVersionNumber: usize = 0x0; // uint32
                pub const m_nLightmapGameVersionNumber: usize = 0x4; // uint32
                pub const m_vLightmapUvScale: usize = 0x8; // Vector2D
                pub const m_bHasLightmaps: usize = 0x10; // bool
                pub const m_bBakedShadowsGamma20: usize = 0x11; // bool
                pub const m_bCompressionEnabled: usize = 0x12; // bool
                pub const m_bSHLightmaps: usize = 0x13; // bool
                pub const m_nChartPackIterations: usize = 0x14; // uint8
                pub const m_nVradQuality: usize = 0x15; // uint8
                pub const m_lightMaps: usize = 0x18; // CUtlVector<CStrongHandle<InfoForResourceTypeCTextureBase>>
                pub const m_bakedShadows: usize = 0x30; // CUtlVector<BakedLightingInfo_t::BakedShadowAssignment_t>
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod VoxelVisBlockOffset_t {
                pub const m_nOffset: usize = 0x0; // uint32
                pub const m_nElementCount: usize = 0x4; // uint32
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // OBJECT_TYPE_MODEL
            // OBJECT_TYPE_BLOCK_LIGHT
            pub mod WorldNodeOnDiskBufferData_t {
                pub const m_nElementCount: usize = 0x0; // int32
                pub const m_nElementSizeInBytes: usize = 0x4; // int32
                pub const m_inputLayoutFields: usize = 0x8; // CUtlVector<RenderInputLayoutField_t>
                pub const m_pData: usize = 0x20; // CUtlVector<uint8>
            }
            // Parent: None
            // Field count: 14
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod AggregateMeshInfo_t {
                pub const m_nVisClusterMemberOffset: usize = 0x0; // uint32
                pub const m_nVisClusterMemberCount: usize = 0x4; // uint8
                pub const m_bHasTransform: usize = 0x5; // bool
                pub const m_nLODGroupMask: usize = 0x6; // uint8
                pub const m_nDrawCallIndex: usize = 0x8; // int16
                pub const m_nLODSetupIndex: usize = 0xA; // int16
                pub const m_vTintColor: usize = 0xC; // Color
                pub const m_objectFlags: usize = 0x10; // ObjectTypeFlags_t
                pub const m_nLightProbeVolumePrecomputedHandshake: usize = 0x14; // int32
                pub const m_nInstanceStreamOffset: usize = 0x18; // uint32
                pub const m_nVertexAlbedoStreamOffset: usize = 0x1C; // uint32
                pub const m_nVertexEmissiveStreamOffset: usize = 0x20; // uint32
                pub const m_instanceStreams: usize = 0x24; // AggregateInstanceStream_t
                pub const m_fEmissiveFactor: usize = 0x28; // float32
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // AGGREGATE_INSTANCE_STREAM_LIGHTMAPUV_UNORM16
            pub mod BakedLightingInfo_t__BakedShadowAssignment_t {
                pub const m_nLightHash: usize = 0x0; // uint32
                pub const m_nMapHash: usize = 0x4; // uint32
                pub const m_nShadowChannel: usize = 0x8; // int8
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // AGGREGATE_INSTANCE_STREAM_LIGHTMAPUV_UNORM16
            // AGGREGATE_INSTANCE_STREAM_VERTEXTINT_UNORM8
            pub mod AggregateRTProxySceneObject_t {
                pub const m_nLayer: usize = 0x0; // int16
                pub const m_BLASes: usize = 0x8; // CUtlVector<RTProxyBLAS_t>
                pub const m_Instances: usize = 0x20; // CUtlVector<RTProxyInstanceInfo_t>
                pub const m_VBData: usize = 0x38; // CUtlBinaryBlock
                pub const m_IBData: usize = 0x48; // CUtlBinaryBlock
                pub const m_InstanceAlbedoData: usize = 0x58; // CUtlBinaryBlock
                pub const m_InstanceEmissiveData: usize = 0x68; // CUtlBinaryBlock
            }
        }
    }
}
