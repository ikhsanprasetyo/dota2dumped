// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-10 12:12:06.506827800 +07:00

pub const source2_dumper = struct {
    pub const schemas = struct {
        // Module: resourcesystem.dll
        // Class count: 4
        // Enum count: 5
        pub const resourcesystem_dll = struct {
            // Alignment: 4
            // Member count: 4
            pub const NoiseStreamModifier_t = enum(u32) {
                NOISE_STREAM_MODIFIER_NONE = 0x0,
                NOISE_STREAM_MODIFIER_LINES = 0x1,
                NOISE_STREAM_MODIFIER_CLUMPS = 0x2,
                NOISE_STREAM_MODIFIER_RINGS = 0x3
            };
            // Alignment: 4
            // Member count: 6
            pub const NoiseStreamTurbulence_t = enum(u32) {
                NOISE_STREAM_TURB_NONE = 0x0,
                NOISE_STREAM_TURB_HIGHLIGHT = 0x1,
                NOISE_STREAM_TURB_FEEDBACK = 0x2,
                NOISE_STREAM_TURB_LOOPY = 0x3,
                NOISE_STREAM_TURB_CONTRAST = 0x4,
                NOISE_STREAM_TURB_ALTERNATE = 0x5
            };
            // Alignment: 4
            // Member count: 5
            pub const NoiseStreamType_t = enum(u32) {
                NOISE_STREAM_TYPE_PERLIN = 0x0,
                NOISE_STREAM_TYPE_SIMPLEX = 0x1,
                NOISE_STREAM_TYPE_WORLEY = 0x2,
                NOISE_STREAM_TYPE_CURL = 0x3,
                NOISE_STREAM_TYPE_NONE = 0x4
            };
            // Alignment: 1
            // Member count: 9
            pub const FuseVariableType_t = enum(u8) {
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
            pub const FuseVariableAccess_t = enum(u8) {
                WRITABLE = 0x0,
                READ_ONLY = 0x1
            };
            // Parent: None
            // Field count: 2
            pub const AABBWS_t = struct {
                pub const m_vMinBounds: usize = 0x0; // VectorWS
                pub const m_vMaxBounds: usize = 0xC; // VectorWS
            };
            // Parent: None
            // Field count: 2
            pub const PackedAABB_t = struct {
                pub const m_nPackedMin: usize = 0x0; // uint32
                pub const m_nPackedMax: usize = 0x4; // uint32
            };
            // Parent: None
            // Field count: 2
            pub const AABB_t = struct {
                pub const m_vMinBounds: usize = 0x0; // Vector
                pub const m_vMaxBounds: usize = 0xC; // Vector
            };
            // Parent: None
            // Field count: 4
            pub const FourQuaternions = struct {
                pub const x: usize = 0x0; // fltx4
                pub const y: usize = 0x10; // fltx4
                pub const z: usize = 0x20; // fltx4
                pub const w: usize = 0x30; // fltx4
            };
        };
    };
};
