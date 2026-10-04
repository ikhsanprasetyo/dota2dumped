// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-04 12:25:34.881424700 +07:00

namespace Source2Dumper.Schemas {
    // Module: scenesystem.dll
    // Class count: 1
    // Enum count: 6
    public static class ScenesystemDll {
        // Alignment: 4
        // Member count: 3
        public enum ESceneObjectMeshletVisualization : uint {
            SCENEOBJECT_MESHLET_VIS_NONE = 0x0,
            SCENEOBJECT_MESHLET_VIS_MESHLET = 0x1,
            SCENEOBJECT_MESHLET_VIS_CULLED = 0x2
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
        public static class CSSDSMsg_ViewTargetList {
            public const nint m_viewId = 0x0; // SceneViewId_t
            public const nint m_ViewName = 0x10; // CUtlString
            public const nint m_Targets = 0x18; // CUtlVector<CSSDSMsg_ViewTarget>
        }
    }
}
