// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-03-30 13:31:39.632920200 +07:00

pub const source2_dumper = struct {
    pub const schemas = struct {
        // Module: schemasystem.dll
        // Class count: 7
        // Enum count: 2
        pub const schemasystem_dll = struct {
            // Alignment: 1
            // Member count: 82
            pub const fieldtype_t = enum(u8) {
                FIELD_VOID = 0x0,
                FIELD_FLOAT32 = 0x1,
                FIELD_STRING = 0x2,
                FIELD_VECTOR = 0x3,
                FIELD_QUATERNION = 0x4,
                FIELD_INT32 = 0x5,
                FIELD_BOOLEAN = 0x6,
                FIELD_INT16 = 0x7,
                FIELD_CHARACTER = 0x8,
                FIELD_COLOR32 = 0x9,
                FIELD_EMBEDDED = 0xA,
                FIELD_CUSTOM = 0xB,
                FIELD_CLASSPTR = 0xC,
                FIELD_EHANDLE = 0xD,
                FIELD_POSITION_VECTOR = 0xE,
                FIELD_TIME = 0xF,
                FIELD_TICK = 0x10,
                FIELD_SOUNDNAME = 0x11,
                FIELD_INPUT = 0x12,
                FIELD_FUNCTION = 0x13,
                FIELD_VMATRIX = 0x14,
                FIELD_VMATRIX_WORLDSPACE = 0x15,
                FIELD_MATRIX3X4_WORLDSPACE = 0x16,
                FIELD_INTERVAL = 0x17,
                FIELD_UNUSED = 0x18,
                FIELD_VECTOR2D = 0x19,
                FIELD_INT64 = 0x1A,
                FIELD_VECTOR4D = 0x1B,
                FIELD_RESOURCE = 0x1C,
                FIELD_TYPEUNKNOWN = 0x1D,
                FIELD_CSTRING = 0x1E,
                FIELD_HSCRIPT = 0x1F,
                FIELD_VARIANT = 0x20,
                FIELD_UINT64 = 0x21,
                FIELD_FLOAT64 = 0x22,
                FIELD_POSITIVEINTEGER_OR_NULL = 0x23,
                FIELD_HSCRIPT_NEW_INSTANCE = 0x24,
                FIELD_UINT32 = 0x25,
                FIELD_UTLSTRINGTOKEN = 0x26,
                FIELD_QANGLE = 0x27,
                FIELD_NETWORK_ORIGIN_CELL_QUANTIZED_VECTOR = 0x28,
                FIELD_HMATERIAL = 0x29,
                FIELD_HMODEL = 0x2A,
                FIELD_NETWORK_QUANTIZED_VECTOR = 0x2B,
                FIELD_NETWORK_QUANTIZED_FLOAT = 0x2C,
                FIELD_DIRECTION_VECTOR_WORLDSPACE = 0x2D,
                FIELD_QANGLE_WORLDSPACE = 0x2E,
                FIELD_QUATERNION_WORLDSPACE = 0x2F,
                FIELD_HSCRIPT_LIGHTBINDING = 0x30,
                FIELD_V8_VALUE = 0x31,
                FIELD_V8_OBJECT = 0x32,
                FIELD_V8_ARRAY = 0x33,
                FIELD_V8_CALLBACK_INFO = 0x34,
                FIELD_UTLSTRING = 0x35,
                FIELD_NETWORK_ORIGIN_CELL_QUANTIZED_POSITION_VECTOR = 0x36,
                FIELD_HRENDERTEXTURE = 0x37,
                FIELD_HPARTICLESYSTEMDEFINITION = 0x38,
                FIELD_UINT8 = 0x39,
                FIELD_UINT16 = 0x3A,
                FIELD_CTRANSFORM = 0x3B,
                FIELD_CTRANSFORM_WORLDSPACE = 0x3C,
                FIELD_HPOSTPROCESSING = 0x3D,
                FIELD_MATRIX3X4 = 0x3E,
                FIELD_SHIM = 0x3F,
                FIELD_CMOTIONTRANSFORM = 0x40,
                FIELD_CMOTIONTRANSFORM_WORLDSPACE = 0x41,
                FIELD_ATTACHMENT_HANDLE = 0x42,
                FIELD_AMMO_INDEX = 0x43,
                FIELD_CONDITION_ID = 0x44,
                FIELD_AI_SCHEDULE_BITS = 0x45,
                FIELD_MODIFIER_HANDLE = 0x46,
                FIELD_ROTATION_VECTOR = 0x47,
                FIELD_ROTATION_VECTOR_WORLDSPACE = 0x48,
                FIELD_HVDATA = 0x49,
                FIELD_SCALE32 = 0x4A,
                FIELD_STRING_AND_TOKEN = 0x4B,
                FIELD_ENGINE_TIME = 0x4C,
                FIELD_ENGINE_TICK = 0x4D,
                FIELD_WORLD_GROUP_ID = 0x4E,
                FIELD_GLOBALSYMBOL = 0x4F,
                FIELD_HNMGRAPHDEFINITION = 0x50,
                FIELD_TYPECOUNT = 0x51
            };
            // Alignment: 4
            // Member count: 3
            pub const ThreeState_t = enum(u32) {
                TRS_FALSE = 0x0,
                TRS_TRUE = 0x1,
                TRS_NONE = 0x2
            };
            // Parent: None
            // Field count: 45
            pub const InfoForResourceTypeCResourceManifestInternal = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 45
            pub const CSchemaSystemInternalRegistration = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: resourcefile
            // Field count: 0
            pub const CExampleSchemaVData_PolymorphicDerivedA = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CExampleSchemaVData_PolymorphicBase = struct {
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
            // Parent: resourcefile
            // Field count: 0
            pub const CExampleSchemaVData_PolymorphicDerivedB = struct {
            };
            // Parent: None
            // Field count: 45
            pub const ResourceId_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CExampleSchemaVData_Monomorphic = struct {
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
        };
    };
};
