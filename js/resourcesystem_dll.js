// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-04 12:25:34.881424700 +07:00

export const Schemas = {
    resourcesystem_dll: {
        FuseVariableType_t: {
            INVALID: 0x0,
            BOOL: 0x1,
            INT8: 0x2,
            INT16: 0x3,
            INT32: 0x4,
            UINT8: 0x5,
            UINT16: 0x6,
            UINT32: 0x7,
            FLOAT32: 0x8,
        },
        FuseVariableAccess_t: {
            WRITABLE: 0x0,
            READ_ONLY: 0x1,
        },
        AABBWS_t: {
            m_vMinBounds: 0x0, // VectorWS
            m_vMaxBounds: 0xC, // VectorWS
        },
        PackedAABB_t: {
            m_nPackedMin: 0x0, // uint32
            m_nPackedMax: 0x4, // uint32
        },
        AABB_t: {
            m_vMinBounds: 0x0, // Vector
            m_vMaxBounds: 0xC, // Vector
        },
        FourQuaternions: {
            x: 0x0, // fltx4
            y: 0x10, // fltx4
            z: 0x20, // fltx4
            w: 0x30, // fltx4
        },
    },
};
