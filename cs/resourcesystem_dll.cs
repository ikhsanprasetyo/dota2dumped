// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-10 12:12:06.506827800 +07:00

namespace Source2Dumper.Schemas {
    // Module: resourcesystem.dll
    // Class count: 4
    // Enum count: 5
    public static class ResourcesystemDll {
        // Alignment: 4
        // Member count: 4
        public enum NoiseStreamModifier_t : uint {
            NOISE_STREAM_MODIFIER_NONE = 0x0,
            NOISE_STREAM_MODIFIER_LINES = 0x1,
            NOISE_STREAM_MODIFIER_CLUMPS = 0x2,
            NOISE_STREAM_MODIFIER_RINGS = 0x3
        }
        // Alignment: 4
        // Member count: 6
        public enum NoiseStreamTurbulence_t : uint {
            NOISE_STREAM_TURB_NONE = 0x0,
            NOISE_STREAM_TURB_HIGHLIGHT = 0x1,
            NOISE_STREAM_TURB_FEEDBACK = 0x2,
            NOISE_STREAM_TURB_LOOPY = 0x3,
            NOISE_STREAM_TURB_CONTRAST = 0x4,
            NOISE_STREAM_TURB_ALTERNATE = 0x5
        }
        // Alignment: 4
        // Member count: 5
        public enum NoiseStreamType_t : uint {
            NOISE_STREAM_TYPE_PERLIN = 0x0,
            NOISE_STREAM_TYPE_SIMPLEX = 0x1,
            NOISE_STREAM_TYPE_WORLEY = 0x2,
            NOISE_STREAM_TYPE_CURL = 0x3,
            NOISE_STREAM_TYPE_NONE = 0x4
        }
        // Alignment: 1
        // Member count: 9
        public enum FuseVariableType_t : byte {
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
        public enum FuseVariableAccess_t : byte {
            WRITABLE = 0x0,
            READ_ONLY = 0x1
        }
        // Parent: None
        // Field count: 2
        public static class AABBWS_t {
            public const nint m_vMinBounds = 0x0; // VectorWS
            public const nint m_vMaxBounds = 0xC; // VectorWS
        }
        // Parent: None
        // Field count: 2
        public static class PackedAABB_t {
            public const nint m_nPackedMin = 0x0; // uint32
            public const nint m_nPackedMax = 0x4; // uint32
        }
        // Parent: None
        // Field count: 2
        public static class AABB_t {
            public const nint m_vMinBounds = 0x0; // Vector
            public const nint m_vMaxBounds = 0xC; // Vector
        }
        // Parent: None
        // Field count: 4
        public static class FourQuaternions {
            public const nint x = 0x0; // fltx4
            public const nint y = 0x10; // fltx4
            public const nint z = 0x20; // fltx4
            public const nint w = 0x30; // fltx4
        }
    }
}
