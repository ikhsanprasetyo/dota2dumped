// printed at 2023-10-22 22:08:43.351134600 +07:00	from Dota2 7.34d
#pragma once
#include <cstdint>

namespace Offsets {
    namespace animationsystem_dll {
        constexpr uint32_t AnimationSystemUtils_001 = 0x634B0;
        constexpr uint32_t AnimationSystem_001 = 0x5E0A0;
    }

    namespace client_dll {
        constexpr uint32_t PanoramaUIClient001 = 0x2428C20;
        constexpr uint32_t PlayButtonService001 = 0x1FA02A0;
        constexpr uint32_t DOTA_CLIENT_GCCLIENT = 0x1B78CE0;
        constexpr uint32_t LegacyGameUI001 = 0x1B5C750;
        constexpr uint32_t Source2ClientPrediction001 = 0x154B4A0;
        constexpr uint32_t ClientToolsInfo_001 = 0x14FA020;
        constexpr uint32_t Source2Client002 = 0x14FA040;
        constexpr uint32_t GameClientExports001 = 0x14FA030;
        constexpr uint32_t Source2ClientConfig001 = 0x128AC70;
        constexpr uint32_t Source2ClientUI001 = 0x3EF6A0;
    }

    namespace engine2_dll {
        constexpr uint32_t SimpleEngineLoopService_001 = 0x1E6900;
        constexpr uint32_t ClientServerEngineLoopService_001 = 0x1DC4F0;
        constexpr uint32_t KeyValueCache001 = 0x1D8880;
        constexpr uint32_t HostStateMgr001 = 0x1D62C0;
        constexpr uint32_t GameEventSystemServerV001 = 0x1D16E0;
        constexpr uint32_t GameEventSystemClientV001 = 0x1D16D0;
        constexpr uint32_t EngineServiceMgr001 = 0x1CCFD0;
        constexpr uint32_t VProfService_001 = 0x1BAE50;
        constexpr uint32_t ToolService_001 = 0x1B9BB0;
        constexpr uint32_t StatsService_001 = 0x1B4E30;
        constexpr uint32_t SplitScreenService_001 = 0x1B20A0;
        constexpr uint32_t SoundService_001 = 0x1ACBF0;
        constexpr uint32_t ScreenshotService001 = 0x1A8AC0;
        constexpr uint32_t RenderService_001 = 0x1A5E20;
        constexpr uint32_t NetworkService_001 = 0x1A5930;
        constexpr uint32_t NetworkServerService_001 = 0x1A0960;
        constexpr uint32_t NetworkP2PService_001 = 0x19C370;
        constexpr uint32_t NetworkClientService_001 = 0x195F40;
        constexpr uint32_t MapListService_001 = 0x18EF80;
        constexpr uint32_t InputService_001 = 0x17CC80;
        constexpr uint32_t GameUIService_001 = 0x1772B0;
        constexpr uint32_t GameResourceServiceServerV001 = 0x175010;
        constexpr uint32_t GameResourceServiceClientV001 = 0x175000;
        constexpr uint32_t BugService001 = 0x16E030;
        constexpr uint32_t BenchmarkService001 = 0x16C800;
        constexpr uint32_t VENGINE_GAMEUIFUNCS_VERSION005 = 0x119320;
        constexpr uint32_t EngineGameUI001 = 0x1185E0;
        constexpr uint32_t INETSUPPORT_001 = 0xE7790;
        constexpr uint32_t Source2EngineToServerStringTable001 = 0xA1EA0;
        constexpr uint32_t Source2EngineToServer001 = 0x8B950;
        constexpr uint32_t Source2EngineToClientStringTable001 = 0x842B0;
        constexpr uint32_t Source2EngineToClient001 = 0x5A870;
    }

    namespace filesystem_stdio_dll {
        constexpr uint32_t VAsyncFileSystem2_001 = 0x66D40;
        constexpr uint32_t VFileSystem017 = 0x66D30;
    }

    namespace host_dll {
        constexpr uint32_t Source2Host001 = 0x1DAB0;
        constexpr uint32_t SinglePlayerSharedMemory001 = 0x1D730;
        constexpr uint32_t SaveRestoreDataVersion001 = 0x1D720;
        constexpr uint32_t PredictionDiffManager001 = 0x1BAC0;
        constexpr uint32_t HostUtils001 = 0x16F90;
        constexpr uint32_t DotaMapUtils001 = 0x12350;
        constexpr uint32_t GameSystem2HostHook = 0x11CB0;
        constexpr uint32_t GameModelInfo001 = 0x11BD0;
        constexpr uint32_t DebugDrawQueueManager001 = 0x11650;
    }

    namespace imemanager_dll {
        constexpr uint32_t IMEManager001 = 0xC470;
    }

    namespace inputsystem_dll {
        constexpr uint32_t InputSystemVersion001 = 0x28D0;
        constexpr uint32_t InputStackSystemVersion001 = 0x16F0;
    }

    namespace localize_dll {
        constexpr uint32_t Localize_001 = 0x3830;
    }

    namespace materialsystem2_dll {
        constexpr uint32_t MaterialUtils_001 = 0x4DDC0;
        constexpr uint32_t TextLayout_001 = 0x4A510;
        constexpr uint32_t PostProcessingSystem_001 = 0x42C90;
        constexpr uint32_t FontManager_001 = 0x37A70;
        constexpr uint32_t VMaterialSystem2_001 = 0x25FD0;
    }

    namespace meshsystem_dll {
        constexpr uint32_t MeshSystem001 = 0x70C0;
    }

    namespace navsystem_dll {
        constexpr uint32_t NavSystem001 = 0x7540;
    }

    namespace networksystem_dll {
        constexpr uint32_t SerializedEntitiesVersion001 = 0xD2A00;
        constexpr uint32_t NetworkSystemVersion001 = 0xBCD40;
        constexpr uint32_t NetworkMessagesVersion001 = 0x9CD70;
        constexpr uint32_t FlattenedSerializersVersion001 = 0x7C520;
    }

    namespace panorama_dll {
        constexpr uint32_t PanoramaUIEngine001 = 0x5E690;
    }

    namespace panorama_text_pango_dll {
        constexpr uint32_t PanoramaTextServices001 = 0x4CCB0;
    }

    namespace particles_dll {
        constexpr uint32_t ParticleSystemMgr003 = 0x52C80;
    }

    namespace pulse_system_dll {
        constexpr uint32_t IPulseSystem_001 = 0x5A20;
    }

    namespace rendersystemdx11_dll {
        constexpr uint32_t RenderUtils_001 = 0x53520;
        constexpr uint32_t VRenderDeviceMgrBackdoor001 = 0x4AA90;
        constexpr uint32_t RenderDeviceMgr001 = 0x4AA80;
    }

    namespace resourcesystem_dll {
        constexpr uint32_t ResourceSystem013 = 0x106B0;
    }

    namespace scenefilecache_dll {
        constexpr uint32_t SceneFileCache002 = 0x68E0;
        constexpr uint32_t ResponseRulesCache001 = 0x3190;
    }

    namespace scenesystem_dll {
        constexpr uint32_t SceneUtils_001 = 0x13D810;
        constexpr uint32_t SceneSystem_002 = 0xCAB70;
        constexpr uint32_t RenderingPipelines_001 = 0x8E9F0;
    }

    namespace schemasystem_dll {
        constexpr uint32_t SchemaSystem_001 = 0xA840;
    }

    namespace server_dll {
        constexpr uint32_t NavGameTest001 = 0x20213A0;
        constexpr uint32_t Source2GameDirector001 = 0x1ABB510;
        constexpr uint32_t ServerToolsInfo_001 = 0x1758370;
        constexpr uint32_t Source2GameClients001 = 0x1758380;
        constexpr uint32_t Source2GameEntities001 = 0x1758390;
        constexpr uint32_t Source2Server001 = 0x17583A0;
        constexpr uint32_t Source2ServerConfig001 = 0x14F1C70;
        constexpr uint32_t EntitySubclassUtilsV001 = 0x3B6D00;
    }

    namespace soundsystem_dll {
        constexpr uint32_t SoundOpSystem001 = 0x156380;
        constexpr uint32_t SoundOpSystemEdit001 = 0x8B4B0;
        constexpr uint32_t VMixEditTool001 = 0x71710;
        constexpr uint32_t SoundSystem001 = 0x46510;
    }

    namespace steamclient64_dll {
        constexpr uint32_t IVALIDATE001 = 0x833640;
        constexpr uint32_t CLIENTENGINE_INTERFACE_VERSION005 = 0x82F4C0;
        constexpr uint32_t SteamClient020 = 0x62CAF0;
        constexpr uint32_t SteamClient019 = 0x62CAE0;
        constexpr uint32_t SteamClient018 = 0x62CAD0;
        constexpr uint32_t SteamClient017 = 0x62CAC0;
        constexpr uint32_t SteamClient016 = 0x62CAB0;
        constexpr uint32_t SteamClient015 = 0x62CAA0;
        constexpr uint32_t SteamClient014 = 0x62CA90;
        constexpr uint32_t SteamClient013 = 0x62CA80;
        constexpr uint32_t SteamClient012 = 0x62CA70;
        constexpr uint32_t SteamClient011 = 0x62CA60;
        constexpr uint32_t SteamClient010 = 0x62CA50;
        constexpr uint32_t SteamClient009 = 0x62CA40;
        constexpr uint32_t SteamClient008 = 0x62CA30;
        constexpr uint32_t SteamClient007 = 0x62CA20;
        constexpr uint32_t SteamClient006 = 0x62CA10;
        constexpr uint32_t p2pvoice002 = 0xD8EA0;
        constexpr uint32_t p2pvoicesingleton002 = 0xD5840;
    }

    namespace tier0_dll {
        constexpr uint32_t VStringTokenSystem001 = 0x18F410;
        constexpr uint32_t TestScriptMgr001 = 0x141530;
        constexpr uint32_t VProcessUtils002 = 0x131540;
        constexpr uint32_t VEngineCvar007 = 0x63530;
    }

    namespace v8system_dll {
        constexpr uint32_t Source2V8System001 = 0x1670;
    }

    namespace valve_avi_dll {
        constexpr uint32_t VAvi001 = 0x2F90;
    }

    namespace valve_wmf_dll {
        constexpr uint32_t VMediaFoundation001 = 0x12D0;
    }

    namespace vphysics2_dll {
        constexpr uint32_t VPhysics2_Handle_Interface_001 = 0x5FA10;
        constexpr uint32_t VPhysics2_Interface_001 = 0x5B790;
    }

    namespace vscript_dll {
        constexpr uint32_t VScriptManager010 = 0x31C40;
    }

    namespace vstdlib_s64_dll {
        constexpr uint32_t IVALIDATE001 = 0x24FF0;
        constexpr uint32_t VEngineCvar002 = 0x5750;
    }

    namespace worldrenderer_dll {
        constexpr uint32_t WorldRendererMgr001 = 0x21280;
    }
}