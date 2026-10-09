// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-09 13:46:49.828411900 +07:00

#![allow(non_upper_case_globals, unused)]

pub mod source2_dumper {
    pub mod interfaces {
        // Module: animationsystem.dll
        pub mod animationsystem_dll {
            pub const AnimationSystemUtils_001: usize = 0x83A738;
            pub const AnimationSystem_001: usize = 0x832658;
        }
        // Module: client.dll
        pub mod client_dll {
            pub const ClientToolsInfo_001: usize = 0x5B2C8F0;
            pub const DOTA_CLIENT_GCCLIENT: usize = 0x63CA7C0;
            pub const GameClientExports001: usize = 0x5B28620;
            pub const LegacyGameUI001: usize = 0x5B89150;
            pub const PanoramaUIClient001: usize = 0x5BB42A0;
            pub const PlayButtonService001: usize = 0x5B96A38;
            pub const Source2Client002: usize = 0x6213E20;
            pub const Source2ClientConfig001: usize = 0x61B11A0;
            pub const Source2ClientPrediction001: usize = 0x5B2F490;
            pub const Source2ClientUI001: usize = 0x5945BC0;
        }
        // Module: engine2.dll
        pub mod engine2_dll {
            pub const BenchmarkService001: usize = 0x5BDD80;
            pub const BugBugService001: usize = 0x5BDE80;
            pub const BugService001: usize = 0x874490;
            pub const ClientServerEngineLoopService_001: usize = 0x8B5B00;
            pub const ClientServerSharedHandleSystem001: usize = 0x8B50E0;
            pub const EngineGameUI001: usize = 0x5BB790;
            pub const EngineServiceMgr001: usize = 0x8B53A0;
            pub const GameEventSystemClientV001: usize = 0x8B5680;
            pub const GameEventSystemServerV001: usize = 0x8B57B0;
            pub const GameResourceServiceClientV001: usize = 0x5BDEC0;
            pub const GameResourceServiceServerV001: usize = 0x5BDF20;
            pub const GameUIService_001: usize = 0x8748E0;
            pub const HostStateMgr001: usize = 0x5BE5E0;
            pub const INETSUPPORT_001: usize = 0x5B6D60;
            pub const InputService_001: usize = 0x874BC0;
            pub const KeyValueCache001: usize = 0x5BE690;
            pub const MapListService_001: usize = 0x8B3A30;
            pub const NetworkClientService_001: usize = 0x8B3BC0;
            pub const NetworkP2PService_001: usize = 0x8B3F00;
            pub const NetworkServerService_001: usize = 0x8B40B0;
            pub const NetworkService_001: usize = 0x5BE090;
            pub const RenderService_001: usize = 0x8B4320;
            pub const ScreenshotService001: usize = 0x8B45E0;
            pub const SimpleEngineLoopService_001: usize = 0x5BE710;
            pub const SoundService_001: usize = 0x5BE0D0;
            pub const Source2EngineToClient001: usize = 0x5BB100;
            pub const Source2EngineToClientStringTable001: usize = 0x5BB160;
            pub const Source2EngineToServer001: usize = 0x5BB1D8;
            pub const Source2EngineToServerStringTable001: usize = 0x5BB200;
            pub const SplitScreenService_001: usize = 0x5BE350;
            pub const StatsService_001: usize = 0x8B4920;
            pub const ToolService_001: usize = 0x5BE450;
            pub const VENGINE_GAMEUIFUNCS_VERSION005: usize = 0x5BB820;
            pub const VProfService_001: usize = 0x5BE490;
        }
        // Module: filesystem_stdio.dll
        pub mod filesystem_stdio_dll {
            pub const VAsyncFileSystem2_001: usize = 0x214610;
            pub const VFileSystem017: usize = 0x2143D0;
        }
        // Module: host.dll
        pub mod host_dll {
            pub const DebugDrawQueueManager001: usize = 0x145EF0;
            pub const DotaMapUtils001: usize = 0x145FA0;
            pub const GameModelInfo001: usize = 0x145F30;
            pub const GameSystem2HostHook: usize = 0x145F70;
            pub const HostUtils001: usize = 0x146000;
            pub const PredictionDiffManager001: usize = 0x146210;
            pub const SaveRestoreDataVersion001: usize = 0x146340;
            pub const SinglePlayerSharedMemory001: usize = 0x146370;
            pub const Source2Host001: usize = 0x1463E0;
        }
        // Module: imemanager.dll
        pub mod imemanager_dll {
            pub const IMEManager001: usize = 0x37AA0;
        }
        // Module: inputsystem.dll
        pub mod inputsystem_dll {
            pub const InputStackSystemVersion001: usize = 0x44E90;
            pub const InputSystemVersion001: usize = 0x46BC0;
        }
        // Module: localize.dll
        pub mod localize_dll {
            pub const Localize_001: usize = 0x59120;
        }
        // Module: materialsystem2.dll
        pub mod materialsystem2_dll {
            pub const FontManager_001: usize = 0x163810;
            pub const MaterialUtils_001: usize = 0x14BD30;
            pub const PostProcessingSystem_001: usize = 0x14BC60;
            pub const TextLayout_001: usize = 0x14BCC0;
            pub const VMaterialSystem2_001: usize = 0x163450;
        }
        // Module: meshsystem.dll
        pub mod meshsystem_dll {
            pub const MeshSystem001: usize = 0x183AF0;
        }
        // Module: navsystem.dll
        pub mod navsystem_dll {
            pub const NavSystem001: usize = 0x12C120;
        }
        // Module: networksystem.dll
        pub mod networksystem_dll {
            pub const FlattenedSerializersVersion001: usize = 0x2779E0;
            pub const NetworkMessagesVersion001: usize = 0x2A39E0;
            pub const NetworkSystemVersion001: usize = 0x291130;
            pub const SerializedEntitiesVersion001: usize = 0x291210;
        }
        // Module: panorama.dll
        pub mod panorama_dll {
            pub const PanoramaUIEngine001: usize = 0x586F60;
        }
        // Module: panorama_text_pango.dll
        pub mod panorama_text_pango_dll {
            pub const PanoramaTextServices001: usize = 0x2BA9D0;
        }
        // Module: particles.dll
        pub mod particles_dll {
            pub const ParticleSystemMgr003: usize = 0x633FB0;
        }
        // Module: pulse_system.dll
        pub mod pulse_system_dll {
            pub const IPulseSystem_001: usize = 0x243A90;
        }
        // Module: rendersystemdx11.dll
        pub mod rendersystemdx11_dll {
            pub const RenderDeviceMgr001: usize = 0x4372F0;
            pub const RenderUtils_001: usize = 0x437BD0;
            pub const VRenderDeviceMgrBackdoor001: usize = 0x437390;
        }
        // Module: resourcesystem.dll
        pub mod resourcesystem_dll {
            pub const ResourceSystem013: usize = 0x970C0;
        }
        // Module: scenefilecache.dll
        pub mod scenefilecache_dll {
            pub const ResponseRulesCache001: usize = 0x1283F0;
            pub const SceneFileCache002: usize = 0x128518;
        }
        // Module: scenesystem.dll
        pub mod scenesystem_dll {
            pub const RenderingPipelines_001: usize = 0x6AB5B0;
            pub const SceneSystem_002: usize = 0x955C00;
            pub const SceneUtils_001: usize = 0x6AC310;
        }
        // Module: schemasystem.dll
        pub mod schemasystem_dll {
            pub const SchemaSystem_001: usize = 0x76610;
        }
        // Module: server.dll
        pub mod server_dll {
            pub const EntitySubclassUtilsV001: usize = 0x48C61C0;
            pub const NavGameTest001: usize = 0x4B19E08;
            pub const ServerToolsInfo_001: usize = 0x4AA98D8;
            pub const Source2GameClients001: usize = 0x4AA8670;
            pub const Source2GameDirector001: usize = 0x5165700;
            pub const Source2GameEntities001: usize = 0x4AA9080;
            pub const Source2Server001: usize = 0x4AA8ED0;
            pub const Source2ServerConfig001: usize = 0x5041F88;
        }
        // Module: soundsystem.dll
        pub mod soundsystem_dll {
            pub const SoundBugBugService001_Client: usize = 0x536A10;
            pub const SoundOpSystem001: usize = 0x5368F0;
            pub const SoundOpSystemEdit001: usize = 0x536800;
            pub const SoundSystem001: usize = 0x64AF00;
            pub const VMixEditTool001: usize = 0x5943E3F;
        }
        // Module: steamaudio.dll
        pub mod steamaudio_dll {
            pub const SteamAudio001: usize = 0x35F1B0;
        }
        // Module: tier0.dll
        pub mod tier0_dll {
            pub const TestScriptMgr001: usize = 0x3A1960;
            pub const VEngineCvar007: usize = 0x3AC6B0;
            pub const VProcessUtils002: usize = 0x3A1820;
            pub const VStringTokenSystem001: usize = 0x3D3340;
        }
        // Module: v8system.dll
        pub mod v8system_dll {
            pub const Source2V8System001: usize = 0x34790;
        }
        // Module: vconcomm.dll
        pub mod vconcomm_dll {
            pub const VConComm001: usize = 0x3C750;
        }
        // Module: vphysics2.dll
        pub mod vphysics2_dll {
            pub const VPhysics2_Interface_001: usize = 0x472F10;
        }
        // Module: vscript.dll
        pub mod vscript_dll {
            pub const VScriptManager010: usize = 0x13E430;
        }
        // Module: worldrenderer.dll
        pub mod worldrenderer_dll {
            pub const WorldRendererMgr001: usize = 0x1D9680;
        }
    }
}
