// printed at 2024-02-04 18:54:53.074631100 +07:00
#pragma once
#include <cstdint>

namespace Offsets {
    namespace RenderInputLayoutField_t {
        constexpr uint32_t m_pSemanticName = 0x0; // uint8[32]
        constexpr uint32_t m_nSemanticIndex = 0x20; // int32
        constexpr uint32_t m_Format = 0x24; // uint32
        constexpr uint32_t m_nOffset = 0x28; // int32
        constexpr uint32_t m_nSlot = 0x2C; // int32
        constexpr uint32_t m_nSlotType = 0x30; // RenderSlotType_t
        constexpr uint32_t m_nInstanceStepRate = 0x34; // int32
    }

    namespace VsInputSignatureElement_t {
        constexpr uint32_t m_pName = 0x0; // char[64]
        constexpr uint32_t m_pSemantic = 0x40; // char[64]
        constexpr uint32_t m_pD3DSemanticName = 0x80; // char[64]
        constexpr uint32_t m_nD3DSemanticIndex = 0xC0; // int32
    }

    namespace VsInputSignature_t {
        constexpr uint32_t m_elems = 0x0; // CUtlVector< VsInputSignatureElement_t >
    }
}