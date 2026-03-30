// Generated using https://github.com/ikhsanprasetyo/Source2Dumped
// 2026-03-30 05:23:54.809776700 UTC

namespace CS2Dumper.Schemas {
    // Module: scenesystem.dll
    // Class count: 7
    // Enum count: 5
    public static class ScenesystemDll {
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
        // Field count: 47
        public static class CSSDSMsg_ViewTarget {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 47
        public static class SceneViewId_t {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 47
        public static class CSSDSEndFrameViewInfo {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 47
        public static class CSSDSMsg_LayerBase {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 47
        public static class CSSDSMsg_ViewTargetList {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 47
        public static class CSSDSMsg_ViewRender {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
        // Parent: None
        // Field count: 47
        public static class CSSDSMsg_EndFrame {
            public const nint  = 0x10120; // 
            public const nint  = 0x10; // 
            public const nint  = 0x0; // 
            public const nint ` = 0x0; // 
            public const nint  = 0xEEFFEEFF; // 
            public const nint  = 0x1000; // 
            public const nint  = 0x0; // 
            public const nint  = 0x10FD0; // �
            public const nint  = 0x10110; // 
            public const nint  = 0x10750; // P
            public const nint  = 0x1FE000; // 
            public const nint  = 0x1; // 
            public const nint P = 0x10330; // 
            public const nint  = 0x10150; // P
            public const nint  = 0x14C3A0; // 
            public const nint  = 0x740073; // 
            public const nint  = 0x64005C; // 
            public const nint  = 0x5C0065; // 
            public const nint  = 0x6F006D; // 
            public const nint  = 0x530000; // 
            public const nint  = 0x5C0079; // 
            public const nint  = 0x6E006F; // 
            public const nint  = 0x6D0061; // 
            public const nint  = 0x670062; // 
            public const nint  = 0x6C006C; // 
            public const nint  = 0x650074; // 
            public const nint  = 0x61006D; // 
            public const nint  = 0x200061; // 
            public const nint  = 0x5C006E; // 
            public const nint  = 0x6E0065; // 
            public const nint  = 0x7FF8; // 
            public const nint  = 0x0; // 
            public const nint  = 0x5D00005D; // 
            public const nint P = 0x10150; // P
            public const nint  = 0x6C0070; // 
            public const nint  = 0x5C; // 
            public const nint  = 0x0; // 
            public const nint  = 0x0; // 
            public const nint  = 0xEE; // 
            public const nint  = 0x3; // 
            public const nint  = 0x790079; // 
            public const nint  = 0x70; // 
            public const nint  = 0x0; // 
            public const nint  = 0x99A46D76; // 
            public const nint  = 0x1D00001D; // 
            public const nint  = 0x100F0; // �
            public const nint � = 0xF000; // 
        }
    }
}
