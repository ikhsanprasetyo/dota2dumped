// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-09-24 17:35:37.601127800 +07:00

#pragma once

#include <cstddef>
#include <cstdint>

namespace source2_dumper {
    namespace schemas {
        // Module: materialsystem2.dll
        // Class count: 7
        // Enum count: 5
        namespace materialsystem2_dll {
            // Alignment: 4
            // Member count: 4
            enum class VertJustification_e : uint32_t {
                VERT_JUSTIFICATION_TOP = 0x0,
                VERT_JUSTIFICATION_CENTER = 0x1,
                VERT_JUSTIFICATION_BOTTOM = 0x2,
                VERT_JUSTIFICATION_NONE = 0x3
            };
            // Alignment: 4
            // Member count: 3
            enum class LayoutPositionType_e : uint32_t {
                LAYOUTPOSITIONTYPE_VIEWPORT_RELATIVE = 0x0,
                LAYOUTPOSITIONTYPE_FRACTIONAL = 0x1,
                LAYOUTPOSITIONTYPE_NONE = 0x2
            };
            // Alignment: 4
            // Member count: 3
            enum class ViewFadeMode_t : uint32_t {
                VIEW_FADE_CONSTANT_COLOR = 0x0,
                VIEW_FADE_MODULATE = 0x1,
                VIEW_FADE_MOD2X = 0x2
            };
            // Alignment: 4
            // Member count: 3
            enum class BloomBlendMode_t : uint32_t {
                BLOOM_BLEND_ADD = 0x0,
                BLOOM_BLEND_SCREEN = 0x1,
                BLOOM_BLEND_BLUR = 0x2
            };
            // Alignment: 4
            // Member count: 4
            enum class HorizJustification_e : uint32_t {
                HORIZ_JUSTIFICATION_LEFT = 0x0,
                HORIZ_JUSTIFICATION_CENTER = 0x1,
                HORIZ_JUSTIFICATION_RIGHT = 0x2,
                HORIZ_JUSTIFICATION_NONE = 0x3
            };
            // Parent: None
            // Field count: 15
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // VIEW_FADE_MODULATE
            // VIEW_FADE_MOD2X
            namespace PostProcessingResource_t {
                constexpr std::ptrdiff_t m_bHasTonemapParams = 0x0; // bool
                constexpr std::ptrdiff_t m_toneMapParams = 0x4; // PostProcessingTonemapParameters_t
                constexpr std::ptrdiff_t m_bHasBloomParams = 0x40; // bool
                constexpr std::ptrdiff_t m_bloomParams = 0x44; // PostProcessingBloomParameters_t
                constexpr std::ptrdiff_t m_bHasVignetteParams = 0xCC; // bool
                constexpr std::ptrdiff_t m_vignetteParams = 0xD0; // PostProcessingVignetteParameters_t
                constexpr std::ptrdiff_t m_bHasLocalContrastParams = 0xF4; // bool
                constexpr std::ptrdiff_t m_localConstrastParams = 0xF8; // PostProcessingLocalContrastParameters_t
                constexpr std::ptrdiff_t m_nColorCorrectionVolumeDim = 0x10C; // int32
                constexpr std::ptrdiff_t m_colorCorrectionVolumeData = 0x110; // CUtlBinaryBlock
                constexpr std::ptrdiff_t m_bHasColorCorrection = 0x120; // bool
                constexpr std::ptrdiff_t m_bHasFogScatteringParams = 0x121; // bool
                constexpr std::ptrdiff_t m_fogScatteringParams = 0x124; // PostProcessingFogScatteringParameters_t
                constexpr std::ptrdiff_t m_bHasLocalExposureParams = 0x144; // bool
                constexpr std::ptrdiff_t m_localExposureParams = 0x148; // PostProcessingLocalExposureParameters_t
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace PostProcessingVignetteParameters_t {
                constexpr std::ptrdiff_t m_flVignetteStrength = 0x0; // float32
                constexpr std::ptrdiff_t m_vCenter = 0x4; // Vector2D
                constexpr std::ptrdiff_t m_flRadius = 0xC; // float32
                constexpr std::ptrdiff_t m_flRoundness = 0x10; // float32
                constexpr std::ptrdiff_t m_flFeather = 0x14; // float32
                constexpr std::ptrdiff_t m_vColorTint = 0x18; // Vector
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace PostProcessingLocalContrastParameters_t {
                constexpr std::ptrdiff_t m_flLocalContrastStrength = 0x0; // float32
                constexpr std::ptrdiff_t m_flLocalContrastEdgeStrength = 0x4; // float32
                constexpr std::ptrdiff_t m_flLocalContrastVignetteStart = 0x8; // float32
                constexpr std::ptrdiff_t m_flLocalContrastVignetteEnd = 0xC; // float32
                constexpr std::ptrdiff_t m_flLocalContrastVignetteBlur = 0x10; // float32
            }
            // Parent: None
            // Field count: 15
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace PostProcessingTonemapParameters_t {
                constexpr std::ptrdiff_t m_flExposureBias = 0x0; // float32
                constexpr std::ptrdiff_t m_flShoulderStrength = 0x4; // float32
                constexpr std::ptrdiff_t m_flLinearStrength = 0x8; // float32
                constexpr std::ptrdiff_t m_flLinearAngle = 0xC; // float32
                constexpr std::ptrdiff_t m_flToeStrength = 0x10; // float32
                constexpr std::ptrdiff_t m_flToeNum = 0x14; // float32
                constexpr std::ptrdiff_t m_flToeDenom = 0x18; // float32
                constexpr std::ptrdiff_t m_flWhitePoint = 0x1C; // float32
                constexpr std::ptrdiff_t m_flLuminanceSource = 0x20; // float32
                constexpr std::ptrdiff_t m_flExposureBiasShadows = 0x24; // float32
                constexpr std::ptrdiff_t m_flExposureBiasHighlights = 0x28; // float32
                constexpr std::ptrdiff_t m_flMinShadowLum = 0x2C; // float32
                constexpr std::ptrdiff_t m_flMaxShadowLum = 0x30; // float32
                constexpr std::ptrdiff_t m_flMinHighlightLum = 0x34; // float32
                constexpr std::ptrdiff_t m_flMaxHighlightLum = 0x38; // float32
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace PostProcessingFogScatteringParameters_t {
                constexpr std::ptrdiff_t m_fRadius = 0x0; // float32
                constexpr std::ptrdiff_t m_fScale = 0x4; // float32
                constexpr std::ptrdiff_t m_fCubemapScale = 0x8; // float32
                constexpr std::ptrdiff_t m_fVolumetricScale = 0xC; // float32
                constexpr std::ptrdiff_t m_fGradientScale = 0x10; // float32
                constexpr std::ptrdiff_t m_fWaterScale = 0x14; // float32
                constexpr std::ptrdiff_t m_fWaterDensity = 0x18; // float32
                constexpr std::ptrdiff_t m_fWaterDepthBlurRadius = 0x1C; // float32
            }
            // Parent: None
            // Field count: 16
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace PostProcessingBloomParameters_t {
                constexpr std::ptrdiff_t m_blendMode = 0x0; // BloomBlendMode_t
                constexpr std::ptrdiff_t m_flBloomStrength = 0x4; // float32
                constexpr std::ptrdiff_t m_flScreenBloomStrength = 0x8; // float32
                constexpr std::ptrdiff_t m_flBlurBloomStrength = 0xC; // float32
                constexpr std::ptrdiff_t m_flBloomThreshold = 0x10; // float32
                constexpr std::ptrdiff_t m_flBloomThresholdWidth = 0x14; // float32
                constexpr std::ptrdiff_t m_flSkyboxBloomStrength = 0x18; // float32
                constexpr std::ptrdiff_t m_flBloomStartValue = 0x1C; // float32
                constexpr std::ptrdiff_t m_flComputeBloomStrength = 0x20; // float32
                constexpr std::ptrdiff_t m_flComputeBloomThreshold = 0x24; // float32
                constexpr std::ptrdiff_t m_flComputeBloomRadius = 0x28; // float32
                constexpr std::ptrdiff_t m_flComputeBloomEffectsScale = 0x2C; // float32
                constexpr std::ptrdiff_t m_flComputeBloomLensDirtStrength = 0x30; // float32
                constexpr std::ptrdiff_t m_flComputeBloomLensDirtBlackLevel = 0x34; // float32
                constexpr std::ptrdiff_t m_flBlurWeight = 0x38; // float32[5]
                constexpr std::ptrdiff_t m_vBlurTint = 0x4C; // Vector[5]
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // VIEW_FADE_MODULATE
            namespace PostProcessingLocalExposureParameters_t {
                constexpr std::ptrdiff_t m_fShadowOffsetEV = 0x0; // float32
                constexpr std::ptrdiff_t m_fHighlightOffsetEV = 0x4; // float32
                constexpr std::ptrdiff_t m_fSigma = 0x8; // float32
                constexpr std::ptrdiff_t m_fBoostLocalContrast = 0xC; // float32
            }
        }
    }
}
