// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-07-19 03:33:58.207251400 +07:00

#pragma once

#include <cstddef>
#include <cstdint>

namespace source2_dumper {
    namespace offsets {
        // Module: client.dll
        namespace client_dll {
            constexpr std::ptrdiff_t dwEntityList = 0x6494BF0;
            constexpr std::ptrdiff_t dwGameEntitySystem = 0x6494BF0;
            constexpr std::ptrdiff_t dwGameEntitySystem_highestEntityIndex = 0x2090;
            constexpr std::ptrdiff_t dwGlobalVars = 0x59EDFE0;
            constexpr std::ptrdiff_t dwLocalPlayerPawn = 0x59F6F88;
            constexpr std::ptrdiff_t dwPrediction = 0x59F6EB0;
            constexpr std::ptrdiff_t dwViewMatrix = 0x611D0F0;
            constexpr std::ptrdiff_t dwViewRender = 0x611C7B8;
        }
        // Module: engine2.dll
        namespace engine2_dll {
            constexpr std::ptrdiff_t dwBuildNumber = 0x60C694;
            constexpr std::ptrdiff_t dwNetworkGameClient = 0x90A540;
            constexpr std::ptrdiff_t dwNetworkGameClient_clientTickCount = 0x378;
            constexpr std::ptrdiff_t dwNetworkGameClient_deltaTick = 0x24C;
            constexpr std::ptrdiff_t dwNetworkGameClient_localPlayer = 0xF8;
            constexpr std::ptrdiff_t dwNetworkGameClient_maxClients = 0x240;
            constexpr std::ptrdiff_t dwNetworkGameClient_serverTickCount = 0x24C;
            constexpr std::ptrdiff_t dwNetworkGameClient_signOnState = 0x230;
            constexpr std::ptrdiff_t dwWindowHeight = 0x90E90C;
            constexpr std::ptrdiff_t dwWindowWidth = 0x90E908;
        }
        // Module: inputsystem.dll
        namespace inputsystem_dll {
            constexpr std::ptrdiff_t dwInputSystem = 0x45BA0;
        }
        // Module: panorama.dll
        namespace panorama_dll {
        }
        // Module: soundsystem.dll
        namespace soundsystem_dll {
            constexpr std::ptrdiff_t dwSoundSystem_engineViewData = 0x7C;
        }
    }
}
