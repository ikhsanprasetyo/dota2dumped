// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-04 12:25:34.881424700 +07:00

pub const source2_dumper = struct {
    pub const schemas = struct {
        // Module: scenesystem.dll
        // Class count: 1
        // Enum count: 6
        pub const scenesystem_dll = struct {
            // Alignment: 4
            // Member count: 3
            pub const ESceneObjectMeshletVisualization = enum(u32) {
                SCENEOBJECT_MESHLET_VIS_NONE = 0x0,
                SCENEOBJECT_MESHLET_VIS_MESHLET = 0x1,
                SCENEOBJECT_MESHLET_VIS_CULLED = 0x2
            };
            // Alignment: 4
            // Member count: 7
            pub const ESceneViewDebugOverlaysListenerDataType_t = enum(u32) {
                k_ESceneViewDebugOverlaysListenerDataType_Unknown = 0x0,
                k_ESceneViewDebugOverlaysListenerDataType_Sphere = 0x1,
                k_ESceneViewDebugOverlaysListenerDataType_Capsule = 0x2,
                k_ESceneViewDebugOverlaysListenerDataType_BoxAngles = 0x3,
                k_ESceneViewDebugOverlaysListenerDataType_Line = 0x4,
                k_ESceneViewDebugOverlaysListenerDataType_SolidBoxAngles = 0x5,
                k_ESceneViewDebugOverlaysListenerDataType_Text3D = 0x6
            };
            // Alignment: 4
            // Member count: 4
            pub const ESilhouetteType_t = enum(u32) {
                SILHOUETTE_NONE = 0x0,
                SILHOUETTE_LIGHT = 0x1,
                SILHOUETTE_ENVMAP = 0x2,
                SILHOUETTE_LPV = 0x4
            };
            // Alignment: 1
            // Member count: 5
            pub const DisableShadows_t = enum(u8) {
                kDisableShadows_None = 0x0,
                kDisableShadows_All = 0x1,
                kDisableShadows_Baked = 0x2,
                kDisableShadows_Realtime = 0x3,
                kDisableShadows_ReallyNone = 0x4
            };
            // Alignment: 1
            // Member count: 6
            pub const DecalRtEncoding_t = enum(u8) {
                kDecalInvalid = 0xFF,
                kDecalMin = 0x0,
                kDecalCloak = 0x1,
                kDecalMax = 0x2
            };
            // Alignment: 4
            // Member count: 6
            pub const ESceneObjectVisualization = enum(u32) {
                SCENEOBJECT_VIS_NONE = 0x0,
                SCENEOBJECT_VIS_OBJECT = 0x1,
                SCENEOBJECT_VIS_MATERIAL = 0x2,
                SCENEOBJECT_VIS_TEXTURE_SIZE = 0x3,
                SCENEOBJECT_VIS_LOD = 0x4,
                SCENEOBJECT_VIS_INSTANCING = 0x5
            };
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
            pub const CSSDSMsg_ViewTargetList = struct {
                pub const m_viewId: usize = 0x0; // SceneViewId_t
                pub const m_ViewName: usize = 0x10; // CUtlString
                pub const m_Targets: usize = 0x18; // CUtlVector<CSSDSMsg_ViewTarget>
            };
        };
    };
};
