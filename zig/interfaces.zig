// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-08-08 09:16:18.362997 +07:00

pub const source2_dumper = struct {
    pub const interfaces = struct {
        // Module: animationsystem.dll
        pub const animationsystem_dll = struct {
            pub const AnimationSystemUtils_001: usize = 0x839F50;
            pub const AnimationSystem_001: usize = 0x831E70;
        };
        // Module: client.dll
        pub const client_dll = struct {
            pub const ClientToolsInfo_001: usize = 0x5A81BD0;
            pub const DOTA_CLIENT_GCCLIENT: usize = 0x63591B0;
            pub const GameClientExports001: usize = 0x5A7D8F8;
            pub const LegacyGameUI001: usize = 0x5AEC020;
            pub const PanoramaUIClient001: usize = 0x5B18C60;
            pub const PlayButtonService001: usize = 0x5AF9D08;
            pub const Source2Client002: usize = 0x61AC230;
            pub const Source2ClientConfig001: usize = 0x6154690;
            pub const Source2ClientPrediction001: usize = 0x5A860B0;
            pub const Source2ClientUI001: usize = 0x586F5C0;
        };
        // Module: engine2.dll
        pub const engine2_dll = struct {
            pub const BenchmarkService001: usize = 0x613B60;
            pub const BugBugService001: usize = 0x613C60;
            pub const BugService001: usize = 0x8CC3E0;
            pub const ClientServerEngineLoopService_001: usize = 0x90DA80;
            pub const ClientServerSharedHandleSystem001: usize = 0x90D0B0;
            pub const EngineGameUI001: usize = 0x611400;
            pub const EngineServiceMgr001: usize = 0x90D360;
            pub const GameEventSystemClientV001: usize = 0x90D640;
            pub const GameEventSystemServerV001: usize = 0x90D770;
            pub const GameResourceServiceClientV001: usize = 0x613CA0;
            pub const GameResourceServiceServerV001: usize = 0x613D00;
            pub const GameUIService_001: usize = 0x8CC830;
            pub const HostStateMgr001: usize = 0x614490;
            pub const INETSUPPORT_001: usize = 0x60CA60;
            pub const InputService_001: usize = 0x8CCB20;
            pub const KeyValueCache001: usize = 0x614540;
            pub const MapListService_001: usize = 0x90B990;
            pub const NetworkClientService_001: usize = 0x90BB20;
            pub const NetworkP2PService_001: usize = 0x90BE60;
            pub const NetworkServerService_001: usize = 0x90C010;
            pub const NetworkService_001: usize = 0x613E70;
            pub const RenderService_001: usize = 0x90C280;
            pub const ScreenshotService001: usize = 0x90C540;
            pub const SimpleEngineLoopService_001: usize = 0x6145A0;
            pub const SoundService_001: usize = 0x613EB0;
            pub const Source2EngineToClient001: usize = 0x610D30;
            pub const Source2EngineToClientStringTable001: usize = 0x610D90;
            pub const Source2EngineToServer001: usize = 0x610E08;
            pub const Source2EngineToServerStringTable001: usize = 0x610E30;
            pub const SplitScreenService_001: usize = 0x614190;
            pub const StatsService_001: usize = 0x90C900;
            pub const ToolService_001: usize = 0x614300;
            pub const VENGINE_GAMEUIFUNCS_VERSION005: usize = 0x611490;
            pub const VProfService_001: usize = 0x614340;
        };
        // Module: filesystem_stdio.dll
        pub const filesystem_stdio_dll = struct {
            pub const VAsyncFileSystem2_001: usize = 0x2135F0;
            pub const VFileSystem017: usize = 0x2133B0;
        };
        // Module: host.dll
        pub const host_dll = struct {
            pub const DebugDrawQueueManager001: usize = 0x145000;
            pub const DotaMapUtils001: usize = 0x1450B0;
            pub const GameModelInfo001: usize = 0x145040;
            pub const GameSystem2HostHook: usize = 0x145080;
            pub const HostUtils001: usize = 0x145110;
            pub const PredictionDiffManager001: usize = 0x145320;
            pub const SaveRestoreDataVersion001: usize = 0x145450;
            pub const SinglePlayerSharedMemory001: usize = 0x145480;
            pub const Source2Host001: usize = 0x1454F0;
        };
        // Module: imemanager.dll
        pub const imemanager_dll = struct {
            pub const IMEManager001: usize = 0x36AA0;
        };
        // Module: inputsystem.dll
        pub const inputsystem_dll = struct {
            pub const InputStackSystemVersion001: usize = 0x43E90;
            pub const InputSystemVersion001: usize = 0x45BA0;
        };
        // Module: localize.dll
        pub const localize_dll = struct {
            pub const Localize_001: usize = 0x58100;
        };
        // Module: materialsystem2.dll
        pub const materialsystem2_dll = struct {
            pub const FontManager_001: usize = 0x15CAB0;
            pub const MaterialUtils_001: usize = 0x144F00;
            pub const PostProcessingSystem_001: usize = 0x144E30;
            pub const TextLayout_001: usize = 0x144E90;
            pub const VMaterialSystem2_001: usize = 0x15C700;
        };
        // Module: meshsystem.dll
        pub const meshsystem_dll = struct {
            pub const MeshSystem001: usize = 0x16BDE0;
        };
        // Module: navsystem.dll
        pub const navsystem_dll = struct {
            pub const NavSystem001: usize = 0x128CA0;
        };
        // Module: networksystem.dll
        pub const networksystem_dll = struct {
            pub const FlattenedSerializersVersion001: usize = 0x26C810;
            pub const NetworkMessagesVersion001: usize = 0x294950;
            pub const NetworkSystemVersion001: usize = 0x285F60;
            pub const SerializedEntitiesVersion001: usize = 0x286050;
        };
        // Module: panorama.dll
        pub const panorama_dll = struct {
            pub const PanoramaUIEngine001: usize = 0x50ED60;
        };
        // Module: panorama_text_pango.dll
        pub const panorama_text_pango_dll = struct {
            pub const PanoramaTextServices001: usize = 0x2B89D0;
        };
        // Module: particles.dll
        pub const particles_dll = struct {
            pub const ParticleSystemMgr003: usize = 0x6087C0;
        };
        // Module: pulse_system.dll
        pub const pulse_system_dll = struct {
            pub const IPulseSystem_001: usize = 0x2182B0;
        };
        // Module: rendersystemdx11.dll
        pub const rendersystemdx11_dll = struct {
            pub const RenderDeviceMgr001: usize = 0x42B550;
            pub const RenderUtils_001: usize = 0x42BE30;
            pub const VRenderDeviceMgrBackdoor001: usize = 0x42B5F0;
        };
        // Module: resourcesystem.dll
        pub const resourcesystem_dll = struct {
            pub const ResourceSystem013: usize = 0x81670;
        };
        // Module: scenefilecache.dll
        pub const scenefilecache_dll = struct {
            pub const ResponseRulesCache001: usize = 0x113450;
            pub const SceneFileCache002: usize = 0x113578;
        };
        // Module: scenesystem.dll
        pub const scenesystem_dll = struct {
            pub const RenderingPipelines_001: usize = 0x666F20;
            pub const SceneSystem_002: usize = 0x9104F0;
            pub const SceneUtils_001: usize = 0x667DE0;
        };
        // Module: schemasystem.dll
        pub const schemasystem_dll = struct {
            pub const SchemaSystem_001: usize = 0x75630;
        };
        // Module: server.dll
        pub const server_dll = struct {
            pub const EntitySubclassUtilsV001: usize = 0x4753A00;
            pub const NavGameTest001: usize = 0x49F7220;
            pub const ServerToolsInfo_001: usize = 0x4960288;
            pub const Source2GameClients001: usize = 0x495A440;
            pub const Source2GameDirector001: usize = 0x5038EA0;
            pub const Source2GameEntities001: usize = 0x495FA30;
            pub const Source2Server001: usize = 0x495F880;
            pub const Source2ServerConfig001: usize = 0x4F63C78;
        };
        // Module: soundsystem.dll
        pub const soundsystem_dll = struct {
            pub const SoundBugBugService001_Client: usize = 0x553250;
            pub const SoundOpSystem001: usize = 0x553130;
            pub const SoundOpSystemEdit001: usize = 0x553040;
            pub const SoundSystem001: usize = 0x552AA0;
            pub const VMixEditTool001: usize = 0x59489FF;
        };
        // Module: steamaudio.dll
        pub const steamaudio_dll = struct {
            pub const SteamAudio001: usize = 0x25F510;
        };
        // Module: tier0.dll
        pub const tier0_dll = struct {
            pub const TestScriptMgr001: usize = 0x3997D0;
            pub const VEngineCvar007: usize = 0x3A4470;
            pub const VProcessUtils002: usize = 0x399770;
            pub const VStringTokenSystem001: usize = 0x3CB170;
        };
        // Module: v8system.dll
        pub const v8system_dll = struct {
            pub const Source2V8System001: usize = 0x31770;
        };
        // Module: vphysics2.dll
        pub const vphysics2_dll = struct {
            pub const VPhysics2_Interface_001: usize = 0x439E30;
        };
        // Module: vscript.dll
        pub const vscript_dll = struct {
            pub const VScriptManager010: usize = 0x13B430;
        };
        // Module: worldrenderer.dll
        pub const worldrenderer_dll = struct {
            pub const WorldRendererMgr001: usize = 0x22FD60;
        };
    };
};
