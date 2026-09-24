// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-09-24 17:35:37.601127800 +07:00

namespace Source2Dumper.Schemas {
    // Module: materialsystem2.dll
    // Class count: 7
    // Enum count: 5
    public static class Materialsystem2Dll {
        // Alignment: 4
        // Member count: 4
        public enum VertJustification_e : uint {
            VERT_JUSTIFICATION_TOP = 0x0,
            VERT_JUSTIFICATION_CENTER = 0x1,
            VERT_JUSTIFICATION_BOTTOM = 0x2,
            VERT_JUSTIFICATION_NONE = 0x3
        }
        // Alignment: 4
        // Member count: 3
        public enum LayoutPositionType_e : uint {
            LAYOUTPOSITIONTYPE_VIEWPORT_RELATIVE = 0x0,
            LAYOUTPOSITIONTYPE_FRACTIONAL = 0x1,
            LAYOUTPOSITIONTYPE_NONE = 0x2
        }
        // Alignment: 4
        // Member count: 3
        public enum ViewFadeMode_t : uint {
            VIEW_FADE_CONSTANT_COLOR = 0x0,
            VIEW_FADE_MODULATE = 0x1,
            VIEW_FADE_MOD2X = 0x2
        }
        // Alignment: 4
        // Member count: 3
        public enum BloomBlendMode_t : uint {
            BLOOM_BLEND_ADD = 0x0,
            BLOOM_BLEND_SCREEN = 0x1,
            BLOOM_BLEND_BLUR = 0x2
        }
        // Alignment: 4
        // Member count: 4
        public enum HorizJustification_e : uint {
            HORIZ_JUSTIFICATION_LEFT = 0x0,
            HORIZ_JUSTIFICATION_CENTER = 0x1,
            HORIZ_JUSTIFICATION_RIGHT = 0x2,
            HORIZ_JUSTIFICATION_NONE = 0x3
        }
        // Parent: None
        // Field count: 15
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // VIEW_FADE_MODULATE
        // VIEW_FADE_MOD2X
        public static class PostProcessingResource_t {
            public const nint m_bHasTonemapParams = 0x0; // bool
            public const nint m_toneMapParams = 0x4; // PostProcessingTonemapParameters_t
            public const nint m_bHasBloomParams = 0x40; // bool
            public const nint m_bloomParams = 0x44; // PostProcessingBloomParameters_t
            public const nint m_bHasVignetteParams = 0xCC; // bool
            public const nint m_vignetteParams = 0xD0; // PostProcessingVignetteParameters_t
            public const nint m_bHasLocalContrastParams = 0xF4; // bool
            public const nint m_localConstrastParams = 0xF8; // PostProcessingLocalContrastParameters_t
            public const nint m_nColorCorrectionVolumeDim = 0x10C; // int32
            public const nint m_colorCorrectionVolumeData = 0x110; // CUtlBinaryBlock
            public const nint m_bHasColorCorrection = 0x120; // bool
            public const nint m_bHasFogScatteringParams = 0x121; // bool
            public const nint m_fogScatteringParams = 0x124; // PostProcessingFogScatteringParameters_t
            public const nint m_bHasLocalExposureParams = 0x144; // bool
            public const nint m_localExposureParams = 0x148; // PostProcessingLocalExposureParameters_t
        }
        // Parent: None
        // Field count: 6
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class PostProcessingVignetteParameters_t {
            public const nint m_flVignetteStrength = 0x0; // float32
            public const nint m_vCenter = 0x4; // Vector2D
            public const nint m_flRadius = 0xC; // float32
            public const nint m_flRoundness = 0x10; // float32
            public const nint m_flFeather = 0x14; // float32
            public const nint m_vColorTint = 0x18; // Vector
        }
        // Parent: None
        // Field count: 5
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class PostProcessingLocalContrastParameters_t {
            public const nint m_flLocalContrastStrength = 0x0; // float32
            public const nint m_flLocalContrastEdgeStrength = 0x4; // float32
            public const nint m_flLocalContrastVignetteStart = 0x8; // float32
            public const nint m_flLocalContrastVignetteEnd = 0xC; // float32
            public const nint m_flLocalContrastVignetteBlur = 0x10; // float32
        }
        // Parent: None
        // Field count: 15
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class PostProcessingTonemapParameters_t {
            public const nint m_flExposureBias = 0x0; // float32
            public const nint m_flShoulderStrength = 0x4; // float32
            public const nint m_flLinearStrength = 0x8; // float32
            public const nint m_flLinearAngle = 0xC; // float32
            public const nint m_flToeStrength = 0x10; // float32
            public const nint m_flToeNum = 0x14; // float32
            public const nint m_flToeDenom = 0x18; // float32
            public const nint m_flWhitePoint = 0x1C; // float32
            public const nint m_flLuminanceSource = 0x20; // float32
            public const nint m_flExposureBiasShadows = 0x24; // float32
            public const nint m_flExposureBiasHighlights = 0x28; // float32
            public const nint m_flMinShadowLum = 0x2C; // float32
            public const nint m_flMaxShadowLum = 0x30; // float32
            public const nint m_flMinHighlightLum = 0x34; // float32
            public const nint m_flMaxHighlightLum = 0x38; // float32
        }
        // Parent: None
        // Field count: 8
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class PostProcessingFogScatteringParameters_t {
            public const nint m_fRadius = 0x0; // float32
            public const nint m_fScale = 0x4; // float32
            public const nint m_fCubemapScale = 0x8; // float32
            public const nint m_fVolumetricScale = 0xC; // float32
            public const nint m_fGradientScale = 0x10; // float32
            public const nint m_fWaterScale = 0x14; // float32
            public const nint m_fWaterDensity = 0x18; // float32
            public const nint m_fWaterDepthBlurRadius = 0x1C; // float32
        }
        // Parent: None
        // Field count: 16
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class PostProcessingBloomParameters_t {
            public const nint m_blendMode = 0x0; // BloomBlendMode_t
            public const nint m_flBloomStrength = 0x4; // float32
            public const nint m_flScreenBloomStrength = 0x8; // float32
            public const nint m_flBlurBloomStrength = 0xC; // float32
            public const nint m_flBloomThreshold = 0x10; // float32
            public const nint m_flBloomThresholdWidth = 0x14; // float32
            public const nint m_flSkyboxBloomStrength = 0x18; // float32
            public const nint m_flBloomStartValue = 0x1C; // float32
            public const nint m_flComputeBloomStrength = 0x20; // float32
            public const nint m_flComputeBloomThreshold = 0x24; // float32
            public const nint m_flComputeBloomRadius = 0x28; // float32
            public const nint m_flComputeBloomEffectsScale = 0x2C; // float32
            public const nint m_flComputeBloomLensDirtStrength = 0x30; // float32
            public const nint m_flComputeBloomLensDirtBlackLevel = 0x34; // float32
            public const nint m_flBlurWeight = 0x38; // float32[5]
            public const nint m_vBlurTint = 0x4C; // Vector[5]
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // VIEW_FADE_MODULATE
        public static class PostProcessingLocalExposureParameters_t {
            public const nint m_fShadowOffsetEV = 0x0; // float32
            public const nint m_fHighlightOffsetEV = 0x4; // float32
            public const nint m_fSigma = 0x8; // float32
            public const nint m_fBoostLocalContrast = 0xC; // float32
        }
    }
}
