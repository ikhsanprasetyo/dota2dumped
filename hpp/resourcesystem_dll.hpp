// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-04 12:25:34.881424700 +07:00

#pragma once

#include <cstddef>
#include <cstdint>

namespace source2_dumper {
    namespace schemas {
        // Module: resourcesystem.dll
        // Class count: 4
        // Enum count: 2
        namespace resourcesystem_dll {
            // Alignment: 1
            // Member count: 9
            enum class FuseVariableType_t : uint8_t {
                INVALID = 0x0,
                BOOL = 0x1,
                INT8 = 0x2,
                INT16 = 0x3,
                INT32 = 0x4,
                UINT8 = 0x5,
                UINT16 = 0x6,
                UINT32 = 0x7,
                FLOAT32 = 0x8
            };
            // Alignment: 1
            // Member count: 2
            enum class FuseVariableAccess_t : uint8_t {
                WRITABLE = 0x0,
                READ_ONLY = 0x1
            };
            // Parent: None
            // Field count: 2
            namespace AABBWS_t {
                constexpr std::ptrdiff_t m_vMinBounds = 0x0; // VectorWS
                constexpr std::ptrdiff_t m_vMaxBounds = 0xC; // VectorWS
            }
            // Parent: None
            // Field count: 2
            namespace PackedAABB_t {
                constexpr std::ptrdiff_t m_nPackedMin = 0x0; // uint32
                constexpr std::ptrdiff_t m_nPackedMax = 0x4; // uint32
            }
            // Parent: None
            // Field count: 2
            namespace AABB_t {
                constexpr std::ptrdiff_t m_vMinBounds = 0x0; // Vector
                constexpr std::ptrdiff_t m_vMaxBounds = 0xC; // Vector
            }
            // Parent: None
            // Field count: 4
            namespace FourQuaternions {
                constexpr std::ptrdiff_t x = 0x0; // fltx4
                constexpr std::ptrdiff_t y = 0x10; // fltx4
                constexpr std::ptrdiff_t z = 0x20; // fltx4
                constexpr std::ptrdiff_t w = 0x30; // fltx4
            }
        }
    }
}
