// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-09 13:46:49.828411900 +07:00

pub const source2_dumper = struct {
    pub const schemas = struct {
        // Module: worldrenderer.dll
        // Class count: 4
        // Enum count: 0
        pub const worldrenderer_dll = struct {
            // Parent: None
            // Field count: 3
            pub const CEntityInstance = struct {
                pub const m_iszPrivateVScripts: usize = 0x8; // CUtlSymbolLarge
                pub const m_pEntity: usize = 0x10; // CEntityIdentity*
                pub const m_CScriptComponent: usize = 0x28; // CScriptComponent*
            };
            // Parent: None
            // Field count: 0
            pub const CEntityComponent = struct {
            };
            // Parent: None
            // Field count: 0
            pub const VMapResourceData_t = struct {
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub const VoxelVisBlockOffset_t = struct {
                pub const m_nOffset: usize = 0x0; // uint32
                pub const m_nElementCount: usize = 0x4; // uint32
            };
        };
    };
};
