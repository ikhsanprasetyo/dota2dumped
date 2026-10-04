// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-04 12:25:34.881424700 +07:00

#![allow(non_upper_case_globals, unused)]

pub mod source2_dumper {
    pub mod interfaces {
        // Module: animationsystem.dll
        pub mod animationsystem_dll {
            pub const AnimationSystemUtils_001: usize = 0x839F10;
            pub const AnimationSystem_001: usize = 0x831E30;
        }
        // Module: client.dll
        pub mod client_dll {
            pub const ClientToolsInfo_001: usize = 0x5A90CD0;
            pub const DOTA_CLIENT_GCCLIENT: usize = 0x6368250;
            pub const GameClientExports001: usize = 0x5A8C9F8;
            pub const LegacyGameUI001: usize = 0x5AFB150;
            pub const PanoramaUIClient001: usize = 0x5B27D90;
            pub const PlayButtonService001: usize = 0x5B08E40;
            pub const Source2Client002: usize = 0x61BB2A0;
            pub const Source2ClientConfig001: usize = 0x6163A30;
            pub const Source2ClientPrediction001: usize = 0x5A951C0;
            pub const Source2ClientUI001: usize = 0x587E5C0;
        }
        // Module: engine2.dll
        pub mod engine2_dll {
            pub const BenchmarkService001: usize = 0x615BB0;
            pub const BugBugService001: usize = 0x615CB0;
            pub const BugService001: usize = 0x8CE450;
            pub const ClientServerEngineLoopService_001: usize = 0x90FAF0;
            pub const ClientServerSharedHandleSystem001: usize = 0x90F120;
            pub const EngineGameUI001: usize = 0x613400;
            pub const EngineServiceMgr001: usize = 0x90F3D0;
            pub const GameEventSystemClientV001: usize = 0x90F6B0;
            pub const GameEventSystemServerV001: usize = 0x90F7E0;
            pub const GameResourceServiceClientV001: usize = 0x615CF0;
            pub const GameResourceServiceServerV001: usize = 0x615D50;
            pub const GameUIService_001: usize = 0x8CE8A0;
            pub const HostStateMgr001: usize = 0x6164E0;
            pub const INETSUPPORT_001: usize = 0x60EA60;
            pub const InputService_001: usize = 0x8CEB90;
            pub const KeyValueCache001: usize = 0x616590;
            pub const MapListService_001: usize = 0x90DA00;
            pub const NetworkClientService_001: usize = 0x90DB90;
            pub const NetworkP2PService_001: usize = 0x90DED0;
            pub const NetworkServerService_001: usize = 0x90E080;
            pub const NetworkService_001: usize = 0x615EC0;
            pub const RenderService_001: usize = 0x90E2F0;
            pub const ScreenshotService001: usize = 0x90E5B0;
            pub const SimpleEngineLoopService_001: usize = 0x6165F0;
            pub const SoundService_001: usize = 0x615F00;
            pub const Source2EngineToClient001: usize = 0x612D30;
            pub const Source2EngineToClientStringTable001: usize = 0x612D90;
            pub const Source2EngineToServer001: usize = 0x612E08;
            pub const Source2EngineToServerStringTable001: usize = 0x612E30;
            pub const SplitScreenService_001: usize = 0x6161E0;
            pub const StatsService_001: usize = 0x90E970;
            pub const ToolService_001: usize = 0x616350;
            pub const VENGINE_GAMEUIFUNCS_VERSION005: usize = 0x613490;
            pub const VProfService_001: usize = 0x616390;
        }
        // Module: filesystem_stdio.dll
        pub mod filesystem_stdio_dll {
            pub const VAsyncFileSystem2_001: usize = 0x2135F0;
            pub const VFileSystem017: usize = 0x2133B0;
        }
        // Module: host.dll
        pub mod host_dll {
            pub const DebugDrawQueueManager001: usize = 0x145000;
            pub const DotaMapUtils001: usize = 0x1450B0;
            pub const GameModelInfo001: usize = 0x145040;
            pub const GameSystem2HostHook: usize = 0x145080;
            pub const HostUtils001: usize = 0x145110;
            pub const PredictionDiffManager001: usize = 0x145320;
            pub const SaveRestoreDataVersion001: usize = 0x145450;
            pub const SinglePlayerSharedMemory001: usize = 0x145480;
            pub const Source2Host001: usize = 0x1454F0;
        }
        // Module: imemanager.dll
        pub mod imemanager_dll {
            pub const IMEManager001: usize = 0x36AA0;
        }
        // Module: inputsystem.dll
        pub mod inputsystem_dll {
            pub const InputStackSystemVersion001: usize = 0x43E90;
            pub const InputSystemVersion001: usize = 0x45BA0;
        }
        // Module: localize.dll
        pub mod localize_dll {
            pub const Localize_001: usize = 0x58100;
        }
        // Module: materialsystem2.dll
        pub mod materialsystem2_dll {
            pub const FontManager_001: usize = 0x15CAB0;
            pub const MaterialUtils_001: usize = 0x144F00;
            pub const PostProcessingSystem_001: usize = 0x144E30;
            pub const TextLayout_001: usize = 0x144E90;
            pub const VMaterialSystem2_001: usize = 0x15C700;
        }
        // Module: meshsystem.dll
        pub mod meshsystem_dll {
            pub const MeshSystem001: usize = 0x16BE20;
        }
        // Module: navsystem.dll
        pub mod navsystem_dll {
            pub const NavSystem001: usize = 0x128CA0;
        }
        // Module: networksystem.dll
        pub mod networksystem_dll {
            pub const FlattenedSerializersVersion001: usize = 0x26E860;
            pub const NetworkMessagesVersion001: usize = 0x296AF0;
            pub const NetworkSystemVersion001: usize = 0x287FB0;
            pub const SerializedEntitiesVersion001: usize = 0x2880A0;
        }
        // Module: panorama.dll
        pub mod panorama_dll {
            pub const PanoramaUIEngine001: usize = 0x50ED60;
        }
        // Module: panorama_text_pango.dll
        pub mod panorama_text_pango_dll {
            pub const PanoramaTextServices001: usize = 0x2B89D0;
        }
        // Module: particles.dll
        pub mod particles_dll {
            pub const ParticleSystemMgr003: usize = 0x608870;
        }
        // Module: pulse_system.dll
        pub mod pulse_system_dll {
            pub const IPulseSystem_001: usize = 0x218290;
        }
        // Module: rendersystemdx11.dll
        pub mod rendersystemdx11_dll {
            pub const RenderDeviceMgr001: usize = 0x42B550;
            pub const RenderUtils_001: usize = 0x42BE30;
            pub const VRenderDeviceMgrBackdoor001: usize = 0x42B5F0;
        }
        // Module: resourcesystem.dll
        pub mod resourcesystem_dll {
            pub const ResourceSystem013: usize = 0x81670;
        }
        // Module: scenefilecache.dll
        pub mod scenefilecache_dll {
            pub const ResponseRulesCache001: usize = 0x113450;
            pub const SceneFileCache002: usize = 0x113578;
        }
        // Module: scenesystem.dll
        pub mod scenesystem_dll {
            pub const RenderingPipelines_001: usize = 0x666F20;
            pub const SceneSystem_002: usize = 0x9104F0;
            pub const SceneUtils_001: usize = 0x667DE0;
        }
        // Module: schemasystem.dll
        pub mod schemasystem_dll {
            pub const SchemaSystem_001: usize = 0x75630;
        }
        // Module: server.dll
        pub mod server_dll {
            pub const EntitySubclassUtilsV001: usize = 0x474FA10;
            pub const NavGameTest001: usize = 0x49F2B90;
            pub const ServerToolsInfo_001: usize = 0x495BC08;
            pub const Source2GameClients001: usize = 0x4955DC0;
            pub const Source2GameDirector001: usize = 0x5033D30;
            pub const Source2GameEntities001: usize = 0x495B3B0;
            pub const Source2Server001: usize = 0x495B200;
            pub const Source2ServerConfig001: usize = 0x4F5E588;
        }
        // Module: soundsystem.dll
        pub mod soundsystem_dll {
            pub const SoundBugBugService001_Client: usize = 0x553230;
            pub const SoundOpSystem001: usize = 0x553110;
            pub const SoundOpSystemEdit001: usize = 0x553020;
            pub const SoundSystem001: usize = 0x552A80;
            pub const VMixEditTool001: usize = 0x59489FF;
        }
        // Module: steamaudio.dll
        pub mod steamaudio_dll {
            pub const SteamAudio001: usize = 0x25F510;
        }
        // Module: tier0.dll
        pub mod tier0_dll {
            pub const TestScriptMgr001: usize = 0x3997D0;
            pub const VEngineCvar007: usize = 0x3A4470;
            pub const VProcessUtils002: usize = 0x399770;
            pub const VStringTokenSystem001: usize = 0x3CB170;
        }
        // Module: v8system.dll
        pub mod v8system_dll {
            pub const Source2V8System001: usize = 0x31770;
        }
        // Module: vconcomm.dll
        pub mod vconcomm_dll {
            pub const VConComm001: usize = 0x3B730;
        }
        // Module: vphysics2.dll
        pub mod vphysics2_dll {
            pub const VPhysics2_Interface_001: usize = 0x439E30;
        }
        // Module: vscript.dll
        pub mod vscript_dll {
            pub const VScriptManager010: usize = 0x13C430;
        }
        // Module: worldrenderer.dll
        pub mod worldrenderer_dll {
            pub const WorldRendererMgr001: usize = 0x22FD60;
        }
    }
}
