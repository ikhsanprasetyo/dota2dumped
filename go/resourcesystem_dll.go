// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-04-29 12:46:53.186668100 +07:00

package schemas

const (
    ResourcesystemDll_FuseVariableType_t_INVALID = 0x0
    ResourcesystemDll_FuseVariableType_t_BOOL = 0x1
    ResourcesystemDll_FuseVariableType_t_INT8 = 0x2
    ResourcesystemDll_FuseVariableType_t_INT16 = 0x3
    ResourcesystemDll_FuseVariableType_t_INT32 = 0x4
    ResourcesystemDll_FuseVariableType_t_UINT8 = 0x5
    ResourcesystemDll_FuseVariableType_t_UINT16 = 0x6
    ResourcesystemDll_FuseVariableType_t_UINT32 = 0x7
    ResourcesystemDll_FuseVariableType_t_FLOAT32 = 0x8
    ResourcesystemDll_FuseVariableAccess_t_WRITABLE = 0x0
    ResourcesystemDll_FuseVariableAccess_t_READ_ONLY = 0x1
    ResourcesystemDll_AABBWS_t_m_vMinBounds = 0x0 // VectorWS
    ResourcesystemDll_AABBWS_t_m_vMaxBounds = 0xC // VectorWS
    ResourcesystemDll_PackedAABB_t_m_nPackedMin = 0x0 // uint32
    ResourcesystemDll_PackedAABB_t_m_nPackedMax = 0x4 // uint32
    ResourcesystemDll_ConstantInfo_t_m_name = 0x0 // CUtlString
    ResourcesystemDll_ConstantInfo_t_m_nameToken = 0x8 // CUtlStringToken
    ResourcesystemDll_ConstantInfo_t_m_flValue = 0xC // float32
    ResourcesystemDll_FuseFunctionIndex_t_m_Value = 0x0 // uint16
    ResourcesystemDll_CFuseSymbolTable_m_constants = 0x0 // CUtlVector<ConstantInfo_t>
    ResourcesystemDll_CFuseSymbolTable_m_variables = 0x18 // CUtlVector<VariableInfo_t>
    ResourcesystemDll_CFuseSymbolTable_m_functions = 0x30 // CUtlVector<FunctionInfo_t>
    ResourcesystemDll_CFuseSymbolTable_m_constantMap = 0x48 // CUtlHashtable<CUtlStringToken,int32>
    ResourcesystemDll_CFuseSymbolTable_m_variableMap = 0x68 // CUtlHashtable<CUtlStringToken,int32>
    ResourcesystemDll_CFuseSymbolTable_m_functionMap = 0x88 // CUtlHashtable<CUtlStringToken,int32>
    ResourcesystemDll_AABB_t_m_vMinBounds = 0x0 // Vector
    ResourcesystemDll_AABB_t_m_vMaxBounds = 0xC // Vector
    ResourcesystemDll_VariableInfo_t_m_name = 0x0 // CUtlString
    ResourcesystemDll_VariableInfo_t_m_nameToken = 0x8 // CUtlStringToken
    ResourcesystemDll_VariableInfo_t_m_nIndex = 0xC // FuseVariableIndex_t
    ResourcesystemDll_VariableInfo_t_m_nNumComponents = 0xE // uint8
    ResourcesystemDll_VariableInfo_t_m_eVarType = 0xF // FuseVariableType_t
    ResourcesystemDll_VariableInfo_t_m_eAccess = 0x10 // FuseVariableAccess_t
    ResourcesystemDll_FourQuaternions_x = 0x0 // fltx4
    ResourcesystemDll_FourQuaternions_y = 0x10 // fltx4
    ResourcesystemDll_FourQuaternions_z = 0x20 // fltx4
    ResourcesystemDll_FourQuaternions_w = 0x30 // fltx4
    ResourcesystemDll_CFuseProgram_m_programBuffer = 0x0 // CUtlVector<uint8>
    ResourcesystemDll_CFuseProgram_m_variablesRead = 0x18 // CUtlVector<FuseVariableIndex_t>
    ResourcesystemDll_CFuseProgram_m_variablesWritten = 0x30 // CUtlVector<FuseVariableIndex_t>
    ResourcesystemDll_CFuseProgram_m_nMaxTempVarsUsed = 0x48 // int32
    ResourcesystemDll_FunctionInfo_t_m_name = 0x8 // CUtlString
    ResourcesystemDll_FunctionInfo_t_m_nameToken = 0x10 // CUtlStringToken
    ResourcesystemDll_FunctionInfo_t_m_nParamCount = 0x14 // int32
    ResourcesystemDll_FunctionInfo_t_m_nIndex = 0x18 // FuseFunctionIndex_t
    ResourcesystemDll_FunctionInfo_t_m_bIsPure = 0x1A // bool
    ResourcesystemDll_FuseVariableIndex_t_m_Value = 0x0 // uint16
    ResourcesystemDll_ManifestTestResource_t_m_name = 0x0 // CUtlString
    ResourcesystemDll_ManifestTestResource_t_m_child = 0x8 // CStrongHandle<InfoForResourceTypeManifestTestResource_t>
)
