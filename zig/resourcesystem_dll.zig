// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-05-08 10:01:14.691628500 +07:00

pub const source2_dumper = struct {
    pub const schemas = struct {
        // Module: resourcesystem.dll
        // Class count: 59
        // Enum count: 2
        pub const resourcesystem_dll = struct {
            // Alignment: 1
            // Member count: 9
            pub const FuseVariableType_t = enum(u8) {
                INVALID = 0x0,
                BOOL = 0x1,
                INT8 = 0x2,
                INT16 = 0x3,
                INT32 = 0x4,
                UINT8 = 0x5,
                UINT16 = 0x6,
                UINT32 = 0x7,
                FLOAT32 = 0x8
            };
            // Alignment: 1
            // Member count: 2
            pub const FuseVariableAccess_t = enum(u8) {
                WRITABLE = 0x0,
                READ_ONLY = 0x1
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCResponseRulesList = struct {
            };
            // Parent: None
            // Field count: 2
            pub const AABBWS_t = struct {
                pub const m_vMinBounds: usize = 0x0; // VectorWS
                pub const m_vMaxBounds: usize = 0xC; // VectorWS
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCDotaItemDefinitionResource = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCMorphSetData = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCVSoundStackScriptList = struct {
            };
            // Parent: None
            // Field count: 2
            pub const PackedAABB_t = struct {
                pub const m_nPackedMin: usize = 0x0; // uint32
                pub const m_nPackedMax: usize = 0x4; // uint32
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCVPhysXSurfacePropertiesList = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeManifestTestResource_t = struct {
            };
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const ConstantInfo_t = struct {
                pub const m_name: usize = 0x0; // CUtlString
                pub const m_nameToken: usize = 0x8; // CUtlStringToken
                pub const m_flValue: usize = 0xC; // float32
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyCustomFGDType
            // vrman
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub const FuseFunctionIndex_t = struct {
                pub const m_Value: usize = 0x0; // uint16
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCGcExportableExternalData = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeIAnimGraphModelBinding = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCJavaScriptResource = struct {
            };
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CFuseSymbolTable = struct {
                pub const m_constants: usize = 0x0; // CUtlVector<ConstantInfo_t>
                pub const m_variables: usize = 0x18; // CUtlVector<VariableInfo_t>
                pub const m_functions: usize = 0x30; // CUtlVector<FunctionInfo_t>
                pub const m_constantMap: usize = 0x48; // CUtlHashtable<CUtlStringToken,int32>
                pub const m_variableMap: usize = 0x68; // CUtlHashtable<CUtlStringToken,int32>
                pub const m_functionMap: usize = 0x88; // CUtlHashtable<CUtlStringToken,int32>
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCRenderMesh = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCVoxelVisibility = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCPhysAggregateData = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCNmClip = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeWorld_t = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeProceduralTestResource_t = struct {
            };
            // Parent: None
            // Field count: 2
            pub const AABB_t = struct {
                pub const m_vMinBounds: usize = 0x0; // Vector
                pub const m_vMaxBounds: usize = 0xC; // Vector
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCPostProcessingResource = struct {
            };
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const VariableInfo_t = struct {
                pub const m_name: usize = 0x0; // CUtlString
                pub const m_nameToken: usize = 0x8; // CUtlStringToken
                pub const m_nIndex: usize = 0xC; // FuseVariableIndex_t
                pub const m_nNumComponents: usize = 0xE; // uint8
                pub const m_eVarType: usize = 0xF; // FuseVariableType_t
                pub const m_eAccess: usize = 0x10; // FuseVariableAccess_t
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeIParticleSnapshot = struct {
            };
            // Parent: None
            // Field count: 4
            pub const FourQuaternions = struct {
                pub const x: usize = 0x0; // fltx4
                pub const y: usize = 0x10; // fltx4
                pub const z: usize = 0x20; // fltx4
                pub const w: usize = 0x30; // fltx4
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCPanoramaLayout = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCTypeScriptResource = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCChoreoSceneResource = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCNmSkeleton = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCTestResourceData = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCAnimationGroup = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCVSoundEventScriptList = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCVoiceContainerBase = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCPanoramaStyle = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCWorldNode = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCSurfaceGraph = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCCSGOEconItem = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCNmGraphDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCSmartProp = struct {
            };
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const CFuseProgram = struct {
                pub const m_programBuffer: usize = 0x0; // CUtlVector<uint8>
                pub const m_variablesRead: usize = 0x18; // CUtlVector<FuseVariableIndex_t>
                pub const m_variablesWritten: usize = 0x30; // CUtlVector<FuseVariableIndex_t>
                pub const m_nMaxTempVarsUsed: usize = 0x48; // int32
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCCompositeMaterialKit = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCVMixListResource = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCAnimData = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeIMaterial2 = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeIVectorGraphic = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCPanoramaDynamicImages = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeIPulseGraphDef = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCVDataItemDefs = struct {
            };
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub const FunctionInfo_t = struct {
                pub const m_name: usize = 0x8; // CUtlString
                pub const m_nameToken: usize = 0x10; // CUtlStringToken
                pub const m_nParamCount: usize = 0x14; // int32
                pub const m_nIndex: usize = 0x18; // FuseFunctionIndex_t
                pub const m_bIsPure: usize = 0x1A; // bool
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCVDataResource = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCModel = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCDOTANovelsList = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCTextureBase = struct {
            };
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // kLh
            // MGetKV3ClassDefaults
            // MPropertyCustomFGDType
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // VApplication001
            // VEngineCvar007
            // VStringTokenSystem001
            // TestScriptMgr001
            // VProcessUtils002
            // VFileSystem017
            // VAsyncFileSystem2_001
            // ResourceSystem013
            // ResourceManifestRegistry001
            // ResourceHandleUtils001
            // SchemaSystem_001
            // ResourceCompilerSystem001
            // VMaterialSystem2_001
            // PostProcessingSystem_001
            // InputSystemVersion001
            // InputStackSystemVersion001
            // RenderDeviceMgr001
            // RenderUtils_001
            // SoundSystem001
            // SoundOpSystemEdit001
            // SoundOpSystem001
            // SteamAudio001
            // VP4003
            // Localize_001
            // VMediaFoundation001
            // VAvi001
            // VWebm001
            // VBik001
            // MeshSystem001
            // MeshUtils001
            // RenderDevice003
            // VRenderDeviceSetupV001
            // RenderHardwareConfig002
            // SceneSystem_002
            // IPulseSystem_001
            // SceneUtils_001
            // WorldRendererMgr001
            // AssetSystem001
            // AssetSystemTest001
            // ParticleSystemMgr003
            // VScriptManager010
            // PropertyEditorSystem_001
            // MATCHFRAMEWORK_001
            // Source2V8System001
            // PanoramaUIEngine001
            // PanoramaUIClient001
            // PanoramaTextServices001
            // ToolFramework2_002
            // PhysicsBuilderMgr001
            // VisBuilder_001
            // BakedLODBuilderMgr001
            // HelpSystem_001
            // ToolSceneNodeFactory_001
            // EconItemToolModel_001
            // SchemaTestExternal_Two_001
            // SchemaTestExternal_One_001
            // AnimationSystem_001
            // AnimationSystemUtils_001
            // HammerMapLoader001
            // MaterialUtils_001
            // FontManager_001
            // TextLayout_001
            // AssetPreviewSystem_001
            // AssetBrowserSystem_001
            // AssetRenameSystem_001
            // VConComm001
            // MODEL_PROCESSING_SERVICES_INTERFACE_001
            // NetworkSystemVersion001
            // NetworkMessagesVersion001
            // FlattenedSerializersVersion001
            // SerializedEntitiesVersion001
            // DemoUpconverterVersion001
            // Source2Client002
            // Source2ClientUI001
            // Source2ClientPrediction001
            // Source2Server001
            // Source2Host001
            // Source2ModTools001
            // Source2GameClients001
            // Source2GameEntities001
            // EngineServiceMgr001
            // HostStateMgr001
            // NetworkService_001
            // NetworkClientService_001
            // NetworkP2PService_001
            // NetworkServerService_001
            // ToolService_001
            // RenderService_001
            // StatsService_001
            // VProfService_001
            // InputService_001
            // MapListService_001
            // GameUIService_001
            // SoundService_001
            // BenchmarkService001
            // KeyValueCache001
            // ClientServerSharedHandleSystem001
            // GameResourceServiceClientV001
            // GameResourceServiceServerV001
            // Source2EngineToClient001
            // Source2EngineToServer001
            // Source2EngineToServerStringTable001
            // Source2EngineToClientStringTable001
            // VPhysics2_Interface_001
            // ModelDocUtils001
            // AnimGraphEditorUtils001
            // EXPORTSYSTEM_INTERFACE_VERSION_001
            // ServerToolsInfo_001
            // ClientToolsInfo_001
            // Vrad3_001
            // NavSystem001
            // NavGameTest001
            pub const FuseVariableIndex_t = struct {
                pub const m_Value: usize = 0x0; // uint16
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeIParticleSystemDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCSequenceGroupData = struct {
            };
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub const ManifestTestResource_t = struct {
                pub const m_name: usize = 0x0; // CUtlString
                pub const m_child: usize = 0x8; // CStrongHandle<InfoForResourceTypeManifestTestResource_t>
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCEntityLump = struct {
            };
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MResourceTypeForInfoType
            // MKV3TransferName
            // MResourceTypeForInfoType
            // MGetKV3ClassDefaults
            pub const InfoForResourceTypeCDOTAPatchNotesList = struct {
            };
        };
    };
};
