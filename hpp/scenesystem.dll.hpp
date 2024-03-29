// printed at 2024-02-04 18:54:53.396778500 +07:00
#pragma once
#include <cstdint>

namespace Offsets {
    namespace CSSDSEndFrameViewInfo {
        constexpr uint32_t m_nViewId = 0x0; // uint64
        constexpr uint32_t m_ViewName = 0x8; // CUtlString
    }

    namespace CSSDSMsg_EndFrame {
        constexpr uint32_t m_Views = 0x0; // CUtlVector< CSSDSEndFrameViewInfo >
    }

    namespace CSSDSMsg_LayerBase {
        constexpr uint32_t m_viewId = 0x0; // SceneViewId_t
        constexpr uint32_t m_ViewName = 0x10; // CUtlString
        constexpr uint32_t m_nLayerIndex = 0x18; // int32
        constexpr uint32_t m_nLayerId = 0x20; // uint64
        constexpr uint32_t m_LayerName = 0x28; // CUtlString
        constexpr uint32_t m_displayText = 0x30; // CUtlString
    }

    namespace CSSDSMsg_ViewRender {
        constexpr uint32_t m_viewId = 0x0; // SceneViewId_t
        constexpr uint32_t m_ViewName = 0x10; // CUtlString
    }

    namespace CSSDSMsg_ViewTarget {
        constexpr uint32_t m_Name = 0x0; // CUtlString
        constexpr uint32_t m_TextureId = 0x8; // uint64
        constexpr uint32_t m_nWidth = 0x10; // int32
        constexpr uint32_t m_nHeight = 0x14; // int32
        constexpr uint32_t m_nRequestedWidth = 0x18; // int32
        constexpr uint32_t m_nRequestedHeight = 0x1C; // int32
        constexpr uint32_t m_nNumMipLevels = 0x20; // int32
        constexpr uint32_t m_nDepth = 0x24; // int32
        constexpr uint32_t m_nMultisampleNumSamples = 0x28; // int32
        constexpr uint32_t m_nFormat = 0x2C; // int32
    }

    namespace CSSDSMsg_ViewTargetList {
        constexpr uint32_t m_viewId = 0x0; // SceneViewId_t
        constexpr uint32_t m_ViewName = 0x10; // CUtlString
        constexpr uint32_t m_Targets = 0x18; // CUtlVector< CSSDSMsg_ViewTarget >
    }

    namespace SceneViewId_t {
        constexpr uint32_t m_nViewId = 0x0; // uint64
        constexpr uint32_t m_nFrameCount = 0x8; // uint64
    }
}