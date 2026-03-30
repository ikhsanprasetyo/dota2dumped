// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-03-30 13:31:39.632920200 +07:00

#pragma once

#include <cstddef>
#include <cstdint>

namespace source2_dumper {
    namespace schemas {
        // Module: scenesystem.dll
        // Class count: 9
        // Enum count: 5
        namespace scenesystem_dll {
            // Alignment: 4
            // Member count: 7
            enum class ESceneViewDebugOverlaysListenerDataType_t : uint32_t {
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
            enum class ESilhouetteType_t : uint32_t {
                SILHOUETTE_NONE = 0x0,
                SILHOUETTE_LIGHT = 0x1,
                SILHOUETTE_ENVMAP = 0x2,
                SILHOUETTE_LPV = 0x4
            };
            // Alignment: 1
            // Member count: 5
            enum class DisableShadows_t : uint8_t {
                kDisableShadows_None = 0x0,
                kDisableShadows_All = 0x1,
                kDisableShadows_Baked = 0x2,
                kDisableShadows_Realtime = 0x3,
                kDisableShadows_ReallyNone = 0x4
            };
            // Alignment: 1
            // Member count: 6
            enum class DecalRtEncoding_t : uint8_t {
                kDecalInvalid = 0xFF,
                kDecalMin = 0x0,
                kDecalBlood = 0x0,
                kDecalCloak = 0x1,
                kDecalMax = 0x2,
                kDecalDefault = 0x0
            };
            // Alignment: 4
            // Member count: 6
            enum class ESceneObjectVisualization : uint32_t {
                SCENEOBJECT_VIS_NONE = 0x0,
                SCENEOBJECT_VIS_OBJECT = 0x1,
                SCENEOBJECT_VIS_MATERIAL = 0x2,
                SCENEOBJECT_VIS_TEXTURE_SIZE = 0x3,
                SCENEOBJECT_VIS_LOD = 0x4,
                SCENEOBJECT_VIS_INSTANCING = 0x5
            };
            // Parent: None
            // Field count: 48
            namespace CSSDSMsg_ViewTarget {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            namespace SceneViewId_t {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            namespace CSSDSEndFrameViewInfo {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace CSSDSMsg_PostLayer {
            }
            // Parent: None
            // Field count: 48
            namespace CSSDSMsg_LayerBase {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            namespace CSSDSMsg_PreLayer {
            }
            // Parent: None
            // Field count: 48
            namespace CSSDSMsg_ViewTargetList {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            namespace CSSDSMsg_ViewRender {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            namespace CSSDSMsg_EndFrame {
                constexpr std::ptrdiff_t  = 0x10120; // 
                constexpr std::ptrdiff_t  = 0x10; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t ` = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEEFFEEFF; // 
                constexpr std::ptrdiff_t  = 0x1000; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10FD0; // �
                constexpr std::ptrdiff_t  = 0x10110; // 
                constexpr std::ptrdiff_t  = 0x10750; // P
                constexpr std::ptrdiff_t  = 0x1FE000; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t P = 0x10330; // 
                constexpr std::ptrdiff_t  = 0x10150; // P
                constexpr std::ptrdiff_t  = 0xE02FA810; // 
                constexpr std::ptrdiff_t  = 0x630069; // 
                constexpr std::ptrdiff_t  = 0x440020; // 
                constexpr std::ptrdiff_t  = 0x6D0072; // 
                constexpr std::ptrdiff_t  = 0x5C0030; // 
                constexpr std::ptrdiff_t  = 0x6F0072; // 
                constexpr std::ptrdiff_t  = 0x6F0073; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x31002E; // 
                constexpr std::ptrdiff_t  = 0x61004F; // 
                constexpr std::ptrdiff_t  = 0x6C0000; // 
                constexpr std::ptrdiff_t  = 0x4D005C; // 
                constexpr std::ptrdiff_t  = 0x730077; // 
                constexpr std::ptrdiff_t  = 0x6F0066; // 
                constexpr std::ptrdiff_t  = 0x2D0036; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x4B; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5D00005D; // 
                constexpr std::ptrdiff_t P = 0x10150; // P
                constexpr std::ptrdiff_t  = 0x6C0070; // 
                constexpr std::ptrdiff_t  = 0x5C; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x3; // 
                constexpr std::ptrdiff_t  = 0x790079; // 
                constexpr std::ptrdiff_t  = 0x70; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x99A46D76; // 
                constexpr std::ptrdiff_t  = 0x1D00001D; // 
                constexpr std::ptrdiff_t  = 0x100F0; // �
                constexpr std::ptrdiff_t � = 0xF000; // 
            }
        }
    }
}
