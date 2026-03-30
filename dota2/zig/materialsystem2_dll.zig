// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-03-30 13:31:39.632920200 +07:00

pub const source2_dumper = struct {
    pub const schemas = struct {
        // Module: materialsystem2.dll
        // Class count: 14
        // Enum count: 5
        pub const materialsystem2_dll = struct {
            // Alignment: 4
            // Member count: 4
            pub const VertJustification_e = enum(u32) {
                VERT_JUSTIFICATION_TOP = 0x0,
                VERT_JUSTIFICATION_CENTER = 0x1,
                VERT_JUSTIFICATION_BOTTOM = 0x2,
                VERT_JUSTIFICATION_NONE = 0x3
            };
            // Alignment: 4
            // Member count: 3
            pub const LayoutPositionType_e = enum(u32) {
                LAYOUTPOSITIONTYPE_VIEWPORT_RELATIVE = 0x0,
                LAYOUTPOSITIONTYPE_FRACTIONAL = 0x1,
                LAYOUTPOSITIONTYPE_NONE = 0x2
            };
            // Alignment: 4
            // Member count: 3
            pub const ViewFadeMode_t = enum(u32) {
                VIEW_FADE_CONSTANT_COLOR = 0x0,
                VIEW_FADE_MODULATE = 0x1,
                VIEW_FADE_MOD2X = 0x2
            };
            // Alignment: 4
            // Member count: 3
            pub const BloomBlendMode_t = enum(u32) {
                BLOOM_BLEND_ADD = 0x0,
                BLOOM_BLEND_SCREEN = 0x1,
                BLOOM_BLEND_BLUR = 0x2
            };
            // Alignment: 4
            // Member count: 4
            pub const HorizJustification_e = enum(u32) {
                HORIZ_JUSTIFICATION_LEFT = 0x0,
                HORIZ_JUSTIFICATION_CENTER = 0x1,
                HORIZ_JUSTIFICATION_RIGHT = 0x2,
                HORIZ_JUSTIFICATION_NONE = 0x3
            };
            // Parent: None
            // Field count: 48
            pub const MaterialParam_t = struct {
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
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
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
            // Parent: materialsystem2
            // Field count: 0
            pub const MaterialParamVector_t = struct {
            };
            // Parent: materialsystem2
            // Field count: 0
            pub const MaterialParamString_t = struct {
            };
            // Parent: None
            // Field count: 48
            pub const PostProcessingResource_t = struct {
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
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
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
            // Field count: 0
            pub const MaterialParamInt_t = struct {
            };
            // Parent: None
            // Field count: 48
            pub const PostProcessingVignetteParameters_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const PostProcessingLocalContrastParameters_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const PostProcessingTonemapParameters_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const PostProcessingFogScatteringParameters_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: ___Y__
            // Field count: 0
            pub const MaterialParamBuffer_t = struct {
            };
            // Parent: None
            // Field count: 48
            pub const MaterialResourceData_t = struct {
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
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
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
            // Field count: 48
            pub const PostProcessingBloomParameters_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const MaterialParamFloat_t = struct {
            };
            // Parent: None
            // Field count: 0
            pub const MaterialParamTexture_t = struct {
            };
        };
    };
};
