// Generated using https://github.com/ikhsanprasetyo/Source2Dumped
// 2026-03-30 05:23:54.809776700 UTC

#pragma once

#include <cstddef>
#include <cstdint>

namespace cs2_dumper {
    namespace schemas {
        // Module: materialsystem2.dll
        // Class count: 8
        // Enum count: 5
        namespace materialsystem2_dll {
            // Alignment: 4
            // Member count: 4
            enum class VertJustification_e : uint32_t {
                VERT_JUSTIFICATION_TOP = 0x0,
                VERT_JUSTIFICATION_CENTER = 0x1,
                VERT_JUSTIFICATION_BOTTOM = 0x2,
                VERT_JUSTIFICATION_NONE = 0x3
            };
            // Alignment: 4
            // Member count: 3
            enum class LayoutPositionType_e : uint32_t {
                LAYOUTPOSITIONTYPE_VIEWPORT_RELATIVE = 0x0,
                LAYOUTPOSITIONTYPE_FRACTIONAL = 0x1,
                LAYOUTPOSITIONTYPE_NONE = 0x2
            };
            // Alignment: 4
            // Member count: 3
            enum class ViewFadeMode_t : uint32_t {
                VIEW_FADE_CONSTANT_COLOR = 0x0,
                VIEW_FADE_MODULATE = 0x1,
                VIEW_FADE_MOD2X = 0x2
            };
            // Alignment: 4
            // Member count: 3
            enum class BloomBlendMode_t : uint32_t {
                BLOOM_BLEND_ADD = 0x0,
                BLOOM_BLEND_SCREEN = 0x1,
                BLOOM_BLEND_BLUR = 0x2
            };
            // Alignment: 4
            // Member count: 4
            enum class HorizJustification_e : uint32_t {
                HORIZ_JUSTIFICATION_LEFT = 0x0,
                HORIZ_JUSTIFICATION_CENTER = 0x1,
                HORIZ_JUSTIFICATION_RIGHT = 0x2,
                HORIZ_JUSTIFICATION_NONE = 0x3
            };
            // Parent: None
            // Field count: 47
            namespace MaterialParam_t {
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
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
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
            // Field count: 47
            namespace PostProcessingResource_t {
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
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
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
            // Field count: 49
            namespace PostProcessingVignetteParameters_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x720061; // 
                constexpr std::ptrdiff_t  = 0x6D006D; // 
                constexpr std::ptrdiff_t  = 0x67005C; // 
                constexpr std::ptrdiff_t  = 0x69006C; // 
                constexpr std::ptrdiff_t  = 0x64002E; // 
                constexpr std::ptrdiff_t  = 0x53005C; // 
                constexpr std::ptrdiff_t  = 0x610065; // 
                constexpr std::ptrdiff_t  = 0x74006F; // 
                constexpr std::ptrdiff_t  = 0x690062; // 
                constexpr std::ptrdiff_t  = 0x53; // 
                constexpr std::ptrdiff_t l = 0xBC16BD86; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x10000A0; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 49
            namespace PostProcessingLocalContrastParameters_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x720061; // 
                constexpr std::ptrdiff_t  = 0x6D006D; // 
                constexpr std::ptrdiff_t  = 0x67005C; // 
                constexpr std::ptrdiff_t  = 0x69006C; // 
                constexpr std::ptrdiff_t  = 0x64002E; // 
                constexpr std::ptrdiff_t  = 0x53005C; // 
                constexpr std::ptrdiff_t  = 0x610065; // 
                constexpr std::ptrdiff_t  = 0x74006F; // 
                constexpr std::ptrdiff_t  = 0x690062; // 
                constexpr std::ptrdiff_t  = 0x53; // 
                constexpr std::ptrdiff_t l = 0xBC16BD86; // 
                constexpr std::ptrdiff_t  = 0x0; // P�~��
                constexpr std::ptrdiff_t  = 0x10000A0; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 49
            namespace PostProcessingTonemapParameters_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x720061; // 
                constexpr std::ptrdiff_t  = 0x6D006D; // 
                constexpr std::ptrdiff_t  = 0x67005C; // 
                constexpr std::ptrdiff_t  = 0x69006C; // 
                constexpr std::ptrdiff_t  = 0x64002E; // 
                constexpr std::ptrdiff_t  = 0x53005C; // 
                constexpr std::ptrdiff_t  = 0x610065; // 
                constexpr std::ptrdiff_t  = 0x74006F; // 
                constexpr std::ptrdiff_t  = 0x690062; // 
                constexpr std::ptrdiff_t  = 0x53; // 
                constexpr std::ptrdiff_t l = 0xBC16BD86; // 
                constexpr std::ptrdiff_t  = 0x0; // ����3�H��$�
                constexpr std::ptrdiff_t  = 0x10000A0; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 49
            namespace PostProcessingFogScatteringParameters_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x720061; // 
                constexpr std::ptrdiff_t  = 0x6D006D; // 
                constexpr std::ptrdiff_t  = 0x67005C; // 
                constexpr std::ptrdiff_t  = 0x69006C; // 
                constexpr std::ptrdiff_t  = 0x64002E; // 
                constexpr std::ptrdiff_t  = 0x53005C; // 
                constexpr std::ptrdiff_t  = 0x610065; // 
                constexpr std::ptrdiff_t  = 0x74006F; // 
                constexpr std::ptrdiff_t  = 0x690062; // 
                constexpr std::ptrdiff_t  = 0x53; // 
                constexpr std::ptrdiff_t l = 0xBC16BD86; // 
                constexpr std::ptrdiff_t  = 0x0; // ����3�H��$�
                constexpr std::ptrdiff_t  = 0x10000A0; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
            // Parent: None
            // Field count: 47
            namespace MaterialResourceData_t {
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
                constexpr std::ptrdiff_t  = 0x14C3A0; // 
                constexpr std::ptrdiff_t  = 0x740073; // 
                constexpr std::ptrdiff_t  = 0x64005C; // 
                constexpr std::ptrdiff_t  = 0x5C0065; // 
                constexpr std::ptrdiff_t  = 0x6F006D; // 
                constexpr std::ptrdiff_t  = 0x530000; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x650074; // 
                constexpr std::ptrdiff_t  = 0x61006D; // 
                constexpr std::ptrdiff_t  = 0x200061; // 
                constexpr std::ptrdiff_t  = 0x5C006E; // 
                constexpr std::ptrdiff_t  = 0x6E0065; // 
                constexpr std::ptrdiff_t  = 0x7FF8; // 
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
            // Field count: 49
            namespace PostProcessingBloomParameters_t {
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x1; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0xFF00; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x5C0079; // 
                constexpr std::ptrdiff_t  = 0x6E006F; // 
                constexpr std::ptrdiff_t  = 0x6D0061; // 
                constexpr std::ptrdiff_t  = 0x670062; // 
                constexpr std::ptrdiff_t  = 0x6C006C; // 
                constexpr std::ptrdiff_t  = 0x720061; // 
                constexpr std::ptrdiff_t  = 0x6D006D; // 
                constexpr std::ptrdiff_t  = 0x67005C; // 
                constexpr std::ptrdiff_t  = 0x69006C; // 
                constexpr std::ptrdiff_t  = 0x64002E; // 
                constexpr std::ptrdiff_t  = 0x53005C; // 
                constexpr std::ptrdiff_t  = 0x610065; // 
                constexpr std::ptrdiff_t  = 0x74006F; // 
                constexpr std::ptrdiff_t  = 0x690062; // 
                constexpr std::ptrdiff_t  = 0x53; // 
                constexpr std::ptrdiff_t l = 0xBC16BD86; // 
                constexpr std::ptrdiff_t  = 0x0; // ����3�H��$�
                constexpr std::ptrdiff_t  = 0x10000A0; // 
                constexpr std::ptrdiff_t  = 0x700073; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x730069; // 
                constexpr std::ptrdiff_t  = 0x720065; // 
                constexpr std::ptrdiff_t  = 0xEE; // 
                constexpr std::ptrdiff_t  = 0x80; // 
                constexpr std::ptrdiff_t  = 0x20004D; // 
                constexpr std::ptrdiff_t  = 0x520002; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x15; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
                constexpr std::ptrdiff_t  = 0x0; // 
            }
        }
    }
}
