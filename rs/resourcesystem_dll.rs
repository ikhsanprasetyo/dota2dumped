// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-08 20:27:02.884074100 +07:00

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]

pub mod source2_dumper {
    pub mod schemas {
        // Module: resourcesystem.dll
        // Class count: 4
        // Enum count: 5
        pub mod resourcesystem_dll {
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum NoiseStreamModifier_t {
                NOISE_STREAM_MODIFIER_NONE = 0x0,
                NOISE_STREAM_MODIFIER_LINES = 0x1,
                NOISE_STREAM_MODIFIER_CLUMPS = 0x2,
                NOISE_STREAM_MODIFIER_RINGS = 0x3
            }
            // Alignment: 4
            // Member count: 6
            #[repr(u32)]
            pub enum NoiseStreamTurbulence_t {
                NOISE_STREAM_TURB_NONE = 0x0,
                NOISE_STREAM_TURB_HIGHLIGHT = 0x1,
                NOISE_STREAM_TURB_FEEDBACK = 0x2,
                NOISE_STREAM_TURB_LOOPY = 0x3,
                NOISE_STREAM_TURB_CONTRAST = 0x4,
                NOISE_STREAM_TURB_ALTERNATE = 0x5
            }
            // Alignment: 4
            // Member count: 5
            #[repr(u32)]
            pub enum NoiseStreamType_t {
                NOISE_STREAM_TYPE_PERLIN = 0x0,
                NOISE_STREAM_TYPE_SIMPLEX = 0x1,
                NOISE_STREAM_TYPE_WORLEY = 0x2,
                NOISE_STREAM_TYPE_CURL = 0x3,
                NOISE_STREAM_TYPE_NONE = 0x4
            }
            // Alignment: 1
            // Member count: 9
            #[repr(u8)]
            pub enum FuseVariableType_t {
                INVALID = 0x0,
                BOOL = 0x1,
                INT8 = 0x2,
                INT16 = 0x3,
                INT32 = 0x4,
                UINT8 = 0x5,
                UINT16 = 0x6,
                UINT32 = 0x7,
                FLOAT32 = 0x8
            }
            // Alignment: 1
            // Member count: 2
            #[repr(u8)]
            pub enum FuseVariableAccess_t {
                WRITABLE = 0x0,
                READ_ONLY = 0x1
            }
            // Parent: None
            // Field count: 2
            pub mod AABBWS_t {
                pub const m_vMinBounds: usize = 0x0; // VectorWS
                pub const m_vMaxBounds: usize = 0xC; // VectorWS
            }
            // Parent: None
            // Field count: 2
            pub mod PackedAABB_t {
                pub const m_nPackedMin: usize = 0x0; // uint32
                pub const m_nPackedMax: usize = 0x4; // uint32
            }
            // Parent: None
            // Field count: 2
            pub mod AABB_t {
                pub const m_vMinBounds: usize = 0x0; // Vector
                pub const m_vMaxBounds: usize = 0xC; // Vector
            }
            // Parent: None
            // Field count: 4
            pub mod FourQuaternions {
                pub const x: usize = 0x0; // fltx4
                pub const y: usize = 0x10; // fltx4
                pub const z: usize = 0x20; // fltx4
                pub const w: usize = 0x30; // fltx4
            }
        }
    }
}
