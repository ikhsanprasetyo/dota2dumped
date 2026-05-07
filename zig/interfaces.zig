// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-05-07 15:06:58.093220400 +07:00

pub const source2_dumper = struct {
    pub const interfaces = struct {
        // Module: animationsystem.dll
        pub const animationsystem_dll = struct {
            pub const AnimationSystemUtils_001: usize = 0x813D90;
            pub const AnimationSystem_001: usize = 0x80BCB0;
        };
        // Module: client.dll
        pub const client_dll = struct {
            pub const ClientToolsInfo_001: usize = 0x5CA10C0;
            pub const DOTA_CLIENT_GCCLIENT: usize = 0x6523298;
            pub const GameClientExports001: usize = 0x5C9CDE8;
            pub const LegacyGameUI001: usize = 0x5D12300;
            pub const PanoramaUIClient001: usize = 0x5D3CDB0;
            pub const PlayButtonService001: usize = 0x5D1E240;
            pub const Source2Client002: usize = 0x63837A0;
            pub const Source2ClientConfig001: usize = 0x6330390;
            pub const Source2ClientPrediction001: usize = 0x5CA7620;
            pub const Source2ClientUI001: usize = 0x5A72870;
        };
        // Module: engine2.dll
        pub const engine2_dll = struct {
            pub const BenchmarkService001: usize = 0x616A30;
            pub const BugService001: usize = 0x8CED40;
            pub const ClientServerEngineLoopService_001: usize = 0x910440;
            pub const ClientServerSharedHandleSystem001: usize = 0x90F9F0;
            pub const EngineGameUI001: usize = 0x6142D0;
            pub const EngineServiceMgr001: usize = 0x90FD20;
            pub const GameEventSystemClientV001: usize = 0x910000;
            pub const GameEventSystemServerV001: usize = 0x910130;
            pub const GameResourceServiceClientV001: usize = 0x616B30;
            pub const GameResourceServiceServerV001: usize = 0x616B90;
            pub const GameUIService_001: usize = 0x8CF170;
            pub const HostStateMgr001: usize = 0x617370;
            pub const INETSUPPORT_001: usize = 0x610070;
            pub const InputService_001: usize = 0x8CF460;
            pub const KeyValueCache001: usize = 0x617420;
            pub const MapListService_001: usize = 0x90E2D0;
            pub const NetworkClientService_001: usize = 0x90E460;
            pub const NetworkP2PService_001: usize = 0x90E7A0;
            pub const NetworkServerService_001: usize = 0x90E950;
            pub const NetworkService_001: usize = 0x616D00;
            pub const RenderService_001: usize = 0x90EBC0;
            pub const ScreenshotService001: usize = 0x90EE80;
            pub const SimpleEngineLoopService_001: usize = 0x617480;
            pub const SoundService_001: usize = 0x616D40;
            pub const Source2EngineToClient001: usize = 0x613BD0;
            pub const Source2EngineToClientStringTable001: usize = 0x613C30;
            pub const Source2EngineToServer001: usize = 0x613CA8;
            pub const Source2EngineToServerStringTable001: usize = 0x613CD0;
            pub const SplitScreenService_001: usize = 0x617020;
            pub const StatsService_001: usize = 0x90F240;
            pub const ToolService_001: usize = 0x6171E0;
            pub const VENGINE_GAMEUIFUNCS_VERSION005: usize = 0x614360;
            pub const VProfService_001: usize = 0x617220;
        };
        // Module: filesystem_stdio.dll
        pub const filesystem_stdio_dll = struct {
            pub const VAsyncFileSystem2_001: usize = 0x2159E0;
            pub const VFileSystem017: usize = 0x2157A0;
        };
        // Module: host.dll
        pub const host_dll = struct {
            pub const DebugDrawQueueManager001: usize = 0x13FFC0;
            pub const DotaMapUtils001: usize = 0x140070;
            pub const GameModelInfo001: usize = 0x140000;
            pub const GameSystem2HostHook: usize = 0x140040;
            pub const HostUtils001: usize = 0x1400D0;
            pub const PredictionDiffManager001: usize = 0x1402E0;
            pub const SaveRestoreDataVersion001: usize = 0x140410;
            pub const SinglePlayerSharedMemory001: usize = 0x140440;
            pub const Source2Host001: usize = 0x1404B0;
        };
        // Module: imemanager.dll
        pub const imemanager_dll = struct {
            pub const IMEManager001: usize = 0x36B20;
        };
        // Module: inputsystem.dll
        pub const inputsystem_dll = struct {
            pub const InputStackSystemVersion001: usize = 0x40E30;
            pub const InputSystemVersion001: usize = 0x42B50;
        };
        // Module: localize.dll
        pub const localize_dll = struct {
            pub const Localize_001: usize = 0x59180;
        };
        // Module: materialsystem2.dll
        pub const materialsystem2_dll = struct {
            pub const FontManager_001: usize = 0x15DD60;
            pub const MaterialUtils_001: usize = 0x145D40;
            pub const PostProcessingSystem_001: usize = 0x145C50;
            pub const TextLayout_001: usize = 0x145CD0;
            pub const VMaterialSystem2_001: usize = 0x15D650;
        };
        // Module: meshsystem.dll
        pub const meshsystem_dll = struct {
            pub const MeshSystem001: usize = 0x150C20;
        };
        // Module: navsystem.dll
        pub const navsystem_dll = struct {
            pub const NavSystem001: usize = 0x122930;
        };
        // Module: networksystem.dll
        pub const networksystem_dll = struct {
            pub const FlattenedSerializersVersion001: usize = 0x26D700;
            pub const NetworkMessagesVersion001: usize = 0x2958D0;
            pub const NetworkSystemVersion001: usize = 0x286E50;
            pub const SerializedEntitiesVersion001: usize = 0x286F40;
        };
        // Module: panorama.dll
        pub const panorama_dll = struct {
            pub const PanoramaUIEngine001: usize = 0x503D30;
        };
        // Module: panorama_text_pango.dll
        pub const panorama_text_pango_dll = struct {
            pub const PanoramaTextServices001: usize = 0x2B8A40;
        };
        // Module: particles.dll
        pub const particles_dll = struct {
            pub const ParticleSystemMgr003: usize = 0x55A720;
        };
        // Module: pulse_system.dll
        pub const pulse_system_dll = struct {
            pub const IPulseSystem_001: usize = 0x1F45A0;
        };
        // Module: rendersystemdx11.dll
        pub const rendersystemdx11_dll = struct {
            pub const RenderDeviceMgr001: usize = 0x42A530;
            pub const RenderUtils_001: usize = 0x42AE28;
            pub const VRenderDeviceMgrBackdoor001: usize = 0x42A5D0;
        };
        // Module: resourcesystem.dll
        pub const resourcesystem_dll = struct {
            pub const ResourceSystem013: usize = 0x832B0;
        };
        // Module: scenefilecache.dll
        pub const scenefilecache_dll = struct {
            pub const ResponseRulesCache001: usize = 0xF58F0;
            pub const SceneFileCache002: usize = 0xF5A78;
        };
        // Module: scenesystem.dll
        pub const scenesystem_dll = struct {
            pub const RenderingPipelines_001: usize = 0x666B80;
            pub const SceneSystem_002: usize = 0x8DE3F0;
            pub const SceneUtils_001: usize = 0x667A90;
        };
        // Module: schemasystem.dll
        pub const schemasystem_dll = struct {
            pub const SchemaSystem_001: usize = 0x77710;
        };
        // Module: server.dll
        pub const server_dll = struct {
            pub const EntitySubclassUtilsV001: usize = 0x472FEF0;
            pub const NavGameTest001: usize = 0x4A03720;
            pub const ServerToolsInfo_001: usize = 0x4959E20;
            pub const Source2GameClients001: usize = 0x494F4D0;
            pub const Source2GameDirector001: usize = 0x4FE28A0;
            pub const Source2GameEntities001: usize = 0x4959530;
            pub const Source2Server001: usize = 0x4959380;
            pub const Source2ServerConfig001: usize = 0x4F208B8;
        };
        // Module: soundsystem.dll
        pub const soundsystem_dll = struct {
            pub const SoundOpSystem001: usize = 0x5139F0;
            pub const SoundOpSystemEdit001: usize = 0x5138B0;
            pub const SoundSystem001: usize = 0x5133A0;
            pub const VMixEditTool001: usize = 0x594873F;
        };
        // Module: steamaudio.dll
        pub const steamaudio_dll = struct {
            pub const SteamAudio001: usize = 0x25E520;
        };
        // Module: steamclient64.dll
        pub const steamclient64_dll = struct {
            pub const IVALIDATE001: usize = 0x16BE0B8;
            pub const SteamClient006: usize = 0x16BB520;
            pub const SteamClient007: usize = 0x16BB528;
            pub const SteamClient008: usize = 0x16BB530;
            pub const SteamClient009: usize = 0x16BB538;
            pub const SteamClient010: usize = 0x16BB540;
            pub const SteamClient011: usize = 0x16BB548;
            pub const SteamClient012: usize = 0x16BB550;
            pub const SteamClient013: usize = 0x16BB558;
            pub const SteamClient014: usize = 0x16BB560;
            pub const SteamClient015: usize = 0x16BB568;
            pub const SteamClient016: usize = 0x16BB570;
            pub const SteamClient017: usize = 0x16BB578;
            pub const SteamClient018: usize = 0x16BB580;
            pub const SteamClient019: usize = 0x16BB588;
            pub const SteamClient020: usize = 0x16BB590;
            pub const SteamClient021: usize = 0x16BB598;
            pub const SteamClient022: usize = 0x16BB5A0;
            pub const SteamClient023: usize = 0x16BB5A8;
            pub const p2pvoice002: usize = 0x14E627F;
            pub const p2pvoicesingleton002: usize = 0x16960F0;
        };
        // Module: tier0.dll
        pub const tier0_dll = struct {
            pub const TestScriptMgr001: usize = 0x39E6F0;
            pub const VEngineCvar007: usize = 0x3A93B0;
            pub const VProcessUtils002: usize = 0x39E690;
            pub const VStringTokenSystem001: usize = 0x3D00B0;
        };
        // Module: v8system.dll
        pub const v8system_dll = struct {
            pub const Source2V8System001: usize = 0x31730;
        };
        // Module: vphysics2.dll
        pub const vphysics2_dll = struct {
            pub const VPhysics2_Interface_001: usize = 0x40FD60;
        };
        // Module: vscript.dll
        pub const vscript_dll = struct {
            pub const VScriptManager010: usize = 0x13B410;
        };
        // Module: vstdlib_s64.dll
        pub const vstdlib_s64_dll = struct {
            pub const IVALIDATE001: usize = 0x6F990;
            pub const VEngineCvar002: usize = 0x6E070;
        };
        // Module: worldrenderer.dll
        pub const worldrenderer_dll = struct {
            pub const WorldRendererMgr001: usize = 0x225B60;
        };
    };
};
