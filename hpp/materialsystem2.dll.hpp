// printed at 2023-11-29 20:52:40.216445200 +07:00	from Dota2 7.34d
#pragma once
#include <cstdint>

namespace Offsets {
    namespace MaterialParamBuffer_t {
        constexpr uint32_t m_value = 0x8; // CUtlBinaryBlock
    }

    namespace MaterialParamFloat_t {
        constexpr uint32_t m_flValue = 0x8; // float32
    }

    namespace MaterialParamInt_t {
        constexpr uint32_t m_nValue = 0x8; // int32
    }

    namespace MaterialParamString_t {
        constexpr uint32_t m_value = 0x8; // CUtlString
    }

    namespace MaterialParamTexture_t {
        constexpr uint32_t m_pValue = 0x8; // CStrongHandle< InfoForResourceTypeCTextureBase >
    }

    namespace MaterialParamVector_t {
        constexpr uint32_t m_value = 0x8; // Vector4D
    }

    namespace MaterialParam_t {
        constexpr uint32_t m_name = 0x0; // CUtlString
    }

    namespace MaterialResourceData_t {
        constexpr uint32_t m_materialName = 0x0; // CUtlString
        constexpr uint32_t m_shaderName = 0x8; // CUtlString
        constexpr uint32_t m_intParams = 0x10; // CUtlVector< MaterialParamInt_t >
        constexpr uint32_t m_floatParams = 0x28; // CUtlVector< MaterialParamFloat_t >
        constexpr uint32_t m_vectorParams = 0x40; // CUtlVector< MaterialParamVector_t >
        constexpr uint32_t m_textureParams = 0x58; // CUtlVector< MaterialParamTexture_t >
        constexpr uint32_t m_dynamicParams = 0x70; // CUtlVector< MaterialParamBuffer_t >
        constexpr uint32_t m_dynamicTextureParams = 0x88; // CUtlVector< MaterialParamBuffer_t >
        constexpr uint32_t m_intAttributes = 0xA0; // CUtlVector< MaterialParamInt_t >
        constexpr uint32_t m_floatAttributes = 0xB8; // CUtlVector< MaterialParamFloat_t >
        constexpr uint32_t m_vectorAttributes = 0xD0; // CUtlVector< MaterialParamVector_t >
        constexpr uint32_t m_textureAttributes = 0xE8; // CUtlVector< MaterialParamTexture_t >
        constexpr uint32_t m_stringAttributes = 0x100; // CUtlVector< MaterialParamString_t >
        constexpr uint32_t m_renderAttributesUsed = 0x118; // CUtlVector< CUtlString >
    }

    namespace PostProcessingBloomParameters_t {
        constexpr uint32_t m_blendMode = 0x0; // BloomBlendMode_t
        constexpr uint32_t m_flBloomStrength = 0x4; // float32
        constexpr uint32_t m_flScreenBloomStrength = 0x8; // float32
        constexpr uint32_t m_flBlurBloomStrength = 0xC; // float32
        constexpr uint32_t m_flBloomThreshold = 0x10; // float32
        constexpr uint32_t m_flBloomThresholdWidth = 0x14; // float32
        constexpr uint32_t m_flSkyboxBloomStrength = 0x18; // float32
        constexpr uint32_t m_flBloomStartValue = 0x1C; // float32
        constexpr uint32_t m_flBlurWeight = 0x20; // float32[5]
        constexpr uint32_t m_vBlurTint = 0x34; // Vector[5]
    }

    namespace PostProcessingLocalContrastParameters_t {
        constexpr uint32_t m_flLocalContrastStrength = 0x0; // float32
        constexpr uint32_t m_flLocalContrastEdgeStrength = 0x4; // float32
        constexpr uint32_t m_flLocalContrastVignetteStart = 0x8; // float32
        constexpr uint32_t m_flLocalContrastVignetteEnd = 0xC; // float32
        constexpr uint32_t m_flLocalContrastVignetteBlur = 0x10; // float32
    }

    namespace PostProcessingResource_t {
        constexpr uint32_t m_bHasTonemapParams = 0x0; // bool
        constexpr uint32_t m_toneMapParams = 0x4; // PostProcessingTonemapParameters_t
        constexpr uint32_t m_bHasBloomParams = 0x40; // bool
        constexpr uint32_t m_bloomParams = 0x44; // PostProcessingBloomParameters_t
        constexpr uint32_t m_bHasVignetteParams = 0xB4; // bool
        constexpr uint32_t m_vignetteParams = 0xB8; // PostProcessingVignetteParameters_t
        constexpr uint32_t m_bHasLocalContrastParams = 0xDC; // bool
        constexpr uint32_t m_localConstrastParams = 0xE0; // PostProcessingLocalContrastParameters_t
        constexpr uint32_t m_nColorCorrectionVolumeDim = 0xF4; // int32
        constexpr uint32_t m_colorCorrectionVolumeData = 0xF8; // CUtlBinaryBlock
        constexpr uint32_t m_bHasColorCorrection = 0x110; // bool
    }

    namespace PostProcessingTonemapParameters_t {
        constexpr uint32_t m_flExposureBias = 0x0; // float32
        constexpr uint32_t m_flShoulderStrength = 0x4; // float32
        constexpr uint32_t m_flLinearStrength = 0x8; // float32
        constexpr uint32_t m_flLinearAngle = 0xC; // float32
        constexpr uint32_t m_flToeStrength = 0x10; // float32
        constexpr uint32_t m_flToeNum = 0x14; // float32
        constexpr uint32_t m_flToeDenom = 0x18; // float32
        constexpr uint32_t m_flWhitePoint = 0x1C; // float32
        constexpr uint32_t m_flLuminanceSource = 0x20; // float32
        constexpr uint32_t m_flExposureBiasShadows = 0x24; // float32
        constexpr uint32_t m_flExposureBiasHighlights = 0x28; // float32
        constexpr uint32_t m_flMinShadowLum = 0x2C; // float32
        constexpr uint32_t m_flMaxShadowLum = 0x30; // float32
        constexpr uint32_t m_flMinHighlightLum = 0x34; // float32
        constexpr uint32_t m_flMaxHighlightLum = 0x38; // float32
    }

    namespace PostProcessingVignetteParameters_t {
        constexpr uint32_t m_flVignetteStrength = 0x0; // float32
        constexpr uint32_t m_vCenter = 0x4; // Vector2D
        constexpr uint32_t m_flRadius = 0xC; // float32
        constexpr uint32_t m_flRoundness = 0x10; // float32
        constexpr uint32_t m_flFeather = 0x14; // float32
        constexpr uint32_t m_vColorTint = 0x18; // Vector
    }
}