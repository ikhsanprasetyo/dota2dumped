// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-09 13:46:49.828411900 +07:00

#pragma once

#include <cstddef>
#include <cstdint>

namespace source2_dumper {
    namespace schemas {
        // Module: worldrenderer.dll
        // Class count: 4
        // Enum count: 0
        namespace worldrenderer_dll {
            // Parent: None
            // Field count: 3
            namespace CEntityInstance {
                constexpr std::ptrdiff_t m_iszPrivateVScripts = 0x8; // CUtlSymbolLarge
                constexpr std::ptrdiff_t m_pEntity = 0x10; // CEntityIdentity*
                constexpr std::ptrdiff_t m_CScriptComponent = 0x28; // CScriptComponent*
            }
            // Parent: None
            // Field count: 0
            namespace CEntityComponent {
            }
            // Parent: None
            // Field count: 0
            namespace VMapResourceData_t {
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace VoxelVisBlockOffset_t {
                constexpr std::ptrdiff_t m_nOffset = 0x0; // uint32
                constexpr std::ptrdiff_t m_nElementCount = 0x4; // uint32
            }
        }
    }
}
