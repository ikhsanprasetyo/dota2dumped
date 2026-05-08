// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-05-08 10:01:14.691628500 +07:00

#pragma once

#include <cstddef>
#include <cstdint>

namespace source2_dumper {
    namespace offsets {
        // Module: client.dll
        namespace client_dll {
            constexpr std::ptrdiff_t dwEntityList = 0x66DF288;
            constexpr std::ptrdiff_t dwGameEntitySystem = 0x66DF288;
            constexpr std::ptrdiff_t dwGameEntitySystem_highestEntityIndex = 0x2090;
            constexpr std::ptrdiff_t dwGlobalVars = 0x5C9C4D0;
            constexpr std::ptrdiff_t dwLocalPlayerPawn = 0x5CA76F8;
            constexpr std::ptrdiff_t dwPrediction = 0x5CA7620;
            constexpr std::ptrdiff_t dwSensitivity = 0x6385898;
            constexpr std::ptrdiff_t dwSensitivity_sensitivity = 0x58;
            constexpr std::ptrdiff_t dwViewMatrix = 0x638D4D0;
            constexpr std::ptrdiff_t dwViewRender = 0x638CBB8;
        }
        // Module: engine2.dll
        namespace engine2_dll {
            constexpr std::ptrdiff_t dwBuildNumber = 0x610CA4;
            constexpr std::ptrdiff_t dwNetworkGameClient = 0x90E500;
            constexpr std::ptrdiff_t dwNetworkGameClient_clientTickCount = 0x378;
            constexpr std::ptrdiff_t dwNetworkGameClient_deltaTick = 0x24C;
            constexpr std::ptrdiff_t dwNetworkGameClient_isBackgroundMap = 0x2C141F;
            constexpr std::ptrdiff_t dwNetworkGameClient_localPlayer = 0xF8;
            constexpr std::ptrdiff_t dwNetworkGameClient_maxClients = 0x240;
            constexpr std::ptrdiff_t dwNetworkGameClient_serverTickCount = 0x24C;
            constexpr std::ptrdiff_t dwNetworkGameClient_signOnState = 0x230;
            constexpr std::ptrdiff_t dwWindowHeight = 0x9128DC;
            constexpr std::ptrdiff_t dwWindowWidth = 0x9128D8;
        }
        // Module: inputsystem.dll
        namespace inputsystem_dll {
            constexpr std::ptrdiff_t dwInputSystem = 0x42B50;
        }
        // Module: panorama.dll
        namespace panorama_dll {
        }
        // Module: soundsystem.dll
        namespace soundsystem_dll {
            constexpr std::ptrdiff_t dwSoundSystem = 0x5133A0;
            constexpr std::ptrdiff_t dwSoundSystem_engineViewData = 0x7C;
        }
    }
}
