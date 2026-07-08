// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-07-08 15:27:57.237814500 +07:00

pub const source2_dumper = struct {
    pub const offsets = struct {
        // Module: client.dll
        pub const client_dll = struct {
            pub const dwEntityList: usize = 0x6459060;
            pub const dwGameEntitySystem: usize = 0x6459060;
            pub const dwGameEntitySystem_highestEntityIndex: usize = 0x2090;
            pub const dwGlobalVars: usize = 0x59BEF60;
            pub const dwViewMatrix: usize = 0x60EC800;
            pub const dwViewRender: usize = 0x60EBEC8;
        };
        // Module: engine2.dll
        pub const engine2_dll = struct {
            pub const dwBuildNumber: usize = 0x60C694;
            pub const dwNetworkGameClient: usize = 0x90A540;
            pub const dwNetworkGameClient_clientTickCount: usize = 0x378;
            pub const dwNetworkGameClient_deltaTick: usize = 0x24C;
            pub const dwNetworkGameClient_localPlayer: usize = 0xF8;
            pub const dwNetworkGameClient_maxClients: usize = 0x240;
            pub const dwNetworkGameClient_serverTickCount: usize = 0x24C;
            pub const dwNetworkGameClient_signOnState: usize = 0x230;
            pub const dwWindowHeight: usize = 0x90E90C;
            pub const dwWindowWidth: usize = 0x90E908;
        };
        // Module: inputsystem.dll
        pub const inputsystem_dll = struct {
            pub const dwInputSystem: usize = 0x45BA0;
        };
        // Module: panorama.dll
        pub const panorama_dll = struct {
        };
        // Module: soundsystem.dll
        pub const soundsystem_dll = struct {
            pub const dwSoundSystem_engineViewData: usize = 0x7C;
        };
    };
};
