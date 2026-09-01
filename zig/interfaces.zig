// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-09-01 21:41:54.904199400 +07:00

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
            pub const DOTA_CLIENT_GCCLIENT: usize = 0x63591E0;
            pub const GameClientExports001: usize = 0x5A7D8F8;
            pub const LegacyGameUI001: usize = 0x5AEC020;
            pub const PanoramaUIClient001: usize = 0x5B18C50;
            pub const PlayButtonService001: usize = 0x5AF9CF8;
            pub const Source2Client002: usize = 0x61AC230;
            pub const Source2ClientConfig001: usize = 0x6154680;
            pub const Source2ClientPrediction001: usize = 0x5A860B0;
            pub const Source2ClientUI001: usize = 0x586F5C0;
        };
        // Module: engine2.dll
        pub const engine2_dll = struct {
            pub const BenchmarkService001: usize = 0x614B60;
            pub const BugBugService001: usize = 0x614C60;
            pub const BugService001: usize = 0x8CD430;
            pub const ClientServerEngineLoopService_001: usize = 0x90EAD0;
            pub const ClientServerSharedHandleSystem001: usize = 0x90E100;
            pub const EngineGameUI001: usize = 0x612400;
            pub const EngineServiceMgr001: usize = 0x90E3B0;
            pub const GameEventSystemClientV001: usize = 0x90E690;
            pub const GameEventSystemServerV001: usize = 0x90E7C0;
            pub const GameResourceServiceClientV001: usize = 0x614CA0;
            pub const GameResourceServiceServerV001: usize = 0x614D00;
            pub const GameUIService_001: usize = 0x8CD880;
            pub const HostStateMgr001: usize = 0x615490;
            pub const INETSUPPORT_001: usize = 0x60DA60;
            pub const InputService_001: usize = 0x8CDB70;
            pub const KeyValueCache001: usize = 0x615540;
            pub const MapListService_001: usize = 0x90C9E0;
            pub const NetworkClientService_001: usize = 0x90CB70;
            pub const NetworkP2PService_001: usize = 0x90CEB0;
            pub const NetworkServerService_001: usize = 0x90D060;
            pub const NetworkService_001: usize = 0x614E70;
            pub const RenderService_001: usize = 0x90D2D0;
            pub const ScreenshotService001: usize = 0x90D590;
            pub const SimpleEngineLoopService_001: usize = 0x6155A0;
            pub const SoundService_001: usize = 0x614EB0;
            pub const Source2EngineToClient001: usize = 0x611D30;
            pub const Source2EngineToClientStringTable001: usize = 0x611D90;
            pub const Source2EngineToServer001: usize = 0x611E08;
            pub const Source2EngineToServerStringTable001: usize = 0x611E30;
            pub const SplitScreenService_001: usize = 0x615190;
            pub const StatsService_001: usize = 0x90D950;
            pub const ToolService_001: usize = 0x615300;
            pub const VENGINE_GAMEUIFUNCS_VERSION005: usize = 0x612490;
            pub const VProfService_001: usize = 0x615340;
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
            pub const EntitySubclassUtilsV001: usize = 0x4741A10;
            pub const NavGameTest001: usize = 0x49E4CD0;
            pub const ServerToolsInfo_001: usize = 0x494DD18;
            pub const Source2GameClients001: usize = 0x4947ED0;
            pub const Source2GameDirector001: usize = 0x5025DA0;
            pub const Source2GameEntities001: usize = 0x494D4C0;
            pub const Source2Server001: usize = 0x494D310;
            pub const Source2ServerConfig001: usize = 0x4F50758;
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
            pub const TestScriptMgr001: usize = 0x39A7D0;
            pub const VEngineCvar007: usize = 0x3A5470;
            pub const VProcessUtils002: usize = 0x39A770;
            pub const VStringTokenSystem001: usize = 0x3CC170;
        };
        // Module: v8system.dll
        pub const v8system_dll = struct {
            pub const Source2V8System001: usize = 0x31770;
        };
        // Module: vconcomm.dll
        pub const vconcomm_dll = struct {
            pub const VConComm001: usize = 0x3B730;
        };
        // Module: vphysics2.dll
        pub const vphysics2_dll = struct {
            pub const VPhysics2_Interface_001: usize = 0x439E30;
        };
        // Module: vscript.dll
        pub const vscript_dll = struct {
            pub const VScriptManager010: usize = 0x13C430;
        };
        // Module: worldrenderer.dll
        pub const worldrenderer_dll = struct {
            pub const WorldRendererMgr001: usize = 0x22FD60;
        };
    };
};
