# Generated using https://github.com/ikhsanprasetyo/source2-dumper
# 2026-09-24 17:35:37.601127800 +07:00

class Schemas:
    # Module: resourcesystem.dll
    class ResourcesystemDll:
        class FuseVariableType_t:
            INVALID = 0x0
            BOOL = 0x1
            INT8 = 0x2
            INT16 = 0x3
            INT32 = 0x4
            UINT8 = 0x5
            UINT16 = 0x6
            UINT32 = 0x7
            FLOAT32 = 0x8
        class FuseVariableAccess_t:
            WRITABLE = 0x0
            READ_ONLY = 0x1
        class AABBWS_t:
            m_vMinBounds = 0x0 # VectorWS
            m_vMaxBounds = 0xC # VectorWS
        class PackedAABB_t:
            m_nPackedMin = 0x0 # uint32
            m_nPackedMax = 0x4 # uint32
        class AABB_t:
            m_vMinBounds = 0x0 # Vector
            m_vMaxBounds = 0xC # Vector
        class FourQuaternions:
            x = 0x0 # fltx4
            y = 0x10 # fltx4
            z = 0x20 # fltx4
            w = 0x30 # fltx4
