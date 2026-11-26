// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-11-26 21:46:41.980512300 +07:00

pub const source2_dumper = struct {
    pub const interfaces = struct {
        // Module: animationsystem.dll
        pub const animationsystem_dll = struct {
            pub const AnimationSystemUtils_001: usize = 0x846178;
            pub const AnimationSystem_001: usize = 0x83E098;
        };
        // Module: client.dll
        pub const client_dll = struct {
            pub const ClientToolsInfo_001: usize = 0x5CC6C00;
            pub const DOTA_CLIENT_GCCLIENT: usize = 0x6598B60;
            pub const GameClientExports001: usize = 0x5CC2940;
            pub const LegacyGameUI001: usize = 0x5D232E0;
            pub const PanoramaUIClient001: usize = 0x5D4E560;
            pub const PlayButtonService001: usize = 0x5D30BF0;
            pub const Source2Client002: usize = 0x63DDDF0;
            pub const Source2ClientConfig001: usize = 0x6373F50;
            pub const Source2ClientPrediction001: usize = 0x63E7D70;
            pub const Source2ClientUI001: usize = 0x5CE75E8;
        };
        // Module: engine2.dll
        pub const engine2_dll = struct {
            pub const BenchmarkService001: usize = 0x61D590;
            pub const BugBugService001: usize = 0x61D690;
            pub const BugService001: usize = 0x8CFCB0;
            pub const ClientServerEngineLoopService_001: usize = 0x9116B0;
            pub const ClientServerSharedHandleSystem001: usize = 0x910C70;
            pub const EngineGameUI001: usize = 0x61AF90;
            pub const EngineServiceMgr001: usize = 0x910F20;
            pub const GameEventSystemClientV001: usize = 0x911210;
            pub const GameEventSystemServerV001: usize = 0x911340;
            pub const GameResourceServiceClientV001: usize = 0x61D6D0;
            pub const GameResourceServiceServerV001: usize = 0x61D730;
            pub const GameUIService_001: usize = 0x8D0100;
            pub const HostStateMgr001: usize = 0x61DB70;
            pub const INETSUPPORT_001: usize = 0x616490;
            pub const InputService_001: usize = 0x8D03E0;
            pub const KeyValueCache001: usize = 0x61DC20;
            pub const MapListService_001: usize = 0x90F250;
            pub const NetworkClientService_001: usize = 0x90F3F0;
            pub const NetworkP2PService_001: usize = 0x90F730;
            pub const NetworkServerService_001: usize = 0x90F8E0;
            pub const NetworkService_001: usize = 0x61D8A0;
            pub const RenderService_001: usize = 0x90FB50;
            pub const ScreenshotService001: usize = 0x90FE20;
            pub const SimpleEngineLoopService_001: usize = 0x61DCA0;
            pub const SoundService_001: usize = 0x90FFD0;
            pub const Source2EngineToClient001: usize = 0x61A900;
            pub const Source2EngineToClientStringTable001: usize = 0x61A960;
            pub const Source2EngineToServer001: usize = 0x61A9D8;
            pub const Source2EngineToServerStringTable001: usize = 0x61AA00;
            pub const SplitScreenService_001: usize = 0x61D8E0;
            pub const StatsService_001: usize = 0x9104B0;
            pub const ToolService_001: usize = 0x61D9E0;
            pub const VENGINE_GAMEUIFUNCS_VERSION005: usize = 0x61B020;
            pub const VProfService_001: usize = 0x61DA20;
        };
        // Module: filesystem_stdio.dll
        pub const filesystem_stdio_dll = struct {
            pub const VAsyncFileSystem2_001: usize = 0x2146F0;
            pub const VFileSystem017: usize = 0x214490;
        };
        // Module: host.dll
        pub const host_dll = struct {
            pub const DebugDrawQueueManager001: usize = 0x146090;
            pub const DotaMapUtils001: usize = 0x146140;
            pub const GameModelInfo001: usize = 0x1460D0;
            pub const GameSystem2HostHook: usize = 0x146110;
            pub const HostUtils001: usize = 0x1461A0;
            pub const PredictionDiffManager001: usize = 0x1463B0;
            pub const SaveRestoreDataVersion001: usize = 0x1464E0;
            pub const SinglePlayerSharedMemory001: usize = 0x146510;
            pub const Source2Host001: usize = 0x146580;
        };
        // Module: imemanager.dll
        pub const imemanager_dll = struct {
            pub const IMEManager001: usize = 0x37AA0;
        };
        // Module: inputsystem.dll
        pub const inputsystem_dll = struct {
            pub const InputStackSystemVersion001: usize = 0x45010;
            pub const InputSystemVersion001: usize = 0x46D40;
        };
        // Module: localize.dll
        pub const localize_dll = struct {
            pub const Localize_001: usize = 0x5A120;
        };
        // Module: materialsystem2.dll
        pub const materialsystem2_dll = struct {
            pub const FontManager_001: usize = 0x16A9E0;
            pub const MaterialUtils_001: usize = 0x152EE0;
            pub const PostProcessingSystem_001: usize = 0x152E10;
            pub const TextLayout_001: usize = 0x152E70;
            pub const VMaterialSystem2_001: usize = 0x16A620;
        };
        // Module: meshsystem.dll
        pub const meshsystem_dll = struct {
            pub const MeshSystem001: usize = 0x18AE70;
        };
        // Module: navsystem.dll
        pub const navsystem_dll = struct {
            pub const NavSystem001: usize = 0x1302C0;
        };
        // Module: networksystem.dll
        pub const networksystem_dll = struct {
            pub const FlattenedSerializersVersion001: usize = 0x273BA0;
            pub const NetworkMessagesVersion001: usize = 0x29FE60;
            pub const NetworkSystemVersion001: usize = 0x28D5C0;
            pub const SerializedEntitiesVersion001: usize = 0x28D6A0;
        };
        // Module: panorama.dll
        pub const panorama_dll = struct {
            pub const PanoramaUIEngine001: usize = 0x5D1030;
        };
        // Module: panorama_text_pango.dll
        pub const panorama_text_pango_dll = struct {
            pub const PanoramaTextServices001: usize = 0x2B99D0;
        };
        // Module: particles.dll
        pub const particles_dll = struct {
            pub const ParticleSystemMgr003: usize = 0x5E84F0;
        };
        // Module: pulse_system.dll
        pub const pulse_system_dll = struct {
            pub const IPulseSystem_001: usize = 0x252D40;
        };
        // Module: rendersystemdx11.dll
        pub const rendersystemdx11_dll = struct {
            pub const RenderDeviceMgr001: usize = 0x3FCCA0;
            pub const RenderUtils_001: usize = 0x3FD5B0;
            pub const VRenderDeviceMgrBackdoor001: usize = 0x3FCD40;
        };
        // Module: resourcesystem.dll
        pub const resourcesystem_dll = struct {
            pub const ResourceSystem013: usize = 0x96FA0;
        };
        // Module: scenefilecache.dll
        pub const scenefilecache_dll = struct {
            pub const ResponseRulesCache001: usize = 0x13A700;
            pub const SceneFileCache002: usize = 0x13A828;
        };
        // Module: scenesystem.dll
        pub const scenesystem_dll = struct {
            pub const RenderingPipelines_001: usize = 0x6C03B0;
            pub const SceneSystem_002: usize = 0x92BC70;
            pub const SceneUtils_001: usize = 0x6C0F50;
        };
        // Module: schemasystem.dll
        pub const schemasystem_dll = struct {
            pub const SchemaSystem_001: usize = 0x76610;
        };
        // Module: server.dll
        pub const server_dll = struct {
            pub const EntitySubclassUtilsV001: usize = 0x4A6D530;
            pub const NavGameTest001: usize = 0x4CCB9C0;
            pub const ServerToolsInfo_001: usize = 0x4C5B068;
            pub const Source2GameClients001: usize = 0x4C59E00;
            pub const Source2GameDirector001: usize = 0x5349360;
            pub const Source2GameEntities001: usize = 0x4C5A810;
            pub const Source2Server001: usize = 0x4C5A660;
            pub const Source2ServerConfig001: usize = 0x521BFB8;
        };
        // Module: soundsystem.dll
        pub const soundsystem_dll = struct {
            pub const SoundBugBugService001_Client: usize = 0x530FB0;
            pub const SoundOpSystem001: usize = 0x530E90;
            pub const SoundOpSystemEdit001: usize = 0x530DA0;
            pub const SoundSystem001: usize = 0x646CD0;
            pub const VMixEditTool001: usize = 0x594287F;
        };
        // Module: steamaudio.dll
        pub const steamaudio_dll = struct {
            pub const SteamAudio001: usize = 0x36FD70;
        };
        // Module: tier0.dll
        pub const tier0_dll = struct {
            pub const TestScriptMgr001: usize = 0x491BA0;
            pub const VEngineCvar007: usize = 0x4A1E70;
            pub const VProcessUtils002: usize = 0x491A60;
            pub const VStringTokenSystem001: usize = 0x4C8BF0;
        };
        // Module: v8system.dll
        pub const v8system_dll = struct {
            pub const Source2V8System001: usize = 0x2C900;
        };
        // Module: vconcomm.dll
        pub const vconcomm_dll = struct {
            pub const VConComm001: usize = 0x3C8D0;
        };
        // Module: vphysics2.dll
        pub const vphysics2_dll = struct {
            pub const VPhysics2_Interface_001: usize = 0x486550;
        };
        // Module: vscript.dll
        pub const vscript_dll = struct {
            pub const VScriptManager010: usize = 0x13E5B0;
        };
        // Module: worldrenderer.dll
        pub const worldrenderer_dll = struct {
            pub const WorldRendererMgr001: usize = 0x2662B0;
        };
    };
};
