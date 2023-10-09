#![allow(non_snake_case, non_upper_case_globals)]

pub mod animationsystem_dll {
    pub const AnimationSystemUtils_001: usize = 0x634B0;
    pub const AnimationSystem_001: usize = 0x5E0A0;
}

pub mod client_dll {
    pub const PanoramaUIClient001: usize = 0x241C350;
    pub const PlayButtonService001: usize = 0x1F961D0;
    pub const DOTA_CLIENT_GCCLIENT: usize = 0x1B76550;
    pub const LegacyGameUI001: usize = 0x1B59FC0;
    pub const Source2ClientPrediction001: usize = 0x1549B00;
    pub const ClientToolsInfo_001: usize = 0x14F8680;
    pub const Source2Client002: usize = 0x14F86A0;
    pub const GameClientExports001: usize = 0x14F8690;
    pub const Source2ClientConfig001: usize = 0x1289930;
    pub const Source2ClientUI001: usize = 0x3EEF00;
}

pub mod engine2_dll {
    pub const SimpleEngineLoopService_001: usize = 0x1E6880;
    pub const ClientServerEngineLoopService_001: usize = 0x1DC470;
    pub const KeyValueCache001: usize = 0x1D8800;
    pub const HostStateMgr001: usize = 0x1D6240;
    pub const GameEventSystemServerV001: usize = 0x1D1660;
    pub const GameEventSystemClientV001: usize = 0x1D1650;
    pub const EngineServiceMgr001: usize = 0x1CCF50;
    pub const VProfService_001: usize = 0x1BAE20;
    pub const ToolService_001: usize = 0x1B9B80;
    pub const StatsService_001: usize = 0x1B4E00;
    pub const SplitScreenService_001: usize = 0x1B2070;
    pub const SoundService_001: usize = 0x1ACBC0;
    pub const ScreenshotService001: usize = 0x1A8A90;
    pub const RenderService_001: usize = 0x1A5DF0;
    pub const NetworkService_001: usize = 0x1A5900;
    pub const NetworkServerService_001: usize = 0x1A0930;
    pub const NetworkP2PService_001: usize = 0x19C340;
    pub const NetworkClientService_001: usize = 0x195F10;
    pub const MapListService_001: usize = 0x18EF50;
    pub const InputService_001: usize = 0x17CC50;
    pub const GameUIService_001: usize = 0x177280;
    pub const GameResourceServiceServerV001: usize = 0x174FE0;
    pub const GameResourceServiceClientV001: usize = 0x174FD0;
    pub const BugService001: usize = 0x16E000;
    pub const BenchmarkService001: usize = 0x16C7D0;
    pub const VENGINE_GAMEUIFUNCS_VERSION005: usize = 0x1192F0;
    pub const EngineGameUI001: usize = 0x1185B0;
    pub const INETSUPPORT_001: usize = 0xE7790;
    pub const Source2EngineToServerStringTable001: usize = 0xA1EA0;
    pub const Source2EngineToServer001: usize = 0x8B950;
    pub const Source2EngineToClientStringTable001: usize = 0x842B0;
    pub const Source2EngineToClient001: usize = 0x5A870;
}

pub mod filesystem_stdio_dll {
    pub const VAsyncFileSystem2_001: usize = 0x66D40;
    pub const VFileSystem017: usize = 0x66D30;
}

pub mod host_dll {
    pub const Source2Host001: usize = 0x1DAB0;
    pub const SinglePlayerSharedMemory001: usize = 0x1D730;
    pub const SaveRestoreDataVersion001: usize = 0x1D720;
    pub const PredictionDiffManager001: usize = 0x1BAC0;
    pub const HostUtils001: usize = 0x16F90;
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
    pub const MaterialUtils_001: usize = 0x4DDC0;
    pub const TextLayout_001: usize = 0x4A510;
    pub const PostProcessingSystem_001: usize = 0x42C90;
    pub const FontManager_001: usize = 0x37A70;
    pub const VMaterialSystem2_001: usize = 0x25FD0;
}

pub mod meshsystem_dll {
    pub const MeshSystem001: usize = 0x70C0;
}

pub mod navsystem_dll {
    pub const NavSystem001: usize = 0x7540;
}

pub mod networksystem_dll {
    pub const SerializedEntitiesVersion001: usize = 0xD2A00;
    pub const NetworkSystemVersion001: usize = 0xBCD40;
    pub const NetworkMessagesVersion001: usize = 0x9CD70;
    pub const FlattenedSerializersVersion001: usize = 0x7C520;
}

pub mod panorama_dll {
    pub const PanoramaUIEngine001: usize = 0x5E690;
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
    pub const RenderUtils_001: usize = 0x53520;
    pub const VRenderDeviceMgrBackdoor001: usize = 0x4AA90;
    pub const RenderDeviceMgr001: usize = 0x4AA80;
}

pub mod resourcesystem_dll {
    pub const ResourceSystem013: usize = 0x106B0;
}

pub mod scenefilecache_dll {
    pub const SceneFileCache002: usize = 0x68E0;
    pub const ResponseRulesCache001: usize = 0x3190;
}

pub mod scenesystem_dll {
    pub const SceneUtils_001: usize = 0x13D810;
    pub const SceneSystem_002: usize = 0xCAB70;
    pub const RenderingPipelines_001: usize = 0x8E9F0;
}

pub mod schemasystem_dll {
    pub const SchemaSystem_001: usize = 0xA840;
}

pub mod server_dll {
    pub const NavGameTest001: usize = 0x2020260;
    pub const Source2GameDirector001: usize = 0x1ABA470;
    pub const ServerToolsInfo_001: usize = 0x1757C90;
    pub const Source2GameClients001: usize = 0x1757CA0;
    pub const Source2GameEntities001: usize = 0x1757CB0;
    pub const Source2Server001: usize = 0x1757CC0;
    pub const Source2ServerConfig001: usize = 0x14F1590;
    pub const EntitySubclassUtilsV001: usize = 0x3B6D00;
}

pub mod soundsystem_dll {
    pub const SoundOpSystem001: usize = 0x156380;
    pub const SoundOpSystemEdit001: usize = 0x8B4B0;
    pub const VMixEditTool001: usize = 0x71710;
    pub const SoundSystem001: usize = 0x46510;
}

pub mod steamclient64_dll {
    pub const IVALIDATE001: usize = 0x833640;
    pub const CLIENTENGINE_INTERFACE_VERSION005: usize = 0x82F4C0;
    pub const SteamClient020: usize = 0x62CAF0;
    pub const SteamClient019: usize = 0x62CAE0;
    pub const SteamClient018: usize = 0x62CAD0;
    pub const SteamClient017: usize = 0x62CAC0;
    pub const SteamClient016: usize = 0x62CAB0;
    pub const SteamClient015: usize = 0x62CAA0;
    pub const SteamClient014: usize = 0x62CA90;
    pub const SteamClient013: usize = 0x62CA80;
    pub const SteamClient012: usize = 0x62CA70;
    pub const SteamClient011: usize = 0x62CA60;
    pub const SteamClient010: usize = 0x62CA50;
    pub const SteamClient009: usize = 0x62CA40;
    pub const SteamClient008: usize = 0x62CA30;
    pub const SteamClient007: usize = 0x62CA20;
    pub const SteamClient006: usize = 0x62CA10;
    pub const p2pvoice002: usize = 0xD8EA0;
    pub const p2pvoicesingleton002: usize = 0xD5840;
}

pub mod tier0_dll {
    pub const VStringTokenSystem001: usize = 0x18D690;
    pub const TestScriptMgr001: usize = 0x13F7B0;
    pub const VProcessUtils002: usize = 0x12F7C0;
    pub const VEngineCvar007: usize = 0x61640;
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
    pub const IVALIDATE001: usize = 0x24FF0;
    pub const VEngineCvar002: usize = 0x5750;
}

pub mod worldrenderer_dll {
    pub const WorldRendererMgr001: usize = 0x21280;
}