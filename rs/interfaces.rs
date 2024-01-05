#![allow(non_snake_case, non_upper_case_globals)]

pub mod animationsystem_dll {
    pub const AnimationSystemUtils_001: usize = 0x634B0;
    pub const AnimationSystem_001: usize = 0x5E0A0;
}

pub mod client_dll {
    pub const PanoramaUIClient001: usize = 0x248AC90;
    pub const PlayButtonService001: usize = 0x1FFDAF0;
    pub const DOTA_CLIENT_GCCLIENT: usize = 0x1BB2080;
    pub const LegacyGameUI001: usize = 0x1B95A50;
    pub const Source2ClientPrediction001: usize = 0x157A430;
    pub const ClientToolsInfo_001: usize = 0x1528F40;
    pub const Source2Client002: usize = 0x1528F60;
    pub const GameClientExports001: usize = 0x1528F50;
    pub const Source2ClientConfig001: usize = 0x12B89F0;
    pub const Source2ClientUI001: usize = 0x3F3AF0;
}

pub mod engine2_dll {
    pub const SimpleEngineLoopService_001: usize = 0x1E7270;
    pub const ClientServerEngineLoopService_001: usize = 0x1DCE50;
    pub const KeyValueCache001: usize = 0x1D91E0;
    pub const HostStateMgr001: usize = 0x1D6C00;
    pub const GameEventSystemServerV001: usize = 0x1D2020;
    pub const GameEventSystemClientV001: usize = 0x1D2010;
    pub const EngineServiceMgr001: usize = 0x1CD910;
    pub const VProfService_001: usize = 0x1BB5B0;
    pub const ToolService_001: usize = 0x1BA310;
    pub const StatsService_001: usize = 0x1B5590;
    pub const SplitScreenService_001: usize = 0x1B2880;
    pub const SoundService_001: usize = 0x1AD3D0;
    pub const ScreenshotService001: usize = 0x1A9270;
    pub const RenderService_001: usize = 0x1A65D0;
    pub const NetworkService_001: usize = 0x1A60E0;
    pub const NetworkServerService_001: usize = 0x1A1100;
    pub const NetworkP2PService_001: usize = 0x19CB10;
    pub const NetworkClientService_001: usize = 0x1966C0;
    pub const MapListService_001: usize = 0x18F6B0;
    pub const InputService_001: usize = 0x17D320;
    pub const GameUIService_001: usize = 0x177950;
    pub const GameResourceServiceServerV001: usize = 0x175530;
    pub const GameResourceServiceClientV001: usize = 0x175520;
    pub const BugService001: usize = 0x16E510;
    pub const BenchmarkService001: usize = 0x16CCE0;
    pub const VENGINE_GAMEUIFUNCS_VERSION005: usize = 0x1197A0;
    pub const EngineGameUI001: usize = 0x118A60;
    pub const INETSUPPORT_001: usize = 0xE7C10;
    pub const Source2EngineToServerStringTable001: usize = 0xA21F0;
    pub const Source2EngineToServer001: usize = 0x8BB10;
    pub const Source2EngineToClientStringTable001: usize = 0x84450;
    pub const Source2EngineToClient001: usize = 0x5A890;
}

pub mod filesystem_stdio_dll {
    pub const VAsyncFileSystem2_001: usize = 0x66EF0;
    pub const VFileSystem017: usize = 0x66EE0;
}

pub mod host_dll {
    pub const Source2Host001: usize = 0x1DAE0;
    pub const SinglePlayerSharedMemory001: usize = 0x1D760;
    pub const SaveRestoreDataVersion001: usize = 0x1D750;
    pub const PredictionDiffManager001: usize = 0x1BAD0;
    pub const HostUtils001: usize = 0x16FA0;
    pub const DotaMapUtils001: usize = 0x12350;
    pub const GameSystem2HostHook: usize = 0x11CB0;
    pub const GameModelInfo001: usize = 0x11BD0;
    pub const DebugDrawQueueManager001: usize = 0x11650;
}

pub mod imemanager_dll {
    pub const IMEManager001: usize = 0xC470;
}

pub mod inputsystem_dll {
    pub const InputSystemVersion001: usize = 0x28D0;
    pub const InputStackSystemVersion001: usize = 0x16F0;
}

pub mod localize_dll {
    pub const Localize_001: usize = 0x3830;
}

pub mod materialsystem2_dll {
    pub const MaterialUtils_001: usize = 0x4DE50;
    pub const TextLayout_001: usize = 0x4A5A0;
    pub const PostProcessingSystem_001: usize = 0x42D20;
    pub const FontManager_001: usize = 0x37AC0;
    pub const VMaterialSystem2_001: usize = 0x25FF0;
}

pub mod meshsystem_dll {
    pub const MeshSystem001: usize = 0x7150;
}

pub mod navsystem_dll {
    pub const NavSystem001: usize = 0x7540;
}

pub mod networksystem_dll {
    pub const SerializedEntitiesVersion001: usize = 0xD3270;
    pub const NetworkSystemVersion001: usize = 0xBD520;
    pub const NetworkMessagesVersion001: usize = 0x9CF40;
    pub const FlattenedSerializersVersion001: usize = 0x7C570;
}

pub mod panorama_dll {
    pub const PanoramaUIEngine001: usize = 0x5DE10;
}

pub mod panorama_text_pango_dll {
    pub const PanoramaTextServices001: usize = 0x4CCB0;
}

pub mod particles_dll {
    pub const ParticleSystemMgr003: usize = 0x52C80;
}

pub mod pulse_system_dll {
    pub const IPulseSystem_001: usize = 0x5A20;
}

pub mod rendersystemdx11_dll {
    pub const RenderUtils_001: usize = 0x53670;
    pub const VRenderDeviceMgrBackdoor001: usize = 0x4ABE0;
    pub const RenderDeviceMgr001: usize = 0x4ABD0;
}

pub mod resourcesystem_dll {
    pub const ResourceSystem013: usize = 0x106B0;
}

pub mod scenefilecache_dll {
    pub const SceneFileCache002: usize = 0x6900;
    pub const ResponseRulesCache001: usize = 0x3190;
}

pub mod scenesystem_dll {
    pub const SceneUtils_001: usize = 0x13D870;
    pub const SceneSystem_002: usize = 0xCABB0;
    pub const RenderingPipelines_001: usize = 0x8EA20;
}

pub mod schemasystem_dll {
    pub const SchemaSystem_001: usize = 0xA840;
}

pub mod server_dll {
    pub const NavGameTest001: usize = 0x2044010;
    pub const Source2GameDirector001: usize = 0x1AD9C60;
    pub const ServerToolsInfo_001: usize = 0x1774C80;
    pub const Source2GameClients001: usize = 0x1774C90;
    pub const Source2GameEntities001: usize = 0x1774CA0;
    pub const Source2Server001: usize = 0x1774CB0;
    pub const Source2ServerConfig001: usize = 0x150E490;
    pub const EntitySubclassUtilsV001: usize = 0x3BA0B0;
}

pub mod soundsystem_dll {
    pub const SoundOpSystem001: usize = 0x156B90;
    pub const SoundOpSystemEdit001: usize = 0x8B970;
    pub const VMixEditTool001: usize = 0x71780;
    pub const SoundSystem001: usize = 0x46580;
}

pub mod steamclient64_dll {
    pub const IVALIDATE001: usize = 0x84FE60;
    pub const CLIENTENGINE_INTERFACE_VERSION005: usize = 0x84BD10;
    pub const SteamClient021: usize = 0x646160;
    pub const SteamClient020: usize = 0x646150;
    pub const SteamClient019: usize = 0x646140;
    pub const SteamClient018: usize = 0x646130;
    pub const SteamClient017: usize = 0x646120;
    pub const SteamClient016: usize = 0x646110;
    pub const SteamClient015: usize = 0x646100;
    pub const SteamClient014: usize = 0x6460F0;
    pub const SteamClient013: usize = 0x6460E0;
    pub const SteamClient012: usize = 0x6460D0;
    pub const SteamClient011: usize = 0x6460C0;
    pub const SteamClient010: usize = 0x6460B0;
    pub const SteamClient009: usize = 0x6460A0;
    pub const SteamClient008: usize = 0x646090;
    pub const SteamClient007: usize = 0x646080;
    pub const SteamClient006: usize = 0x646070;
    pub const p2pvoice002: usize = 0xD97C0;
    pub const p2pvoicesingleton002: usize = 0xD6220;
}

pub mod tier0_dll {
    pub const VStringTokenSystem001: usize = 0x18F630;
    pub const TestScriptMgr001: usize = 0x1415C0;
    pub const VProcessUtils002: usize = 0x1315B0;
    pub const VEngineCvar007: usize = 0x635E0;
}

pub mod v8system_dll {
    pub const Source2V8System001: usize = 0x1670;
}

pub mod valve_avi_dll {
    pub const VAvi001: usize = 0x2F90;
}

pub mod valve_wmf_dll {
    pub const VMediaFoundation001: usize = 0x12D0;
}

pub mod vphysics2_dll {
    pub const VPhysics2_Handle_Interface_001: usize = 0x5FA10;
    pub const VPhysics2_Interface_001: usize = 0x5B790;
}

pub mod vscript_dll {
    pub const VScriptManager010: usize = 0x31C40;
}

pub mod vstdlib_s64_dll {
    pub const IVALIDATE001: usize = 0x25000;
    pub const VEngineCvar002: usize = 0x5760;
}

pub mod worldrenderer_dll {
    pub const WorldRendererMgr001: usize = 0x212A0;
}