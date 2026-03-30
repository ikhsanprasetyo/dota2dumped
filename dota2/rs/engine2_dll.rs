// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-03-30 13:31:39.632920200 +07:00

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]

pub mod source2_dumper {
    pub mod schemas {
        // Module: engine2.dll
        // Class count: 56
        // Enum count: 2
        pub mod engine2_dll {
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum EntityDormancyType_t {
                ENTITY_NOT_DORMANT = 0x0,
                ENTITY_DORMANT = 0x1,
                ENTITY_SUSPENDED = 0x2
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum EntityIOTargetType_t {
                ENTITY_IO_TARGET_INVALID = u32::MAX,
                ENTITY_IO_TARGET_ENTITYNAME = 0x2,
                ENTITY_IO_TARGET_EHANDLE = 0x6,
                ENTITY_IO_TARGET_ENTITYNAME_OR_CLASSNAME = 0x7
            }
            // Parent: None
            // Field count: 45
            pub mod CEntityInstance {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod CEntityComponent {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CScriptComponent {
            }
            // Parent: None
            // Field count: 45
            pub mod CEntityIdentity {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: engine2
            // Field count: 0
            pub mod EventClientPostSimulate_t {
            }
            // Parent: None
            // Field count: 45
            pub mod EventSimpleLoopFrameUpdate_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: engine2
            // Field count: 0
            pub mod EventPostAdvanceTick_t {
            }
            // Parent: None
            // Field count: 45
            pub mod CEntityIOOutput {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod EventClientSceneSystemThreadStateChange_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod EventClientOutput_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: ___Y__
            // Field count: 0
            pub mod EventServerPostSimulate_t {
            }
            // Parent: None
            // Field count: 45
            pub mod CEntityComponentHelper {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod GameTime_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: ___Y__
            // Field count: 0
            pub mod EventServerBeginSimulate_t {
            }
            // Parent: None
            // Field count: 48
            pub mod EntityIOQueuePrioritizedEvent_t {
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
            // Field count: 45
            pub mod EventServerEndAsyncPostTickWork_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: engine2
            // Field count: 0
            pub mod EventClientAdvanceTick_t {
            }
            // Parent: None
            // Field count: 45
            pub mod EntInput_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod CNetworkVarChainer {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: engine2
            // Field count: 0
            pub mod EventClientSimulate_t {
            }
            // Parent: None
            // Field count: 45
            pub mod EventClientPostOutput_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod GameTick_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod EventClientPollInput_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod EventPreDataUpdate_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod EventClientProcessGameInput_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod EventFrameBoundary_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod EventAppShutdown_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: ___Y__
            // Field count: 0
            pub mod EventServerPostAdvanceTick_t {
            }
            // Parent: None
            // Field count: 45
            pub mod EventProfileStorageAvailable_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod EventPostDataUpdate_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: engine2
            // Field count: 0
            pub mod EventClientPreSimulate_t {
            }
            // Parent: ___Y__
            // Field count: 0
            pub mod EventClientPauseSimulate_t {
            }
            // Parent: None
            // Field count: 45
            pub mod EventClientProcessNetworking_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod CEntityAttributeTable {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: ___Y__
            // Field count: 0
            pub mod EventClientPreOutputParallelWithServer_t {
            }
            // Parent: ___Y__
            // Field count: 0
            pub mod EventAdvanceTick_t {
            }
            // Parent: None
            // Field count: 45
            pub mod EventSplitScreenStateChanged_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: engine2
            // Field count: 0
            pub mod EventClientPostAdvanceTick_t {
            }
            // Parent: None
            // Field count: 45
            pub mod CVariantDefaultAllocator {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod EventModInitialized_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod EventClientPreOutput_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod EventClientFrameSimulate_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: ___Y__
            // Field count: 0
            pub mod EventServerAdvanceTick_t {
            }
            // Parent: None
            // Field count: 45
            pub mod EventSetTime_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod EventSimulate_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod CEntityKeyValues {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod EventClientAdvanceNonRenderedFrame_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: engine2
            // Field count: 0
            pub mod EventServerProcessNetworking_t {
            }
            // Parent: None
            // Field count: 45
            pub mod CEmptyEntityInstance {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod EntComponentInfo_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod EngineLoopState_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod EventClientPollNetworking_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod EventServerBeginAsyncPostTickWork_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod EventClientProcessInput_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod EventServerEndSimulate_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: ___Y__
            // Field count: 0
            pub mod EventServerPollNetworking_t {
            }
        }
    }
}
