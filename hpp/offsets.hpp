// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-09 13:46:49.828411900 +07:00

#pragma once

#include <cstddef>
#include <cstdint>

namespace source2_dumper {
    namespace offsets {
        // Module: client.dll
        namespace client_dll {
            constexpr std::ptrdiff_t dwEntityList = 0x65A7068;
            constexpr std::ptrdiff_t dwGameEntitySystem = 0x65A7068;
            constexpr std::ptrdiff_t dwGameEntitySystem_highestEntityIndex = 0x20C0;
            constexpr std::ptrdiff_t dwGlobalVars = 0x5B283D8;
            constexpr std::ptrdiff_t dwLocalPlayerPawn = 0x5B2F568;
            constexpr std::ptrdiff_t dwPrediction = 0x5B2F490;
            constexpr std::ptrdiff_t dwViewMatrix = 0x621EB30;
            constexpr std::ptrdiff_t dwViewRender = 0x621F668;
        }
        // Module: engine2.dll
        namespace engine2_dll {
            constexpr std::ptrdiff_t dwBuildNumber = 0x5B80F8;
            constexpr std::ptrdiff_t dwNetworkGameClient = 0x8B3C60;
            constexpr std::ptrdiff_t dwNetworkGameClient_clientTickCount = 0x398;
            constexpr std::ptrdiff_t dwNetworkGameClient_deltaTick = 0x24C;
            constexpr std::ptrdiff_t dwNetworkGameClient_isBackgroundMap = 0x2C143F;
            constexpr std::ptrdiff_t dwNetworkGameClient_localPlayer = 0xF8;
            constexpr std::ptrdiff_t dwNetworkGameClient_maxClients = 0x240;
            constexpr std::ptrdiff_t dwNetworkGameClient_serverTickCount = 0x24C;
            constexpr std::ptrdiff_t dwNetworkGameClient_signOnState = 0x230;
            constexpr std::ptrdiff_t dwWindowHeight = 0x8B8014;
            constexpr std::ptrdiff_t dwWindowWidth = 0x8B8010;
        }
        // Module: inputsystem.dll
        namespace inputsystem_dll {
            constexpr std::ptrdiff_t dwInputSystem = 0x46BC0;
        }
        // Module: panorama.dll
        namespace panorama_dll {
        }
        // Module: soundsystem.dll
        namespace soundsystem_dll {
            constexpr std::ptrdiff_t dwSoundSystem = 0x64AF00;
        }
    }
}
