// Generated using https://github.com/ikhsanprasetyo/Source2Dumped
// 2026-03-30 05:23:54.809776700 UTC

pub const cs2_dumper = struct {
    pub const schemas = struct {
        // Module: scenesystem.dll
        // Class count: 7
        // Enum count: 5
        pub const scenesystem_dll = struct {
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
            // Field count: 47
            pub const CSSDSMsg_ViewTarget = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0x14C3A0; // 
                pub const @"": usize = 0x740073; // 
                pub const @"": usize = 0x64005C; // 
                pub const @"": usize = 0x5C0065; // 
                pub const @"": usize = 0x6F006D; // 
                pub const @"": usize = 0x530000; // 
                pub const @"": usize = 0x5C0079; // 
                pub const @"": usize = 0x6E006F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x670062; // 
                pub const @"": usize = 0x6C006C; // 
                pub const @"": usize = 0x650074; // 
                pub const @"": usize = 0x61006D; // 
                pub const @"": usize = 0x200061; // 
                pub const @"": usize = 0x5C006E; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x7FF8; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 47
            pub const SceneViewId_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0x14C3A0; // 
                pub const @"": usize = 0x740073; // 
                pub const @"": usize = 0x64005C; // 
                pub const @"": usize = 0x5C0065; // 
                pub const @"": usize = 0x6F006D; // 
                pub const @"": usize = 0x530000; // 
                pub const @"": usize = 0x5C0079; // 
                pub const @"": usize = 0x6E006F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x670062; // 
                pub const @"": usize = 0x6C006C; // 
                pub const @"": usize = 0x650074; // 
                pub const @"": usize = 0x61006D; // 
                pub const @"": usize = 0x200061; // 
                pub const @"": usize = 0x5C006E; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x7FF8; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 47
            pub const CSSDSEndFrameViewInfo = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0x14C3A0; // 
                pub const @"": usize = 0x740073; // 
                pub const @"": usize = 0x64005C; // 
                pub const @"": usize = 0x5C0065; // 
                pub const @"": usize = 0x6F006D; // 
                pub const @"": usize = 0x530000; // 
                pub const @"": usize = 0x5C0079; // 
                pub const @"": usize = 0x6E006F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x670062; // 
                pub const @"": usize = 0x6C006C; // 
                pub const @"": usize = 0x650074; // 
                pub const @"": usize = 0x61006D; // 
                pub const @"": usize = 0x200061; // 
                pub const @"": usize = 0x5C006E; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x7FF8; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 47
            pub const CSSDSMsg_LayerBase = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0x14C3A0; // 
                pub const @"": usize = 0x740073; // 
                pub const @"": usize = 0x64005C; // 
                pub const @"": usize = 0x5C0065; // 
                pub const @"": usize = 0x6F006D; // 
                pub const @"": usize = 0x530000; // 
                pub const @"": usize = 0x5C0079; // 
                pub const @"": usize = 0x6E006F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x670062; // 
                pub const @"": usize = 0x6C006C; // 
                pub const @"": usize = 0x650074; // 
                pub const @"": usize = 0x61006D; // 
                pub const @"": usize = 0x200061; // 
                pub const @"": usize = 0x5C006E; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x7FF8; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 47
            pub const CSSDSMsg_ViewTargetList = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0x14C3A0; // 
                pub const @"": usize = 0x740073; // 
                pub const @"": usize = 0x64005C; // 
                pub const @"": usize = 0x5C0065; // 
                pub const @"": usize = 0x6F006D; // 
                pub const @"": usize = 0x530000; // 
                pub const @"": usize = 0x5C0079; // 
                pub const @"": usize = 0x6E006F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x670062; // 
                pub const @"": usize = 0x6C006C; // 
                pub const @"": usize = 0x650074; // 
                pub const @"": usize = 0x61006D; // 
                pub const @"": usize = 0x200061; // 
                pub const @"": usize = 0x5C006E; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x7FF8; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 47
            pub const CSSDSMsg_ViewRender = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0x14C3A0; // 
                pub const @"": usize = 0x740073; // 
                pub const @"": usize = 0x64005C; // 
                pub const @"": usize = 0x5C0065; // 
                pub const @"": usize = 0x6F006D; // 
                pub const @"": usize = 0x530000; // 
                pub const @"": usize = 0x5C0079; // 
                pub const @"": usize = 0x6E006F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x670062; // 
                pub const @"": usize = 0x6C006C; // 
                pub const @"": usize = 0x650074; // 
                pub const @"": usize = 0x61006D; // 
                pub const @"": usize = 0x200061; // 
                pub const @"": usize = 0x5C006E; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x7FF8; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 47
            pub const CSSDSMsg_EndFrame = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0x14C3A0; // 
                pub const @"": usize = 0x740073; // 
                pub const @"": usize = 0x64005C; // 
                pub const @"": usize = 0x5C0065; // 
                pub const @"": usize = 0x6F006D; // 
                pub const @"": usize = 0x530000; // 
                pub const @"": usize = 0x5C0079; // 
                pub const @"": usize = 0x6E006F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x670062; // 
                pub const @"": usize = 0x6C006C; // 
                pub const @"": usize = 0x650074; // 
                pub const @"": usize = 0x61006D; // 
                pub const @"": usize = 0x200061; // 
                pub const @"": usize = 0x5C006E; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x7FF8; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
        };
    };
};
