// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-03-30 13:31:39.632920200 +07:00

export const Schemas = {
    rendersystemdx11_dll: {
        RenderPrimitiveType_t: {
            RENDER_PRIM_POINTS: 0x0,
            RENDER_PRIM_LINES: 0x1,
            RENDER_PRIM_LINES_WITH_ADJACENCY: 0x2,
            RENDER_PRIM_LINE_STRIP: 0x3,
            RENDER_PRIM_LINE_STRIP_WITH_ADJACENCY: 0x4,
            RENDER_PRIM_TRIANGLES: 0x5,
            RENDER_PRIM_TRIANGLES_WITH_ADJACENCY: 0x6,
            RENDER_PRIM_TRIANGLE_STRIP: 0x7,
            RENDER_PRIM_TRIANGLE_STRIP_WITH_ADJACENCY: 0x8,
            RENDER_PRIM_INSTANCED_QUADS: 0x9,
            RENDER_PRIM_HETEROGENOUS: 0xA,
            RENDER_PRIM_COMPUTE_SHADER: 0xB,
            RENDER_PRIM_MESH_SHADER: 0xC,
            RENDER_PRIM_TYPE_COUNT: 0xD,
        },
        RenderBufferFlags_t: {
            RENDER_BUFFER_USAGE_NONE: 0x0,
            RENDER_BUFFER_USAGE_VERTEX_BUFFER: 0x1,
            RENDER_BUFFER_USAGE_INDEX_BUFFER: 0x2,
            RENDER_BUFFER_USAGE_SHADER_RESOURCE: 0x4,
            RENDER_BUFFER_USAGE_UNORDERED_ACCESS: 0x8,
            RENDER_BUFFER_BYTEADDRESS_BUFFER: 0x10,
            RENDER_BUFFER_STRUCTURED_BUFFER: 0x20,
            RENDER_BUFFER_UAV_DRAW_INDIRECT_ARGS: 0x100,
            RENDER_BUFFER_ACCELERATION_STRUCTURE: 0x200,
            RENDER_BUFFER_SHADER_BINDING_TABLE: 0x400,
            RENDER_BUFFER_POOL_ALLOCATED: 0x800,
            RENDER_BUFFER_USAGE_CONDITIONAL_RENDERING: 0x1000,
            RENDER_BUFFER_IMMOVABLE_ALLOCATION: 0x2000,
        },
        RsCullMode_t: {
            RS_CULL_NONE: 0x0,
            RS_CULL_BACK: 0x1,
            RS_CULL_FRONT: 0x2,
        },
        RsComparison_t: {
            RS_CMP_NEVER: 0x0,
            RS_CMP_LESS: 0x1,
            RS_CMP_EQUAL: 0x2,
            RS_CMP_LESS_EQUAL: 0x3,
            RS_CMP_GREATER: 0x4,
            RS_CMP_NOT_EQUAL: 0x5,
            RS_CMP_GREATER_EQUAL: 0x6,
            RS_CMP_ALWAYS: 0x7,
        },
        RsFillMode_t: {
            RS_FILL_SOLID: 0x0,
            RS_FILL_WIREFRAME: 0x1,
        },
        RenderMultisampleType_t: {
            RENDER_MULTISAMPLE_INVALID: 0xFFFFFFFFFFFFFFFF,
            RENDER_MULTISAMPLE_NONE: 0x0,
            RENDER_MULTISAMPLE_2X: 0x1,
            RENDER_MULTISAMPLE_4X: 0x2,
            RENDER_MULTISAMPLE_6X: 0x3,
            RENDER_MULTISAMPLE_8X: 0x4,
            RENDER_MULTISAMPLE_16X: 0x5,
            RENDER_MULTISAMPLE_TYPE_COUNT: 0x6,
        },
        InputLayoutVariation_t: {
            INPUT_LAYOUT_VARIATION_DEFAULT: 0x0,
            INPUT_LAYOUT_VARIATION_STREAM1_INSTANCEID: 0x1,
            INPUT_LAYOUT_VARIATION_STREAM1_INSTANCEID_MORPH_VERT_ID: 0x2,
            INPUT_LAYOUT_VARIATION_MAX: 0x3,
        },
        RenderSlotType_t: {
            RENDER_SLOT_INVALID: 0xFFFFFFFFFFFFFFFF,
            RENDER_SLOT_PER_VERTEX: 0x0,
            RENDER_SLOT_PER_INSTANCE: 0x1,
        },
        RsDepthStencilStateDesc_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        SheetSequenceIntegerId_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        RsBlendStateDesc_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        VsInputSignatureElement_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        RsRasterizerStateDesc_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        RsStencilStateDesc_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        VsInputSignature_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        RenderInputLayoutField_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
    },
};
