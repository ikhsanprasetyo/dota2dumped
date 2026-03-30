// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-03-30 13:31:39.632920200 +07:00

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]

pub mod source2_dumper {
    pub mod schemas {
        // Module: materialsystem2.dll
        // Class count: 14
        // Enum count: 5
        pub mod materialsystem2_dll {
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum VertJustification_e {
                VERT_JUSTIFICATION_TOP = 0x0,
                VERT_JUSTIFICATION_CENTER = 0x1,
                VERT_JUSTIFICATION_BOTTOM = 0x2,
                VERT_JUSTIFICATION_NONE = 0x3
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum LayoutPositionType_e {
                LAYOUTPOSITIONTYPE_VIEWPORT_RELATIVE = 0x0,
                LAYOUTPOSITIONTYPE_FRACTIONAL = 0x1,
                LAYOUTPOSITIONTYPE_NONE = 0x2
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum ViewFadeMode_t {
                VIEW_FADE_CONSTANT_COLOR = 0x0,
                VIEW_FADE_MODULATE = 0x1,
                VIEW_FADE_MOD2X = 0x2
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum BloomBlendMode_t {
                BLOOM_BLEND_ADD = 0x0,
                BLOOM_BLEND_SCREEN = 0x1,
                BLOOM_BLEND_BLUR = 0x2
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum HorizJustification_e {
                HORIZ_JUSTIFICATION_LEFT = 0x0,
                HORIZ_JUSTIFICATION_CENTER = 0x1,
                HORIZ_JUSTIFICATION_RIGHT = 0x2,
                HORIZ_JUSTIFICATION_NONE = 0x3
            }
            // Parent: None
            // Field count: 48
            pub mod MaterialParam_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: materialsystem2
            // Field count: 0
            pub mod MaterialParamVector_t {
            }
            // Parent: materialsystem2
            // Field count: 0
            pub mod MaterialParamString_t {
            }
            // Parent: None
            // Field count: 48
            pub mod PostProcessingResource_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod MaterialParamInt_t {
            }
            // Parent: None
            // Field count: 48
            pub mod PostProcessingVignetteParameters_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod PostProcessingLocalContrastParameters_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod PostProcessingTonemapParameters_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod PostProcessingFogScatteringParameters_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: ___Y__
            // Field count: 0
            pub mod MaterialParamBuffer_t {
            }
            // Parent: None
            // Field count: 48
            pub mod MaterialResourceData_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod PostProcessingBloomParameters_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod MaterialParamFloat_t {
            }
            // Parent: None
            // Field count: 0
            pub mod MaterialParamTexture_t {
            }
        }
    }
}
