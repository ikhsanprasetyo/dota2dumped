// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-08 20:27:02.884074100 +07:00

pub const source2_dumper = struct {
    pub const offsets = struct {
        // Module: client.dll
        pub const client_dll = struct {
            pub const dwEntityList: usize = 0x65A7068;
            pub const dwGameEntitySystem: usize = 0x65A7068;
            pub const dwGameEntitySystem_highestEntityIndex: usize = 0x20C0;
            pub const dwGlobalVars: usize = 0x5B283D8;
            pub const dwLocalPlayerPawn: usize = 0x5B2F568;
            pub const dwPrediction: usize = 0x5B2F490;
            pub const dwViewMatrix: usize = 0x621EB30;
            pub const dwViewRender: usize = 0x621F668;
        };
        // Module: engine2.dll
        pub const engine2_dll = struct {
            pub const dwBuildNumber: usize = 0x5B80F8;
            pub const dwNetworkGameClient: usize = 0x8B3C60;
            pub const dwNetworkGameClient_clientTickCount: usize = 0x398;
            pub const dwNetworkGameClient_deltaTick: usize = 0x24C;
            pub const dwNetworkGameClient_isBackgroundMap: usize = 0x2C143F;
            pub const dwNetworkGameClient_localPlayer: usize = 0xF8;
            pub const dwNetworkGameClient_maxClients: usize = 0x240;
            pub const dwNetworkGameClient_serverTickCount: usize = 0x24C;
            pub const dwNetworkGameClient_signOnState: usize = 0x230;
            pub const dwWindowHeight: usize = 0x8B8014;
            pub const dwWindowWidth: usize = 0x8B8010;
        };
        // Module: inputsystem.dll
        pub const inputsystem_dll = struct {
            pub const dwInputSystem: usize = 0x46BC0;
        };
        // Module: panorama.dll
        pub const panorama_dll = struct {
        };
        // Module: soundsystem.dll
        pub const soundsystem_dll = struct {
            pub const dwSoundSystem: usize = 0x64AF00;
        };
    };
};
