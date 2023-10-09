#![allow(non_snake_case, non_upper_case_globals)]

pub mod ActiveModelConfig_t {
    pub const m_Handle: usize = 0x28; // ModelConfigHandle_t
    pub const m_Name: usize = 0x30; // CUtlSymbolLarge
    pub const m_AssociatedEntities: usize = 0x38; // C_NetworkUtlVectorBase< CHandle< C_BaseModelEntity > >
    pub const m_AssociatedEntityNames: usize = 0x50; // C_NetworkUtlVectorBase< CUtlSymbolLarge >
}

pub mod CAmbientCreatures {
    pub const m_szAnimationName: usize = 0x800; // CUtlString
}

pub mod CAnimGraphNetworkedVariables {
    pub const m_PredNetBoolVariables: usize = 0x8; // C_NetworkUtlVectorBase< uint32 >
    pub const m_PredNetByteVariables: usize = 0x20; // C_NetworkUtlVectorBase< uint8 >
    pub const m_PredNetUInt16Variables: usize = 0x38; // C_NetworkUtlVectorBase< uint16 >
    pub const m_PredNetIntVariables: usize = 0x50; // C_NetworkUtlVectorBase< int32 >
    pub const m_PredNetUInt32Variables: usize = 0x68; // C_NetworkUtlVectorBase< uint32 >
    pub const m_PredNetUInt64Variables: usize = 0x80; // C_NetworkUtlVectorBase< uint64 >
    pub const m_PredNetFloatVariables: usize = 0x98; // C_NetworkUtlVectorBase< float32 >
    pub const m_PredNetVectorVariables: usize = 0xB0; // C_NetworkUtlVectorBase< Vector >
    pub const m_PredNetQuaternionVariables: usize = 0xC8; // C_NetworkUtlVectorBase< Quaternion >
    pub const m_OwnerOnlyPredNetBoolVariables: usize = 0xE0; // C_NetworkUtlVectorBase< uint32 >
    pub const m_OwnerOnlyPredNetByteVariables: usize = 0xF8; // C_NetworkUtlVectorBase< uint8 >
    pub const m_OwnerOnlyPredNetUInt16Variables: usize = 0x110; // C_NetworkUtlVectorBase< uint16 >
    pub const m_OwnerOnlyPredNetIntVariables: usize = 0x128; // C_NetworkUtlVectorBase< int32 >
    pub const m_OwnerOnlyPredNetUInt32Variables: usize = 0x140; // C_NetworkUtlVectorBase< uint32 >
    pub const m_OwnerOnlyPredNetUInt64Variables: usize = 0x158; // C_NetworkUtlVectorBase< uint64 >
    pub const m_OwnerOnlyPredNetFloatVariables: usize = 0x170; // C_NetworkUtlVectorBase< float32 >
    pub const m_OwnerOnlyPredNetVectorVariables: usize = 0x188; // C_NetworkUtlVectorBase< Vector >
    pub const m_OwnerOnlyPredNetQuaternionVariables: usize = 0x1A0; // C_NetworkUtlVectorBase< Quaternion >
    pub const m_nBoolVariablesCount: usize = 0x1B8; // int32
    pub const m_nOwnerOnlyBoolVariablesCount: usize = 0x1BC; // int32
    pub const m_nRandomSeedOffset: usize = 0x1C0; // int32
    pub const m_flLastTeleportTime: usize = 0x1C4; // float32
}

pub mod CAnimationLayer {
    pub const m_op: usize = 0x30; // CNetworkedSequenceOperation
    pub const m_nOrder: usize = 0x58; // int32
    pub const m_flPlaybackRate: usize = 0x5C; // CNetworkedQuantizedFloat
    pub const m_bLooping: usize = 0x64; // bool
    pub const m_nNewSequenceParity: usize = 0x68; // int32
    pub const m_nFlags: usize = 0x6C; // int32
    pub const m_bSequenceFinished: usize = 0x70; // bool
    pub const m_flKillRate: usize = 0x74; // float32
    pub const m_flKillDelay: usize = 0x78; // float32
    pub const m_flLayerAnimtime: usize = 0x7C; // float32
    pub const m_flLayerFadeOuttime: usize = 0x80; // float32
    pub const m_nActivity: usize = 0x84; // int32
    pub const m_nPriority: usize = 0x88; // int32
    pub const m_flLastEventCycle: usize = 0x8C; // float32
    pub const m_flLastAccess: usize = 0x90; // float32
}

pub mod CAnnouncerDescriptor {
    pub const m_strAnnouncerName: usize = 0x8; // CUtlString
    pub const m_strAnnouncerVoiceFile: usize = 0x10; // CUtlString
    pub const m_nAnnouncerItemId: usize = 0x18; // itemid_t
    pub const m_bItemOwnedByLocalPlayer: usize = 0x20; // bool
}

pub mod CAttributeContainer {
    pub const m_Item: usize = 0x68; // C_EconItemView
}

pub mod CAttributeList {
    pub const m_Attributes: usize = 0x8; // C_UtlVectorEmbeddedNetworkVar< C_EconItemAttribute >
    pub const m_pManager: usize = 0x58; // CAttributeManager*
}

pub mod CAttributeManager {
    pub const m_Providers: usize = 0x8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_Receivers: usize = 0x20; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_iReapplyProvisionParity: usize = 0x38; // int32
    pub const m_hOuter: usize = 0x3C; // CHandle< C_BaseEntity >
    pub const m_bPreventLoopback: usize = 0x40; // bool
    pub const m_ProviderType: usize = 0x44; // attributeprovidertypes_t
    pub const m_CachedResults: usize = 0x48; // CUtlVector< CAttributeManager::cached_attribute_float_t >
}

pub mod CAttributeManager_cached_attribute_float_t {
    pub const flIn: usize = 0x0; // float32
    pub const iAttribHook: usize = 0x8; // CUtlSymbolLarge
    pub const flOut: usize = 0x10; // float32
}

pub mod CBaseAnimGraph {
    pub const m_bInitiallyPopulateInterpHistory: usize = 0x7B8; // bool
    pub const m_bShouldAnimateDuringGameplayPause: usize = 0x7B9; // bool
    pub const m_bSuppressAnimEventSounds: usize = 0x7BB; // bool
    pub const m_bAnimGraphUpdateEnabled: usize = 0x7C8; // bool
    pub const m_flMaxSlopeDistance: usize = 0x7CC; // float32
    pub const m_vLastSlopeCheckPos: usize = 0x7D0; // Vector
    pub const m_vecForce: usize = 0x7E0; // Vector
    pub const m_nForceBone: usize = 0x7EC; // int32
    pub const m_pClientsideRagdoll: usize = 0x7F0; // CBaseAnimGraph*
    pub const m_bBuiltRagdoll: usize = 0x7F8; // bool
    pub const m_pRagdollPose: usize = 0x810; // PhysicsRagdollPose_t*
    pub const m_bClientRagdoll: usize = 0x818; // bool
    pub const m_bHasAnimatedMaterialAttributes: usize = 0x828; // bool
}

pub mod CBaseAnimGraphController {
    pub const m_baseLayer: usize = 0x18; // CNetworkedSequenceOperation
    pub const m_bSequenceFinished: usize = 0x40; // bool
    pub const m_flLastEventCycle: usize = 0x44; // float32
    pub const m_flLastEventAnimTime: usize = 0x48; // float32
    pub const m_flPlaybackRate: usize = 0x4C; // CNetworkedQuantizedFloat
    pub const m_flPrevAnimTime: usize = 0x54; // float32
    pub const m_bClientSideAnimation: usize = 0x58; // bool
    pub const m_bNetworkedAnimationInputsChanged: usize = 0x59; // bool
    pub const m_nPrevNewSequenceParity: usize = 0x5A; // uint8
    pub const m_nPrevResetEventsParity: usize = 0x5B; // uint8
    pub const m_nNewSequenceParity: usize = 0x5C; // int32
    pub const m_nResetEventsParity: usize = 0x60; // int32
    pub const m_nAnimLoopMode: usize = 0x64; // AnimLoopMode_t
    pub const m_hAnimationUpdate: usize = 0x104; // AnimationUpdateListHandle_t
    pub const m_hLastAnimEventSequence: usize = 0x108; // HSequence
}

pub mod CBaseAnimatingActivity {
    pub const m_bShouldAnimateDuringGameplayPause: usize = 0x7B8; // bool
    pub const m_bInitiallyPopulateInterpHistory: usize = 0x7B9; // bool
    pub const m_pSuppressedAnimEventTags: usize = 0x7C0; // CUtlVector< CUtlString >*
    pub const m_bHasAnimatedMaterialAttributes: usize = 0x7C8; // bool
    pub const m_bSuppressAnimEventSounds: usize = 0x7D8; // bool
}

pub mod CBasePlayerController {
    pub const m_nFinalPredictedTick: usize = 0x540; // int32
    pub const m_CommandContext: usize = 0x548; // C_CommandContext
    pub const m_nInButtonsWhichAreToggles: usize = 0x5C0; // uint64
    pub const m_nTickBase: usize = 0x5C8; // uint32
    pub const m_hPawn: usize = 0x5CC; // CHandle< C_BasePlayerPawn >
    pub const m_hPredictedPawn: usize = 0x5D0; // CHandle< C_BasePlayerPawn >
    pub const m_nSplitScreenSlot: usize = 0x5D4; // CSplitScreenSlot
    pub const m_hSplitOwner: usize = 0x5D8; // CHandle< CBasePlayerController >
    pub const m_hSplitScreenPlayers: usize = 0x5E0; // CUtlVector< CHandle< CBasePlayerController > >
    pub const m_bIsHLTV: usize = 0x5F8; // bool
    pub const m_iConnected: usize = 0x5FC; // PlayerConnectedState
    pub const m_iszPlayerName: usize = 0x600; // char[128]
    pub const m_steamID: usize = 0x688; // uint64
    pub const m_bIsLocalPlayerController: usize = 0x690; // bool
    pub const m_iDesiredFOV: usize = 0x694; // uint32
}

pub mod CBasePlayerVData {
    pub const m_sModelName: usize = 0x28; // CResourceNameTyped< CWeakHandle< InfoForResourceTypeCModel > >
    pub const m_flHeadDamageMultiplier: usize = 0x108; // CSkillFloat
    pub const m_flChestDamageMultiplier: usize = 0x118; // CSkillFloat
    pub const m_flStomachDamageMultiplier: usize = 0x128; // CSkillFloat
    pub const m_flArmDamageMultiplier: usize = 0x138; // CSkillFloat
    pub const m_flLegDamageMultiplier: usize = 0x148; // CSkillFloat
    pub const m_flHoldBreathTime: usize = 0x158; // float32
    pub const m_flDrowningDamageInterval: usize = 0x15C; // float32
    pub const m_nDrowningDamageInitial: usize = 0x160; // int32
    pub const m_nDrowningDamageMax: usize = 0x164; // int32
    pub const m_nWaterSpeed: usize = 0x168; // int32
    pub const m_flUseRange: usize = 0x16C; // float32
    pub const m_flUseAngleTolerance: usize = 0x170; // float32
    pub const m_flCrouchTime: usize = 0x174; // float32
}

pub mod CBasePlayerWeaponVData {
    pub const m_szClassName: usize = 0x10; // CUtlString
    pub const m_szWorldModel: usize = 0x18; // CResourceNameTyped< CWeakHandle< InfoForResourceTypeCModel > >
    pub const m_bBuiltRightHanded: usize = 0xF8; // bool
    pub const m_bAllowFlipping: usize = 0xF9; // bool
    pub const m_bIsFullAuto: usize = 0xFA; // bool
    pub const m_nNumBullets: usize = 0xFC; // int32
    pub const m_sMuzzleAttachment: usize = 0x100; // CUtlString
    pub const m_szMuzzleFlashParticle: usize = 0x108; // CResourceNameTyped< CWeakHandle< InfoForResourceTypeIParticleSystemDefinition > >
    pub const m_iFlags: usize = 0x1E8; // ItemFlagTypes_t
    pub const m_nPrimaryAmmoType: usize = 0x1E9; // AmmoIndex_t
    pub const m_nSecondaryAmmoType: usize = 0x1EA; // AmmoIndex_t
    pub const m_iMaxClip1: usize = 0x1EC; // int32
    pub const m_iMaxClip2: usize = 0x1F0; // int32
    pub const m_iDefaultClip1: usize = 0x1F4; // int32
    pub const m_iDefaultClip2: usize = 0x1F8; // int32
    pub const m_iWeight: usize = 0x1FC; // int32
    pub const m_bAutoSwitchTo: usize = 0x200; // bool
    pub const m_bAutoSwitchFrom: usize = 0x201; // bool
    pub const m_iRumbleEffect: usize = 0x204; // RumbleEffect_t
    pub const m_aShootSounds: usize = 0x208; // CUtlMap< WeaponSound_t, CSoundEventName >
    pub const m_iSlot: usize = 0x228; // int32
    pub const m_iPosition: usize = 0x22C; // int32
}

pub mod CBasePortraitData {
    pub const m_iModelIndex: usize = 0x410; // CStrongHandle< InfoForResourceTypeCModel >
    pub const m_skin: usize = 0x418; // CUtlStringToken
    pub const m_bHasSetupView: usize = 0xDBC; // bool
    pub const m_flRotation: usize = 0xDD8; // float32
}

pub mod CBaseProp {
    pub const m_bModelOverrodeBlockLOS: usize = 0x7E0; // bool
    pub const m_iShapeType: usize = 0x7E4; // int32
    pub const m_bConformToCollisionBounds: usize = 0x7E8; // bool
    pub const m_mPreferredCatchTransform: usize = 0x7EC; // matrix3x4_t
}

pub mod CBodyComponent {
    pub const m_pSceneNode: usize = 0x8; // CGameSceneNode*
    pub const __m_pChainEntity: usize = 0x20; // CNetworkVarChainer
}

pub mod CBodyComponentBaseAnimGraph {
    pub const m_animationController: usize = 0x470; // CBaseAnimGraphController
    pub const __m_pChainEntity: usize = 0x5D0; // CNetworkVarChainer
}

pub mod CBodyComponentBaseModelEntity {
    pub const __m_pChainEntity: usize = 0x470; // CNetworkVarChainer
}

pub mod CBodyComponentPoint {
    pub const m_sceneNode: usize = 0x50; // CGameSceneNode
    pub const __m_pChainEntity: usize = 0x1A0; // CNetworkVarChainer
}

pub mod CBodyComponentSkeletonInstance {
    pub const m_skeletonInstance: usize = 0x50; // CSkeletonInstance
    pub const __m_pChainEntity: usize = 0x440; // CNetworkVarChainer
}

pub mod CBuoyancyHelper {
    pub const m_flFluidDensity: usize = 0x18; // float32
}

pub mod CClientAlphaProperty {
    pub const m_nRenderFX: usize = 0x10; // uint8
    pub const m_nRenderMode: usize = 0x11; // uint8
    pub const m_bAlphaOverride: usize = 0x0; // bitfield:1
    pub const m_bShadowAlphaOverride: usize = 0x0; // bitfield:1
    pub const m_nReserved: usize = 0x0; // bitfield:6
    pub const m_nAlpha: usize = 0x13; // uint8
    pub const m_nDesyncOffset: usize = 0x14; // uint16
    pub const m_nReserved2: usize = 0x16; // uint16
    pub const m_nDistFadeStart: usize = 0x18; // uint16
    pub const m_nDistFadeEnd: usize = 0x1A; // uint16
    pub const m_flFadeScale: usize = 0x1C; // float32
    pub const m_flRenderFxStartTime: usize = 0x20; // GameTime_t
    pub const m_flRenderFxDuration: usize = 0x24; // float32
}

pub mod CCollisionProperty {
    pub const m_collisionAttribute: usize = 0x10; // VPhysicsCollisionAttribute_t
    pub const m_vecMins: usize = 0x40; // Vector
    pub const m_vecMaxs: usize = 0x4C; // Vector
    pub const m_usSolidFlags: usize = 0x5A; // uint8
    pub const m_nSolidType: usize = 0x5B; // SolidType_t
    pub const m_triggerBloat: usize = 0x5C; // uint8
    pub const m_nSurroundType: usize = 0x5D; // SurroundingBoundsType_t
    pub const m_CollisionGroup: usize = 0x5E; // uint8
    pub const m_nEnablePhysics: usize = 0x5F; // uint8
    pub const m_flBoundingRadius: usize = 0x60; // float32
    pub const m_vecSpecifiedSurroundingMins: usize = 0x64; // Vector
    pub const m_vecSpecifiedSurroundingMaxs: usize = 0x70; // Vector
    pub const m_vecSurroundingMaxs: usize = 0x7C; // Vector
    pub const m_vecSurroundingMins: usize = 0x88; // Vector
    pub const m_vCapsuleCenter1: usize = 0x94; // Vector
    pub const m_vCapsuleCenter2: usize = 0xA0; // Vector
    pub const m_flCapsuleRadius: usize = 0xAC; // float32
}

pub mod CComicBook {
    pub const m_CoverImage: usize = 0x8; // CPanoramaImageName
    pub const m_XmlFile: usize = 0x18; // CUtlString
}

pub mod CDOTABattleReportHighlight {
    pub const m_nID: usize = 0x8; // uint16
    pub const m_bEnabled: usize = 0xA; // bool
    pub const m_eHighlightType: usize = 0xC; // CMsgBattleReport_HighlightType
    pub const m_eHighlightCategory: usize = 0x10; // CMsgBattleReport_HighlightCategory
    pub const m_eHighlightRarity: usize = 0x14; // CMsgBattleReport_HighlightRarity
    pub const m_sNameToken: usize = 0x18; // CUtlString
    pub const m_sFlavorToken: usize = 0x20; // CUtlString
    pub const m_bTooltip: usize = 0x28; // bool
    pub const m_sTooltipLocString: usize = 0x30; // CUtlString
    pub const m_eFormat: usize = 0x38; // EHighlightNumberFormat
    pub const m_vecRoles: usize = 0x40; // CUtlVector< CMsgBattleReport_Role >
    pub const m_vecTiers: usize = 0x58; // CUtlVector< CDOTABattleReportHighlightTier_t >
}

pub mod CDOTABattleReportHighlightCompareContext_t {
    pub const m_eCompareContext: usize = 0x0; // CMsgBattleReport_CompareContext
    pub const m_eComparisonType: usize = 0x4; // EHighlightScoreComparison
    pub const m_flCompareValue: usize = 0x8; // float32
}

pub mod CDOTABattleReportHighlightTier_t {
    pub const m_eTier: usize = 0x0; // CMsgBattleReport_HighlightTier
    pub const m_vecCompareContexts: usize = 0x8; // CUtlVector< CDOTABattleReportHighlightCompareContext_t >
}

pub mod CDOTACandyShopDefinition {
    pub const m_unCandyShopID: usize = 0x8; // CandyShopID_t
    pub const m_sLocName: usize = 0x10; // CUtlString
    pub const m_unDefaultInventorySize: usize = 0x18; // uint32
    pub const m_sInventorySizeUpgradeImage: usize = 0x20; // CPanoramaImageName
    pub const m_unDefaultRerollCharges: usize = 0x30; // uint32
    pub const m_unCandyBagItemDef: usize = 0x34; // item_definition_index_t
    pub const m_unFixedExchangeRecipeMaxCandies: usize = 0x38; // uint8
    pub const m_unFixedExchangeRecipeStartDate: usize = 0x3C; // uint32
    pub const m_unFixedExchangeRecipeUpdateRateInSeconds: usize = 0x40; // uint32
    pub const m_unFixedExchangeRecipeCount: usize = 0x44; // uint8
    pub const m_unVariableExchangeInputCandyCount: usize = 0x45; // uint8
    pub const m_unVariableExchangeOutputCandyCount: usize = 0x46; // uint8
    pub const m_eExpireEvent: usize = 0x48; // EEvent
    pub const m_vecCandyTypes: usize = 0x50; // CUtlVector< CandyShopCandyType_t >
    pub const m_vecRewardSlots: usize = 0x68; // CUtlVector< CandyShopRewardSlot_t >
    pub const m_vecDefaultRewardOptions: usize = 0x80; // CUtlVector< CandyShopRewardOption_t >
}

pub mod CDOTACustomShopInfo {
    pub const m_CustomShopName: usize = 0x30; // char[256]
    pub const m_CustomShopItems: usize = 0x130; // C_UtlVectorEmbeddedNetworkVar< CDOTACustomShopItemInfo >
}

pub mod CDOTACustomShopItemInfo {
    pub const m_nAbilityID: usize = 0x30; // AbilityID_t
    pub const m_nStockCount: usize = 0x34; // int32
    pub const m_Category: usize = 0x38; // char[256]
}

pub mod CDOTAInGamePredictionState {
    pub const m_bVotingClosed: usize = 0x538; // bool
    pub const m_bAllPredictionsFinished: usize = 0x539; // bool
    pub const m_vecPredictions: usize = 0x540; // C_UtlVectorEmbeddedNetworkVar< InGamePredictionData_t >
    pub const m_nLeagueID: usize = 0x590; // LeagueID_t
    pub const m_vecPrevPredictions: usize = 0x598; // CUtlVector< InGamePredictionData_t >
}

pub mod CDOTAPropAghsfortArenaOfBloodWarrior {
    pub const m_bDying: usize = 0xA58; // bool
}

pub mod CDOTAPropArenaOfBloodWarrior {
    pub const m_bDying: usize = 0xA58; // bool
}

pub mod CDOTAPropConsumableBanner {
    pub const m_nPlayerID: usize = 0xAB0; // PlayerID_t
    pub const m_hAvatarTexture: usize = 0xAB8; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_bUseAvatar: usize = 0xAC0; // bool
}

pub mod CDOTAPropPlusPlayerGuildBanner {
    pub const m_nPlayerID: usize = 0xAB0; // int32
}

pub mod CDOTAPropTI10PlayerGuildBanner {
    pub const m_nPlayerID: usize = 0xAB0; // int32
}

pub mod CDOTARoadToTIChallengeDefinition {
    pub const m_eEvent: usize = 0x0; // EEvent
    pub const m_unTotalQuestPeriods: usize = 0x4; // uint32
    pub const m_unHeroesPerQuest: usize = 0x8; // uint32
    pub const m_vecQuestPattern: usize = 0x10; // CUtlVector< uint32 >
    pub const m_unCullingBladeItemDef: usize = 0x28; // item_definition_index_t
    pub const m_unRerollItemDef: usize = 0x2C; // item_definition_index_t
    pub const m_vecQuests: usize = 0x30; // CUtlVector< RoadToTIQuestDefinition_t >
}

pub mod CDOTASubChallengeInfo {
    pub const nType: usize = 0x30; // int32
    pub const nTier: usize = 0x34; // int32
    pub const nSlotID: usize = 0x38; // int32
    pub const nProgress: usize = 0x3C; // int32
    pub const nCompletionThreshold: usize = 0x40; // int32
    pub const nPlayerID: usize = 0x44; // PlayerID_t
    pub const nQueryIndex: usize = 0x48; // int32
    pub const nEventID: usize = 0x4C; // int32
    pub const nSequenceID: usize = 0x50; // int32
    pub const nRequiredHero: usize = 0x54; // int32
    pub const nCompleted: usize = 0x58; // int32
}

pub mod CDOTA_AbilityDraftAbilityState {
    pub const m_nAbilityID: usize = 0x30; // AbilityID_t
    pub const m_unPlayerID: usize = 0x34; // PlayerID_t
    pub const m_unAbilityPlayerSlot: usize = 0x38; // int32
}

pub mod CDOTA_Ability_AbyssalUnderlord_Firestorm {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
}

pub mod CDOTA_Ability_AbyssalUnderlord_PitOfMalice {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
}

pub mod CDOTA_Ability_AghsFort_Arcanist_Potion {
    pub const cooldown_reduction_pct: usize = 0x648; // int32
    pub const manacost_reduction_pct: usize = 0x64C; // int32
}

pub mod CDOTA_Ability_AghsFort_Ascension_Invis {
    pub const warning_duration: usize = 0x5B0; // float32
}

pub mod CDOTA_Ability_AghsFort_Capture {
    pub const m_hTarget: usize = 0x5B0; // CHandle< C_DOTA_BaseNPC >
}

pub mod CDOTA_Ability_AghsFort_Clinkz_BurningArmy {
    pub const range: usize = 0x5B0; // int32
}

pub mod CDOTA_Ability_AghsFort_Clinkz_BurningBarrage {
    pub const m_vStartPos: usize = 0x5B0; // Vector
    pub const m_iArrowProjectile: usize = 0x5BC; // int32
    pub const m_nFXIndex: usize = 0x5C0; // int32
    pub const m_nTriSplitCount: usize = 0x5E0; // int32
    pub const projectile_speed: usize = 0x5E4; // int32
    pub const projectile_width: usize = 0x5E8; // int32
}

pub mod CDOTA_Ability_AghsFort_Creature_Impale {
    pub const m_nPreviewFX: usize = 0x5B0; // ParticleIndex_t
    pub const width: usize = 0x5B4; // int32
    pub const duration: usize = 0x5B8; // float32
    pub const length: usize = 0x5BC; // int32
    pub const speed: usize = 0x5C0; // int32
}

pub mod CDOTA_Ability_AghsFort_Creature_Phoenix_LaunchFireSpirit {
    pub const spirit_speed: usize = 0x5B0; // int32
    pub const duration: usize = 0x5B4; // float32
    pub const radius: usize = 0x5B8; // int32
    pub const hp_cost_perc: usize = 0x5BC; // int32
    pub const m_nFXIndex: usize = 0x5C0; // ParticleIndex_t
}

pub mod CDOTA_Ability_AghsFort_Creature_Phoenix_Supernova {
    pub const m_nPreviewFX: usize = 0x5B0; // ParticleIndex_t
}

pub mod CDOTA_Ability_AghsFort_DragonKnight_BreatheFire {
    pub const start_radius: usize = 0x5B0; // int32
    pub const end_radius: usize = 0x5B4; // int32
    pub const m_vStartPos: usize = 0x5B8; // Vector
    pub const m_fStartTime: usize = 0x5C4; // GameTime_t
    pub const m_fTotalTime: usize = 0x5C8; // float32
}

pub mod CDOTA_Ability_AghsFort_Dragon_Potion {
    pub const bonus_movement_speed: usize = 0x648; // int32
    pub const bonus_attack_damage: usize = 0x64C; // int32
    pub const bonus_attack_range: usize = 0x650; // int32
    pub const magic_resistance: usize = 0x654; // int32
    pub const model_scale: usize = 0x658; // int32
    pub const skin_number: usize = 0x65C; // int32
    pub const flying_movement: usize = 0x660; // bool
}

pub mod CDOTA_Ability_AghsFort_EarthSpiritBoss_BoulderSmash {
    pub const speed: usize = 0x5B0; // int32
    pub const rock_damage: usize = 0x5B4; // int32
    pub const radius: usize = 0x5B8; // int32
    pub const rock_search_aoe: usize = 0x5BC; // int32
    pub const unit_distance: usize = 0x5C0; // float32
    pub const rock_distance: usize = 0x5C4; // float32
    pub const m_nProjectileID: usize = 0x5C8; // int32
    pub const m_hCursorTarget: usize = 0x5CC; // CHandle< C_BaseEntity >
    pub const m_bUsedStone: usize = 0x5D0; // bool
    pub const m_hTarget: usize = 0x5D4; // CHandle< C_BaseEntity >
    pub const m_bTargetStone: usize = 0x5D8; // bool
    pub const m_vecBoulderSmashedEnts: usize = 0x5E0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Ability_AghsFort_EarthSpiritBoss_GeomagneticGrip {
    pub const m_hTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Ability_AghsFort_EarthSpiritBoss_RollingBoulder {
    pub const radius: usize = 0x5B0; // int32
    pub const speed: usize = 0x5B4; // int32
    pub const rock_speed: usize = 0x5B8; // int32
    pub const damage: usize = 0x5BC; // int32
    pub const distance: usize = 0x5C0; // float32
    pub const rock_distance: usize = 0x5C4; // float32
    pub const slow_duration: usize = 0x5C8; // float32
    pub const m_nFXIndex: usize = 0x5CC; // ParticleIndex_t
    pub const m_boulderSetposBool: usize = 0x5D0; // bool
    pub const m_nProjectileID: usize = 0x5D4; // int32
    pub const m_vStartingLocation: usize = 0x5D8; // Vector
    pub const m_vProjectileLocation: usize = 0x5E4; // Vector
    pub const m_vDir: usize = 0x5F0; // Vector
    pub const m_vVel: usize = 0x5FC; // Vector
    pub const m_bUsedStone: usize = 0x608; // bool
}

pub mod CDOTA_Ability_AghsFort_EchoSlamPotion {
    pub const echo_slam_damage_range: usize = 0x648; // int32
    pub const echo_slam_echo_search_range: usize = 0x64C; // int32
    pub const echo_slam_echo_range: usize = 0x650; // int32
    pub const echo_slam_echo_damage: usize = 0x654; // int32
    pub const echo_slam_initial_damage: usize = 0x658; // int32
    pub const spend_charge_delay: usize = 0x65C; // float32
}

pub mod CDOTA_Ability_AghsFort_Luna_GlaiveShield {
    pub const m_GlaiveDefs: usize = 0x5B0; // CUtlVector< sGlaiveDef >
}

pub mod CDOTA_Ability_AghsFort_Phoenix_IcarusDive {
    pub const hp_cost_perc: usize = 0x5B0; // int32
}

pub mod CDOTA_Ability_AghsFort_Phoenix_LaunchFireSpirit {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
    pub const m_bUnitCarriedByArtillery: usize = 0x5B4; // bool
    pub const m_nSpiritSpeed: usize = 0x5B8; // int32
    pub const m_fDuration: usize = 0x5BC; // float32
    pub const m_nRadius: usize = 0x5C0; // int32
    pub const m_nSpiritCount: usize = 0x5C4; // int32
}

pub mod CDOTA_Ability_AghsFort_Ravage_Potion {
    pub const m_hEntsHit: usize = 0x648; // CUtlVector< CHandle< C_BaseEntity > >
    pub const damage: usize = 0x660; // int32
    pub const duration: usize = 0x664; // float32
    pub const spend_charge_delay: usize = 0x668; // float32
}

pub mod CDOTA_Ability_AghsFort_Sniper_Assassinate {
    pub const m_hTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const m_iIndex: usize = 0x5B4; // ParticleIndex_t
    pub const m_bHasBuckshotAbility: usize = 0x5B8; // bool
    pub const m_bInBuckshot: usize = 0x5B9; // bool
    pub const m_nScatterRange: usize = 0x5BC; // int32
    pub const m_nScatterWidth: usize = 0x5C0; // int32
    pub const m_nBuckshotDamagePct: usize = 0x5C4; // int32
    pub const m_nDamageType: usize = 0x5C8; // int32
    pub const damage: usize = 0x5CC; // int32
}

pub mod CDOTA_Ability_AghsFort_Special_VoidSpirit_ResonantPulse_Suppression {
    pub const m_nBonusCharges: usize = 0x5B0; // int32
}

pub mod CDOTA_Ability_AghsFort_Spectre_ActiveDispersion {
    pub const m_nPreviewFX: usize = 0x5B0; // ParticleIndex_t
    pub const duration: usize = 0x5B4; // float32
}

pub mod CDOTA_Ability_AghsFort_Tower_BlastWave {
    pub const m_nPreviewFX: usize = 0x5B0; // ParticleIndex_t
    pub const duration: usize = 0x5B4; // float32
}

pub mod CDOTA_Ability_AghsFort_VoidSpiritBoss_AetherRemnant {
    pub const m_nFXPreview: usize = 0x5B0; // ParticleIndex_t
    pub const start_radius: usize = 0x5B4; // int32
    pub const end_radius: usize = 0x5B8; // int32
    pub const remnant_watch_distance: usize = 0x5BC; // int32
    pub const projectile_speed: usize = 0x5C0; // int32
}

pub mod CDOTA_Ability_AghsFort_VoidSpiritBoss_AstralStep {
    pub const m_nFXPreview: usize = 0x5B0; // ParticleIndex_t
    pub const m_nFXDestinationPreview: usize = 0x5B4; // ParticleIndex_t
    pub const m_vStartPos: usize = 0x5B8; // Vector
    pub const m_vDestination: usize = 0x5C4; // Vector
    pub const m_vDirection: usize = 0x5D0; // Vector
    pub const radius: usize = 0x5DC; // int32
    pub const min_travel_distance: usize = 0x5E0; // int32
    pub const max_travel_distance: usize = 0x5E4; // int32
    pub const debuff_duration: usize = 0x5E8; // float32
    pub const pop_damage_delay: usize = 0x5EC; // float32
}

pub mod CDOTA_Ability_AghsFort_VoidSpiritBoss_Dissimilate {
    pub const m_nFXPreview: usize = 0x5B0; // ParticleIndex_t
    pub const phase_duration: usize = 0x5B4; // float32
}

pub mod CDOTA_Ability_AghsFort_VoidSpiritBoss_ResonantPulse {
    pub const m_nFXPreview: usize = 0x5B0; // ParticleIndex_t
    pub const m_bGrantedScepterCharges: usize = 0x5B4; // bool
    pub const buff_duration: usize = 0x5B8; // float32
    pub const base_absorb_amount: usize = 0x5BC; // int32
    pub const absorb_per_hero_hit: usize = 0x5C0; // int32
}

pub mod CDOTA_Ability_AghsFort_VoidSpirit_AetherRemnant {
    pub const start_radius: usize = 0x5B0; // int32
    pub const end_radius: usize = 0x5B4; // int32
    pub const remnant_watch_distance: usize = 0x5B8; // int32
    pub const projectile_speed: usize = 0x5BC; // int32
}

pub mod CDOTA_Ability_AghsFort_VoidSpirit_AstralStep {
    pub const m_vStartPos: usize = 0x5B0; // Vector
    pub const m_vDestination: usize = 0x5BC; // Vector
    pub const m_vDirection: usize = 0x5C8; // Vector
    pub const radius: usize = 0x5D4; // int32
    pub const min_travel_distance: usize = 0x5D8; // int32
    pub const max_travel_distance: usize = 0x5DC; // int32
    pub const debuff_duration: usize = 0x5E0; // float32
    pub const pop_damage_delay: usize = 0x5E4; // float32
    pub const pop_damage: usize = 0x5E8; // int32
}

pub mod CDOTA_Ability_AghsFort_VoidSpirit_Dissimilate {
    pub const m_vecTranslocatingAllies: usize = 0x5B0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const phase_duration: usize = 0x5C8; // float32
}

pub mod CDOTA_Ability_AghsFort_VoidSpirit_ResonantPulse {
    pub const buff_duration: usize = 0x5B0; // float32
    pub const base_absorb_amount: usize = 0x5B4; // int32
    pub const absorb_per_unit_hit: usize = 0x5B8; // int32
    pub const hero_absorb_multiplier: usize = 0x5BC; // float32
    pub const boss_absorb_multiplier: usize = 0x5C0; // float32
}

pub mod CDOTA_Ability_Aghsfort_Aziyog_Underlord_Firestorm {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
}

pub mod CDOTA_Ability_Aghsfort_Bonus_Pudge_MeatHook {
    pub const m_nConsecutiveHits: usize = 0x5B0; // int32
}

pub mod CDOTA_Ability_Aghsfort_Dawnbreaker_Celestial_Hammer {
    pub const projectile_speed: usize = 0x5B0; // int32
    pub const projectile_radius: usize = 0x5B4; // int32
    pub const hammer_damage: usize = 0x5B8; // int32
    pub const hammer_aoe_radius: usize = 0x5BC; // int32
    pub const flare_radius: usize = 0x5C0; // int32
    pub const bHasStartedBurning: usize = 0x5C4; // bool
    pub const flare_debuff_duration: usize = 0x5C8; // float32
    pub const return_anim_distance_threshold: usize = 0x5CC; // int32
    pub const range: usize = 0x5D0; // int32
}

pub mod CDOTA_Ability_Aghsfort_Gyrocopter_Call_Down {
    pub const m_vEndpoint: usize = 0x5B0; // Vector
    pub const range_scepter: usize = 0x5BC; // int32
}

pub mod CDOTA_Ability_Aghsfort_Lina_FierySoul {
    pub const active_duration: usize = 0x5B0; // float32
}

pub mod CDOTA_Ability_Aghsfort_Mars_Bulwark {
    pub const active_duration: usize = 0x5B0; // float32
}

pub mod CDOTA_Ability_Aghsfort_Phoenix_Flame_Revenant_Attack {
    pub const m_hTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Ability_Aghsfort_PrimalBeast_Boss_PrimalRoar {
    pub const path_width: usize = 0x5B0; // int32
    pub const push_duration: usize = 0x5B4; // float32
    pub const push_distance: usize = 0x5B8; // int32
    pub const min_distance: usize = 0x5BC; // int32
}

pub mod CDOTA_Ability_Aghsfort_PrimalBeast_Boss_ThrowAttack {
    pub const base_damage: usize = 0x5B0; // int32
    pub const hp_damage_pct: usize = 0x5B4; // int32
    pub const impact_radius: usize = 0x5B8; // int32
    pub const fragment_impact_distance: usize = 0x5BC; // int32
    pub const min_range: usize = 0x5C0; // int32
    pub const m_nFXPreview: usize = 0x5C4; // ParticleIndex_t
}

pub mod CDOTA_Ability_Aghsfort_Snapfire_FiresnapCookie {
    pub const m_hTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const projectile_speed: usize = 0x5BC; // int32
    pub const pre_hop_duration: usize = 0x5C0; // float32
    pub const cast_on_ally_gesture_time: usize = 0x5C4; // float32
}

pub mod CDOTA_Ability_Aghsfort_Snapfire_GobbleUp {
    pub const max_time_in_belly: usize = 0x5B0; // float32
}

pub mod CDOTA_Ability_Aghsfort_Snapfire_LilShredder {
    pub const m_iAttackIndex: usize = 0x5B0; // int32
    pub const m_BounceInfo: usize = 0x5B8; // CUtlVector< sLilShredderBounceInfo >
}

pub mod CDOTA_Ability_Aghsfort_Snapfire_MortimerKisses {
    pub const m_nDamagePerProjectile: usize = 0x5B0; // int32
    pub const damage_per_impact: usize = 0x5B4; // int32
    pub const impact_radius: usize = 0x5B8; // int32
    pub const projectile_vision: usize = 0x5BC; // int32
    pub const m_nFragmentationImpactRadius: usize = 0x5C0; // int32
    pub const firetrail_radius: usize = 0x5C4; // int32
    pub const firetrail_damage: usize = 0x5C8; // int32
    pub const firetrail_tick_interval: usize = 0x5CC; // int32
    pub const m_vFirePoolLocations: usize = 0x5D0; // CUtlVector< Vector >
    pub const m_fNextDamageTick: usize = 0x5E8; // float32
    pub const m_vLastFirePoolLoc: usize = 0x5EC; // Vector
}

pub mod CDOTA_Ability_Aghsfort_Snapfire_Scatterblast {
    pub const damage: usize = 0x5B0; // int32
    pub const debuff_duration: usize = 0x5B4; // float32
    pub const point_blank_range: usize = 0x5B8; // int32
    pub const point_blank_dmg_bonus_pct: usize = 0x5BC; // float32
    pub const blast_width_end: usize = 0x5C0; // int32
}

pub mod CDOTA_Ability_Aghsfort_Snapfire_SpitCreep {
    pub const m_nDamagePerProjectile: usize = 0x5B0; // int32
    pub const m_hGobbledUnit: usize = 0x5B4; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Ability_Aghsfort_Winter_Wyvern_Arctic_Burn {
    pub const m_BurnedTargets: usize = 0x5B0; // CUtlVector< CEntityIndex >
}

pub mod CDOTA_Ability_Aghsfort_Winter_Wyvern_Splinter_Blast {
    pub const splinter_damage: usize = 0x5B8; // int32
    pub const m_hMainTarget: usize = 0x5BC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Ability_Alchemist_UnstableConcoctionThrow {
    pub const m_fCookTime: usize = 0x5B0; // float32
    pub const m_vProjectileLoc: usize = 0x5B4; // Vector
}

pub mod CDOTA_Ability_AncientApparition_IceAge {
    pub const radius: usize = 0x5B0; // int32
    pub const health_degen: usize = 0x5B4; // int32
    pub const duration: usize = 0x5B8; // float32
    pub const damage: usize = 0x5BC; // int32
}

pub mod CDOTA_Ability_AncientApparition_IceBlast {
    pub const m_PathTimer: usize = 0x5B0; // CountdownTimer
    pub const m_vTarget: usize = 0x5C8; // Vector
    pub const m_vStartPos: usize = 0x5D4; // Vector
    pub const m_vLastTempViewer: usize = 0x5E0; // Vector
    pub const m_iTrackerProjectile: usize = 0x5EC; // int32
    pub const path_radius: usize = 0x5F0; // int32
    pub const radius_min: usize = 0x5F4; // int32
    pub const radius_max: usize = 0x5F8; // int32
    pub const radius_grow: usize = 0x5FC; // float32
    pub const frostbite_duration: usize = 0x600; // float32
    pub const target_sight_radius: usize = 0x604; // int32
    pub const m_hFrostbittenEntities: usize = 0x608; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Ability_AncientApparition_IceVortex {
    pub const vision_aoe: usize = 0x5B0; // int32
    pub const vortex_duration: usize = 0x5B4; // float32
}

pub mod CDOTA_Ability_Beastmaster_PrimalRoar {
    pub const duration: usize = 0x5B0; // float32
    pub const slow_duration: usize = 0x5B4; // float32
    pub const side_damage: usize = 0x5B8; // int32
    pub const damage_radius: usize = 0x5BC; // int32
    pub const path_width: usize = 0x5C0; // int32
    pub const push_duration: usize = 0x5C4; // float32
    pub const push_distance: usize = 0x5C8; // int32
    pub const damage: usize = 0x5CC; // int32
    pub const movement_speed_duration: usize = 0x5D0; // float32
}

pub mod CDOTA_Ability_Brewmaster_Void_Astral_Pull {
    pub const pull_speed: usize = 0x5B0; // int32
    pub const pull_distance: usize = 0x5B4; // int32
}

pub mod CDOTA_Ability_Capture {
    pub const m_hTarget: usize = 0x5B0; // CHandle< C_DOTA_BaseNPC >
    pub const m_pMyBuff: usize = 0x5B8; // CDOTA_Buff*
}

pub mod CDOTA_Ability_Centaur_DoubleEdge {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
}

pub mod CDOTA_Ability_Centaur_Stampede {
    pub const duration: usize = 0x5B0; // float32
    pub const base_damage: usize = 0x5B4; // int32
    pub const strength_damage: usize = 0x5B8; // float32
    pub const slow_duration: usize = 0x5BC; // float32
    pub const scepter_bonus_duration: usize = 0x5C0; // float32
    pub const m_hHitEntities: usize = 0x5C8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Ability_Centaur_Work_Horse {
    pub const m_hCart: usize = 0x5B0; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Ability_CrystalMaiden_IceRink {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
}

pub mod CDOTA_Ability_Dawnbreaker_Celestial_Hammer {
    pub const m_nProjectileIndex: usize = 0x5B0; // int32
    pub const m_nReturnProjectileID: usize = 0x5B4; // int32
    pub const m_vEndLocation: usize = 0x5B8; // Vector
    pub const m_fZCoord: usize = 0x5C4; // float32
    pub const m_vLastTrailThinkerLocation: usize = 0x5C8; // Vector
    pub const m_bFlareDone: usize = 0x5D4; // bool
    pub const m_bStartedCatchAnimation: usize = 0x5D5; // bool
    pub const m_bIsReturning: usize = 0x5D6; // bool
    pub const m_hReturnHits: usize = 0x5D8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_hAoEHits: usize = 0x5F0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_nFXIndex: usize = 0x608; // ParticleIndex_t
    pub const m_nFXIndexC: usize = 0x60C; // ParticleIndex_t
    pub const m_nFXHammerReturnIndex: usize = 0x610; // ParticleIndex_t
    pub const m_nFXHammerProjectileIndex: usize = 0x614; // ParticleIndex_t
    pub const m_nFXHeroSolarGuardianTrailIndex: usize = 0x618; // ParticleIndex_t
    pub const m_hThinker: usize = 0x61C; // CHandle< C_BaseEntity >
    pub const projectile_speed: usize = 0x620; // int32
    pub const projectile_radius: usize = 0x624; // int32
    pub const hammer_damage: usize = 0x628; // int32
    pub const hammer_aoe_radius: usize = 0x62C; // int32
    pub const flare_radius: usize = 0x630; // int32
    pub const bHasStartedBurning: usize = 0x634; // bool
    pub const flare_debuff_duration: usize = 0x638; // float32
    pub const return_anim_distance_threshold: usize = 0x63C; // int32
    pub const range: usize = 0x640; // int32
}

pub mod CDOTA_Ability_Diretide_Portal_Channel {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
    pub const m_hInitialPortal: usize = 0x5B4; // CHandle< C_BaseEntity >
    pub const m_vDestination: usize = 0x5B8; // Vector
    pub const m_hTarget: usize = 0x5C4; // CHandle< C_DOTA_BaseNPC >
}

pub mod CDOTA_Ability_EarthSpirit_Petrify {
    pub const ally_cast_range: usize = 0x5B0; // int32
}

pub mod CDOTA_Ability_Elder_Titan_EarthSplitter {
    pub const crack_width: usize = 0x5B0; // int32
    pub const crack_distance: usize = 0x5B4; // int32
    pub const speed: usize = 0x5B8; // int32
    pub const vision_width: usize = 0x5BC; // int32
    pub const crack_time: usize = 0x5C0; // float32
}

pub mod CDOTA_Ability_EnragedWildkin_Hurricane {
    pub const distance: usize = 0x5B0; // float32
    pub const m_vEndpoint: usize = 0x5B4; // Vector
}

pub mod CDOTA_Ability_FelBeast_Haunt {
    pub const radius: usize = 0x5B0; // int32
    pub const duration: usize = 0x5B4; // float32
}

pub mod CDOTA_Ability_Frostivus2018_Centaur_Stampede {
    pub const duration: usize = 0x5B0; // float32
    pub const base_damage: usize = 0x5B4; // int32
    pub const strength_damage: usize = 0x5B8; // float32
    pub const slow_duration: usize = 0x5BC; // float32
    pub const m_hHitEntities: usize = 0x5C0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const armor_amount: usize = 0x5D8; // int32
    pub const armor_duration: usize = 0x5DC; // float32
    pub const max_armor_stacks: usize = 0x5E0; // int32
}

pub mod CDOTA_Ability_Frostivus2018_Clinkz_Burning_Army {
    pub const range: usize = 0x5B0; // int32
}

pub mod CDOTA_Ability_Frostivus2018_FacelessVoid_TimeWalk {
    pub const speed: usize = 0x5B0; // int32
    pub const range: usize = 0x5B4; // int32
    pub const radius: usize = 0x5B8; // int32
    pub const damage: usize = 0x5BC; // int32
}

pub mod CDOTA_Ability_Frostivus2018_Luna_LucentBeam {
    pub const radius: usize = 0x5B0; // int32
}

pub mod CDOTA_Ability_Frostivus2018_Spectre_ActiveDispersion {
    pub const m_nPreviewFX: usize = 0x5B0; // ParticleIndex_t
    pub const duration: usize = 0x5B4; // float32
}

pub mod CDOTA_Ability_Frostivus2018_Tusk_WalrusKick {
    pub const m_nFXKickIndex: usize = 0x5B0; // ParticleIndex_t
}

pub mod CDOTA_Ability_GiantWolf_Intimidate {
    pub const radius: usize = 0x5B0; // int32
    pub const duration: usize = 0x5B4; // float32
}

pub mod CDOTA_Ability_Grimstroke_DarkArtistry {
    pub const m_vCastDir: usize = 0x5B0; // Vector
    pub const m_fStartTime: usize = 0x5BC; // GameTime_t
    pub const m_fTotalTime: usize = 0x5C0; // float32
    pub const m_nProjectileID: usize = 0x5C4; // int32
    pub const m_vProjectileDir: usize = 0x5C8; // Vector
    pub const m_nFXIndex: usize = 0x5D4; // ParticleIndex_t
    pub const m_nFXIndexB: usize = 0x5D8; // ParticleIndex_t
    pub const m_nTargetsHit: usize = 0x5DC; // int32
    pub const m_nHeroesHit: usize = 0x5E0; // int32
    pub const m_nVisibleHeroesHit: usize = 0x5E4; // int32
    pub const m_fDmgMultiplierTalent: usize = 0x5E8; // float32
    pub const slow_duration: usize = 0x5EC; // float32
    pub const start_radius: usize = 0x5F0; // int32
    pub const end_radius: usize = 0x5F4; // int32
    pub const damage: usize = 0x5F8; // int32
    pub const bonus_damage_per_target: usize = 0x5FC; // int32
    pub const vision_duration: usize = 0x600; // float32
}

pub mod CDOTA_Ability_Grimstroke_InkCreature {
    pub const spawn_time: usize = 0x5B0; // float32
}

pub mod CDOTA_Ability_Grimstroke_SoulChain {
    pub const chain_duration: usize = 0x5B0; // float32
    pub const chain_latch_radius: usize = 0x5B4; // int32
    pub const creep_duration_pct: usize = 0x5B8; // float32
}

pub mod CDOTA_Ability_Grimstroke_SpiritWalk {
    pub const buff_duration: usize = 0x5B0; // float32
    pub const m_hTarget: usize = 0x5B4; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Ability_Gyrocopter_Call_Down {
    pub const m_vEndpoint: usize = 0x5B0; // Vector
    pub const range_scepter: usize = 0x5BC; // int32
}

pub mod CDOTA_Ability_Hoodwink_Decoy {
    pub const hIllusion: usize = 0x5B0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const decoy_stun_duration: usize = 0x5C8; // float32
    pub const decoy_detonate_radius: usize = 0x5CC; // int32
    pub const projectile_speed: usize = 0x5D0; // int32
    pub const m_nFXIndex: usize = 0x5D4; // ParticleIndex_t
}

pub mod CDOTA_Ability_Invoker_InvokedBase {
    pub const m_nQuasLevel: usize = 0x5B0; // int32
    pub const m_nWexLevel: usize = 0x5B4; // int32
    pub const m_nExortLevel: usize = 0x5B8; // int32
}

pub mod CDOTA_Ability_Lamp_Use {
    pub const m_flLastCaptureTime: usize = 0x5B0; // GameTime_t
    pub const m_nChannelFXIndex: usize = 0x5B4; // ParticleIndex_t
}

pub mod CDOTA_Ability_Life_Stealer_Assimilate {
    pub const m_hLastAssimilation: usize = 0x5B0; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Ability_Marci_Companion_Run {
    pub const m_nTrackingProjectileID: usize = 0x5B0; // int32
    pub const m_nFXIndex: usize = 0x5B4; // ParticleIndex_t
    pub const m_nMaxJumpDistance: usize = 0x5B8; // int32
    pub const landing_radius: usize = 0x5BC; // int32
    pub const move_speed: usize = 0x5C0; // int32
    pub const ally_buff_duration: usize = 0x5C4; // float32
    pub const min_jump_distance: usize = 0x5C8; // int32
    pub const max_jump_distance: usize = 0x5CC; // int32
    pub const impact_position_offset: usize = 0x5D0; // int32
    pub const vector_preview_radius: usize = 0x5D4; // int32
    pub const m_vEndpoint: usize = 0x5D8; // Vector
    pub const m_nToBounceFXIndex: usize = 0x5E4; // ParticleIndex_t
    pub const m_nToTargetFXIndex: usize = 0x5E8; // ParticleIndex_t
}

pub mod CDOTA_Ability_Marci_Guardian {
    pub const buff_duration: usize = 0x5B0; // float32
    pub const nearest_ally_search_range: usize = 0x5B4; // int32
}

pub mod CDOTA_Ability_Marci_Unleash {
    pub const duration: usize = 0x5B0; // float32
    pub const pulse_radius: usize = 0x5B4; // int32
    pub const pulse_debuff_duration: usize = 0x5B8; // float32
    pub const pulse_damage: usize = 0x5BC; // int32
    pub const pulse_silence_duration: usize = 0x5C0; // float32
}

pub mod CDOTA_Ability_Mars_Bulwark {
    pub const m_flNextAttack: usize = 0x5B0; // GameTime_t
    pub const m_flLastActive: usize = 0x5B4; // GameTime_t
}

pub mod CDOTA_Ability_Meepo_Poof {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
    pub const radius: usize = 0x5BC; // int32
}

pub mod CDOTA_Ability_Nevermore_Necromastery {
    pub const attack_range_bonus: usize = 0x5B0; // int32
}

pub mod CDOTA_Ability_Nyx_Assassin_Burrow {
    pub const m_nSpellStartFXIndex: usize = 0x5B0; // ParticleIndex_t
    pub const m_nPhaseStartFXIndex: usize = 0x5B4; // ParticleIndex_t
}

pub mod CDOTA_Ability_Nyx_Assassin_Jolt {
    pub const aoe: usize = 0x5B0; // int32
    pub const damage: usize = 0x5B4; // int32
}

pub mod CDOTA_Ability_Nyx_Assassin_ManaBurn {
    pub const aoe: usize = 0x5B0; // int32
}

pub mod CDOTA_Ability_PluckFamango {
    pub const m_hTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const m_pMyBuff: usize = 0x5B8; // CDOTA_Buff*
    pub const famango_rate: usize = 0x5C0; // float32
}

pub mod CDOTA_Ability_PrimalBeast_Rock_Throw {
    pub const base_damage: usize = 0x5B0; // int32
    pub const impact_radius: usize = 0x5B4; // int32
    pub const fragment_impact_radius: usize = 0x5B8; // int32
    pub const fragment_impact_distance: usize = 0x5BC; // int32
    pub const min_range: usize = 0x5C0; // int32
    pub const stun_duration: usize = 0x5C4; // float32
    pub const m_nFXPreview: usize = 0x5C8; // ParticleIndex_t
}

pub mod CDOTA_Ability_Puck_WaningRift {
    pub const max_distance: usize = 0x5B0; // int32
}

pub mod CDOTA_Ability_Pudge_Dismember {
    pub const m_hVictim: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const shard_cast_range: usize = 0x5B4; // int32
}

pub mod CDOTA_Ability_Pudge_MeatHook {
    pub const m_vEndpoint: usize = 0x5B0; // Vector
    pub const m_nConsecutiveHits: usize = 0x5BC; // int32
    pub const m_bIsVectorTargeted: usize = 0x5C0; // bool
    pub const m_vTurnLocation: usize = 0x5C4; // Vector
    pub const m_vDirectionAfterTurn: usize = 0x5D0; // Vector
    pub const m_bHasTurned: usize = 0x5DC; // bool
    pub const m_flDistanceAfterTurn: usize = 0x5E0; // float32
    pub const m_vFinalPosition: usize = 0x5E4; // Vector
}

pub mod CDOTA_Ability_Seasonal_Summon_Dragon_Thinker {
    pub const grace_period: usize = 0x12C8; // float32
    pub const search_distance: usize = 0x12CC; // int32
    pub const think_interval: usize = 0x12D0; // float32
}

pub mod CDOTA_Ability_Seasonal_Summon_Penguin {
    pub const spawn_offset: usize = 0x5B0; // int32
}

pub mod CDOTA_Ability_Seasonal_TI10_HotPotato {
    pub const m_hTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const projectile_speed: usize = 0x5BC; // int32
}

pub mod CDOTA_Ability_Seasonal_TI10_SoccerBall {
    pub const m_vEndpoint: usize = 0x5B0; // Vector
}

pub mod CDOTA_Ability_Seasonal_TI11_Balloon {
    pub const charge_use_interval: usize = 0x5B0; // float32
    pub const max_usable_charges: usize = 0x5B4; // float32
    pub const m_flLastChargeUseTime: usize = 0x5B8; // GameTime_t
    pub const m_hBalloon: usize = 0x5BC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Ability_Seasonal_TI11_CongaLine {
    pub const max_duration: usize = 0x5B0; // float32
}

pub mod CDOTA_Ability_Seasonal_TI11_Duel {
    pub const model_scale: usize = 0x5B0; // float32
}

pub mod CDOTA_Ability_Snapfire_FiresnapCookie {
    pub const m_hTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const projectile_speed: usize = 0x5BC; // int32
    pub const pre_hop_duration: usize = 0x5C0; // float32
    pub const cast_on_ally_gesture_time: usize = 0x5C4; // float32
}

pub mod CDOTA_Ability_Snapfire_GobbleUp {
    pub const max_time_in_belly: usize = 0x5B0; // float32
}

pub mod CDOTA_Ability_Snapfire_MortimerKisses {
    pub const m_nDamagePerProjectile: usize = 0x5B0; // int32
    pub const damage_per_impact: usize = 0x5B4; // int32
    pub const impact_radius: usize = 0x5B8; // int32
    pub const projectile_vision: usize = 0x5BC; // int32
}

pub mod CDOTA_Ability_Snapfire_Scatterblast {
    pub const damage: usize = 0x5B0; // int32
    pub const debuff_duration: usize = 0x5B4; // float32
    pub const point_blank_range: usize = 0x5B8; // int32
    pub const point_blank_dmg_bonus_pct: usize = 0x5BC; // float32
    pub const blast_width_end: usize = 0x5C0; // int32
}

pub mod CDOTA_Ability_Snapfire_SpitCreep {
    pub const m_nDamagePerProjectile: usize = 0x5B0; // int32
    pub const m_hGobbledUnit: usize = 0x5B4; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Ability_Spring2021_DefusalBomb_Defuse {
    pub const m_hTarget: usize = 0x5B0; // CHandle< C_DOTA_BaseNPC >
}

pub mod CDOTA_Ability_TI10_DiscoBall_Channel {
    pub const m_hTarget: usize = 0x5B0; // CHandle< C_DOTA_BaseNPC >
    pub const m_flChannelTime: usize = 0x5B4; // float32
}

pub mod CDOTA_Ability_Techies_Minefield_Sign {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
    pub const m_nFXHammerIndex: usize = 0x5B4; // ParticleIndex_t
    pub const aura_radius: usize = 0x5B8; // int32
    pub const m_hSign: usize = 0x5BC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Ability_Techies_ReactiveTazer {
    pub const m_hTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Ability_Techies_RemoteMines {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
    pub const m_hRMine: usize = 0x5B4; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Ability_Techies_StasisTrap {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
    pub const m_hTrap: usize = 0x5B4; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Ability_Tinker_WarpGrenade {
    pub const debuff_duration: usize = 0x5B0; // float32
    pub const knockback_distance: usize = 0x5B4; // float32
    pub const damage: usize = 0x5B8; // float32
}

pub mod CDOTA_Ability_Treant_LifeBomb {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
    pub const m_hRMine: usize = 0x5B4; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Ability_Tusk_SnowballMeteor {
    pub const area_of_effect: usize = 0x5B0; // int32
    pub const damage_interval: usize = 0x5B4; // float32
    pub const vision_distance: usize = 0x5B8; // int32
    pub const end_vision_duration: usize = 0x5BC; // float32
    pub const main_damage: usize = 0x5C0; // float32
    pub const burn_duration: usize = 0x5C4; // float32
    pub const burn_dps: usize = 0x5C8; // float32
}

pub mod CDOTA_Ability_Tusk_WalrusKick {
    pub const m_nTargetFXIndex: usize = 0x5B0; // ParticleIndex_t
    pub const m_vEndpoint: usize = 0x5B4; // Vector
    pub const landing_radius: usize = 0x5C0; // int32
    pub const push_length: usize = 0x5C4; // int32
}

pub mod CDOTA_Ability_VoidSpirit_AetherRemnant {
    pub const start_radius: usize = 0x5B0; // int32
    pub const end_radius: usize = 0x5B4; // int32
    pub const remnant_watch_distance: usize = 0x5B8; // int32
    pub const projectile_speed: usize = 0x5BC; // int32
}

pub mod CDOTA_Ability_VoidSpirit_AstralStep {
    pub const m_vStartPos: usize = 0x5B0; // Vector
    pub const m_vDestination: usize = 0x5BC; // Vector
    pub const m_vDirection: usize = 0x5C8; // Vector
    pub const radius: usize = 0x5D4; // int32
    pub const min_travel_distance: usize = 0x5D8; // int32
    pub const max_travel_distance: usize = 0x5DC; // int32
    pub const debuff_duration: usize = 0x5E0; // float32
    pub const pop_damage_delay: usize = 0x5E4; // float32
}

pub mod CDOTA_Ability_VoidSpirit_Dissimilate {
    pub const phase_duration: usize = 0x5B0; // float32
}

pub mod CDOTA_Ability_VoidSpirit_ResonantPulse {
    pub const m_bGrantedScepterCharges: usize = 0x5B0; // bool
    pub const buff_duration: usize = 0x5B4; // float32
    pub const base_absorb_amount: usize = 0x5B8; // int32
    pub const absorb_per_hero_hit: usize = 0x5BC; // int32
}

pub mod CDOTA_Ability_Winter_Wyvern_Arctic_Burn {
    pub const m_BurnedTargets: usize = 0x5B0; // CUtlVector< CEntityIndex >
}

pub mod CDOTA_AghsFort_BossPreview {
    pub const m_strBossUnit: usize = 0x538; // CUtlSymbolLarge
    pub const m_strModelName: usize = 0x540; // CUtlSymbolLarge
    pub const m_flBossModelScale: usize = 0x548; // float32
    pub const m_flExtraModelScale: usize = 0x54C; // float32
    pub const m_nEncounterType: usize = 0x550; // int32
}

pub mod CDOTA_AghsFort_Modifier_Lich_Frost_Giant {
    pub const movement_speed: usize = 0x12C8; // int32
    pub const model_scale: usize = 0x12CC; // int32
    pub const damage_radius: usize = 0x12D0; // int32
    pub const pct_int_damage_per_second: usize = 0x12D4; // int32
    pub const tick_rate: usize = 0x12D8; // float32
}

pub mod CDOTA_AghsFort_Unit_ArcWarden_Boss {
    pub const m_nTalkFXIndex: usize = 0x1AB0; // ParticleIndex_t
    pub const m_nFXDeath: usize = 0x1AB4; // ParticleIndex_t
    pub const m_nTempestFX: usize = 0x1AB8; // ParticleIndex_t
}

pub mod CDOTA_Aghsfort_Modifier_Magnus_Push_Skewer_Movement {
    pub const m_nProjectileID: usize = 0x12C8; // int32
}

pub mod CDOTA_ArcanaDataEntity_DrowRanger {
    pub const m_vecDrowRangerArcanaTargetPlayerID: usize = 0x30; // int8[24]
    pub const m_vecDrowRangerArcanaDeathTime: usize = 0x48; // GameTime_t[24]
    pub const m_vecDrowRangerArcanaKillTime: usize = 0xA8; // GameTime_t[24]
}

pub mod CDOTA_ArcanaDataEntity_FacelessVoid {
    pub const m_nNumPoints: usize = 0x30; // int32
    pub const m_flShowPopupTime: usize = 0x34; // GameTime_t
}

pub mod CDOTA_ArcanaDataEntity_Razor {
    pub const m_nEmpoweredKills: usize = 0x30; // int32
}

pub mod CDOTA_BaseNPC_AghsFort_Watch_Tower {
    pub const m_nOptionNumber: usize = 0x17B8; // int32
    pub const m_flMovePlayersRadius: usize = 0x17BC; // float32
    pub const m_nExitDirection: usize = 0x17C0; // CDOTA_BaseNPC_AghsFort_Watch_Tower::ExitDirection_t
    pub const m_vExitLocation: usize = 0x17C4; // Vector
    pub const m_nPathSelectedID: usize = 0x17D0; // int32
    pub const m_nEncounterType: usize = 0x17D4; // int32
    pub const m_bIsEliteEncounter: usize = 0x17D8; // bool
    pub const m_bIsAscensionLevelPicker: usize = 0x17D9; // bool
    pub const m_strEncounterName: usize = 0x17E0; // CUtlSymbolLarge
    pub const m_strAscensionAbilities: usize = 0x17E8; // CUtlSymbolLarge
    pub const m_hEffigy: usize = 0x17F0; // CHandle< C_BaseEntity >
    pub const m_flYaw: usize = 0x17F4; // float32
    pub const m_flStartTime: usize = 0x17F8; // float32
    pub const m_flCursorEnterTime: usize = 0x17FC; // GameTime_t
    pub const m_bShowingTooltip: usize = 0x1800; // bool
    pub const m_flLastUpdateTime: usize = 0x181C; // float32
    pub const m_nChannellingParticle: usize = 0x1820; // ParticleIndex_t
    pub const m_hRoomGate: usize = 0x1824; // CHandle< CBaseAnimatingActivity >
    pub const m_bIsBeingChanneled: usize = 0x1828; // bool
    pub const m_flGoalCaptureProgress: usize = 0x182C; // float32
}

pub mod CDOTA_BaseNPC_CustomEffigy {
    pub const m_unStatusEffectIndex: usize = 0x17B8; // uint32
    pub const m_hPedestal: usize = 0x17BC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_BaseNPC_Effigy_AghsFort {
    pub const m_unStatusEffectIndex: usize = 0x17B8; // uint32
}

pub mod CDOTA_BaseNPC_Phantom_Assassin_GroundDagger {
    pub const m_nFXIndex: usize = 0x1750; // ParticleIndex_t
}

pub mod CDOTA_Creature_Jungle_Spirit {
    pub const m_iCurrentXP: usize = 0x1760; // int32
    pub const m_fInitialScaleGain: usize = 0x1764; // float32
    pub const m_fScaleGainMultiplierPerLevel: usize = 0x1768; // float32
    pub const m_fLastModelScaleGained: usize = 0x176C; // float32
    pub const m_bGainedSecondPlatemail: usize = 0x1770; // bool
    pub const m_bGainedBonusHealth: usize = 0x1771; // bool
    pub const m_nAmbientFXIndex: usize = 0x1774; // ParticleIndex_t
}

pub mod CDOTA_DB_Page_StickerEntity {
    pub const m_bIsPlaced: usize = 0x538; // bool
    pub const m_ItemDefinitionIndex: usize = 0x53C; // item_definition_index_t
    pub const m_flStickerScale: usize = 0x54C; // float32
    pub const m_unDepthBias: usize = 0x550; // uint16
    pub const m_nStickerNumber: usize = 0x554; // int32
    pub const m_nDbPageNumber: usize = 0x558; // int32
    pub const m_bVisible: usize = 0x55C; // bool
    pub const m_hDecalSpawnGroupHandle: usize = 0x580; // uint32
    pub const m_hDynamicPropSpawnGroupHandle: usize = 0x584; // uint32
}

pub mod CDOTA_ItemStockInfo {
    pub const iTeamNumber: usize = 0x30; // int32
    pub const nItemAbilityID: usize = 0x34; // AbilityID_t
    pub const fStockDuration: usize = 0x38; // float32
    pub const fStockTime: usize = 0x3C; // GameTime_t
    pub const iStockCount: usize = 0x40; // int32
    pub const iMaxCount: usize = 0x44; // int32
    pub const fInitialStockDuration: usize = 0x48; // float32
    pub const iPlayerID: usize = 0x4C; // PlayerID_t
    pub const iBonusDelayedStockCount: usize = 0x50; // int32
}

pub mod CDOTA_Item_Demonicon {
    pub const m_hDemonSummons: usize = 0x648; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Item_Pirate_Hat {
    pub const m_vChannelPos: usize = 0x648; // Vector
    pub const m_bIsUnderwater: usize = 0x654; // bool
    pub const m_nPhaseStartFXIndex: usize = 0x658; // ParticleIndex_t
}

pub mod CDOTA_Item_Tombstone_Drop {
    pub const m_iTempViewer: usize = 0x8A0; // int32
    pub const m_iTeam: usize = 0x8A4; // int32
}

pub mod CDOTA_Item_Trusty_Shovel {
    pub const m_vChannelPos: usize = 0x648; // Vector
    pub const m_bIsUnderwater: usize = 0x654; // bool
    pub const m_nPhaseStartFXIndex: usize = 0x658; // ParticleIndex_t
}

pub mod CDOTA_Item_Ward_Maker {
    pub const sentry_refresh: usize = 0x648; // float32
    pub const max_sentry_charges: usize = 0x64C; // int32
    pub const m_flLastThinkTime: usize = 0x650; // GameTime_t
    pub const m_flTimeAccumulator: usize = 0x654; // float32
}

pub mod CDOTA_Modifer_Furbolg_Enrage_AttackSpeed {
    pub const bonus_aspd: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifer_Furbolg_Enrage_Damage {
    pub const bonus_dmg_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifer_Item_BlightedSpirit {
    pub const magic_resist: usize = 0x12C8; // int32
    pub const duration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifer_Item_Blitz_Knuckles {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifer_Item_DandelionAmulet {
    pub const move_speed: usize = 0x12C8; // int32
    pub const mana: usize = 0x12CC; // int32
    pub const magic_block: usize = 0x12D0; // int32
    pub const min_damage: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifer_Item_GlovesOfHaste {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifer_Item_TurtleShell {
    pub const armor: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifer_Item_TurtleShell_Shell {
    pub const damage_reduction: usize = 0x12C8; // int32
    pub const m_vecPosition: usize = 0x12CC; // Vector
    pub const m_nFXIndex: usize = 0x12D8; // ParticleIndex_t
}

pub mod CDOTA_ModifierManager {
    pub const m_nHasTruesightForTeam: usize = 0x27E; // uint16
    pub const m_nHasTruesightForTeamValid: usize = 0x280; // uint16
    pub const m_nProvidesFOWPositionForTeam: usize = 0x282; // uint16
    pub const m_nProvidesFOWPositionForTeamValid: usize = 0x284; // uint16
    pub const m_iBuffIndex: usize = 0x288; // int32
    pub const m_iLockRefCount: usize = 0x28C; // int32
}

pub mod CDOTA_Modifier_ARDM_NewHero {
    pub const m_hOldHero: usize = 0x12C8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Abaddon_AphoticShield {
    pub const radius: usize = 0x12C8; // int32
    pub const damage_absorb: usize = 0x12CC; // float32
    pub const m_nDamageAbsorbed: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Abaddon_BorrowedTime {
    pub const redirect_range_scepter: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Abaddon_BorrowedTime_ImmolationAura {
    pub const immolate_aoe: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Abaddon_BorrowedTime_ImmolationDamage {
    pub const immolate_damage: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Abaddon_BorrowedTime_Passive {
    pub const hp_threshold: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Abaddon_Borrowed_Time_Damage_Redirect {
    pub const m_fDamageTaken: usize = 0x12C8; // float32
    pub const ally_threshold_scepter: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Abaddon_DeathCoil_Effect {
    pub const armor_bonus: usize = 0x12C8; // int32
    pub const magic_resistance: usize = 0x12CC; // int32
    pub const m_bFriendly: usize = 0x12D0; // bool
}

pub mod CDOTA_Modifier_Abaddon_Frostmourne {
    pub const slow_duration: usize = 0x12C8; // float32
    pub const curse_duration: usize = 0x12CC; // float32
    pub const hit_count: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Abaddon_Frostmourne_Buff {
    pub const curse_attack_speed: usize = 0x12C8; // int32
    pub const m_hTarget: usize = 0x12CC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Abaddon_Frostmourne_Debuff {
    pub const movement_speed: usize = 0x12C8; // int32
    pub const m_nFXStackIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Abaddon_Frostmourne_Debuff_Bonus {
    pub const curse_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AbyssalUnderlord_AtrophyAura {
    pub const radius: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_AbyssalUnderlord_AtrophyAura_CreepDmgBuff {
    pub const bonus_damage_from_creep: usize = 0x12C8; // int32
    pub const bonus_shared_by_allies_pct: usize = 0x12CC; // float32
    pub const m_flModifier: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_AbyssalUnderlord_AtrophyAura_Effect {
    pub const damage_reduction_pct: usize = 0x12C8; // int32
    pub const bonus_damage_duration: usize = 0x12CC; // float32
    pub const bonus_damage_from_creep: usize = 0x12D0; // int32
    pub const bonus_damage_from_hero: usize = 0x12D4; // int32
    pub const m_bWasHidden: usize = 0x12D8; // bool
}

pub mod CDOTA_Modifier_AbyssalUnderlord_AtrophyAura_HeroDmgBuff {
    pub const bonus_damage_from_hero: usize = 0x12C8; // int32
    pub const bonus_shared_by_allies_pct: usize = 0x12CC; // float32
    pub const m_flModifier: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_AbyssalUnderlord_AtrophyAura_Scepter {
    pub const bonus_damage_from_creep: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AbyssalUnderlord_DarkRift {
    pub const m_nfxTargetTp: usize = 0x12C8; // ParticleIndex_t
    pub const radius: usize = 0x12CC; // int32
    pub const duration: usize = 0x12D0; // int32
    pub const m_nfxAmbientFx: usize = 0x12D4; // ParticleIndex_t
    pub const bPointTarget: usize = 0x12D8; // bool
    pub const vDestination: usize = 0x12DC; // Vector
}

pub mod CDOTA_Modifier_AbyssalUnderlord_DarkRift_Bonus_Health {
    pub const scepter_health_bonus: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AbyssalUnderlord_Firestorm_Burn {
    pub const burn_damage: usize = 0x12C8; // float32
    pub const burn_interval: usize = 0x12CC; // float32
    pub const building_damage: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_AbyssalUnderlord_Firestorm_Thinker {
    pub const wave_damage: usize = 0x12C8; // int32
    pub const wave_count: usize = 0x12CC; // int32
    pub const radius: usize = 0x12D0; // int32
    pub const wave_interval: usize = 0x12D4; // float32
    pub const burn_duration: usize = 0x12D8; // float32
    pub const burn_interval: usize = 0x12DC; // float32
    pub const first_wave_delay: usize = 0x12E0; // float32
    pub const building_damage: usize = 0x12E4; // float32
    pub const m_hTarget: usize = 0x12E8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_AbyssalUnderlord_PitOfMalice_Slow {
    pub const speed_bonus: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AbyssalUnderlord_PitOfMalice_Thinker {
    pub const pit_damage: usize = 0x12C8; // int32
    pub const radius: usize = 0x12CC; // int32
    pub const pit_interval: usize = 0x12D0; // float32
    pub const ensnare_duration: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Activity_Modifier {
    pub const m_activityModifier: usize = 0x12C8; // CUtlSymbolLarge
}

pub mod CDOTA_Modifier_Aether_Lens {
    pub const bonus_mana: usize = 0x12C8; // int32
    pub const bonus_mana_regen: usize = 0x12CC; // float32
    pub const cast_range_bonus: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Aetherial_Halo {
    pub const bonus_all_stats: usize = 0x12C8; // int32
    pub const bonus_health: usize = 0x12CC; // int32
    pub const bonus_mana: usize = 0x12D0; // int32
    pub const bonus_mana_regen: usize = 0x12D4; // float32
    pub const slow_duration: usize = 0x12D8; // float32
    pub const cast_range_bonus: usize = 0x12DC; // int32
    pub const aoe_bonus: usize = 0x12E0; // int32
    pub const bonus_spell_damage: usize = 0x12E4; // int32
}

pub mod CDOTA_Modifier_AghsFort2_DrowRanger_FrostArrows {
    pub const m_InFlightAttackRecords: usize = 0x12C8; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_AghsFort2_DrowRanger_FrostArrows_Chill {
    pub const interval: usize = 0x12C8; // float32
    pub const max_stacks: usize = 0x12CC; // int32
    pub const damage_percent: usize = 0x12D0; // int32
    pub const movement_damage_percent: usize = 0x12D4; // int32
    pub const m_nDamage: usize = 0x12D8; // int32
    pub const m_nFXStackIndex: usize = 0x12DC; // ParticleIndex_t
    pub const m_vLastPosition: usize = 0x12E0; // Vector
}

pub mod CDOTA_Modifier_AghsFort2_DrowRanger_Marksmanship_Active {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const bonus_range: usize = 0x12CC; // int32
    pub const attack_speed_reduction: usize = 0x12D0; // int32
    pub const duration: usize = 0x12D4; // float32
    pub const charges: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_AghsFort2_DrowRanger_Marksmanship_WaveOfSilence_Primed {
    pub const aura_disable_range: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort2_DrowRanger_WaveOfSilence {
    pub const blind_percent: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort2_DrowRanger_WaveOfSilence_Agility {
    pub const m_AgilityTimers: usize = 0x12C8; // CUtlVector< GameTime_t >
}

pub mod CDOTA_Modifier_AghsFort2_DrowRanger_WaveOfSilence_Echo_Thinker {
    pub const m_vTarget: usize = 0x12C8; // Vector
}

pub mod CDOTA_Modifier_AghsFort2_DrowRanger_WaveOfSilence_Frost {
    pub const m_fDamagePerSecond: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_AghsFort2_DrowRanger_WaveOfSilence_Movement {
    pub const movespeed_percent: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_ArcWardenBoss_Flux {
    pub const m_nCasterTeam: usize = 0x12C8; // int32
    pub const damage_per_tick_min: usize = 0x12CC; // int32
    pub const damage_per_tick_max: usize = 0x12D0; // int32
    pub const search_radius: usize = 0x12D4; // int32
    pub const move_speed_slow_pct: usize = 0x12D8; // int32
    pub const think_interval: usize = 0x12DC; // float32
    pub const m_fCurrentDamage: usize = 0x12E0; // float32
    pub const m_fDamageIncreasePerTick: usize = 0x12E4; // float32
    pub const m_nCurrentSlow: usize = 0x12E8; // int32
    pub const m_bAlone: usize = 0x12EC; // bool
    pub const m_nFXIndex: usize = 0x12F0; // ParticleIndex_t
}

pub mod CDOTA_Modifier_AghsFort_ArcWardenBoss_MagneticField_Attack_Speed {
    pub const attack_speed_bonus: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_ArcWardenBoss_MagneticField_Evasion {
    pub const evasion_chance: usize = 0x12C8; // int32
    pub const radius: usize = 0x12CC; // int32
    pub const aura_origin_x: usize = 0x12D0; // float32
    pub const aura_origin_y: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_AghsFort_ArcWardenBoss_MagneticField_Thinker_Attack_Speed {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_ArcWardenBoss_MagneticField_Thinker_Evasion {
    pub const radius: usize = 0x12D0; // int32
    pub const explosion_delay_time: usize = 0x12D4; // float32
    pub const explosion_stun_duration: usize = 0x12D8; // float32
    pub const explosion_damage: usize = 0x12DC; // int32
    pub const caster_team: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_AghsFort_ArcWardenBoss_MagneticField_Thinker_Explosion {
    pub const radius: usize = 0x12C8; // int32
    pub const explosion_damage: usize = 0x12CC; // int32
    pub const explosion_stun_duration: usize = 0x12D0; // float32
    pub const explosion_delay_time: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_AghsFort_ArcWardenBoss_MeteorShockwave_Thinker {
    pub const m_fLastRadius: usize = 0x12C8; // float32
    pub const m_fCurRadius: usize = 0x12CC; // float32
    pub const m_fLastThink: usize = 0x12D0; // GameTime_t
    pub const m_bWindingUp: usize = 0x12D4; // bool
    pub const m_ViewerTimer: usize = 0x12D8; // CountdownTimer
    pub const m_nFXIndex: usize = 0x12F0; // ParticleIndex_t
    pub const m_EntitiesHit: usize = 0x12F8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const speed: usize = 0x1310; // int32
    pub const radius: usize = 0x1314; // int32
    pub const knockback_distance: usize = 0x1318; // float32
    pub const knockback_duration: usize = 0x131C; // float32
    pub const windup_time: usize = 0x1320; // float32
}

pub mod CDOTA_Modifier_AghsFort_ArcWardenBoss_SparkWraith_Purge {
    pub const move_speed_slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_ArcWardenBoss_SparkWraith_Thinker {
    pub const radius: usize = 0x12C8; // int32
    pub const wraith_vision_radius: usize = 0x12CC; // int32
    pub const wraith_speed: usize = 0x12D0; // int32
    pub const activation_delay: usize = 0x12DC; // float32
    pub const scepter_activation_delay: usize = 0x12E0; // float32
    pub const think_interval: usize = 0x12E4; // float32
    pub const search_timer_min: usize = 0x12E8; // float32
    pub const search_timer_max: usize = 0x12EC; // float32
    pub const mini_stun_duration: usize = 0x12F0; // float32
    pub const move_speed_slow_pct: usize = 0x12F4; // int32
    pub const m_nSparkDamage: usize = 0x12F8; // int32
    pub const m_bActive: usize = 0x12FC; // bool
    pub const m_bOriginal: usize = 0x12FD; // bool
    pub const m_fSearchTimer: usize = 0x1300; // float32
    pub const m_fRotationSpeed: usize = 0x1304; // float32
    pub const m_nRotationDirection: usize = 0x1308; // int32
    pub const m_vRotationCenter: usize = 0x130C; // Vector
    pub const m_nFXIndex: usize = 0x1318; // ParticleIndex_t
}

pub mod CDOTA_Modifier_AghsFort_ArcWardenBoss_TempestDouble {
    pub const damage_increase: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_ArcWardenBoss_TempestDouble_Phase {
    pub const num_doubles: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Arcanist_Potion {
    pub const m_nCooldownReductionPct: usize = 0x12C8; // int32
    pub const m_nManaCostReductionPct: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Ascension_MagneticField_Evasion {
    pub const evasion_chance: usize = 0x12C8; // int32
    pub const radius: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Ascension_MagneticField_Thinker_Evasion {
    pub const radius: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_AghsFort_Ascension_PlasmaField_Slow {
    pub const slow_amount: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Ascension_PlasmaField_Thinker {
    pub const m_fLastRadius: usize = 0x12C8; // float32
    pub const m_fCurRadius: usize = 0x12CC; // float32
    pub const m_fLastThink: usize = 0x12D0; // GameTime_t
    pub const m_bWindingUp: usize = 0x12D4; // bool
    pub const m_ViewerTimer: usize = 0x12D8; // CountdownTimer
    pub const m_nFXIndex: usize = 0x12F0; // ParticleIndex_t
    pub const m_EntitiesHit: usize = 0x12F8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const speed: usize = 0x1310; // int32
    pub const radius: usize = 0x1314; // int32
    pub const damage_min: usize = 0x1318; // int32
    pub const damage_max: usize = 0x131C; // int32
    pub const slow_min: usize = 0x1320; // int32
    pub const slow_max: usize = 0x1324; // int32
    pub const slow_duration: usize = 0x1328; // float32
    pub const windup_time: usize = 0x132C; // float32
}

pub mod CDOTA_Modifier_AghsFort_AssaultCaptain_SunRay {
    pub const hp_cost_perc_per_second: usize = 0x12C8; // int32
    pub const beam_range: usize = 0x12CC; // int32
    pub const base_damage: usize = 0x12D0; // int32
    pub const base_heal: usize = 0x12D4; // int32
    pub const tick_interval: usize = 0x12D8; // float32
    pub const forward_move_speed: usize = 0x12DC; // float32
    pub const turn_rate_initial: usize = 0x12E0; // float32
    pub const turn_rate: usize = 0x12E4; // float32
    pub const radius: usize = 0x12E8; // int32
    pub const self_turn_rate_percent: usize = 0x12EC; // int32
    pub const m_flCurrentTime: usize = 0x12F0; // float32
    pub const m_flLastDamageTime: usize = 0x12F4; // GameTime_t
    pub const m_flAccumulatedSelfDamage: usize = 0x12F8; // float32
    pub const m_bMovingForward: usize = 0x12FC; // bool
    pub const m_bTurningFast: usize = 0x12FD; // bool
    pub const m_flFacingTarget: usize = 0x1300; // float32
    pub const hp_perc_damage: usize = 0x1304; // float32
    pub const hp_perc_heal: usize = 0x1308; // float32
    pub const m_nBeamFXIndex: usize = 0x130C; // ParticleIndex_t
    pub const m_hVisionThinkers: usize = 0x1310; // CHandle< C_BaseEntity >[8]
    pub const m_hBeamEnd: usize = 0x1330; // CHandle< C_BaseEntity >
    pub const m_bCreatedVisionThinkers: usize = 0x1334; // bool
    pub const m_hBeamEndSound: usize = 0x1338; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_AghsFort_Bane_BrainSap_Invis {
    pub const invis_movement_speed: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Bane_BrainSap_Knockback {
    pub const radius: usize = 0x12F8; // int32
    pub const heal_pct: usize = 0x12FC; // int32
    pub const slow_duration: usize = 0x1300; // float32
    pub const m_nBrainSapDamage: usize = 0x1304; // int32
    pub const m_vecHitTargets: usize = 0x1308; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_AghsFort_Bane_BrainSap_KnockbackSlow {
    pub const movement_slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Bane_BrainSap_NightmareMirror {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Bane_FiendsGripThinker {
    pub const latch_range: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Bane_Invis_Crit {
    pub const invis_brain_sap_crit_pct: usize = 0x12C8; // int32
    pub const m_nCritPct: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_BossDarkWillow_CursedCrown {
    pub const delay: usize = 0x12C8; // float32
    pub const stun_duration: usize = 0x12CC; // float32
    pub const stun_radius: usize = 0x12D0; // int32
    pub const damage: usize = 0x12D4; // int32
    pub const m_fStartTime: usize = 0x12D8; // GameTime_t
}

pub mod CDOTA_Modifier_AghsFort_BossDarkWillow_ShadowRealm_Buff {
    pub const m_fAddInterval: usize = 0x12C8; // float32
    pub const m_fNextAddTime: usize = 0x12CC; // GameTime_t
    pub const m_flStartTime: usize = 0x12D0; // GameTime_t
    pub const m_flDamageScale: usize = 0x12D4; // float32
    pub const duration: usize = 0x12D8; // float32
    pub const attack_logic_duration: usize = 0x12DC; // float32
    pub const speed_boost: usize = 0x12E0; // int32
    pub const num_adds: usize = 0x12E4; // int32
    pub const projectile_range: usize = 0x12E8; // int32
    pub const projectile_speed: usize = 0x12EC; // int32
    pub const projectile_width: usize = 0x12F0; // int32
}

pub mod CDOTA_Modifier_AghsFort_BossEarthshaker_EnchantTotem {
    pub const bonus_attack_range: usize = 0x12C8; // int32
    pub const totem_damage_percentage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_BossEarthshaker_EnchantTotem_LandDebuff {
    pub const landing_move_slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_BossEarthshaker_Fissure_Shard {
    pub const m_vStartPos: usize = 0x12C8; // Vector
    pub const m_vEndPos: usize = 0x12D4; // Vector
    pub const m_flEndTime: usize = 0x12E0; // GameTime_t
}

pub mod CDOTA_Modifier_AghsFort_BossWinterWyvern_Cold_Embrace_Debuff {
    pub const linger_movement_slow: usize = 0x12C8; // int32
    pub const linger_damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_BossWinterWyvern_Cold_Embrace_Thinker {
    pub const freeze_radius: usize = 0x12C8; // int32
    pub const freeze_damage: usize = 0x12CC; // int32
    pub const freeze_debuff_duration: usize = 0x12D0; // float32
    pub const freeze_delay: usize = 0x12D4; // float32
    pub const m_nFXIndex: usize = 0x12D8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_AghsFort_Clinkz_BurningArmy {
    pub const attack_rate: usize = 0x12C8; // float32
    pub const damage_percent: usize = 0x12CC; // int32
    pub const m_nFXIndex: usize = 0x12D0; // ParticleIndex_t
    pub const m_bHasAttacked: usize = 0x12D4; // bool
}

pub mod CDOTA_Modifier_AghsFort_Clinkz_BurningArmy_BigSkeletons {
    pub const model_scale: usize = 0x1300; // int32
    pub const radius: usize = 0x1304; // float32
    pub const splitshot_count: usize = 0x1308; // int32
    pub const debuff_duration: usize = 0x130C; // float32
    pub const pct_for_moveslow: usize = 0x1310; // float32
}

pub mod CDOTA_Modifier_AghsFort_Clinkz_BurningArmy_BigSkeletons_Debuff {
    pub const m_fMovementSlow: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_AghsFort_Clinkz_BurningArmy_Entrenchment_Buff {
    pub const m_fLifestealPct: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_AghsFort_Clinkz_BurningArmy_Entrenchment_Debuff {
    pub const m_fSlowPct: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_AghsFort_Clinkz_BurningArmy_Thinker {
    pub const m_iCount: usize = 0x12C8; // int32
    pub const m_vFacePosition: usize = 0x12CC; // Vector
}

pub mod CDOTA_Modifier_AghsFort_Clinkz_BurningBarrage {
    pub const m_vOriginalTarget: usize = 0x12C8; // Vector
    pub const wave_count: usize = 0x12D4; // int32
    pub const projectile_speed: usize = 0x12D8; // int32
    pub const projectile_width: usize = 0x12DC; // int32
    pub const m_flInterval: usize = 0x12E0; // float32
    pub const m_nArrowsFired: usize = 0x12E4; // int32
}

pub mod CDOTA_Modifier_AghsFort_Clinkz_BurningBarrage_DamageReduction {
    pub const damage_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Clinkz_BurningBarrage_Singe_Slow {
    pub const move_slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Clinkz_SearingArrows {
    pub const m_InFlightAttackRecords: usize = 0x12C8; // CUtlVector< int16 >
    pub const m_bBonusAttack: usize = 0x12E0; // bool
    pub const damage_bonus: usize = 0x12E4; // int32
    pub const debuff_duration: usize = 0x12E8; // float32
}

pub mod CDOTA_Modifier_AghsFort_Clinkz_SearingArrows_Debuff {
    pub const attack_slow: usize = 0x12C8; // int32
    pub const m_fIgniteThinkInterval: usize = 0x12CC; // float32
    pub const m_fIgniteDmgPerSec: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_AghsFort_Clinkz_SearingArrows_Melter {
    pub const armor_reduction_per_arrow: usize = 0x12C8; // float32
    pub const m_nMaxStacks: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Clinkz_SkeletalColossus_BigSkeletons {
    pub const m_nInitialHealthBarOffset: usize = 0x12C8; // int32
    pub const colossus_model_scale: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Clinkz_SkeletalColossus_Thinker {
    pub const lifetime: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_AghsFort_Clinkz_SkeletonColossus {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const cleave_damage_pct: usize = 0x12CC; // float32
    pub const cleave_starting_width: usize = 0x12D0; // int32
    pub const cleave_ending_width: usize = 0x12D4; // int32
    pub const cleave_distance: usize = 0x12D8; // int32
    pub const block_damage_ranged_active: usize = 0x12DC; // int32
    pub const block_damage_melee_active: usize = 0x12E0; // int32
    pub const hp_gain_per_caster_level: usize = 0x12E4; // int32
    pub const hp_regen_per_caster_level: usize = 0x12E8; // float32
}

pub mod CDOTA_Modifier_AghsFort_Clinkz_SkeletonWalk {
    pub const m_vecWraithTouchedEntities: usize = 0x12D8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const move_speed_bonus_pct: usize = 0x12F0; // int32
    pub const bonus_magic_resist: usize = 0x12F4; // int32
}

pub mod CDOTA_Modifier_AghsFort_Clinkz_SkeletonWalk_Strafe {
    pub const m_fAttackSpeed: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_AghsFort_Clinkz_SkeletonWalk_WraithTouch_Caster {
    pub const max_stacks: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Clinkz_SkeletonWalk_WraithTouch_Enemy {
    pub const m_fAttackLoss: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_AghsFort_Creature_Phoenix_FireSpiritCount {
    pub const spirit_count: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_AghsFort_Creature_Phoenix_Sun {
    pub const aura_radius: usize = 0x12C8; // int32
    pub const stun_duration: usize = 0x12CC; // float32
    pub const max_hero_attacks: usize = 0x12D0; // int32
    pub const max_hero_attacks_scepter: usize = 0x12D4; // int32
    pub const max_hero_attacks_required: usize = 0x12D8; // int32
    pub const caster_life_pct: usize = 0x12DC; // float32
    pub const m_hSecondaryTarget: usize = 0x12E0; // CHandle< C_BaseEntity >
    pub const m_iAttackCount: usize = 0x12E4; // int32
}

pub mod CDOTA_Modifier_AghsFort_Creature_SpikedCarapace {
    pub const stun_duration: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_AghsFort_Creature_Venomancer_PoisonNova {
    pub const damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Creature_Venomancer_PoisonSting_Applier {
    pub const duration: usize = 0x12C8; // float32
    pub const damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Dawnbreaker_Starbreaker_CounterOrbital {
    pub const trigger_chance: usize = 0x12C8; // int32
    pub const cooldown: usize = 0x12CC; // float32
    pub const m_fLastProc: usize = 0x12D0; // GameTime_t
}

pub mod CDOTA_Modifier_AghsFort_DragonPotion {
    pub const m_iOriginalAttackCapabilities: usize = 0x12C8; // int32
    pub const m_iszRangedAttackEffect: usize = 0x12D0; // CUtlSymbolLarge
    pub const bonus_movement_speed: usize = 0x12D8; // int32
    pub const bonus_attack_damage: usize = 0x12DC; // int32
    pub const bonus_attack_range: usize = 0x12E0; // int32
    pub const attack_projectile_speed: usize = 0x12E4; // int32
    pub const magic_resistance: usize = 0x12E8; // int32
    pub const model_scale: usize = 0x12EC; // int32
    pub const skin_number: usize = 0x12F0; // int32
    pub const flying_movement: usize = 0x12F4; // bool
}

pub mod CDOTA_Modifier_AghsFort_EarthSpiritBoss_BoulderSmash {
    pub const m_bInterrupted: usize = 0x12C8; // bool
    pub const m_nProjectileID: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_EarthSpiritBoss_BoulderSmash_Debuff {
    pub const move_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_EarthSpiritBoss_GeomagneticGrip {
    pub const radius: usize = 0x12C8; // int32
    pub const rock_damage: usize = 0x12CC; // int32
    pub const pull_units_per_second: usize = 0x12D0; // float32
    pub const pull_units_per_second_heroes: usize = 0x12D4; // float32
    pub const total_pull_distance: usize = 0x12D8; // float32
    pub const duration: usize = 0x12DC; // float32
    pub const m_bUsedStone: usize = 0x12E0; // bool
    pub const m_vDestination: usize = 0x12E4; // Vector
    pub const m_vLocation: usize = 0x12F0; // Vector
    pub const m_hHitEntities: usize = 0x1300; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_AghsFort_EarthSpiritBoss_Magnetize {
    pub const rock_search_radius: usize = 0x12C8; // int32
    pub const damage_per_second: usize = 0x12CC; // int32
    pub const damage_interval: usize = 0x12D0; // float32
    pub const rock_explosion_delay: usize = 0x12D4; // float32
    pub const damage_duration: usize = 0x12D8; // float32
    pub const cast_radius: usize = 0x12DC; // int32
    pub const rock_explosion_radius: usize = 0x12E0; // int32
    pub const m_hExplodedRocks: usize = 0x12E8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const duration: usize = 0x1300; // float32
    pub const m_bShowOverhead: usize = 0x1304; // bool
}

pub mod CDOTA_Modifier_AghsFort_EarthSpiritBoss_Petrify {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const m_bHasBeenMagnetized: usize = 0x12CC; // bool
}

pub mod CDOTA_Modifier_AghsFort_EarthSpiritBoss_StoneThinker {
    pub const m_bHasBeenMagnetized: usize = 0x12C8; // bool
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_AghsFort_ExplosiveBarrel {
    pub const m_nPreviewFX: usize = 0x12C8; // ParticleIndex_t
    pub const radius: usize = 0x12CC; // int32
    pub const damage: usize = 0x12D0; // int32
    pub const vision_radius: usize = 0x12D4; // int32
    pub const vision_duration: usize = 0x12D8; // float32
    pub const model_scale: usize = 0x12DC; // int32
    pub const bExploding: usize = 0x12E0; // bool
    pub const stun_duration: usize = 0x12E4; // float32
}

pub mod CDOTA_Modifier_AghsFort_Firefly {
    pub const pool_duration: usize = 0x12C8; // float32
    pub const radius: usize = 0x12CC; // int32
    pub const m_ifirefly_Active: usize = 0x12D0; // int32
    pub const tree_radius: usize = 0x12D4; // int32
    pub const damage_pct_per_second: usize = 0x12D8; // int32
    pub const movement_speed: usize = 0x12DC; // int32
    pub const trail_placement_duration: usize = 0x12E0; // float32
    pub const burn_linger_duration: usize = 0x12E4; // float32
    pub const m_bActive: usize = 0x12E8; // bool
    pub const m_nFXIndex: usize = 0x12EC; // ParticleIndex_t
    pub const m_nFXIndexB: usize = 0x12F0; // ParticleIndex_t
    pub const m_flStartTime: usize = 0x12F4; // GameTime_t
    pub const m_vLastFirePoolLoc: usize = 0x12F8; // Vector
    pub const m_vFirePoolLocations: usize = 0x1308; // CUtlVector< Vector >
}

pub mod CDOTA_Modifier_AghsFort_Firefly_Burn {
    pub const damage_pct_per_second: usize = 0x12C8; // int32
    pub const movement_speed: usize = 0x12CC; // int32
    pub const tick_interval: usize = 0x12D0; // float32
    pub const m_fNextDamageTick: usize = 0x12D4; // GameTime_t
}

pub mod CDOTA_Modifier_AghsFort_Juggernaut_BladeDance {
    pub const blade_dance_crit_mult: usize = 0x12C8; // int32
    pub const blade_dance_crit_chance: usize = 0x12CC; // int32
    pub const m_bDidBladeDanceCrit: usize = 0x12D0; // bool
}

pub mod CDOTA_Modifier_AghsFort_Juggernaut_BladeDance_Buff {
    pub const evasion_per_stack: usize = 0x12C8; // int32
    pub const movespeed_per_stack: usize = 0x12CC; // int32
    pub const evasion_bonus: usize = 0x12D0; // int32
    pub const movespeed_bonus: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_AghsFort_Juggernaut_BladeFury {
    pub const blade_fury_radius: usize = 0x12C8; // int32
    pub const blade_fury_damage: usize = 0x12CC; // int32
    pub const blade_fury_damage_tick: usize = 0x12D0; // float32
    pub const movement_bonus: usize = 0x12D4; // int32
    pub const spark_radius: usize = 0x12D8; // int32
    pub const spark_damage_percent: usize = 0x12DC; // int32
    pub const m_bDestroyParentThinkerOnDeath: usize = 0x12E0; // bool
}

pub mod CDOTA_Modifier_AghsFort_Juggernaut_BladeFury_Force {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Juggernaut_BladeFury_Force_Pull {
    pub const blade_fury_radius: usize = 0x12C8; // int32
    pub const pull_speed: usize = 0x12CC; // int32
    pub const pull_stop: usize = 0x12D0; // int32
    pub const pull_origin: usize = 0x12D4; // Vector
    pub const m_flLastThinkTime: usize = 0x12E0; // GameTime_t
}

pub mod CDOTA_Modifier_AghsFort_Juggernaut_Double_Attack {
    pub const range: usize = 0x12C8; // int32
    pub const bonus_attack_duration: usize = 0x12CC; // float32
    pub const bonus_attack_count: usize = 0x12D0; // int32
    pub const bonus_attack_speed: usize = 0x12D4; // int32
    pub const m_fBonusRange: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_AghsFort_Juggernaut_Healing_Ward {
    pub const healing_ward_aura_radius: usize = 0x12C8; // int32
    pub const healing_ward_heal_amount: usize = 0x12CC; // float32
    pub const healing_ward_interval: usize = 0x12D0; // float32
    pub const m_nTotalHealing: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_AghsFort_Juggernaut_Healing_Ward_Primed {
    pub const healing_ward_aura_radius: usize = 0x12C8; // int32
    pub const trigger_radius: usize = 0x12CC; // int32
    pub const search_radius: usize = 0x12D0; // int32
    pub const damage_percent: usize = 0x12D4; // int32
    pub const movement_bonus: usize = 0x12D8; // int32
    pub const power: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Juggernaut_Omnislash_Ignite {
    pub const slow_movement_speed_pct: usize = 0x12C8; // int32
    pub const tick_damage: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_AghsFort_Juggernaut_Zen_Ward {
    pub const healing_ward_aura_radius: usize = 0x12C8; // int32
    pub const healing_ward_heal_amount: usize = 0x12CC; // float32
    pub const healing_ward_interval: usize = 0x12D0; // float32
    pub const m_nTotalRegen: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_AghsFort_Juggernaut_Zen_Ward_Armor {
    pub const armor_bonus: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Kunkka_Fire_Cannons_Slow {
    pub const movespeed_bonus: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Kunkka_GhostShip_DamageAbsorb {
    pub const m_iAccumulatedDamage: usize = 0x12C8; // int32
    pub const movespeed_bonus: usize = 0x12CC; // int32
    pub const ghostship_absorb: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_AghsFort_Kunkka_GhostShip_DamageDelay {
    pub const m_iAccumulatedDamage: usize = 0x12C8; // int32
    pub const m_flDuration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_AghsFort_Kunkka_Ghost_Ship_Fire_Cannons {
    pub const m_flDistance: usize = 0x12C8; // float32
    pub const m_flSpeed: usize = 0x12CC; // float32
    pub const m_nNumCannons: usize = 0x12D0; // int32
    pub const m_flProjectileRadius: usize = 0x12D4; // float32
    pub const m_bGhostShipIsEntity: usize = 0x12D8; // bool
    pub const m_hGhostShipProjectile: usize = 0x12DC; // int32
    pub const m_hGhostShip: usize = 0x12E0; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_AghsFort_Kunkka_Ghost_Ship_Fleet {
    pub const vCasterOrigin: usize = 0x12C8; // Vector
    pub const vEnd: usize = 0x12D4; // Vector
    pub const m_nHitCount: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_AghsFort_Kunkka_SeaSickness {
    pub const m_iAccumulatedDamage: usize = 0x12C8; // int32
    pub const m_flDuration: usize = 0x12CC; // float32
    pub const m_flMovespeedBonus: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_AghsFort_Kunkka_Tidebringer {
    pub const m_bTidebringerAttack: usize = 0x12C8; // bool
    pub const cleave_starting_width: usize = 0x12CC; // int32
    pub const cleave_ending_width: usize = 0x12D0; // int32
    pub const cleave_distance: usize = 0x12D4; // int32
    pub const damage_bonus: usize = 0x12D8; // int32
    pub const m_nFXIndex: usize = 0x12DC; // ParticleIndex_t
    pub const cleave_damage: usize = 0x12E0; // int32
    pub const m_bIgnoreCooldown: usize = 0x12E4; // bool
    pub const m_bDreadPirate: usize = 0x12E5; // bool
}

pub mod CDOTA_Modifier_AghsFort_Kunkka_Torrent {
    pub const flIntervalThinkTime: usize = 0x12C8; // float32
    pub const torrent_damage: usize = 0x12CC; // int32
    pub const m_flDamageScale: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_AghsFort_Kunkka_Torrent_Slow {
    pub const movespeed_bonus: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Kunkka_Torrent_Storm {
    pub const torrent_min_distance: usize = 0x12C8; // int32
    pub const torrent_max_distance: usize = 0x12CC; // int32
    pub const m_iExplosionQuadrant: usize = 0x12D0; // int32
    pub const m_vPosition: usize = 0x12D4; // Vector
}

pub mod CDOTA_Modifier_AghsFort_Kunkka_Torrent_Thinker {
    pub const m_bIsSquall: usize = 0x12C8; // bool
    pub const m_bIsSeaLegs: usize = 0x12C9; // bool
    pub const m_flRadius: usize = 0x12CC; // float32
    pub const m_bShowEnemies: usize = 0x12D0; // bool
    pub const m_bTorrentStorm: usize = 0x12D1; // bool
    pub const m_flDelay: usize = 0x12D4; // float32
    pub const m_flFireTime: usize = 0x12D8; // float32
    pub const m_bForcePosition: usize = 0x12DC; // bool
    pub const m_vStartPosition: usize = 0x12E0; // Vector
    pub const m_vEndPosition: usize = 0x12EC; // Vector
    pub const m_nPercentChanceOfGhostShip: usize = 0x12F8; // int32
    pub const m_hTarget: usize = 0x12FC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_AghsFort_Kunkka_XMarksTheSpot {
    pub const armour_increase: usize = 0x12C8; // float32
    pub const speed_increase_pct: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_AghsFort_Kunkka_XMarksTheSpot_SeaLegs_Thinker {
    pub const m_vLastPosition: usize = 0x12C8; // Vector
    pub const m_hEntity: usize = 0x12D4; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_AghsFort_Kunkka_XMarksTheSpot_Thinker {
    pub const m_vStartLoc: usize = 0x12C8; // Vector
    pub const m_bIsReflection: usize = 0x12D4; // bool
    pub const m_hEntity: usize = 0x12D8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_AghsFort_Lich_ChainFrost_Slow {
    pub const slow_movement_speed: usize = 0x12C8; // int32
    pub const slow_attack_speed: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Lich_Chain_Frost_Thinker {
    pub const m_nDamageToHeroes: usize = 0x12C8; // int32
    pub const m_nHeroesKilled: usize = 0x12CC; // int32
    pub const m_nJumps: usize = 0x12D0; // int32
    pub const m_hTarget: usize = 0x12D4; // CHandle< C_BaseEntity >
    pub const m_hAvoidTarget: usize = 0x12D8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_AghsFort_Lich_FrostNova_Slow {
    pub const slow_movement_speed: usize = 0x12C8; // int32
    pub const slow_attack_speed: usize = 0x12CC; // int32
    pub const is_primary_target: usize = 0x12D0; // bool
}

pub mod CDOTA_Modifier_AghsFort_Lich_FrostShield {
    pub const radius: usize = 0x12C8; // int32
    pub const interval: usize = 0x12CC; // float32
    pub const damage_reduction: usize = 0x12D0; // int32
    pub const bonus_health_regen: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_AghsFort_Lich_FrostShield_Magic_Resistance_Debuff {
    pub const magic_resist_reduction: usize = 0x12C8; // int32
    pub const max_stacks: usize = 0x12CC; // int32
    pub const m_nFXStackIndex: usize = 0x12D0; // ParticleIndex_t
}

pub mod CDOTA_Modifier_AghsFort_Lich_FrostShield_Slow {
    pub const movement_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Lich_Ice_Spire {
    pub const aura_radius: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
    pub const m_Timer: usize = 0x12D0; // CountdownTimer
}

pub mod CDOTA_Modifier_AghsFort_Lich_Ice_Spire_Debuff {
    pub const bonus_movespeed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Lich_Sinister_Gaze {
    pub const movement_speed: usize = 0x12C8; // int32
    pub const mana_drain: usize = 0x12CC; // int32
    pub const m_flIntervalRate: usize = 0x12D0; // float32
    pub const m_nFXIndex: usize = 0x12D4; // ParticleIndex_t
}

pub mod CDOTA_Modifier_AghsFort_Lich_Sinister_Gaze_Self {
    pub const mana_drain: usize = 0x12C8; // int32
    pub const m_flIntervalRate: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_AghsFort_Lich_Sinister_Gaze_Summon {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const base_damage_outgoing_percentage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Lifestealer_Enraged_Pulse {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Luna_Eclipse {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const radius: usize = 0x12CC; // int32
    pub const beams: usize = 0x12D0; // int32
    pub const hit_count: usize = 0x12D4; // int32
    pub const m_iBeamDamage: usize = 0x12D8; // int32
    pub const m_flBeamStun: usize = 0x12DC; // float32
    pub const vPosition: usize = 0x12E0; // Vector
    pub const m_bAreaTarget: usize = 0x12EC; // bool
    pub const m_bMoonWell: usize = 0x12ED; // bool
    pub const m_iTickCount: usize = 0x12F0; // int32
    pub const m_nMoonlightFXIndex: usize = 0x12F4; // ParticleIndex_t
    pub const m_HitTargets: usize = 0x12F8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_HitAllies: usize = 0x1310; // CUtlVector< CHandle< C_BaseEntity > >
    pub const beam_interval: usize = 0x1328; // float32
    pub const m_flInterval: usize = 0x132C; // float32
}

pub mod CDOTA_Modifier_AghsFort_Luna_Eclipse_IntoTheNight_Aura {
    pub const m_nMoveSpeed: usize = 0x12C8; // int32
    pub const m_nEvasion: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Luna_GlaiveShield {
    pub const m_bSpiritsExplode: usize = 0x12C8; // bool
    pub const hit_radius: usize = 0x12CC; // int32
    pub const rotate_range: usize = 0x12D0; // int32
    pub const glaive_movement_rate: usize = 0x12D4; // int32
    pub const replenish_time: usize = 0x12D8; // float32
    pub const m_flRotation: usize = 0x12DC; // float32
    pub const m_flGlaiveRadius: usize = 0x12E0; // float32
    pub const m_flNextSpawn: usize = 0x12E4; // GameTime_t
    pub const m_bPaused: usize = 0x12E8; // bool
}

pub mod CDOTA_Modifier_AghsFort_Luna_LucentBeam_Moonglow {
    pub const bonus_aspd: usize = 0x12C8; // int32
    pub const lifesteal: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Luna_LunarBlessing {
    pub const radius: usize = 0x12C8; // int32
    pub const bonus_damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Luna_LunarBlessingAura {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const night_bonus_pct: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Luna_LunarBlessingAura_MoonShield {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Luna_LunarBlessing_Active_Buff {
    pub const attack_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Luna_LunarBlessing_Leap {
    pub const leap_distance: usize = 0x12C8; // int32
    pub const leap_speed: usize = 0x12CC; // float32
    pub const leap_acceleration: usize = 0x12D0; // float32
    pub const leap_radius: usize = 0x12D4; // int32
    pub const search_range: usize = 0x12D8; // int32
    pub const damage_multiplier: usize = 0x12DC; // float32
    pub const m_flPowerLevelPct: usize = 0x12E0; // float32
    pub const m_bLaunched: usize = 0x12E4; // bool
    pub const m_vecHitEntities: usize = 0x12E8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_AghsFort_Luna_LunarBlessing_LunarRemnant {
    pub const m_nBonusDamage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Luna_LunarBlessing_MoonMark {
    pub const spell_resist_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Luna_LunarBlessing_MoonShield_Aura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Luna_LunarBlessing_Moonfright {
    pub const m_vOriginal: usize = 0x12C8; // Vector
}

pub mod CDOTA_Modifier_AghsFort_Luna_Lunar_Grace {
    pub const attack_speed: usize = 0x12C8; // int32
    pub const night_multiplier: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_AghsFort_Luna_MoonGlaive {
    pub const range: usize = 0x12C8; // int32
    pub const bounces: usize = 0x12CC; // int32
    pub const damage_reduction_percent: usize = 0x12D0; // int32
    pub const bounce_pct: usize = 0x12D4; // int32
    pub const m_InFlightAttackRecords: usize = 0x12D8; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_AghsFort_Luna_MoonGlaive_FX {
    pub const m_nMoonGlaiveFXIndex: usize = 0x12C8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_AghsFort_Luna_MoonGlaive_Knockback {
    pub const attack_speed_slow: usize = 0x12C8; // int32
    pub const move_speed_slow_pct: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Magnataur_Skewer_Bonus_Strength {
    pub const strength_gain: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Mars_Gods_Rebuke_Strength_Buff {
    pub const strength_gain: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Omniknight_Repel_Damage_Instance_Refraction {
    pub const damage_threshold: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Phoenix_FireSpiritBurn {
    pub const tick_interval: usize = 0x12C8; // float32
    pub const attackspeed_slow: usize = 0x12CC; // int32
    pub const damage_per_second: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_AghsFort_Phoenix_FireSpiritCount {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const spirit_count: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Phoenix_FlameRevenant_Attack {
    pub const radius_pct_as_length: usize = 0x12C8; // float32
    pub const width: usize = 0x12CC; // int32
    pub const linger_duration: usize = 0x12D0; // float32
    pub const m_fLength: usize = 0x12D4; // float32
    pub const m_nBeamFXIndex: usize = 0x12D8; // ParticleIndex_t
    pub const m_hBeamEnd: usize = 0x12DC; // CHandle< C_BaseEntity >
    pub const m_flLastHit: usize = 0x12E0; // GameTime_t
    pub const m_vecBurningTrees: usize = 0x12E8; // CUtlVector< C_BaseEntity* >
}

pub mod CDOTA_Modifier_AghsFort_Phoenix_FlameRevenant_Attack_TreeFire_Thinker {
    pub const m_vecTreeFires: usize = 0x12C8; // CUtlVector< CUtlPair< CHandle< C_BaseEntity >, GameTime_t > >
    pub const m_bCleanupWhenEmpty: usize = 0x12E0; // bool
}

pub mod CDOTA_Modifier_AghsFort_Phoenix_IcarusDive {
    pub const m_vSource: usize = 0x12C8; // Vector
    pub const m_vTarget: usize = 0x12D4; // Vector
    pub const m_vDirection: usize = 0x12E0; // Vector
    pub const m_angDirection: usize = 0x12EC; // QAngle
    pub const m_flCurrentTime: usize = 0x12F8; // float32
    pub const dash_length: usize = 0x12FC; // int32
    pub const dash_width: usize = 0x1300; // int32
    pub const hit_radius: usize = 0x1304; // int32
    pub const burn_duration: usize = 0x1308; // float32
    pub const dive_duration: usize = 0x130C; // float32
    pub const damage_per_second: usize = 0x1310; // int32
    pub const m_vecHitEntities: usize = 0x1318; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_bSavedAlly: usize = 0x1330; // bool
}

pub mod CDOTA_Modifier_AghsFort_Phoenix_IcarusDiveBurn {
    pub const burn_tick_interval: usize = 0x12C8; // float32
    pub const damage_per_second: usize = 0x12CC; // int32
    pub const slow_movement_speed_pct: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_AghsFort_Phoenix_SecondSun {
    pub const aura_radius: usize = 0x12C8; // int32
    pub const stun_duration: usize = 0x12CC; // float32
    pub const attacks_to_kill: usize = 0x12D0; // int32
    pub const captain_attack_value: usize = 0x12D4; // int32
    pub const boss_attack_value: usize = 0x12D8; // int32
    pub const m_nAttackCount: usize = 0x12DC; // int32
    pub const m_nRadius: usize = 0x12E0; // int32
    pub const m_fSecondSunRadiusPct: usize = 0x12E4; // float32
}

pub mod CDOTA_Modifier_AghsFort_Phoenix_Sun {
    pub const aura_radius: usize = 0x12C8; // int32
    pub const stun_duration: usize = 0x12CC; // float32
    pub const attacks_to_kill: usize = 0x12D0; // int32
    pub const captain_attack_value: usize = 0x12D4; // int32
    pub const boss_attack_value: usize = 0x12D8; // int32
    pub const m_hSecondaryTarget: usize = 0x12DC; // CHandle< C_BaseEntity >
    pub const m_bHasExpelCoreShard: usize = 0x12E0; // bool
    pub const m_nAttackCount: usize = 0x12E4; // int32
}

pub mod CDOTA_Modifier_AghsFort_Phoenix_SunRay {
    pub const hp_cost_pct: usize = 0x12C8; // float32
    pub const beam_range: usize = 0x12CC; // int32
    pub const base_damage: usize = 0x12D0; // int32
    pub const base_heal: usize = 0x12D4; // int32
    pub const tick_interval: usize = 0x12D8; // float32
    pub const forward_move_speed: usize = 0x12DC; // float32
    pub const turn_rate_initial: usize = 0x12E0; // float32
    pub const turn_rate: usize = 0x12E4; // float32
    pub const radius: usize = 0x12E8; // int32
    pub const m_flCurrentTime: usize = 0x12EC; // float32
    pub const m_flAccumulatedSelfDamage: usize = 0x12F0; // float32
    pub const m_bMovingForward: usize = 0x12F4; // bool
    pub const m_bTurningFast: usize = 0x12F5; // bool
    pub const m_flFacingTarget: usize = 0x12F8; // float32
    pub const hp_perc_heal: usize = 0x12FC; // float32
    pub const m_nBeamFXIndex: usize = 0x1300; // ParticleIndex_t
    pub const m_hVisionThinkers: usize = 0x1304; // CHandle< C_BaseEntity >[8]
    pub const m_hBeamEnd: usize = 0x1324; // CHandle< C_BaseEntity >
    pub const m_bCreatedVisionThinkers: usize = 0x1328; // bool
    pub const m_hBeamEndSound: usize = 0x132C; // CHandle< C_BaseEntity >
    pub const m_vecDispelledAllies: usize = 0x1330; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_AghsFort_Phoenix_Sun_Debuff {
    pub const damage_per_sec: usize = 0x12C8; // int32
    pub const damage_interval: usize = 0x12CC; // float32
    pub const aura_radius: usize = 0x12D0; // int32
    pub const m_fRadius: usize = 0x12D4; // float32
    pub const m_fSecondSunRadiusPct: usize = 0x12D8; // float32
    pub const m_fNextDamageTime: usize = 0x12DC; // GameTime_t
    pub const m_fLastThinkTime: usize = 0x12E0; // GameTime_t
    pub const m_bHasHorizonAbility: usize = 0x12E4; // bool
    pub const m_nHorizonPushDistance: usize = 0x12E8; // int32
    pub const m_fHorizonPushInterval: usize = 0x12EC; // float32
    pub const m_fNextHorizonPushTime: usize = 0x12F0; // GameTime_t
    pub const m_nHorizonFX: usize = 0x12F4; // ParticleIndex_t
}

pub mod CDOTA_Modifier_AghsFort_PlayerTransform {
    pub const m_nCourierItemId: usize = 0x12C8; // itemid_t
    pub const m_ModelNameOfDisguise: usize = 0x12D0; // CUtlString
    pub const m_nMoveSpeed: usize = 0x12E0; // int32
    pub const m_nDisguisedSpeed: usize = 0x12E4; // int32
    pub const m_nDisguiseEffectIndex: usize = 0x12E8; // ParticleIndex_t
    pub const m_hCourier: usize = 0x12EC; // CHandle< C_BaseEntity >
    pub const m_bIsFlyingCourier: usize = 0x12F0; // bool
    pub const s_pszDonkeyDisguise: usize = 0x12F8; // char*
    pub const invul_duration: usize = 0x1300; // float32
    pub const m_bParticlesSpawned: usize = 0x1304; // bool
}

pub mod CDOTA_Modifier_AghsFort_PoisonNova_Creature_Thinker {
    pub const m_fCurRadius: usize = 0x12C8; // float32
    pub const m_fLastThink: usize = 0x12CC; // GameTime_t
    pub const m_entitiesHit: usize = 0x12D0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const speed: usize = 0x12E8; // int32
    pub const radius: usize = 0x12EC; // int32
    pub const start_radius: usize = 0x12F0; // int32
    pub const duration: usize = 0x12F4; // float32
}

pub mod CDOTA_Modifier_AghsFort_QueenOfPain_ShadowStrike {
    pub const duration_damage: usize = 0x12C8; // int32
    pub const duration_heal: usize = 0x12CC; // int32
    pub const movement_slow: usize = 0x12D0; // int32
    pub const m_SlowInterval: usize = 0x12D8; // CountdownTimer
    pub const m_flSlowStep: usize = 0x12F0; // float32
    pub const m_flSlowStepStep: usize = 0x12F4; // float32
}

pub mod CDOTA_Modifier_AghsFort_RestorativeFlower {
    pub const radius: usize = 0x12C8; // int32
    pub const heal: usize = 0x12CC; // int32
    pub const vision_radius: usize = 0x12D0; // int32
    pub const vision_duration: usize = 0x12D4; // float32
    pub const model_scale: usize = 0x12D8; // int32
    pub const bExploding: usize = 0x12DC; // bool
    pub const stun_duration: usize = 0x12E0; // float32
    pub const m_nFXAmbient: usize = 0x12E4; // ParticleIndex_t
}

pub mod CDOTA_Modifier_AghsFort_RockGolem_Avalanche {
    pub const m_pHeroesHit: usize = 0x12C8; // CUtlVector< C_DOTA_BaseNPC* >
    pub const radius: usize = 0x12E0; // int32
    pub const total_duration: usize = 0x12E4; // float32
    pub const stun_duration: usize = 0x12E8; // float32
    pub const tick_count: usize = 0x12EC; // int32
    pub const m_damage: usize = 0x12F0; // int32
    pub const m_nTicks: usize = 0x12F4; // int32
}

pub mod CDOTA_Modifier_AghsFort_ShadowShaman_Shackles {
    pub const tick_interval: usize = 0x12C8; // float32
    pub const total_damage: usize = 0x12CC; // float32
    pub const channel_time: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_AghsFort_ShadowWaveEffectPotion {
    pub const m_iCurJumpCount: usize = 0x12C8; // int32
    pub const m_vCurTargetLoc: usize = 0x12CC; // Vector
    pub const m_hHitEntities: usize = 0x12D8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_fProcChance: usize = 0x12F0; // float32
    pub const m_nCastRange: usize = 0x12F4; // int32
    pub const m_nBounceRadius: usize = 0x12F8; // int32
    pub const m_nDamageRadius: usize = 0x12FC; // int32
    pub const m_nDamage: usize = 0x1300; // int32
    pub const m_nMaxTargets: usize = 0x1304; // int32
}

pub mod CDOTA_Modifier_AghsFort_Shadow_Demon_Shadow_Poison {
    pub const stack_damage: usize = 0x12C8; // float32
    pub const bonus_stack_damage: usize = 0x12CC; // float32
    pub const max_multiply_stacks: usize = 0x12D0; // int32
    pub const release_at_max_stacks: usize = 0x12D4; // int32
    pub const m_nFXStackIndex: usize = 0x12D8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_AghsFort_Sniper_Headshot {
    pub const proc_chance: usize = 0x12C8; // int32
    pub const slow_duration: usize = 0x12CC; // float32
    pub const damage: usize = 0x12D0; // int32
    pub const knockback_distance: usize = 0x12D4; // int32
    pub const m_fLastStunProcTime: usize = 0x12D8; // GameTime_t
    pub const m_vHeadshotRecords: usize = 0x12E0; // CUtlVector< int16 >
    pub const m_vHeadshotAssassinateRecords: usize = 0x12F8; // CUtlVector< int16 >
    pub const m_nCritPct: usize = 0x1310; // int32
}

pub mod CDOTA_Modifier_AghsFort_Sniper_Headshot_ArmorReduction {
    pub const armor_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Sniper_Headshot_Slow {
    pub const slow: usize = 0x12C8; // int32
    pub const slow_vs_considered_hero: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Sniper_Shrapnel_AttackSpeed {
    pub const value: usize = 0x12C8; // int32
    pub const value2: usize = 0x12CC; // float32
    pub const m_nAttackSpeedBonus: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_AghsFort_Sniper_Shrapnel_Slow {
    pub const m_fAccumulatedDamage: usize = 0x12C8; // float32
    pub const shrapnel_damage: usize = 0x12CC; // int32
    pub const slow_movement_speed: usize = 0x12D0; // int32
    pub const miss_chance: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_AghsFort_Sniper_Shrapnel_Thinker {
    pub const radius: usize = 0x12C8; // int32
    pub const duration: usize = 0x12CC; // float32
    pub const damage_delay: usize = 0x12D0; // float32
    pub const m_nShrapnelPct: usize = 0x12D4; // int32
    pub const m_bExploded: usize = 0x12D8; // bool
}

pub mod CDOTA_Modifier_AghsFort_Sniper_TakeAim {
    pub const bonus_attack_range: usize = 0x12C8; // int32
    pub const active_attack_range_multiplier: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Sniper_TakeAim_ArmorReduction {
    pub const value2: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Sniper_TakeAim_Bonus {
    pub const slow: usize = 0x12C8; // int32
    pub const m_nAttackSpeedPerAttack: usize = 0x12CC; // int32
    pub const m_nShrapnelPct: usize = 0x12D0; // int32
    pub const m_bNoCollision: usize = 0x12D4; // bool
    pub const m_vecArtilleryEnemies: usize = 0x12D8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_AghsFort_Sniper_TakeAim_RapidFire {
    pub const value2: usize = 0x12C8; // int32
    pub const value3: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Special_Dawnbreaker_Solar_Guardian_DeathOfAStar_Active {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_bPassive: usize = 0x12CC; // bool
    pub const m_flCooldown: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_AghsFort_Special_Gyrocopter_Homing_Missile_Ride {
    pub const grab_radius: usize = 0x12C8; // int32
    pub const m_hLoadedUnit: usize = 0x12CC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_AghsFort_Special_Gyrocopter_Homing_Missile_Ride_LoadedUnit {
    pub const hit_radius: usize = 0x12C8; // int32
    pub const timeout_duration: usize = 0x12CC; // float32
    pub const damage: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_AghsFort_Special_Magnataur_Shockwave_Damage_Reduction {
    pub const damage_reduction_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Special_Omniknight_Purification_Cooldown_Reduction {
    pub const cooldown_reduction_per_hit: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_AghsFort_Special_PhantomAssassin_CoupDeGrace_BloodyDebuff {
    pub const damage_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Special_Phoenix_FireSpirits_Artillery_Carry {
    pub const unit_position_offset: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_AghsFort_Special_Phoenix_IcarusDive_Savior_Ally {
    pub const m_hSaviorEnt: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const ally_position_offset: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_AghsFort_Special_Phoenix_IcarusDive_Savior_Invis {
    pub const duration: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_AghsFort_Special_Phoenix_SunRay_GammaRay_Slow {
    pub const move_slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Special_QueenOfPain_Blink_AttackSpeed {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Special_Snapfire_MortimerKisses_Autoattack {
    pub const fire_rate: usize = 0x12C8; // float32
    pub const radius: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Special_Snapfire_MortimerKisses_Incoming_Damage_Reduction {
    pub const damage_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Special_Templar_Assassin_PsiBlades_AutoAttack {
    pub const fire_rate: usize = 0x12C8; // float32
    pub const radius: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Special_Undying_ConsumeZombies_Buff {
    pub const bonus_move_speed_per_stack: usize = 0x12C8; // int32
    pub const bonus_attack_speed_per_stack: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Special_Ursa_Enrage_Armor {
    pub const value2: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Special_Ursa_Enrage_AttackSpeed {
    pub const value2: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Special_Ursa_Overpower_Taunt {
    pub const m_hZombieTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_bDidSetAttackTarget: usize = 0x12CC; // bool
}

pub mod CDOTA_Modifier_AghsFort_Special_Ursa_Overpower_Taunt_Armor {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Special_VoidSpirit_AetherRemnant_Bowling_ImpactSlow {
    pub const move_slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Special_VoidSpirit_AetherRemnant_Bowling_Movement {
    pub const m_nProjectileID: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Special_WitchDoctor_DeathWard_Damage_Resist {
    pub const damage_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Spectre_ActiveDispersion {
    pub const damage_reflection_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Spectre_ActiveDispersion_Thinker {
    pub const m_fDamage: usize = 0x12C8; // float32
    pub const m_fLastRadius: usize = 0x12CC; // float32
    pub const m_fCurRadius: usize = 0x12D0; // float32
    pub const m_fLastThink: usize = 0x12D4; // GameTime_t
    pub const m_bContracting: usize = 0x12D8; // bool
    pub const m_ViewerTimer: usize = 0x12E0; // CountdownTimer
    pub const m_nFXIndex: usize = 0x12F8; // ParticleIndex_t
    pub const m_EntitiesHit: usize = 0x1300; // CUtlVector< CHandle< C_BaseEntity > >
    pub const speed: usize = 0x1318; // int32
    pub const radius: usize = 0x131C; // int32
}

pub mod CDOTA_Modifier_AghsFort_StonehallGeneral_OverwhelmingOdds_Thinker {
    pub const radius: usize = 0x12C8; // int32
    pub const max_steps: usize = 0x12CC; // int32
    pub const damage_interval: usize = 0x12D0; // float32
    pub const m_nNumSteps: usize = 0x12D4; // int32
    pub const m_nRadiusStep: usize = 0x12D8; // int32
    pub const m_nFXIndex: usize = 0x12DC; // ParticleIndex_t
    pub const m_vDir: usize = 0x12E0; // Vector
}

pub mod CDOTA_Modifier_AghsFort_SwampSickness {
    pub const slow_percent: usize = 0x12C8; // int32
    pub const m_bInRiver: usize = 0x12CC; // bool
}

pub mod CDOTA_Modifier_AghsFort_TorrentEffectPotion_Thinker {
    pub const m_fProcChance: usize = 0x12C8; // float32
    pub const m_nRadius: usize = 0x12CC; // int32
    pub const m_nMovespeedBonus: usize = 0x12D0; // int32
    pub const m_fStunDuration: usize = 0x12D4; // float32
    pub const m_fSlowDuration: usize = 0x12D8; // float32
    pub const m_fDelay: usize = 0x12DC; // float32
    pub const m_nTorrentDamage: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_AghsFort_TorrentEffectPotion_Torrent {
    pub const m_nTorrentDamage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_TorrentEffectPotion_Torrent_Slow {
    pub const m_nMovespeedBonus: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Tower_BlastWave_Thinker {
    pub const damage_pct: usize = 0x12C8; // float32
    pub const m_fCurRadius: usize = 0x12CC; // float32
    pub const m_fLastThink: usize = 0x12D0; // GameTime_t
    pub const m_ViewerTimer: usize = 0x12D8; // CountdownTimer
    pub const m_nFXIndex: usize = 0x12F0; // ParticleIndex_t
    pub const m_EntitiesHit: usize = 0x12F8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const speed: usize = 0x1310; // int32
    pub const radius: usize = 0x1314; // int32
}

pub mod CDOTA_Modifier_AghsFort_TrapRoom_Hookshot {
    pub const speed: usize = 0x12D0; // int32
    pub const duration: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_AghsFort_TreantMiniboss_NaturesGuise {
    pub const m_RevealTimer: usize = 0x12C8; // CountdownTimer
    pub const m_InvisAttackTimer: usize = 0x12E0; // CountdownTimer
    pub const m_CooldownTimer: usize = 0x12F8; // CountdownTimer
}

pub mod CDOTA_Modifier_AghsFort_TreantMiniboss_NaturesGuise_Root {
    pub const m_iEntangleDamage: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_AghsFort_TreantMiniboss_NaturesGuise_Tree_Walking {
    pub const movement_bonus: usize = 0x12C8; // int32
    pub const regen_amp: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Undying_Decay_Buff {
    pub const str_stolen: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_AghsFort_Undying_Decay_BuffCounter {
    pub const m_fStrengthStolen: usize = 0x12C8; // float32
    pub const model_scale_per_strength: usize = 0x12CC; // float32
    pub const model_scale_max: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_AghsFort_Undying_Decay_Thinker {
    pub const m_iCount: usize = 0x12C8; // int32
    pub const interval: usize = 0x12CC; // float32
    pub const extra_decays: usize = 0x12D0; // int32
    pub const radius: usize = 0x12D4; // int32
    pub const decay_strength_percent: usize = 0x12D8; // int32
    pub const m_nFXIndex: usize = 0x12DC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_AghsFort_Undying_FleshGolem {
    pub const str_percentage: usize = 0x12C8; // int32
    pub const movement_bonus: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Undying_FleshGolem_PlagueAura {
    pub const slow: usize = 0x12C8; // int32
    pub const damage: usize = 0x12CC; // int32
    pub const tick_rate: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_AghsFort_Undying_FleshGolem_Slow {
    pub const damage_amp: usize = 0x12C8; // int32
    pub const slow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Undying_SoulRipWard {
    pub const ward_radius: usize = 0x12C8; // int32
    pub const explosion_damage: usize = 0x12CC; // int32
    pub const taunt_interval: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_AghsFort_Undying_SoulRip_Share_Strength {
    pub const str_gain: usize = 0x12C8; // float32
    pub const model_scale: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Undying_Tombstone_Bunker {
    pub const m_hLoadedUnit: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const heal_percent_per_tick: usize = 0x12CC; // float32
    pub const tombstone_grab_radius: usize = 0x12D0; // int32
    pub const m_nBunkerEnterFXIndex: usize = 0x12D4; // ParticleIndex_t
}

pub mod CDOTA_Modifier_AghsFort_Undying_Tombstone_HP {
    pub const tombstone_health: usize = 0x12C8; // int32
    pub const bonus_health: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Undying_Tombstone_Zombie_Aura {
    pub const radius: usize = 0x12C8; // int32
    pub const zombie_interval: usize = 0x12CC; // float32
    pub const spawn_at_parent: usize = 0x12D0; // int32
    pub const remove_on_death: usize = 0x12D4; // int32
    pub const knockback_duration: usize = 0x12D8; // float32
    pub const knockback_distance: usize = 0x12DC; // int32
    pub const knockback_height: usize = 0x12E0; // int32
    pub const spawn_distance: usize = 0x12E4; // int32
}

pub mod CDOTA_Modifier_AghsFort_Undying_Tombstone_Zombie_Deathlust {
    pub const bonus_move_speed: usize = 0x12C8; // int32
    pub const bonus_attack_speed: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Undying_Tombstone_Zombie_Deathstrike {
    pub const radius: usize = 0x12C8; // int32
    pub const health_threshold_pct: usize = 0x12CC; // float32
    pub const duration: usize = 0x12D0; // float32
    pub const m_hTombstoneSourceAbility: usize = 0x12D4; // CHandle< C_DOTABaseAbility >
    pub const m_hChaseUnit: usize = 0x12D8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_AghsFort_Undying_Tombstone_Zombie_Deathstrike_Slow {
    pub const slow: usize = 0x12C8; // int32
    pub const m_flDecrementTime: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_AghsFort_Undying_Tombstone_Zombie_Deathstrike_Slow_Counter {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Ursa_Earthshock {
    pub const m_nMissChance: usize = 0x12C8; // int32
    pub const m_bAddOverpower: usize = 0x12CC; // bool
    pub const m_flOverpowerDuration: usize = 0x12D0; // float32
    pub const m_nNumOverpowerAttacks: usize = 0x12D4; // int32
    pub const movement_slow: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Ursa_Earthshock_DeepInvis {
    pub const m_hVisibleEntity: usize = 0x12D8; // CHandle< C_BaseEntity >
    pub const m_nFXIndex: usize = 0x12DC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_AghsFort_Ursa_Enrage {
    pub const damage_reduction: usize = 0x12C8; // int32
    pub const status_resistance: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Ursa_Enrage_Fear {
    pub const m_vOriginal: usize = 0x12C8; // Vector
}

pub mod CDOTA_Modifier_AghsFort_Ursa_Enrage_Magic_Immunity {
    pub const m_nModelScale: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Ursa_Fury_Swipes {
    pub const damage_per_stack: usize = 0x12C8; // int32
    pub const bonus_reset_time: usize = 0x12CC; // float32
    pub const bonus_reset_time_roshan: usize = 0x12D0; // float32
    pub const max_swipe_stack: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_AghsFort_Ursa_Fury_Swipes_Damage_Increase {
    pub const damage_per_stack: usize = 0x12C8; // int32
    pub const m_nArmorReductionPerStack: usize = 0x12CC; // int32
    pub const m_nLifestealPerStack: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_AghsFort_Ursa_Overpower {
    pub const attack_speed_bonus_pct: usize = 0x12C8; // int32
    pub const m_nEarthshockAttacks: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_Ursa_Overpower_Evasion {
    pub const value2: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Venomancer_PoisonSting {
    pub const m_iDamage: usize = 0x12C8; // int32
    pub const movement_speed: usize = 0x12CC; // int32
    pub const m_nTicksRemaining: usize = 0x12D0; // int32
    pub const m_flAttackDamage: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_AghsFort_VoidSpiritBoss_AetherRemnantThinker {
    pub const radius: usize = 0x12C8; // int32
    pub const m_vFacingDir: usize = 0x12CC; // Vector
    pub const m_hRemnant: usize = 0x12D8; // CHandle< C_BaseEntity >
    pub const m_bTriggered: usize = 0x12DC; // bool
    pub const m_flCurThink: usize = 0x12E0; // float32
}

pub mod CDOTA_Modifier_AghsFort_VoidSpiritBoss_AetherRemnantUnit_Truesight {
    pub const m_nRadius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_VoidSpiritBoss_AetherRemnant_Pull {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const m_nPlayerID: usize = 0x12CC; // PlayerID_t
    pub const think_interval: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_AghsFort_VoidSpiritBoss_AetherRemnant_Unit {
    pub const m_vFacingDir: usize = 0x12C8; // Vector
    pub const m_hWatchPathThinkers: usize = 0x12D8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_nBeamFXIndex: usize = 0x12F0; // ParticleIndex_t
    pub const m_nViewerIDs: usize = 0x12F8; // CUtlVector< int32 >
    pub const remnant_watch_distance: usize = 0x1310; // int32
    pub const remnant_watch_radius: usize = 0x1314; // int32
    pub const duration: usize = 0x1318; // float32
    pub const watch_path_vision_radius: usize = 0x131C; // int32
    pub const impact_damage: usize = 0x1320; // int32
    pub const pull_duration: usize = 0x1324; // float32
    pub const activation_delay: usize = 0x1328; // float32
}

pub mod CDOTA_Modifier_AghsFort_VoidSpiritBoss_AetherRemnant_WatchThinker {
    pub const remnant_watch_radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_VoidSpiritBoss_AstralStep_Caster {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_VoidSpiritBoss_AstralStep_Debuff {
    pub const m_nFXStackIndex: usize = 0x12C8; // ParticleIndex_t
    pub const movement_slow_pct: usize = 0x12CC; // int32
    pub const pop_damage: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_AghsFort_VoidSpiritBoss_Dissimilate_Phase {
    pub const m_vFirstPortalPos: usize = 0x12C8; // Vector
    pub const m_vecDestinations: usize = 0x12D8; // CUtlVector< Vector >
    pub const m_vecDestinationParticles: usize = 0x12F0; // CUtlVector< ParticleIndex_t >
    pub const m_vecDestinationParticles_EnemyTeam: usize = 0x1308; // CUtlVector< ParticleIndex_t >
    pub const m_nCurrentDestinationIndex: usize = 0x1320; // int32
    pub const m_nClosestIndexPosToClick: usize = 0x1324; // int32
    pub const m_nPortalHeightOffset: usize = 0x1328; // int32
    pub const m_szAlliesPortalFX: usize = 0x1330; // char*
    pub const m_szEnemiesPortalFX: usize = 0x1338; // char*
    pub const m_nPortalRadius: usize = 0x1340; // int32
    pub const m_nPortalPadding: usize = 0x1344; // int32
    pub const m_nFinalImpactDamageRadius: usize = 0x1348; // int32
    pub const m_nFinalImpactFXRadius: usize = 0x134C; // int32
    pub const destination_fx_radius: usize = 0x1350; // int32
    pub const damage_radius: usize = 0x1354; // int32
    pub const portals_per_ring: usize = 0x1358; // int32
    pub const angle_per_ring_portal: usize = 0x135C; // int32
    pub const first_ring_distance_offset: usize = 0x1360; // int32
    pub const debuff_duration: usize = 0x1364; // float32
}

pub mod CDOTA_Modifier_AghsFort_VoidSpiritBoss_ResonantPulse_PhysicalBuff {
    pub const base_absorb_amount: usize = 0x12C8; // int32
    pub const m_nAbsorbRemaining: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_VoidSpiritBoss_ResonantPulse_Ring {
    pub const m_fLastRadius: usize = 0x12C8; // float32
    pub const m_fCurRadius: usize = 0x12CC; // float32
    pub const m_fLastThink: usize = 0x12D0; // GameTime_t
    pub const m_nFXIndex: usize = 0x12D4; // ParticleIndex_t
    pub const m_EntitiesHit: usize = 0x12D8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const speed: usize = 0x12F0; // int32
    pub const radius: usize = 0x12F4; // int32
    pub const damage: usize = 0x12F8; // int32
}

pub mod CDOTA_Modifier_AghsFort_VoidSpirit_AetherRemnantThinker {
    pub const radius: usize = 0x12C8; // int32
    pub const m_vFacingDir: usize = 0x12CC; // Vector
    pub const m_hRemnant: usize = 0x12D8; // CHandle< C_BaseEntity >
    pub const m_bTriggered: usize = 0x12DC; // bool
    pub const m_flCurThink: usize = 0x12E0; // float32
}

pub mod CDOTA_Modifier_AghsFort_VoidSpirit_AetherRemnantUnit_Truesight {
    pub const m_nRadius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_VoidSpirit_AetherRemnant_PhantomAttack_Crit {
    pub const m_nCritPercent: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_VoidSpirit_AetherRemnant_Pull {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const m_nPlayerID: usize = 0x12CC; // PlayerID_t
    pub const think_interval: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_AghsFort_VoidSpirit_AetherRemnant_Unit {
    pub const m_bActivated: usize = 0x12C8; // bool
    pub const m_vFacingDir: usize = 0x12CC; // Vector
    pub const m_vecWatchPositions: usize = 0x12D8; // CUtlVector< Vector >
    pub const m_nBeamFXIndex: usize = 0x12F0; // ParticleIndex_t
    pub const m_nViewerIDs: usize = 0x12F8; // CUtlVector< int32 >
    pub const remnant_watch_distance: usize = 0x1310; // int32
    pub const remnant_watch_radius: usize = 0x1314; // int32
    pub const duration: usize = 0x1318; // float32
    pub const watch_path_vision_radius: usize = 0x131C; // int32
    pub const impact_damage: usize = 0x1320; // int32
    pub const pull_duration: usize = 0x1324; // float32
    pub const activation_delay: usize = 0x1328; // float32
}

pub mod CDOTA_Modifier_AghsFort_VoidSpirit_AstralStep_Caster {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_VoidSpirit_AstralStep_Debuff {
    pub const m_nFXStackIndex: usize = 0x12C8; // ParticleIndex_t
    pub const m_fVoidMarkPct: usize = 0x12CC; // float32
    pub const movement_slow_pct: usize = 0x12D0; // int32
    pub const pop_damage: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_AghsFort_VoidSpirit_AstralStep_Vacuum_Motion {
    pub const m_fVacuumSpeed: usize = 0x12C8; // float32
    pub const m_fStunDuration: usize = 0x12CC; // float32
    pub const m_vPullLoc: usize = 0x12D0; // Vector
}

pub mod CDOTA_Modifier_AghsFort_VoidSpirit_Dissimilate_Phase {
    pub const m_vFirstPortalPos: usize = 0x12C8; // Vector
    pub const m_vecDestinations: usize = 0x12D8; // CUtlVector< Vector >
    pub const m_vecDestinationParticles: usize = 0x12F0; // CUtlVector< ParticleIndex_t >
    pub const m_vecDestinationParticles_EnemyTeam: usize = 0x1308; // CUtlVector< ParticleIndex_t >
    pub const m_nCurrentDestinationIndex: usize = 0x1320; // int32
    pub const m_nClosestIndexPosToClick: usize = 0x1324; // int32
    pub const m_nPortalHeightOffset: usize = 0x1328; // int32
    pub const m_szAlliesPortalFX: usize = 0x1330; // char*
    pub const m_szEnemiesPortalFX: usize = 0x1338; // char*
    pub const m_nPortalRadius: usize = 0x1340; // int32
    pub const m_nPortalPadding: usize = 0x1344; // int32
    pub const m_nFinalImpactDamageRadius: usize = 0x1348; // int32
    pub const m_nFinalImpactFXRadius: usize = 0x134C; // int32
    pub const destination_fx_radius: usize = 0x1350; // int32
    pub const damage_radius: usize = 0x1354; // int32
    pub const portals_per_ring: usize = 0x1358; // int32
    pub const angle_per_ring_portal: usize = 0x135C; // int32
    pub const first_ring_distance_offset: usize = 0x1360; // int32
    pub const debuff_duration: usize = 0x1364; // float32
    pub const damage: usize = 0x1368; // int32
}

pub mod CDOTA_Modifier_AghsFort_VoidSpirit_Dissimilate_Phase_Ally {
    pub const damage_radius: usize = 0x12C8; // int32
    pub const debuff_duration: usize = 0x12CC; // float32
    pub const destination_fx_radius: usize = 0x12D0; // int32
    pub const damage: usize = 0x12D4; // int32
    pub const m_vExitPos: usize = 0x12D8; // Vector
}

pub mod CDOTA_Modifier_AghsFort_VoidSpirit_ResonantPulse_Cadence {
    pub const m_fSpellAmp: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_AghsFort_VoidSpirit_ResonantPulse_PhysicalBuff {
    pub const base_absorb_amount: usize = 0x12C8; // int32
    pub const m_nAbsorbRemaining: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AghsFort_VoidSpirit_ResonantPulse_Ring {
    pub const m_fLastRadius: usize = 0x12C8; // float32
    pub const m_fCurRadius: usize = 0x12CC; // float32
    pub const m_fLastThink: usize = 0x12D0; // GameTime_t
    pub const m_nFXIndex: usize = 0x12D4; // ParticleIndex_t
    pub const m_EnemiesHit: usize = 0x12D8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_AlliesHit: usize = 0x12F0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const speed: usize = 0x1308; // int32
    pub const radius: usize = 0x130C; // int32
    pub const damage: usize = 0x1310; // int32
}

pub mod CDOTA_Modifier_AghsFort_VoidSpirit_ResonantPulse_SpellAmped {
    pub const m_nSpellAmp: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AghsFort_Watch_Tower {
    pub const m_nState: usize = 0x12C8; // TowerState_t
    pub const m_flYaw: usize = 0x12CC; // float32
    pub const m_nCaptureDuration: usize = 0x12D0; // int32
    pub const m_flEffectiveCaptureStartTime: usize = 0x12D4; // GameTime_t
    pub const m_nCapturingPlayerCount: usize = 0x12D8; // int32
    pub const m_flCaptureProgress: usize = 0x12DC; // float32
    pub const m_iCapturingTeam: usize = 0x12E0; // int32
    pub const m_nFxOutpostAmbient: usize = 0x12E4; // ParticleIndex_t
    pub const m_flDestroyTime: usize = 0x12E8; // GameTime_t
    pub const m_flAutoChannelCompleteTime: usize = 0x12EC; // GameTime_t
    pub const m_bAscensionLevelPicker: usize = 0x12F0; // bool
    pub const m_nEliteChallengeLevel: usize = 0x12F4; // int32
    pub const m_strNextRoomName: usize = 0x12F8; // CUtlString
    pub const m_strNextEncounterName: usize = 0x1300; // CUtlString
    pub const m_bStartedBeamFacing: usize = 0x1308; // bool
}

pub mod CDOTA_Modifier_AghsFort_Waveblaster_Leap {
    pub const leap_speed: usize = 0x12C8; // float32
    pub const leap_acceleration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Aghsfort2_DrowRanger_Marksmanship_Aura {
    pub const aura_disable_range: usize = 0x12C8; // int32
    pub const aura_range: usize = 0x12CC; // int32
    pub const bonus_damage: usize = 0x12D0; // int32
    pub const m_nBonusFactor: usize = 0x12D4; // int32
    pub const m_fLastEnableTime: usize = 0x12D8; // GameTime_t
    pub const m_nFxIndex: usize = 0x12DC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Aghsfort2_DrowRanger_Marksmanship_Aura_Bonus {
    pub const aura_agility_multiplier: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort2_DrowRanger_Multishot {
    pub const arrow_width: usize = 0x12C8; // int32
    pub const arrow_speed: usize = 0x12CC; // float32
    pub const range: usize = 0x12D0; // int32
    pub const wave_count: usize = 0x12D4; // int32
    pub const wave_delay: usize = 0x12D8; // float32
    pub const arrow_count_per_wave: usize = 0x12DC; // int32
    pub const arrow_delay: usize = 0x12E0; // float32
    pub const per_arrow_angle: usize = 0x12E4; // float32
    pub const m_fYawOffset: usize = 0x12E8; // float32
    pub const m_iWaveIndex: usize = 0x12EC; // int32
    pub const m_iArrowIndex: usize = 0x12F0; // int32
    pub const m_vTarget: usize = 0x12F4; // Vector
    pub const m_vWaveAngle: usize = 0x1300; // QAngle
    pub const m_flInterval: usize = 0x130C; // float32
    pub const m_flNextThink: usize = 0x1310; // GameTime_t
}

pub mod CDOTA_Modifier_Aghsfort2_DrowRanger_Multishot_Buff {
    pub const damage_percent: usize = 0x12C8; // int32
    pub const knockback_distance: usize = 0x12CC; // int32
    pub const knockback_duration: usize = 0x12D0; // float32
    pub const move_speed_percent: usize = 0x12D4; // int32
    pub const max_move_speed_percent: usize = 0x12D8; // int32
    pub const model_scale: usize = 0x12DC; // int32
    pub const max_model_scale: usize = 0x12E0; // int32
    pub const m_bActive: usize = 0x12E4; // bool
}

pub mod CDOTA_Modifier_Aghsfort2_DrowRanger_Multishot_SideLine {
    pub const m_iWaveIndex: usize = 0x12C8; // int32
    pub const m_fYawOffset: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Aghsfort_ArcWardenBoss_SparkWraith_Missile {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const m_nFXIndex2: usize = 0x12CC; // ParticleIndex_t
    pub const hero_damage: usize = 0x12D0; // int32
    pub const acceleration: usize = 0x12D4; // int32
    pub const min_damage: usize = 0x12D8; // int32
    pub const max_distance: usize = 0x12DC; // int32
    pub const pre_flight_time: usize = 0x12E0; // float32
    pub const stun_duration: usize = 0x12E4; // float32
    pub const m_nMissileDamage: usize = 0x12E8; // int32
    pub const m_nTeamNumber: usize = 0x12EC; // int32
    pub const speed: usize = 0x12F8; // float32
    pub const m_hAttachTarget: usize = 0x12FC; // CHandle< C_BaseEntity >
    pub const m_vStartPosition: usize = 0x1300; // Vector
    pub const m_EnemyVision: usize = 0x1310; // CountdownTimer
    pub const m_MoveTime: usize = 0x1328; // CountdownTimer
}

pub mod CDOTA_Modifier_Aghsfort_Aziyog_Underlord_Firestorm_Burn {
    pub const burn_damage: usize = 0x12C8; // float32
    pub const burn_interval: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Aziyog_Underlord_Firestorm_Thinker {
    pub const wave_damage: usize = 0x12C8; // int32
    pub const wave_count: usize = 0x12CC; // int32
    pub const radius: usize = 0x12D0; // int32
    pub const summon_count: usize = 0x12D4; // int32
    pub const wave_interval: usize = 0x12D8; // float32
    pub const burn_duration: usize = 0x12DC; // float32
    pub const first_wave_delay: usize = 0x12E0; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Aziyog_Underlord_Portal_FX {
    pub const m_nPortalAmbientFX: usize = 0x12C8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Aghsfort_Aziyog_Underlord_Portal_Warp_Channel {
    pub const m_nfxTargetTp: usize = 0x12C8; // ParticleIndex_t
    pub const m_nfxTargetTp2: usize = 0x12CC; // ParticleIndex_t
    pub const m_nfxAmbientFx: usize = 0x12D0; // ParticleIndex_t
    pub const m_nfxPortal1: usize = 0x12D4; // ParticleIndex_t
    pub const m_nfxPortal2: usize = 0x12D8; // ParticleIndex_t
    pub const animation_rate: usize = 0x12DC; // float32
    pub const stop_distance: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Bane_Enfeeble_DamageStealBuff {
    pub const damage_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Bane_Enfeeble_Effect {
    pub const damage_reduction: usize = 0x12C8; // int32
    pub const heal_reduction: usize = 0x12CC; // int32
    pub const cast_reduction: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Bane_FiendsGrip {
    pub const fiend_grip_mana_drain: usize = 0x12C8; // int32
    pub const fiend_grip_boost_lifesteal_multiplier: usize = 0x12CC; // int32
    pub const fiend_grip_damage: usize = 0x12D0; // int32
    pub const fiend_grip_spread_radius: usize = 0x12D4; // int32
    pub const fiend_grip_tick_interval: usize = 0x12D8; // float32
    pub const m_flLastSpreadTime: usize = 0x12DC; // GameTime_t
    pub const m_bThinkerSource: usize = 0x12E0; // bool
    pub const m_bRequiresChanneling: usize = 0x12E1; // bool
    pub const m_bPrimaryGrip: usize = 0x12E2; // bool
}

pub mod CDOTA_Modifier_Aghsfort_Bane_FiendsGrip_Cast_Illusion {
    pub const fiend_grip_mana_drain: usize = 0x12C8; // int32
    pub const fiend_grip_tick_interval: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Bane_Nightmare {
    pub const m_bSweetDreams: usize = 0x12C8; // bool
    pub const m_bConfusion: usize = 0x12C9; // bool
    pub const m_bSelfBreakAvailable: usize = 0x12CA; // bool
    pub const m_bNoStatusEffect: usize = 0x12CB; // bool
    pub const m_nSource: usize = 0x12CC; // int32
    pub const animation_rate: usize = 0x12D0; // float32
    pub const m_flNextNightmareDamageTime: usize = 0x12D4; // GameTime_t
    pub const m_flAccumNightmareDamage: usize = 0x12D8; // float32
    pub const movespeed_pct: usize = 0x12DC; // int32
    pub const attack_speed: usize = 0x12E0; // int32
    pub const model_scale: usize = 0x12E4; // int32
    pub const m_hConfusionAttackTarget: usize = 0x12E8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Aghsfort_Bane_Nightmare_Invulnerable {
    pub const m_nMoveSpeed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Clinkz_BurningArmy_Entrenchment {
    pub const m_fSlowPct: usize = 0x12C8; // float32
    pub const radius: usize = 0x12CC; // float32
    pub const think_interval: usize = 0x12D0; // float32
    pub const linger_duration: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Dawnbreaker_Celestial_Hammer_Movement {
    pub const m_nProjectileID: usize = 0x12C8; // int32
    pub const projectile_speed: usize = 0x12CC; // int32
    pub const travel_speed_pct: usize = 0x12D0; // int32
    pub const m_nMaxRange: usize = 0x12D4; // int32
    pub const m_vStartPoint: usize = 0x12D8; // Vector
    pub const m_vLastTrailThinkerLocation: usize = 0x12E4; // Vector
    pub const m_vEndPointShard: usize = 0x12F0; // Vector
    pub const m_nStatusFXIndex: usize = 0x12FC; // ParticleIndex_t
    pub const flare_radius: usize = 0x1300; // int32
    pub const bHasStartedBurning: usize = 0x1304; // bool
    pub const flare_debuff_duration: usize = 0x1308; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Dawnbreaker_Celestial_Hammer_Thinker {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const trail_duration: usize = 0x12CC; // float32
    pub const flare_radius: usize = 0x12D0; // int32
    pub const m_nProjectileHandle: usize = 0x12D4; // int32
    pub const m_hHavenThinker: usize = 0x12D8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Aghsfort_Dawnbreaker_Converge {
    pub const move_slow: usize = 0x12C8; // int32
    pub const burn_damage: usize = 0x12CC; // int32
    pub const burn_interval: usize = 0x12D0; // float32
    pub const m_hInflictor: usize = 0x12D4; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Aghsfort_Dawnbreaker_Converge_Thinker {
    pub const flare_radius: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
    pub const m_hInflictor: usize = 0x12D0; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Aghsfort_Dawnbreaker_Fire_Wreath_Attack_Bonus {
    pub const attack_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Dawnbreaker_Fire_Wreath_Caster {
    pub const swipe_radius: usize = 0x12C8; // int32
    pub const swipe_damage: usize = 0x12CC; // int32
    pub const smash_radius: usize = 0x12D0; // int32
    pub const smash_damage: usize = 0x12D4; // int32
    pub const m_nCurrentSpeed: usize = 0x12D8; // int32
    pub const flSwipeInterval: usize = 0x12DC; // float32
    pub const m_flNextHit: usize = 0x12E0; // GameTime_t
    pub const iCurrentAttack: usize = 0x12E4; // int32
    pub const total_attacks: usize = 0x12E8; // int32
    pub const smash_stun_duration: usize = 0x12EC; // float32
    pub const sweep_stun_duration: usize = 0x12F0; // float32
    pub const m_bHasCompletedMove: usize = 0x12F4; // bool
    pub const duration: usize = 0x12F8; // float32
    pub const m_vTargetHorizontalDirection: usize = 0x12FC; // Vector
    pub const smash_distance_from_hero: usize = 0x1308; // int32
    pub const animation_rate: usize = 0x130C; // float32
    pub const turn_rate: usize = 0x1310; // float32
    pub const m_flFacingTarget: usize = 0x1314; // float32
    pub const m_bSuncrusher: usize = 0x1318; // bool
    pub const m_flDistanceSinceStandAblazeThinker: usize = 0x131C; // float32
    pub const m_flDistancePerStandAblazeThinker: usize = 0x1320; // float32
    pub const m_hCelestialHammerForFire: usize = 0x1324; // CHandle< C_DOTABaseAbility >
}

pub mod CDOTA_Modifier_Aghsfort_Dawnbreaker_Fire_Wreath_Slow {
    pub const swipe_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Dawnbreaker_Fire_Wreath_Smash_Stun {
    pub const damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Dawnbreaker_Luminosity {
    pub const m_bAppliesToCreeps: usize = 0x12C8; // bool
    pub const attack_count: usize = 0x12CC; // int32
    pub const m_bShouldIncrement: usize = 0x12D0; // bool
}

pub mod CDOTA_Modifier_Aghsfort_Dawnbreaker_Luminosity_Attack_Buff {
    pub const m_bIsBuffedAttack: usize = 0x12C8; // bool
    pub const m_bSunSentinel: usize = 0x12C9; // bool
    pub const heal_pct: usize = 0x12CC; // int32
    pub const bonus_damage: usize = 0x12D0; // int32
    pub const heal_radius: usize = 0x12D4; // int32
    pub const heal_from_creeps: usize = 0x12D8; // int32
    pub const allied_healing_pct: usize = 0x12DC; // int32
    pub const m_nSmashDamage: usize = 0x12E0; // int32
    pub const m_nCounterOrbitalChance: usize = 0x12E4; // int32
    pub const m_vecHeroesToHeal: usize = 0x12E8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_flTotalToHeal: usize = 0x1300; // float32
    pub const m_bCounterOrbitalProcessed: usize = 0x1304; // bool
    pub const m_bIsCounterOrbitalSmash: usize = 0x1305; // bool
    pub const m_bBrightFoundryProcessed: usize = 0x1306; // bool
}

pub mod CDOTA_Modifier_Aghsfort_Dawnbreaker_Luminosity_SolarBattery {
    pub const move_speed_pct: usize = 0x12C8; // int32
    pub const attack_speed: usize = 0x12CC; // int32
    pub const m_nBonusMovementSpeedPct: usize = 0x12D0; // int32
    pub const m_nBonusAttackSpeed: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Dawnbreaker_Solar_Guardian_Disable {
    pub const m_nCasterFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const m_nStatusFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Aghsfort_Dawnbreaker_Solar_Guardian_Evasion {
    pub const miss_rate: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Dawnbreaker_Solar_Guardian_Thinker {
    pub const pulse_interval: usize = 0x12C8; // float32
    pub const iEffectRadius: usize = 0x12CC; // int32
    pub const base_damage: usize = 0x12D0; // int32
    pub const base_heal: usize = 0x12D4; // int32
    pub const scepter_heal: usize = 0x12D8; // int32
    pub const m_hFollowTarget: usize = 0x12DC; // CHandle< C_BaseEntity >
    pub const m_hInflictor: usize = 0x12E0; // CHandle< C_BaseEntity >
    pub const m_flNextPulseTime: usize = 0x12E4; // GameTime_t
    pub const m_nThinkerFXIndex: usize = 0x12E8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Aghsfort_Disruptor_Glimpse {
    pub const m_PositionIndex: usize = 0x12C8; // Vector[9]
}

pub mod CDOTA_Modifier_Aghsfort_Disruptor_Glimpse_Attack_Buff {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Disruptor_Glimpse_Thinker {
    pub const m_vStartLoc: usize = 0x12C8; // Vector
    pub const m_hEntity: usize = 0x12D4; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Aghsfort_Disruptor_Glimpse_Travel_Damage {
    pub const damage: usize = 0x12C8; // float32
    pub const max_ticks: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Disruptor_KineticField {
    pub const radius: usize = 0x12C8; // int32
    pub const m_vOriginLoc: usize = 0x12CC; // Vector
    pub const m_bCanBeAffected: usize = 0x12D8; // bool
    pub const m_bTruesight: usize = 0x12D9; // bool
    pub const intellect_bonus: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Disruptor_KineticFieldThinker {
    pub const radius: usize = 0x12C8; // int32
    pub const formation_time: usize = 0x12CC; // float32
    pub const m_bActive: usize = 0x12D0; // bool
    pub const m_nFXIndex: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Disruptor_StaticStorm {
    pub const miss_rate: usize = 0x12C8; // int32
    pub const movement_speed_reduction: usize = 0x12CC; // int32
    pub const attack_speed_reduction: usize = 0x12D0; // int32
    pub const m_bHasScepter: usize = 0x12D4; // bool
}

pub mod CDOTA_Modifier_Aghsfort_Disruptor_StaticStormThinker {
    pub const m_nCurrentPulse: usize = 0x12C8; // int32
    pub const radius: usize = 0x12CC; // int32
    pub const damage_max: usize = 0x12D0; // int32
    pub const pulses: usize = 0x12D4; // int32
    pub const duration: usize = 0x12D8; // float32
    pub const m_nFXIndex: usize = 0x12DC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Aghsfort_Disruptor_StaticStorm_Damage_Reduction {
    pub const damage_reduction_pct: usize = 0x12C8; // int32
    pub const move_slow_pct: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Disruptor_Thunder_Strike {
    pub const strike_interval: usize = 0x12C8; // float32
    pub const radius: usize = 0x12CC; // int32
    pub const strike_damage: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Disruptor_Thunder_Strike_Slow {
    pub const slow_amount: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Elemental_Wisp_Tether {
    pub const m_bInManaGained: usize = 0x12C8; // bool
    pub const m_hTarget: usize = 0x12CC; // CHandle< C_BaseEntity >
    pub const hStunnedEntities: usize = 0x12D0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const slow_duration: usize = 0x12E8; // float32
    pub const movespeed: usize = 0x12EC; // int32
    pub const self_bonus: usize = 0x12F0; // int32
    pub const m_bIsInRange: usize = 0x12F4; // bool
    pub const radius: usize = 0x12F8; // int32
    pub const latch_distance: usize = 0x12FC; // int32
    pub const m_flHealthHealed: usize = 0x1300; // float32
    pub const m_flManaHealed: usize = 0x1304; // float32
    pub const tether_heal_amp: usize = 0x1308; // float32
    pub const m_flHealMessageTime: usize = 0x130C; // GameTime_t
    pub const m_flManaMessageTime: usize = 0x1310; // GameTime_t
}

pub mod CDOTA_Modifier_Aghsfort_Elemental_Wisp_Tether_Haste {
    pub const movement_speed: usize = 0x12C8; // int32
    pub const attack_speed: usize = 0x12CC; // int32
    pub const health_regen: usize = 0x12D0; // int32
    pub const status_resist: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Elemental_Wisp_Tether_Slow {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Gyrocopter_Call_Down {
    pub const slow_duration_first: usize = 0x12C8; // int32
    pub const slow_duration_second: usize = 0x12CC; // int32
    pub const damage_first: usize = 0x12D0; // int32
    pub const damage_second: usize = 0x12D4; // int32
    pub const power: usize = 0x12D8; // float32
    pub const radius: usize = 0x12DC; // int32
    pub const slow_first: usize = 0x12E0; // float32
    pub const slow_second: usize = 0x12E4; // float32
    pub const m_bDelayed: usize = 0x12E8; // bool
    pub const m_bHalfDamage: usize = 0x12E9; // bool
    pub const m_bFirstStrike: usize = 0x12EA; // bool
    pub const m_hInflictorAbility: usize = 0x12EC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Aghsfort_Gyrocopter_Call_Down_Slow {
    pub const movespeed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Gyrocopter_Flak_Cannon {
    pub const max_attacks: usize = 0x12C8; // int32
    pub const bonus_damage: usize = 0x12CC; // int32
    pub const m_iNumAttacks: usize = 0x12D0; // int32
    pub const projectile_speed: usize = 0x12D4; // int32
    pub const radius: usize = 0x12D8; // int32
    pub const max_targets: usize = 0x12DC; // int32
    pub const m_bFirstStrike: usize = 0x12E0; // bool
    pub const m_bShouldFireMissiles: usize = 0x12E1; // bool
}

pub mod CDOTA_Modifier_Aghsfort_Gyrocopter_Flak_Cannon_Scepter {
    pub const fire_rate: usize = 0x12C8; // float32
    pub const scepter_radius: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Gyrocopter_Homing_Missile {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const m_nFXIndex2: usize = 0x12CC; // ParticleIndex_t
    pub const acceleration: usize = 0x12D0; // int32
    pub const pre_flight_time: usize = 0x12D4; // float32
    pub const stun_duration: usize = 0x12D8; // float32
    pub const m_nMissileDamage: usize = 0x12DC; // int32
    pub const damage: usize = 0x12E0; // int32
    pub const m_nTeamNumber: usize = 0x12E4; // int32
    pub const speed: usize = 0x12F0; // float32
    pub const m_flFacingTarget: usize = 0x12F4; // float32
    pub const movement_turn_rate: usize = 0x12F8; // int32
    pub const m_nPower: usize = 0x12FC; // int32
    pub const m_hAttachTarget: usize = 0x1300; // CHandle< C_BaseEntity >
    pub const m_MoveTime: usize = 0x1308; // CountdownTimer
}

pub mod CDOTA_Modifier_Aghsfort_Gyrocopter_Rocket_Barrage {
    pub const rocket_damage: usize = 0x12C8; // int32
    pub const m_nPower: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Kunkka_NoQuarter {
    pub const m_nSpeedBonusPercentage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Lina_DragonSlave_Ignite {
    pub const burn_interval: usize = 0x12C8; // float32
    pub const burn_damage: usize = 0x12CC; // float32
    pub const mana_restore: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Lina_DragonSlave_Movement {
    pub const m_flPushDistance: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Lina_FierySoul {
    pub const fiery_soul_attack_speed_bonus: usize = 0x12C8; // int32
    pub const fiery_soul_move_speed_bonus: usize = 0x12CC; // float32
    pub const fiery_soul_max_stacks: usize = 0x12D0; // int32
    pub const fiery_soul_stack_duration: usize = 0x12D4; // float32
    pub const m_nFXIndex: usize = 0x12D8; // ParticleIndex_t
    pub const m_flFierySoulDieTime: usize = 0x12DC; // GameTime_t
}

pub mod CDOTA_Modifier_Aghsfort_Lina_FierySoul_Active {
    pub const attack_range_per_stack: usize = 0x12C8; // int32
    pub const m_bShouldReset: usize = 0x12CC; // bool
    pub const m_bIsActive: usize = 0x12CD; // bool
    pub const fiery_soul_attack_speed_bonus: usize = 0x12D0; // int32
    pub const fiery_soul_move_speed_bonus: usize = 0x12D4; // float32
    pub const m_nFXIndex: usize = 0x12D8; // ParticleIndex_t
    pub const m_InFlightAttackRecords: usize = 0x12E0; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Aghsfort_Lina_LagunaBlade_Line {
    pub const vStart: usize = 0x12C8; // Vector
    pub const vEnd: usize = 0x12D4; // Vector
    pub const damage_delay: usize = 0x12E0; // float32
    pub const m_hMainTarget: usize = 0x12E4; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Aghsfort_Lina_LightStrikeArray_Thinker {
    pub const m_InFlightAttackRecords: usize = 0x12C8; // CUtlVector< int16 >
    pub const m_iDamage: usize = 0x12E0; // int32
    pub const m_flStunDuration: usize = 0x12E4; // float32
    pub const light_strike_array_damage: usize = 0x12E8; // int32
    pub const light_strike_array_stun_duration: usize = 0x12EC; // float32
    pub const light_strike_array_delay: usize = 0x12F0; // float32
    pub const m_nLSARadius: usize = 0x12F4; // int32
    pub const m_nCount: usize = 0x12F8; // int32
    pub const max_count: usize = 0x12FC; // int32
    pub const iPower: usize = 0x1300; // int32
    pub const pulse_interval: usize = 0x1304; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Magnataur_Empower {
    pub const bonus_damage_pct: usize = 0x12C8; // int32
    pub const cleave_damage_pct: usize = 0x12CC; // float32
    pub const cleave_starting_width: usize = 0x12D0; // int32
    pub const cleave_ending_width: usize = 0x12D4; // int32
    pub const cleave_distance: usize = 0x12D8; // int32
    pub const splash_damage_pct: usize = 0x12DC; // float32
    pub const splash_radius: usize = 0x12E0; // int32
    pub const self_multiplier: usize = 0x12E4; // float32
    pub const lifesteal_pct: usize = 0x12E8; // int32
    pub const m_fLastShockwaveProcTime: usize = 0x12EC; // GameTime_t
}

pub mod CDOTA_Modifier_Aghsfort_Magnataur_ReversePolarity_Steroid {
    pub const move_speed_percent: usize = 0x12C8; // int32
    pub const attack_speed: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Magnataur_Shockwave {
    pub const movement_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Magnataur_Shockwave_Pull {
    pub const m_vDirection: usize = 0x12C8; // Vector
    pub const m_flEndTime: usize = 0x12D4; // float32
    pub const m_flCurTime: usize = 0x12D8; // float32
    pub const pull_duration: usize = 0x12DC; // float32
    pub const effective_distance: usize = 0x12E0; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Magnataur_Skewer_Impact {
    pub const slow_duration: usize = 0x12C8; // float32
    pub const skewer_damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Magnataur_Skewer_Movement {
    pub const m_nProjectileID: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Aghsfort_Magnataur_Skewer_Slow {
    pub const slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Mars_ArenaOfBlood {
    pub const radius: usize = 0x12C8; // int32
    pub const width: usize = 0x12CC; // int32
    pub const spear_damage: usize = 0x12D0; // int32
    pub const spear_distance_from_wall: usize = 0x12D4; // int32
    pub const spear_attack_interval: usize = 0x12D8; // float32
    pub const warrior_fade_min_dist: usize = 0x12F8; // float32
    pub const warrior_fade_max_dist: usize = 0x12FC; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Mars_ArenaOfBlood_AnimationAura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Mars_ArenaOfBlood_Buff {
    pub const health_regen: usize = 0x12C8; // int32
    pub const attack_speed: usize = 0x12CC; // int32
    pub const bonus_damage: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Mars_ArenaOfBlood_BuffAura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Mars_ArenaOfBlood_Fear {
    pub const m_vOriginal: usize = 0x12C8; // Vector
}

pub mod CDOTA_Modifier_Aghsfort_Mars_ArenaOfBlood_Leash {
    pub const radius: usize = 0x12C8; // int32
    pub const width: usize = 0x12CC; // float32
    pub const m_vOriginLoc: usize = 0x12D0; // Vector
}

pub mod CDOTA_Modifier_Aghsfort_Mars_ArenaOfBlood_Thinker {
    pub const radius: usize = 0x12C8; // float32
    pub const formation_time: usize = 0x12CC; // float32
    pub const m_flInitialZ: usize = 0x12D0; // float32
    pub const m_flFinalZ: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Mars_Bulwark {
    pub const physical_damage_reduction: usize = 0x12C8; // float32
    pub const physical_damage_reduction_side: usize = 0x12CC; // float32
    pub const forward_angle: usize = 0x12D0; // float32
    pub const side_angle: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Mars_Bulwark_Active {
    pub const redirect_chance: usize = 0x12C8; // int32
    pub const redirect_range: usize = 0x12CC; // int32
    pub const forward_angle: usize = 0x12D0; // float32
    pub const side_angle: usize = 0x12D4; // float32
    pub const redirect_speed_penatly: usize = 0x12D8; // int32
    pub const m_nPoseParameterWE: usize = 0x12DC; // int32
    pub const m_nPoseParameterNS: usize = 0x12E0; // int32
    pub const m_flLastPoseX: usize = 0x12E4; // float32
    pub const m_flLastPoseY: usize = 0x12E8; // float32
    pub const m_nLastMaxDirection: usize = 0x12EC; // int32
    pub const m_vLastOrigin: usize = 0x12F0; // Vector
    pub const m_flLastGameTime: usize = 0x12FC; // GameTime_t
}

pub mod CDOTA_Modifier_Aghsfort_Mars_Bulwark_Soldier_Bonus {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Mars_GodsRebuke_Crit {
    pub const crit_mult: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Mars_Gods_Rebuke {
    pub const knockback_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Minor_Stats_Upgrade {
    pub const bonus_all_stats: usize = 0x12C8; // int32
    pub const bonus_primary_stat: usize = 0x12CC; // int32
    pub const bonus_movespeed: usize = 0x12D0; // int32
    pub const bonus_hp_regen: usize = 0x12D4; // int32
    pub const bonus_armor: usize = 0x12D8; // int32
    pub const bonus_attack_damage: usize = 0x12DC; // int32
    pub const bonus_cast_range: usize = 0x12E0; // int32
    pub const bonus_spell_amp: usize = 0x12E4; // int32
    pub const bonus_health: usize = 0x12E8; // int32
    pub const bonus_mana: usize = 0x12EC; // int32
    pub const bonus_attack_speed: usize = 0x12F0; // int32
    pub const bonus_heal_amp: usize = 0x12F4; // int32
    pub const bonus_mana_regen: usize = 0x12F8; // int32
    pub const bonus_evasion: usize = 0x12FC; // int32
    pub const bonus_magic_resist: usize = 0x1300; // int32
    pub const bonus_lifesteal: usize = 0x1304; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Omniknight_Degen_Aura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Omniknight_Degen_Aura_Effect {
    pub const move_speed_bonus: usize = 0x12C8; // int32
    pub const attack_speed_bonus: usize = 0x12CC; // int32
    pub const damage_interval: usize = 0x12D0; // float32
    pub const attack_damage_pct: usize = 0x12D4; // int32
    pub const death_heal: usize = 0x12D8; // int32
    pub const bIsAllied: usize = 0x12DC; // bool
}

pub mod CDOTA_Modifier_Aghsfort_Omniknight_GuardianAngel {
    pub const hp_regen: usize = 0x12C8; // int32
    pub const magic_resistance: usize = 0x12CC; // int32
    pub const purification_trigger_chance: usize = 0x12D0; // int32
    pub const purification_trigger_cooldown: usize = 0x12D4; // float32
    pub const m_fLastProc: usize = 0x12D8; // GameTime_t
}

pub mod CDOTA_Modifier_Aghsfort_Omniknight_Repel {
    pub const damage_reduction: usize = 0x12C8; // int32
    pub const hp_regen: usize = 0x12CC; // float32
    pub const bonus_str: usize = 0x12D0; // int32
    pub const damage_increase_outgoing_pct: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Aghsfort_PhantomAssassin_Blur {
    pub const bonus_evasion: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_PhantomAssassin_BlurActive {
    pub const radius: usize = 0x12D8; // int32
    pub const fade_duration: usize = 0x12DC; // float32
    pub const m_bDestroyNext: usize = 0x12E0; // bool
}

pub mod CDOTA_Modifier_Aghsfort_PhantomAssassin_CoupdeGrace {
    pub const crit_bonus: usize = 0x12C8; // int32
    pub const crit_chance: usize = 0x12CC; // int32
    pub const crit_active: usize = 0x12D0; // bool
}

pub mod CDOTA_Modifier_Aghsfort_PhantomAssassin_PhantomStrike {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
    pub const bonus_evasion: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_PhantomAssassin_StiflingDagger {
    pub const move_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_PhantomAssassin_StiflingDagger_Caster {
    pub const base_damage: usize = 0x12C8; // int32
    pub const attack_factor: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Phoenix_FlameRevenant_Attack_Damage {
    pub const m_fDamagePerSecond: usize = 0x12C8; // float32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
    pub const m_nActiveFXIndex: usize = 0x12D0; // ParticleIndex_t
    pub const m_nActiveStatusFXIndex: usize = 0x12D4; // ParticleIndex_t
    pub const flame_revenant_damage_pct: usize = 0x12D8; // float32
    pub const think_interval: usize = 0x12DC; // float32
    pub const move_slow_pct: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_Aghsfort_PrimalBeast_Boss_Inhibition {
    pub const debuff_duration: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Aghsfort_PrimalBeast_Boss_Inhibition_Debuff {
    pub const heal_suppression_pct: usize = 0x12C8; // int32
    pub const armor_reduction_pct: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_PrimalBeast_Boss_NoCC {
    pub const bCanbeMotionControlled: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_Aghsfort_PrimalBeast_Boss_Onslaught_Movement {
    pub const charge_speed: usize = 0x12C8; // float32
    pub const knockback_radius: usize = 0x12CC; // int32
    pub const knockback_distance: usize = 0x12D0; // int32
    pub const knockback_damage: usize = 0x12D4; // int32
    pub const forward_offset: usize = 0x12D8; // int32
    pub const m_hTarget: usize = 0x12DC; // CHandle< C_BaseEntity >
    pub const m_flFacingTarget: usize = 0x12E0; // float32
    pub const m_nFXIndex: usize = 0x12E4; // ParticleIndex_t
    pub const m_hHitEntities: usize = 0x12E8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Aghsfort_PrimalBeast_Boss_Onslaught_Windup {
    pub const charge_duration: usize = 0x12C8; // float32
    pub const max_charge_time: usize = 0x12CC; // float32
    pub const turn_rate: usize = 0x12D0; // float32
    pub const base_power: usize = 0x12D4; // float32
    pub const charge_speed: usize = 0x12D8; // int32
    pub const animation_rate: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Aghsfort_PrimalBeast_Boss_PrimalRoar_Push {
    pub const m_vVelocity: usize = 0x12C8; // Vector
}

pub mod CDOTA_Modifier_Aghsfort_PrimalBeast_Boss_PrimalRoar_Slow {
    pub const slow_movement_speed_pct: usize = 0x12C8; // int32
    pub const slow_attack_speed_pct: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_PrimalBeast_Boss_Pummel {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const m_flOffset: usize = 0x12CC; // float32
    pub const m_vStartAngles: usize = 0x12D0; // QAngle
    pub const animation_rate: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Aghsfort_PrimalBeast_Boss_Pummel_Damage_Tracking {
    pub const damage_limit: usize = 0x12C8; // int32
    pub const m_nFxIndex: usize = 0x12CC; // ParticleIndex_t
    pub const m_fTotalDamage: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Aghsfort_PrimalBeast_Boss_Pummel_Ripple_Thinker {
    pub const ripple_effect_width: usize = 0x12C8; // int32
    pub const ripple_damage: usize = 0x12CC; // int32
    pub const health_damage: usize = 0x12D0; // int32
    pub const ripple_count: usize = 0x12D4; // int32
    pub const total_ripple_circles: usize = 0x12D8; // int32
    pub const radius: usize = 0x12DC; // float32
    pub const ripple_slow_duration: usize = 0x12E0; // float32
    pub const ripple_knockback_duration: usize = 0x12E4; // float32
    pub const m_hGrabbedEnemy: usize = 0x12E8; // CHandle< C_BaseEntity >
    pub const m_nPreviewFXIndex: usize = 0x12EC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Aghsfort_PrimalBeast_Boss_Pummel_Self {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_DOTA_BaseNPC >
    pub const m_vPos: usize = 0x12CC; // Vector
    pub const splash_radius: usize = 0x12D8; // int32
    pub const interval: usize = 0x12DC; // float32
    pub const ministun: usize = 0x12E0; // float32
    pub const m_nCurrentHitCount: usize = 0x12E4; // int32
    pub const total_ripple_circles: usize = 0x12E8; // int32
    pub const ripple_step_distance: usize = 0x12EC; // int32
    pub const ripple_step_distance_offset: usize = 0x12F0; // int32
    pub const health_damage: usize = 0x12F4; // float32
    pub const ripple_duration: usize = 0x12F8; // float32
    pub const ripple_damage: usize = 0x12FC; // int32
    pub const m_vLastRippleOffsets: usize = 0x1300; // CUtlVector< int32 >
    pub const self_animation_rate: usize = 0x1318; // float32
}

pub mod CDOTA_Modifier_Aghsfort_PrimalBeast_Boss_Rock_Crush {
    pub const m_hRock: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_qRotation: usize = 0x12CC; // QAngle
    pub const m_flOldPitch: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Aghsfort_PrimalBeast_Boss_Rock_HP {
    pub const rock_health: usize = 0x12C8; // int32
    pub const rock_visualzdelta: usize = 0x12CC; // int32
    pub const m_bIsCrushing: usize = 0x12D0; // bool
    pub const summon_count: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Aghsfort_PrimalBeast_Boss_Rock_Thinker {
    pub const effect_radius: usize = 0x12C8; // int32
    pub const rock_duration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Aghsfort_PrimalBeast_Boss_TectonicShift {
    pub const projectile_speed: usize = 0x12C8; // int32
    pub const projectile_speed_increment: usize = 0x12CC; // int32
    pub const projectile_distance: usize = 0x12D0; // int32
    pub const projectile_width: usize = 0x12D4; // int32
    pub const projectiles_per_interval: usize = 0x12D8; // int32
    pub const projectiles_per_interval_increment: usize = 0x12DC; // int32
    pub const initial_spread_angle: usize = 0x12E0; // int32
    pub const angle_offset: usize = 0x12E4; // int32
    pub const iCurrentInterval: usize = 0x12E8; // int32
    pub const splinter_angle: usize = 0x12EC; // int32
    pub const m_LeftHandSmashTimer: usize = 0x12F0; // CountdownTimer
    pub const m_RightHandSmashTimer: usize = 0x1308; // CountdownTimer
    pub const m_AttackIntervalTimer: usize = 0x1320; // CountdownTimer
    pub const initial_interval: usize = 0x1338; // float32
    pub const interval_reduction: usize = 0x133C; // float32
    pub const min_interval: usize = 0x1340; // float32
    pub const arm_distance_forward: usize = 0x1344; // int32
    pub const arm_distance_side: usize = 0x1348; // int32
    pub const splash_radius: usize = 0x134C; // int32
    pub const max_split_amount: usize = 0x1350; // int32
    pub const m_nCurrentCastCount: usize = 0x1354; // int32
    pub const max_projectiles: usize = 0x1358; // int32
}

pub mod CDOTA_Modifier_Aghsfort_PrimalBeast_Boss_TectonicShift_Damage {
    pub const tick_damage: usize = 0x12C8; // float32
    pub const damage_ticks: usize = 0x12CC; // int32
    pub const m_nTickAmount: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Aghsfort_PrimalBeast_Boss_Vault {
    pub const vault_speed: usize = 0x12C8; // float32
    pub const vault_acceleration: usize = 0x12CC; // float32
    pub const vault_radius: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Aghsfort_PrimalBeast_Pummel_DroppedItem {
    pub const m_nPreviewFXIndex: usize = 0x12C8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Aghsfort_Primalbeast_Boss_HeavySteps {
    pub const vLastPos: usize = 0x12C8; // Vector
    pub const flCurrentDistance: usize = 0x12D4; // float32
    pub const step_distance: usize = 0x12D8; // int32
    pub const effect_radius: usize = 0x12DC; // int32
    pub const base_damage: usize = 0x12E0; // int32
    pub const m_hTarget: usize = 0x12E4; // CHandle< C_BaseEntity >
    pub const ctLastCommandTimer: usize = 0x12E8; // CountdownTimer
    pub const m_bMoveCommandIssued: usize = 0x1300; // bool
}

pub mod CDOTA_Modifier_Aghsfort_Primalbeast_Boss_RoarAttack_Start {
    pub const rock_drop_intervals: usize = 0x12C8; // int32
    pub const rock_drop_per_interval: usize = 0x12CC; // int32
    pub const rock_drop_distance: usize = 0x12D0; // int32
    pub const m_nCurrentInterval: usize = 0x12D4; // int32
    pub const rock_drop_rotation_angle: usize = 0x12D8; // int32
    pub const m_hTarget: usize = 0x12DC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Aghsfort_Pugna_Grandmaster_NetherWard {
    pub const radius: usize = 0x12C8; // int32
    pub const mana_multiplier: usize = 0x12CC; // float32
    pub const mana_drained_per_attack: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Reward_ArmorAura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Reward_ArmorAura_Bonus {
    pub const bonus_armor: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Reward_CritAura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Reward_CritAura_Bonus {
    pub const crit_chance: usize = 0x12C8; // int32
    pub const crit_multiplier: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Reward_HPAura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Reward_HPAura_Bonus {
    pub const bonus_hp: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Reward_MagicResistAura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Reward_MagicResistAura_Bonus {
    pub const bonus_magical_armor: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Sand_King_BurrowStrike {
    pub const m_vPopupLocation: usize = 0x12C8; // Vector
    pub const m_bTeleported: usize = 0x12D4; // bool
}

pub mod CDOTA_Modifier_Aghsfort_Sand_King_BurrowStrike_Channel {
    pub const m_vTargetPos: usize = 0x12C8; // Vector
    pub const m_nCrosshairFX: usize = 0x12D4; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Aghsfort_Sand_King_BurrowStrike_Reverse_Tracking {
    pub const m_vStartLocation: usize = 0x12C8; // Vector
}

pub mod CDOTA_Modifier_Aghsfort_Sand_King_Burrowstrike_Transport {
    pub const m_nProjectileID: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Sand_King_CausticFinale {
    pub const caustic_finale_duration: usize = 0x12C8; // float32
    pub const m_InFlightAttackRecords: usize = 0x12D0; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Aghsfort_Sand_King_CausticFinaleOrb {
    pub const caustic_finale_radius: usize = 0x12C8; // int32
    pub const caustic_finale_damage_death: usize = 0x12CC; // int32
    pub const caustic_finale_damage_dot: usize = 0x12D0; // int32
    pub const caustic_finale_slow: usize = 0x12D4; // int32
    pub const caustic_finale_dot_tick: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Sand_King_Epicenter {
    pub const epicenter_pulses: usize = 0x12C8; // int32
    pub const epicenter_radius: usize = 0x12CC; // int32[16]
    pub const epicenter_damage: usize = 0x130C; // int32
    pub const m_iMaxPulses: usize = 0x1310; // int32
    pub const m_iPulseCount: usize = 0x1314; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Sand_King_Epicenter_Slow {
    pub const epicenter_slow: usize = 0x12C8; // int32
    pub const epicenter_slow_as: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Sand_King_SandStorm {
    pub const sand_storm_radius: usize = 0x12C8; // float32
    pub const sand_storm_damage: usize = 0x12CC; // int32
    pub const damage_tick_rate: usize = 0x12D0; // float32
    pub const m_flLastDamageTime: usize = 0x12D4; // GameTime_t
    pub const m_nSandStormParticleIndex1: usize = 0x12D8; // ParticleIndex_t
    pub const m_nSandStormParticleIndex2: usize = 0x12DC; // ParticleIndex_t
    pub const m_hThinker: usize = 0x12E8; // CHandle< C_BaseEntity >
    pub const m_hBurrowstrikeFieldThinker: usize = 0x12EC; // CHandle< C_BaseEntity >
    pub const m_bIsPrimarySandstorm: usize = 0x12F0; // bool
    pub const vecSpawnPos: usize = 0x12F4; // Vector
}

pub mod CDOTA_Modifier_Aghsfort_Sand_King_SandStorm_Slow {
    pub const blind_slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Sand_King_SandStorm_Slow_Aura_Thinker {
    pub const sand_storm_radius: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Sand_King_Sandstorm_Burrowstrike_Field_Thinker {
    pub const radius: usize = 0x12C8; // float32
    pub const explosion_interval: usize = 0x12CC; // float32
    pub const explosion_radius: usize = 0x12D0; // int32
    pub const explosion_min_dist: usize = 0x12D4; // int32
    pub const m_fLastTick: usize = 0x12D8; // float32
    pub const m_fTimeAccumulator: usize = 0x12DC; // float32
    pub const m_iExplosionCount: usize = 0x12E0; // int32
    pub const m_iExplosionTotalCount: usize = 0x12E4; // int32
    pub const m_iExplosionQuadrant: usize = 0x12E8; // int32
    pub const m_iExplosionDistance: usize = 0x12EC; // int32
    pub const m_nFXIndex: usize = 0x12F0; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Aghsfort_Slark_DarkPact_Pulses {
    pub const radius: usize = 0x12C8; // int32
    pub const total_damage: usize = 0x12CC; // float32
    pub const total_pulses: usize = 0x12D0; // int32
    pub const heal_pct: usize = 0x12D4; // int32
    pub const pulse_interval: usize = 0x12D8; // float32
    pub const bIsBaseAbility: usize = 0x12DC; // bool
}

pub mod CDOTA_Modifier_Aghsfort_Slark_EssenceShift_Active {
    pub const agi_gain: usize = 0x12C8; // int32
    pub const iPrimaryAttribute: usize = 0x12CC; // int32
    pub const model_scale: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Slark_EssenceShift_Counter {
    pub const max_stacks: usize = 0x12C8; // int32
    pub const passive_agility: usize = 0x12CC; // int32
    pub const flLastProcTime: usize = 0x12D0; // GameTime_t
    pub const debuff_duration: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Slark_Essence_Shift_Debuff {
    pub const reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Slark_Pounce {
    pub const hitEntities: usize = 0x12C8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const pounce_distance: usize = 0x12E0; // int32
    pub const pounce_distance_default: usize = 0x12E4; // int32
    pub const pounce_speed: usize = 0x12E8; // float32
    pub const pounce_acceleration: usize = 0x12EC; // float32
    pub const pounce_radius: usize = 0x12F0; // int32
    pub const pounce_damage: usize = 0x12F4; // int32
    pub const leash_duration: usize = 0x12F8; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Slark_Pounce_Leash {
    pub const leash_radius: usize = 0x12C8; // int32
    pub const vLeashLocation: usize = 0x12CC; // Vector
    pub const m_bEscaped: usize = 0x12D8; // bool
    pub const pounce_damage_multiplier: usize = 0x12DC; // float32
    pub const pounce_damage: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Slark_ShadowDance {
    pub const m_hVisibleEntity: usize = 0x12D8; // CHandle< C_BaseEntity >
    pub const m_nFXIndex: usize = 0x12DC; // ParticleIndex_t
    pub const flLastPulseTime: usize = 0x12E0; // GameTime_t
}

pub mod CDOTA_Modifier_Aghsfort_Slark_ShadowDance_Aura {
    pub const scepter_aoe: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Slark_ShadowDance_PassiveRegen {
    pub const bonus_movement_speed: usize = 0x12C8; // int32
    pub const bonus_regen_pct: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Slark_ShadowDance_Visual {
    pub const m_bScepter: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_Aghsfort_Snapfire_FiresnapCookie_PreHop {
    pub const jump_duration: usize = 0x12C8; // float32
    pub const m_bMovesHorizontal: usize = 0x12CC; // bool
    pub const m_bSecondaryCookie: usize = 0x12CD; // bool
}

pub mod CDOTA_Modifier_Aghsfort_Snapfire_GobbleUp_BellyHasUnit {
    pub const m_hGobbledUnit: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const max_time_in_belly: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Snapfire_LilShredder_Attack {
    pub const damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Snapfire_LilShredder_Buff {
    pub const m_InFlightAttackRecords: usize = 0x12D0; // CUtlVector< int16 >
    pub const m_nIsActive: usize = 0x12E8; // int32
    pub const m_bHasAttackTalent: usize = 0x12EC; // bool
    pub const attack_range_bonus: usize = 0x12F0; // int32
    pub const buffed_attacks: usize = 0x12F4; // int32
    pub const base_attack_time: usize = 0x12F8; // float32
    pub const attack_speed_bonus: usize = 0x12FC; // int32
    pub const armor_duration: usize = 0x1300; // float32
    pub const damage: usize = 0x1304; // int32
    pub const bounce_range: usize = 0x1308; // int32
    pub const bounces: usize = 0x130C; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Snapfire_LilShredder_Debuff {
    pub const armor_reduction_per_attack: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Snapfire_Magma_Burn_Slow {
    pub const m_nMoveSlowPct: usize = 0x12C8; // int32
    pub const m_fBurnDamagePerTick: usize = 0x12CC; // float32
    pub const move_slow_pct: usize = 0x12D0; // int32
    pub const burn_interval: usize = 0x12D4; // float32
    pub const burn_damage: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Snapfire_Magma_Thinker {
    pub const burn_interval: usize = 0x12C8; // float32
    pub const impact_radius: usize = 0x12CC; // int32
    pub const move_slow_pct: usize = 0x12D0; // int32
    pub const burn_linger_duration: usize = 0x12D4; // float32
    pub const m_nPathEffectIndex: usize = 0x12D8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Aghsfort_Snapfire_MortimerKisses {
    pub const m_fIntervalPerRocket: usize = 0x12C8; // float32
    pub const m_flFacingTarget: usize = 0x12CC; // float32
    pub const m_nBeamFXIndex: usize = 0x12D0; // ParticleIndex_t
    pub const m_hBeamEnd: usize = 0x12D4; // CHandle< C_BaseEntity >
    pub const m_flCurDistance: usize = 0x12D8; // float32
    pub const m_vAimTarget: usize = 0x12DC; // Vector
    pub const m_fLastTurnAmount: usize = 0x12E8; // float32
    pub const m_nProjectilesLaunched: usize = 0x12EC; // int32
    pub const m_bDestroyOnNextThink: usize = 0x12F0; // bool
    pub const m_nProjectilesToLaunch: usize = 0x12F4; // int32
    pub const m_bHasProjectileTalent: usize = 0x12F8; // bool
    pub const projectile_count: usize = 0x12FC; // int32
    pub const projectile_vision: usize = 0x1300; // int32
    pub const turn_rate: usize = 0x1304; // float32
    pub const min_range: usize = 0x1308; // int32
    pub const delay_after_last_projectile: usize = 0x130C; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Snapfire_Scatterblast_Slow {
    pub const m_bIsPointBlank: usize = 0x12C8; // bool
    pub const movement_slow_pct: usize = 0x12CC; // int32
    pub const attack_slow_pct: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Snapfire_SpitCreep_ArcingUnit {
    pub const min_range: usize = 0x12C8; // int32
    pub const min_lob_travel_time: usize = 0x12CC; // float32
    pub const max_lob_travel_time: usize = 0x12D0; // float32
    pub const impact_radius: usize = 0x12D4; // int32
    pub const projectile_vision: usize = 0x12D8; // int32
    pub const stun_duration: usize = 0x12DC; // float32
    pub const impact_damage: usize = 0x12E0; // int32
    pub const min_height_above_lowest: usize = 0x12E4; // float32
    pub const min_height_above_highest: usize = 0x12E8; // float32
    pub const min_acceleration: usize = 0x12EC; // float32
    pub const max_acceleration: usize = 0x12F0; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Sniper_Concussive_Grenade {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Disruptor_ThunderStrike_Int_Increase {
    pub const int_increase_amount: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Disruptor_Thunder_Strike_On_Attack {
    pub const strike_chance: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Gyrocopter_Call_Down_Attack_Buff {
    pub const movespeed: usize = 0x12C8; // int32
    pub const attackspeed: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Gyrocopter_Flak_Cannon_Fire_Trail_Burn {
    pub const flDamage: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Gyrocopter_Flak_Cannon_Fire_Trail_Thinker {
    pub const m_vPathDir: usize = 0x12C8; // Vector
    pub const path_radius: usize = 0x12D4; // int32
    pub const linger_duration: usize = 0x12D8; // float32
    pub const flDamage: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Gyrocopter_RocketBarrage_On_Attack {
    pub const strike_chance: usize = 0x12C8; // int32
    pub const duration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Lina_Fiery_Soul_Splitshot {
    pub const damage_modifier: usize = 0x12C8; // int32
    pub const split_shot_bonus_range: usize = 0x12CC; // int32
    pub const arrow_count: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Lina_Laguna_Blade_Bounce {
    pub const bounces_remaining: usize = 0x12C8; // int32
    pub const m_hHitEntities: usize = 0x12D0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Aghsfort_Special_Lina_Laguna_Blade_Channel {
    pub const stack_limit: usize = 0x12C8; // int32
    pub const stack_interval: usize = 0x12CC; // float32
    pub const m_hEntitiesToHit: usize = 0x12D0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_nFxIndexB: usize = 0x12E8; // ParticleIndex_t
    pub const m_nFXIndices: usize = 0x12F0; // CUtlVector< ParticleIndex_t >
    pub const bounces_remaining: usize = 0x1308; // int32
    pub const iCur_stack: usize = 0x130C; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Lina_Light_Strike_Array_Attacks_Bonus {
    pub const attack_damage: usize = 0x12C8; // float32
    pub const damage_penalty: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Lina_Light_Strike_Array_Vacuum {
    pub const pull_speed: usize = 0x12C8; // float32
    pub const vPullLocation: usize = 0x12CC; // Vector
}

pub mod CDOTA_Modifier_Aghsfort_Special_Mars_Bulwark_Spears {
    pub const stack_limit: usize = 0x12C8; // int32
    pub const stack_interval: usize = 0x12CC; // float32
    pub const m_nFxIndexB: usize = 0x12D0; // ParticleIndex_t
    pub const iCur_stack: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Mars_Spear_Burning_Trail_Burn {
    pub const flDamage: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Mars_Spear_Burning_Trail_Thinker {
    pub const m_vPathDir: usize = 0x12C8; // Vector
    pub const path_radius: usize = 0x12D4; // int32
    pub const linger_duration: usize = 0x12D8; // float32
    pub const flDamage: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Special_PhantomAssassin_PhantomStrike_Damage_Reduction {
    pub const damage_reduction_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Special_PhantomAssassin_Stifling_Dagger_AutoDagger {
    pub const fire_rate: usize = 0x12C8; // float32
    pub const radius: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Special_PhantomAssassin_Stifling_Dagger_On_Attack {
    pub const strike_chance: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Special_QueenOfPain_SonicWave_AttackBuff {
    pub const spell_amp: usize = 0x12C8; // int32
    pub const bonus_damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Special_QueenOfPain_Sonic_Wave_Trail_Burn {
    pub const flDamage: usize = 0x12C8; // float32
    pub const burn_interval: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Special_QueenOfPain_Sonic_Wave_Trail_Thinker {
    pub const m_vPathDir: usize = 0x12C8; // Vector
    pub const m_nRadius: usize = 0x12D4; // int32
    pub const m_nTrailFX: usize = 0x12D8; // ParticleIndex_t
    pub const linger_duration: usize = 0x12DC; // float32
    pub const flDamage: usize = 0x12E0; // float32
    pub const damage_pct: usize = 0x12E4; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Sand_King_Caustic_Finale_StrengthBuff {
    pub const str_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Sand_King_Epicenter_DamageResist {
    pub const damage_reduction: usize = 0x12C8; // int32
    pub const status_resistance: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Sand_King_Epicenter_Movement {
    pub const vLastPos: usize = 0x12C8; // Vector
    pub const m_fAccumulatedDistance: usize = 0x12D4; // float32
    pub const movement_distance: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Sand_King_Sandstorm_Vacuum {
    pub const pull_speed: usize = 0x12C8; // float32
    pub const vPullLocation: usize = 0x12CC; // Vector
}

pub mod CDOTA_Modifier_Aghsfort_Special_Slark_Pounce_Leashed_Bonus {
    pub const attack_speed_bonus: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Slark_Shadow_Dance_Leash {
    pub const m_vPullLocation: usize = 0x12C8; // Vector
    pub const pull_speed: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Snapfire_FiresnapCookie_AlliedBuff {
    pub const bonus_attack_range: usize = 0x12C8; // int32
    pub const bonus_attack_damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Ursa_Earthshock_Knockback {
    pub const value3: usize = 0x12F8; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Viper_Periodic_Strike {
    pub const fire_rate: usize = 0x12C8; // float32
    pub const radius: usize = 0x12CC; // int32
    pub const m_nTargetTeam: usize = 0x12D0; // int32
    pub const m_hAlreadyHit: usize = 0x12D8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Aghsfort_Special_VoidSpirit_AstralStep_Breach_Burn {
    pub const m_fDamagePerInterval: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Special_VoidSpirit_AstralStep_Trail_Burn {
    pub const m_fDamagePerInterval: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Special_VoidSpirit_AstralStep_Trail_Thinker {
    pub const m_fThinkInterval: usize = 0x12C8; // float32
    pub const m_fPopDamagePctPerInterval: usize = 0x12CC; // float32
    pub const m_fLingerDuration: usize = 0x12D0; // float32
    pub const radius: usize = 0x12D4; // int32
    pub const pop_damage: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Special_VoidSpirit_Dissimilate_Lure_Burn {
    pub const m_fDamagePerInterval: usize = 0x12C8; // float32
    pub const m_nBonusIncomingMagicDamage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Special_VoidSpirit_Dissimilate_Lure_Thinker {
    pub const m_nDamageRadius: usize = 0x12C8; // int32
    pub const m_fDissimilateDamage: usize = 0x12CC; // float32
    pub const m_fThinkInterval: usize = 0x12D0; // float32
    pub const m_fDissimilateDamagePctPerInterval: usize = 0x12D4; // float32
    pub const m_fLingerDuration: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Weaver_Shukuchi_Trail_Burn {
    pub const flDamage: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Weaver_Shukuchi_Trail_Thinker {
    pub const m_vPathDir: usize = 0x12C8; // Vector
    pub const path_radius: usize = 0x12D4; // int32
    pub const linger_duration: usize = 0x12D8; // float32
    pub const flDamage: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Winter_Wyvern_Arctic_Burn_DoubleAttack {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_iAttacksRemaining: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Winter_Wyvern_Arctic_Burn_Splitshot {
    pub const damage_modifier: usize = 0x12C8; // int32
    pub const split_shot_bonus_range: usize = 0x12CC; // int32
    pub const arrow_count: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Winter_Wyvern_Cold_Embrace_Magic_Damage_Block {
    pub const barrier_amount: usize = 0x12C8; // int32
    pub const barrier_block: usize = 0x12CC; // int32
    pub const m_nMaxMovementSpeed: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Special_Winter_Wyvern_Splinter_Blast_Vacuum {
    pub const pull_speed: usize = 0x12C8; // float32
    pub const vPullLocation: usize = 0x12CC; // Vector
}

pub mod CDOTA_Modifier_Aghsfort_Special_WitchDoctor_ParalyzingCask_Attack_Procs {
    pub const cask_chance: usize = 0x12C8; // int32
    pub const cask_cooldown: usize = 0x12CC; // int32
    pub const flLastProcTime: usize = 0x12D0; // GameTime_t
}

pub mod CDOTA_Modifier_Aghsfort_Special_Witch_Doctor_Death_Ward_Splitshot {
    pub const damage_modifier: usize = 0x12C8; // int32
    pub const split_shot_bonus_range: usize = 0x12CC; // int32
    pub const split_shot_count: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Aghsfort_TempBuff_CorpseExplosion_Aura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_TempBuff_CorpseExplosion_Debuff {
    pub const corpse_explosion_radius: usize = 0x12C8; // int32
    pub const corpse_explosion_damage_pct: usize = 0x12CC; // int32
    pub const corpse_explosion_chance_pct: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Aghsfort_TemplarAssassin_Meld {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const launched_attack: usize = 0x12CC; // bool
    pub const m_hTarget: usize = 0x12D0; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Aghsfort_TemplarAssassin_MeldArmor {
    pub const bonus_armor: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_TemplarAssassin_Meld_Animation {
    pub const launched_attack: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_Aghsfort_TemplarAssassin_PsiBlades {
    pub const bonus_attack_range: usize = 0x12C8; // int32
    pub const attack_spill_range: usize = 0x12CC; // float32
    pub const attack_spill_width: usize = 0x12D0; // int32
    pub const attack_spill_pct: usize = 0x12D4; // int32
    pub const attack_spill_penalty: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_TemplarAssassin_PsiBlades_Slow {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_TemplarAssassin_Psionic_Trap_Thinker {
    pub const trap_radius: usize = 0x12C8; // int32
    pub const trap_delay_time: usize = 0x12CC; // float32
    pub const slow_duration: usize = 0x12D0; // float32
    pub const max_pulses: usize = 0x12D4; // int32
    pub const iCurrentPulse: usize = 0x12D8; // int32
    pub const m_hImage: usize = 0x12DC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Aghsfort_TemplarAssassin_RefractionAbsorb {
    pub const damage_threshold: usize = 0x12C8; // int32
    pub const max_damage_absorb: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_TemplarAssassin_RefractionDamage {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_TemplarAssassin_Refraction_Holdout {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const damage_absorb: usize = 0x12CC; // int32
    pub const m_flDamageAbsorbed: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Aghsfort_TemplarAssassin_Trap_Slow {
    pub const movement_speed_slow: usize = 0x12C8; // int32
    pub const trap_damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_TuskBoss_IceShard {
    pub const m_vPos: usize = 0x12C8; // Vector
    pub const radius: usize = 0x12D4; // int32
    pub const damage: usize = 0x12D8; // float32
    pub const explosion_interval: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Aghsfort_TuskBoss_Tag_Team {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const slow_duration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Aghsfort_TuskBoss_Tag_Team_Aura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_TuskBoss_Tag_Team_Slow {
    pub const movement_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_TuskBoss_WalrusPunch {
    pub const crit_multiplier: usize = 0x12C8; // int32
    pub const air_time: usize = 0x12CC; // float32
    pub const slow_duration: usize = 0x12D0; // float32
    pub const m_iszRangedAttackEffect: usize = 0x12D8; // CUtlSymbolLarge
    pub const m_hTarget: usize = 0x12E0; // CHandle< C_BaseEntity >
    pub const m_bWalrusPunch: usize = 0x12E4; // bool
    pub const m_bPunchKill: usize = 0x12E5; // bool
    pub const m_InFlightAttackRecords: usize = 0x12E8; // CUtlVector< int16 >
    pub const m_nFXIndex: usize = 0x1300; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Aghsfort_TuskBoss_WalrusPunch_AirTime {
    pub const hp_threshold: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_TuskBoss_WalrusPunch_Slow {
    pub const move_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Tusk_IceShard {
    pub const m_vPos: usize = 0x12C8; // Vector
    pub const radius: usize = 0x12D4; // int32
    pub const damage: usize = 0x12D8; // float32
    pub const explosion_interval: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Tusk_Snowball_SpellLifesteal {
    pub const spell_lifesteal_pct: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Tusk_Tag_Team {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const slow_duration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Tusk_Tag_Team_Aura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Tusk_Tag_Team_Slow {
    pub const movement_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Tusk_WalrusPunch {
    pub const crit_multiplier: usize = 0x12C8; // int32
    pub const air_time: usize = 0x12CC; // float32
    pub const slow_duration: usize = 0x12D0; // float32
    pub const m_iszRangedAttackEffect: usize = 0x12D8; // CUtlSymbolLarge
    pub const m_hTarget: usize = 0x12E0; // CHandle< C_BaseEntity >
    pub const m_bWalrusPunch: usize = 0x12E4; // bool
    pub const m_bPunchKill: usize = 0x12E5; // bool
    pub const m_InFlightAttackRecords: usize = 0x12E8; // CUtlVector< int16 >
    pub const m_nFXIndex: usize = 0x1300; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Aghsfort_Tusk_WalrusPunch_AirTime {
    pub const hp_threshold: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Tusk_WalrusPunch_Slow {
    pub const move_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Viper_CorrosiveSkin {
    pub const bonus_magic_resistance: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Viper_CorrosiveSkin_Flying {
    pub const m_nRadius: usize = 0x12C8; // int32
    pub const duration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Viper_CorrosiveSkin_Slow {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
    pub const damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Viper_CorrosiveSkin_Speed_Steal {
    pub const m_flFactor: usize = 0x12C8; // float32
    pub const bonus_attack_speed: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Viper_Nethertoxin {
    pub const min_damage: usize = 0x12C8; // int32
    pub const max_damage: usize = 0x12CC; // int32
    pub const max_duration: usize = 0x12D0; // float32
    pub const m_flDamageInterval: usize = 0x12D4; // float32
    pub const m_flTimeIncrement: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Viper_Nethertoxin_Thinker {
    pub const radius: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Aghsfort_Viper_PoisonAttack {
    pub const duration: usize = 0x12C8; // float32
    pub const m_InFlightAttackRecords: usize = 0x12D0; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Aghsfort_Viper_PoisonAttack_Slow {
    pub const damage: usize = 0x12C8; // float32
    pub const movement_speed: usize = 0x12CC; // int32
    pub const magic_resistance: usize = 0x12D0; // int32
    pub const m_flAccumDamage: usize = 0x12D4; // float32
    pub const m_flPoisonSnapCounter: usize = 0x12D8; // float32
    pub const m_nPoisonSnapThreshold: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Viper_ViperStrike_Slow {
    pub const bonus_movement_speed: usize = 0x12C8; // int32
    pub const bonus_attack_speed: usize = 0x12CC; // int32
    pub const damage: usize = 0x12D0; // int32
    pub const m_bTargetedAlly: usize = 0x12D4; // bool
}

pub mod CDOTA_Modifier_Aghsfort_Weaver_GeminateAttack {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_iAttacksRemaining: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Weaver_GeminateAttack_Bonus {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Weaver_Shukuchi {
    pub const radius: usize = 0x12D8; // int32
    pub const damage: usize = 0x12DC; // int32
    pub const speed: usize = 0x12E0; // int32
    pub const m_hEntitiesAffected: usize = 0x12E8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const bHasStartedBurning: usize = 0x1300; // bool
    pub const m_nBurningTrailRadius: usize = 0x1304; // int32
    pub const m_vLastTrailThinkerLocation: usize = 0x1308; // Vector
}

pub mod CDOTA_Modifier_Aghsfort_Weaver_Swarm {
    pub const m_iMaxDist: usize = 0x12C8; // int32
    pub const m_iAttackCount: usize = 0x12CC; // int32
    pub const destroy_attacks: usize = 0x12D0; // int32
    pub const m_vDirection: usize = 0x12D4; // Vector
    pub const m_vStartLoc: usize = 0x12E0; // Vector
    pub const m_hAttachTarget: usize = 0x12EC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Aghsfort_Weaver_Swarm_Debuff {
    pub const armor_reduction: usize = 0x12C8; // float32
    pub const damage: usize = 0x12CC; // int32
    pub const damage_share_percentage: usize = 0x12D0; // int32
    pub const m_flCurrentArmorReduction: usize = 0x12D4; // float32
    pub const flTotalDamage: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Weaver_TimeLapse {
    pub const m_PositionIndex: usize = 0x12C8; // Vector[11]
    pub const m_flLife: usize = 0x134C; // float32[11]
    pub const m_flMana: usize = 0x1378; // float32[11]
    pub const m_bHasValidHistory: usize = 0x13A4; // bool
}

pub mod CDOTA_Modifier_Aghsfort_Wildwing_Tornado_Blast_Debuff {
    pub const m_flStartTime: usize = 0x12C8; // GameTime_t
}

pub mod CDOTA_Modifier_Aghsfort_Winter_Wyvern_Arctic_Burn_Flight {
    pub const attack_point: usize = 0x12C8; // float32
    pub const attack_range_bonus: usize = 0x12CC; // int32
    pub const projectile_speed_bonus: usize = 0x12D0; // int32
    pub const night_vision_bonus: usize = 0x12D4; // int32
    pub const m_iszRangedAttackEffect: usize = 0x12D8; // CUtlSymbolLarge
}

pub mod CDOTA_Modifier_Aghsfort_Winter_Wyvern_Arctic_Burn_Frost_Attack {
    pub const m_InFlightAttackRecords: usize = 0x12C8; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Aghsfort_Winter_Wyvern_Arctic_Burn_Slow {
    pub const damage_per_second: usize = 0x12C8; // float32
    pub const move_slow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Winter_Wyvern_Cold_Embrace {
    pub const m_nOriginalAttackCapability: usize = 0x12C8; // int32
    pub const m_bDidBlast: usize = 0x12D4; // bool
    pub const heal_additive: usize = 0x12D8; // int32
    pub const heal_percentage: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Aghsfort_Winter_Wyvern_Splinter_Blast_Slow {
    pub const bonus_movespeed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Winter_Wyvern_Winters_Curse {
    pub const m_hZombieTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_hDesiredTarget: usize = 0x12CC; // CHandle< C_BaseEntity >
    pub const damage_reduction: usize = 0x12D0; // int32
    pub const bonus_attack_speed: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Winter_Wyvern_Winters_Curse_Aura {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_nAlliesTaunted: usize = 0x12CC; // int32
    pub const m_bRelicTriggered: usize = 0x12D0; // bool
    pub const radius: usize = 0x12D4; // int32
    pub const damage_amplification: usize = 0x12D8; // int32
    pub const damage_reduction: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_Winter_Wyvern_Winters_Curse_Kill_Credit {
    pub const m_InFlightAttackRecords: usize = 0x12C8; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Aghsfort_WitchDoctor_Cask_Thinker {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const remaining_bounces: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_WitchDoctor_DeathWard {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const m_iBounceRadius: usize = 0x12CC; // int32
    pub const m_bFirstAttack: usize = 0x12D0; // bool
    pub const m_iBounces: usize = 0x12D4; // int32
    pub const m_nStaffParticle: usize = 0x12D8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Aghsfort_WitchDoctor_Maledict {
    pub const m_iStartHealth: usize = 0x12C8; // int32
    pub const bonus_damage: usize = 0x12CC; // int32
    pub const bonus_damage_threshold: usize = 0x12D0; // int32
    pub const max_bonus_damage: usize = 0x12D4; // int32
    pub const ticks: usize = 0x12D8; // int32
    pub const iCurrentTick: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Aghsfort_WitchDoctor_Voodoo_Restoration_Heal {
    pub const heal: usize = 0x12C8; // int32
    pub const mana_per_second: usize = 0x12CC; // float32
    pub const heal_interval: usize = 0x12D0; // float32
    pub const lifesteal_percent: usize = 0x12D4; // int32
    pub const spell_lifesteal_pct: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Alchemist_AcidSpray {
    pub const armor_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Alchemist_AcidSpray_Thinker {
    pub const damage: usize = 0x12C8; // int32
    pub const armor_allies: usize = 0x12CC; // int32
    pub const radius: usize = 0x12D0; // int32
    pub const m_nFXIndex: usize = 0x12D4; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Alchemist_Berserk_Potion {
    pub const attack_speed: usize = 0x12C8; // int32
    pub const hp_regen: usize = 0x12CC; // int32
    pub const move_speed: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Alchemist_ChemicalRage {
    pub const base_attack_time: usize = 0x12C8; // float32
    pub const bonus_health: usize = 0x12CC; // int32
    pub const bonus_health_regen: usize = 0x12D0; // int32
    pub const bonus_mana_regen: usize = 0x12D4; // float32
    pub const bonus_movespeed: usize = 0x12D8; // int32
    pub const m_nFXAlchemistCRIndex: usize = 0x12DC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Alchemist_Corrosive_Weaponry_Debuff {
    pub const slow_per_stack: usize = 0x12C8; // int32
    pub const status_resist_per_stack: usize = 0x12CC; // float32
    pub const chemical_rage_bonus: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Alchemist_GoblinsGreed {
    pub const m_DeathList: usize = 0x12C8; // CUtlVector< GameTime_t >
    pub const damage: usize = 0x12E0; // float32
    pub const duration: usize = 0x12E4; // float32
    pub const bonus_gold: usize = 0x12E8; // int32
    pub const bonus_bonus_gold: usize = 0x12EC; // int32
    pub const bonus_gold_cap: usize = 0x12F0; // int32
}

pub mod CDOTA_Modifier_Alchemist_Scepter_Bonus_Damage {
    pub const scepter_bonus_damage: usize = 0x12C8; // int32
    pub const scepter_spell_amp: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Alchemist_UnstableConcoction {
    pub const brew_time: usize = 0x12C8; // float32
    pub const brew_explosion: usize = 0x12CC; // float32
    pub const m_fStartTime: usize = 0x12D0; // GameTime_t
    pub const m_fLastAlertTime: usize = 0x12D4; // GameTime_t
    pub const m_nConcoctionFXIndex: usize = 0x12D8; // ParticleIndex_t
    pub const m_bHasStunned: usize = 0x12DC; // bool
    pub const damage_resistance: usize = 0x12E0; // int32
    pub const move_speed: usize = 0x12E4; // int32
}

pub mod CDOTA_Modifier_AlphaWolf_CommandAura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AlphaWolf_CommandAura_Bonus {
    pub const bonus_damage_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AlphaWolf_CriticalStrike {
    pub const crit_mult: usize = 0x12C8; // int32
    pub const crit_chance: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_AncientApparition_ChillingTouch {
    pub const damage: usize = 0x12C8; // int32
    pub const duration: usize = 0x12CC; // float32
    pub const attack_range_bonus: usize = 0x12D0; // int32
    pub const m_InFlightAttackRecords: usize = 0x12D8; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_AncientApparition_ChillingTouch_Slow {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AncientApparition_ColdFeet {
    pub const m_vecStartPosition: usize = 0x12C8; // Vector
    pub const m_nCurrentTick: usize = 0x12D4; // int32
    pub const damage: usize = 0x12D8; // float32
    pub const break_distance: usize = 0x12DC; // int32
    pub const stun_duration: usize = 0x12E0; // float32
}

pub mod CDOTA_Modifier_AncientApparition_IceAge {
    pub const damage: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_AncientApparition_IceBlast {
    pub const dot_damage: usize = 0x12C8; // float32
    pub const kill_pct: usize = 0x12CC; // float32
    pub const m_hFracturingEntity: usize = 0x12D0; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_AncientApparition_IceVortex {
    pub const movement_speed_pct: usize = 0x12C8; // int32
    pub const spell_resist_pct: usize = 0x12CC; // int32
    pub const drag_speed: usize = 0x12D0; // int32
    pub const aura_origin_x: usize = 0x12D4; // float32
    pub const aura_origin_y: usize = 0x12D8; // float32
    pub const shard_dps: usize = 0x12DC; // int32
    pub const shard_attack_speed_reduction: usize = 0x12E0; // int32
    pub const m_vCenter: usize = 0x12E4; // Vector
}

pub mod CDOTA_Modifier_AncientApparition_IceVortexThinker {
    pub const radius: usize = 0x12C8; // int32
    pub const movement_speed_pct: usize = 0x12CC; // int32
    pub const spell_resist_pct: usize = 0x12D0; // int32
    pub const m_nFXIndex: usize = 0x12D4; // ParticleIndex_t
    pub const m_hChilledEntities: usize = 0x12D8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_AncientRockGolem_Weakening {
    pub const armor_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AncientRockGolem_Weakening_Aura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Animation_LeftClawSwipe {
    pub const damage: usize = 0x12C8; // int32
    pub const damage_radius: usize = 0x12CC; // int32
    pub const m_flScalar: usize = 0x12D0; // float32
    pub const m_vHitEntities: usize = 0x12D8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Animation_RightClawSwipe {
    pub const damage: usize = 0x12C8; // int32
    pub const damage_radius: usize = 0x12CC; // int32
    pub const m_flScalar: usize = 0x12D0; // float32
    pub const m_vHitEntities: usize = 0x12D8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Animation_TailSpin {
    pub const damage: usize = 0x12C8; // int32
    pub const damage_radius: usize = 0x12CC; // int32
    pub const m_flPlaybackRate: usize = 0x12D0; // float32
    pub const m_vHitEntities: usize = 0x12D8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_AntiMage_Counterspell {
    pub const m_LastParams: usize = 0x12C8; // CModifierParams
    pub const magic_resistance: usize = 0x14A0; // int32
}

pub mod CDOTA_Modifier_AntiMage_Counterspell_Passive {
    pub const magic_resistance: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_AntiMage_ManaBreak {
    pub const percent_damage_per_burn: usize = 0x12C8; // int32
    pub const mana_per_hit: usize = 0x12CC; // int32
    pub const mana_per_hit_pct: usize = 0x12D0; // float32
    pub const slow_duration: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_AntiMage_ManaBreak_Slow {
    pub const move_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Antimage_DampenMagic {
    pub const shard_spell_damage_reduction_pct: usize = 0x12C8; // int32
    pub const shard_bonus_spell_damage_reduction_pct: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Antimage_DampenMagic_Aura {
    pub const shard_radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Antimage_DampenMagic_Aura_Strong {
    pub const shard_bonus_aura_range: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_ArcWarden_Flux {
    pub const m_nCasterTeam: usize = 0x12C8; // int32
    pub const damage_per_second: usize = 0x12CC; // int32
    pub const tempest_damage_per_second: usize = 0x12D0; // int32
    pub const search_radius: usize = 0x12D4; // int32
    pub const move_speed_slow_pct: usize = 0x12D8; // int32
    pub const tempest_move_speed_slow_pct: usize = 0x12DC; // int32
    pub const status_resist: usize = 0x12E0; // int32
    pub const think_interval: usize = 0x12E4; // float32
    pub const m_nCurrentMovementSlow: usize = 0x12E8; // int32
    pub const m_nCurrentStatusResistance: usize = 0x12EC; // int32
    pub const m_bAlone: usize = 0x12F0; // bool
    pub const m_nFXIndex: usize = 0x12F4; // ParticleIndex_t
    pub const m_bIsTempestVersion: usize = 0x12F8; // bool
}

pub mod CDOTA_Modifier_ArcWarden_MagneticField_AttackRange {
    pub const attack_range_bonus: usize = 0x12C8; // int32
    pub const attack_magic_damage: usize = 0x12CC; // int32
    pub const attack_damage_bonus: usize = 0x12D0; // int32
    pub const radius: usize = 0x12D4; // int32
    pub const shard_magic_resist: usize = 0x12D8; // int32
    pub const shard_slow_pct: usize = 0x12DC; // int32
    pub const aura_origin_x: usize = 0x12E0; // float32
    pub const aura_origin_y: usize = 0x12E4; // float32
    pub const m_InFlightAttackRecords: usize = 0x12E8; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_ArcWarden_MagneticField_Attack_Speed {
    pub const attack_speed_bonus: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_ArcWarden_MagneticField_Evasion {
    pub const evasion_chance: usize = 0x12C8; // int32
    pub const radius: usize = 0x12CC; // int32
    pub const shard_magic_resist: usize = 0x12D0; // int32
    pub const shard_slow_pct: usize = 0x12D4; // int32
    pub const aura_origin_x: usize = 0x12D8; // float32
    pub const aura_origin_y: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_ArcWarden_MagneticField_Thinker_AttackRange {
    pub const radius: usize = 0x12D0; // int32
    pub const m_vPos: usize = 0x12D4; // Vector
}

pub mod CDOTA_Modifier_ArcWarden_MagneticField_Thinker_Attack_Speed {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_ArcWarden_MagneticField_Thinker_Evasion {
    pub const radius: usize = 0x12D0; // int32
    pub const m_vPos: usize = 0x12D4; // Vector
}

pub mod CDOTA_Modifier_ArcWarden_SparkWraith_Purge {
    pub const move_speed_slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_ArcWarden_SparkWraith_Thinker {
    pub const radius: usize = 0x12C8; // int32
    pub const scepter_radius: usize = 0x12CC; // int32
    pub const wraith_vision_radius: usize = 0x12D0; // int32
    pub const wraith_speed: usize = 0x12D4; // int32
    pub const activation_delay: usize = 0x12E0; // float32
    pub const scepter_activation_delay: usize = 0x12E4; // float32
    pub const think_interval: usize = 0x12E8; // float32
    pub const m_flSparkDamage: usize = 0x12EC; // float32
    pub const m_nViewerID: usize = 0x12F0; // int32
    pub const m_bActive: usize = 0x12F4; // bool
    pub const m_bOriginal: usize = 0x12F5; // bool
}

pub mod CDOTA_Modifier_ArcWarden_TempestDouble {
    pub const penalty_distance: usize = 0x12C8; // int32
    pub const attack_damage_penalty: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_ArcWarden_TempestDouble_DistancePenalty {
    pub const attack_damage_penalty: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Arc_Warden_Scepter {
    pub const m_hRune: usize = 0x12C8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_ArcaneBoots_ManaRegen {
    pub const value: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Armlet_UnholyStrength {
    pub const unholy_bonus_damage: usize = 0x12C8; // int32
    pub const unholy_bonus_attack_speed: usize = 0x12CC; // int32
    pub const unholy_bonus_strength: usize = 0x12D0; // int32
    pub const unholy_bonus_armor: usize = 0x12D4; // int32
    pub const unholy_health_drain_per_second: usize = 0x12D8; // int32
    pub const str_tick_count: usize = 0x12DC; // int32
    pub const unholy_bonus_slow_resistance: usize = 0x12E0; // int32
    pub const tick_interval: usize = 0x12E4; // float32
    pub const m_flDamageRemainder: usize = 0x12E8; // float32
}

pub mod CDOTA_Modifier_Ascension_AcidBlood {
    pub const armor_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Ascension_AcidBlood_Thinker {
    pub const damage: usize = 0x12C8; // int32
    pub const radius: usize = 0x12CC; // int32
    pub const m_nFXIndex: usize = 0x12D0; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Ascension_Bulwark {
    pub const physical_damage_reduction: usize = 0x12C8; // float32
    pub const physical_damage_reduction_side: usize = 0x12CC; // float32
    pub const forward_angle: usize = 0x12D0; // float32
    pub const side_angle: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Ascension_Meteoric_Land {
    pub const impact_radius: usize = 0x12C8; // int32
    pub const land_time: usize = 0x12CC; // float32
    pub const burn_duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Assassins_Dagger_Debuff {
    pub const armor_debuff: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Attached_Unit {
    pub const m_hAttachTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_bPhysicalImmune: usize = 0x12CC; // bool
    pub const m_bMagicImmune: usize = 0x12CD; // bool
}

pub mod CDOTA_Modifier_Axe_BatleHunger_Self {
    pub const scepter_armor_change: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Axe_BatleHunger_Self_Movespeed {
    pub const speed_bonus: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Axe_BattleHunger {
    pub const damage_per_second: usize = 0x12C8; // int32
    pub const slow: usize = 0x12CC; // int32
    pub const scepter_armor_change: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Axe_BerserkersCall {
    pub const m_hZombieTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_bDidSetAttackTarget: usize = 0x12CC; // bool
    pub const bonus_attack_speed: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Axe_BerserkersCallArmor {
    pub const bonus_armor: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Axe_CounterHelix {
    pub const radius: usize = 0x12C8; // int32
    pub const trigger_attacks: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Axe_CounterHelix_DamageReduction {
    pub const shard_damage_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Axe_CullingBlade_Boost {
    pub const speed_bonus: usize = 0x12C8; // int32
    pub const atk_speed_bonus: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Axe_CullingBlade_Permanent {
    pub const armor_per_stack: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Banana {
    pub const slide_distance: usize = 0x12C8; // int32
    pub const radius: usize = 0x12CC; // int32
    pub const slide_duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Banana_Knockback {
    pub const m_vCenter: usize = 0x12C8; // Vector
    pub const knockback_distance: usize = 0x12D4; // int32
    pub const knockback_height: usize = 0x12D8; // int32
    pub const knockback_duration: usize = 0x12DC; // float32
    pub const m_vHorizOffset: usize = 0x12E0; // Vector
    pub const m_vStartPosition: usize = 0x12EC; // Vector
    pub const m_vDir: usize = 0x12F8; // Vector
    pub const m_fCurrentTimeHoriz: usize = 0x1304; // float32
    pub const m_fCurrentTimeVert: usize = 0x1308; // float32
    pub const m_bShouldStun: usize = 0x130C; // bool
    pub const flTimeSinceChange: usize = 0x1310; // float32
}

pub mod CDOTA_Modifier_Bane_Enfeeble_Effect {
    pub const damage_reduction: usize = 0x12C8; // int32
    pub const heal_reduction: usize = 0x12CC; // int32
    pub const cast_reduction: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Bane_FiendsGrip {
    pub const fiend_grip_mana_drain: usize = 0x12C8; // int32
    pub const fiend_grip_tick_interval: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Bane_FiendsGrip_Cast_Illusion {
    pub const fiend_grip_mana_drain: usize = 0x12C8; // int32
    pub const fiend_grip_tick_interval: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Bane_Nightmare {
    pub const m_nSource: usize = 0x12C8; // int32
    pub const animation_rate: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Bane_Nightmare_AttackSpeed_Bonus {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
    pub const m_hTarget: usize = 0x12CC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_BaseBlocker {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Batrider_Firefly {
    pub const m_bActive: usize = 0x12C8; // bool
    pub const radius: usize = 0x12CC; // int32
    pub const m_ifirefly_Active: usize = 0x12D0; // int32
    pub const tree_radius: usize = 0x12D4; // int32
    pub const damage_per_second: usize = 0x12D8; // int32
    pub const movement_speed: usize = 0x12DC; // int32
    pub const bonus_slow_resistance: usize = 0x12E0; // int32
    pub const tick_interval: usize = 0x12E4; // float32
    pub const m_nFXIndex: usize = 0x12E8; // ParticleIndex_t
    pub const m_nFXIndexB: usize = 0x12EC; // ParticleIndex_t
    pub const m_fNextDamageTick: usize = 0x12F0; // GameTime_t
    pub const m_vLastFirePoolLoc: usize = 0x12F4; // Vector
    pub const bonus_vision: usize = 0x1300; // int32
}

pub mod CDOTA_Modifier_Batrider_Flamebreak_Damage {
    pub const damage_impact: usize = 0x12C8; // int32
    pub const damage_per_second: usize = 0x12CC; // int32
    pub const slow: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Batrider_FlamingLasso {
    pub const tick_rate: usize = 0x12C8; // float32
    pub const drag_distance: usize = 0x12CC; // int32
    pub const break_distance: usize = 0x12D0; // int32
    pub const m_nFXIndex: usize = 0x12D4; // ParticleIndex_t
    pub const vPreviousLocation: usize = 0x12D8; // Vector
}

pub mod CDOTA_Modifier_Batrider_FlamingLasso_Damage {
    pub const max_ticks: usize = 0x12C8; // int32
    pub const damage: usize = 0x12CC; // float32
    pub const duration: usize = 0x12D0; // float32
    pub const tick_rate: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Batrider_StickyNapalm {
    pub const movement_speed_pct: usize = 0x12C8; // float32
    pub const turn_rate_pct: usize = 0x12CC; // int32
    pub const damage: usize = 0x12D0; // int32
    pub const application_damage: usize = 0x12D4; // int32
    pub const m_nFXIndex: usize = 0x12D8; // ParticleIndex_t
    pub const m_nFXStackIndex: usize = 0x12DC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Batrider_StickyNapalm_Application {
    pub const m_bShouldApply: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_Beastmaster_Axe_Stack_Counter {
    pub const damage_amp: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Beastmaster_CallOfTheWild_Hawk {
    pub const m_hOwner: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const attack_radius: usize = 0x12CC; // int32
    pub const m_flLastAttack: usize = 0x12D0; // GameTime_t
    pub const roaming_seconds_per_rotation: usize = 0x12D4; // float32
    pub const roaming_radius: usize = 0x12D8; // int32
    pub const attack_interval: usize = 0x12DC; // float32
    pub const min_move_speed: usize = 0x12E0; // int32
    pub const max_move_speed: usize = 0x12E4; // int32
    pub const m_iPreviousHawkCount: usize = 0x12E8; // int32
    pub const m_flRotation: usize = 0x12EC; // float32
}

pub mod CDOTA_Modifier_Beastmaster_DrumsOfSlom {
    pub const aura_radius: usize = 0x12C8; // int32
    pub const radius: usize = 0x12CC; // int32
    pub const max_stacks: usize = 0x12D0; // int32
    pub const base_damage: usize = 0x12D4; // float32
    pub const heal_pct: usize = 0x12D8; // int32
    pub const creep_heal_pct: usize = 0x12DC; // int32
    pub const iCurrentAttacksAtMinInterval: usize = 0x12E0; // int32
    pub const stack_decay_time: usize = 0x12E4; // float32
    pub const max_drum_hit_interval: usize = 0x12E8; // float32
    pub const min_drum_hit_interval: usize = 0x12EC; // float32
    pub const m_flLastStackChangeTime: usize = 0x12F0; // GameTime_t
    pub const m_flLastDrumHitTime: usize = 0x12F4; // GameTime_t
}

pub mod CDOTA_Modifier_Beastmaster_Hawk_Dive {
    pub const dive_damage: usize = 0x12C8; // int32
    pub const dive_root_duration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Beastmaster_Hawk_Perch_Flight {
    pub const perch_flight_height: usize = 0x12C8; // int32
    pub const perch_tree_height: usize = 0x12CC; // int32
    pub const flight_speed: usize = 0x12D0; // int32
    pub const m_flZDelta: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Beastmaster_Hawk_Perch_Perching {
    pub const perching_tree_height: usize = 0x12C8; // int32
    pub const m_hTree: usize = 0x12CC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Beastmaster_InnerBeast_Aura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Beastmaster_PrimalRoar_Push {
    pub const m_vVelocity: usize = 0x12C8; // Vector
}

pub mod CDOTA_Modifier_Beastmaster_PrimalRoar_Slow {
    pub const slow_movement_speed_pct: usize = 0x12C8; // int32
    pub const slow_attack_speed_pct: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Beastmaster_PrimalRoar_Speed {
    pub const movement_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Beastmaster_WildAxes {
    pub const m_vLeftControl: usize = 0x12C8; // Vector
    pub const m_vRightControl: usize = 0x12D4; // Vector
    pub const m_vTargetLoc: usize = 0x12E0; // Vector
    pub const m_vSourceLoc: usize = 0x12EC; // Vector
    pub const m_flDuration: usize = 0x12F8; // float32
    pub const m_flDieTime: usize = 0x12FC; // GameTime_t
    pub const m_bReturning: usize = 0x1300; // bool
    pub const m_bCatchingAxes: usize = 0x1301; // bool
    pub const m_hAxes: usize = 0x1308; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_nAxeFXIndex: usize = 0x1320; // ParticleIndex_t[2]
    pub const m_hHitList: usize = 0x1328; // CUtlVector< CHandle< C_BaseEntity > >[2]
    pub const radius: usize = 0x1358; // int32
    pub const spread: usize = 0x135C; // int32
    pub const axe_damage: usize = 0x1360; // int32
    pub const duration: usize = 0x1364; // float32
    pub const min_throw_duration: usize = 0x1368; // float32
    pub const max_throw_duration: usize = 0x136C; // float32
}

pub mod CDOTA_Modifier_BerserkerTroll_Break {
    pub const duration: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_BigThunderLizard_Frenzy {
    pub const attackspeed_bonus: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_BigThunderLizard_Slam {
    pub const movespeed_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_BigThunderLizard_Wardrums {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_BigThunderLizard_Wardrums_Aura {
    pub const speed_bonus: usize = 0x12C8; // int32
    pub const accuracy: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_BlackDragon_DragonhideAura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_BlackDragon_DragonhideAura_Bonus {
    pub const bonus_armor: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_BlackDragon_Fireball_Thinker {
    pub const burn_interval: usize = 0x12C8; // float32
    pub const radius: usize = 0x12CC; // int32
    pub const damage: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_BlackDragon_SplashAttack {
    pub const range: usize = 0x12C8; // int32
    pub const damage_percent: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_BlackDrake_MagicAmplification {
    pub const spell_amp: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_BlackDrake_MagicAmplification_Aura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Black_King_Bar_Immune {
    pub const model_scale: usize = 0x12C8; // int32
    pub const magic_resist: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Blight_Stone_Corruption {
    pub const corruption_armor: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Bloodseeker_BloodMist {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Bloodseeker_BloodMist_Barrier {
    pub const flLastGameTime: usize = 0x12C8; // GameTime_t
    pub const m_flCurrentShield: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Bloodseeker_BloodMist_Slow {
    pub const movement_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Bloodseeker_Bloodbath_Thinker {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Bloodseeker_Bloodrage {
    pub const attack_speed: usize = 0x12C8; // int32
    pub const spell_amp: usize = 0x12CC; // int32
    pub const shard_max_health_dmg_pct: usize = 0x12D0; // float32
    pub const damage_pct: usize = 0x12D4; // float32
    pub const flTickRate: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Bloodseeker_Rupture {
    pub const movement_damage_pct: usize = 0x12C8; // int32
    pub const hp_pct: usize = 0x12CC; // float32
    pub const vLastPos: usize = 0x12D0; // Vector
    pub const bResetPosition: usize = 0x12DC; // bool
    pub const m_fAccumulatedDamage: usize = 0x12E0; // float32
}

pub mod CDOTA_Modifier_Bloodseeker_Thirst {
    pub const hero_kill_heal: usize = 0x12C8; // int32
    pub const creep_kill_heal: usize = 0x12CC; // int32
    pub const half_bonus_aoe: usize = 0x12D0; // int32
    pub const visibility_threshold_pct: usize = 0x12D4; // int32
    pub const invis_threshold_pct: usize = 0x12D8; // int32
    pub const min_bonus_pct: usize = 0x12DC; // int32
    pub const max_bonus_pct: usize = 0x12E0; // int32
    pub const bonus_movement_speed: usize = 0x12E4; // int32
    pub const linger_duration: usize = 0x12E8; // float32
    pub const m_nFXIndex: usize = 0x12EC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Bloodseeker_Thirst_Speed {
    pub const m_iszModifierString: usize = 0x12C8; // CUtlSymbolLarge
}

pub mod CDOTA_Modifier_Bloodseeker_Thirst_Vision {
    pub const invis_threshold_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Bloodthorn_Debuff {
    pub const target_crit_multiplier: usize = 0x12C8; // float32
    pub const silence_damage_percent: usize = 0x12CC; // float32
    pub const m_flDamageTaken: usize = 0x12D0; // float32
    pub const m_vRecords: usize = 0x12D8; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_BlueDragonspawnOverseer_DevotionAura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_BlueDragonspawnOverseer_DevotionAura_Bonus {
    pub const bonus_armor: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_BlueDragonspawnOverseer_Evasion {
    pub const evasion_chance_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_BlueDragonspawnSorcerer_Evasion {
    pub const evasion_chance_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Boss_DarkWillow_Bedlam {
    pub const m_flRotation: usize = 0x12C8; // float32
    pub const m_hWisp: usize = 0x12CC; // CHandle< C_BaseEntity >
    pub const m_flLastAttack: usize = 0x12D0; // GameTime_t
    pub const reversed: usize = 0x12D4; // bool
    pub const roaming_radius: usize = 0x12D8; // int32
    pub const attack_radius: usize = 0x12DC; // int32
    pub const roaming_seconds_per_rotation: usize = 0x12E0; // float32
    pub const attack_interval: usize = 0x12E4; // float32
    pub const m_bSetupWarning: usize = 0x12E8; // bool
    pub const m_nPreviewFX: usize = 0x12EC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_BottleRegeneration {
    pub const health_restore: usize = 0x12C8; // int32
    pub const mana_restore: usize = 0x12CC; // int32
    pub const health_restore_pct: usize = 0x12D0; // int32
    pub const mana_restore_pct: usize = 0x12D4; // int32
    pub const break_on_hero_damage: usize = 0x12D8; // int32
    pub const m_fHealingDone: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_BountyHunter_Jinada {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const gold_steal: usize = 0x12CC; // int32
    pub const m_nFXIndexA: usize = 0x12D0; // ParticleIndex_t
    pub const m_nFXIndexB: usize = 0x12D4; // ParticleIndex_t
    pub const m_InFlightAttackRecords: usize = 0x12D8; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_BountyHunter_Lookout {
    pub const radius: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_BountyHunter_Track {
    pub const aura_radius: usize = 0x12C8; // int32
    pub const m_nParentGold: usize = 0x12CC; // int32
    pub const target_crit_multiplier: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_BountyHunter_TrackEffect {
    pub const bonus_move_speed_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_BountyHunter_Track_Bear_Trap {
    pub const m_bActivated: usize = 0x12D8; // bool
}

pub mod CDOTA_Modifier_BountyHunter_WindWalk {
    pub const damage_reduction_pct: usize = 0x12D8; // int32
    pub const shard_stun_duration: usize = 0x12DC; // float32
    pub const m_InFlightAttackRecords: usize = 0x12E0; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_BountyHunter_WindWalk_Slow {
    pub const slow: usize = 0x12C8; // int32
    pub const attack_slow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Brewmaster_BrewUp {
    pub const m_bBuffExtended: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_Brewmaster_CinderBrew {
    pub const movement_slow: usize = 0x12C8; // int32
    pub const m_bIgniteActive: usize = 0x12CC; // bool
    pub const m_flDamagePerTick: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Brewmaster_DrunkenBrawler {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const m_nFXIndexB: usize = 0x12CC; // ParticleIndex_t
    pub const min_movement: usize = 0x12D0; // int32
    pub const max_movement: usize = 0x12D4; // int32
    pub const m_iMovementBonus: usize = 0x12D8; // int32
    pub const m_flNextUpdateTime: usize = 0x12DC; // GameTime_t
}

pub mod CDOTA_Modifier_Brewmaster_DrunkenBrawler_Passive {
    pub const dodge_chance: usize = 0x12C8; // int32
    pub const crit_chance: usize = 0x12CC; // int32
    pub const crit_multiplier: usize = 0x12D0; // int32
    pub const attack_speed: usize = 0x12D4; // int32
    pub const active_multiplier: usize = 0x12D8; // float32
    pub const stun_chance: usize = 0x12DC; // int32
    pub const bash_cooldown: usize = 0x12E0; // float32
    pub const stun_duration: usize = 0x12E4; // float32
    pub const slow_duration: usize = 0x12E8; // float32
    pub const bonus_move_speed: usize = 0x12EC; // int32
    pub const bonus_status_resist: usize = 0x12F0; // int32
    pub const armor: usize = 0x12F4; // int32
    pub const magic_resist: usize = 0x12F8; // int32
    pub const m_flLastBash: usize = 0x12FC; // float32
}

pub mod CDOTA_Modifier_Brewmaster_DrunkenBrawler_Slow {
    pub const movespeed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Brewmaster_DrunkenHaze {
    pub const movement_slow: usize = 0x12C8; // int32
    pub const miss_chance: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Brewmaster_Fire_Phase {
    pub const status_resistance: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Brewmaster_PermanentImmolation {
    pub const damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Brewmaster_PermanentImmolation_Aura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Brewmaster_PrimalSplit {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_hSecondaryTarget: usize = 0x12CC; // CHandle< C_BaseEntity >
    pub const m_hTertiaryTarget: usize = 0x12D0; // CHandle< C_BaseEntity >
    pub const m_hFourthTarget: usize = 0x12D4; // CHandle< C_BaseEntity >
    pub const m_hReturnBrewling: usize = 0x12D8; // CHandle< C_BaseEntity >
    pub const m_nFXIndex: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Brewmaster_PrimalSplitDelay {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Brewmaster_PrimalSplitDuration {
    pub const bonus_brewling_hp: usize = 0x12C8; // int32
    pub const bonus_damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Brewmaster_Primal_Companion {
    pub const cooldown_on_death: usize = 0x12C8; // float32
    pub const cooldown_on_take_damage: usize = 0x12CC; // float32
    pub const disable_distance: usize = 0x12D0; // int32
    pub const movement_slow_when_out_of_range: usize = 0x12D4; // int32
    pub const m_bCanAttack: usize = 0x12D8; // bool
    pub const m_nFxIndex: usize = 0x12DC; // ParticleIndex_t
    pub const cooldown_penalty_pct: usize = 0x12E0; // int32
    pub const bonus_brewling_hp: usize = 0x12E4; // int32
    pub const bonus_damage: usize = 0x12E8; // int32
}

pub mod CDOTA_Modifier_Brewmaster_Pulverize {
    pub const bonus_building_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Brewmaster_ThunderClap {
    pub const movement_slow: usize = 0x12C8; // int32
    pub const attack_speed_slow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Brewmaster_Void_AstralPulse {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Brewmaster_Void_Astral_Pull_Movement {
    pub const pull_distance: usize = 0x12C8; // int32
    pub const damage: usize = 0x12CC; // int32
    pub const pull_duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Brewmaster_Void_Brawler_Slow {
    pub const movespeed: usize = 0x12C8; // int32
    pub const active_multiplier: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Brewmaster_WindWalk {
    pub const bonus_movement_speeed: usize = 0x12D8; // int32
    pub const bonus_damage: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Bristleback_Active_ConicalQuillSpray {
    pub const activation_num_quill_sprays: usize = 0x12C8; // int32
    pub const activation_spray_interval: usize = 0x12CC; // float32
    pub const activation_angle: usize = 0x12D0; // int32
    pub const cast_range_bonus: usize = 0x12D4; // int32
    pub const activation_movement_speed_pct: usize = 0x12D8; // int32
    pub const activation_turn_rate_pct: usize = 0x12DC; // int32
    pub const activation_disable_turning: usize = 0x12E0; // int32
    pub const activation_ignore_cast_angle: usize = 0x12E4; // int32
    pub const activation_turn_rate: usize = 0x12E8; // int32
    pub const activation_delay: usize = 0x12EC; // float32
    pub const m_fStartTime: usize = 0x12F0; // GameTime_t
    pub const bDelayFinished: usize = 0x12F4; // bool
    pub const m_flFacingTarget: usize = 0x12F8; // float32
    pub const m_vFacing: usize = 0x12FC; // Vector
    pub const m_nNumSprays: usize = 0x1308; // int32
    pub const m_nPoseParameterWE: usize = 0x130C; // int32
    pub const m_nPoseParameterNS: usize = 0x1310; // int32
    pub const m_flLastPoseX: usize = 0x1314; // float32
    pub const m_flLastPoseY: usize = 0x1318; // float32
    pub const m_nLastMaxDirection: usize = 0x131C; // int32
    pub const m_vLastOrigin: usize = 0x1320; // Vector
    pub const m_flLastGameTime: usize = 0x132C; // GameTime_t
}

pub mod CDOTA_Modifier_Bristleback_Bristleback {
    pub const side_damage_reduction: usize = 0x12C8; // int32
    pub const back_damage_reduction: usize = 0x12CC; // int32
    pub const side_angle: usize = 0x12D0; // int32
    pub const back_angle: usize = 0x12D4; // int32
    pub const quill_release_threshold: usize = 0x12D8; // float32
    pub const quill_release_interval: usize = 0x12DC; // float32
    pub const m_bRearHit: usize = 0x12E0; // bool
    pub const m_bSideHit: usize = 0x12E1; // bool
    pub const m_flDamageAccumulated: usize = 0x12E4; // float32
    pub const m_nAccumulatedHits: usize = 0x12E8; // int32
}

pub mod CDOTA_Modifier_Bristleback_QuillSpray {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Bristleback_QuillSpray_Thinker {
    pub const m_fCurRadius: usize = 0x12C8; // float32
    pub const m_fLastThink: usize = 0x12CC; // GameTime_t
    pub const m_entitiesHit: usize = 0x12D0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_ViewerTimer: usize = 0x12E8; // CountdownTimer
    pub const projectile_speed: usize = 0x1300; // int32
    pub const radius: usize = 0x1304; // float32
}

pub mod CDOTA_Modifier_Bristleback_ViscousNasalGoo {
    pub const base_armor: usize = 0x12C8; // float32
    pub const armor_per_stack: usize = 0x12CC; // float32
    pub const base_move_slow: usize = 0x12D0; // int32
    pub const move_slow_per_stack: usize = 0x12D4; // int32
    pub const stack_limit: usize = 0x12D8; // int32
    pub const m_nFXStackIndex: usize = 0x12DC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Bristleback_Warpath {
    pub const damage_per_stack: usize = 0x12C8; // int32
    pub const move_speed_per_stack: usize = 0x12CC; // int32
    pub const max_stacks: usize = 0x12D0; // int32
    pub const stack_duration: usize = 0x12D4; // float32
    pub const m_flDuration: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Broodmother_IncapacitatingBite {
    pub const duration: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Broodmother_IncapacitatingBiteOrb {
    pub const miss_chance: usize = 0x12C8; // int32
    pub const bonus_movespeed: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Broodmother_InsatiableHunger {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const lifesteal_pct: usize = 0x12CC; // int32
    pub const slow_duration: usize = 0x12D0; // float32
    pub const bat_bonus: usize = 0x12D4; // float32
    pub const shard_damage_tick_interval: usize = 0x12D8; // float32
    pub const damage_tick_count: usize = 0x12DC; // int32
    pub const shard_damage_per_tick: usize = 0x12E0; // int32
    pub const creep_lifesteal_reduction_pct: usize = 0x12E4; // int32
}

pub mod CDOTA_Modifier_Broodmother_PoisonSting {
    pub const duration: usize = 0x12C8; // float32
    pub const duration_hero: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Broodmother_PoisonStingDebuff {
    pub const damage_per_second: usize = 0x12C8; // int32
    pub const movement_speed: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Broodmother_Silken_Bola {
    pub const attack_damage: usize = 0x12C8; // int32
    pub const movement_speed: usize = 0x12CC; // int32
    pub const miss_chance: usize = 0x12D0; // int32
    pub const shard_miss_chance: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Broodmother_SpawnSpiderite {
    pub const buff_duration: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Broodmother_SpawnSpideriteDebuff {
    pub const m_hParentSpider: usize = 0x12C8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Broodmother_SpawnSpiderlings {
    pub const spiderling_duration: usize = 0x12C8; // float32
    pub const count: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Broodmother_SpawnSpiderlingsShard {
    pub const shard_spiderling_duration: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Broodmother_Spider_HP {
    pub const hp_bonus: usize = 0x12C8; // int32
    pub const damage_bonus: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Broodmother_SpinWeb {
    pub const heath_regen: usize = 0x12C8; // int32
    pub const bonus_movespeed: usize = 0x12CC; // int32
    pub const bonus_movespeed_scepter: usize = 0x12D0; // int32
    pub const bonus_turn_rate: usize = 0x12D4; // float32
    pub const flLastDamageTime: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Broodmother_SpinWeb_Slowed {
    pub const bonus_movespeed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Broodmother_SpinWeb_Thinker {
    pub const radius: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Broodmother_StickySnare {
    pub const m_vOrigin: usize = 0x12C8; // Vector
    pub const m_vWallDirection: usize = 0x12D4; // Vector
    pub const m_vWallRight: usize = 0x12E0; // Vector
    pub const width: usize = 0x12EC; // int32
    pub const root_duration: usize = 0x12F0; // float32
    pub const formation_delay: usize = 0x12F4; // float32
    pub const m_bTouching: usize = 0x12F8; // bool
    pub const m_hRight: usize = 0x12FC; // CHandle< C_BaseEntity >
    pub const m_bParticle: usize = 0x1300; // bool
    pub const m_flStartingTime: usize = 0x1304; // GameTime_t
    pub const m_nFoWID: usize = 0x1308; // int32
    pub const m_nTeamID: usize = 0x130C; // int32
    pub const m_vecAffectedHeroes: usize = 0x1310; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_nWarmupFXIndex: usize = 0x1328; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Broodmother_StickySnare_Root {
    pub const damage_per_second: usize = 0x12C8; // float32
    pub const m_bFirstSecond: usize = 0x12CC; // bool
}

pub mod CDOTA_Modifier_CallOfTheWild_Boar_BonusDamage {
    pub const boar_bonus_damage: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_CallOfTheWild_Boar_PoisonBase {
    pub const duration: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_CallOfTheWild_Boar_PoisonEffect {
    pub const attack_speed: usize = 0x12C8; // int32
    pub const movement_speed: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_CentaurKhan_EnduranceAura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_CentaurKhan_EnduranceAura_Bonus {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Centaur_Cart {
    pub const m_hUnit: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_bWasMoving: usize = 0x12CC; // bool
    pub const m_bSpawnDone: usize = 0x12CD; // bool
    pub const m_vecOldForward: usize = 0x12D0; // Vector
    pub const break_distance: usize = 0x12DC; // int32
    pub const m_vecPreviousLocation: usize = 0x12E0; // Vector
}

pub mod CDOTA_Modifier_Centaur_DoubleEdge_Buff {
    pub const shard_str_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Centaur_DoubleEdge_Slow {
    pub const shard_movement_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Centaur_HoofStomp_Windup {
    pub const bInterrupted: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_Centaur_Mounted {
    pub const melee_attack_range: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Centaur_Return {
    pub const return_damage: usize = 0x12C8; // int32
    pub const return_damage_str: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Centaur_Return_Aura {
    pub const aura_radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Centaur_Return_Bonus_Damage {
    pub const damage_gain_pct: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Centaur_Stampede {
    pub const radius: usize = 0x12C8; // int32
    pub const damage_reduction: usize = 0x12CC; // int32
    pub const has_flying_movement: usize = 0x12D0; // int32
    pub const m_hEntitiesAffected: usize = 0x12D8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Centaur_Stampede_Slow {
    pub const cast_time_increase: usize = 0x12C8; // int32
    pub const slow_movement_speed: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Ceremonial_Robe_Aura {
    pub const status_resistance: usize = 0x12C8; // int32
    pub const magic_resistance: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_ChaosKnight_Chaos_Strike {
    pub const creep_multiplier: usize = 0x12C8; // float32
    pub const creep_lifesteal_reduction_pct: usize = 0x12CC; // float32
    pub const crit_min: usize = 0x12D0; // int32
    pub const crit_max: usize = 0x12D4; // int32
    pub const lifesteal: usize = 0x12D8; // int32
    pub const chance: usize = 0x12DC; // float32
    pub const m_nNextCrit: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_ChaosKnight_Chaos_Strike_Debuff {
    pub const maim_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_ChaosKnight_Reality_Rift_Debuff {
    pub const armor_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Chen_DivineFavor {
    pub const armor: usize = 0x12C8; // int32
    pub const heal_rate: usize = 0x12CC; // float32
    pub const creep_bonus: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Chen_DivineFavor_Aura {
    pub const aura_radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Chen_Divine_Favor_Teleport {
    pub const teleport_delay: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Chen_HandOfGod_Hot {
    pub const heal_per_second: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Chen_HolyPersuasion {
    pub const health_min: usize = 0x12C8; // int32
    pub const new_max: usize = 0x12CC; // int32
    pub const health_bonus: usize = 0x12D0; // int32
    pub const damage_bonus: usize = 0x12D4; // int32
    pub const movement_speed_bonus: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Chen_Penitence {
    pub const bonus_movement_speed: usize = 0x12C8; // int32
    pub const bonus_damage_taken: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Chen_Penitence_Attack_Speed_Buff {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Chen_Penitence_Self_Attack_Range {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const self_attack_range_bonus: usize = 0x12CC; // int32
    pub const m_bActive: usize = 0x12D0; // bool
}

pub mod CDOTA_Modifier_Chen_TestOfFaith_Teleport {
    pub const bToChen: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_ClarityPotion {
    pub const mana_regen: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Clinkz_BurningBarrage {
    pub const m_iArrowCount: usize = 0x12C8; // int32
    pub const m_vOriginalTarget: usize = 0x12CC; // Vector
    pub const arrow_width: usize = 0x12D8; // int32
    pub const arrow_speed: usize = 0x12DC; // float32
    pub const arrow_range_multiplier: usize = 0x12E0; // float32
    pub const wave_count: usize = 0x12E4; // int32
    pub const arrow_count_per_wave: usize = 0x12E8; // int32
    pub const arrow_angle: usize = 0x12EC; // int32
    pub const m_iLoopCount: usize = 0x12F0; // int32
    pub const m_flInterval: usize = 0x12F4; // float32
    pub const m_flCycleDelay: usize = 0x12F8; // float32
    pub const m_flTimeWaste: usize = 0x12FC; // float32
    pub const m_flExpectedTime: usize = 0x1300; // GameTime_t
}

pub mod CDOTA_Modifier_Clinkz_Burning_Army {
    pub const attack_rate: usize = 0x12C8; // float32
    pub const damage_percent: usize = 0x12CC; // int32
    pub const skeleton_health: usize = 0x12D0; // int32
    pub const m_nFXIndex: usize = 0x12D4; // ParticleIndex_t
    pub const skeleton_building_damage_reduction: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Clinkz_Burning_Army_Thinker {
    pub const m_iCount: usize = 0x12C8; // int32
    pub const count: usize = 0x12CC; // int32
    pub const range: usize = 0x12D0; // int32
    pub const m_vFacePosition: usize = 0x12D4; // Vector
}

pub mod CDOTA_Modifier_Clinkz_DeathPact {
    pub const health_gain: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Clinkz_SearingArrows {
    pub const damage_bonus: usize = 0x12C8; // int32
    pub const m_InFlightAttackRecords: usize = 0x12D0; // CUtlVector< int16 >
    pub const m_bBonusAttack: usize = 0x12E8; // bool
}

pub mod CDOTA_Modifier_Clinkz_Strafe {
    pub const attack_speed_bonus: usize = 0x12C8; // int32
    pub const attack_range_bonus: usize = 0x12CC; // int32
    pub const archer_attack_speed_pct: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Clinkz_Tar_Bomb_SearingArrows {
    pub const damage_bonus: usize = 0x12C8; // int32
    pub const m_InFlightAttackRecords: usize = 0x12D0; // CUtlVector< int16 >
    pub const m_bBonusAttack: usize = 0x12E8; // bool
}

pub mod CDOTA_Modifier_Clinkz_Tar_Bomb_Slow {
    pub const slow_movement_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Clinkz_Tar_Bomb_Thinker {
    pub const radius: usize = 0x12C8; // int32
    pub const slow_duration: usize = 0x12CC; // float32
    pub const m_nFXIndex: usize = 0x12D0; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Clinkz_WindWalk {
    pub const move_speed_bonus_pct: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Cloak_Of_Flames_Debuff {
    pub const damage: usize = 0x12C8; // int32
    pub const damage_illusions: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Clockwerk_RocketFlare_Thinker {
    pub const radius: usize = 0x12C8; // float32
    pub const duration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Corpselord_Revive {
    pub const radius: usize = 0x12C8; // int32
    pub const ticks_to_revive: usize = 0x12CC; // int32
    pub const tick_interval: usize = 0x12D0; // float32
    pub const m_pReviveTarget: usize = 0x12D4; // CHandle< C_DOTA_BaseNPC >
    pub const m_nCurrentTicks: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Courier_Burst {
    pub const movement_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Courier_ReturnStashItems {
    pub const m_vLocation: usize = 0x12C8; // Vector
}

pub mod CDOTA_Modifier_Courier_TakeStashItems {
    pub const m_hStashOwner: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_vLocation: usize = 0x12CC; // Vector
    pub const m_bTransferAfterTake: usize = 0x12D8; // bool
    pub const stash_pickup_distance: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Courier_TransferItems {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Creature_Flamestrike {
    pub const impact_radius: usize = 0x12C8; // int32
    pub const ground_burn_duration: usize = 0x12CC; // float32
    pub const damage: usize = 0x12D0; // float32
    pub const vision_distance: usize = 0x12D4; // int32
    pub const vision_duration: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Creature_Flamestrike_Ground {
    pub const burn_interval: usize = 0x12C8; // float32
    pub const burn_radius: usize = 0x12CC; // int32
    pub const burn_dps: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Creature_Full_Avoidance {
    pub const m_flOriginalAvoidance: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Creature_HybridFlyer {
    pub const m_ctTimer: usize = 0x12C8; // CountdownTimer
}

pub mod CDOTA_Modifier_Creature_IceSlam_Thinker {
    pub const radius: usize = 0x12C8; // int32
    pub const m_nPathEffectIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Creep_Bonus_XP {
    pub const radius: usize = 0x12C8; // int32
    pub const hero_damage_penalty: usize = 0x12CC; // int32
    pub const bonus_gold: usize = 0x12D0; // int32
    pub const m_iGoldBounty: usize = 0x12D4; // int32
    pub const iBannerEffect: usize = 0x12D8; // ParticleIndex_t
    pub const iRingEffect: usize = 0x12DC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Creep_Irresolute {
    pub const hero_damage_penalty: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Creep_Piercing {
    pub const creep_damage_bonus: usize = 0x12C8; // int32
    pub const hero_damage_penalty: usize = 0x12CC; // int32
    pub const heavy_damage_penalty: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Creep_Siege {
    pub const bonus_building_damage: usize = 0x12C8; // int32
    pub const incoming_hero_damage_penalty: usize = 0x12CC; // int32
    pub const incoming_basic_damage_penalty: usize = 0x12D0; // int32
    pub const incoming_controlled_unit_penalty: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_CrystalMaiden_BrillianceAuraEffect {
    pub const base_mana_regen: usize = 0x12C8; // float32
    pub const self_factor: usize = 0x12CC; // float32
    pub const proximity_bonus_factor: usize = 0x12D0; // float32
    pub const proximity_bonus_radius: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_CrystalMaiden_CrystalNova {
    pub const movespeed_slow: usize = 0x12C8; // int32
    pub const attackspeed_slow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_CrystalMaiden_FreezingField {
    pub const radius: usize = 0x12C8; // int32
    pub const damage: usize = 0x12CC; // int32
    pub const bonus_armor: usize = 0x12D0; // int32
    pub const explosion_interval: usize = 0x12D4; // float32
    pub const shard_bonus_explosion: usize = 0x12D8; // float32
    pub const explosion_radius: usize = 0x12DC; // int32
    pub const slow_duration: usize = 0x12E0; // float32
    pub const explosion_min_dist: usize = 0x12E4; // int32
    pub const explosion_max_dist: usize = 0x12E8; // int32
    pub const frostbite_delay: usize = 0x12EC; // float32
    pub const m_fLastTick: usize = 0x12F0; // GameTime_t
    pub const m_fTimeAccumulator: usize = 0x12F4; // float32
    pub const m_iExplosionCount: usize = 0x12F8; // int32
    pub const m_iExplosionTotalCount: usize = 0x12FC; // int32
    pub const m_iExplosionQuadrant: usize = 0x1300; // int32
    pub const m_iExplosionDistance: usize = 0x1304; // int32
    pub const shard_self_movement_speed_slow_pct: usize = 0x1308; // int32
    pub const m_nFXIndex: usize = 0x130C; // ParticleIndex_t
    pub const m_hHitEntities: usize = 0x1318; // CUtlVector< CHandle< C_BaseEntity > >
    pub const can_move: usize = 0x1330; // int32
}

pub mod CDOTA_Modifier_CrystalMaiden_FreezingField_Slow {
    pub const movespeed_slow: usize = 0x12C8; // int32
    pub const attack_slow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_CrystalMaiden_Frostbite {
    pub const max_ticks: usize = 0x12C8; // int32
    pub const damage_per_second: usize = 0x12CC; // int32
    pub const creep_damage_per_second: usize = 0x12D0; // int32
    pub const tick_interval: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_CrystalMaiden_IceRink_Movement {
    pub const m_bHitFirstUpdate: usize = 0x12C8; // bool
    pub const flMovementSpeed: usize = 0x12CC; // float32
    pub const m_flTurnBoostProgress: usize = 0x12D0; // float32
    pub const m_flFacingTarget: usize = 0x12D4; // float32
    pub const m_nFXIndex: usize = 0x12D8; // ParticleIndex_t
    pub const m_flHitEndTime: usize = 0x12DC; // GameTime_t
    pub const hit_recover_time: usize = 0x12E0; // float32
    pub const m_vDirection: usize = 0x12E4; // Vector
}

pub mod CDOTA_Modifier_CrystalMaiden_IceRink_Thinker {
    pub const radius: usize = 0x12C8; // int32
    pub const rink_formation_time: usize = 0x12CC; // float32
    pub const m_bActive: usize = 0x12D0; // bool
}

pub mod CDOTA_Modifier_CrystalMaiden_Let_It_Go_Thinker {
    pub const m_vPathStart: usize = 0x12C8; // Vector
    pub const m_vPathEnd: usize = 0x12D4; // Vector
}

pub mod CDOTA_Modifier_Crystal_Maiden_Crystal_Clone_Statue {
    pub const frostbite_radius: usize = 0x12C8; // int32
    pub const anim_delay: usize = 0x12CC; // float32
    pub const m_bActivated: usize = 0x12D0; // bool
    pub const clone_health: usize = 0x12D4; // int32
    pub const m_flDamageTaken: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Cyclone {
    pub const m_flStartTime: usize = 0x12C8; // GameTime_t
}

pub mod CDOTA_Modifier_DaggerOfRistul_Buff {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_DarkSeer_IonShell_IllusionInvulnerability {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_DarkSeer_Normal_Punch {
    pub const m_PositionIndex: usize = 0x12C8; // Vector[30]
    pub const m_nNormalPunchBuffIndex: usize = 0x1430; // ParticleIndex_t
    pub const m_flDistanceTraveled: usize = 0x1434; // float32
    pub const m_bIsValidTarget: usize = 0x1438; // bool
}

pub mod CDOTA_Modifier_DarkSeer_Normal_Punch_Illusion {
    pub const replica_scale: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_DarkSeer_Normal_Punch_Illusion_Thinker {
    pub const m_vecIllusionSpawnPosition: usize = 0x12C8; // Vector
    pub const speed: usize = 0x12D4; // int32
    pub const m_hTarget: usize = 0x12D8; // CHandle< C_BaseEntity >
    pub const m_nFXIndex: usize = 0x12DC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_DarkSeer_Surge {
    pub const speed_boost: usize = 0x12C8; // int32
    pub const trail_radius: usize = 0x12CC; // int32
    pub const trail_duration: usize = 0x12D0; // float32
    pub const m_vLastTrailThinkerLocation: usize = 0x12D4; // Vector
    pub const m_bTrailStarted: usize = 0x12E0; // bool
}

pub mod CDOTA_Modifier_DarkSeer_Surge_Trail {
    pub const trail_move_slow: usize = 0x12C8; // int32
    pub const trail_damage: usize = 0x12CC; // int32
    pub const trail_damage_interval: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_DarkSeer_Surge_Trail_Thinker {
    pub const trail_radius: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_DarkSeer_Vacuum {
    pub const pull_speed: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_DarkSeer_WallOfReplica {
    pub const m_vWallDirection: usize = 0x12C8; // Vector
    pub const m_vWallRight: usize = 0x12D4; // Vector
    pub const m_PreventReplicateTime: usize = 0x12F8; // CUtlMap< int32, GameTime_t >
    pub const width: usize = 0x1318; // int32
    pub const slow_duration: usize = 0x131C; // float32
    pub const replica_damage_incoming: usize = 0x1320; // int32
    pub const replica_damage_outgoing: usize = 0x1324; // int32
}

pub mod CDOTA_Modifier_DarkSeer_WallOfReplica_Illusion {
    pub const replica_scale: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_DarkSeer_WallOfReplica_Slow {
    pub const movement_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_DarkWillow_Bedlam {
    pub const m_flRotation: usize = 0x12C8; // float32
    pub const m_hWisp: usize = 0x12CC; // CHandle< C_BaseEntity >
    pub const m_flLastAttack: usize = 0x12D0; // GameTime_t
    pub const roaming_radius: usize = 0x12D4; // int32
    pub const attack_radius: usize = 0x12D8; // int32
    pub const roaming_seconds_per_rotation: usize = 0x12DC; // float32
    pub const attack_interval: usize = 0x12E0; // float32
    pub const target_count: usize = 0x12E4; // int32
    pub const m_bTravelling: usize = 0x12E8; // bool
    pub const travel_speed: usize = 0x12EC; // int32
}

pub mod CDOTA_Modifier_DarkWillow_BrambleMaze_Creation_Thinker {
    pub const latch_creation_interval: usize = 0x12C8; // float32
    pub const initial_creation_delay: usize = 0x12CC; // float32
    pub const placement_count: usize = 0x12D0; // int32
    pub const placement_duration: usize = 0x12D4; // float32
    pub const m_iBramblesIndex: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_DarkWillow_BrambleMaze_Thinker {
    pub const latch_range: usize = 0x12C8; // int32
    pub const latch_vision: usize = 0x12CC; // int32
    pub const latch_creation_delay: usize = 0x12D0; // float32
    pub const latch_duration: usize = 0x12D4; // float32
    pub const m_bActive: usize = 0x12E0; // bool
}

pub mod CDOTA_Modifier_DarkWillow_CursedCrown {
    pub const delay: usize = 0x12C8; // float32
    pub const stun_duration: usize = 0x12CC; // float32
    pub const stun_radius: usize = 0x12D0; // int32
    pub const m_fStartTime: usize = 0x12D4; // GameTime_t
}

pub mod CDOTA_Modifier_DarkWillow_Debuff_Fear {
    pub const m_bDidGiveOrder: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_DarkWillow_ShadowRealm_Buff {
    pub const damage: usize = 0x12C8; // int32
    pub const max_damage_duration: usize = 0x12CC; // float32
    pub const attack_range_bonus: usize = 0x12D0; // int32
    pub const m_InFlightAttackRecords: usize = 0x12D8; // CUtlVector< int16 >
    pub const bAttackRange: usize = 0x12F0; // bool
    pub const m_flStartTime: usize = 0x12F4; // GameTime_t
    pub const m_flFadeTime: usize = 0x12F8; // GameTime_t
    pub const m_flDamageScale: usize = 0x12FC; // float32
    pub const duration: usize = 0x1300; // float32
}

pub mod CDOTA_Modifier_DarkWillow_ShadowRealm_Buff_Attack_Logic {
    pub const damage: usize = 0x12C8; // int32
    pub const m_iRecord: usize = 0x12CC; // int32
    pub const m_flDamageScale: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_DarkWillow_Terrorize_Thinker {
    pub const m_hWisp: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const destination_travel_speed: usize = 0x12CC; // int32
    pub const return_travel_speed: usize = 0x12D0; // int32
    pub const destination_radius: usize = 0x12D4; // int32
    pub const destination_status_duration: usize = 0x12D8; // float32
    pub const initial_delay: usize = 0x12DC; // float32
    pub const starting_height: usize = 0x12E0; // float32
    pub const m_vAttackLocation: usize = 0x12E4; // Vector
    pub const m_bAttacking: usize = 0x12F0; // bool
    pub const m_bReturning: usize = 0x12F1; // bool
    pub const m_fCurHeight: usize = 0x12F4; // float32
    pub const m_fEstimatedTravelTime: usize = 0x12F8; // float32
    pub const think_interval: usize = 0x12FC; // float32
    pub const m_bInFlight: usize = 0x1300; // bool
}

pub mod CDOTA_Modifier_DataDriven {
    pub const m_nAbilityID: usize = 0x12C8; // AbilityID_t
    pub const m_nIndexInAbility: usize = 0x12CC; // int32
    pub const m_bIsHidden: usize = 0x12D0; // bool
    pub const m_bIsDebuff: usize = 0x12D1; // bool
    pub const m_bIsPurgable: usize = 0x12D2; // bool
    pub const m_bIsStunDebuff: usize = 0x12D3; // bool
    pub const m_bAllowIllusionDuplicate: usize = 0x12D4; // bool
    pub const m_bRemoveOnDeath: usize = 0x12D5; // bool
    pub const m_pszAuraModifier: usize = 0x12D8; // char*
    pub const m_nAuraModifierIndex: usize = 0x12E0; // int32
    pub const m_nAuraRadius: usize = 0x12E4; // int32
    pub const m_nAuraSearchTeam: usize = 0x12E8; // int32
    pub const m_nAuraSearchType: usize = 0x12EC; // int32
    pub const m_nAuraSearchFlags: usize = 0x12F0; // int32
    pub const m_bAuraApplyToCaster: usize = 0x12F4; // bool
    pub const m_bAuraApplyToParent: usize = 0x12F5; // bool
    pub const m_nAttributes: usize = 0x12F8; // int32
    pub const m_pszEffectName: usize = 0x1300; // char*
    pub const m_nEffectAttachType: usize = 0x1308; // int32
    pub const m_nPriority: usize = 0x130C; // int32
    pub const m_pszStatusEffectName: usize = 0x1310; // char*
    pub const m_nStatusEffectPriority: usize = 0x1318; // int32
    pub const m_nOverrideAnimation: usize = 0x131C; // int32
    pub const m_nOrbPriority: usize = 0x1320; // int32
    pub const m_nOrbLabel: usize = 0x1324; // int32
    pub const m_bOrbIsCastAttack: usize = 0x1328; // bool
    pub const m_flThinkInterval: usize = 0x132C; // float32
    pub const m_bNeedsReParseIfRefreshed: usize = 0x1330; // bool
    pub const m_pszModelName: usize = 0x1338; // char*
    pub const m_pszOrbProjectileName: usize = 0x1340; // char*
    pub const m_nModifierStatesEnabled: usize = 0x3728; // uint64
    pub const m_nModifierStatesDisabled: usize = 0x3730; // uint64
    pub const m_pOnCreated: usize = 0x3738; // KeyValues*
    pub const m_pOnIntervalThink: usize = 0x3740; // KeyValues*
    pub const m_pOnDestroy: usize = 0x3748; // KeyValues*
    pub const m_pOnAttackStart: usize = 0x3750; // KeyValues*
    pub const m_pOnAttack: usize = 0x3758; // KeyValues*
    pub const m_pOnAttackLanded: usize = 0x3760; // KeyValues*
    pub const m_pOnAttackFailed: usize = 0x3768; // KeyValues*
    pub const m_pOnAttackAllied: usize = 0x3770; // KeyValues*
    pub const m_pOnProjectileDodge: usize = 0x3778; // KeyValues*
    pub const m_pOnOrder: usize = 0x3780; // KeyValues*
    pub const m_pOnUnitMoved: usize = 0x3788; // KeyValues*
    pub const m_pOnAbilityStart: usize = 0x3790; // KeyValues*
    pub const m_pOnAbilityExecuted: usize = 0x3798; // KeyValues*
    pub const m_pOnBreakInvisibility: usize = 0x37A0; // KeyValues*
    pub const m_pOnAbilityEndChannel: usize = 0x37A8; // KeyValues*
    pub const m_pOnTakeDamage: usize = 0x37B0; // KeyValues*
    pub const m_pOnDealDamage: usize = 0x37B8; // KeyValues*
    pub const m_pOnStateChanged: usize = 0x37C0; // KeyValues*
    pub const m_pOnAttacked: usize = 0x37C8; // KeyValues*
    pub const m_pOnDeath: usize = 0x37D0; // KeyValues*
    pub const m_pOnKill: usize = 0x37D8; // KeyValues*
    pub const m_pOnRespawn: usize = 0x37E0; // KeyValues*
    pub const m_pOnSpentMana: usize = 0x37E8; // KeyValues*
    pub const m_pOnTeleporting: usize = 0x37F0; // KeyValues*
    pub const m_pOnTeleported: usize = 0x37F8; // KeyValues*
    pub const m_pOnHealthGained: usize = 0x3800; // KeyValues*
    pub const m_pOnManaGained: usize = 0x3808; // KeyValues*
    pub const m_pOnHeroKilled: usize = 0x3810; // KeyValues*
    pub const m_pOnHealReceived: usize = 0x3818; // KeyValues*
    pub const m_pOnAttackFinished: usize = 0x3820; // KeyValues*
    pub const m_pOnAttackCancelled: usize = 0x3828; // KeyValues*
}

pub mod CDOTA_Modifier_Dawnbreaker_Celestial_Hammer_Movement {
    pub const m_nProjectileID: usize = 0x12C8; // int32
    pub const projectile_speed: usize = 0x12CC; // int32
    pub const travel_speed_pct: usize = 0x12D0; // int32
    pub const m_nMaxRange: usize = 0x12D4; // int32
    pub const m_vStartPoint: usize = 0x12D8; // Vector
    pub const m_vLastTrailThinkerLocation: usize = 0x12E4; // Vector
    pub const m_vEndPointShard: usize = 0x12F0; // Vector
    pub const m_nStatusFXIndex: usize = 0x12FC; // ParticleIndex_t
    pub const flare_radius: usize = 0x1300; // int32
    pub const bHasStartedBurning: usize = 0x1304; // bool
    pub const flare_debuff_duration: usize = 0x1308; // float32
}

pub mod CDOTA_Modifier_Dawnbreaker_Celestial_Hammer_Thinker {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Dawnbreaker_Converge {
    pub const move_slow: usize = 0x12C8; // int32
    pub const burn_damage: usize = 0x12CC; // int32
    pub const burn_interval: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Dawnbreaker_Converge_Thinker {
    pub const flare_radius: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Dawnbreaker_Fire_Wreath_Attack_Bonus {
    pub const attack_damage: usize = 0x12C8; // int32
    pub const creep_damage_penalty: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Dawnbreaker_Fire_Wreath_Caster {
    pub const movement_speed: usize = 0x12C8; // int32
    pub const shard_movement_penalty: usize = 0x12CC; // int32
    pub const swipe_radius: usize = 0x12D0; // int32
    pub const swipe_damage: usize = 0x12D4; // int32
    pub const smash_radius: usize = 0x12D8; // int32
    pub const smash_damage: usize = 0x12DC; // int32
    pub const m_nCurrentSpeed: usize = 0x12E0; // int32
    pub const flSwipeInterval: usize = 0x12E4; // float32
    pub const m_flNextHit: usize = 0x12E8; // GameTime_t
    pub const iCurrentAttack: usize = 0x12EC; // int32
    pub const total_attacks: usize = 0x12F0; // int32
    pub const smash_stun_duration: usize = 0x12F4; // float32
    pub const sweep_stun_duration: usize = 0x12F8; // float32
    pub const m_bHasCompletedMove: usize = 0x12FC; // bool
    pub const duration: usize = 0x1300; // float32
    pub const m_vTargetHorizontalDirection: usize = 0x1304; // Vector
    pub const smash_distance_from_hero: usize = 0x1310; // int32
    pub const animation_rate: usize = 0x1314; // float32
    pub const turn_rate: usize = 0x1318; // float32
    pub const m_flFacingTarget: usize = 0x131C; // float32
}

pub mod CDOTA_Modifier_Dawnbreaker_Fire_Wreath_Slow {
    pub const swipe_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Dawnbreaker_Fire_Wreath_Smash_Stun {
    pub const damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Dawnbreaker_Luminosity {
    pub const m_bAppliesToCreeps: usize = 0x12C8; // bool
    pub const attack_count: usize = 0x12CC; // int32
    pub const m_bShouldIncrement: usize = 0x12D0; // bool
}

pub mod CDOTA_Modifier_Dawnbreaker_Luminosity_Attack_Buff {
    pub const m_bIsBuffedAttack: usize = 0x12C8; // bool
    pub const heal_pct: usize = 0x12CC; // int32
    pub const bonus_damage: usize = 0x12D0; // int32
    pub const heal_radius: usize = 0x12D4; // int32
    pub const heal_from_creeps: usize = 0x12D8; // int32
    pub const allied_healing_pct: usize = 0x12DC; // int32
    pub const hTargets: usize = 0x12E0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_flTotalToHeal: usize = 0x12F8; // float32
}

pub mod CDOTA_Modifier_Dawnbreaker_Solar_Guardian_Disable {
    pub const m_nCasterFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const m_nStatusFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Dawnbreaker_Solar_Guardian_Evasion {
    pub const miss_rate: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Dawnbreaker_Solar_Guardian_Thinker {
    pub const pulse_interval: usize = 0x12C8; // float32
    pub const iEffectRadius: usize = 0x12CC; // int32
    pub const base_damage: usize = 0x12D0; // int32
    pub const base_heal: usize = 0x12D4; // int32
    pub const m_nThinkerFXIndex: usize = 0x12D8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Dazzle_Bad_Juju_Armor {
    pub const armor_reduction: usize = 0x12C8; // float32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Dazzle_Bad_Juju_Armor_Counter {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const armor_reduction: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Dazzle_Bad_Juju_Manacost {
    pub const mana_cost_increase_duration: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Dazzle_Good_Juju {
    pub const cooldown_reduction: usize = 0x12C8; // float32
    pub const item_cooldown_reduction: usize = 0x12CC; // float32
    pub const duration: usize = 0x12D0; // float32
    pub const radius: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Dazzle_Poison_Touch {
    pub const slow: usize = 0x12C8; // int32
    pub const bonus_slow: usize = 0x12CC; // float32
    pub const bonus_damage: usize = 0x12D0; // int32
    pub const flAccummulatedBonusSlow: usize = 0x12D4; // float32
    pub const iAccummulatedBonusDamage: usize = 0x12D8; // int32
    pub const attack_range_bonus: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Dazzle_Poison_Touch_Self {
    pub const attack_range_bonus: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Dazzle_Rain_Of_Vermin {
    pub const radius: usize = 0x12C8; // int32
    pub const interval: usize = 0x12CC; // float32
    pub const hex_chance: usize = 0x12D0; // float32
    pub const hex_duration: usize = 0x12D4; // float32
    pub const damage: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Dazzle_Shallow_Grave {
    pub const heal_amplify: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Dazzle_Weave_Armor {
    pub const armor_per_second: usize = 0x12C8; // float32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_DeathProphet_CryptSwarm_Slow {
    pub const slow_percent: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_DeathProphet_Exorcism {
    pub const radius: usize = 0x12C8; // int32
    pub const spirit_speed: usize = 0x12CC; // int32
    pub const max_distance: usize = 0x12D0; // int32
    pub const give_up_distance: usize = 0x12D4; // int32
    pub const min_damage: usize = 0x12D8; // int32
    pub const max_damage: usize = 0x12DC; // int32
    pub const heal_percent: usize = 0x12E0; // int32
    pub const spirit_duration: usize = 0x12E4; // int32
    pub const ghost_spawn_rate: usize = 0x12E8; // float32
    pub const movement_bonus: usize = 0x12EC; // int32
    pub const m_iSpirits: usize = 0x12F0; // int32
    pub const m_fStartTime: usize = 0x12F4; // GameTime_t
    pub const m_fLastThinkTime: usize = 0x12F8; // GameTime_t
    pub const m_fSpawnTime: usize = 0x12FC; // GameTime_t
    pub const m_bFirstSpawn: usize = 0x1300; // bool
    pub const m_bCommentedOnExpired: usize = 0x1301; // bool
    pub const m_bForceExpired: usize = 0x1302; // bool
    pub const m_vecSpirits: usize = 0x1308; // CUtlVector< sSpiritInfo* >
}

pub mod CDOTA_Modifier_DeathProphet_Scepter {
    pub const radius: usize = 0x12C8; // int32
    pub const spirit_speed: usize = 0x12CC; // int32
    pub const max_distance: usize = 0x12D0; // int32
    pub const give_up_distance: usize = 0x12D4; // int32
    pub const min_damage: usize = 0x12D8; // int32
    pub const max_damage: usize = 0x12DC; // int32
    pub const heal_percent: usize = 0x12E0; // int32
    pub const spirit_duration: usize = 0x12E4; // int32
    pub const m_fStartTime: usize = 0x12E8; // GameTime_t
    pub const m_fLastThinkTime: usize = 0x12EC; // GameTime_t
    pub const m_bExpired: usize = 0x12F0; // bool
    pub const m_SpiritInfo: usize = 0x12F8; // sSpiritInfo*
}

pub mod CDOTA_Modifier_DeathProphet_Slow {
    pub const speed_bonus: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_DeathProphet_SpiritSiphon {
    pub const flSmoothness: usize = 0x12C8; // float32
    pub const damage: usize = 0x12CC; // float32
    pub const damage_pct: usize = 0x12D0; // float32
    pub const drain_range: usize = 0x12D4; // int32
    pub const haunt_duration: usize = 0x12D8; // float32
    pub const siphon_buffer: usize = 0x12DC; // int32
    pub const movement_steal: usize = 0x12E0; // int32
    pub const m_hTarget: usize = 0x12E4; // CHandle< C_BaseEntity >
    pub const m_iLinkIndex: usize = 0x12E8; // ParticleIndex_t
    pub const m_nSelfBuffSerialNumber: usize = 0x12EC; // int32
    pub const m_nTargetDebuffSerialNumber: usize = 0x12F0; // int32
    pub const m_bAppliedFear: usize = 0x12F4; // bool
    pub const shard_fear_duration: usize = 0x12F8; // float32
    pub const shard_consecutive_siphon_duration: usize = 0x12FC; // float32
}

pub mod CDOTA_Modifier_DeathProphet_SpiritSiphon_Fear {
    pub const m_vOriginal: usize = 0x12C8; // Vector
}

pub mod CDOTA_Modifier_DeathProphet_SpiritSiphon_Slow {
    pub const movement_steal: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_DeathProphet_Witchcraft {
    pub const bonus_movement_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Demonicon_Bonus {
    pub const hp_bonus: usize = 0x12C8; // int32
    pub const damage_bonus: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Desolator_2_Corruption {
    pub const corruption_armor: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Desolator_Corruption {
    pub const corruption_armor: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Diretide_Portal_Buff {
    pub const m_hInvader: usize = 0x12C8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Disruptor_Glimpse {
    pub const m_PositionIndex: usize = 0x12C8; // Vector[18]
}

pub mod CDOTA_Modifier_Disruptor_Glimpse_Thinker {
    pub const m_vStartLoc: usize = 0x12C8; // Vector
    pub const m_hEntity: usize = 0x12D4; // CHandle< C_BaseEntity >
    pub const m_flDistance: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Disruptor_KineticField {
    pub const radius: usize = 0x12C8; // int32
    pub const m_vOriginLoc: usize = 0x12CC; // Vector
    pub const m_bTruesight: usize = 0x12D8; // bool
    pub const damage_per_second: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Disruptor_KineticFieldThinker {
    pub const radius: usize = 0x12C8; // int32
    pub const formation_time: usize = 0x12CC; // float32
    pub const m_bActive: usize = 0x12D0; // bool
    pub const m_nFXIndex: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Disruptor_StaticStorm {
    pub const miss_rate: usize = 0x12C8; // int32
    pub const movement_speed_reduction: usize = 0x12CC; // int32
    pub const attack_speed_reduction: usize = 0x12D0; // int32
    pub const m_bHasScepter: usize = 0x12D4; // bool
}

pub mod CDOTA_Modifier_Disruptor_StaticStormThinker {
    pub const m_nCurrentPulse: usize = 0x12C8; // int32
    pub const radius: usize = 0x12CC; // int32
    pub const damage_max: usize = 0x12D0; // int32
    pub const pulses: usize = 0x12D4; // int32
    pub const duration: usize = 0x12D8; // float32
    pub const m_nFXIndex: usize = 0x12DC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Disruptor_Thunder_Strike {
    pub const strike_interval: usize = 0x12C8; // float32
    pub const radius: usize = 0x12CC; // int32
    pub const strike_damage: usize = 0x12D0; // int32
    pub const strike_damage_bonus: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Disruptor_Thunder_Strike_Slow {
    pub const slow_amount: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Disruptor_Thunder_Strike_Speed {
    pub const shard_movement_speed_bonus: usize = 0x12C8; // int32
    pub const shard_attack_speed_bonus: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_DoomBringer_Devour {
    pub const armor: usize = 0x12C8; // int32
    pub const magic_resist: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_DoomBringer_Devour_Intrinsic {
    pub const cast_range_bonus: usize = 0x12C8; // int32
    pub const spell_amp_bonus: usize = 0x12CC; // int32
    pub const cooldown_bonus: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_DoomBringer_Doom {
    pub const damage: usize = 0x12C8; // int32
    pub const duration: usize = 0x12CC; // float32
    pub const deniable_pct: usize = 0x12D0; // int32
    pub const flElapsedTime: usize = 0x12D4; // float32
    pub const damage_amp: usize = 0x12D8; // int32
    pub const m_bDoesBreak: usize = 0x12DC; // bool
    pub const m_bDoesMute: usize = 0x12DD; // bool
}

pub mod CDOTA_Modifier_DoomBringer_Doom_Aura_Enemy {
    pub const scepter_aura_radius: usize = 0x12C8; // int32
    pub const damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_DoomBringer_Doom_Aura_Self {
    pub const scepter_aura_radius: usize = 0x12C8; // int32
    pub const damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_DoomBringer_InfernalBlade {
    pub const m_bInfernalBladeAttack: usize = 0x12C8; // bool
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
    pub const burn_duration: usize = 0x12D0; // float32
    pub const ministun_duration: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_DoomBringer_InfernalBlade_Burn {
    pub const burn_damage: usize = 0x12C8; // int32
    pub const burn_damage_pct: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_DoomBringer_ScorchedEarthEffect {
    pub const bonus_movement_speed_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_DoomBringer_ScorchedEarthEffectAura {
    pub const damage_per_second: usize = 0x12C8; // int32
    pub const bonus_movement_speed_pct: usize = 0x12CC; // int32
    pub const radius: usize = 0x12D0; // int32
    pub const heal_pct: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_DragonKnight_BreatheFire_Reduction {
    pub const reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_DragonKnight_CorrosiveBreath {
    pub const corrosive_breath_duration: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_DragonKnight_CorrosiveBreathDoT {
    pub const corrosive_breath_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_DragonKnight_DragonBlood {
    pub const bonus_health_regen: usize = 0x12C8; // int32
    pub const bonus_armor: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_DragonKnight_DragonBlood_Aura {
    pub const aura_radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_DragonKnight_DragonForm {
    pub const bonus_movement_speed: usize = 0x12C8; // int32
    pub const bonus_attack_damage: usize = 0x12CC; // int32
    pub const bonus_attack_range: usize = 0x12D0; // int32
    pub const attack_projectile_speed_bonus: usize = 0x12D4; // int32
    pub const magic_resistance: usize = 0x12D8; // int32
    pub const model_scale: usize = 0x12DC; // int32
    pub const iLevel: usize = 0x12E0; // int32
    pub const m_iOriginalAttackCapabilities: usize = 0x12E4; // int32
    pub const m_iszRangedAttackEffect: usize = 0x12E8; // CUtlSymbolLarge
    pub const bonus_slow_resistance: usize = 0x12F0; // int32
}

pub mod CDOTA_Modifier_DragonKnight_Fireball_Burn {
    pub const burn_interval: usize = 0x12C8; // float32
    pub const damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_DragonKnight_Fireball_Thinker {
    pub const radius: usize = 0x12C8; // int32
    pub const linger_duration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_DragonKnight_FrostBreath {
    pub const frost_duration: usize = 0x12C8; // float32
    pub const frost_aoe: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_DragonKnight_FrostBreath_Slow {
    pub const frost_bonus_movement_speed: usize = 0x12C8; // int32
    pub const frost_bonus_attack_speed: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_DragonKnight_SplashAttack {
    pub const splash_radius: usize = 0x12C8; // int32
    pub const splash_damage_percent: usize = 0x12CC; // int32
    pub const corrosive_breath_duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Dragon_Scale_Burn {
    pub const damage_per_sec: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_DrowRanger_FrostArrows {
    pub const m_InFlightAttackRecords: usize = 0x12C8; // CUtlVector< int16 >
    pub const shard_bonus_damage_per_stack: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_DrowRanger_FrostArrows_Hypothermia {
    pub const shard_regen_reduction_pct_per_stack: usize = 0x12C8; // int32
    pub const shard_burst_radius: usize = 0x12CC; // int32
    pub const m_nFXStackIndex: usize = 0x12D0; // ParticleIndex_t
}

pub mod CDOTA_Modifier_DrowRanger_FrostArrows_Shard_Slow {
    pub const shard_burst_move_slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_DrowRanger_FrostArrows_Slow {
    pub const frost_arrows_movement_speed: usize = 0x12C8; // int32
    pub const shard_max_stacks: usize = 0x12CC; // int32
    pub const shard_stack_duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_DrowRanger_Marksmanship {
    pub const chance: usize = 0x12C8; // int32
    pub const m_nFxIndex: usize = 0x12CC; // ParticleIndex_t
    pub const m_InFlightAttackRecords: usize = 0x12D0; // CUtlVector< int16 >
    pub const m_GlacialInFlightAttackRecords: usize = 0x12E8; // CUtlVector< int16 >
    pub const bonus_factor: usize = 0x1300; // int32
    pub const disable_range: usize = 0x1304; // int32
    pub const agility_range: usize = 0x1308; // int32
    pub const bonus_damage: usize = 0x130C; // int32
    pub const split_count: usize = 0x1310; // int32
    pub const split_range: usize = 0x1314; // int32
    pub const m_bGlacialAttack: usize = 0x1318; // bool
}

pub mod CDOTA_Modifier_DrowRanger_Marksmanship_Aura_Bonus {
    pub const agility_multiplier: usize = 0x12C8; // int32
    pub const agility_multiplier_ally: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_DrowRanger_Marksmanship_Reduction {
    pub const split_damage_reduction: usize = 0x12C8; // int32
    pub const m_vecRecords: usize = 0x12D0; // CUtlVectorFixedGrowable< int32 >
    pub const m_vecRecordTimes: usize = 0x1308; // CUtlVectorFixedGrowable< GameTime_t >
}

pub mod CDOTA_Modifier_DrowRanger_TrueshotAura {
    pub const trueshot_ranged_attack_speed: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_DrowRanger_WaveOfSilence {
    pub const miss_chance: usize = 0x12C8; // int32
    pub const gust_reveals_invis: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_DrowRanger_WaveOfSilence_Buff {
    pub const bonus_movespeed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Drow_Ranger_Glacier_Hilltop {
    pub const multishot_arrow_bonus: usize = 0x12C8; // int32
    pub const end_height: usize = 0x12CC; // int32
    pub const attack_range_bonus: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Drow_Ranger_Glacier_Hilltop_Aura {
    pub const shard_width: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Drow_Ranger_Glacier_Hilltop_Removal {
    pub const z_speed_override: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Drow_Ranger_Multishot {
    pub const m_iArrowCount: usize = 0x12C8; // int32
    pub const m_vOriginalTarget: usize = 0x12CC; // Vector
    pub const arrow_width: usize = 0x12D8; // int32
    pub const arrow_speed: usize = 0x12DC; // float32
    pub const arrow_range_multiplier: usize = 0x12E0; // float32
    pub const wave_count: usize = 0x12E4; // int32
    pub const arrow_count_per_wave: usize = 0x12E8; // int32
    pub const arrow_angle: usize = 0x12EC; // int32
    pub const m_iLoopCount: usize = 0x12F0; // int32
    pub const m_flInterval: usize = 0x12F4; // float32
    pub const m_flCycleDelay: usize = 0x12F8; // float32
    pub const m_flTimeWaste: usize = 0x12FC; // float32
    pub const m_flExpectedTime: usize = 0x1300; // GameTime_t
}

pub mod CDOTA_Modifier_Drow_Ranger_Multishot_Damage {
    pub const arrow_damage_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_EarthSpirit_BoulderSmash {
    pub const m_bInterrupted: usize = 0x12C8; // bool
    pub const m_nProjectileID: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_EarthSpirit_Boulder_Smash_Debuff {
    pub const move_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_EarthSpirit_GeomagneticGrip {
    pub const radius: usize = 0x12C8; // int32
    pub const rock_damage: usize = 0x12CC; // int32
    pub const pull_units_per_second: usize = 0x12D0; // float32
    pub const pull_units_per_second_heroes: usize = 0x12D4; // float32
    pub const total_pull_distance: usize = 0x12D8; // float32
    pub const duration: usize = 0x12DC; // float32
    pub const m_bUsedStone: usize = 0x12E0; // bool
    pub const m_vDestination: usize = 0x12E4; // Vector
    pub const m_vLocation: usize = 0x12F0; // Vector
    pub const m_hHitEntities: usize = 0x1300; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_EarthSpirit_Magnetize {
    pub const rock_search_radius: usize = 0x12C8; // int32
    pub const damage_per_second: usize = 0x12CC; // int32
    pub const damage_interval: usize = 0x12D0; // float32
    pub const rock_explosion_delay: usize = 0x12D4; // float32
    pub const damage_duration: usize = 0x12D8; // float32
    pub const cast_radius: usize = 0x12DC; // int32
    pub const rock_explosion_radius: usize = 0x12E0; // int32
    pub const m_hExplodedRocks: usize = 0x12E8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const duration: usize = 0x1300; // float32
    pub const m_bShowOverhead: usize = 0x1304; // bool
}

pub mod CDOTA_Modifier_EarthSpirit_Petrify {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const m_bHasBeenMagnetized: usize = 0x12CC; // bool
}

pub mod CDOTA_Modifier_EarthSpirit_Polarization {
    pub const damage_per_second: usize = 0x12C8; // int32
    pub const damage_interval: usize = 0x12CC; // float32
    pub const damage_duration: usize = 0x12D0; // float32
    pub const rock_search_radius: usize = 0x12D4; // int32
    pub const m_hMagnetizeAbility: usize = 0x12D8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_EarthSpirit_RollingBoulder_Caster {
    pub const m_vDir: usize = 0x12C8; // Vector
    pub const m_bBoulderCreated: usize = 0x12D4; // bool
}

pub mod CDOTA_Modifier_EarthSpirit_StoneThinker {
    pub const m_pVisionRangeFX: usize = 0x12C8; // CNewParticleEffect*
    pub const vision_radius: usize = 0x12D0; // int32
    pub const m_bHasBeenMagnetized: usize = 0x12D4; // bool
    pub const m_nFXIndex: usize = 0x12D8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Earthshaker_Aftershock {
    pub const aftershock_range: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Earthshaker_Arcana {
    pub const m_nComboCount: usize = 0x12C8; // int32
    pub const m_flComboWindow: usize = 0x12CC; // float32
    pub const m_flComboDisplayInterval: usize = 0x12D0; // float32
    pub const m_flDamageDone: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Earthshaker_Arcana_Kill_Effect {
    pub const m_bFrozen: usize = 0x12C8; // bool
    pub const m_bFlail: usize = 0x12C9; // bool
}

pub mod CDOTA_Modifier_Earthshaker_EnchantTotem {
    pub const bonus_attack_range: usize = 0x12C8; // int32
    pub const totem_damage_percentage: usize = 0x12CC; // int32
    pub const scepter_cleave_pct: usize = 0x12D0; // int32
    pub const scepter_cleave_starting_width: usize = 0x12D4; // float32
    pub const scepter_cleave_ending_width: usize = 0x12D8; // float32
    pub const scepter_cleave_distance: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Earthshaker_Fissure_Shard {
    pub const shard_aftershock_stun_duration_pct: usize = 0x12C8; // int32
    pub const shard_free_pathing_linger_duration: usize = 0x12CC; // float32
    pub const m_vStartPos: usize = 0x12D0; // Vector
    pub const m_vEndPos: usize = 0x12DC; // Vector
    pub const m_flEndTime: usize = 0x12E8; // GameTime_t
}

pub mod CDOTA_Modifier_EchoSabre_Debuff {
    pub const movement_slow: usize = 0x12C8; // int32
    pub const attack_speed_slow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Elder_Titan_AncestralSpirit {
    pub const pass_damage: usize = 0x12C8; // int32
    pub const radius: usize = 0x12CC; // int32
    pub const m_nCreepsHit: usize = 0x12D0; // int32
    pub const m_nHeroesHit: usize = 0x12D4; // int32
    pub const m_flSpeed: usize = 0x12D8; // float32
    pub const m_hHitUnits: usize = 0x12E0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_nBonusMoveSpeed: usize = 0x12F8; // int32
}

pub mod CDOTA_Modifier_Elder_Titan_AncestralSpirit_Buff {
    pub const move_pct_creeps: usize = 0x12C8; // float32
    pub const move_pct_heroes: usize = 0x12CC; // int32
    pub const damage_creeps: usize = 0x12D0; // int32
    pub const damage_heroes: usize = 0x12D4; // int32
    pub const armor_creeps: usize = 0x12D8; // float32
    pub const armor_heroes: usize = 0x12DC; // float32
    pub const move_pct_cap: usize = 0x12E0; // float32
    pub const m_nCreepsHit: usize = 0x12E4; // int32
    pub const m_nHeroesHit: usize = 0x12E8; // int32
    pub const m_fSpeedPercentage: usize = 0x12EC; // float32
    pub const m_nDamage: usize = 0x12F0; // int32
    pub const m_nArmor: usize = 0x12F4; // int32
    pub const m_bSpellImmunity: usize = 0x12F8; // bool
}

pub mod CDOTA_Modifier_Elder_Titan_EarthSplitter {
    pub const slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Elder_Titan_EarthSplitter_Caster {
    pub const m_hHitUnits: usize = 0x12C8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const vision_width: usize = 0x12E0; // int32
    pub const vision_interval: usize = 0x12E4; // float32
    pub const vision_duration: usize = 0x12E8; // float32
    pub const vision_step: usize = 0x12EC; // int32
    pub const total_steps: usize = 0x12F0; // int32
    pub const m_nCompletedSteps: usize = 0x12F4; // int32
    pub const m_vStart: usize = 0x12F8; // Vector
    pub const m_vEnd: usize = 0x1304; // Vector
    pub const m_vNextVisionLocation: usize = 0x1310; // Vector
    pub const m_vCastDirection: usize = 0x131C; // Vector
}

pub mod CDOTA_Modifier_Elder_Titan_EarthSplitter_Thinker {
    pub const crack_width: usize = 0x12C8; // int32
    pub const damage_pct: usize = 0x12CC; // int32
    pub const slow_duration: usize = 0x12D0; // float32
    pub const slow_duration_scepter: usize = 0x12D4; // float32
    pub const m_pParentModifier: usize = 0x12D8; // CDOTA_Buff*
}

pub mod CDOTA_Modifier_Elder_Titan_EchoStomp {
    pub const wake_damage_limit: usize = 0x12C8; // int32
    pub const animation_rate: usize = 0x12CC; // float32
    pub const initial_stun_duration: usize = 0x12D0; // float32
    pub const ctStunTimer: usize = 0x12D8; // CountdownTimer
    pub const m_flDamageTaken: usize = 0x12F0; // float32
}

pub mod CDOTA_Modifier_Elder_Titan_EchoStomp_Magic_Immune {
    pub const model_scale: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Elder_Titan_NaturalOrder_Armor {
    pub const armor_reduction_pct: usize = 0x12C8; // int32
    pub const m_flArmorReduction: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Elder_Titan_NaturalOrder_Aura_Armor {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Elder_Titan_NaturalOrder_Aura_MagicResistance {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Elder_Titan_NaturalOrder_MagicResistance {
    pub const magic_resistance_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_ElixerHealing {
    pub const health: usize = 0x12C8; // int32
    pub const mana: usize = 0x12CC; // int32
    pub const duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_EmberSpirit_FireRemnantThinker {
    pub const m_nProjectileHandle: usize = 0x12C8; // int32
    pub const m_bIsShardRemnant: usize = 0x12CC; // bool
}

pub mod CDOTA_Modifier_EmberSpirit_FireRemnant_RemnantTracker {
    pub const m_vActiveRemnants: usize = 0x12C8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_vTrackingDelayedRemnants: usize = 0x12E0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const shard_charge_radius: usize = 0x12F8; // int32
}

pub mod CDOTA_Modifier_EmberSpirit_FireRemnant_Shard {
    pub const shard_radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_EmberSpirit_FireRemnant_Shard_Debuff {
    pub const shard_damage_per_second: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_EmberSpirit_FlameGuard {
    pub const absorb_amount: usize = 0x12C8; // int32
    pub const damage_per_second: usize = 0x12CC; // int32
    pub const radius: usize = 0x12D0; // int32
    pub const tick_interval: usize = 0x12D4; // float32
    pub const shield_pct_absorb: usize = 0x12D8; // int32
    pub const m_nAbsorbRemaining: usize = 0x12DC; // int32
    pub const m_flShowParticleInterval: usize = 0x12E0; // float32
    pub const m_bDestroy: usize = 0x12E4; // bool
}

pub mod CDOTA_Modifier_EmberSpirit_FlameGuard_Debuff {
    pub const blind_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_EmberSpirit_SearingChains {
    pub const damage_per_second: usize = 0x12C8; // int32
    pub const tick_interval: usize = 0x12CC; // float32
    pub const tick_damage: usize = 0x12D0; // float32
    pub const duration: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_EmberSpirit_SleightOfFist_Caster {
    pub const bonus_hero_damage: usize = 0x12C8; // int32
    pub const creep_damage_penalty: usize = 0x12CC; // int32
    pub const radius: usize = 0x12D0; // int32
    pub const attack_interval: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Enchantress_Enchant_Controlled {
    pub const bonus_movement_speed: usize = 0x12C8; // int32
    pub const enchant_health: usize = 0x12CC; // int32
    pub const enchant_armor: usize = 0x12D0; // int32
    pub const enchant_damage: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Enchantress_Enchant_Slow {
    pub const slow_movement_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Enchantress_Impetus {
    pub const distance_damage_pct: usize = 0x12C8; // float32
    pub const distance_cap: usize = 0x12CC; // int32
    pub const creep_multiplier: usize = 0x12D0; // float32
    pub const m_InFlightAttackRecords: usize = 0x12D8; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Enchantress_Little_Friends {
    pub const m_hZombieTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_hDesiredTarget: usize = 0x12CC; // CHandle< C_BaseEntity >
    pub const damage_reduction: usize = 0x12D0; // int32
    pub const bonus_attack_speed: usize = 0x12D4; // int32
    pub const bonus_move_speed: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Enchantress_Little_Friends_Aura {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_nAlliesTaunted: usize = 0x12CC; // int32
    pub const m_bRelicTriggered: usize = 0x12D0; // bool
    pub const radius: usize = 0x12D4; // int32
    pub const damage_amplification: usize = 0x12D8; // int32
    pub const damage_reduction: usize = 0x12DC; // int32
    pub const root_base_duration: usize = 0x12E0; // float32
    pub const root_per_target: usize = 0x12E4; // float32
    pub const max_root: usize = 0x12E8; // float32
    pub const m_flLastSeen: usize = 0x12EC; // GameTime_t
}

pub mod CDOTA_Modifier_Enchantress_Little_Friends_Kill_Credit {
    pub const m_InFlightAttackRecords: usize = 0x12C8; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Enchantress_NaturesAttendants {
    pub const heal_interval: usize = 0x12C8; // float32
    pub const heal: usize = 0x12CC; // float32
    pub const radius: usize = 0x12D0; // int32
    pub const wisp_count: usize = 0x12D4; // int32
    pub const movespeed: usize = 0x12D8; // int32
    pub const m_nWispFXIndex: usize = 0x12DC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Enchantress_NaturesAttendants_Shard {
    pub const heal_interval: usize = 0x12C8; // float32
    pub const heal: usize = 0x12CC; // float32
    pub const radius: usize = 0x12D0; // int32
    pub const shard_permanent_wisp_count: usize = 0x12D4; // int32
    pub const m_bActive: usize = 0x12D8; // bool
    pub const m_nWispFXIndex: usize = 0x12DC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Enchantress_Untouchable_Slow {
    pub const slow_attack_speed: usize = 0x12C8; // int32
    pub const m_nStatusFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Enigma_BlackHoleThinker {
    pub const radius: usize = 0x12C8; // int32
    pub const n_FXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Enigma_BlackHole_Pull {
    pub const pull_speed: usize = 0x12C8; // int32
    pub const radius: usize = 0x12CC; // int32
    pub const damage: usize = 0x12D0; // int32
    pub const tick_rate: usize = 0x12D4; // float32
    pub const pull_rotate_speed: usize = 0x12D8; // float32
    pub const animation_rate: usize = 0x12DC; // float32
    pub const scepter_pct_damage: usize = 0x12E0; // float32
    pub const m_flDamageTick: usize = 0x12E4; // GameTime_t
}

pub mod CDOTA_Modifier_Enigma_Black_Hole_Pull_Scepter {
    pub const scepter_drag_speed: usize = 0x12C8; // int32
    pub const scepter_pull_rotate_speed: usize = 0x12CC; // float32
    pub const aura_origin_x: usize = 0x12D0; // float32
    pub const aura_origin_y: usize = 0x12D4; // float32
    pub const m_nFXIndex: usize = 0x12D8; // ParticleIndex_t
    pub const m_vCenter: usize = 0x12DC; // Vector
    pub const m_flLastThinkTime: usize = 0x12E8; // GameTime_t
}

pub mod CDOTA_Modifier_Enigma_Black_Hole_Thinker_Scepter {
    pub const scepter_radius: usize = 0x12C8; // int32
    pub const n_FXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Enigma_DemonicConversion {
    pub const m_iAttackCount: usize = 0x12D0; // int32
    pub const split_attack_count: usize = 0x12D4; // int32
    pub const life_extension: usize = 0x12D8; // float32
    pub const eidolon_attack_range: usize = 0x12DC; // int32
    pub const eidolon_bonus_damage: usize = 0x12E0; // int32
    pub const eidolon_bonus_attack_speed: usize = 0x12E4; // int32
    pub const m_bAllowSplit: usize = 0x12E8; // bool
    pub const m_nSpawnNum: usize = 0x12EC; // int32
}

pub mod CDOTA_Modifier_Enigma_DemonicConversion_ModelScale {
    pub const self_modelscale: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Enigma_Malefice {
    pub const tick_rate: usize = 0x12C8; // float32
    pub const stun_duration: usize = 0x12CC; // float32
    pub const damage: usize = 0x12D0; // int32
    pub const duration: usize = 0x12D4; // float32
    pub const m_nEidolonSpawnNum: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Enigma_MidnightPulseThinker {
    pub const radius: usize = 0x12C8; // int32
    pub const n_FXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Enigma_MidnightPulse_Damage {
    pub const damage_percent: usize = 0x12C8; // float32
    pub const tick_rate: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_EnragedWildkin_Hurricane {
    pub const m_FX: usize = 0x12C8; // ParticleIndex_t
    pub const distance: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_EnragedWildkin_ToughnessAura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_EnragedWildkin_ToughnessAura_Bonus {
    pub const bonus_armor: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Ethereal_Blade_Ethereal {
    pub const ethereal_damage_bonus: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Eul_Cyclone {
    pub const m_hBlocker: usize = 0x12D0; // CHandle< C_BaseEntity >
    pub const m_bBlockerCreated: usize = 0x12D4; // bool
}

pub mod CDOTA_Modifier_FacelessVoid_Arcana_Kill_Effect {
    pub const m_nCasterFXIndex: usize = 0x12C8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_FacelessVoid_Backtrack {
    pub const dodge_chance_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_FacelessVoid_Chronosphere {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_FacelessVoid_Chronosphere_SelfBuff {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_FacelessVoid_Chronosphere_Speed {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_FacelessVoid_TimeDilation_Distortion {
    pub const slow_distance_min: usize = 0x12C8; // float32
    pub const slow_distance_max: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_FacelessVoid_TimeDilation_Slow {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const m_nAffectedAbilities: usize = 0x12CC; // int32
    pub const m_flLastDamageTime: usize = 0x12D0; // GameTime_t
    pub const slow: usize = 0x12D4; // int32
    pub const cooldown_percentage: usize = 0x12D8; // int32
    pub const damage_per_stack: usize = 0x12DC; // int32
    pub const base_damage: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_FacelessVoid_TimeLock {
    pub const duration: usize = 0x12C8; // float32
    pub const duration_creep: usize = 0x12CC; // float32
    pub const delay: usize = 0x12D0; // float32
    pub const chance_pct: usize = 0x12D4; // int32
    pub const bonus_damage: usize = 0x12D8; // int32
    pub const last_attack_time: usize = 0x12DC; // GameTime_t
    pub const m_ProcAttackRecords: usize = 0x12E0; // CUtlVector< int16 >
    pub const apply_activity_modifier_until: usize = 0x12F8; // GameTime_t
    pub const disable_activity_modifier_until: usize = 0x12FC; // GameTime_t
    pub const m_mapTargets: usize = 0x1300; // CUtlOrderedMap< CHandle< C_DOTA_BaseNPC >, int32, F( size=1 ) >
}

pub mod CDOTA_Modifier_FacelessVoid_TimeLock_ForceProc {
    pub const duration: usize = 0x12C8; // float32
    pub const duration_creep: usize = 0x12CC; // float32
    pub const bonus_damage: usize = 0x12D0; // int32
    pub const delay: usize = 0x12D4; // float32
    pub const m_hTarget: usize = 0x12D8; // CHandle< C_DOTA_BaseNPC >
}

pub mod CDOTA_Modifier_FacelessVoid_TimeWalk_ShardBuff {
    pub const m_vStartLocation: usize = 0x12C8; // Vector
}

pub mod CDOTA_Modifier_FacelessVoid_TimeWalk_Slow {
    pub const movement_speed_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_FelBeast_Haunt {
    pub const damage_per_second: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_FelBeast_Haunt_OnDeath {
    pub const projectile_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_FillerThinker {
    pub const hCasterList: usize = 0x12C8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const bStarted: usize = 0x12E0; // bool
    pub const nCount: usize = 0x12E4; // int32
}

pub mod CDOTA_Modifier_Filler_Heal {
    pub const hp_heal: usize = 0x12C8; // int32
    pub const mp_heal: usize = 0x12CC; // int32
    pub const hp_heal_growth: usize = 0x12D0; // int32
    pub const mp_heal_growth: usize = 0x12D4; // float32
    pub const hp_heal_pct: usize = 0x12D8; // int32
    pub const mp_heal_pct: usize = 0x12DC; // int32
    pub const tota_hp_per_second: usize = 0x12E0; // int32
    pub const tota_mp_per_second: usize = 0x12E4; // int32
}

pub mod CDOTA_Modifier_Filler_Heal_Aura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Filler_LastDitch {
    pub const melee_number: usize = 0x12C8; // float32
    pub const ranged_number: usize = 0x12CC; // float32
    pub const siege_number: usize = 0x12D0; // float32
    pub const lane: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Firecracker_Debuff {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Firework_Mine {
    pub const damage_radius: usize = 0x12C8; // int32
    pub const trigger_radius: usize = 0x12CC; // int32
    pub const damage: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_FixedNumberOfHitsToKill {
    pub const creep_attack_divisor: usize = 0x12C8; // int32
    pub const attack_count: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Flagbearer_Creep_Aura_Effect {
    pub const bonus_health_regen: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_FlaskHealing {
    pub const health_regen: usize = 0x12C8; // int32
    pub const break_on_hero_damage: usize = 0x12CC; // int32
    pub const m_fHealingDone: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_ForceStaff {
    pub const push_length: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Force_Boots {
    pub const push_length: usize = 0x12C8; // int32
    pub const push_duration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_ForestTrollHighPriest_HealAmp {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_ForestTrollHighPriest_HealAmp_Bonus {
    pub const heal_amp: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_ForestTrollHighPriest_HealAutocast {
    pub const health: usize = 0x12C8; // int32
    pub const m_hHealTarget: usize = 0x12CC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_ForestTrollHighPriest_ManaAura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_ForestTrollHighPriest_ManaAura_Bonus {
    pub const mana_regen: usize = 0x12C8; // float32
    pub const allow_multiple: usize = 0x12CC; // int32
    pub const m_nAttributes: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_ForgedSpirit_MeltingStrike {
    pub const duration: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_ForgedSpirit_Stats {
    pub const level: usize = 0x12C8; // int32
    pub const armor: usize = 0x12CC; // int32
    pub const mana: usize = 0x12D0; // int32
    pub const attack_range: usize = 0x12D4; // float32
    pub const armor_per_attack: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_FountainInvulnerabilityBuff {
    pub const m_vecPosition: usize = 0x12C8; // Vector
}

pub mod CDOTA_Modifier_FountainPassive {
    pub const bonus_chance: usize = 0x12C8; // int32
    pub const m_InFlightAttackRecords: usize = 0x12D0; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_FrostbittenGolem_TimeWarpAura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_FrostbittenGolem_TimeWarpAura_Bonus {
    pub const bonus_cdr: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Frostivus2018_Centaur_Return {
    pub const return_damage: usize = 0x12C8; // int32
    pub const strength_gain_duration: usize = 0x12CC; // float32
    pub const max_stacks: usize = 0x12D0; // int32
    pub const ally_factor: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Frostivus2018_Centaur_Return_Aura {
    pub const aura_radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Frostivus2018_Centaur_Return_Bonus_Strength {
    pub const strength_gain: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Frostivus2018_Centaur_Stampede {
    pub const radius: usize = 0x12C8; // int32
    pub const damage_reduction: usize = 0x12CC; // int32
    pub const m_hEntitiesAffected: usize = 0x12D0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Frostivus2018_Centaur_Stampede_Bonus_Armor {
    pub const bonus_base_armor_per_stack: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Frostivus2018_Centaur_Stampede_Slow {
    pub const slow_movement_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Frostivus2018_Clinkz_Burning_Army {
    pub const attack_rate: usize = 0x12C8; // float32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Frostivus2018_Clinkz_Burning_Army_Thinker {
    pub const m_iCount: usize = 0x12C8; // int32
    pub const m_vFacePosition: usize = 0x12CC; // Vector
}

pub mod CDOTA_Modifier_Frostivus2018_Clinkz_SearingArrows {
    pub const m_InFlightAttackRecords: usize = 0x12C8; // CUtlVector< int16 >
    pub const m_bBonusAttack: usize = 0x12E0; // bool
    pub const damage_bonus: usize = 0x12E4; // int32
    pub const attack_spill_width: usize = 0x12E8; // int32
    pub const attack_spill_range: usize = 0x12EC; // int32
    pub const spill_damage_multiplier: usize = 0x12F0; // float32
}

pub mod CDOTA_Modifier_Frostivus2018_Clinkz_Strafe {
    pub const attack_speed_bonus_pct: usize = 0x12C8; // int32
    pub const attack_spill_width: usize = 0x12CC; // int32
    pub const attack_spill_range: usize = 0x12D0; // int32
    pub const spill_damage_multiplier: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Frostivus2018_Clinkz_WindWalk {
    pub const m_hEntitiesAffected: usize = 0x12D8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_bFreePathing: usize = 0x12F0; // bool
    pub const move_speed_bonus_pct: usize = 0x12F4; // int32
    pub const radius: usize = 0x12F8; // int32
    pub const attack_steal_amount: usize = 0x12FC; // int32
    pub const attack_steal_duration: usize = 0x1300; // float32
    pub const max_stacks: usize = 0x1304; // int32
}

pub mod CDOTA_Modifier_Frostivus2018_Clinkz_WindWalk_AttackDebuff {
    pub const attack_steal_amount: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Frostivus2018_Clinkz_WindWalk_BonusDamage {
    pub const bonus_base_dmg_per_stack: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Frostivus2018_DarkWillow_Bedlam {
    pub const m_flRotation: usize = 0x12C8; // float32
    pub const m_hWisp: usize = 0x12CC; // CHandle< C_BaseEntity >
    pub const m_flLastAttack: usize = 0x12D0; // GameTime_t
    pub const roaming_radius: usize = 0x12D4; // int32
    pub const attack_radius: usize = 0x12D8; // int32
    pub const roaming_seconds_per_rotation: usize = 0x12DC; // float32
    pub const attack_interval: usize = 0x12E0; // float32
    pub const attack_targets: usize = 0x12E4; // int32
}

pub mod CDOTA_Modifier_Frostivus2018_DarkWillow_BrambleMaze_Creation_Thinker {
    pub const latch_creation_interval: usize = 0x12C8; // float32
    pub const initial_creation_delay: usize = 0x12CC; // float32
    pub const placement_count: usize = 0x12D0; // int32
    pub const placement_duration: usize = 0x12D4; // float32
    pub const m_iBramblesIndex: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Frostivus2018_DarkWillow_BrambleMaze_Thinker {
    pub const latch_count: usize = 0x12C8; // int32
    pub const latch_range: usize = 0x12CC; // int32
    pub const latch_vision: usize = 0x12D0; // int32
    pub const latch_creation_delay: usize = 0x12D4; // float32
    pub const latch_duration: usize = 0x12D8; // float32
    pub const m_bActive: usize = 0x12E4; // bool
    pub const m_nLatchedEnemies: usize = 0x12E8; // int32
}

pub mod CDOTA_Modifier_Frostivus2018_DarkWillow_ShadowRealm_Buff {
    pub const m_flStartTime: usize = 0x12C8; // GameTime_t
    pub const damage: usize = 0x12CC; // float32
    pub const max_damage_duration: usize = 0x12D0; // float32
    pub const slow_radius: usize = 0x12D4; // float32
    pub const burn_duration: usize = 0x12D8; // float32
    pub const burn_damage: usize = 0x12DC; // float32
    pub const slow_attack_speed_pct: usize = 0x12E0; // float32
}

pub mod CDOTA_Modifier_Frostivus2018_DarkWillow_ShadowRealm_Burn_Slow {
    pub const burn_damage: usize = 0x12C8; // float32
    pub const move_slow: usize = 0x12CC; // int32
    pub const m_flDamageScale: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Frostivus2018_Decorate_Tree_Thinker {
    pub const num_explosions: usize = 0x12C8; // int32
    pub const radius: usize = 0x12CC; // int32
    pub const m_fExplosions: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Frostivus2018_Earthshaker_Aftershock {
    pub const aftershock_range: usize = 0x12C8; // int32
    pub const stun_duration: usize = 0x12CC; // float32
    pub const ability_damage: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Frostivus2018_FacelessVoid_TimeLock {
    pub const duration: usize = 0x12C8; // float32
    pub const duration_creep: usize = 0x12CC; // float32
    pub const chance_pct: usize = 0x12D0; // int32
    pub const bonus_damage: usize = 0x12D4; // int32
    pub const radius: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Frostivus2018_FacelessVoid_TimeWalk_Tracker {
    pub const m_vDamageSources: usize = 0x12C8; // CUtlVector< CDOTA_Modifier_Frostivus2018_FacelessVoid_TimeWalk_Tracker::sDamageSources >
}

pub mod CDOTA_Modifier_Frostivus2018_FacelessVoid_TimeWalk_Tracker_sDamageSources {
    pub const flTime: usize = 0x0; // GameTime_t
    pub const flAmount: usize = 0x4; // float32
}

pub mod CDOTA_Modifier_Frostivus2018_FestiveFirework_Blind {
    pub const miss_rate: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Frostivus2018_Huskar_Burning_Spear_Counter {
    pub const duration: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Frostivus2018_Huskar_Burning_Spear_Debuff {
    pub const burn_damage: usize = 0x12C8; // int32
    pub const tick_rate: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Frostivus2018_Huskar_Burning_Spear_Self {
    pub const health_cost: usize = 0x12C8; // int32
    pub const spear_aoe: usize = 0x12CC; // int32
    pub const m_InFlightAttackRecords: usize = 0x12D0; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Frostivus2018_Luna_Eclipse {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const radius: usize = 0x12CC; // int32
    pub const beams: usize = 0x12D0; // int32
    pub const hit_count: usize = 0x12D4; // int32
    pub const m_iBeamDamage: usize = 0x12D8; // int32
    pub const m_flBeamStun: usize = 0x12DC; // float32
    pub const vPosition: usize = 0x12E0; // Vector
    pub const lucent_beam_radius: usize = 0x12EC; // int32
    pub const bAreaTarget: usize = 0x12F0; // bool
    pub const m_iTickCount: usize = 0x12F4; // int32
    pub const m_nMoonlightFXIndex: usize = 0x12F8; // ParticleIndex_t
    pub const m_HitTargets: usize = 0x1300; // CUtlVector< CHandle< C_BaseEntity > >
    pub const beam_interval: usize = 0x1318; // float32
    pub const beam_interval_scepter: usize = 0x131C; // float32
    pub const m_flInterval: usize = 0x1320; // float32
}

pub mod CDOTA_Modifier_Frostivus2018_Magnataur_Skewer_Bonus_Strength {
    pub const strength_gain: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Frostivus2018_Omniknight_Degen_Aura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Frostivus2018_Omniknight_Degen_Aura_Effect {
    pub const speed_bonus: usize = 0x12C8; // int32
    pub const bonus_health_regen: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Frostivus2018_Omniknight_Repel {
    pub const damage_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Frostivus2018_Puck_DreamCoil_Thinker {
    pub const m_hLinkedEntities: usize = 0x12C8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const coil_radius: usize = 0x12E0; // int32
    pub const coil_break_radius: usize = 0x12E4; // int32
    pub const coil_stun_duration: usize = 0x12E8; // float32
    pub const coil_stun_duration_scepter: usize = 0x12EC; // float32
    pub const coil_slow: usize = 0x12F0; // float32
    pub const coil_break_damage: usize = 0x12F4; // int32
    pub const coil_break_damage_scepter: usize = 0x12F8; // int32
    pub const m_bHasScepter: usize = 0x12FC; // bool
    pub const m_FXIndex: usize = 0x1300; // CUtlVector< ParticleIndex_t >
}

pub mod CDOTA_Modifier_Frostivus2018_Siege_Fireball_Burn {
    pub const damage_per_tick: usize = 0x12C8; // int32
    pub const tick_interval: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Frostivus2018_Siege_Fireball_Thinker {
    pub const radius: usize = 0x12C8; // int32
    pub const duration: usize = 0x12CC; // float32
    pub const tick_interval: usize = 0x12D0; // float32
    pub const m_nFXIndex: usize = 0x12D4; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Frostivus2018_Snowman_Taunt {
    pub const m_hZombieTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_bDidSetAttackTarget: usize = 0x12CC; // bool
}

pub mod CDOTA_Modifier_Frostivus2018_Spectre_ActiveDispersion {
    pub const damage_reflection_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Frostivus2018_Spectre_ActiveDispersion_Thinker {
    pub const m_fDamage: usize = 0x12C8; // float32
    pub const m_fLastRadius: usize = 0x12CC; // float32
    pub const m_fCurRadius: usize = 0x12D0; // float32
    pub const m_fLastThink: usize = 0x12D4; // GameTime_t
    pub const m_bContracting: usize = 0x12D8; // bool
    pub const m_ViewerTimer: usize = 0x12E0; // CountdownTimer
    pub const m_nFXIndex: usize = 0x12F8; // ParticleIndex_t
    pub const m_EntitiesHit: usize = 0x1300; // CUtlVector< CHandle< C_BaseEntity > >
    pub const speed: usize = 0x1318; // int32
    pub const radius: usize = 0x131C; // int32
}

pub mod CDOTA_Modifier_Frostivus2018_Summon_Snowman_Thinker {
    pub const tick_rate: usize = 0x12C8; // float32
    pub const taunt_radius: usize = 0x12CC; // int32
    pub const taunt_duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Frostivus2018_TrollWarlord_BattleTrance {
    pub const movement_speed: usize = 0x12C8; // int32
    pub const attack_speed: usize = 0x12CC; // int32
    pub const lifesteal: usize = 0x12D0; // int32
    pub const bonus_damage_pct: usize = 0x12D4; // int32
    pub const cleave_damage_pct: usize = 0x12D8; // float32
    pub const cleave_starting_width: usize = 0x12DC; // int32
    pub const cleave_ending_width: usize = 0x12E0; // int32
    pub const cleave_distance: usize = 0x12E4; // int32
    pub const splash_damage_pct: usize = 0x12E8; // float32
    pub const splash_radius: usize = 0x12EC; // int32
    pub const m_hTarget: usize = 0x12F0; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Frostivus2018_TrollWarlord_Fervor {
    pub const attack_speed: usize = 0x12C8; // int32
    pub const max_stacks: usize = 0x12CC; // int32
    pub const m_hUnit: usize = 0x12D0; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Frostivus2018_Tusk_WalrusKick_AirTime {
    pub const hp_threshold: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Frostivus2018_Tusk_WalrusKick_Slow {
    pub const move_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Frostivus2018_Weaver_GeminateAttack {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_iAttacksRemaining: usize = 0x12CC; // int32
    pub const arrow_count: usize = 0x1308; // int32
    pub const bonus_range: usize = 0x130C; // int32
}

pub mod CDOTA_Modifier_Furbolg_Enrage_AttackSpeed_OnDeath {
    pub const duration: usize = 0x12C8; // float32
    pub const radius: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Furbolg_Enrage_Damage_OnDeath {
    pub const duration: usize = 0x12C8; // float32
    pub const radius: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Furion_Arboreal_Might_Armor {
    pub const armor_per_stack: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Furion_Arboreal_Might_Attack_Damage {
    pub const damage_per_stack: usize = 0x12C8; // float32
    pub const iMaxStacks: usize = 0x12CC; // int32
    pub const m_nFXIndex: usize = 0x12D0; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Furion_CurseOfTheForest {
    pub const radius: usize = 0x12C8; // int32
    pub const damage_per_tree: usize = 0x12CC; // int32
    pub const slow_per_tree: usize = 0x12D0; // int32
    pub const m_nSlow: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Furion_Sprout_Blind {
    pub const blind_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Furion_Sprout_Damage {
    pub const sprout_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Furion_Sprout_Damage_Aura {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Furion_WrathOfNature_Thinker {
    pub const damage: usize = 0x12C8; // int32
    pub const max_targets: usize = 0x12CC; // int32
    pub const damage_percent_add: usize = 0x12D0; // int32
    pub const jump_delay: usize = 0x12D4; // float32
    pub const m_iFixedDamage: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Fusion_runeHealing {
    pub const health_regen: usize = 0x12C8; // int32
    pub const m_fHealingDone: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Gem_Active_TrueSight {
    pub const active_radius: usize = 0x12C8; // int32
    pub const duration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_GhostScepter_Active {
    pub const extra_spell_damage_percent: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Ghost_FrostAttack {
    pub const duration: usize = 0x12C8; // float32
    pub const movespeed_slow: usize = 0x12CC; // int32
    pub const attackspeed_slow: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Ghost_FrostAttack_Slow {
    pub const movespeed_slow: usize = 0x12C8; // int32
    pub const attackspeed_slow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_GiantWolf_CriticalStrike {
    pub const crit_mult: usize = 0x12C8; // int32
    pub const crit_chance: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_GiantWolf_Intimidate {
    pub const damage_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Gladiator_Helm_Buff {
    pub const damage: usize = 0x12C8; // int32
    pub const armor: usize = 0x12CC; // int32
    pub const movement_speed: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_GlimmerCape_Fade {
    pub const active_movement_speed: usize = 0x12C8; // int32
    pub const barrier_block: usize = 0x12CC; // int32
    pub const barrier_amount: usize = 0x12D0; // int32
    pub const initial_fade_delay: usize = 0x12D4; // float32
    pub const secondary_fade_delay: usize = 0x12D8; // float32
    pub const m_flFadeTime: usize = 0x12DC; // float32
    pub const m_flCurentFadeDelay: usize = 0x12E0; // float32
    pub const m_flLastActionTime: usize = 0x12E4; // GameTime_t
}

pub mod CDOTA_Modifier_GnollAssassin_EnvenomedWeapon {
    pub const damage_per_second: usize = 0x12C8; // int32
    pub const non_hero_duration: usize = 0x12CC; // float32
    pub const hero_duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_GnollAssassin_EnvenomedWeapon_Poison {
    pub const damage_per_second: usize = 0x12C8; // int32
    pub const regen_reduction: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Gold_Bag_Launch {
    pub const gold_amount: usize = 0x12F8; // int32
}

pub mod CDOTA_Modifier_Grandmasters_Glaive_Bonus_Agi {
    pub const agi_stance: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Grandmasters_Glaive_Bonus_Int {
    pub const int_stance: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Grandmasters_Glaive_Bonus_Str {
    pub const str_stance: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_GraniteGolem_HPAura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_GraniteGolem_HPAura_Bonus {
    pub const bonus_hp: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_GreaterClarity {
    pub const total_mana: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Greevil_Miniboss_Black_Nightmare {
    pub const animation_rate: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Greevil_Miniboss_Blue_ColdFeet {
    pub const m_vecStartPosition: usize = 0x12C8; // Vector
    pub const m_nCurrentTick: usize = 0x12D4; // int32
    pub const damage: usize = 0x12D8; // float32
    pub const break_distance: usize = 0x12DC; // int32
    pub const stun_duration: usize = 0x12E0; // float32
}

pub mod CDOTA_Modifier_Greevil_Miniboss_Blue_IceVortex {
    pub const movement_speed_pct: usize = 0x12C8; // int32
    pub const spell_resist_pct: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Greevil_Miniboss_Blue_IceVortexThinker {
    pub const radius: usize = 0x12C8; // int32
    pub const movement_speed_pct: usize = 0x12CC; // int32
    pub const spell_resist_pct: usize = 0x12D0; // int32
    pub const m_nFXIndex: usize = 0x12D4; // ParticleIndex_t
    pub const m_hChilledEntities: usize = 0x12D8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Greevil_Miniboss_Casting {
    pub const cast_animation: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Greevil_Miniboss_Green_LivingArmor {
    pub const health_regen: usize = 0x12C8; // int32
    pub const damage_block: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Greevil_Miniboss_Orange_LightStrikeArray_Thinker {
    pub const m_iDamage: usize = 0x12C8; // int32
    pub const m_flStunDuration: usize = 0x12CC; // float32
    pub const light_strike_array_aoe: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Greevil_Miniboss_Purple_VenomousGale {
    pub const movement_slow: usize = 0x12C8; // int32
    pub const tick_damage: usize = 0x12CC; // int32
    pub const tick_interval: usize = 0x12D0; // float32
    pub const duration: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Greevil_Miniboss_Red_Earthshock {
    pub const movement_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Greevil_Miniboss_Red_Overpower {
    pub const attack_speed_bonus_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Greevil_Miniboss_Sight {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Greevil_Miniboss_White_Degen_Aura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Greevil_Miniboss_White_Degen_Aura_Effect {
    pub const speed_bonus: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Greevil_Miniboss_Yellow_IonShell {
    pub const radius: usize = 0x12C8; // int32
    pub const damage_per_second: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Grimstroke_DarkArtistry_Slow {
    pub const movement_slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Grimstroke_InkCreature {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const m_nAmbientFXIndex: usize = 0x12CC; // ParticleIndex_t
    pub const m_nAttackCount: usize = 0x12D0; // int32
    pub const m_bIsLatched: usize = 0x12D4; // bool
    pub const m_hAttachTarget: usize = 0x12D8; // CHandle< C_BaseEntity >
    pub const m_EnemyVision: usize = 0x12E0; // CountdownTimer
    pub const latch_duration: usize = 0x12F8; // float32
    pub const speed: usize = 0x12FC; // float32
    pub const destroy_attacks: usize = 0x1300; // int32
    pub const hero_attack_multiplier: usize = 0x1304; // int32
}

pub mod CDOTA_Modifier_Grimstroke_InkCreature_Debuff {
    pub const m_flCurrentArmorReduction: usize = 0x12C8; // float32
    pub const m_hLatchedCreature: usize = 0x12CC; // CHandle< C_BaseEntity >
    pub const tick_interval: usize = 0x12D0; // float32
    pub const damage_per_second: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Grimstroke_InkCreature_Latched {
    pub const m_hAttachTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_fZOffset: usize = 0x12CC; // float32
    pub const m_bRemovedByEnemy: usize = 0x12D0; // bool
    pub const latch_duration: usize = 0x12D4; // float32
    pub const pop_damage: usize = 0x12D8; // int32
    pub const latched_unit_offset: usize = 0x12DC; // int32
    pub const latched_unit_offset_short: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_Grimstroke_InkCreature_Spawning {
    pub const m_hAttachTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Grimstroke_Scepter_Buff {
    pub const images_movespeed_bonus: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Grimstroke_Shard_Buff {
    pub const total_damage: usize = 0x12C8; // int32
    pub const attack_speed: usize = 0x12CC; // int32
    pub const move_speed: usize = 0x12D0; // int32
    pub const m_flInterval: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Grimstroke_SoulChain {
    pub const m_hPartner: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_bIsPrimary: usize = 0x12CC; // bool
    pub const m_bEscaped: usize = 0x12CD; // bool
    pub const m_bTethered: usize = 0x12CE; // bool
    pub const m_nFXIndex: usize = 0x12D0; // ParticleIndex_t
    pub const m_nFXIndexA: usize = 0x12D4; // ParticleIndex_t
    pub const m_nFXIndexB: usize = 0x12D8; // ParticleIndex_t
    pub const m_bStartedLeashSound: usize = 0x12DC; // bool
    pub const m_fLeashDistance: usize = 0x12E0; // float32
    pub const m_fOriginalStartTime: usize = 0x12E4; // GameTime_t
    pub const m_nOverheadFXIndex: usize = 0x12E8; // ParticleIndex_t
    pub const chain_latch_radius: usize = 0x12EC; // int32
    pub const chain_break_distance: usize = 0x12F0; // int32
    pub const leash_limit_multiplier: usize = 0x12F4; // float32
    pub const chain_duration: usize = 0x12F8; // float32
    pub const creep_duration_pct: usize = 0x12FC; // float32
    pub const bonus_reflected_spell_damage: usize = 0x1300; // int32
}

pub mod CDOTA_Modifier_Grimstroke_SoulChain_ChannelCheck {
    pub const m_hAbility: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_hTarget: usize = 0x12CC; // CHandle< C_BaseEntity >
    pub const m_hAbilities: usize = 0x12D0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_vLocation: usize = 0x12E8; // Vector
    pub const m_fChannelEnd: usize = 0x12F4; // GameTime_t
    pub const m_bInterrupt: usize = 0x12F8; // bool
    pub const m_bExpired: usize = 0x12F9; // bool
}

pub mod CDOTA_Modifier_Grimstroke_SpiritWalk_Buff {
    pub const m_fStartTime: usize = 0x12C8; // GameTime_t
    pub const m_nHeroTickDamageApplied: usize = 0x12CC; // int32
    pub const m_fLastEffectsTime: usize = 0x12D0; // GameTime_t
    pub const radius: usize = 0x12D4; // int32
    pub const buff_duration: usize = 0x12D8; // float32
    pub const debuff_duration: usize = 0x12DC; // float32
    pub const max_damage: usize = 0x12E0; // int32
    pub const max_stun: usize = 0x12E4; // float32
    pub const movespeed_bonus_pct: usize = 0x12E8; // int32
    pub const damage_per_tick: usize = 0x12EC; // int32
    pub const tick_rate: usize = 0x12F0; // float32
    pub const max_threshold_duration: usize = 0x12F4; // float32
    pub const shard_bonus_damage_pct: usize = 0x12F8; // int32
    pub const shard_heal_pct: usize = 0x12FC; // int32
    pub const m_bIsReflection: usize = 0x1300; // bool
}

pub mod CDOTA_Modifier_GungirChain {
    pub const chain_radius: usize = 0x12C8; // int32
    pub const chain_strikes: usize = 0x12CC; // int32
    pub const chain_damage: usize = 0x12D0; // int32
    pub const chain_delay: usize = 0x12D4; // float32
    pub const m_iCurJumpCount: usize = 0x12D8; // int32
    pub const m_vCurTargetLoc: usize = 0x12DC; // Vector
    pub const m_hHitEntities: usize = 0x12E8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Gyrocopter_Call_Down {
    pub const slow_duration_first: usize = 0x12C8; // int32
    pub const slow_duration_second: usize = 0x12CC; // int32
    pub const damage_first: usize = 0x12D0; // int32
    pub const damage_second: usize = 0x12D4; // int32
    pub const damage_second_scepter: usize = 0x12D8; // int32
    pub const radius: usize = 0x12DC; // int32
    pub const slow_first: usize = 0x12E0; // float32
    pub const slow_second: usize = 0x12E4; // float32
    pub const m_bFirstStrike: usize = 0x12E8; // bool
    pub const m_bDelayed: usize = 0x12E9; // bool
    pub const m_bHalfDamage: usize = 0x12EA; // bool
}

pub mod CDOTA_Modifier_Gyrocopter_Call_Down_Slow {
    pub const movespeed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Gyrocopter_Flak_Cannon {
    pub const max_attacks: usize = 0x12C8; // int32
    pub const bonus_damage: usize = 0x12CC; // int32
    pub const m_iNumAttacks: usize = 0x12D0; // int32
    pub const projectile_speed: usize = 0x12D4; // int32
    pub const bonus_night_vision: usize = 0x12D8; // int32
    pub const radius: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Gyrocopter_Flak_Cannon_Scepter {
    pub const fire_rate: usize = 0x12C8; // float32
    pub const scepter_radius: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Gyrocopter_Homing_Missile {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const m_nFXIndex2: usize = 0x12CC; // ParticleIndex_t
    pub const hero_damage: usize = 0x12D0; // int32
    pub const acceleration: usize = 0x12D4; // int32
    pub const min_damage: usize = 0x12D8; // int32
    pub const max_distance: usize = 0x12DC; // int32
    pub const shard_radius: usize = 0x12E0; // int32
    pub const shard_delay: usize = 0x12E4; // float32
    pub const pre_flight_time: usize = 0x12E8; // float32
    pub const stun_duration: usize = 0x12EC; // float32
    pub const m_nMissileDamage: usize = 0x12F0; // int32
    pub const m_nTeamNumber: usize = 0x12F4; // int32
    pub const speed: usize = 0x1300; // float32
    pub const m_hAttachTarget: usize = 0x1304; // CHandle< C_BaseEntity >
    pub const m_vStartPosition: usize = 0x1308; // Vector
    pub const m_EnemyVision: usize = 0x1318; // CountdownTimer
    pub const m_MoveTime: usize = 0x1330; // CountdownTimer
}

pub mod CDOTA_Modifier_Gyrocopter_Homing_Rocket_Barrage {
    pub const rocket_damage: usize = 0x12C8; // int32
    pub const shard_radius: usize = 0x12CC; // int32
    pub const m_hTarget: usize = 0x12D0; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Gyrocopter_Rocket_Barrage {
    pub const rocket_damage: usize = 0x12C8; // int32
    pub const bonus_movement_speed: usize = 0x12CC; // int32
    pub const slow_resistance: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_HallOfFame_Glow {
    pub const m_vecGlowingPlayerIDs: usize = 0x12C8; // CUtlVector< PlayerID_t >
    pub const m_vecGlowingPlayerIDs2: usize = 0x12E0; // CUtlVector< PlayerID_t >
}

pub mod CDOTA_Modifier_Halo {
    pub const bonus_all_stats: usize = 0x12C8; // int32
    pub const bonus_health: usize = 0x12CC; // int32
    pub const bonus_mana: usize = 0x12D0; // int32
    pub const bonus_mana_regen: usize = 0x12D4; // float32
    pub const slow_duration: usize = 0x12D8; // float32
    pub const cast_range_bonus: usize = 0x12DC; // int32
    pub const aoe_bonus: usize = 0x12E0; // int32
    pub const bonus_spell_damage: usize = 0x12E4; // int32
}

pub mod CDOTA_Modifier_HarpyScout_TakeOff {
    pub const move_speed_penalty: usize = 0x12C8; // int32
    pub const cost_per_second: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_HarpyStorm_ChainLightning {
    pub const jump_range: usize = 0x12C8; // int32
    pub const max_targets: usize = 0x12CC; // int32
    pub const damage_percent_loss: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Havoc_Hammer_Slow {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Healing_Campfire_Heal {
    pub const heal_amount: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_HeavensHalberd_Debuff {
    pub const m_bPierceDebuffImmunity: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_HeroStatue {
    pub const m_unStatusEffectIndex: usize = 0x12C8; // uint32
    pub const m_hPedestal: usize = 0x12CC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Hide_On_Minimap {
    pub const m_bEnemiesOnly: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_HighFiveRequested {
    pub const acknowledge_range: usize = 0x12C8; // int32
    pub const think_interval: usize = 0x12CC; // float32
    pub const acknowledged_cooldown: usize = 0x12D0; // float32
    pub const m_bAcknowledged: usize = 0x12D4; // bool
    pub const m_bFirstThink: usize = 0x12D5; // bool
    pub const high_five_level: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_HillTroll_Rally {
    pub const damage_bonus: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_HillTroll_RallyAura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_HoldoutCullingBlade {
    pub const leap_distance: usize = 0x12C8; // int32
    pub const leap_radius: usize = 0x12CC; // int32
    pub const leap_speed: usize = 0x12D0; // float32
    pub const leap_acceleration: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Holdout_BladeFury {
    pub const blade_fury_damage: usize = 0x12C8; // int32
    pub const blade_dance_crit_chance: usize = 0x12CC; // int32
    pub const blade_dance_crit_mult: usize = 0x12D0; // int32
    pub const blade_fury_damage_tick: usize = 0x12D4; // float32
    pub const blade_fury_radius: usize = 0x12D8; // int32
    pub const bonus_evasion: usize = 0x12DC; // int32
    pub const m_bTryToCrit: usize = 0x12E0; // bool
}

pub mod CDOTA_Modifier_Holdout_GladiatorsUnite {
    pub const bonus_armor_per_hero: usize = 0x12C8; // int32
    pub const bonus_damage_per_hero: usize = 0x12CC; // int32
    pub const lifesteal_per_hero: usize = 0x12D0; // int32
    pub const scepter_damage_reduction_pct: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Holdout_GladiatorsUnite_Thinker {
    pub const radius: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Holdout_Omnislash {
    pub const animation_rate: usize = 0x12C8; // float32
    pub const bonus_damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Holdout_StaticRemnantThinker {
    pub const static_remnant_radius: usize = 0x12C8; // int32
    pub const static_remnant_damage_radius: usize = 0x12CC; // int32
    pub const static_remnant_damage: usize = 0x12D0; // int32
    pub const static_remnant_delay: usize = 0x12D4; // float32
    pub const overload_duration: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Hoodwink_AcornShot_ArmorCorruption {
    pub const value: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Hoodwink_AcornShot_BonusDamage {
    pub const acorn_shot_damage: usize = 0x12C8; // int32
    pub const base_damage_pct: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Hoodwink_AcornShot_Slow {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Hoodwink_Bushwhack_Trap {
    pub const visual_height: usize = 0x12C8; // int32
    pub const animation_rate: usize = 0x12CC; // float32
    pub const m_iIndex: usize = 0x12D0; // ParticleIndex_t
    pub const m_bPullComplete: usize = 0x12D4; // bool
    pub const m_TreeId: usize = 0x12D8; // uint32
    pub const m_flDamagePerTick: usize = 0x12DC; // float32
    pub const m_nDamageTicks: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_Hoodwink_Caltrops_Debuff {
    pub const movespeed_pct: usize = 0x12C8; // int32
    pub const tick_damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Hoodwink_Camouflage {
    pub const fade_time: usize = 0x12C8; // float32
    pub const m_flFadeTime: usize = 0x12CC; // GameTime_t
    pub const m_bInvisLastThink: usize = 0x12D0; // bool
}

pub mod CDOTA_Modifier_Hoodwink_Decoy_Illusion {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Hoodwink_Decoy_Invisibility {
    pub const movement_speed: usize = 0x12D8; // int32
    pub const m_hEntitiesAffected: usize = 0x12E0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Hoodwink_HeavyQuiver {
    pub const bonus_projectile_speed_pct: usize = 0x12C8; // int32
    pub const bonus_attack_range: usize = 0x12CC; // int32
    pub const deadly_blow_multiplier: usize = 0x12D0; // int32
    pub const maim_duration: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Hoodwink_HuntersQuiver {
    pub const bonus_attack_range: usize = 0x12C8; // int32
    pub const max_stacks: usize = 0x12CC; // int32
    pub const debuff_duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Hoodwink_HuntersQuiver_Mark {
    pub const stack_crit_chance_pct: usize = 0x12C8; // int32
    pub const critical_damage_pct: usize = 0x12CC; // int32
    pub const stack_slow_pct: usize = 0x12D0; // int32
    pub const m_nFXStackIndex: usize = 0x12D4; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Hoodwink_Hunters_Mark {
    pub const slow_pct: usize = 0x12C8; // int32
    pub const spell_amp: usize = 0x12CC; // int32
    pub const status_resistance: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Hoodwink_Scurry_Active {
    pub const movement_speed_pct: usize = 0x12C8; // int32
    pub const attack_range: usize = 0x12CC; // int32
    pub const cast_range: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Hoodwink_Scurry_Passive {
    pub const evasion: usize = 0x12C8; // int32
    pub const radius: usize = 0x12CC; // int32
    pub const bonus_active_evasion: usize = 0x12D0; // int32
    pub const m_nFXIndex: usize = 0x12D4; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Hoodwink_Sharpshooter_Debuff {
    pub const slow_move_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Hoodwink_Sharpshooter_VisionThinker {
    pub const arrow_width: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Hoodwink_Sharpshooter_Windup {
    pub const arrow_vision: usize = 0x12C8; // int32
    pub const max_charge_time: usize = 0x12CC; // float32
    pub const turn_rate: usize = 0x12D0; // float32
    pub const base_power: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Hurricane_Pike_Active {
    pub const push_length: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Hurricane_Pike_Active_Alternate {
    pub const enemy_length: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Hurricane_Pike_Range {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const max_attacks: usize = 0x12CC; // int32
    pub const m_iNumAttacks: usize = 0x12D0; // int32
    pub const bActive: usize = 0x12D4; // bool
    pub const bonus_attack_speed: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Huskar_Berserkers_Blood {
    pub const maximum_health_regen: usize = 0x12C8; // int32
    pub const maximum_attack_speed: usize = 0x12CC; // int32
    pub const maximum_magic_resist: usize = 0x12D0; // int32
    pub const hp_threshold_max: usize = 0x12D4; // int32
    pub const m_nFxHuskIndex: usize = 0x12D8; // ParticleIndex_t
    pub const m_nFxIndexA: usize = 0x12DC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Huskar_Burning_Spear_Counter {
    pub const duration: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Huskar_Burning_Spear_Debuff {
    pub const burn_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Huskar_Burning_Spear_Self {
    pub const m_InFlightAttackRecords: usize = 0x12C8; // CUtlVector< int16 >
    pub const duration: usize = 0x12E0; // float32
}

pub mod CDOTA_Modifier_Huskar_Inner_Fire_Disarm {
    pub const movement_slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Huskar_Inner_Fire_Knockback {
    pub const knockback_distance: usize = 0x12C8; // float32
    pub const knockback_duration: usize = 0x12CC; // float32
    pub const effective_distance: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Huskar_Inner_Vitality {
    pub const heal: usize = 0x12C8; // int32
    pub const attrib_bonus: usize = 0x12CC; // float32
    pub const hurt_attrib_bonus: usize = 0x12D0; // float32
    pub const hurt_percent: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Huskar_Life_Break {
    pub const attack_speed: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Huskar_Life_Break_Taunt {
    pub const m_hZombieTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_bDidSetAttackTarget: usize = 0x12CC; // bool
}

pub mod CDOTA_Modifier_IceShaman_IncendiaryBomb {
    pub const burn_damage: usize = 0x12C8; // int32
    pub const building_damage_pct: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_IceSlide {
    pub const m_fLastUpdateTime: usize = 0x12C8; // GameTime_t
    pub const m_vVelocity: usize = 0x12CC; // Vector
}

pub mod CDOTA_Modifier_Illusion {
    pub const outgoing_damage: usize = 0x12C8; // int32
    pub const outgoing_damage_structure: usize = 0x12CC; // int32
    pub const outgoing_damage_roshan: usize = 0x12D0; // int32
    pub const incoming_damage: usize = 0x12D4; // int32
    pub const bounty_base: usize = 0x12D8; // int32
    pub const bounty_growth: usize = 0x12DC; // int32
    pub const m_strIllusionType: usize = 0x12E0; // CUtlString
    pub const m_szIllusionLabel: usize = 0x12E8; // char[64]
    pub const m_bHidden: usize = 0x1328; // bool
}

pub mod CDOTA_Modifier_InvisibilityEdge_WindWalk {
    pub const windwalk_bonus_damage: usize = 0x12D8; // int32
    pub const windwalk_movement_speed: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Invisible {
    pub const m_bSubtle: usize = 0x12C8; // bool
    pub const m_flFadeTime: usize = 0x12CC; // GameTime_t
    pub const m_flFadeTimeValue: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Invoke_Bonuses {
    pub const spell_amp: usize = 0x12C8; // float32
    pub const spell_lifesteal: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Invoker_Alacrity {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
    pub const bonus_damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Invoker_ChaosMeteor_Burn {
    pub const spell_amp: usize = 0x12C8; // float32
    pub const spell_lifesteal: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Invoker_ColdSnap {
    pub const freeze_damage: usize = 0x12C8; // float32
    pub const freeze_duration: usize = 0x12CC; // float32
    pub const freeze_cooldown: usize = 0x12D0; // float32
    pub const damage_trigger: usize = 0x12D4; // float32
    pub const spell_lifesteal: usize = 0x12D8; // float32
    pub const spell_amp: usize = 0x12DC; // float32
    pub const freeze_heal: usize = 0x12E0; // float32
    pub const nQuasApplications: usize = 0x12E4; // int32
    pub const nExortApplications: usize = 0x12E8; // int32
}

pub mod CDOTA_Modifier_Invoker_DeafeningBlast_Knockback {
    pub const disarm_duration: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Invoker_EMP {
    pub const area_of_effect: usize = 0x12C8; // int32
    pub const mana_burned: usize = 0x12CC; // int32
    pub const damage_per_mana: usize = 0x12D0; // float32
    pub const spell_lifesteal: usize = 0x12D4; // float32
    pub const spell_amp: usize = 0x12D8; // float32
    pub const self_mana_restore_pct: usize = 0x12DC; // int32
    pub const m_hPullThinker: usize = 0x12E0; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Invoker_EMP_Pull {
    pub const shard_drag_speed: usize = 0x12C8; // int32
    pub const aura_origin_x: usize = 0x12CC; // float32
    pub const aura_origin_y: usize = 0x12D0; // float32
    pub const m_nFXIndex: usize = 0x12D4; // ParticleIndex_t
    pub const m_vCenter: usize = 0x12D8; // Vector
    pub const m_flLastThinkTime: usize = 0x12E4; // GameTime_t
}

pub mod CDOTA_Modifier_Invoker_EMP_Pull_Thinker {
    pub const area_of_effect: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Invoker_ExortInstance {
    pub const bonus_damage_per_instance: usize = 0x12C8; // int32
    pub const spell_amp: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Invoker_GhostWalk_Enemy {
    pub const enemy_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Invoker_GhostWalk_Self {
    pub const self_slow: usize = 0x12D8; // int32
    pub const area_of_effect: usize = 0x12DC; // int32
    pub const aura_fade_time: usize = 0x12E0; // float32
    pub const health_regen: usize = 0x12E4; // float32
    pub const mana_regen: usize = 0x12E8; // float32
    pub const disable_time: usize = 0x12EC; // float32
    pub const m_timeLastDamage: usize = 0x12F0; // GameTime_t
}

pub mod CDOTA_Modifier_Invoker_IceWall_SlowAura {
    pub const radius: usize = 0x12C8; // int32
    pub const slow: usize = 0x12CC; // int32
    pub const slow_duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Invoker_IceWall_SlowDebuff {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Invoker_IceWall_Thinker {
    pub const damage_per_second: usize = 0x12C8; // float32
    pub const num_wall_elements: usize = 0x12CC; // int32
    pub const wall_element_spacing: usize = 0x12D0; // int32
    pub const wall_element_radius: usize = 0x12D4; // int32
    pub const spell_amp: usize = 0x12D8; // float32
    pub const spell_lifesteal: usize = 0x12DC; // float32
    pub const m_vWallDir: usize = 0x12E0; // Vector
}

pub mod CDOTA_Modifier_Invoker_QuasInstance {
    pub const spell_lifesteal: usize = 0x12C8; // float32
    pub const health_regen_per_instance: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Invoker_SunStrike {
    pub const area_of_effect: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Invoker_SunStrike_Cataclysm {
    pub const area_of_effect: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Invoker_SunStrike_Cataclysm_Thinker {
    pub const damage: usize = 0x12C8; // float32
    pub const spell_lifesteal: usize = 0x12CC; // float32
    pub const spell_amp: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Invoker_WexInstance {
    pub const move_speed_per_instance: usize = 0x12C8; // float32
    pub const cooldown_reduction: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Invoker_Wex_Cooldown_Reduction {
    pub const cooldown_reduction: usize = 0x12C8; // int32
    pub const m_nTotalCDR: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_IonShell {
    pub const bonus_health: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_AbyssalBlade {
    pub const bonus_strength: usize = 0x12C8; // int32
    pub const bash_chance_melee: usize = 0x12CC; // int32
    pub const bash_chance_ranged: usize = 0x12D0; // int32
    pub const bash_duration: usize = 0x12D4; // float32
    pub const bash_cooldown: usize = 0x12D8; // float32
    pub const bonus_chance_damage: usize = 0x12DC; // int32
    pub const bonus_damage: usize = 0x12E0; // int32
    pub const m_InFlightAttackRecords: usize = 0x12E8; // CUtlVector< int16 >
    pub const bonus_health: usize = 0x1300; // int32
    pub const bonus_health_regen: usize = 0x1304; // float32
    pub const block_damage_ranged: usize = 0x1308; // int32
    pub const block_damage_melee: usize = 0x130C; // int32
    pub const block_chance: usize = 0x1310; // int32
}

pub mod CDOTA_Modifier_Item_Aegis {
    pub const reincarnate_time: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Item_AeonDisk {
    pub const bonus_health: usize = 0x12C8; // int32
    pub const bonus_mana: usize = 0x12CC; // int32
    pub const health_threshold_pct: usize = 0x12D0; // int32
    pub const buff_duration: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Item_AeonDisk_Buff {
    pub const status_resistance: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_AghsFort_Bloodstone {
    pub const bonus_health: usize = 0x12C8; // int32
    pub const bonus_mana: usize = 0x12CC; // int32
    pub const mana_regen_multiplier: usize = 0x12D0; // int32
    pub const regen_per_charge: usize = 0x12D4; // float32
    pub const amp_per_charge: usize = 0x12D8; // float32
    pub const death_charges: usize = 0x12DC; // int32
    pub const kill_charges: usize = 0x12E0; // int32
    pub const hero_lifesteal: usize = 0x12E4; // float32
    pub const creep_lifesteal: usize = 0x12E8; // float32
    pub const bonus_intellect: usize = 0x12EC; // int32
    pub const spell_amp: usize = 0x12F0; // int32
    pub const spell_lifesteal_amp: usize = 0x12F4; // int32
}

pub mod CDOTA_Modifier_Item_AghsFort_Bloodstone_Active {
    pub const mana_cost_percentage: usize = 0x12C8; // int32
    pub const restore_duration: usize = 0x12CC; // float32
    pub const m_flDrainAmount: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Item_Ancient_Janggo {
    pub const radius: usize = 0x12C8; // int32
    pub const bonus_str: usize = 0x12CC; // int32
    pub const bonus_int: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Ancient_Janggo_Active {
    pub const bonus_attack_speed_pct: usize = 0x12C8; // int32
    pub const bonus_movement_speed_pct: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Ancient_Janggo_Aura {
    pub const aura_movement_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Ancient_Perseverance {
    pub const damage: usize = 0x12C8; // int32
    pub const hp_regen: usize = 0x12CC; // int32
    pub const mana_regen_amp: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Apex {
    pub const primary_stat: usize = 0x12C8; // int32
    pub const primary_stat_universal: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Arcane_Blink {
    pub const blink_damage_cooldown: usize = 0x12C8; // float32
    pub const bonus_intellect: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Arcane_Blink_Buff {
    pub const cast_pct_improvement: usize = 0x12C8; // int32
    pub const manacost_reduction: usize = 0x12CC; // int32
    pub const debuff_amp: usize = 0x12D0; // int32
    pub const cast_range_bonus: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_Arcane_Boots {
    pub const bonus_movement: usize = 0x12C8; // int32
    pub const bonus_mana: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Arcane_Ring {
    pub const bonus_armor: usize = 0x12C8; // int32
    pub const bonus_intelligence: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Arcane_Scout {
    pub const vision_bonus: usize = 0x12C8; // int32
    pub const magic_resist: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Arcane_Scout_Invulnerability {
    pub const scout_movespeed: usize = 0x12C8; // int32
    pub const sight_range: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Armlet {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const bonus_attack_speed: usize = 0x12CC; // int32
    pub const bonus_armor: usize = 0x12D0; // int32
    pub const bonus_health_regen: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_AsceticCap {
    pub const bonus_health: usize = 0x12C8; // int32
    pub const hp_regen: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Item_AsceticCapBuff {
    pub const status_resistance: usize = 0x12C8; // int32
    pub const slow_resistance: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_AssassinsDagger {
    pub const m_InFlightAttackRecords: usize = 0x12C8; // CUtlVector< int16 >
    pub const debuff_duration: usize = 0x12E0; // int32
    pub const bonus_attack_speed: usize = 0x12E4; // int32
}

pub mod CDOTA_Modifier_Item_Assault_Cuirass {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
    pub const bonus_armor: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Assault_Cuirass_Negative_Armor {
    pub const aura_negative_armor: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Assault_Cuirass_Positive {
    pub const aura_attack_speed: usize = 0x12C8; // int32
    pub const aura_positive_armor: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Ballista {
    pub const attack_range_bonus: usize = 0x12C8; // int32
    pub const knockback_distance: usize = 0x12CC; // float32
    pub const knockback_duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Item_Barricade {
    pub const bonus_health: usize = 0x12C8; // int32
    pub const bonus_armor: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Barricade_Thinker {
    pub const length: usize = 0x12C8; // int32
    pub const width: usize = 0x12CC; // int32
    pub const max_hits: usize = 0x12D0; // int32
    pub const m_iCurrentHits: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_Battlefury {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const bonus_health_regen: usize = 0x12CC; // float32
    pub const bonus_mana_regen: usize = 0x12D0; // float32
    pub const cleave_damage_percent: usize = 0x12D4; // int32
    pub const cleave_damage_percent_creep: usize = 0x12D8; // int32
    pub const cleave_starting_width: usize = 0x12DC; // int32
    pub const cleave_ending_width: usize = 0x12E0; // int32
    pub const cleave_distance: usize = 0x12E4; // int32
    pub const upgraded_cleave_bonus: usize = 0x12E8; // int32
    pub const upgraded_damage_bonus: usize = 0x12EC; // int32
    pub const quelling_bonus: usize = 0x12F0; // int32
    pub const quelling_bonus_ranged: usize = 0x12F4; // int32
}

pub mod CDOTA_Modifier_Item_BeltOfStrength {
    pub const bonus_strength: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Black_King_Bar {
    pub const bonus_strength: usize = 0x12C8; // int32
    pub const bonus_damage: usize = 0x12CC; // int32
    pub const duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Item_BladeOfAlacrity {
    pub const bonus_agility: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Blade_Mail {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const bonus_armor: usize = 0x12CC; // int32
    pub const bonus_intellect: usize = 0x12D0; // int32
    pub const passive_reflection_constant: usize = 0x12D4; // int32
    pub const passive_reflection_pct: usize = 0x12D8; // int32
    pub const active_reflection_pct: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Item_BladesOfAttack {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Blight_Stone {
    pub const corruption_duration: usize = 0x12C8; // float32
    pub const m_InFlightAttackRecords: usize = 0x12D0; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Item_BlightedSpirit_Blight {
    pub const damage_per_second: usize = 0x12C8; // int32
    pub const heal_reduction: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_BlinkDagger {
    pub const blink_damage_cooldown: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Item_Blood_Grenade {
    pub const bonus_health: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Blood_Grenade_Debuff {
    pub const movespeed_slow: usize = 0x12C8; // int32
    pub const damage_over_time: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Item_Blood_Grenade_Flight_Thinker {
    pub const speed: usize = 0x12C8; // int32
    pub const radius: usize = 0x12CC; // int32
    pub const damage_over_time: usize = 0x12D0; // int32
    pub const impact_damage: usize = 0x12D4; // int32
    pub const debuff_duration: usize = 0x12D8; // float32
    pub const movespeed_slow: usize = 0x12DC; // int32
    pub const tick_rate: usize = 0x12E0; // float32
    pub const m_nFXIndex: usize = 0x12E4; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Item_Bloodstone {
    pub const bonus_health: usize = 0x12C8; // int32
    pub const bonus_mana: usize = 0x12CC; // int32
    pub const spell_lifesteal: usize = 0x12D0; // float32
    pub const lifesteal_multiplier: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Item_Bloodthorn {
    pub const bonus_intellect: usize = 0x12C8; // int32
    pub const bonus_mana_regen: usize = 0x12CC; // float32
    pub const bonus_damage: usize = 0x12D0; // int32
    pub const bonus_attack_speed: usize = 0x12D4; // int32
    pub const spell_amp: usize = 0x12D8; // int32
    pub const mana_regen_multiplier: usize = 0x12DC; // int32
    pub const bonus_magic_resist: usize = 0x12E0; // int32
    pub const duration: usize = 0x12E4; // float32
    pub const bonus_health_regen: usize = 0x12E8; // float32
}

pub mod CDOTA_Modifier_Item_Book_Of_Shadows {
    pub const bonus_all_stats: usize = 0x12C8; // int32
    pub const night_vision: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Book_Of_Shadows_Buff {
    pub const m_bUntargetableAllies: usize = 0x12C8; // bool
    pub const m_bUntargetableEnemies: usize = 0x12C9; // bool
}

pub mod CDOTA_Modifier_Item_BootsOfElven {
    pub const bonus_agility: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_BootsOfSpeed {
    pub const bonus_movement_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_BootsOfTravel {
    pub const bonus_movement_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Boots_Of_Bearing {
    pub const radius: usize = 0x12C8; // int32
    pub const bonus_movement_speed: usize = 0x12CC; // int32
    pub const bonus_health_regen: usize = 0x12D0; // int32
    pub const bonus_str: usize = 0x12D4; // int32
    pub const bonus_int: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Item_Boots_Of_Bearing_Active {
    pub const bonus_attack_speed_pct: usize = 0x12C8; // int32
    pub const bonus_movement_speed_pct: usize = 0x12CC; // int32
    pub const bonus_ms_duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Item_Boots_Of_Bearing_Aura {
    pub const aura_movement_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Bottomless_Chalice {
    pub const max_charges_per_kill: usize = 0x12C8; // int32
    pub const recharge_time: usize = 0x12CC; // float32
    pub const m_bFull: usize = 0x12D0; // bool
    pub const bonus_intellect: usize = 0x12D4; // int32
    pub const movement_speed: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Item_Bottomless_Chalice_Regen {
    pub const health: usize = 0x12C8; // int32
    pub const mana: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Bracer {
    pub const bonus_strength: usize = 0x12C8; // int32
    pub const bonus_agility: usize = 0x12CC; // int32
    pub const bonus_intellect: usize = 0x12D0; // int32
    pub const clock_time: usize = 0x12D4; // int32
    pub const bonus_health_regen: usize = 0x12D8; // float32
    pub const bonus_damage: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Item_Broadsword {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Broom_Handle {
    pub const melee_attack_range: usize = 0x12C8; // int32
    pub const bonus_armor: usize = 0x12CC; // int32
    pub const bonus_damage: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Buckler {
    pub const m_bAppliesToCreeps: usize = 0x12C8; // bool
    pub const bonus_aoe_radius: usize = 0x12CC; // int32
    pub const armor: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Item_Buckler_Effect {
    pub const bonus_aoe_armor: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Item_Bullwhip {
    pub const bonus_health_regen: usize = 0x12C8; // float32
    pub const bonus_mana_regen: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Item_Bullwhip_Buff {
    pub const speed: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Item_Bullwhip_Buff_Thinker {
    pub const speed: usize = 0x12C8; // int32
    pub const m_hTarget: usize = 0x12CC; // CHandle< C_BaseEntity >
    pub const m_nFXIndex: usize = 0x12D0; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Item_Butterfly {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const bonus_agility: usize = 0x12CC; // int32
    pub const bonus_evasion: usize = 0x12D0; // int32
    pub const bonus_attack_speed: usize = 0x12D4; // int32
    pub const bonus_slow_resistance: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Item_Butterfly_Extra {
    pub const bonus_move_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Ceremonial_Robe {
    pub const bonus_mana: usize = 0x12C8; // int32
    pub const aura_radius: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_ChainMail {
    pub const bonus_armor: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Chipped_Vest {
    pub const hp_regen: usize = 0x12C8; // float32
    pub const damage_return_hero: usize = 0x12CC; // int32
    pub const damage_return_creep: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Circlet {
    pub const bonus_all_stats: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Claymore {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Cloak_Of_Flames {
    pub const armor: usize = 0x12C8; // int32
    pub const magic_resistance: usize = 0x12CC; // int32
    pub const radius: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Clumsy_Net {
    pub const all_stats: usize = 0x12C8; // int32
    pub const mana_regen: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Cornucopia {
    pub const bonus_health_regen: usize = 0x12C8; // float32
    pub const bonus_mana_regen: usize = 0x12CC; // float32
    pub const bonus_damage: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Craggy_Coat {
    pub const armor_bonus: usize = 0x12C8; // int32
    pub const attack_speed: usize = 0x12CC; // int32
    pub const bonus_health: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_CraniumBasher {
    pub const bonus_strength: usize = 0x12C8; // int32
    pub const bash_chance_melee: usize = 0x12CC; // int32
    pub const bash_chance_ranged: usize = 0x12D0; // int32
    pub const bash_duration: usize = 0x12D4; // float32
    pub const bash_cooldown: usize = 0x12D8; // float32
    pub const bonus_chance_damage: usize = 0x12DC; // int32
    pub const bonus_damage: usize = 0x12E0; // int32
    pub const m_InFlightAttackRecords: usize = 0x12E8; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Item_Crimson_Guard {
    pub const bonus_health: usize = 0x12C8; // int32
    pub const bonus_health_regen: usize = 0x12CC; // float32
    pub const bonus_armor: usize = 0x12D0; // float32
    pub const block_damage_ranged: usize = 0x12D4; // int32
    pub const block_damage_melee: usize = 0x12D8; // int32
    pub const block_chance: usize = 0x12DC; // int32
    pub const bonus_aoe_radius: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_Item_Crimson_Guard_Extra {
    pub const block_chance_active: usize = 0x12C8; // int32
    pub const block_damage_active: usize = 0x12CC; // int32
    pub const strength_pct: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Crown {
    pub const bonus_all_stats: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Cyclone {
    pub const bonus_intellect: usize = 0x12C8; // int32
    pub const bonus_mana_regen: usize = 0x12CC; // float32
    pub const bonus_movement_speed: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_DaggerOfRistul {
    pub const aspd: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Dagon {
    pub const bonus_int: usize = 0x12C8; // int32
    pub const bonus_str: usize = 0x12CC; // int32
    pub const bonus_agi: usize = 0x12D0; // int32
    pub const passive_spell_lifesteal: usize = 0x12D4; // float32
    pub const dagon_spell_lifesteal: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Item_DemonEdge {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Demonicon {
    pub const bonus_strength: usize = 0x12C8; // int32
    pub const bonus_intellect: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Desolator {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const bonus_damage_per_kill: usize = 0x12CC; // int32
    pub const bonus_damage_per_assist: usize = 0x12D0; // int32
    pub const max_damage: usize = 0x12D4; // int32
    pub const corruption_duration: usize = 0x12D8; // float32
    pub const m_InFlightAttackRecords: usize = 0x12E0; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Item_Desolator_2 {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const corruption_duration: usize = 0x12CC; // float32
    pub const m_InFlightAttackRecords: usize = 0x12D0; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Item_Diadem {
    pub const bonus_all_stats: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Diffusal_Blade {
    pub const bonus_agility: usize = 0x12C8; // int32
    pub const bonus_intellect: usize = 0x12CC; // int32
    pub const damage_per_burn: usize = 0x12D0; // float32
    pub const feedback_mana_burn: usize = 0x12D4; // int32
    pub const feedback_mana_burn_illusion_melee: usize = 0x12D8; // int32
    pub const feedback_mana_burn_illusion_ranged: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Item_Diffusal_Blade_Slow {
    pub const purge_rate: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Disperser {
    pub const bonus_agility: usize = 0x12C8; // int32
    pub const bonus_intellect: usize = 0x12CC; // int32
    pub const bonus_attack_speed: usize = 0x12D0; // int32
    pub const bonus_damage: usize = 0x12D4; // int32
    pub const bonus_mana_regen: usize = 0x12D8; // float32
    pub const damage_per_burn: usize = 0x12DC; // float32
    pub const feedback_mana_burn: usize = 0x12E0; // int32
    pub const feedback_mana_burn_illusion_melee: usize = 0x12E4; // int32
    pub const feedback_mana_burn_illusion_ranged: usize = 0x12E8; // int32
}

pub mod CDOTA_Modifier_Item_Disperser_Attack_Debuff {
    pub const attack_speed_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Disperser_Evasion_Buff {
    pub const purge_bonus_miss_rate: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Disperser_Movespeed_Buff {
    pub const movement_speed_buff_rate: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Item_DivineRapier {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_DragonLance {
    pub const bonus_agility: usize = 0x12C8; // int32
    pub const bonus_strength: usize = 0x12CC; // int32
    pub const base_attack_range: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Dragon_Scale {
    pub const bonus_armor: usize = 0x12C8; // int32
    pub const bonus_hp_regen: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_DuelistGloves {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
    pub const bonus_damage: usize = 0x12CC; // int32
    pub const radius: usize = 0x12D0; // float32
    pub const actual_attack_speed: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_DustofAppearance {
    pub const movespeed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_DustofAppearance_Thinker {
    pub const radius: usize = 0x12C8; // float32
    pub const duration: usize = 0x12CC; // float32
    pub const linger_duration: usize = 0x12D0; // float32
    pub const movespeed: usize = 0x12D4; // int32
    pub const damage: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Item_Dynamite_Jacket {
    pub const bonus_armor: usize = 0x12C8; // int32
    pub const radius: usize = 0x12CC; // int32
    pub const damage: usize = 0x12D0; // int32
    pub const blind_duration: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Item_EagleEye {
    pub const radius: usize = 0x12C8; // int32
    pub const bonus_agi: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_EagleHorn {
    pub const bonus_agility: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_EchoSabre {
    pub const bonus_intellect: usize = 0x12C8; // int32
    pub const bonus_attack_speed: usize = 0x12CC; // int32
    pub const bonus_damage: usize = 0x12D0; // int32
    pub const bonus_mana_regen: usize = 0x12D4; // float32
    pub const bonus_strength: usize = 0x12D8; // int32
    pub const slow_duration: usize = 0x12DC; // float32
    pub const bonus_speed: usize = 0x12E0; // bool
}

pub mod CDOTA_Modifier_Item_Elven_Tunic {
    pub const attack_speed: usize = 0x12C8; // int32
    pub const evasion: usize = 0x12CC; // int32
    pub const movment: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Enchanted_Mango {
    pub const hp_regen: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Item_Enchanted_Quiver {
    pub const bonus_attack_range: usize = 0x12C8; // int32
    pub const bonus_damage: usize = 0x12CC; // int32
    pub const active_bonus_attack_range: usize = 0x12D0; // int32
    pub const m_InFlightAttackRecords: usize = 0x12D8; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Item_EnergyBooster {
    pub const bonus_mana: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Essence_Ring {
    pub const bonus_int: usize = 0x12C8; // int32
    pub const mp_regen: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Item_Essence_Ring_Active {
    pub const health_gain: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Eternal_Shroud {
    pub const bonus_spell_resist: usize = 0x12C8; // int32
    pub const bonus_strength: usize = 0x12CC; // int32
    pub const bonus_armor: usize = 0x12D0; // int32
    pub const bonus_health_regen: usize = 0x12D4; // float32
    pub const mana_restore_pct: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Item_Eternal_Shroud_Barrier {
    pub const barrier_block: usize = 0x12C8; // int32
    pub const barrier_amount: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Ethereal_Blade {
    pub const bonus_strength: usize = 0x12C8; // int32
    pub const bonus_agility: usize = 0x12CC; // int32
    pub const bonus_intellect: usize = 0x12D0; // int32
    pub const spell_amp: usize = 0x12D4; // int32
    pub const spell_lifesteal_amp: usize = 0x12D8; // int32
    pub const mana_regen_multiplier: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Item_Ethereal_Blade_Slow {
    pub const blast_movement_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Ex_Machina {
    pub const bonus_armor: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_EyeOfTheVizier {
    pub const cast_range_bonus: usize = 0x12C8; // int32
    pub const mana_reduction_pct: usize = 0x12CC; // int32
    pub const bonus_mana_regen: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Item_Faded_Broach {
    pub const bonus_mana: usize = 0x12C8; // int32
    pub const bonus_movement_speed: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Faerie_Fire {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Falcon_Blade {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const bonus_damage_per_kill: usize = 0x12CC; // int32
    pub const bonus_damage_per_assist: usize = 0x12D0; // int32
    pub const bonus_health: usize = 0x12D4; // int32
    pub const max_damage: usize = 0x12D8; // int32
    pub const bonus_mana_regen: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Item_Fallen_Sky {
    pub const bonus_strength: usize = 0x12C8; // int32
    pub const bonus_intellect: usize = 0x12CC; // int32
    pub const bonus_health_regen: usize = 0x12D0; // float32
    pub const bonus_mana_regen: usize = 0x12D4; // float32
    pub const blink_damage_cooldown: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Item_Fallen_Sky_Burn {
    pub const burn_dps_buildings: usize = 0x12C8; // int32
    pub const burn_dps_units: usize = 0x12CC; // int32
    pub const burn_interval: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Item_Fallen_Sky_Land {
    pub const impact_radius: usize = 0x12C8; // int32
    pub const land_time: usize = 0x12CC; // float32
    pub const burn_duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Item_Flicker {
    pub const bonus_movement_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Fluffy_Hat {
    pub const bonus_health: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_ForceStaff {
    pub const bonus_health: usize = 0x12C8; // int32
    pub const bonus_intellect: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Force_Boots {
    pub const bonus_movement_speed: usize = 0x12C8; // int32
    pub const hp_regen: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Force_Field {
    pub const m_bAppliesToCreeps: usize = 0x12C8; // bool
    pub const bonus_aoe_radius: usize = 0x12CC; // int32
    pub const self_armor: usize = 0x12D0; // int32
    pub const self_mres: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_Force_Field_Bonus_Aura {
    pub const bonus_aoe_radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Force_Field_Effect {
    pub const bonus_aoe_armor: usize = 0x12C8; // float32
    pub const bonus_aoe_ms: usize = 0x12CC; // float32
    pub const bonus_aoe_mres: usize = 0x12D0; // float32
    pub const self_mres: usize = 0x12D4; // float32
    pub const self_armor: usize = 0x12D8; // float32
    pub const active_reflection_pct: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Item_Fortitude_Ring {
    pub const bonus_health: usize = 0x12C8; // int32
    pub const hp_regen: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Item_Fortitude_Ring_Active {
    pub const damage_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Gauntlets {
    pub const bonus_strength: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_GemOfTrueSight {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_GhostScepter {
    pub const bonus_all_stats: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Giants_Ring {
    pub const bonus_strength: usize = 0x12C8; // int32
    pub const movement_speed: usize = 0x12CC; // int32
    pub const model_scale: usize = 0x12D0; // int32
    pub const damage_radius: usize = 0x12D4; // int32
    pub const pct_str_damage_per_second: usize = 0x12D8; // int32
    pub const tick_rate: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Item_GlimmerCape {
    pub const bonus_magical_armor: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Gloves_Of_Travel {
    pub const attack_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Gossamer_Cape {
    pub const movement_speed: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Item_Grandmasters_Glaive {
    pub const sange_bonus_strength: usize = 0x12C8; // int32
    pub const sange_status_resistance: usize = 0x12CC; // int32
    pub const sange_hp_regen_amp: usize = 0x12D0; // int32
    pub const kaya_bonus_intellect: usize = 0x12D4; // int32
    pub const kaya_spell_amp: usize = 0x12D8; // int32
    pub const kaya_mana_regen_multiplier: usize = 0x12DC; // int32
    pub const kaya_magic_damage_attack: usize = 0x12E0; // int32
    pub const yasha_bonus_agility: usize = 0x12E4; // int32
    pub const yasha_bonus_attack_speed: usize = 0x12E8; // int32
    pub const yasha_movement_speed_percent_bonus: usize = 0x12EC; // int32
    pub const m_iCurrentStance: usize = 0x12F0; // int32
    pub const bonus_strength: usize = 0x12F4; // int32
    pub const bash_chance_melee: usize = 0x12F8; // int32
    pub const bash_chance_ranged: usize = 0x12FC; // int32
    pub const bash_duration: usize = 0x1300; // float32
    pub const bash_cooldown: usize = 0x1304; // float32
    pub const bonus_chance_damage: usize = 0x1308; // int32
    pub const bonus_damage: usize = 0x130C; // int32
    pub const m_InFlightAttackRecords: usize = 0x1310; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Item_GreaterCritical {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const crit_chance: usize = 0x12CC; // int32
    pub const crit_multiplier: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Greater_Faerie_Fire {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Grove_Bow {
    pub const attack_range_bonus: usize = 0x12C8; // int32
    pub const attack_speed_bonus: usize = 0x12CC; // int32
    pub const m_InFlightAttackRecords: usize = 0x12D0; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Item_Grove_Bow_Debuff {
    pub const magic_resistance_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_GuardianShell_Active {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Item_Guardian_Greaves {
    pub const bonus_movement: usize = 0x12C8; // int32
    pub const bonus_mana: usize = 0x12CC; // int32
    pub const bonus_armor: usize = 0x12D0; // int32
    pub const aura_radius: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_Guardian_Greaves_Aura {
    pub const aura_health_regen: usize = 0x12C8; // float32
    pub const aura_armor: usize = 0x12CC; // float32
    pub const aura_health_regen_bonus: usize = 0x12D0; // float32
    pub const aura_armor_bonus: usize = 0x12D4; // float32
    pub const aura_bonus_threshold: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Item_Guardian_Shell {
    pub const all_stats: usize = 0x12C8; // int32
    pub const bonus_armor: usize = 0x12CC; // int32
    pub const counter_cooldown: usize = 0x12D0; // float32
    pub const m_flLastCounterTime: usize = 0x12D4; // GameTime_t
}

pub mod CDOTA_Modifier_Item_Gungir {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const bonus_strength: usize = 0x12CC; // int32
    pub const bonus_agility: usize = 0x12D0; // int32
    pub const bonus_intellect: usize = 0x12D4; // int32
    pub const bonus_hp: usize = 0x12D8; // int32
    pub const chain_chance: usize = 0x12DC; // int32
    pub const chain_strikes: usize = 0x12E0; // int32
    pub const chain_damage: usize = 0x12E4; // int32
    pub const chain_radius: usize = 0x12E8; // int32
    pub const chain_cooldown: usize = 0x12EC; // float32
    pub const m_InFlightAttackRecords: usize = 0x12F0; // CUtlVector< int16 >
    pub const m_ChainTimer: usize = 0x1308; // CountdownTimer
}

pub mod CDOTA_Modifier_Item_HandOfMidas {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
    pub const charge_gain_timer: usize = 0x12CC; // float32
    pub const max_charges: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Harpoon {
    pub const bonus_chance: usize = 0x12C8; // int32
    pub const bonus_chance_damage: usize = 0x12CC; // int32
    pub const bonus_strength: usize = 0x12D0; // int32
    pub const bonus_agility: usize = 0x12D4; // int32
    pub const bonus_intellect: usize = 0x12D8; // int32
    pub const bonus_mana_regen: usize = 0x12DC; // float32
    pub const bonus_damage: usize = 0x12E0; // int32
    pub const bonus_speed: usize = 0x12E4; // int32
    pub const m_InFlightProcAttackRecords: usize = 0x12E8; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Item_Harpoon_EchoSabre_Component {
    pub const slow_duration: usize = 0x12C8; // float32
    pub const bonus_attack_speed: usize = 0x12CC; // int32
    pub const bonus_speed: usize = 0x12D0; // bool
    pub const passive_cooldown: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Item_Harpoon_Pull {
    pub const m_flDistance: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Item_Harpoon_Slow {
    pub const movement_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Havoc_Hammer {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const bonus_strength: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Headdress {
    pub const health_regen: usize = 0x12C8; // float32
    pub const aura_radius: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Headdress_Aura {
    pub const aura_health_regen: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Item_Heart {
    pub const bonus_strength: usize = 0x12C8; // int32
    pub const bonus_health: usize = 0x12CC; // int32
    pub const health_regen_pct: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Item_HeavensHalberd {
    pub const bonus_strength: usize = 0x12C8; // int32
    pub const hp_regen_amp: usize = 0x12CC; // int32
    pub const bonus_evasion: usize = 0x12D0; // int32
    pub const status_resistance: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_Heavy_Blade {
    pub const damage: usize = 0x12C8; // float32
    pub const attack_speed: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_HelmOfIronWill {
    pub const bonus_armor: usize = 0x12C8; // int32
    pub const bonus_regen: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Item_HelmOfTheDominator {
    pub const bonus_stats: usize = 0x12C8; // int32
    pub const bonus_armor: usize = 0x12CC; // int32
    pub const bonus_regen: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Item_HelmOfTheDominator_BonusHealth {
    pub const health_min: usize = 0x12C8; // int32
    pub const new_max: usize = 0x12CC; // int32
    pub const health_bonus: usize = 0x12D0; // int32
    pub const model_scale: usize = 0x12D4; // int32
    pub const creep_bonus_damage: usize = 0x12D8; // int32
    pub const creep_bonus_hp_regen: usize = 0x12DC; // int32
    pub const creep_bonus_mp_regen: usize = 0x12E0; // int32
    pub const creep_bonus_armor: usize = 0x12E4; // int32
}

pub mod CDOTA_Modifier_Item_Helm_Of_The_Undying {
    pub const bonus_armor: usize = 0x12C8; // int32
    pub const duration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Item_Helm_Of_The_Undying_Active {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_bPassive: usize = 0x12CC; // bool
}

pub mod CDOTA_Modifier_Item_Hermes_Sandals {
    pub const bonus_movement_speed: usize = 0x12C8; // int32
    pub const bonus_damage_melee: usize = 0x12CC; // int32
    pub const bonus_damage_range: usize = 0x12D0; // int32
    pub const bonus_armor: usize = 0x12D4; // int32
    pub const bonus_agility: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Item_Hermes_Sandals_Active {
    pub const phase_movement_speed: usize = 0x12C8; // int32
    pub const phase_movement_speed_range: usize = 0x12CC; // int32
    pub const m_nFXIndex: usize = 0x12D0; // ParticleIndex_t
    pub const m_bWasRooted: usize = 0x12D4; // bool
}

pub mod CDOTA_Modifier_Item_Holy_Locket {
    pub const aura_radius: usize = 0x12C8; // int32
    pub const bonus_health: usize = 0x12CC; // int32
    pub const bonus_mana: usize = 0x12D0; // int32
    pub const heal_increase: usize = 0x12D4; // int32
    pub const charge_radius: usize = 0x12D8; // int32
    pub const max_charges: usize = 0x12DC; // int32
    pub const bonus_all_stats: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_Item_Holy_Locket_Aura {
    pub const aura_health_regen: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Item_Hood_Of_Defiance {
    pub const bonus_spell_resist: usize = 0x12C8; // int32
    pub const bonus_health_regen: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Item_Hood_Of_Defiance_Barrier {
    pub const barrier_block: usize = 0x12C8; // int32
    pub const barrier_amount: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_HorizonsEquilibrium {
    pub const distance: usize = 0x12C8; // int32
    pub const damage_reduction: usize = 0x12CC; // int32
    pub const attack_lifesteal: usize = 0x12D0; // int32
    pub const spell_lifesteal: usize = 0x12D4; // int32
    pub const creep_lifesteal_reduction_pct: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Item_Hurricane_Pike {
    pub const bonus_health: usize = 0x12C8; // int32
    pub const bonus_intellect: usize = 0x12CC; // int32
    pub const bonus_agility: usize = 0x12D0; // int32
    pub const bonus_strength: usize = 0x12D4; // int32
    pub const base_attack_range: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Item_Hyperstone {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_IcarusWings {
    pub const move_speed: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Item_IcarusWingsBuff {
    pub const status_resistance: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_IcarusWingsDebuff {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Illusionists_Cape_Aura {
    pub const attack_damage_aura: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Illusionsts_Cape {
    pub const bonus_agi: usize = 0x12C8; // int32
    pub const bonus_str: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Imp_Claw {
    pub const crit_multiplier: usize = 0x12C8; // int32
    pub const bonus_damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Infused_Raindrop {
    pub const mana_regen: usize = 0x12C8; // float32
    pub const bonus_armor: usize = 0x12CC; // float32
    pub const bonus_magical_armor: usize = 0x12D0; // int32
    pub const magic_damage_block: usize = 0x12D4; // int32
    pub const min_damage: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Item_InvisibilityEdge {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
    pub const bonus_damage: usize = 0x12CC; // int32
    pub const m_InFlightAttackRecords: usize = 0x12D0; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Item_Iron_Talon {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
    pub const bonus_armor: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Item_IronwoodBranch {
    pub const bonus_all_stats: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Ironwood_tree {
    pub const bonus_all_stats: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Javelin {
    pub const bonus_chance: usize = 0x12C8; // int32
    pub const bonus_chance_damage: usize = 0x12CC; // int32
    pub const m_InFlightAttackRecords: usize = 0x12D0; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Item_Kaya {
    pub const bonus_intellect: usize = 0x12C8; // int32
    pub const spell_amp: usize = 0x12CC; // int32
    pub const mana_regen_multiplier: usize = 0x12D0; // int32
    pub const spell_lifesteal_amp: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_Kaya_And_Sange {
    pub const bonus_strength: usize = 0x12C8; // int32
    pub const hp_regen_amp: usize = 0x12CC; // int32
    pub const bonus_intellect: usize = 0x12D0; // int32
    pub const spell_amp: usize = 0x12D4; // int32
    pub const mana_regen_multiplier: usize = 0x12D8; // int32
    pub const status_resistance: usize = 0x12DC; // int32
    pub const spell_lifesteal_amp: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_Item_LanceOfPursuit {
    pub const bonus_mana: usize = 0x12C8; // int32
    pub const bonus_strength: usize = 0x12CC; // int32
    pub const backstab_damage: usize = 0x12D0; // int32
    pub const backstab_angle: usize = 0x12D4; // int32
    pub const slow_duration: usize = 0x12D8; // float32
    pub const m_bBackstab: usize = 0x12DC; // bool
}

pub mod CDOTA_Modifier_Item_Lance_of_Pursuit_Slow {
    pub const slow_pct_melee: usize = 0x12C8; // int32
    pub const slow_pct_ranged: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_LesserCritical {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const crit_chance: usize = 0x12CC; // int32
    pub const crit_multiplier: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_LightRobes {
    pub const health_penalty: usize = 0x12C8; // int32
    pub const attack_speed: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Lotus_Orb {
    pub const bonus_health_regen: usize = 0x12C8; // float32
    pub const bonus_mana_regen: usize = 0x12CC; // float32
    pub const bonus_armor: usize = 0x12D0; // int32
    pub const bonus_mana: usize = 0x12D4; // int32
    pub const block_cooldown: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Item_Lotus_Orb_Active {
    pub const m_LastParams: usize = 0x12C8; // CModifierParams
}

pub mod CDOTA_Modifier_Item_Lotus_Orb_ChannelCheck {
    pub const m_hAbility: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_hTarget: usize = 0x12CC; // CHandle< C_BaseEntity >
    pub const m_vLocation: usize = 0x12D0; // Vector
    pub const m_fChannelEnd: usize = 0x12DC; // GameTime_t
    pub const m_bInterrupt: usize = 0x12E0; // bool
    pub const m_bExpired: usize = 0x12E1; // bool
}

pub mod CDOTA_Modifier_Item_Lunar_Crest {
    pub const bonus_health: usize = 0x12C8; // int32
    pub const movement_speed: usize = 0x12CC; // int32
    pub const bonus_spell_resist: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Lunar_Crest_Barrier_Addition {
    pub const barrier_block: usize = 0x12C8; // int32
    pub const barrier_amount: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Lunar_Crest_Barrier_Reduction {
    pub const barrier_block: usize = 0x12C8; // int32
    pub const m_flDamageTotal: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Item_Maelstrom {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const chain_chance: usize = 0x12CC; // int32
    pub const chain_strikes: usize = 0x12D0; // int32
    pub const chain_damage: usize = 0x12D4; // int32
    pub const chain_radius: usize = 0x12D8; // int32
    pub const chain_cooldown: usize = 0x12DC; // float32
    pub const m_InFlightAttackRecords: usize = 0x12E0; // CUtlVector< int16 >
    pub const m_ChainTimer: usize = 0x12F8; // CountdownTimer
}

pub mod CDOTA_Modifier_Item_Mage_Slayer {
    pub const bonus_magical_armor: usize = 0x12C8; // int32
    pub const bonus_attack_speed: usize = 0x12CC; // int32
    pub const bonus_damage: usize = 0x12D0; // int32
    pub const duration: usize = 0x12D4; // float32
    pub const bonus_mana_regen: usize = 0x12D8; // float32
    pub const bonus_intellect: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Item_Mage_Slayer_Debuff {
    pub const spell_amp_debuff: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_MagicStick {
    pub const charge_radius: usize = 0x12C8; // int32
    pub const max_charges: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_MagicWand {
    pub const charge_radius: usize = 0x12C8; // int32
    pub const max_charges: usize = 0x12CC; // int32
    pub const bonus_all_stats: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Magus_Minimus {
    pub const bonus_agility: usize = 0x12C8; // int32
    pub const bonus_intellect: usize = 0x12CC; // int32
    pub const damage_per_burn: usize = 0x12D0; // float32
    pub const feedback_mana_burn: usize = 0x12D4; // int32
    pub const feedback_mana_burn_illusion_melee: usize = 0x12D8; // int32
    pub const feedback_mana_burn_illusion_ranged: usize = 0x12DC; // int32
    pub const aura_radius: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_Item_Magus_Minimus_Aura_Slow {
    pub const max_slow_pct: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Item_Magus_Minimus_Inhibit {
    pub const purge_rate: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_ManaclesOfPower {
    pub const bonus_strength: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_ManaclesOfPower_Effect {
    pub const m_hPartner: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
    pub const leash_distance: usize = 0x12D0; // float32
    pub const leash_limit_multiplier: usize = 0x12D4; // float32
    pub const m_hAnchor: usize = 0x12D8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Item_Mango_Tree {
    pub const m_hTree: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_Timer: usize = 0x12D0; // CountdownTimer
    pub const seconds: usize = 0x12E8; // float32
}

pub mod CDOTA_Modifier_Item_MantaStyle {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
    pub const bonus_strength: usize = 0x12CC; // int32
    pub const bonus_agility: usize = 0x12D0; // int32
    pub const bonus_intellect: usize = 0x12D4; // int32
    pub const bonus_movement_speed: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Item_Mantle {
    pub const bonus_intellect: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_MartyrsPlate {
    pub const duration: usize = 0x12C8; // float32
    pub const magic_resist: usize = 0x12CC; // int32
    pub const hp_regen: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Item_MartyrsPlate_Aura {
    pub const aura_radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_MartyrsPlate_Effect {
    pub const damage_redirection: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_MaskOfDeath {
    pub const lifesteal_percent: usize = 0x12C8; // int32
    pub const creep_lifesteal_reduction_pct: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_MaskOfDispair {
    pub const bonus_strength: usize = 0x12C8; // int32
    pub const bonus_armor: usize = 0x12CC; // float32
    pub const spell_lifesteal: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_MaskOfMadness {
    pub const lifesteal_percent: usize = 0x12C8; // int32
    pub const bonus_attack_speed: usize = 0x12CC; // int32
    pub const bonus_damage: usize = 0x12D0; // int32
    pub const creep_lifesteal_reduction_pct: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_Mechanical_Arm {
    pub const m_bAppliesToCreeps: usize = 0x12C8; // bool
    pub const bat: usize = 0x12CC; // float32
    pub const stun_duration: usize = 0x12D0; // float32
    pub const stun_chance: usize = 0x12D4; // int32
    pub const m_nAttack: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Item_Medallion_Of_Courage {
    pub const bonus_armor: usize = 0x12C8; // int32
    pub const bonus_mana_regen_pct: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Item_Medallion_Of_Courage_Armor_Addition {
    pub const bonus_armor: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Medallion_Of_Courage_Armor_Reduction {
    pub const armor_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Mekansm {
    pub const aura_radius: usize = 0x12C8; // int32
    pub const bonus_armor: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Mekansm_Aura {
    pub const aura_health_regen: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Item_MeteorHammer {
    pub const bonus_all_stats: usize = 0x12C8; // int32
    pub const bonus_health_regen: usize = 0x12CC; // float32
    pub const bonus_mana_regen: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Item_MeteorHammer_Burn {
    pub const burn_dps_buildings: usize = 0x12C8; // int32
    pub const burn_dps_units: usize = 0x12CC; // int32
    pub const burn_interval: usize = 0x12D0; // float32
    pub const burn_slow: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_MeteorHammer_Land {
    pub const impact_radius: usize = 0x12C8; // int32
    pub const land_time: usize = 0x12CC; // float32
    pub const burn_duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Item_Mind_Breaker {
    pub const magic_damage: usize = 0x12C8; // int32
    pub const attack_speed: usize = 0x12CC; // int32
    pub const duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Item_Minotaur_Horn {
    pub const bonus_strength: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Mirror_Shield {
    pub const m_LastParams: usize = 0x12C8; // CModifierParams
    pub const all_stats: usize = 0x14A0; // int32
    pub const reflect_chance: usize = 0x14A4; // int32
    pub const m_bProcSuccessful: usize = 0x14A8; // bool
}

pub mod CDOTA_Modifier_Item_Misericorde {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const missing_hp: usize = 0x12CC; // int32
    pub const damage: usize = 0x12D0; // int32
    pub const aspd: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_MithrilHammer {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Mjollnir {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const bonus_attack_speed: usize = 0x12CC; // int32
    pub const chain_chance: usize = 0x12D0; // int32
    pub const chain_strikes: usize = 0x12D4; // int32
    pub const chain_damage: usize = 0x12D8; // int32
    pub const chain_radius: usize = 0x12DC; // int32
    pub const chain_damage_per_charge: usize = 0x12E0; // int32
    pub const max_charges: usize = 0x12E4; // int32
    pub const chain_cooldown: usize = 0x12E8; // float32
    pub const m_InFlightAttackRecords: usize = 0x12F0; // CUtlVector< int16 >
    pub const m_ChainTimer: usize = 0x1308; // CountdownTimer
}

pub mod CDOTA_Modifier_Item_Mjollnir_Static {
    pub const static_chance: usize = 0x12D0; // int32
    pub const static_strikes: usize = 0x12D4; // int32
    pub const static_damage: usize = 0x12D8; // int32
    pub const static_primary_radius: usize = 0x12DC; // int32
    pub const static_seconary_radius: usize = 0x12E0; // int32
    pub const static_radius: usize = 0x12E4; // int32
    pub const static_cooldown: usize = 0x12E8; // float32
    pub const chain_damage_per_charge: usize = 0x12EC; // int32
}

pub mod CDOTA_Modifier_Item_MonkeyKingBar {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
    pub const bonus_chance: usize = 0x12CC; // int32
    pub const bonus_chance_damage: usize = 0x12D0; // int32
    pub const bonus_damage: usize = 0x12D4; // int32
    pub const m_InFlightAttackRecords: usize = 0x12D8; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Item_Moonshard {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
    pub const bonus_night_vision: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Moonshard_Consumed {
    pub const consumed_bonus: usize = 0x12C8; // int32
    pub const consumed_bonus_night_vision: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_MuertasGun {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const bonus_cast_pct: usize = 0x12CC; // int32
    pub const bonus_spell_amp: usize = 0x12D0; // int32
    pub const health_pct_penalty: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_Mysterious_Hat {
    pub const bonus_health: usize = 0x12C8; // int32
    pub const spell_amp: usize = 0x12CC; // int32
    pub const manacost_reduction: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_MysticStaff {
    pub const bonus_intellect: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Naginata {
    pub const bonus_intellect: usize = 0x12C8; // int32
    pub const bonus_attack_speed: usize = 0x12CC; // int32
    pub const bonus_damage: usize = 0x12D0; // int32
    pub const bonus_health: usize = 0x12D4; // int32
    pub const bonus_mana_regen: usize = 0x12D8; // float32
    pub const cooldown_reduction_percent: usize = 0x12DC; // int32
    pub const bonus_chance: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_Item_Necronomicon {
    pub const bonus_strength: usize = 0x12C8; // int32
    pub const bonus_mana_regen: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Item_Necronomicon_2 {
    pub const bonus_mana_regen: usize = 0x12C8; // float32
    pub const bonus_strength: usize = 0x12CC; // int32
    pub const aura_radius: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Necronomicon_3 {
    pub const bonus_mana_regen: usize = 0x12C8; // float32
    pub const bonus_strength: usize = 0x12CC; // int32
    pub const aura_radius: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Necronomicon_Archer_Aura {
    pub const ms_bonus: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Necronomicon_Mana_Aura {
    pub const aura_mana_regen: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Item_Necronomicon_Mana_Aura_2 {
    pub const aura_mana_regen: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Item_Necronomicon_Mana_Aura_3 {
    pub const aura_mana_regen: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Item_Nether_Shawl {
    pub const bonus_magic_resistance: usize = 0x12C8; // int32
    pub const bonus_spell_amp: usize = 0x12CC; // int32
    pub const bonus_armor: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Ninja_Gear {
    pub const bonus_agility: usize = 0x12C8; // int32
    pub const passive_movement_bonus: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_NullTalisman {
    pub const bonus_strength: usize = 0x12C8; // int32
    pub const bonus_agility: usize = 0x12CC; // int32
    pub const bonus_intellect: usize = 0x12D0; // int32
    pub const bonus_mana_regen: usize = 0x12D4; // float32
    pub const bonus_max_mana_percentage: usize = 0x12D8; // int32
    pub const clock_time: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Item_Nullifier_Slow {
    pub const slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Oakheart {
    pub const strength: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_OblivionStaff {
    pub const bonus_intellect: usize = 0x12C8; // int32
    pub const bonus_attack_speed: usize = 0x12CC; // int32
    pub const bonus_damage: usize = 0x12D0; // int32
    pub const bonus_mana_regen: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Item_Occult_Bracelet {
    pub const bonus_all_stats: usize = 0x12C8; // int32
    pub const stack_limit: usize = 0x12CC; // int32
    pub const stack_duration: usize = 0x12D0; // float32
    pub const m_flStackDieTime: usize = 0x12D4; // GameTime_t
    pub const magic_resistance: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Item_Occult_Bracelet_Stack {
    pub const mana_regen: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Item_Ocean_Heart {
    pub const water_hp_regen: usize = 0x12C8; // float32
    pub const water_mp_regen: usize = 0x12CC; // float32
    pub const all_stats: usize = 0x12D0; // int32
    pub const m_bInRiver: usize = 0x12D4; // bool
}

pub mod CDOTA_Modifier_Item_Octarine_Core {
    pub const bonus_health: usize = 0x12C8; // int32
    pub const bonus_mana: usize = 0x12CC; // int32
    pub const bonus_cooldown: usize = 0x12D0; // float32
    pub const bonus_health_regen: usize = 0x12D4; // float32
    pub const bonus_mana_regen: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Item_OgreAxe {
    pub const bonus_strength: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_OgreSealTotem {
    pub const bonus_strength: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Orb_Of_Corrosion {
    pub const health_bonus: usize = 0x12C8; // int32
    pub const duration: usize = 0x12CC; // float32
    pub const m_InFlightAttackRecords: usize = 0x12D0; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Item_Orb_Of_Destruction {
    pub const duration: usize = 0x12C8; // float32
    pub const m_InFlightAttackRecords: usize = 0x12D0; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Item_Orb_of_Pestilence {
    pub const poison_movement_speed_melee: usize = 0x12C8; // int32
    pub const poison_movement_speed_range: usize = 0x12CC; // int32
    pub const poison_duration: usize = 0x12D0; // float32
    pub const poison_damage: usize = 0x12D4; // float32
    pub const armor: usize = 0x12D8; // int32
    pub const hp_regen: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Item_Orb_of_Pestilence_Slow {
    pub const duration: usize = 0x12C8; // float32
    pub const slow: usize = 0x12CC; // int32
    pub const damage: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Item_Orb_of_Venom {
    pub const poison_movement_speed_melee: usize = 0x12C8; // int32
    pub const poison_movement_speed_range: usize = 0x12CC; // int32
    pub const poison_duration: usize = 0x12D0; // float32
    pub const poison_damage_melee: usize = 0x12D4; // float32
    pub const poison_damage_range: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Item_Orb_of_Venom_Slow {
    pub const duration: usize = 0x12C8; // float32
    pub const slow: usize = 0x12CC; // float32
    pub const damage: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Item_OrchidMalevolence {
    pub const bonus_intellect: usize = 0x12C8; // int32
    pub const bonus_mana_regen: usize = 0x12CC; // float32
    pub const bonus_health_regen: usize = 0x12D0; // float32
    pub const bonus_damage: usize = 0x12D4; // int32
    pub const bonus_attack_speed: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Item_Overwhelming_Blink {
    pub const bonus_strength: usize = 0x12C8; // int32
    pub const blink_damage_cooldown: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Item_Overwhelming_Blink_Debuff {
    pub const caster_strength: usize = 0x12C8; // float32
    pub const movement_slow: usize = 0x12CC; // int32
    pub const attack_slow: usize = 0x12D0; // int32
    pub const damage_pct_over_time: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Item_Paintball {
    pub const movespeed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Paintball_Debuff {
    pub const dps: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Paladin_Sword {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const bonus_lifesteal: usize = 0x12CC; // int32
    pub const bonus_spell_lifesteal: usize = 0x12D0; // int32
    pub const bonus_amp: usize = 0x12D4; // int32
    pub const creep_lifesteal_reduction_pct: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Item_Panic_Button {
    pub const bonus_health: usize = 0x12C8; // int32
    pub const health_threshold: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Item_Pavise {
    pub const bonus_armor: usize = 0x12C8; // int32
    pub const bonus_mana_regen: usize = 0x12CC; // float32
    pub const bonus_health: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Pavise_Shield {
    pub const absorb_amount: usize = 0x12C8; // int32
    pub const m_nAbsorbRemaining: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Penta_Edged_Sword {
    pub const damage: usize = 0x12C8; // int32
    pub const maim_chance: usize = 0x12CC; // int32
    pub const maim_duration: usize = 0x12D0; // float32
    pub const melee_attack_range: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_Perseverance {
    pub const bonus_health_regen: usize = 0x12C8; // float32
    pub const bonus_mana_regen: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Item_PhaseBoots {
    pub const bonus_movement_speed: usize = 0x12C8; // int32
    pub const bonus_damage_melee: usize = 0x12CC; // int32
    pub const bonus_damage_range: usize = 0x12D0; // int32
    pub const damage_block_melee: usize = 0x12D4; // int32
    pub const damage_block_ranged: usize = 0x12D8; // int32
    pub const block_chance: usize = 0x12DC; // int32
    pub const bonus_attack_speed: usize = 0x12E0; // int32
    pub const bonus_armor: usize = 0x12E4; // int32
}

pub mod CDOTA_Modifier_Item_PhaseBoots_Active {
    pub const phase_movement_speed: usize = 0x12C8; // int32
    pub const phase_movement_speed_range: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Philosophers_Stone {
    pub const bonus_mana: usize = 0x12C8; // int32
    pub const bonus_gpm: usize = 0x12CC; // int32
    pub const bonus_damage: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Phoenix_Ash {
    pub const health_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Phylactery {
    pub const bonus_health: usize = 0x12C8; // int32
    pub const bonus_mana: usize = 0x12CC; // int32
    pub const bonus_mana_regen: usize = 0x12D0; // float32
    pub const bonus_spell_damage: usize = 0x12D4; // int32
    pub const slow_duration: usize = 0x12D8; // float32
    pub const bonus_all_stats: usize = 0x12DC; // int32
    pub const vecActivatedAbilities: usize = 0x12E0; // CUtlVector< C_DOTABaseAbility* >
}

pub mod CDOTA_Modifier_Item_Phylactery_Slow {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Pipe {
    pub const bonus_all_stats: usize = 0x12C8; // int32
    pub const health_regen: usize = 0x12CC; // float32
    pub const magic_resistance: usize = 0x12D0; // int32
    pub const aura_radius: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_Pipe_Aura {
    pub const aura_health_regen: usize = 0x12C8; // float32
    pub const magic_resistance_aura: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Pipe_Barrier {
    pub const barrier_amount: usize = 0x12C8; // int32
    pub const barrier_block: usize = 0x12CC; // int32
    pub const barrier_block_creep: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Pirate_Hat {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
    pub const bonus_ms: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_PlaneswalkersCloak {
    pub const bonus_magical_armor: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_PlateMail {
    pub const bonus_armor: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_PogoStick {
    pub const bonus_health: usize = 0x12C8; // int32
    pub const bonus_mana: usize = 0x12CC; // int32
    pub const bonus_intellect: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_PointBooster {
    pub const bonus_mana: usize = 0x12C8; // int32
    pub const bonus_health: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_PoorMansShield {
    pub const damage_block_melee: usize = 0x12C8; // int32
    pub const damage_block_ranged: usize = 0x12CC; // int32
    pub const block_chance: usize = 0x12D0; // int32
    pub const bonus_agility: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_Possessed_Mask {
    pub const primary_attribute: usize = 0x12C8; // int32
    pub const lifesteal: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_PowerTreads {
    pub const bonus_movement_speed_ranged: usize = 0x12C8; // int32
    pub const bonus_movement_speed_melee: usize = 0x12CC; // int32
    pub const bonus_attack_speed: usize = 0x12D0; // int32
    pub const bonus_stat: usize = 0x12D4; // int32
    pub const bonus_damage: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Item_Princes_Knife {
    pub const duration: usize = 0x12C8; // float32
    pub const projectile_bonus: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Princes_Knife_Hex {
    pub const movespeed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Psychic_Headband {
    pub const intelligence_pct: usize = 0x12C8; // int32
    pub const cast_range: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Pupils_gift {
    pub const secondary_stats: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Quarterstaff {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const bonus_speed: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_QuellingBlade {
    pub const damage_bonus: usize = 0x12C8; // int32
    pub const damage_bonus_ranged: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Quickening_Charm {
    pub const bonus_cooldown: usize = 0x12C8; // int32
    pub const bonus_health_regen: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Quicksilver_Amulet {
    pub const base_attack: usize = 0x12C8; // int32
    pub const bonus_attack: usize = 0x12CC; // int32
    pub const base_movement: usize = 0x12D0; // int32
    pub const bonus_movement: usize = 0x12D4; // int32
    pub const projectile_increase: usize = 0x12D8; // int32
    pub const anim_increase: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Item_Radiance {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const evasion: usize = 0x12CC; // int32
    pub const upgrade_day_vision: usize = 0x12D0; // int32
    pub const aura_radius: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_Radiance_Debuff {
    pub const aura_damage: usize = 0x12D0; // int32
    pub const aura_damage_upgrade_bonus: usize = 0x12D4; // int32
    pub const aura_damage_illusions: usize = 0x12D8; // int32
    pub const blind_pct: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Item_Reaver {
    pub const bonus_strength: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_RefresherOrb {
    pub const bonus_health_regen: usize = 0x12C8; // int32
    pub const bonus_mana_regen: usize = 0x12CC; // float32
    pub const bonus_damage: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_RefresherShard {
    pub const bonus_health_regen: usize = 0x12C8; // int32
    pub const bonus_mana_regen: usize = 0x12CC; // float32
    pub const bonus_damage: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Repair_Kit {
    pub const hp_regen: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Revenants_Brooch {
    pub const bonus_armor: usize = 0x12C8; // int32
    pub const projectile_speed: usize = 0x12CC; // int32
    pub const bonus_intellect: usize = 0x12D0; // int32
    pub const bonus_attack_speed: usize = 0x12D4; // int32
    pub const mana_cost: usize = 0x12D8; // int32
    pub const slow_duration: usize = 0x12DC; // int32
    pub const spell_amp_duration: usize = 0x12E0; // int32
    pub const damage_penalty: usize = 0x12E4; // int32
    pub const passive_cooldown: usize = 0x12E8; // int32
    pub const m_InFlightAttackRecords: usize = 0x12F0; // CUtlVector< int16 >
    pub const m_InFlightWitchBladeAttackRecords: usize = 0x1308; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Item_Revenants_Brooch_Counter {
    pub const active_attack_speed: usize = 0x12C8; // float32
    pub const active_spell_amp: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Item_RingOfAquila {
    pub const bonus_strength: usize = 0x12C8; // int32
    pub const bonus_agility: usize = 0x12CC; // int32
    pub const bonus_intellect: usize = 0x12D0; // int32
    pub const bonus_damage: usize = 0x12D4; // int32
    pub const bonus_armor: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Item_RingOfAquila_Aura {
    pub const m_bAppliesToCreeps: usize = 0x12C8; // bool
    pub const aura_radius: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_RingOfAquila_Aura_Bonus {
    pub const aura_bonus_armor: usize = 0x12C8; // int32
    pub const aura_mana_regen: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Item_RingOfHealth {
    pub const bonus_health_regen: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Item_RingOfProtection {
    pub const bonus_armor: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Item_RingOfRegeneration {
    pub const bonus_health_regen: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Item_RingOfTarrasque {
    pub const bonus_health_regen: usize = 0x12C8; // float32
    pub const bonus_health: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Ring_Of_Basilius {
    pub const aura_radius: usize = 0x12C8; // int32
    pub const mana_regen: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Item_Ring_Of_Basilius_Effect {
    pub const aura_mana_regen: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Item_RobeOfMagi {
    pub const bonus_intellect: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_RodOfAtos {
    pub const bonus_strength: usize = 0x12C8; // int32
    pub const bonus_agility: usize = 0x12CC; // int32
    pub const bonus_intellect: usize = 0x12D0; // int32
    pub const bonus_hp: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_SacredRelic {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_SamplePicker {
    pub const base_gold: usize = 0x12C8; // int32
    pub const gold_per_level: usize = 0x12CC; // int32
    pub const strength_penalty: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Samurai_Tabi {
    pub const bonus_movement_speed: usize = 0x12C8; // int32
    pub const bonus_attack_speed: usize = 0x12CC; // int32
    pub const bonus_all_stats: usize = 0x12D0; // int32
    pub const bonus_damage: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_Samurai_Tabi_Agi {
    pub const stat_per_tick: usize = 0x12C8; // int32
    pub const base_interval: usize = 0x12CC; // float32
    pub const max_tick_count: usize = 0x12D0; // int32
    pub const bonus_all_stats: usize = 0x12D4; // int32
    pub const agi_counter_chance: usize = 0x12D8; // int32
    pub const agi_counter_cooldown: usize = 0x12DC; // float32
    pub const m_flLastCounterTime: usize = 0x12E0; // GameTime_t
    pub const iCurrentTickCount: usize = 0x12E4; // int32
}

pub mod CDOTA_Modifier_Item_Samurai_Tabi_Int {
    pub const stat_per_tick: usize = 0x12C8; // int32
    pub const base_interval: usize = 0x12CC; // float32
    pub const max_tick_count: usize = 0x12D0; // int32
    pub const int_damage_interval: usize = 0x12D4; // float32
    pub const int_damage_range: usize = 0x12D8; // float32
    pub const int_damage: usize = 0x12DC; // float32
    pub const iCurrentTickCount: usize = 0x12E0; // int32
    pub const int_max_targets: usize = 0x12E4; // int32
    pub const m_flNextHit: usize = 0x12E8; // GameTime_t
    pub const bonus_all_stats: usize = 0x12EC; // int32
}

pub mod CDOTA_Modifier_Item_Samurai_Tabi_Str {
    pub const stat_per_tick: usize = 0x12C8; // int32
    pub const bonus_all_stats: usize = 0x12CC; // int32
    pub const base_interval: usize = 0x12D0; // float32
    pub const max_tick_count: usize = 0x12D4; // int32
    pub const str_root_duration: usize = 0x12D8; // float32
    pub const str_root_cooldown: usize = 0x12DC; // float32
    pub const str_root_chance: usize = 0x12E0; // int32
    pub const str_bonus_damage: usize = 0x12E4; // float32
    pub const iCurrentTickCount: usize = 0x12E8; // int32
    pub const m_flLastRootTime: usize = 0x12EC; // GameTime_t
    pub const m_InFlightAttackRecords: usize = 0x12F0; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Item_Sange {
    pub const bonus_strength: usize = 0x12C8; // int32
    pub const hp_regen_amp: usize = 0x12CC; // int32
    pub const status_resistance: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_SangeAndYasha {
    pub const bonus_agility: usize = 0x12C8; // int32
    pub const bonus_attack_speed: usize = 0x12CC; // int32
    pub const movement_speed_percent_bonus: usize = 0x12D0; // int32
    pub const bonus_strength: usize = 0x12D4; // int32
    pub const hp_regen_amp: usize = 0x12D8; // int32
    pub const status_resistance: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Item_Satanic {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const bonus_strength: usize = 0x12CC; // int32
    pub const lifesteal_percent: usize = 0x12D0; // int32
    pub const unholy_lifesteal_percent: usize = 0x12D4; // int32
    pub const creep_lifesteal_reduction_pct: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Item_Satanic_Unholy {
    pub const unholy_lifesteal_percent: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Satchel {
    pub const bonus_all_stats: usize = 0x12C8; // int32
    pub const xp_gain: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_SeedsOfSerenity {
    pub const bonus_health: usize = 0x12C8; // int32
    pub const bonus_health_regen: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Item_SeedsOfSerenity_Active {
    pub const aura_health_regen: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_SeedsOfSerenity_Active_Aura {
    pub const radius: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Item_SheepStick {
    pub const bonus_strength: usize = 0x12C8; // int32
    pub const bonus_agility: usize = 0x12CC; // int32
    pub const bonus_intellect: usize = 0x12D0; // int32
    pub const bonus_mana_regen: usize = 0x12D4; // float32
    pub const bonus_mana: usize = 0x12D8; // int32
    pub const cast_range_bonus: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Item_Shivas_Guard {
    pub const bonus_intellect: usize = 0x12C8; // int32
    pub const bonus_armor: usize = 0x12CC; // int32
    pub const aura_radius: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Shivas_Guard_Aura {
    pub const aura_attack_speed: usize = 0x12C8; // int32
    pub const hp_regen_degen_aura: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Shivas_Guard_Blast {
    pub const blast_movement_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Shivas_Guard_Thinker {
    pub const m_fCurRadius: usize = 0x12C8; // float32
    pub const m_fLastThink: usize = 0x12CC; // GameTime_t
    pub const m_entitiesHit: usize = 0x12D0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_ViewerTimer: usize = 0x12E8; // CountdownTimer
    pub const blast_speed: usize = 0x1300; // int32
    pub const blast_radius: usize = 0x1304; // int32
    pub const blast_damage: usize = 0x1308; // int32
    pub const illusion_multiplier_pct: usize = 0x130C; // int32
    pub const blast_debuff_duration: usize = 0x1310; // float32
}

pub mod CDOTA_Modifier_Item_Silver_Edge {
    pub const bonus_intellect: usize = 0x12C8; // int32
    pub const bonus_mana_regen: usize = 0x12CC; // float32
    pub const bonus_attack_speed: usize = 0x12D0; // int32
    pub const bonus_damage: usize = 0x12D4; // int32
    pub const crit_chance: usize = 0x12D8; // int32
    pub const crit_multiplier: usize = 0x12DC; // int32
    pub const backstab_duration: usize = 0x12E0; // float32
    pub const m_InFlightAttackRecords: usize = 0x12E8; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Item_Skadi {
    pub const bonus_all_stats: usize = 0x12C8; // int32
    pub const bonus_health: usize = 0x12CC; // int32
    pub const bonus_mana: usize = 0x12D0; // int32
    pub const cold_duration: usize = 0x12D4; // float32
    pub const m_InFlightAttackRecords: usize = 0x12D8; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Item_Skadi_Slow {
    pub const cold_slow_melee: usize = 0x12C8; // int32
    pub const cold_attack_slow_melee: usize = 0x12CC; // int32
    pub const cold_slow_ranged: usize = 0x12D0; // int32
    pub const cold_attack_slow_ranged: usize = 0x12D4; // int32
    pub const heal_reduction: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Item_Slime_Vial {
    pub const health_regen: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Slime_Vial_Spill_Debuff {
    pub const spill_movement_speed: usize = 0x12C8; // int32
    pub const spill_attack_speed: usize = 0x12CC; // int32
    pub const spill_self_bonus_armor: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Slime_Vial_Thinker {
    pub const m_fCurRadius: usize = 0x12C8; // float32
    pub const m_fLastThink: usize = 0x12CC; // GameTime_t
    pub const m_entitiesHit: usize = 0x12D0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_ViewerTimer: usize = 0x12E8; // CountdownTimer
    pub const spill_speed: usize = 0x1300; // int32
    pub const spill_radius: usize = 0x1304; // int32
    pub const spill_damage: usize = 0x1308; // int32
    pub const spill_debuff_duration: usize = 0x130C; // float32
}

pub mod CDOTA_Modifier_Item_Slippers {
    pub const bonus_agility: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_SobiMask {
    pub const bonus_mana_regen: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Item_Solar_Crest {
    pub const bonus_mana_regen_pct: usize = 0x12C8; // float32
    pub const bonus_armor: usize = 0x12CC; // int32
    pub const bonus_all_stats: usize = 0x12D0; // int32
    pub const self_movement_speed: usize = 0x12D4; // int32
    pub const nFXIndex: usize = 0x12D8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Item_Solar_Crest_Armor_Addition {
    pub const target_armor: usize = 0x12C8; // int32
    pub const target_attack_speed: usize = 0x12CC; // int32
    pub const target_movement_speed: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Solar_Crest_Armor_Reduction {
    pub const target_armor: usize = 0x12C8; // int32
    pub const target_attack_speed: usize = 0x12CC; // int32
    pub const target_movement_speed: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Sorcerers_Staff {
    pub const mana_regen_amp: usize = 0x12C8; // int32
    pub const spell_amp_creeps: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Soul_Booster {
    pub const bonus_health: usize = 0x12C8; // int32
    pub const bonus_mana: usize = 0x12CC; // int32
    pub const bonus_magical_armor: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Soul_Ring {
    pub const bonus_strength: usize = 0x12C8; // int32
    pub const bonus_armor: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Item_Spark_Of_Courage {
    pub const damage: usize = 0x12C8; // int32
    pub const armor: usize = 0x12CC; // int32
    pub const health_pct: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_SpecialistsArray {
    pub const all_stats: usize = 0x12C8; // int32
    pub const bonus_damage: usize = 0x12CC; // int32
    pub const proc_bonus_damage: usize = 0x12D0; // int32
    pub const count: usize = 0x12D4; // int32
    pub const secondary_target_range_bonus: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Item_SpecialistsArray_ProcDamage {
    pub const proc_bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Sphere {
    pub const bonus_all_stats: usize = 0x12C8; // int32
    pub const bonus_health_regen: usize = 0x12CC; // float32
    pub const bonus_mana_regen: usize = 0x12D0; // float32
    pub const bonus_damage: usize = 0x12D4; // int32
    pub const block_cooldown: usize = 0x12D8; // float32
    pub const upgrade_absorb_duration: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Item_Sphere_Target {
    pub const upgrade_absorb_duration: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Item_Sphere_Upgrade_Absorb {
    pub const damage_absorb: usize = 0x12C8; // int32
    pub const m_bStartedTimer: usize = 0x12CC; // bool
    pub const m_flDamageAbsorbed: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Spider_Legs_Active {
    pub const bonus_movement_speed_active: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Spirit_Vessel {
    pub const bonus_health: usize = 0x12C8; // int32
    pub const bonus_mana_regen: usize = 0x12CC; // float32
    pub const bonus_armor: usize = 0x12D0; // float32
    pub const bonus_all_stats: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_Spirit_Vessel_Damage {
    pub const hp_regen_reduction_enemy: usize = 0x12C8; // int32
    pub const soul_damage_amount: usize = 0x12CC; // int32
    pub const enemy_hp_drain: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Item_Spirit_Vessel_Heal {
    pub const soul_heal_amount: usize = 0x12C8; // int32
    pub const m_fHealingDone: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Item_Spy_Gadget {
    pub const attack_range: usize = 0x12C8; // int32
    pub const cast_range: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Spy_Gadget_Aura {
    pub const scan_cooldown_reduction: usize = 0x12C8; // int32
    pub const aura_range: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Star_Mace {
    pub const movement_speed_pct: usize = 0x12C8; // int32
    pub const bonus_mana_regen: usize = 0x12CC; // float32
    pub const cleave_damage_percent: usize = 0x12D0; // int32
    pub const cleave_starting_width: usize = 0x12D4; // int32
    pub const cleave_ending_width: usize = 0x12D8; // int32
    pub const cleave_distance: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Item_Stormcrafter {
    pub const m_flNextHit: usize = 0x12C8; // GameTime_t
    pub const passive_movement_bonus: usize = 0x12CC; // int32
    pub const range: usize = 0x12D0; // int32
    pub const interval: usize = 0x12D4; // float32
    pub const damage: usize = 0x12D8; // int32
    pub const slow_duration: usize = 0x12DC; // float32
    pub const bonus_mana_regen: usize = 0x12E0; // float32
    pub const max_targets: usize = 0x12E4; // int32
}

pub mod CDOTA_Modifier_Item_StoutShield {
    pub const damage_block_melee: usize = 0x12C8; // int32
    pub const damage_block_ranged: usize = 0x12CC; // int32
    pub const block_chance: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Swift_Blink {
    pub const blink_damage_cooldown: usize = 0x12C8; // float32
    pub const bonus_agility: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Swift_Blink_Buff {
    pub const bonus_movement: usize = 0x12C8; // int32
    pub const bonus_agi_active: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_TalismanOfEvasion {
    pub const bonus_evasion: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Tenderizer {
    pub const bonus_strength: usize = 0x12C8; // int32
    pub const bonus_agility: usize = 0x12CC; // int32
    pub const bash_chance_melee: usize = 0x12D0; // int32
    pub const bash_chance_ranged: usize = 0x12D4; // int32
    pub const bash_duration: usize = 0x12D8; // float32
    pub const bash_cooldown: usize = 0x12DC; // float32
    pub const bonus_chance_damage: usize = 0x12E0; // int32
    pub const bonus_damage: usize = 0x12E4; // int32
    pub const weaken_per_hit: usize = 0x12E8; // int32
    pub const weaken_duration: usize = 0x12EC; // float32
    pub const m_InFlightAttackRecords: usize = 0x12F0; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Item_The_Leveller {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
    pub const bonus_armor: usize = 0x12CC; // int32
    pub const demolish: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Timeless_Relic {
    pub const spell_amp: usize = 0x12C8; // int32
    pub const debuff_amp: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Titan_Sliver {
    pub const base_attack_damage: usize = 0x12C8; // int32
    pub const magic_resistance: usize = 0x12CC; // int32
    pub const status_resistance: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Tome_of_Omniscience {
    pub const bonus_mana: usize = 0x12C8; // int32
    pub const bonus_xpm: usize = 0x12CC; // int32
    pub const bonus_armor: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_TranquilBoots {
    pub const bonus_movement_speed: usize = 0x12C8; // int32
    pub const broken_movement_speed: usize = 0x12CC; // int32
    pub const bonus_armor: usize = 0x12D0; // int32
    pub const bonus_health_regen: usize = 0x12D4; // int32
    pub const break_threshold: usize = 0x12D8; // int32
    pub const break_time: usize = 0x12DC; // int32
    pub const break_count: usize = 0x12E0; // int32
    pub const m_flResetTime: usize = 0x12E4; // float32
}

pub mod CDOTA_Modifier_Item_TranquilBoots2 {
    pub const bonus_movement_speed: usize = 0x12C8; // int32
    pub const broken_movement_speed: usize = 0x12CC; // int32
    pub const bonus_armor: usize = 0x12D0; // int32
    pub const bonus_health_regen: usize = 0x12D4; // int32
    pub const break_threshold: usize = 0x12D8; // int32
    pub const break_time: usize = 0x12DC; // int32
    pub const break_count: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_Item_Tree_Processor {
    pub const bonus_int: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Tree_Processor_Speed {
    pub const health_per_tree: usize = 0x12C8; // int32
    pub const hp_bonus: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Trickster_Cloak {
    pub const evasion: usize = 0x12C8; // int32
    pub const magic_resistance: usize = 0x12CC; // int32
    pub const debuff_limit: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Item_Trident {
    pub const bonus_agility: usize = 0x12C8; // int32
    pub const bonus_attack_speed: usize = 0x12CC; // int32
    pub const movement_speed_percent_bonus: usize = 0x12D0; // int32
    pub const bonus_strength: usize = 0x12D4; // int32
    pub const hp_regen_amp: usize = 0x12D8; // int32
    pub const status_resistance: usize = 0x12DC; // int32
    pub const bonus_intellect: usize = 0x12E0; // int32
    pub const spell_amp: usize = 0x12E4; // int32
    pub const mana_regen_multiplier: usize = 0x12E8; // int32
    pub const magic_damage_attack: usize = 0x12EC; // int32
}

pub mod CDOTA_Modifier_Item_Trusty_Shovel {
    pub const bonus_health: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_UltimateOrb {
    pub const bonus_all_stats: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_UltimateScepter {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const bonus_all_stats: usize = 0x12CC; // int32
    pub const bonus_health: usize = 0x12D0; // int32
    pub const bonus_mana: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_UltimateScepter_Consumed_Alchemist {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const bonus_all_stats: usize = 0x12CC; // int32
    pub const bonus_health: usize = 0x12D0; // int32
    pub const bonus_mana: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_UnstableWand {
    pub const all_stats: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Urn_Damage {
    pub const soul_damage_amount: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Urn_Heal {
    pub const soul_heal_amount: usize = 0x12C8; // int32
    pub const m_fHealingDone: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Item_Urn_Of_Shadows {
    pub const mana_regen: usize = 0x12C8; // float32
    pub const bonus_all_stats: usize = 0x12CC; // int32
    pub const bonus_armor: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Item_Vambrace {
    pub const bonus_primary_stat: usize = 0x12C8; // int32
    pub const bonus_secondary_stat: usize = 0x12CC; // int32
    pub const bonus_magic_resistance: usize = 0x12D0; // int32
    pub const bonus_spell_amp: usize = 0x12D4; // int32
    pub const bonus_attack_speed: usize = 0x12D8; // int32
    pub const m_iStat: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Item_Vampire_Fangs {
    pub const attack_lifesteal: usize = 0x12C8; // int32
    pub const spell_lifesteal: usize = 0x12CC; // int32
    pub const night_vision: usize = 0x12D0; // int32
    pub const creep_lifesteal_reduction_pct: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_Vanguard {
    pub const bonus_health: usize = 0x12C8; // int32
    pub const bonus_health_regen: usize = 0x12CC; // float32
    pub const block_damage_ranged: usize = 0x12D0; // int32
    pub const block_damage_melee: usize = 0x12D4; // int32
    pub const block_chance: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Item_Veil_Of_Discord {
    pub const aura_radius: usize = 0x12C8; // int32
    pub const bonus_all_stats: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Veil_Of_Discord_Debuff {
    pub const spell_amp: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Veil_Of_Discord_Mana {
    pub const aura_mana_regen: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Item_Veil_Of_Discord_Thinker {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const debuff_radius: usize = 0x12CC; // int32
    pub const resist_debuff_duration: usize = 0x12D0; // float32
    pub const m_nHeroesHit: usize = 0x12D4; // int32
    pub const m_bHitInvisibleHero: usize = 0x12D8; // bool
    pub const m_bFirstPulse: usize = 0x12D9; // bool
}

pub mod CDOTA_Modifier_Item_Vengeances_Shadow {
    pub const bonus_health: usize = 0x12C8; // int32
    pub const damage_return: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Venom_Gland {
    pub const primary_attribute: usize = 0x12C8; // int32
    pub const debuff_amp: usize = 0x12CC; // int32
    pub const duration: usize = 0x12D0; // float32
    pub const m_InFlightAttackRecords: usize = 0x12D8; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Item_Vermillion_Robe {
    pub const mana_regen_pct: usize = 0x12C8; // int32
    pub const move_speed_bonus: usize = 0x12CC; // int32
    pub const bonus_stats: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Vermillion_Robe_Flames {
    pub const radius: usize = 0x12C8; // int32
    pub const damage_per_health_missing: usize = 0x12CC; // float32
    pub const delay: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Item_VindicatorsAxe {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
    pub const bonus_damage: usize = 0x12CC; // int32
    pub const bonus_armor: usize = 0x12D0; // int32
    pub const bonus_slow_resist: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_VitalityBooster {
    pub const bonus_health: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Vladmir {
    pub const aura_radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Vladmir_Aura {
    pub const lifesteal_aura: usize = 0x12C8; // int32
    pub const damage_aura: usize = 0x12CC; // int32
    pub const armor_aura: usize = 0x12D0; // float32
    pub const mana_regen_aura: usize = 0x12D4; // float32
    pub const creep_lifesteal_reduction_pct: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Item_VoidStone {
    pub const bonus_mana_regen: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Item_VoidwalkerScythe {
    pub const bonus_agility: usize = 0x12C8; // int32
    pub const attacks: usize = 0x12CC; // int32
    pub const bonus_damage: usize = 0x12D0; // int32
    pub const buff_duration: usize = 0x12D4; // float32
    pub const m_InFlightAttackRecords: usize = 0x12D8; // CUtlVector< int16 >
    pub const m_nCounter: usize = 0x12F0; // int32
}

pub mod CDOTA_Modifier_Item_Voodoo_Mask {
    pub const spell_lifesteal: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Item_WandOfSanctitude {
    pub const all_stats: usize = 0x12C8; // int32
    pub const cast_range_bonus: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_WandOfSanctitude_Active_Thinker {
    pub const radius: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Item_WandOfTheBrine {
    pub const heal_increase: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_WandOfTheBrine_Active {
    pub const movement_speed: usize = 0x12C8; // int32
    pub const interval: usize = 0x12CC; // float32
    pub const damage_reduction: usize = 0x12D0; // int32
    pub const heal: usize = 0x12D4; // int32
    pub const m_nFXIndex: usize = 0x12D8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Item_WardTrueSight {
    pub const true_sight_range: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Ward_Dispenser {
    pub const bonus_health: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Ward_Maker {
    pub const bonus_health: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Warhammer {
    pub const bonus_strength: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Warhammer_Armor {
    pub const armor_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_WindLace {
    pub const movement_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Item_Wind_Waker {
    pub const bonus_intellect: usize = 0x12C8; // int32
    pub const bonus_mana_regen: usize = 0x12CC; // float32
    pub const bonus_movement_speed: usize = 0x12D0; // int32
    pub const bonus_mana: usize = 0x12D4; // int32
    pub const cast_range_bonus: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Item_Witch_Blade {
    pub const bonus_intellect: usize = 0x12C8; // int32
    pub const bonus_attack_speed: usize = 0x12CC; // int32
    pub const bonus_armor: usize = 0x12D0; // int32
    pub const slow_duration: usize = 0x12D4; // float32
    pub const projectile_speed: usize = 0x12D8; // int32
    pub const m_InFlightAttackRecords: usize = 0x12E0; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Item_Witch_Blade_Slow {
    pub const slow: usize = 0x12C8; // int32
    pub const int_damage_multiplier: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Item_Witches_Switch {
    pub const bonus_health_regen: usize = 0x12C8; // float32
    pub const bonus_mana_regen: usize = 0x12CC; // float32
    pub const bonus_armor: usize = 0x12D0; // int32
    pub const aura_radius: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Item_Witches_Switch_Aura_Effect {
    pub const aura_mana_regen: usize = 0x12C8; // float32
    pub const aura_health_regen: usize = 0x12CC; // float32
    pub const aura_armor: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Witless_shako {
    pub const max_health: usize = 0x12C8; // int32
    pub const max_mana: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Wizard_Glass {
    pub const debuff_amp: usize = 0x12C8; // int32
    pub const cooldown_reduction: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_Woodland_Striders {
    pub const bonus_movement_speed: usize = 0x12C8; // int32
    pub const bonus_hp_regen: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Item_WraithBand {
    pub const bonus_strength: usize = 0x12C8; // int32
    pub const bonus_agility: usize = 0x12CC; // int32
    pub const bonus_intellect: usize = 0x12D0; // int32
    pub const bonus_attack_speed: usize = 0x12D4; // int32
    pub const clock_time: usize = 0x12D8; // int32
    pub const bonus_armor: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Item_WraithPact {
    pub const aura_radius: usize = 0x12C8; // int32
    pub const bonus_mana: usize = 0x12CC; // int32
    pub const bonus_health: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_WraithPact_DeathAura {
    pub const armor_penalty_aura: usize = 0x12C8; // int32
    pub const damage_penalty_aura: usize = 0x12CC; // int32
    pub const aura_dps: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Item_WraithPact_Thinker {
    pub const pact_aura_radius: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
    pub const aura_dps: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Item_Yasha {
    pub const bonus_agility: usize = 0x12C8; // int32
    pub const bonus_attack_speed: usize = 0x12CC; // int32
    pub const movement_speed_percent_bonus: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Item_Yasha_And_Kaya {
    pub const bonus_agility: usize = 0x12C8; // int32
    pub const bonus_attack_speed: usize = 0x12CC; // int32
    pub const movement_speed_percent_bonus: usize = 0x12D0; // int32
    pub const bonus_intellect: usize = 0x12D4; // int32
    pub const spell_amp: usize = 0x12D8; // int32
    pub const mana_regen_multiplier: usize = 0x12DC; // int32
    pub const spell_lifesteal_amp: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_Jacket_Blind {
    pub const blind_pct: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Jakiro_DualBreath_Burn {
    pub const m_bFirst: usize = 0x12C8; // bool
    pub const burn_damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Jakiro_DualBreath_Slow {
    pub const slow_movement_speed_pct: usize = 0x12C8; // int32
    pub const slow_attack_speed_pct: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Jakiro_DualBreath_Thinker {
    pub const start_radius: usize = 0x12C8; // int32
    pub const end_radius: usize = 0x12CC; // int32
    pub const m_fStartTime: usize = 0x12D0; // GameTime_t
    pub const m_fTotalTime: usize = 0x12D4; // float32
    pub const m_vCastPosition: usize = 0x12D8; // Vector
}

pub mod CDOTA_Modifier_Jakiro_IcePath_Thinker {
    pub const path_delay: usize = 0x12C8; // float32
    pub const path_radius: usize = 0x12CC; // int32
    pub const m_nDamage: usize = 0x12D0; // int32
    pub const m_flRadius: usize = 0x12D4; // float32
    pub const stun_duration: usize = 0x12D8; // float32
    pub const m_hUnitsHit: usize = 0x12E0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_vPathStart: usize = 0x12F8; // Vector
    pub const m_vPathEnd: usize = 0x1304; // Vector
    pub const m_fStartTime: usize = 0x1310; // GameTime_t
}

pub mod CDOTA_Modifier_Jakiro_LiquidFire {
    pub const duration: usize = 0x12C8; // float32
    pub const m_InFlightAttackRecords: usize = 0x12D0; // CUtlVector< int16 >
    pub const radius: usize = 0x12E8; // int32
    pub const m_nFXIndex: usize = 0x12EC; // ParticleIndex_t
    pub const m_bForceProc: usize = 0x12F0; // bool
}

pub mod CDOTA_Modifier_Jakiro_LiquidFire_Burn {
    pub const slow_attack_speed_pct: usize = 0x12C8; // int32
    pub const damage: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Jakiro_LiquidIce {
    pub const duration: usize = 0x12C8; // float32
    pub const m_InFlightAttackRecords: usize = 0x12D0; // CUtlVector< int16 >
    pub const radius: usize = 0x12E8; // int32
    pub const m_nFXIndex: usize = 0x12EC; // ParticleIndex_t
    pub const m_bForceProc: usize = 0x12F0; // bool
}

pub mod CDOTA_Modifier_Jakiro_LiquidIce_Debuff {
    pub const base_damage: usize = 0x12C8; // int32
    pub const pct_health_damage: usize = 0x12CC; // float32
    pub const movement_slow: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Jakiro_Macropyre_Burn {
    pub const damage: usize = 0x12C8; // int32
    pub const burn_interval: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Jakiro_Macropyre_Ice_EdgeThinker {
    pub const m_vPathDir: usize = 0x12C8; // Vector
    pub const ice_edge_path_radius: usize = 0x12D4; // int32
    pub const cast_range: usize = 0x12D8; // int32
    pub const ice_edge_linger_duration: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Jakiro_Macropyre_Ice_Edge_Slow {
    pub const ice_edge_movement_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Jakiro_Macropyre_Thinker {
    pub const m_vPathDir: usize = 0x12C8; // Vector
    pub const burn_interval: usize = 0x12D4; // float32
    pub const path_radius: usize = 0x12D8; // int32
    pub const cast_range: usize = 0x12DC; // int32
    pub const linger_duration: usize = 0x12E0; // float32
}

pub mod CDOTA_Modifier_Jugg_Caster {
    pub const shard_damage_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Juggernaut_BladeDance {
    pub const blade_dance_crit_mult: usize = 0x12C8; // int32
    pub const blade_dance_crit_chance: usize = 0x12CC; // int32
    pub const blade_dance_lifesteal: usize = 0x12D0; // int32
    pub const m_bHasCrit: usize = 0x12D4; // bool
    pub const m_bWasBladeDanceTriggeredCrit: usize = 0x12D5; // bool
}

pub mod CDOTA_Modifier_Juggernaut_BladeFury {
    pub const blade_fury_radius: usize = 0x12C8; // int32
    pub const blade_fury_damage: usize = 0x12CC; // int32
    pub const blade_fury_damage_tick: usize = 0x12D0; // float32
    pub const m_flTotalAppliedDamage: usize = 0x12D4; // float32
    pub const shard_bonus_move_speed: usize = 0x12D8; // int32
    pub const shard_attack_rate: usize = 0x12DC; // float32
    pub const m_flNextAttack: usize = 0x12E0; // GameTime_t
    pub const m_bIgnoreAttackRestriction: usize = 0x12E4; // bool
}

pub mod CDOTA_Modifier_Juggernaut_Healing_Ward_Aura {
    pub const healing_ward_movespeed_tooltip: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Juggernaut_Healing_Ward_Heal {
    pub const healing_ward_heal_amount: usize = 0x12C8; // int32
    pub const m_nStartingHealthThisInterval: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Juggernaut_Omnislash {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_hLastTarget: usize = 0x12CC; // CHandle< C_BaseEntity >
    pub const m_nJumps: usize = 0x12D0; // int32
    pub const bonus_damage: usize = 0x12D4; // int32
    pub const bonus_attack_speed: usize = 0x12D8; // int32
    pub const omni_slash_radius: usize = 0x12DC; // int32
    pub const attack_rate_multiplier: usize = 0x12E0; // float32
    pub const m_bFirstHit: usize = 0x12E4; // bool
    pub const m_iTotalDamage: usize = 0x12E8; // int32
    pub const m_iHeroDamage: usize = 0x12EC; // int32
    pub const m_iKilledHeroes: usize = 0x12F0; // int32
    pub const m_fNextAttackTime: usize = 0x12F4; // GameTime_t
    pub const m_bScepterCast: usize = 0x12F8; // bool
    pub const m_bReflection: usize = 0x12F9; // bool
    pub const m_bEndNext: usize = 0x12FA; // bool
}

pub mod CDOTA_Modifier_JumpBoots {
    pub const push_length: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_JumpBootsIntrinsic {
    pub const bonus_movement_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_JungleSpirit_RiverRejuvenation {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_JungleSpirit_RiverRejuvenation_Effect {
    pub const distance_from_shrine: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_JungleSpirit_RiverRejuvenation_Regen {
    pub const health_restore: usize = 0x12C8; // int32
    pub const mana_restore: usize = 0x12CC; // int32
    pub const regen_interval: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_JungleSpirit_Storm_Cyclone {
    pub const m_FXIndex: usize = 0x12C8; // CUtlVector< ParticleIndex_t >
    pub const m_hCyclones: usize = 0x12E0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const projectile_distance: usize = 0x12F8; // int32
    pub const projectile_radius: usize = 0x12FC; // int32
    pub const projectile_amount: usize = 0x1300; // int32
    pub const projectile_duration: usize = 0x1304; // float32
    pub const delay: usize = 0x1308; // float32
    pub const m_flDieTime: usize = 0x130C; // GameTime_t
    pub const m_flStartTime: usize = 0x1310; // GameTime_t
    pub const m_vLeftControl: usize = 0x1318; // CUtlVector< Vector >
    pub const m_vSourceLoc: usize = 0x1330; // CUtlVector< Vector >
    pub const m_vTargetLoc: usize = 0x1348; // Vector
}

pub mod CDOTA_Modifier_JungleVarmint_Creator {
    pub const m_hVarmint: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_nClientVarmintEntIndex: usize = 0x12CC; // CEntityIndex
}

pub mod CDOTA_Modifier_JungleVarmint_Dive {
    pub const distance: usize = 0x12C8; // int32
    pub const speed: usize = 0x12CC; // float32
    pub const acceleration: usize = 0x12D0; // float32
    pub const radius: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Jungle_Spirit {
    pub const hCasterList: usize = 0x12C8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const bAcceptingGems: usize = 0x12E0; // bool
}

pub mod CDOTA_Modifier_Jungle_Spirit_Ally_Alert {
    pub const m_flLastHowl: usize = 0x12C8; // GameTime_t
}

pub mod CDOTA_Modifier_Jungle_Spirit_Range_Attack {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const projectile_speed: usize = 0x12CC; // int32
    pub const turn_rate: usize = 0x12D0; // float32
    pub const m_hAttachTarget: usize = 0x12D4; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Jungle_Spirit_Reductions {
    pub const m_vecAlliesNearby: usize = 0x12C8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_nAlliesNearby: usize = 0x12E0; // int32
    pub const stat_reduction_pct: usize = 0x12E4; // int32
    pub const armor_reduction: usize = 0x12E8; // int32
    pub const radius: usize = 0x12EC; // int32
}

pub mod CDOTA_Modifier_Jungle_Spirit_Storm_Cyclone_Debuff {
    pub const m_flStartTime: usize = 0x12C8; // GameTime_t
}

pub mod CDOTA_Modifier_Jungle_Spirit_Storm_Cyclone_Invulnerable {
    pub const nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const projectile_radius: usize = 0x12CC; // int32
    pub const projectile_destruction_radius: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Jungle_Spirit_Volcano_Fire_Strike_Caster {
    pub const damage_modifier: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Keen_Optic {
    pub const bonus_mana_regen: usize = 0x12C8; // float32
    pub const cast_range_bonus: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_KeeperOfTheLight_BlindingLight {
    pub const miss_rate: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_KeeperOfTheLight_BlindingLight_Thinker {
    pub const radius: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_KeeperOfTheLight_Illuminate {
    pub const range: usize = 0x12C8; // int32
    pub const total_damage: usize = 0x12CC; // int32
    pub const radius: usize = 0x12D0; // int32
    pub const channel_vision_radius: usize = 0x12D4; // int32
    pub const channel_vision_interval: usize = 0x12D8; // float32
    pub const channel_vision_duration: usize = 0x12DC; // float32
    pub const channel_vision_step: usize = 0x12E0; // int32
    pub const m_flLastChantTime: usize = 0x12E4; // GameTime_t
    pub const m_vNextVisionLocation: usize = 0x12E8; // Vector
    pub const m_vCastDirection: usize = 0x12F4; // Vector
}

pub mod CDOTA_Modifier_KeeperOfTheLight_ManaLeak {
    pub const mana_leak_pct: usize = 0x12C8; // float32
    pub const vLastPos: usize = 0x12CC; // Vector
    pub const bResetPosition: usize = 0x12D8; // bool
    pub const m_fAccumulatedManaLeak: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_KeeperOfTheLight_Radiant_Bind {
    pub const slow: usize = 0x12C8; // float32
    pub const magic_resistance: usize = 0x12CC; // int32
    pub const vLastPos: usize = 0x12D0; // Vector
    pub const bResetPosition: usize = 0x12DC; // bool
    pub const m_fAccumulatedDistance: usize = 0x12E0; // float32
    pub const m_nFXIndex: usize = 0x12E4; // ParticleIndex_t
    pub const m_nStatusFXIndex: usize = 0x12E8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_KeeperOfTheLight_Recall {
    pub const m_bWasAttacked: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_KeeperOfTheLight_SpiritForm {
    pub const m_nFXIndexA: usize = 0x12C8; // ParticleIndex_t
    pub const movement_speed: usize = 0x12CC; // int32
    pub const cast_range: usize = 0x12D0; // int32
    pub const m_bDidSwapSpiritForm: usize = 0x12D4; // bool
}

pub mod CDOTA_Modifier_KeeperOfTheLight_SpiritForm_Illuminate {
    pub const range: usize = 0x12C8; // int32
    pub const vision_radius: usize = 0x12CC; // int32
    pub const radius: usize = 0x12D0; // int32
    pub const total_damage: usize = 0x12D4; // int32
    pub const max_channel_time: usize = 0x12D8; // float32
    pub const speed: usize = 0x12DC; // float32
    pub const vision_duration: usize = 0x12E0; // float32
    pub const channel_vision_radius: usize = 0x12E4; // int32
    pub const channel_vision_interval: usize = 0x12E8; // float32
    pub const channel_vision_duration: usize = 0x12EC; // float32
    pub const channel_vision_step: usize = 0x12F0; // int32
    pub const m_vCastLoc: usize = 0x12F4; // Vector
    pub const m_vTargetLoc: usize = 0x1300; // Vector
    pub const m_fStartTime: usize = 0x130C; // GameTime_t
    pub const m_qCastAngle: usize = 0x1310; // QAngle
    pub const m_iProjectile: usize = 0x131C; // int32
    pub const m_nFXIndex: usize = 0x1320; // ParticleIndex_t
    pub const m_nFXIndexB: usize = 0x1324; // ParticleIndex_t
    pub const m_vNextVisionLocation: usize = 0x1328; // Vector
    pub const m_vCastDirection: usize = 0x1334; // Vector
}

pub mod CDOTA_Modifier_KeeperOfTheLight_Will_O_Wisp {
    pub const fixed_movement_speed: usize = 0x12C8; // int32
    pub const m_vSource: usize = 0x12CC; // Vector
}

pub mod CDOTA_Modifier_KeeperOfTheLight_Will_O_Wisp_Aura {
    pub const radius: usize = 0x12C8; // int32
    pub const off_duration: usize = 0x12CC; // float32
    pub const off_duration_initial: usize = 0x12D0; // float32
    pub const on_duration: usize = 0x12D4; // float32
    pub const hit_count: usize = 0x12D8; // int32
    pub const m_iAttackCount: usize = 0x12DC; // int32
    pub const m_flNextTimeOn: usize = 0x12E0; // GameTime_t
    pub const m_bActive: usize = 0x12E4; // bool
    pub const m_nFXIndex: usize = 0x12E8; // ParticleIndex_t
    pub const m_nFXIndexB: usize = 0x12EC; // ParticleIndex_t
    pub const m_flNextTime: usize = 0x12F0; // GameTime_t
    pub const m_flCorrectionTime: usize = 0x12F4; // float32
}

pub mod CDOTA_Modifier_Kill {
    pub const m_bHideOnKill: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_Knockback {
    pub const knockback_distance: usize = 0x12C8; // float32
    pub const knockback_height: usize = 0x12CC; // int32
    pub const knockback_duration: usize = 0x12D0; // float32
    pub const m_vHorizOffset: usize = 0x12D4; // Vector
    pub const m_vStartPosition: usize = 0x12E0; // Vector
    pub const m_fCurrentTimeHoriz: usize = 0x12EC; // float32
    pub const m_fCurrentTimeVert: usize = 0x12F0; // float32
    pub const m_bShouldStun: usize = 0x12F4; // bool
    pub const m_bRespectLeash: usize = 0x12F5; // bool
    pub const m_bReversePolarity: usize = 0x12F6; // bool
}

pub mod CDOTA_Modifier_KoboldTaskmaster_SpeedAura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_KoboldTaskmaster_SpeedAura_Bonus {
    pub const bonus_movement_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_KoboldTunneler_ProspectingAura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_KoboldTunneler_ProspectingAura_Money {
    pub const gpm_aura: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Kobold_Disarm {
    pub const duration: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Kunkka_Davy_Jones_Locker_Aura {
    pub const m_flRadius: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Kunkka_Fear {
    pub const m_pEntityToBeAfraidOf: usize = 0x12C8; // CHandle< C_DOTA_BaseNPC >
}

pub mod CDOTA_Modifier_Kunkka_GhostShip_DamageAbsorb {
    pub const m_iAccumulatedDamage: usize = 0x12C8; // int32
    pub const movespeed_bonus: usize = 0x12CC; // int32
    pub const ghostship_absorb: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Kunkka_GhostShip_DamageDelay {
    pub const m_iAccumulatedDamage: usize = 0x12C8; // int32
    pub const m_flDuration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Kunkka_Ghost_Ship_Fleet {
    pub const vCasterOrigin: usize = 0x12C8; // Vector
    pub const vEnd: usize = 0x12D4; // Vector
    pub const hitcount: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_Kunkka_Man_the_Helm {
    pub const tick_interval: usize = 0x12C8; // float32
    pub const forward_move_speed: usize = 0x12CC; // float32
    pub const turn_rate_boosted: usize = 0x12D0; // float32
    pub const turn_rate: usize = 0x12D4; // float32
    pub const hit_radius: usize = 0x12D8; // int32
    pub const knockback_radius: usize = 0x12DC; // int32
    pub const m_bHitFirstUpdate: usize = 0x12E0; // bool
    pub const m_flHitEndTime: usize = 0x12E4; // GameTime_t
    pub const hit_recover_time: usize = 0x12E8; // float32
    pub const m_flTurnBoostProgress: usize = 0x12EC; // float32
    pub const m_flFacingTarget: usize = 0x12F0; // float32
    pub const m_flLastHeroAttackTime: usize = 0x12F4; // GameTime_t
    pub const m_nFXIndex: usize = 0x12F8; // ParticleIndex_t
    pub const m_flGyroshellDurationRemaining: usize = 0x12FC; // float32
    pub const m_flTurnHistory: usize = 0x1300; // CUtlVector< float32 >
    pub const m_vecHeroesHitLastRicochet: usize = 0x1318; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_vecHeroesCredited: usize = 0x1330; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_vecHeroesHitCurrentRicochet: usize = 0x1348; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_vDirection: usize = 0x1360; // Vector
}

pub mod CDOTA_Modifier_Kunkka_No_Quarter_Counter {
    pub const m_nStackCount: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Kunkka_Rumboldened_Immunity {
    pub const model_scale: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Kunkka_Tidebringer {
    pub const m_bTidebringerAttack: usize = 0x12C8; // bool
    pub const cleave_starting_width: usize = 0x12CC; // int32
    pub const cleave_ending_width: usize = 0x12D0; // int32
    pub const cleave_distance: usize = 0x12D4; // int32
    pub const damage_bonus: usize = 0x12D8; // int32
    pub const m_nFXIndex: usize = 0x12DC; // ParticleIndex_t
    pub const cleave_damage: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_Kunkka_Tidebringer_Slow {
    pub const movespeed_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Kunkka_Torrent {
    pub const torrent_damage: usize = 0x12C8; // int32
    pub const damage_tick_interval: usize = 0x12CC; // float32
    pub const percent_instant: usize = 0x12D0; // float32
    pub const flDamagePerTick: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Kunkka_Torrent_Slow {
    pub const movespeed_bonus: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Kunkka_Torrent_Storm {
    pub const torrent_min_distance: usize = 0x12C8; // int32
    pub const torrent_max_distance: usize = 0x12CC; // int32
    pub const m_iExplosionQuadrant: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Kunkka_Torrent_Thinker {
    pub const m_bShowEnemies: usize = 0x12C8; // bool
    pub const m_bTorrentStorm: usize = 0x12C9; // bool
    pub const m_bTorrentStarted: usize = 0x12CA; // bool
    pub const m_fTorrentStartTime: usize = 0x12CC; // GameTime_t
    pub const flDamagePerTick: usize = 0x12D0; // float32
    pub const flFirstDamage: usize = 0x12D4; // float32
    pub const damage_tick_interval: usize = 0x12D8; // float32
    pub const percent_instant: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Kunkka_XMarksTheSpot {
    pub const ally_ms: usize = 0x12C8; // int32
    pub const ally_armor: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Kunkka_XMarksTheSpot_Thinker {
    pub const m_vStartLoc: usize = 0x12C8; // Vector
    pub const m_bIsReflection: usize = 0x12D4; // bool
    pub const m_hEntity: usize = 0x12D8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Lamp_Off {
    pub const m_iTempViewer: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Legion_Commander_Duel {
    pub const reward_damage: usize = 0x12C8; // int32
    pub const scepter_damage_reduction_pct: usize = 0x12CC; // int32
    pub const m_bAttacked: usize = 0x12D0; // bool
    pub const m_bHadAVictor: usize = 0x12D1; // bool
    pub const m_flTimeDuelStart: usize = 0x12D4; // GameTime_t
    pub const m_hPartner: usize = 0x12D8; // CHandle< C_BaseEntity >
    pub const m_flNextTime: usize = 0x12DC; // float32
    pub const hAlreadyHitList: usize = 0x12E0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Legion_Commander_MomentOfCourage {
    pub const trigger_chance: usize = 0x12C8; // int32
    pub const buff_duration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Legion_Commander_MomentOfCourage_Lifesteal {
    pub const hp_leech_percent: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Legion_Commander_OverwhelmingOdds {
    pub const bonus_attack_speed_creeps: usize = 0x12C8; // int32
    pub const bonus_attack_speed: usize = 0x12CC; // int32
    pub const armor_per_creep: usize = 0x12D0; // float32
    pub const armor_per_hero: usize = 0x12D4; // float32
    pub const m_nTotalBonus: usize = 0x12D8; // int32
    pub const m_flBonusArmor: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Legion_Commander_PressTheAttack {
    pub const move_speed: usize = 0x12C8; // int32
    pub const hp_regen: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Legion_Commander_PressTheAttack_Immunity {
    pub const model_scale: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Leshrac_Decrepify {
    pub const magic_amp: usize = 0x12C8; // int32
    pub const slow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Leshrac_Decrepify_Aura {
    pub const magic_amp: usize = 0x12C8; // int32
    pub const slow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Leshrac_Diabolic_Edict {
    pub const num_explosions: usize = 0x12C8; // int32
    pub const radius: usize = 0x12CC; // int32
    pub const m_fExplosions: usize = 0x12D0; // float32
    pub const tower_bonus: usize = 0x12D4; // float32
    pub const m_bTalentActive: usize = 0x12D8; // bool
}

pub mod CDOTA_Modifier_Leshrac_Greater_Lightning_Storm {
    pub const radius: usize = 0x12C8; // int32
    pub const interval: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Leshrac_Lightning_Storm {
    pub const damage: usize = 0x12C8; // int32
    pub const radius: usize = 0x12CC; // int32
    pub const jump_count: usize = 0x12D0; // int32
    pub const slow_duration: usize = 0x12D4; // float32
    pub const jump_delay: usize = 0x12D8; // float32
    pub const hInitialTarget: usize = 0x12DC; // CHandle< C_BaseEntity >
    pub const m_flDamage: usize = 0x12E0; // float32
}

pub mod CDOTA_Modifier_Leshrac_Lightning_Storm_Scepter_Thinker {
    pub const pulse_nova_lightning_interval: usize = 0x12C8; // float32
    pub const radius: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Leshrac_Lightning_Storm_Slow {
    pub const slow_movement_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Leshrac_Pulse_Nova {
    pub const m_bFirst: usize = 0x12C8; // bool
    pub const damage_resistance: usize = 0x12D4; // int32
    pub const bonus_movespeed: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Leshrac_Split_Earth_Thinker {
    pub const m_iCount: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Lesser_NightCrawler_Pounce {
    pub const pounce_distance: usize = 0x12C8; // int32
    pub const pounce_speed: usize = 0x12CC; // float32
    pub const pounce_acceleration: usize = 0x12D0; // float32
    pub const pounce_radius: usize = 0x12D4; // int32
    pub const pounce_damage: usize = 0x12D8; // int32
    pub const leash_duration: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Lich_ChainFrost_OnDeath {
    pub const should_fire_chain_frost_on_death: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Lich_ChainFrost_Slow {
    pub const slow_movement_speed: usize = 0x12C8; // int32
    pub const slow_attack_speed: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Lich_Chain_Frost_Thinker {
    pub const m_nDamageToHeroes: usize = 0x12C8; // int32
    pub const m_nHeroesKilled: usize = 0x12CC; // int32
    pub const m_nJumps: usize = 0x12D0; // int32
    pub const m_bFirstJump: usize = 0x12D4; // bool
    pub const m_hTarget: usize = 0x12D8; // CHandle< C_BaseEntity >
    pub const m_hAvoidTarget: usize = 0x12DC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Lich_DarkSorcery_Buff {
    pub const mana_drain: usize = 0x12C8; // int32
    pub const attack_count: usize = 0x12CC; // int32
    pub const attack_speed: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Lich_FrostArmor {
    pub const armor_bonus: usize = 0x12C8; // int32
    pub const health_regen: usize = 0x12CC; // int32
    pub const slow_duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Lich_FrostArmor_Slow {
    pub const slow_movement_speed: usize = 0x12C8; // int32
    pub const slow_attack_speed: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Lich_FrostAura {
    pub const duration: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Lich_FrostAura_Aura {
    pub const aura_radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Lich_FrostAura_Slow {
    pub const movespeed_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Lich_FrostNova_Slow {
    pub const slow_movement_speed: usize = 0x12C8; // int32
    pub const slow_attack_speed: usize = 0x12CC; // int32
    pub const slow_attack_speed_primary: usize = 0x12D0; // int32
    pub const m_bIsPrimary: usize = 0x12D4; // bool
}

pub mod CDOTA_Modifier_Lich_FrostShield {
    pub const radius: usize = 0x12C8; // int32
    pub const interval: usize = 0x12CC; // float32
    pub const damage_reduction: usize = 0x12D0; // int32
    pub const health_regen: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Lich_FrostShield_Slow {
    pub const movement_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Lich_Ice_Spire {
    pub const aura_radius: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
    pub const m_Timer: usize = 0x12D0; // CountdownTimer
}

pub mod CDOTA_Modifier_Lich_Ice_Spire_Debuff {
    pub const bonus_movespeed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Lich_Sinister_Gaze {
    pub const movement_speed: usize = 0x12C8; // int32
    pub const mana_drain: usize = 0x12CC; // int32
    pub const m_flIntervalRate: usize = 0x12D0; // float32
    pub const m_nFXIndex: usize = 0x12D4; // ParticleIndex_t
    pub const creep_damage: usize = 0x12D8; // float32
    pub const m_bBlasted: usize = 0x12DC; // bool
}

pub mod CDOTA_Modifier_Life_Stealer_Assimilate {
    pub const order_lock_duration: usize = 0x12C8; // float32
    pub const m_fOrderLockDone: usize = 0x12CC; // GameTime_t
    pub const m_nFXIndex: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Life_Stealer_Feast {
    pub const hp_leech_percent: usize = 0x12C8; // float32
    pub const hp_damage_percent: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Life_Stealer_Ghoul_Frenzy {
    pub const attack_speed_bonus: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Life_Stealer_Ghoul_Frenzy_Slow {
    pub const miss_pct: usize = 0x12C8; // int32
    pub const movement_slow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Life_Stealer_Infest {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_nFXIndex: usize = 0x12CC; // int32
    pub const self_regen: usize = 0x12D0; // float32
    pub const m_nStartingHealth: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Life_Stealer_Infest_Creep {
    pub const m_bChangedTeams: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_Life_Stealer_Infest_Effect {
    pub const bonus_movement_speed: usize = 0x12C8; // int32
    pub const bonus_health: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Life_Stealer_Infest_Enemy_Hero {
    pub const attack_rate_enemy: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Life_Stealer_Open_Wounds {
    pub const slow_steps: usize = 0x12C8; // int32[8]
    pub const heal_percent: usize = 0x12E8; // int32
    pub const m_nDamageTracker: usize = 0x12EC; // int32
    pub const damage_threshold: usize = 0x12F0; // int32
    pub const spread_radius: usize = 0x12F4; // int32
    pub const max_health_as_damage_pct: usize = 0x12F8; // int32
}

pub mod CDOTA_Modifier_Life_Stealer_Rage {
    pub const movement_speed_bonus: usize = 0x12C8; // int32
    pub const bonus_armor: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Lina_DragonSlave_Burn {
    pub const dragon_slave_burn: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Lina_FierySoul {
    pub const fiery_soul_attack_speed_bonus: usize = 0x12C8; // int32
    pub const fiery_soul_move_speed_bonus: usize = 0x12CC; // float32
    pub const fiery_soul_max_stacks: usize = 0x12D0; // int32
    pub const fiery_soul_stack_duration: usize = 0x12D4; // float32
    pub const m_nFXIndex: usize = 0x12D8; // ParticleIndex_t
    pub const m_flFierySoulDieTime: usize = 0x12DC; // GameTime_t
}

pub mod CDOTA_Modifier_Lina_Fiery_Cloak {
    pub const magic_resistance: usize = 0x12C8; // int32
    pub const spell_amp: usize = 0x12CC; // int32
    pub const visualzdelta: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Lina_LagunaBlade {
    pub const damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Lina_LagunaBlade_Line {
    pub const vStart: usize = 0x12C8; // Vector
    pub const vEnd: usize = 0x12D4; // Vector
}

pub mod CDOTA_Modifier_Lina_LagunaBlade_Superheated {
    pub const increased_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Lina_LightStrikeArray_Thinker {
    pub const m_iDamage: usize = 0x12C8; // int32
    pub const m_flStunDuration: usize = 0x12CC; // float32
    pub const light_strike_array_aoe: usize = 0x12D0; // int32
    pub const light_strike_array_damage: usize = 0x12D4; // int32
    pub const light_strike_array_stun_duration: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Lion_Arcana_Kill_Effect {
    pub const m_bFrozen: usize = 0x12C8; // bool
    pub const m_bFlail: usize = 0x12C9; // bool
}

pub mod CDOTA_Modifier_Lion_FingerOfDeath {
    pub const m_bIgnoreBonus: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_Lion_ManaDrain {
    pub const mana_per_second: usize = 0x12C8; // int32
    pub const break_distance: usize = 0x12CC; // int32
    pub const tick_interval: usize = 0x12D0; // float32
    pub const movespeed: usize = 0x12D4; // int32
    pub const damage_pct: usize = 0x12D8; // int32
    pub const ally_pct: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Lion_Voodoo {
    pub const movespeed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_LoneDruid_Entangling_Claws {
    pub const attack_amp: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_LoneDruid_Rabid {
    pub const shard_movement_speed_pct: usize = 0x12C8; // int32
    pub const shard_attack_speed: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_LoneDruid_SavageRoar {
    pub const bonus_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_LoneDruid_SpiritBear_AttackCheck {
    pub const m_bCanBeResummoned: usize = 0x12C8; // bool
    pub const m_bCanAttack: usize = 0x12C9; // bool
    pub const m_nFxIndex: usize = 0x12CC; // ParticleIndex_t
    pub const m_nStatusFxIndex: usize = 0x12D0; // ParticleIndex_t
}

pub mod CDOTA_Modifier_LoneDruid_SpiritBear_Defender {
    pub const damage_share: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_LoneDruid_SpiritBear_Demolish {
    pub const bonus_building_damage: usize = 0x12C8; // int32
    pub const true_form_bonus_building_damage: usize = 0x12CC; // int32
    pub const spell_resistance: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_LoneDruid_SpiritBear_Entangle {
    pub const entangle_chance: usize = 0x12C8; // int32
    pub const hero_entangle_chance: usize = 0x12CC; // int32
    pub const hero_duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_LoneDruid_SpiritBear_Entangle_Effect {
    pub const m_iEntangleDamage: usize = 0x12C8; // int32
    pub const interval_rate: usize = 0x12CC; // float32
    pub const damage: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_LoneDruid_SpiritBear_Fetch {
    pub const tick_rate: usize = 0x12C8; // float32
    pub const drag_distance: usize = 0x12CC; // int32
    pub const break_distance: usize = 0x12D0; // int32
    pub const vPreviousLocation: usize = 0x12D4; // Vector
}

pub mod CDOTA_Modifier_LoneDruid_SpiritBear_Fetch_Damage {
    pub const max_ticks: usize = 0x12C8; // int32
    pub const damage: usize = 0x12CC; // float32
    pub const duration: usize = 0x12D0; // float32
    pub const tick_rate: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_LoneDruid_SpiritBear_Fetch_Self {
    pub const self_slow: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
    pub const m_hTarget: usize = 0x12D0; // CHandle< C_BaseEntity >
    pub const m_bFetchingRune: usize = 0x12D4; // bool
    pub const m_vPreviousLoc: usize = 0x12D8; // Vector
    pub const drag_distance: usize = 0x12E4; // int32
    pub const break_distance: usize = 0x12E8; // int32
}

pub mod CDOTA_Modifier_LoneDruid_SpiritLink {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
    pub const lifesteal_percent: usize = 0x12CC; // int32
    pub const armor: usize = 0x12D0; // int32
    pub const armor_sharing: usize = 0x12D4; // int32
    pub const active_bonus: usize = 0x12D8; // int32
    pub const m_hTarget: usize = 0x12DC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_LoneDruid_SpiritLink_Active {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_LoneDruid_SpiritLink_BearFear {
    pub const bonus_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_LoneDruid_TrueForm {
    pub const bonus_armor: usize = 0x12C8; // int32
    pub const bonus_hp: usize = 0x12CC; // int32
    pub const base_attack_time: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_LoneDruid_TrueForm_BattleCry {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const bonus_armor: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Lua {
    pub const m_bHasCustomTransmitterData: usize = 0x13B0; // bool
}

pub mod CDOTA_Modifier_Luna_Eclipse {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const radius: usize = 0x12CC; // int32
    pub const beams: usize = 0x12D0; // int32
    pub const hit_count: usize = 0x12D4; // int32
    pub const m_iBeamDamage: usize = 0x12D8; // int32
    pub const beam_interval: usize = 0x12DC; // float32
    pub const stun_duration: usize = 0x12E0; // float32
    pub const vPosition: usize = 0x12E4; // Vector
    pub const bAreaTarget: usize = 0x12F0; // bool
    pub const m_iTickCount: usize = 0x12F4; // int32
    pub const m_nMoonlightFXIndex: usize = 0x12F8; // ParticleIndex_t
    pub const m_HitTargets: usize = 0x1300; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_flBeamStun: usize = 0x1318; // float32
}

pub mod CDOTA_Modifier_Luna_LucentBeam_Damage_Buff {
    pub const damage_buff_per_beam: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Luna_LunarBlessing {
    pub const radius: usize = 0x12C8; // int32
    pub const bonus_night_vision: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Luna_LunarBlessingAura {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const bonus_damage_self: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Luna_LunarBlessing_Active {
    pub const attack_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Luna_LunarBlessing_Smoke {
    pub const bonus_speed_day: usize = 0x12D8; // int32
    pub const bonus_speed_night: usize = 0x12DC; // int32
    pub const break_distance_day: usize = 0x12E0; // int32
    pub const break_distance_night: usize = 0x12E4; // int32
    pub const visibility_radius: usize = 0x12E8; // int32
    pub const bonus_movement_speed: usize = 0x12EC; // int32
}

pub mod CDOTA_Modifier_Luna_Lunar_Grace {
    pub const attack_speed: usize = 0x12C8; // int32
    pub const night_multiplier: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Luna_MoonGlaive {
    pub const range: usize = 0x12C8; // int32
    pub const bounces: usize = 0x12CC; // int32
    pub const damage_reduction_percent: usize = 0x12D0; // int32
    pub const m_nMoonGlaiveFXIndex: usize = 0x12D4; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Luna_MoonGlaive_Shield {
    pub const rotating_glaives: usize = 0x12C8; // int32
    pub const rotating_glaives_hit_radius: usize = 0x12CC; // float32
    pub const rotating_glaives_speed: usize = 0x12D0; // float32
    pub const rotating_glaives_collision_damage: usize = 0x12D4; // float32
    pub const rotating_glaives_movement_radius: usize = 0x12D8; // float32
    pub const rotating_glaives_damage_reduction: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Lycan_FeralImpulse {
    pub const bonus_hp_regen: usize = 0x12C8; // int32
    pub const bonus_damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Lycan_FeralImpulse_Aura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Lycan_Howl {
    pub const attack_damage_reduction: usize = 0x12C8; // int32
    pub const total_attack_damage_reduction: usize = 0x12CC; // int32
    pub const armor: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Lycan_Shapeshift {
    pub const bonus_night_vision: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
    pub const m_iOriginalAttackCapability: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Lycan_Shapeshift_Speed {
    pub const speed: usize = 0x12C8; // int32
    pub const crit_chance: usize = 0x12CC; // int32
    pub const crit_multiplier: usize = 0x12D0; // int32
    pub const health_bonus: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Lycan_Shard {
    pub const m_flNextSpawn: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Lycan_SummonWolves_Bash {
    pub const bash_chance: usize = 0x12C8; // int32
    pub const bash_duration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Lycan_SummonWolves_BonusDamage {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Lycan_SummonWolves_CriticalStrike {
    pub const m_CooldownTimer: usize = 0x12C8; // CountdownTimer
    pub const maim_chance: usize = 0x12E0; // int32
    pub const maim_duration: usize = 0x12E4; // float32
}

pub mod CDOTA_Modifier_Lycan_SummonWolves_Health {
    pub const bonus_health: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Lycan_SummonWolves_Maim {
    pub const maim_movement_speed: usize = 0x12C8; // int32
    pub const maim_attack_speed: usize = 0x12CC; // int32
    pub const maim_damage: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Lycan_Wolf_Bite_Attack_Range {
    pub const attack_range: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Lycan_Wolf_Bite_Lifesteal {
    pub const lifesteal_percent: usize = 0x12C8; // int32
    pub const lifesteal_range: usize = 0x12CC; // int32
    pub const m_vecTargets: usize = 0x12D0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_MaelstromChain {
    pub const chain_radius: usize = 0x12C8; // int32
    pub const chain_strikes: usize = 0x12CC; // int32
    pub const chain_damage: usize = 0x12D0; // int32
    pub const chain_delay: usize = 0x12D4; // float32
    pub const m_iCurJumpCount: usize = 0x12D8; // int32
    pub const m_vCurTargetLoc: usize = 0x12DC; // Vector
    pub const m_hHitEntities: usize = 0x12E8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Magnataur_Empower {
    pub const bonus_damage_pct: usize = 0x12C8; // int32
    pub const cleave_damage_pct: usize = 0x12CC; // float32
    pub const cleave_starting_width: usize = 0x12D0; // int32
    pub const cleave_ending_width: usize = 0x12D4; // int32
    pub const cleave_distance: usize = 0x12D8; // int32
    pub const self_multiplier: usize = 0x12DC; // float32
    pub const secondary_cleave_distance: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_Magnataur_Horn_Toss_Slow {
    pub const horn_toss_slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Magnataur_ReversePolarity_Stats {
    pub const stats_per_stack: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Magnataur_Shockwave {
    pub const movement_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Magnataur_Shockwave_Erupt {
    pub const scepter_width: usize = 0x12C8; // int32
    pub const scepter_duration: usize = 0x12CC; // float32
    pub const play_particle: usize = 0x12D0; // bool
    pub const m_vStart: usize = 0x12D4; // Vector
    pub const m_vEnd: usize = 0x12E0; // Vector
}

pub mod CDOTA_Modifier_Magnataur_Shockwave_Eruption_Slow {
    pub const scepter_slow_pct: usize = 0x12C8; // int32
    pub const scepter_armor_pct: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Magnataur_Shockwave_Pull {
    pub const m_vDirection: usize = 0x12C8; // Vector
    pub const m_flEndTime: usize = 0x12D4; // float32
    pub const m_flCurTime: usize = 0x12D8; // float32
    pub const pull_duration: usize = 0x12DC; // float32
    pub const effective_distance: usize = 0x12E0; // float32
}

pub mod CDOTA_Modifier_Magnataur_Skewer_Impact {
    pub const slow_duration: usize = 0x12C8; // float32
    pub const skewer_damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Magnataur_Skewer_Movement {
    pub const m_nProjectileID: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Magnataur_Skewer_Slow {
    pub const slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Magnus_Strength_Of_Joelrak {
    pub const strength_per_stack: usize = 0x12C8; // int32
    pub const max_stacks: usize = 0x12CC; // int32
    pub const stack_duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Marci_CompanionRun_AllyMovespeed {
    pub const ally_movespeed_pct: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Marci_Dispose_Debuff {
    pub const movement_slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Marci_Grapple_VictimMotion {
    pub const impact_damage: usize = 0x12C8; // int32
    pub const debuff_duration: usize = 0x12CC; // float32
    pub const landing_radius: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Marci_Guardian_Buff {
    pub const lifesteal_pct: usize = 0x12C8; // float32
    pub const bonus_damage: usize = 0x12CC; // int32
    pub const bonus_attack_range: usize = 0x12D0; // int32
    pub const max_partner_distance: usize = 0x12D4; // int32
    pub const max_partner_penalty: usize = 0x12D8; // int32
    pub const creep_lifesteal_reduction_pct: usize = 0x12DC; // int32
    pub const m_fLifestealPct: usize = 0x12E0; // float32
    pub const m_fDamageAbsorbed: usize = 0x12E4; // float32
    pub const m_bActive: usize = 0x12E8; // bool
    pub const m_nFxIndex: usize = 0x12EC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Marci_Lunge_Arc {
    pub const m_bTriggeredLandingAnim: usize = 0x12C8; // bool
    pub const m_nMaxJumpDistance: usize = 0x12CC; // int32
    pub const min_jump_distance: usize = 0x12D0; // int32
    pub const max_jump_distance: usize = 0x12D4; // int32
    pub const min_lob_travel_time: usize = 0x12D8; // float32
    pub const max_lob_travel_time: usize = 0x12DC; // float32
    pub const landing_radius: usize = 0x12E0; // int32
    pub const debuff_duration: usize = 0x12E4; // float32
    pub const min_height_above_lowest: usize = 0x12E8; // float32
    pub const min_height_above_highest: usize = 0x12EC; // float32
    pub const min_acceleration: usize = 0x12F0; // float32
    pub const max_acceleration: usize = 0x12F4; // float32
    pub const impact_damage: usize = 0x12F8; // float32
    pub const impact_position_offset: usize = 0x12FC; // int32
    pub const ally_buff_duration: usize = 0x1300; // float32
}

pub mod CDOTA_Modifier_Marci_Lunge_Buff {
    pub const scepter_armor: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Marci_Lunge_Debuff {
    pub const movement_slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Marci_Lunge_TrackingMotion {
    pub const m_nProjectileID: usize = 0x12C8; // int32
    pub const m_nMaxJumpDistance: usize = 0x12CC; // int32
    pub const landing_radius: usize = 0x12D0; // int32
    pub const max_jump_distance: usize = 0x12D4; // int32
    pub const min_jump_distance: usize = 0x12D8; // int32
    pub const target_abort_distance: usize = 0x12DC; // int32
    pub const impact_position_offset: usize = 0x12E0; // int32
    pub const m_flCastDistance: usize = 0x12E4; // float32
    pub const m_nAoEFXIndex: usize = 0x12E8; // ParticleIndex_t
    pub const m_vDestination: usize = 0x12EC; // Vector
    pub const m_hBounceEntity: usize = 0x12F8; // CHandle< C_BaseEntity >
    pub const m_hBounceEntityClient: usize = 0x12FC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Marci_Unleash {
    pub const time_between_flurries: usize = 0x12C8; // float32
    pub const charges_per_flurry: usize = 0x12CC; // int32
    pub const flurry_charge_increment: usize = 0x12D0; // int32
    pub const bonus_movespeed: usize = 0x12D4; // int32
    pub const m_InFlightAttackRecords: usize = 0x12D8; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Marci_Unleash_Flurry {
    pub const flurry_bonus_attack_speed: usize = 0x12C8; // int32
    pub const time_between_flurries: usize = 0x12CC; // float32
    pub const debuff_duration: usize = 0x12D0; // float32
    pub const max_time_window_per_hit: usize = 0x12D4; // float32
    pub const m_bBonusSpeed: usize = 0x12D8; // bool
    pub const m_nFXStackIndex: usize = 0x12DC; // ParticleIndex_t
    pub const m_bIsDoingFlurryAttack: usize = 0x12E0; // bool
    pub const m_bIsDoingFlurryPulseAttack: usize = 0x12E1; // bool
    pub const m_fLastAttackTime: usize = 0x12E4; // GameTime_t
}

pub mod CDOTA_Modifier_Marci_Unleash_FlurryCooldown {
    pub const charges_per_flurry: usize = 0x12C8; // int32
    pub const time_between_flurries: usize = 0x12CC; // float32
    pub const recovery_fixed_attack_rate: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Marci_Unleash_FlurryPulse_Debuff {
    pub const pulse_move_slow_pct: usize = 0x12C8; // float32
    pub const pulse_attack_slow_pct: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Marci_Unleash_Pull {
    pub const shard_push_length: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Mars_ArenaOfBlood {
    pub const radius: usize = 0x12C8; // int32
    pub const width: usize = 0x12CC; // int32
    pub const spear_damage: usize = 0x12D0; // int32
    pub const spear_distance_from_wall: usize = 0x12D4; // int32
    pub const spear_attack_interval: usize = 0x12D8; // float32
    pub const warrior_fade_min_dist: usize = 0x12F8; // float32
    pub const warrior_fade_max_dist: usize = 0x12FC; // float32
}

pub mod CDOTA_Modifier_Mars_ArenaOfBlood_AnimationAura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Mars_ArenaOfBlood_Buff {
    pub const health_regen: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Mars_ArenaOfBlood_BuffAura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Mars_ArenaOfBlood_Leash {
    pub const radius: usize = 0x12C8; // int32
    pub const width: usize = 0x12CC; // float32
    pub const m_vOriginLoc: usize = 0x12D0; // Vector
}

pub mod CDOTA_Modifier_Mars_ArenaOfBlood_Thinker {
    pub const radius: usize = 0x12C8; // float32
    pub const formation_time: usize = 0x12CC; // float32
    pub const m_flInitialZ: usize = 0x12D0; // float32
    pub const m_flFinalZ: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Mars_Bulwark {
    pub const physical_damage_reduction: usize = 0x12C8; // float32
    pub const physical_damage_reduction_side: usize = 0x12CC; // float32
    pub const forward_angle: usize = 0x12D0; // float32
    pub const side_angle: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Mars_Bulwark_Active {
    pub const redirect_chance: usize = 0x12C8; // int32
    pub const redirect_range: usize = 0x12CC; // int32
    pub const forward_angle: usize = 0x12D0; // float32
    pub const side_angle: usize = 0x12D4; // float32
    pub const redirect_speed_penatly: usize = 0x12D8; // int32
    pub const redirect_close_range: usize = 0x12DC; // int32
    pub const m_nPoseParameterWE: usize = 0x12E0; // int32
    pub const m_nPoseParameterNS: usize = 0x12E4; // int32
    pub const m_flLastPoseX: usize = 0x12E8; // float32
    pub const m_flLastPoseY: usize = 0x12EC; // float32
    pub const m_nLastMaxDirection: usize = 0x12F0; // int32
    pub const m_vLastOrigin: usize = 0x12F4; // Vector
    pub const m_flLastGameTime: usize = 0x1300; // GameTime_t
}

pub mod CDOTA_Modifier_Mars_Bulwark_Soldier_Bonus {
    pub const m_nPoseParameterWE: usize = 0x12C8; // int32
    pub const m_nPoseParameterNS: usize = 0x12CC; // int32
    pub const m_flLastPoseX: usize = 0x12D0; // float32
    pub const m_flLastPoseY: usize = 0x12D4; // float32
    pub const m_nLastMaxDirection: usize = 0x12D8; // int32
    pub const m_vLastOrigin: usize = 0x12DC; // Vector
    pub const m_flLastGameTime: usize = 0x12E8; // GameTime_t
}

pub mod CDOTA_Modifier_Mars_Bulwark_Soldier_Thinker {
    pub const m_NextAttack: usize = 0x12C8; // GameTime_t
    pub const soldier_offset: usize = 0x12CC; // int32
    pub const soldier_count: usize = 0x12D0; // int32
    pub const forward_angle: usize = 0x12D4; // int32
    pub const knockback_distance: usize = 0x12D8; // int32
    pub const m_hAllSoldiers: usize = 0x12E0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_flLastStationaryTime: usize = 0x12F8; // GameTime_t
    pub const m_vLastStationaryPosition: usize = 0x12FC; // Vector
    pub const stationary_attack_delay: usize = 0x1308; // float32
}

pub mod CDOTA_Modifier_Mars_GodsRebuke_Crit {
    pub const crit_mult: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Mars_Gods_Rebuke {
    pub const knockback_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Mars_Scepter_Damage {
    pub const scepter_bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Mars_Scepter_Damage_Slow {
    pub const scepter_movement_slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_MaskOfDispair_Berserk {
    pub const berserk_spell_amp: usize = 0x12C8; // int32
    pub const berserk_manacost_reduction: usize = 0x12CC; // int32
    pub const berserk_magic_resistance: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_MaskOfMadness_Berserk {
    pub const berserk_bonus_attack_speed: usize = 0x12C8; // int32
    pub const berserk_bonus_movement_speed: usize = 0x12CC; // int32
    pub const berserk_armor_reduction: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Medusa_ManaShield {
    pub const bonus_mana: usize = 0x12C8; // int32
    pub const absorption_pct: usize = 0x12CC; // float32
    pub const damage_per_mana: usize = 0x12D0; // float32
    pub const illusion_damage_per_mana: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Medusa_Mystic_Snake_Slow {
    pub const movement_slow: usize = 0x12C8; // int32
    pub const turn_slow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Medusa_SplitShot {
    pub const damage_modifier: usize = 0x12C8; // int32
    pub const range: usize = 0x12CC; // int32
    pub const arrow_count: usize = 0x12D0; // int32
    pub const projectile_speed: usize = 0x12D4; // int32
    pub const m_nSplitShotBowFXIndex: usize = 0x12D8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Medusa_StoneGaze {
    pub const radius: usize = 0x12C8; // int32
    pub const vision_cone: usize = 0x12CC; // float32
    pub const duration: usize = 0x12D0; // float32
    pub const speed_boost: usize = 0x12D4; // int32
    pub const m_hHitEntities: usize = 0x12D8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Medusa_StoneGaze_Facing {
    pub const face_duration: usize = 0x12C8; // float32
    pub const stone_duration: usize = 0x12CC; // float32
    pub const duration: usize = 0x12D0; // float32
    pub const vision_cone: usize = 0x12D4; // float32
    pub const m_flAccumulatedTime: usize = 0x12D8; // float32
    pub const ctFacing: usize = 0x12E0; // CountdownTimer
    pub const m_bAlreadyStoned: usize = 0x12F8; // bool
    pub const m_nFXIndex: usize = 0x12FC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Medusa_StoneGaze_Slow {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Medusa_StoneGaze_Stone {
    pub const bonus_physical_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Medusa_StoneGaze_Tracker {
    pub const m_nNumStoned: usize = 0x12C8; // int32
    pub const m_bAnyInvisible: usize = 0x12CC; // bool
}

pub mod CDOTA_Modifier_Meepo_Divided_We_Stand {
    pub const respawn: usize = 0x12C8; // float32
    pub const magic_resist: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Meepo_Fling_Slow {
    pub const movement_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Meepo_Flung {
    pub const m_nHandle: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Meepo_Geostrike_Debuff {
    pub const slow: usize = 0x12C8; // int32
    pub const damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Meepo_MegaMeepo {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const m_flOffset: usize = 0x12CC; // float32
    pub const m_vStartAngles: usize = 0x12D0; // QAngle
    pub const m_vStartLocation: usize = 0x12DC; // Vector
    pub const animation_rate: usize = 0x12E8; // float32
    pub const m_bInterrupted: usize = 0x12EC; // bool
    pub const m_bHasBeenDestroyed: usize = 0x12ED; // bool
    pub const m_bIsHighestMeepo: usize = 0x12EE; // bool
}

pub mod CDOTA_Modifier_Meepo_MegaMeepo_Self {
    pub const base_strength: usize = 0x12C8; // int32
    pub const base_int: usize = 0x12CC; // int32
    pub const base_agi: usize = 0x12D0; // int32
    pub const stats_pct: usize = 0x12D4; // int32
    pub const m_hMegameepoFrame: usize = 0x12D8; // CHandle< C_BaseEntity >
    pub const m_bWasOutOfGame: usize = 0x12DC; // bool
}

pub mod CDOTA_Modifier_Meepo_Petrify {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const hp_restore: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Meepo_Poof_Damage_Sharing {
    pub const damage_share_percentage: usize = 0x12C8; // int32
    pub const damage_share_radius: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Meepo_Poof_Slow {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Meepo_Ransack {
    pub const health_steal_heroes: usize = 0x12C8; // int32
    pub const health_steal_creeps: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_MeltingStrike_Debuff {
    pub const armor_removed: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Miniboss_Reflect {
    pub const passive_reflection_pct: usize = 0x12C8; // int32
    pub const passive_reflection_bonus_per_death: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Miniboss_UnyieldingShield {
    pub const damage_absorb: usize = 0x12C8; // int32
    pub const regen_per_second: usize = 0x12CC; // float32
    pub const regen_bonus_per_death: usize = 0x12D0; // float32
    pub const m_nDamageAbsorbed: usize = 0x12D4; // int32
    pub const m_timeLastTick: usize = 0x12D8; // GameTime_t
    pub const nFXIndex: usize = 0x12DC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Minotaur_Horn_Immune {
    pub const model_scale: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Mirana_Leap {
    pub const leap_distance: usize = 0x12C8; // int32
    pub const shard_radius: usize = 0x12CC; // float32
    pub const shard_radius_end: usize = 0x12D0; // float32
    pub const shard_damage: usize = 0x12D4; // float32
    pub const shard_slow_pct: usize = 0x12D8; // float32
    pub const shard_slow_duration: usize = 0x12DC; // float32
    pub const leap_speed: usize = 0x12E0; // float32
    pub const leap_acceleration: usize = 0x12E4; // float32
    pub const leap_radius: usize = 0x12E8; // int32
    pub const leap_bonus_duration: usize = 0x12EC; // float32
    pub const m_bLaunched: usize = 0x12F0; // bool
}

pub mod CDOTA_Modifier_Mirana_Leap_Buff {
    pub const leap_speedbonus: usize = 0x12C8; // int32
    pub const leap_speedbonus_as: usize = 0x12CC; // int32
    pub const m_bCritUsed: usize = 0x12D0; // bool
}

pub mod CDOTA_Modifier_Mirana_Leap_Slow {
    pub const shard_slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Mirana_MoonlightShadow {
    pub const duration: usize = 0x12D8; // float32
    pub const bonus_movement_speed: usize = 0x12DC; // int32
    pub const evasion: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_Mirana_Starfall_Scepter_Thinker {
    pub const interval_scepter: usize = 0x12C8; // float32
    pub const bInBufferTime: usize = 0x12CC; // bool
    pub const damage: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Mirana_Starfall_Thinker {
    pub const starfall_secondary_radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Misery_Debuff {
    pub const active_mres_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_MjollnirChain {
    pub const chain_radius: usize = 0x12C8; // int32
    pub const chain_strikes: usize = 0x12CC; // int32
    pub const chain_damage: usize = 0x12D0; // int32
    pub const chain_delay: usize = 0x12D4; // float32
    pub const chain_damage_per_charge: usize = 0x12D8; // int32
    pub const m_iCurJumpCount: usize = 0x12DC; // int32
    pub const m_vCurTargetLoc: usize = 0x12E0; // Vector
    pub const m_hHitEntities: usize = 0x12F0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_MonkeyKing_ArcToGround {
    pub const leap_speed: usize = 0x12C8; // float32
    pub const give_up_distance: usize = 0x12CC; // float32
    pub const attackspeed_duration: usize = 0x12D0; // float32
    pub const m_flOriginalZDelta: usize = 0x12D4; // float32
    pub const m_flZDelta: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_MonkeyKing_Bounce {
    pub const jump_damage_cooldown: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_MonkeyKing_BounceLeap {
    pub const give_up_distance: usize = 0x12C8; // float32
    pub const ground_jump_distance: usize = 0x12CC; // float32
    pub const m_flZDelta: usize = 0x12D0; // float32
    pub const perched_day_vision: usize = 0x12D4; // float32
    pub const perched_night_vision: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_MonkeyKing_BouncePerch {
    pub const m_hTree: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_bGroundToTree: usize = 0x12CC; // bool
    pub const m_bTreeToGround: usize = 0x12CD; // bool
    pub const m_bTreeToTree: usize = 0x12CE; // bool
    pub const perched_day_vision: usize = 0x12D0; // float32
    pub const perched_night_vision: usize = 0x12D4; // float32
    pub const m_bAbilityIsStolen: usize = 0x12D8; // bool
}

pub mod CDOTA_Modifier_MonkeyKing_BoundlessStrike_ShardMovement {
    pub const acceleration_z: usize = 0x12C8; // int32
    pub const min_height_above_lowest: usize = 0x12CC; // int32
    pub const min_height_above_highest: usize = 0x12D0; // int32
    pub const max_horizontal_acceleration: usize = 0x12D4; // int32
    pub const spring_channel_pct: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_MonkeyKing_FurArmyThinker {
    pub const m_fDuration: usize = 0x12C8; // float32
    pub const m_fTimeThinkerCreated: usize = 0x12CC; // GameTime_t
    pub const m_vTargetPositions: usize = 0x12D0; // CUtlVector< Vector >
    pub const m_vInitialPos: usize = 0x12E8; // Vector
    pub const first_radius: usize = 0x12F4; // int32
    pub const num_first_soldiers: usize = 0x12F8; // int32
    pub const second_radius: usize = 0x12FC; // int32
    pub const num_second_soldiers: usize = 0x1300; // int32
    pub const m_nRingFXIndex: usize = 0x1304; // ParticleIndex_t
    pub const final_radius: usize = 0x1308; // int32
}

pub mod CDOTA_Modifier_MonkeyKing_FurArmy_BonusDamage {
    pub const bonus_armor: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_MonkeyKing_FurArmy_Soldier {
    pub const m_vTargetPos: usize = 0x12C8; // Vector
    pub const m_vDirection: usize = 0x12D4; // Vector
    pub const move_speed: usize = 0x12E0; // int32
    pub const m_hThinker: usize = 0x12E4; // CHandle< C_BaseEntity >
    pub const m_bIsInPosition: usize = 0x12E8; // bool
    pub const m_nFXIndex: usize = 0x12EC; // ParticleIndex_t
    pub const m_bAutoSpawn: usize = 0x12F0; // bool
}

pub mod CDOTA_Modifier_MonkeyKing_FurArmy_SoldierInPosition {
    pub const m_vTargetPos: usize = 0x12C8; // Vector
    pub const m_hAttackTarget: usize = 0x12D4; // CHandle< C_BaseEntity >
    pub const attack_speed: usize = 0x12D8; // float32
    pub const second_radius: usize = 0x12DC; // int32
    pub const outer_attack_buffer: usize = 0x12E0; // int32
    pub const m_hThinker: usize = 0x12E4; // CHandle< C_BaseEntity >
    pub const m_bDisarmed: usize = 0x12E8; // bool
    pub const m_flNextAttackTime: usize = 0x12EC; // GameTime_t
    pub const m_nFXIndex: usize = 0x12F0; // ParticleIndex_t
    pub const m_bAutoSpawn: usize = 0x12F4; // bool
}

pub mod CDOTA_Modifier_MonkeyKing_QuadrupleTap {
    pub const counter_duration: usize = 0x12C8; // int32
    pub const required_hits: usize = 0x12CC; // int32
    pub const max_duration: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_MonkeyKing_QuadrupleTap_Bonuses {
    pub const charges: usize = 0x12C8; // int32
    pub const bonus_damage: usize = 0x12CC; // int32
    pub const lifesteal: usize = 0x12D0; // int32
    pub const m_nFXIndex: usize = 0x12D4; // ParticleIndex_t
    pub const m_nIgnoreStrikeIndex: usize = 0x12D8; // int32
    pub const m_bIsAttackAnim: usize = 0x12DC; // bool
}

pub mod CDOTA_Modifier_MonkeyKing_QuadrupleTap_Counter {
    pub const m_nFXStackIndex: usize = 0x12C8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_MonkeyKing_SpringSlow {
    pub const impact_movement_slow: usize = 0x12C8; // int32
    pub const m_nSlowPct: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_MonkeyKing_Spring_Thinker {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_MonkeyKing_Strike_Crit {
    pub const strike_crit_mult: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_MonkeyKing_Transform {
    pub const m_strDisguise: usize = 0x12C8; // CUtlString
    pub const movespeed: usize = 0x12D0; // int32
    pub const m_nDisguisedSpeed: usize = 0x12D4; // int32
    pub const reveal_radius: usize = 0x12D8; // int32
    pub const m_nDisguiseEffectIndex: usize = 0x12DC; // ParticleIndex_t
    pub const m_hCourierToClone: usize = 0x12E0; // CHandle< C_BaseEntity >
    pub const m_hCreepToClone: usize = 0x12E4; // CHandle< C_BaseEntity >
    pub const m_aryBountyRuneSpawners: usize = 0x12E8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_bIsFlyingCourier: usize = 0x1300; // bool
    pub const m_bIsRune: usize = 0x1301; // bool
    pub const m_bIsBanana: usize = 0x1302; // bool
    pub const m_bIsFakeAlly: usize = 0x1303; // bool
    pub const m_nMischiefUnitType: usize = 0x1304; // uint32
    pub const m_nMischiefHealthbarOffset: usize = 0x1308; // int32
    pub const invul_duration: usize = 0x130C; // float32
}

pub mod CDOTA_Modifier_MonkeyKing_TreeDance_Activity {
    pub const m_fAbilityRangePct: usize = 0x12C8; // float32
    pub const m_bIsSpring: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Morokai_JungleHealBeam {
    pub const beam_count: usize = 0x12C8; // int32
    pub const beam_range: usize = 0x12CC; // int32
    pub const beam_radius: usize = 0x12D0; // int32
    pub const beam_buffer: usize = 0x12D4; // int32
    pub const turn_rate: usize = 0x12D8; // int32
    pub const hp_per_second: usize = 0x12DC; // int32
    pub const m_flLastThinkTime: usize = 0x12E0; // GameTime_t
    pub const m_vecTargetedEntities: usize = 0x1300; // CUtlVector< CHandle< C_BaseEntity > >
    pub const flLastTreeDestroyed: usize = 0x1318; // float32
    pub const flTreeDestructionCooldown: usize = 0x131C; // float32
}

pub mod CDOTA_Modifier_Morphling_Adaptive_Strike_Armor {
    pub const armor_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Morphling_Morph {
    pub const tick_rate: usize = 0x12C8; // float32
    pub const bonus_attributes: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Morphling_Morph_Agi {
    pub const mana_cost: usize = 0x12C8; // int32
    pub const points_per_tick: usize = 0x12CC; // int32
    pub const morph_cooldown: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Morphling_Morph_Str {
    pub const mana_cost: usize = 0x12C8; // int32
    pub const points_per_tick: usize = 0x12CC; // int32
    pub const morph_cooldown: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Morphling_Replicate {
    pub const m_hTinyTreeWearable: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_flBaseAttackRange: usize = 0x12CC; // float32
    pub const m_flBaseMovementSpeed: usize = 0x12D0; // float32
    pub const m_iszModelName: usize = 0x12D8; // CUtlSymbolLarge
    pub const m_iszProjectileName: usize = 0x12E0; // CUtlSymbolLarge
    pub const m_iszOriginalModel: usize = 0x12E8; // CUtlSymbolLarge
    pub const m_nCopiedHeroID: usize = 0x12F0; // int32
    pub const m_fOriginalModelScale: usize = 0x12F4; // float32
    pub const m_vecOriginalItems: usize = 0x12F8; // CUtlVector< C_EconItemView* >
    pub const m_flOriginalStr: usize = 0x1340; // float32
    pub const m_flOriginalAgi: usize = 0x1344; // float32
    pub const m_flOriginalInt: usize = 0x1348; // float32
    pub const m_iOriginalAttackCapability: usize = 0x134C; // int32
    pub const m_flOriginalHealthPercentage: usize = 0x1350; // float32
    pub const m_flOriginalManaPercentage: usize = 0x1354; // float32
}

pub mod CDOTA_Modifier_Morphling_ScepterStatsDrain_Agility_Buff {
    pub const scepter_stat_steal: usize = 0x12C8; // int32
    pub const scepter_attack_speed: usize = 0x12CC; // int32
    pub const nStrengthSteal: usize = 0x12D0; // int32
    pub const nAgilitySteal: usize = 0x12D4; // int32
    pub const nIntSteal: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Morphling_ScepterStatsDrain_Agility_Debuff {
    pub const scepter_stat_steal: usize = 0x12C8; // int32
    pub const scepter_attack_speed: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Morphling_ScepterStatsDrain_All_Buff {
    pub const scepter_stat_steal: usize = 0x12C8; // int32
    pub const nStrengthSteal: usize = 0x12CC; // int32
    pub const nAgiSteal: usize = 0x12D0; // int32
    pub const nIntSteal: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Morphling_ScepterStatsDrain_All_Debuff {
    pub const scepter_stat_steal: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Morphling_ScepterStatsDrain_Intelligence_Buff {
    pub const scepter_stat_steal: usize = 0x12C8; // int32
    pub const scepter_spell_amplify: usize = 0x12CC; // int32
    pub const nStrengthSteal: usize = 0x12D0; // int32
    pub const nIntSteal: usize = 0x12D4; // int32
    pub const nAgilitySteal: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Morphling_ScepterStatsDrain_Intelligence_Debuff {
    pub const scepter_stat_steal: usize = 0x12C8; // int32
    pub const scepter_spell_amplify: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Morphling_ScepterStatsDrain_Strength_Buff {
    pub const scepter_stat_steal: usize = 0x12C8; // int32
    pub const scepter_status_resist: usize = 0x12CC; // int32
    pub const nStrengthSteal: usize = 0x12D0; // int32
    pub const nAgiSteal: usize = 0x12D4; // int32
    pub const nIntSteal: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Morphling_ScepterStatsDrain_Strength_Debuff {
    pub const scepter_stat_steal: usize = 0x12C8; // int32
    pub const scepter_status_resist: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Morty_Hop {
    pub const duration: usize = 0x12C8; // float32
    pub const height: usize = 0x12CC; // int32
    pub const damage: usize = 0x12D0; // int32
    pub const damage_radius: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_MoveSpeed_Percentage {
    pub const movespeed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_MudGolem_CloakAura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_MudGolem_CloakAura_Bonus {
    pub const bonus_magical_armor: usize = 0x12C8; // int32
    pub const bonus_magical_armor_creeps: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_MudGolem_RockDestroy {
    pub const radius: usize = 0x12C8; // int32
    pub const duration: usize = 0x12CC; // float32
    pub const shard_health: usize = 0x12D0; // int32
    pub const shard_damage: usize = 0x12D4; // int32
    pub const shard_duration: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Muerta_DeadShot_Fear {
    pub const m_vFearDir: usize = 0x12C8; // Vector
}

pub mod CDOTA_Modifier_Muerta_DeadShot_Slow {
    pub const impact_slow_percent: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Muerta_Gunslinger {
    pub const double_shot_chance: usize = 0x12C8; // float32
    pub const target_search_bonus_range: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Muerta_PartingShot_PhysicalBodyDebuff {
    pub const m_hSoulEntityClient: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const damage_reduction_percent: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Muerta_PartingShot_ProjectileReturn {
    pub const m_nProjectileHandle: usize = 0x12C8; // int32
    pub const m_hPhysicalBody: usize = 0x12CC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Muerta_PartingShot_SoulDebuff {
    pub const ethereal_damage_bonus: usize = 0x12C8; // int32
    pub const movement_slow: usize = 0x12CC; // int32
    pub const m_hPhysicalBody: usize = 0x12D0; // CHandle< C_BaseEntity >
    pub const m_nTetherFXIndex: usize = 0x12D4; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Muerta_PierceTheVeil {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const m_InFlightAttackRecords: usize = 0x12D0; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Muerta_PierceTheVeil_Buff {
    pub const modelscale: usize = 0x12C8; // int32
    pub const bonus_damage: usize = 0x12CC; // int32
    pub const spell_lifesteal: usize = 0x12D0; // int32
    pub const attack_projectile_speed_bonus: usize = 0x12D4; // int32
    pub const attack_range_bonus: usize = 0x12D8; // int32
    pub const m_iOriginalAttackCapabilities: usize = 0x12DC; // int32
    pub const m_nScreenFXIndex: usize = 0x12E0; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Muerta_Revenant_Thinker {
    pub const damage: usize = 0x12C8; // int32
    pub const hit_radius: usize = 0x12CC; // int32
    pub const dead_zone_distance: usize = 0x12D0; // int32
    pub const speed_initial: usize = 0x12D4; // float32
    pub const speed_max: usize = 0x12D8; // float32
    pub const acceleration: usize = 0x12DC; // float32
    pub const num_revenants: usize = 0x12E0; // int32
    pub const rotation_direction: usize = 0x12E4; // int32
    pub const radius: usize = 0x12E8; // int32
    pub const rotation_initial: usize = 0x12EC; // float32
    pub const show_warning: usize = 0x12F0; // int32
    pub const m_nWarningFX: usize = 0x12F4; // ParticleIndex_t
    pub const m_SpiritDefs: usize = 0x12F8; // CUtlVector< sRevenantDef >
}

pub mod CDOTA_Modifier_Muerta_TheCallingAuraSlow {
    pub const aura_movespeed_slow: usize = 0x12C8; // int32
    pub const aura_attackspeed_slow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Muertas_Gun_Fear {
    pub const m_vFearDir: usize = 0x12C8; // Vector
}

pub mod CDOTA_Modifier_Muertas_Gun_Slow {
    pub const impact_slow_percent: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Mutation_Cooldown_Reduction {
    pub const m_iCooldownReduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Mutation_Crit_Chance {
    pub const m_iChance: usize = 0x12C8; // int32
    pub const m_iDamageMultiplier: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Mutation_DeathExplosionDelayed {
    pub const m_iDamageBase: usize = 0x12C8; // int32
    pub const m_iDamagePerLevel: usize = 0x12CC; // int32
    pub const m_iRadius: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Mutation_KillstreakPower {
    pub const m_iDamageDonePct: usize = 0x12C8; // int32
    pub const m_iDamageTakenPct: usize = 0x12CC; // int32
    pub const m_iModelScale: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Mutation_StationaryDamageReduction {
    pub const m_iReductionPct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Mutation_Treecutter {
    pub const m_iRadius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Mutation_Vampire {
    pub const m_iLifeStealPct: usize = 0x12C8; // int32
    pub const m_iHealthDrain: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_NPXBuff {
    pub const m_nCurReduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_NagaSiren_Crit_Passive {
    pub const chance: usize = 0x12C8; // int32
    pub const crit: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_NagaSiren_RipTide {
    pub const armor_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_NagaSiren_SongOfTheSiren {
    pub const animation_rate: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_NagaSiren_SongOfTheSiren_Aura {
    pub const radius: usize = 0x12C8; // int32
    pub const m_nAffectedEnemies: usize = 0x12CC; // int32
    pub const m_vhAffectedHeroes: usize = 0x12D0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_NagaSiren_SongOfTheSiren_Healing_Aura {
    pub const radius: usize = 0x12C8; // int32
    pub const m_nAffectedAllies: usize = 0x12CC; // int32
    pub const m_vhAffectedHeroes: usize = 0x12D0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Naga_Siren_Reel_In {
    pub const pull_strength: usize = 0x12C8; // float32
    pub const min_pull_distance: usize = 0x12CC; // int32
    pub const m_flLastThinkTime: usize = 0x12D0; // GameTime_t
}

pub mod CDOTA_Modifier_Necrolyte_Heartstopper_Aura {
    pub const aura_radius: usize = 0x12C8; // int32
    pub const m_bStackCountChanged: usize = 0x12CC; // bool
    pub const m_fStackExpireTimes: usize = 0x12D0; // CUtlVector< GameTime_t >
}

pub mod CDOTA_Modifier_Necrolyte_Heartstopper_Aura_Counter {
    pub const mana_regen: usize = 0x12C8; // float32
    pub const health_regen: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Necrolyte_Heartstopper_Aura_Effect {
    pub const aura_damage: usize = 0x12C8; // float32
    pub const heal_reduction_pct: usize = 0x12CC; // int32
    pub const heal_regen_to_damage: usize = 0x12D0; // float32
    pub const m_nFXIndex: usize = 0x12D4; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Necrolyte_ReapersScythe {
    pub const stun_duration: usize = 0x12C8; // float32
    pub const m_bIllusion: usize = 0x12CC; // bool
    pub const m_bArcWardenClone: usize = 0x12CD; // bool
    pub const m_bSpiritBear: usize = 0x12CE; // bool
}

pub mod CDOTA_Modifier_Necrolyte_ReapersScythe_RespawnTime {
    pub const hp_per_kill: usize = 0x12C8; // float32
    pub const mana_per_kill: usize = 0x12CC; // float32
    pub const hp_regen: usize = 0x12D0; // float32
    pub const mp_regen: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Necrolyte_Sadist_Active {
    pub const heal_bonus: usize = 0x12C8; // int32
    pub const bonus_damage: usize = 0x12CC; // int32
    pub const slow_aoe: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Necrolyte_Sadist_Aura_Effect {
    pub const movement_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Necronomicon_Archer_AoE {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Necronomicon_Archer_Purge {
    pub const purge_rate: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Necronomicon_Warrior_LastWill {
    pub const explosion: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Necronomicon_Warrior_ManaBurn {
    pub const burn_amount: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Necronomicon_Warrior_Sight {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Necrophos_Death_Seeker_Ethereal {
    pub const magic_resistance_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Neutral_Sleep_AI {
    pub const m_iFXIndex: usize = 0x12C8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Neutral_Upgrade {
    pub const increase_time: usize = 0x12C8; // float32
    pub const increase_damage: usize = 0x12CC; // int32
    pub const increase_health: usize = 0x12D0; // int32
    pub const increase_armor: usize = 0x12D4; // float32
    pub const increase_aspd: usize = 0x12D8; // int32
    pub const increase_gold: usize = 0x12DC; // int32
    pub const increase_xp: usize = 0x12E0; // int32
    pub const current_level: usize = 0x12E4; // int32
    pub const max_level: usize = 0x12E8; // int32
}

pub mod CDOTA_Modifier_Nevermore_Necromastery {
    pub const necromastery_damage_per_soul: usize = 0x12C8; // int32
    pub const necromastery_max_souls: usize = 0x12CC; // int32
    pub const necromastery_max_souls_scepter: usize = 0x12D0; // int32
    pub const shard_crit_pct: usize = 0x12D4; // int32
    pub const shard_souls_per_kill: usize = 0x12D8; // int32
    pub const shard_fear_duration: usize = 0x12DC; // float32
    pub const m_iParticleSoulsIndex: usize = 0x12E0; // ParticleIndex_t
    pub const m_FXIndex: usize = 0x12E4; // ParticleIndex_t
    pub const m_InFlightAttackRecords: usize = 0x12E8; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Nevermore_Presence {
    pub const presence_armor_reduction: usize = 0x12C8; // int32
    pub const bonus_armor_per_stack: usize = 0x12CC; // int32
    pub const m_bWasHidden: usize = 0x12D0; // bool
}

pub mod CDOTA_Modifier_Nevermore_Presence_Aura {
    pub const presence_radius: usize = 0x12C8; // int32
    pub const kill_buff_duration: usize = 0x12CC; // float32
    pub const m_fLastStackChange: usize = 0x12D0; // GameTime_t
}

pub mod CDOTA_Modifier_Nevermore_Requiem_Fear {
    pub const m_vOriginal: usize = 0x12C8; // Vector
}

pub mod CDOTA_Modifier_Nevermore_Requiem_InvisBreak {
    pub const bDisableInvis: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_Nevermore_Requiem_Slow {
    pub const requiem_reduction_ms: usize = 0x12C8; // int32
    pub const requiem_reduction_mres: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_NianCharge {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_NianChargePinned {
    pub const pinned_damage_amount: usize = 0x12C8; // float32
    pub const pinned_damage_interval: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Nian_Apocalypse {
    pub const area_of_effect: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Nian_Damage_Reflection {
    pub const m_flReflectionStartTime: usize = 0x12C8; // GameTime_t
    pub const m_flReflectionAmount: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Nian_Dive {
    pub const pounce_distance: usize = 0x12C8; // int32
    pub const pounce_speed: usize = 0x12CC; // float32
    pub const pounce_acceleration: usize = 0x12D0; // float32
    pub const pounce_radius: usize = 0x12D4; // int32
    pub const pounce_damage: usize = 0x12D8; // int32
    pub const stun_radius: usize = 0x12DC; // int32
    pub const stun_duration: usize = 0x12E0; // float32
    pub const leash_duration: usize = 0x12E4; // float32
    pub const initial_delay: usize = 0x12E8; // float32
    pub const landing_delay: usize = 0x12EC; // float32
    pub const vertical_adjust: usize = 0x12F0; // float32
    pub const vertical_adjust_max_distance: usize = 0x12F4; // float32
    pub const vertical_adjust_min_distance: usize = 0x12F8; // float32
    pub const claw_damage: usize = 0x12FC; // int32
    pub const claw_damage_radius: usize = 0x1300; // int32
    pub const claw_damage_delay: usize = 0x1304; // float32
    pub const claw_damage_duration: usize = 0x1308; // float32
    pub const m_vHitEntities: usize = 0x1310; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Nian_EruptionPendingThinker {
    pub const tick_count: usize = 0x12C8; // int32
    pub const beam_radius: usize = 0x12CC; // int32
    pub const m_nTickCount: usize = 0x12D0; // int32
    pub const n_FXIndex: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Nian_EruptionThinker {
    pub const explosion_radius: usize = 0x12C8; // int32
    pub const damage_interval: usize = 0x12CC; // float32
    pub const damage: usize = 0x12D0; // int32
    pub const n_FXIndex: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Nian_Flag_Trap_Thinker {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Nian_Flag_Trapped {
    pub const bonus_spell_damage_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Nian_Frenzy {
    pub const damage: usize = 0x12C8; // int32
    pub const damage_radius: usize = 0x12CC; // int32
    pub const stun_radius: usize = 0x12D0; // int32
    pub const dive_distance: usize = 0x12D4; // float32
    pub const initial_rise_time: usize = 0x12D8; // float32
    pub const right_swipe_time: usize = 0x12DC; // float32
    pub const left_swipe_time: usize = 0x12E0; // float32
    pub const knockdown_duration: usize = 0x12E4; // float32
    pub const stun_duration: usize = 0x12E8; // float32
    pub const m_nTickCounter: usize = 0x12EC; // int32
    pub const m_hEnemies: usize = 0x12F0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Nian_GreaterBash {
    pub const chance_pct: usize = 0x12C8; // int32
    pub const damage: usize = 0x12CC; // float32
    pub const movespeed_duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Nian_GreaterBash_Speed {
    pub const bonus_movespeed_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Nian_Hurricane_Whirlpool {
    pub const pull_switch_interval: usize = 0x12C8; // float32
    pub const pull_speed: usize = 0x12CC; // int32
    pub const m_bForward: usize = 0x12D0; // bool
    pub const m_nfxIndex: usize = 0x12D4; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Nian_Knockdown {
    pub const m_flStandUpDuration: usize = 0x12C8; // float32
    pub const m_flKnockDownDuration: usize = 0x12CC; // float32
    pub const m_bDebuff: usize = 0x12D0; // bool
    pub const m_bStandingUp: usize = 0x12D1; // bool
}

pub mod CDOTA_Modifier_Nian_Leap {
    pub const pounce_distance: usize = 0x12C8; // int32
    pub const pounce_speed: usize = 0x12CC; // float32
    pub const pounce_acceleration: usize = 0x12D0; // float32
    pub const initial_delay: usize = 0x12D4; // float32
    pub const landing_delay: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Nian_Waterball {
    pub const trail_damage_per_second: usize = 0x12C8; // int32
    pub const trail_damage_radius: usize = 0x12CC; // int32
    pub const m_vFXIndices: usize = 0x12D0; // CUtlVector< int32 >
    pub const m_vLocations: usize = 0x12E8; // CUtlVector< Vector >
}

pub mod CDOTA_Modifier_Nian_WhirlpoolThinker {
    pub const pull_radius: usize = 0x12C8; // int32
    pub const n_FXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Nian_Whirlpool_Pull {
    pub const pull_radius: usize = 0x12C8; // int32
    pub const pull_speed: usize = 0x12CC; // int32
    pub const radius: usize = 0x12D0; // int32
    pub const whirlpool_damage: usize = 0x12D4; // int32
    pub const tick_rate: usize = 0x12D8; // float32
    pub const m_flDamageTick: usize = 0x12DC; // GameTime_t
}

pub mod CDOTA_Modifier_NightStalker_CripplingFear {
    pub const dps: usize = 0x12D0; // int32
    pub const tick_rate: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_NightStalker_CripplingFear_Aura {
    pub const radius: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_NightStalker_Darkness {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const status_resistance: usize = 0x12CC; // int32
    pub const hp_regen: usize = 0x12D0; // int32
    pub const m_nFXIndex: usize = 0x12D4; // ParticleIndex_t
}

pub mod CDOTA_Modifier_NightStalker_HunterInTheNight {
    pub const bonus_movement_speed_pct_night: usize = 0x12C8; // int32
    pub const bonus_attack_speed_night: usize = 0x12CC; // int32
    pub const bonus_status_resist_night: usize = 0x12D0; // int32
    pub const m_nFXIndex: usize = 0x12D4; // ParticleIndex_t
}

pub mod CDOTA_Modifier_NightStalker_Void {
    pub const movespeed_slow: usize = 0x12C8; // int32
    pub const attackspeed_slow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_NightStalker_Void_ZoneThinker {
    pub const radius_scepter: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Nullifier {
    pub const bonus_armor: usize = 0x12C8; // int32
    pub const bonus_regen: usize = 0x12CC; // int32
    pub const bonus_damage: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Nullifier_Mute {
    pub const slow_interval_duration: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Nyx_Assassin_Burrow {
    pub const health_regen_rate: usize = 0x12D8; // float32
    pub const mana_regen_rate: usize = 0x12DC; // float32
    pub const damage_reduction: usize = 0x12E0; // int32
    pub const m_vecInitialPos: usize = 0x12E4; // Vector
}

pub mod CDOTA_Modifier_Nyx_Assassin_Impale {
    pub const impale_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Nyx_Assassin_Jolt_Debuff {
    pub const resist_debuff: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Nyx_Assassin_SpikedCarapace {
    pub const stun_duration: usize = 0x12C8; // float32
    pub const m_hProcessedEnemies: usize = 0x12D0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Nyx_Assassin_Vendetta {
    pub const bonus_damage: usize = 0x12D8; // int32
    pub const movement_speed: usize = 0x12DC; // int32
    pub const shard_movement_speed_bonus: usize = 0x12E0; // int32
    pub const mana_removal_pct: usize = 0x12E4; // int32
    pub const m_bFreePathing: usize = 0x12E8; // bool
    pub const attack_animation_bonus: usize = 0x12EC; // int32
    pub const attack_range_bonus: usize = 0x12F0; // int32
}

pub mod CDOTA_Modifier_Nyx_Assassin_Vendetta_Armor_Reduction {
    pub const shard_magic_resist_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Oakheart_Healing {
    pub const heal: usize = 0x12C8; // int32
    pub const duration: usize = 0x12CC; // float32
    pub const regen: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Obsidian_Destroyer_ArcaneOrb {
    pub const mana_pool_damage_pct: usize = 0x12C8; // float32
    pub const m_InFlightAttackRecords: usize = 0x12D0; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Obsidian_Destroyer_AstralImprisonment_Prison {
    pub const allied_movement_speed_pct: usize = 0x12C8; // int32
    pub const m_bAllowMovement: usize = 0x12CC; // bool
}

pub mod CDOTA_Modifier_Obsidian_Destroyer_Equilibrium {
    pub const proc_chance: usize = 0x12C8; // int32
    pub const mana_restore: usize = 0x12CC; // int32
    pub const mana_capacity_steal: usize = 0x12D0; // float32
    pub const mana_capacity_duration: usize = 0x12D4; // float32
    pub const shard_mana_duration: usize = 0x12D8; // float32
    pub const scepter_barrier_threshold: usize = 0x12DC; // float32
    pub const scepter_barrier_duration: usize = 0x12E0; // float32
    pub const scepter_barrier_cooldown: usize = 0x12E4; // float32
    pub const scepter_max_mana_barrier_pct: usize = 0x12E8; // float32
}

pub mod CDOTA_Modifier_Obsidian_Destroyer_Equilibrium_Barrier {
    pub const barrier_amt: usize = 0x12C8; // float32
    pub const m_nDamageAbsorbed: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Obsidian_Destroyer_Equilibrium_BonusMana {
    pub const shard_bonus_mana: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Obsidian_Destroyer_Equilibrium_Buff {
    pub const mana_steal: usize = 0x12C8; // int32
    pub const m_flCurrentMana: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Obsidian_Destroyer_Equilibrium_Debuff {
    pub const mana_steal: usize = 0x12C8; // int32
    pub const m_flCurrentMana: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Obsidian_Destroyer_EssenceAura {
    pub const bonus_mana: usize = 0x12C8; // int32
    pub const radius: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Obsidian_Destroyer_EssenceAura_Effect {
    pub const radius: usize = 0x12C8; // int32
    pub const restore_chance: usize = 0x12CC; // int32
    pub const restore_amount: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Obsidian_Destroyer_Mana_Allergy {
    pub const percentage_damage: usize = 0x12C8; // int32
    pub const percentage_heal: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_OgreMagi_FrostArmor {
    pub const armor_bonus: usize = 0x12C8; // int32
    pub const slow_duration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_OgreMagi_FrostArmor_Slow {
    pub const movespeed_slow: usize = 0x12C8; // int32
    pub const attackspeed_slow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_OgreSealTotem_Active {
    pub const m_nCurrentBounce: usize = 0x12C8; // int32
    pub const leap_distance: usize = 0x12CC; // int32
    pub const leap_speed: usize = 0x12D0; // float32
    pub const leap_acceleration: usize = 0x12D4; // float32
    pub const leap_radius: usize = 0x12D8; // int32
    pub const leap_bonus_duration: usize = 0x12DC; // float32
    pub const m_bLaunched: usize = 0x12E0; // bool
}

pub mod CDOTA_Modifier_OgreSealTotem_Slow {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Ogre_Magi_Bloodlust {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
    pub const bonus_movement_speed: usize = 0x12CC; // int32
    pub const self_bonus: usize = 0x12D0; // int32
    pub const modelscale: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Ogre_Magi_DumbLuck {
    pub const mana_per_str: usize = 0x12C8; // float32
    pub const mana_regen_per_str: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Ogre_Magi_Fireblast_Multicast {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_DOTA_BaseNPC >
    pub const multicast_delay: usize = 0x12CC; // float32
    pub const m_nMultiCastCount: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Ogre_Magi_Ignite {
    pub const slow_movement_speed_pct: usize = 0x12C8; // int32
    pub const burn_damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Ogre_Magi_Ignite_Multicast {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_DOTA_BaseNPC >
    pub const multicast_delay: usize = 0x12CC; // float32
    pub const m_nMultiCastCount: usize = 0x12D0; // int32
    pub const ignite_multicast_aoe: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Ogre_Magi_Smash_Buff {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const attacks: usize = 0x12CC; // int32
    pub const damage: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Omniknight_Angelic_Flight {
    pub const movement_speed: usize = 0x12C8; // int32
    pub const hp_regen: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Omniknight_Degen_Aura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Omniknight_Degen_Aura_Effect {
    pub const speed_bonus: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Omniknight_GuardianAngel {
    pub const scepter_status_resist: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Omniknight_HammerOfPurity {
    pub const m_flNextAttackTime: usize = 0x12C8; // GameTime_t
    pub const attack_cooldown: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Omniknight_Hammer_Of_Purity_Bomb {
    pub const movement_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Omniknight_Marty {
    pub const base_strength: usize = 0x12C8; // int32
    pub const base_hpregen: usize = 0x12CC; // int32
    pub const strength_bonus: usize = 0x12D0; // int32
    pub const nDebuffAmount: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Omniknight_Pacify {
    pub const spell_resistance_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Omniknight_Repel {
    pub const status_resistance: usize = 0x12C8; // int32
    pub const hp_regen: usize = 0x12CC; // float32
    pub const bonus_str: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Oracle_FalsePromise {
    pub const m_bWaitingForInvulnerability: usize = 0x12F8; // bool
    pub const m_bDisableHealing: usize = 0x12F9; // bool
    pub const m_flHealthOnCreated: usize = 0x12FC; // float32
    pub const m_nFXIndex: usize = 0x1300; // ParticleIndex_t
    pub const m_nFXIndexB: usize = 0x1304; // ParticleIndex_t
    pub const m_flRunningDmg: usize = 0x1308; // float32
    pub const m_flRunningHealth: usize = 0x130C; // float32
    pub const bonus_armor: usize = 0x1310; // int32
}

pub mod CDOTA_Modifier_Oracle_FalsePromise_Invis {
    pub const shard_spell_amp_bonus: usize = 0x12D8; // float32
    pub const shard_bat_bonus: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Oracle_PurifyingFlames {
    pub const heal_per_second: usize = 0x12C8; // float32
    pub const tick_rate: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Oracle_RainOfDestiny {
    pub const heal_amp: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Oracle_RainOfDestiny_Aura {
    pub const damage: usize = 0x12C8; // float32
    pub const radius: usize = 0x12CC; // int32
    pub const m_nFXIndex: usize = 0x12D0; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Orb_Of_Corrosion_Debuff {
    pub const armor: usize = 0x12C8; // int32
    pub const slow_melee: usize = 0x12CC; // int32
    pub const slow_range: usize = 0x12D0; // int32
    pub const bCasterRanged: usize = 0x12D4; // bool
    pub const damage: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Orb_Of_Destruction_Debuff {
    pub const armor_reduction: usize = 0x12C8; // int32
    pub const slow_melee: usize = 0x12CC; // int32
    pub const slow_range: usize = 0x12D0; // int32
    pub const bCasterRanged: usize = 0x12D4; // bool
}

pub mod CDOTA_Modifier_Orb_Of_Revelations {
    pub const bonus_health: usize = 0x12C8; // int32
    pub const bonus_mana: usize = 0x12CC; // int32
    pub const bonus_mana_regen: usize = 0x12D0; // float32
    pub const cast_range_bonus: usize = 0x12D4; // int32
    pub const bonus_magical_armor: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Orb_Of_Revelations_Reveal {
    pub const movespeed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_OrchidMalevolence_Debuff {
    pub const silence_damage_percent: usize = 0x12C8; // float32
    pub const m_flDamageTaken: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Overflowing_Elixir_Regen {
    pub const health: usize = 0x12C8; // int32
    pub const mana: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Pangolier_Gyroshell {
    pub const mp_cost_per_second: usize = 0x12C8; // int32
    pub const tick_interval: usize = 0x12CC; // float32
    pub const forward_move_speed: usize = 0x12D0; // float32
    pub const turn_rate_boosted: usize = 0x12D4; // float32
    pub const turn_rate: usize = 0x12D8; // float32
    pub const hit_radius: usize = 0x12DC; // int32
    pub const knockback_radius: usize = 0x12E0; // int32
    pub const m_bHitFirstUpdate: usize = 0x12E4; // bool
    pub const m_flHitEndTime: usize = 0x12E8; // GameTime_t
    pub const hit_recover_time: usize = 0x12EC; // float32
    pub const m_flJumpEndTime: usize = 0x12F0; // GameTime_t
    pub const jump_recover_time: usize = 0x12F4; // float32
    pub const m_flTurnBoostProgress: usize = 0x12F8; // float32
    pub const m_flFacingTarget: usize = 0x12FC; // float32
    pub const m_bIsJumping: usize = 0x1300; // bool
    pub const m_nFXIndex: usize = 0x1304; // ParticleIndex_t
    pub const m_flTurnHistory: usize = 0x1308; // CUtlVector< float32 >
    pub const m_vecHeroesHitLastRicochet: usize = 0x1320; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_vecHeroesCredited: usize = 0x1338; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_vecHeroesHitCurrentRicochet: usize = 0x1350; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Pangolier_HeartPiercer {
    pub const debuff_delay: usize = 0x12C8; // float32
    pub const chance_pct: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Pangolier_HeartPiercer_Debuff {
    pub const slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Pangolier_HeartPiercer_Delay {
    pub const duration: usize = 0x12C8; // float32
    pub const debuff_delay: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Pangolier_LuckyShot {
    pub const chance_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Pangolier_LuckyShot_Disarm {
    pub const armor: usize = 0x12C8; // int32
    pub const attack_slow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Pangolier_LuckyShot_Silence {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Pangolier_Rollup {
    pub const mp_cost_per_second: usize = 0x12C8; // int32
    pub const tick_interval: usize = 0x12CC; // float32
    pub const forward_move_speed: usize = 0x12D0; // float32
    pub const turn_rate_boosted: usize = 0x12D4; // float32
    pub const turn_rate: usize = 0x12D8; // float32
    pub const hit_radius: usize = 0x12DC; // int32
    pub const knockback_radius: usize = 0x12E0; // int32
    pub const m_bHitFirstUpdate: usize = 0x12E4; // bool
    pub const m_flHitEndTime: usize = 0x12E8; // GameTime_t
    pub const hit_recover_time: usize = 0x12EC; // float32
    pub const m_flJumpEndTime: usize = 0x12F0; // GameTime_t
    pub const jump_recover_time: usize = 0x12F4; // float32
    pub const m_flTurnBoostProgress: usize = 0x12F8; // float32
    pub const m_flFacingTarget: usize = 0x12FC; // float32
    pub const m_flLastHeroAttackTime: usize = 0x1300; // GameTime_t
    pub const m_bIsJumping: usize = 0x1304; // bool
    pub const m_nFXIndex: usize = 0x1308; // ParticleIndex_t
    pub const m_flGyroshellDurationRemaining: usize = 0x130C; // float32
    pub const m_flTurnHistory: usize = 0x1310; // CUtlVector< float32 >
    pub const m_vecHeroesHitLastRicochet: usize = 0x1328; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_vecHeroesCredited: usize = 0x1340; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_vecHeroesHitCurrentRicochet: usize = 0x1358; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Pangolier_ShieldCrash_Buff {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const m_nAbsorbRemaining: usize = 0x12CC; // int32
    pub const hero_shield: usize = 0x12D0; // int32
    pub const base_shield: usize = 0x12D4; // int32
    pub const accummulated_value: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Pangolier_ShieldCrash_Slow {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Pangolier_Swashbuckle {
    pub const m_nStrikesLeft: usize = 0x12C8; // int32
    pub const attack_interval: usize = 0x12CC; // float32
    pub const m_vSpawnPos: usize = 0x12D0; // Vector
    pub const m_vStrikeDir: usize = 0x12DC; // Vector
    pub const m_vDashDir: usize = 0x12E8; // Vector
}

pub mod CDOTA_Modifier_Pangolier_Swashbuckle_Attack {
    pub const damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Passive_Mango_Tree {
    pub const m_hTree: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_nMangoSeconds: usize = 0x12CC; // int32
    pub const m_MangoTimer: usize = 0x12D0; // CountdownTimer
    pub const m_nRespawnSeconds: usize = 0x12E8; // int32
    pub const m_RespawnTimer: usize = 0x12F0; // CountdownTimer
    pub const m_nMangosAvailable: usize = 0x1308; // int32
    pub const m_nChannelCount: usize = 0x130C; // int32
}

pub mod CDOTA_Modifier_Penta_Edged_Sword_Maim {
    pub const maim_slow_movement: usize = 0x12C8; // int32
    pub const maim_slow_attack: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_PersistentInvisibility {
    pub const fade_delay: usize = 0x12C8; // float32
    pub const m_flFadeTime: usize = 0x12CC; // float32
    pub const m_flLastActionTime: usize = 0x12D0; // GameTime_t
}

pub mod CDOTA_Modifier_PhantomAssassin_Blur {
    pub const bonus_evasion: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_PhantomAssassin_BlurActive {
    pub const radius: usize = 0x12D8; // int32
    pub const fade_duration: usize = 0x12DC; // float32
    pub const scepter_fade_duration: usize = 0x12E0; // float32
    pub const m_bDestroyNext: usize = 0x12E4; // bool
}

pub mod CDOTA_Modifier_PhantomAssassin_CoupdeGrace {
    pub const crit_bonus: usize = 0x12C8; // int32
    pub const crit_chance: usize = 0x12CC; // int32
    pub const dagger_crit_chance: usize = 0x12D0; // int32
    pub const crit_active: usize = 0x12D4; // bool
    pub const duration: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_PhantomAssassin_Fan_Of_Knives_Thinker {
    pub const m_fCurRadius: usize = 0x12C8; // float32
    pub const m_fLastThink: usize = 0x12CC; // GameTime_t
    pub const m_entitiesHit: usize = 0x12D0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_ViewerTimer: usize = 0x12E8; // CountdownTimer
    pub const projectile_speed: usize = 0x1300; // int32
    pub const radius: usize = 0x1304; // int32
    pub const duration: usize = 0x1308; // float32
    pub const pct_health_damage_initial: usize = 0x130C; // float32
    pub const max_damage_initial: usize = 0x1310; // float32
}

pub mod CDOTA_Modifier_PhantomAssassin_Gravestone {
    pub const m_nFXIndex: usize = 0x12C8; // int32
    pub const m_bContractGravestoneComplete: usize = 0x12CC; // bool
}

pub mod CDOTA_Modifier_PhantomAssassin_Gravestone_Thinker {
    pub const m_nEpitaph: usize = 0x12C8; // int32
    pub const m_nVictimPlayerID: usize = 0x12CC; // PlayerID_t
    pub const m_nVictimHeroID: usize = 0x12D0; // int32
    pub const m_nCritDmg: usize = 0x12D4; // int32
    pub const m_nContractComplete: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_PhantomAssassin_PhantomStrike {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
    pub const lifesteal_pct: usize = 0x12CC; // int32
    pub const creep_lifesteal_penalty: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_PhantomAssassin_StiflingDagger {
    pub const move_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_PhantomAssassin_StiflingDagger_Caster {
    pub const base_damage: usize = 0x12C8; // int32
    pub const attack_factor: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_PhantomLancer_Doppelwalk_Illusion {
    pub const flDamageTaken: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_PhantomLancer_Dopplewalk_Phase {
    pub const vTargetPosition: usize = 0x12C8; // Vector
}

pub mod CDOTA_Modifier_PhantomLancer_Juxtapose {
    pub const proc_chance_pct: usize = 0x12C8; // int32
    pub const illusion_proc_chance_pct: usize = 0x12CC; // int32
    pub const max_illusions: usize = 0x12D0; // int32
    pub const illusion_duration: usize = 0x12D4; // float32
    pub const illusion_damage_out_pct: usize = 0x12D8; // int32
    pub const illusion_damage_in_pct: usize = 0x12DC; // int32
    pub const shard_bonus_illusions: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_PhantomLancer_JuxtaposeIllusion {
    pub const m_bstrongIllusion: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_PhantomLancer_Juxtapose_Cache {
    pub const m_nControllingPlayer: usize = 0x12C8; // PlayerID_t
}

pub mod CDOTA_Modifier_PhantomLancer_Juxtapose_Invisibility {
    pub const invis_movespeed: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_PhantomLancer_Juxtapose_Thinker {
    pub const max_illusions: usize = 0x12C8; // int32
    pub const scepter_bonus_illusions: usize = 0x12CC; // int32
    pub const illusion_duration: usize = 0x12D0; // float32
    pub const illusion_damage_out_pct: usize = 0x12D4; // int32
    pub const illusion_damage_in_pct: usize = 0x12D8; // int32
    pub const scepter_bonus_duration: usize = 0x12DC; // float32
    pub const m_iIllusionsSoFar: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_PhantomLancer_PhantomEdge {
    pub const min_distance: usize = 0x12C8; // int32
    pub const max_distance: usize = 0x12CC; // int32
    pub const m_hTarget: usize = 0x12D0; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_PhantomLancer_PhantomEdge_Agility {
    pub const bonus_agility: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_PhantomLancer_PhantomEdge_Boost {
    pub const bonus_speed: usize = 0x12C8; // int32
    pub const agility_duration: usize = 0x12CC; // float32
    pub const m_bGiveAgility: usize = 0x12D0; // bool
    pub const m_hTarget: usize = 0x12D4; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_PhantomLancer_SpiritLance {
    pub const movement_speed_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Phoenix_FireSpiritBurn {
    pub const tick_interval: usize = 0x12C8; // float32
    pub const attackspeed_slow: usize = 0x12CC; // int32
    pub const damage_per_second: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Phoenix_FireSpiritCount {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Phoenix_IcarusDive {
    pub const m_vSource: usize = 0x12C8; // Vector
    pub const m_vTarget: usize = 0x12D4; // Vector
    pub const m_vDirection: usize = 0x12E0; // Vector
    pub const m_angDirection: usize = 0x12EC; // QAngle
    pub const m_flCurrentTime: usize = 0x12F8; // float32
    pub const dash_length: usize = 0x12FC; // int32
    pub const dash_width: usize = 0x1300; // int32
    pub const hit_radius: usize = 0x1304; // int32
    pub const burn_duration: usize = 0x1308; // float32
    pub const dive_duration: usize = 0x130C; // float32
    pub const impact_damage: usize = 0x1310; // float32
    pub const m_vecHitEntities: usize = 0x1318; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Phoenix_IcarusDiveBurn {
    pub const burn_tick_interval: usize = 0x12C8; // float32
    pub const damage_per_second: usize = 0x12CC; // int32
    pub const slow_movement_speed_pct: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Phoenix_Sun {
    pub const aura_radius: usize = 0x12C8; // int32
    pub const stun_duration: usize = 0x12CC; // float32
    pub const max_hero_attacks: usize = 0x12D0; // int32
    pub const max_hero_attacks_scepter: usize = 0x12D4; // int32
    pub const max_hero_attacks_required: usize = 0x12D8; // int32
    pub const m_hSecondaryTarget: usize = 0x12DC; // CHandle< C_BaseEntity >
    pub const m_iAttackCount: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_Phoenix_SunRay {
    pub const hp_cost_perc_per_second: usize = 0x12C8; // int32
    pub const beam_range: usize = 0x12CC; // int32
    pub const base_damage: usize = 0x12D0; // int32
    pub const base_heal: usize = 0x12D4; // int32
    pub const tick_interval: usize = 0x12D8; // float32
    pub const forward_move_speed: usize = 0x12DC; // float32
    pub const turn_rate_initial: usize = 0x12E0; // float32
    pub const turn_rate: usize = 0x12E4; // float32
    pub const radius: usize = 0x12E8; // int32
    pub const m_flCurrentTime: usize = 0x12EC; // float32
    pub const m_flAccumulatedSelfDamage: usize = 0x12F0; // float32
    pub const m_bMovingForward: usize = 0x12F4; // bool
    pub const m_bTurningFast: usize = 0x12F5; // bool
    pub const m_flFacingTarget: usize = 0x12F8; // float32
    pub const hp_perc_damage: usize = 0x12FC; // float32
    pub const hp_perc_heal: usize = 0x1300; // float32
    pub const blind_duration: usize = 0x1304; // float32
    pub const blind_per_second: usize = 0x1308; // int32
    pub const m_nBeamFXIndex: usize = 0x130C; // ParticleIndex_t
    pub const m_hVisionThinkers: usize = 0x1310; // CHandle< C_BaseEntity >[8]
    pub const m_hBeamEnd: usize = 0x1330; // CHandle< C_BaseEntity >
    pub const m_bCreatedVisionThinkers: usize = 0x1334; // bool
    pub const m_hBeamEndSound: usize = 0x1338; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Phoenix_SunRay_Luminance {
    pub const pct_of_heal_as_attack_speed: usize = 0x12C8; // float32
    pub const m_fAttackSpeedBonus: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Phoenix_Sun_Debuff {
    pub const damage_per_sec: usize = 0x12C8; // int32
    pub const tick_interval: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Phoenix_Sun_Ray_Slow {
    pub const shard_move_slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_PineCone_AcornShot_BonusDamage {
    pub const acorn_shot_damage: usize = 0x12C8; // int32
    pub const base_damage_pct: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_PineCone_AcornShot_Slow {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_PineCone_ShieldBash_Crit {
    pub const crit_mult: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_PineCone_ShieldBash_Slow {
    pub const knockback_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Plague_Wards_Bonus {
    pub const ward_hp: usize = 0x12C8; // int32
    pub const ward_damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Plague_Wards_Bonus_Range {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Plus_HighFiveRequested {
    pub const acknowledge_range: usize = 0x12C8; // int32
    pub const think_interval: usize = 0x12CC; // float32
    pub const acknowledged_cooldown: usize = 0x12D0; // float32
    pub const m_bAcknowledged: usize = 0x12D4; // bool
    pub const m_bFirstThink: usize = 0x12D5; // bool
    pub const high_five_level: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Pogo_Stick_Active {
    pub const leap_distance: usize = 0x12C8; // int32
    pub const leap_speed: usize = 0x12CC; // float32
    pub const leap_acceleration: usize = 0x12D0; // float32
    pub const leap_radius: usize = 0x12D4; // int32
    pub const leap_bonus_duration: usize = 0x12D8; // float32
    pub const m_bLaunched: usize = 0x12DC; // bool
}

pub mod CDOTA_Modifier_PoisonNova_Thinker {
    pub const m_fCurRadius: usize = 0x12C8; // float32
    pub const m_fLastThink: usize = 0x12CC; // GameTime_t
    pub const m_entitiesHit: usize = 0x12D0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const speed: usize = 0x12E8; // int32
    pub const radius: usize = 0x12EC; // int32
    pub const start_radius: usize = 0x12F0; // int32
    pub const duration: usize = 0x12F4; // float32
}

pub mod CDOTA_Modifier_PolarFurbolgUrsaWarrior_ThunderClap {
    pub const movespeed_slow: usize = 0x12C8; // int32
    pub const attackspeed_slow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_PrimalBeast_Onslaught_Movement {
    pub const tick_interval: usize = 0x12C8; // float32
    pub const charge_speed: usize = 0x12CC; // float32
    pub const movement_turn_rate: usize = 0x12D0; // float32
    pub const knockback_radius: usize = 0x12D4; // int32
    pub const knockback_distance: usize = 0x12D8; // float32
    pub const knockback_damage: usize = 0x12DC; // int32
    pub const m_nEnemyHeroesHit: usize = 0x12E0; // int32
    pub const m_flFacingTarget: usize = 0x12E4; // float32
    pub const m_nFXIndex: usize = 0x12E8; // ParticleIndex_t
    pub const m_hHitEntities: usize = 0x12F0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_PrimalBeast_Onslaught_Windup {
    pub const max_distance: usize = 0x12C8; // int32
    pub const max_charge_time: usize = 0x12CC; // float32
    pub const turn_rate: usize = 0x12D0; // float32
    pub const base_power: usize = 0x12D4; // float32
    pub const charge_speed: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_PrimalBeast_Pulverize {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const m_flOffset: usize = 0x12CC; // float32
    pub const m_vStartAngles: usize = 0x12D0; // QAngle
    pub const m_vStartLocation: usize = 0x12DC; // Vector
    pub const animation_rate: usize = 0x12E8; // float32
    pub const m_bInterrupted: usize = 0x12EC; // bool
    pub const m_bHasBeenDestroyed: usize = 0x12ED; // bool
}

pub mod CDOTA_Modifier_PrimalBeast_Pulverize_Self {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_DOTA_BaseNPC >
    pub const m_vPulverizeCenter: usize = 0x12CC; // Vector
    pub const m_vCasterStartPos: usize = 0x12D8; // Vector
    pub const splash_radius: usize = 0x12E4; // int32
    pub const interval: usize = 0x12E8; // float32
    pub const ministun: usize = 0x12EC; // float32
    pub const damage: usize = 0x12F0; // int32
    pub const bonus_damage_per_hit: usize = 0x12F4; // int32
    pub const m_nHitCount: usize = 0x12F8; // int32
}

pub mod CDOTA_Modifier_PrimalBeast_Uproar {
    pub const stack_limit: usize = 0x12C8; // int32
    pub const damage_limit: usize = 0x12CC; // int32
    pub const stack_duration: usize = 0x12D0; // float32
    pub const damage_min: usize = 0x12D4; // float32
    pub const damage_max: usize = 0x12D8; // float32
    pub const bonus_damage_per_stack: usize = 0x12DC; // int32
    pub const m_nFxIndexA: usize = 0x12E0; // ParticleIndex_t
    pub const iCur_stack: usize = 0x12E4; // int32
    pub const bonus_damage: usize = 0x12E8; // int32
    pub const slow_duration: usize = 0x12EC; // float32
    pub const m_fTotalDamage: usize = 0x1308; // float32
    pub const m_flLastStackTime: usize = 0x130C; // GameTime_t
}

pub mod CDOTA_Modifier_PrimalBeast_Uproar_Roared_Self {
    pub const roared_bonus_attack_speed: usize = 0x12C8; // int32
    pub const roared_bonus_armor: usize = 0x12CC; // int32
    pub const m_nBuffStackCount: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_PrimalBeast_Uproar_Slow {
    pub const move_slow_per_stack: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Primal_Beast_Uproar_Projectile_Thinker {
    pub const projectiles_per_stack: usize = 0x12C8; // int32
    pub const projectile_speed: usize = 0x12CC; // int32
    pub const projectile_distance: usize = 0x12D0; // int32
    pub const projectile_width: usize = 0x12D4; // int32
    pub const splinter_angle: usize = 0x12D8; // int32
    pub const split_delay: usize = 0x12DC; // float32
    pub const projectile_waves: usize = 0x12E0; // int32
    pub const m_nCurrentWaveCount: usize = 0x12E4; // int32
    pub const tectonic_shift_projectiles: usize = 0x12E8; // int32
}

pub mod CDOTA_Modifier_Primalbeast_Trample {
    pub const effect_radius: usize = 0x12C8; // int32
    pub const vLastPos: usize = 0x12CC; // Vector
    pub const flCurrentDistance: usize = 0x12D8; // float32
    pub const step_distance: usize = 0x12DC; // int32
    pub const bonus_magic_resistance: usize = 0x12E0; // int32
    pub const m_bIsUnslowable: usize = 0x12E4; // bool
}

pub mod CDOTA_Modifier_Psychic_Headband {
    pub const push_length: usize = 0x12C8; // int32
    pub const push_duration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Puck_Coil_Break_Stun {
    pub const m_vCastLocation: usize = 0x12C8; // Vector
    pub const coil_rapid_fire_z_offset: usize = 0x12D4; // int32
    pub const coil_rapid_fire_rate: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Puck_Coiled {
    pub const m_vCastLocation: usize = 0x12C8; // Vector
    pub const coil_rapid_fire_z_offset: usize = 0x12D4; // int32
    pub const coil_rapid_fire_rate: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Puck_DreamCoil_Thinker {
    pub const m_hLinkedEntities: usize = 0x12C8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const coil_radius: usize = 0x12E0; // int32
    pub const coil_break_radius: usize = 0x12E4; // int32
    pub const coil_initial_damage: usize = 0x12E8; // int32
    pub const coil_stun_duration: usize = 0x12EC; // float32
    pub const coil_duration: usize = 0x12F0; // float32
    pub const coil_break_damage: usize = 0x12F4; // int32
    pub const m_FXIndex: usize = 0x12F8; // CUtlVector< ParticleIndex_t >
}

pub mod CDOTA_Modifier_Puck_PhaseShift_AttackBonus {
    pub const shard_bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_PudgeMiniboss_ArmorCorruption {
    pub const armor_reduction_per_stack: usize = 0x12C8; // float32
    pub const stack_duration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_PudgeMiniboss_ArmorCorruptionDebuff {
    pub const armor_reduction_per_stack: usize = 0x12C8; // float32
    pub const stack_duration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_PudgeMiniboss_HatefulStrike {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const maxhp_percent_damage: usize = 0x12CC; // int32
    pub const every_n_attacks: usize = 0x12D0; // int32
    pub const crit_active: usize = 0x12D4; // bool
}

pub mod CDOTA_Modifier_Pudge_Dismember {
    pub const dismember_damage: usize = 0x12C8; // int32
    pub const strength_damage: usize = 0x12CC; // float32
    pub const m_nTicks: usize = 0x12D0; // int32
    pub const m_nMaxTicks: usize = 0x12D4; // int32
    pub const m_flTickRate: usize = 0x12D8; // float32
    pub const animation_rate: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Pudge_Dismember_Pull {
    pub const m_vDestination: usize = 0x12C8; // Vector
    pub const pull_units_per_second: usize = 0x12D4; // int32
    pub const pull_distance_limit: usize = 0x12D8; // float32
    pub const animation_rate: usize = 0x12E4; // float32
}

pub mod CDOTA_Modifier_Pudge_FleshHeap {
    pub const magic_resistance: usize = 0x12C8; // float32
    pub const damage_block: usize = 0x12CC; // float32
    pub const flesh_heap_strength_buff_amount: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Pudge_FleshHeap_Block {
    pub const damage_block: usize = 0x12C8; // float32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Pudge_Rot {
    pub const rot_damage: usize = 0x12C8; // int32
    pub const rot_slow: usize = 0x12CC; // int32
    pub const scepter_rot_regen_reduction_pct: usize = 0x12D0; // int32
    pub const m_flLastRotTime: usize = 0x12DC; // GameTime_t
    pub const m_bQualifiesAsPotentionalDeny: usize = 0x12E0; // bool
}

pub mod CDOTA_Modifier_Pudge_Swallow {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Pudge_Swallow_Hide {
    pub const order_lock_duration: usize = 0x12C8; // float32
    pub const m_fOrderLockDone: usize = 0x12CC; // GameTime_t
    pub const m_nFXIndex: usize = 0x12D0; // int32
    pub const shard_regen_pct: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Pugna_Decrepify {
    pub const bonus_spell_damage_pct: usize = 0x12C8; // int32
    pub const bonus_heal_amp_pct_allies: usize = 0x12CC; // int32
    pub const bonus_movement_speed: usize = 0x12D0; // int32
    pub const bonus_movement_speed_allies: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Pugna_LifeDrain {
    pub const health_drain: usize = 0x12C8; // int32
    pub const ally_healing: usize = 0x12CC; // int32
    pub const tick_rate: usize = 0x12D0; // float32
    pub const m_bDoRangeCheck: usize = 0x12D4; // bool
    pub const m_nFXIndex: usize = 0x12D8; // ParticleIndex_t
    pub const m_flElapsedTime: usize = 0x12DC; // GameTime_t
    pub const m_bPrimary: usize = 0x12E0; // bool
    pub const m_bShard: usize = 0x12E1; // bool
    pub const m_bFromWard: usize = 0x12E2; // bool
    pub const m_hWard: usize = 0x12E4; // CHandle< C_BaseEntity >
    pub const spell_amp_drain_duration: usize = 0x12E8; // float32
    pub const max_spell_amp_drain_pct: usize = 0x12EC; // int32
    pub const spell_amp_drain_rate: usize = 0x12F0; // int32
    pub const spell_amp_drain_rate_ward: usize = 0x12F4; // int32
    pub const spell_amp_drain_max: usize = 0x12F8; // int32
    pub const health_to_mana_rate: usize = 0x12FC; // int32
}

pub mod CDOTA_Modifier_Pugna_LifeDrain_SpellAmp {
    pub const m_bIsFriendly: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_Pugna_NetherWard {
    pub const radius: usize = 0x12C8; // int32
    pub const mana_multiplier: usize = 0x12CC; // float32
    pub const mana_drained_per_attack: usize = 0x12D0; // float32
    pub const attacks_to_destroy: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Pugna_NetherWard_Aura {
    pub const mana_regen: usize = 0x12C8; // float32
    pub const spell_damage_reduction: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_QueenOfPain_Arcana {
    pub const m_bIsMeleeAttack: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_QueenOfPain_ScreamOfPain_Fear {
    pub const m_vOriginal: usize = 0x12C8; // Vector
}

pub mod CDOTA_Modifier_QueenOfPain_ShadowStrike {
    pub const duration_damage: usize = 0x12C8; // int32
    pub const duration_heal: usize = 0x12CC; // int32
    pub const movement_slow: usize = 0x12D0; // int32
    pub const m_SlowInterval: usize = 0x12D8; // CountdownTimer
    pub const m_flSlowStep: usize = 0x12F0; // float32
    pub const m_flSlowStepStep: usize = 0x12F4; // float32
    pub const attack_buff_duration: usize = 0x12F8; // float32
}

pub mod CDOTA_Modifier_QueenOfPain_SonicWave_Damage {
    pub const m_flDamagePerTick: usize = 0x12C8; // float32
    pub const m_nCurrentTick: usize = 0x12CC; // int32
    pub const m_nTotalTicks: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Radar_Thinker {
    pub const radius: usize = 0x12C8; // int32
    pub const m_bEverFound: usize = 0x12CC; // bool
    pub const m_bBonus: usize = 0x12CD; // bool
}

pub mod CDOTA_Modifier_Rattletrap_Cog {
    pub const m_bEnabled: usize = 0x12C8; // bool
    pub const trigger_distance: usize = 0x12CC; // int32
    pub const push_length: usize = 0x12D0; // int32
    pub const push_duration: usize = 0x12D4; // float32
    pub const cogs_radius: usize = 0x12D8; // int32
    pub const attacks_to_destroy: usize = 0x12DC; // int32
    pub const m_iCogIndex: usize = 0x12E0; // int32
    pub const m_nFXIndex: usize = 0x12E4; // ParticleIndex_t
    pub const vCenter: usize = 0x12E8; // Vector
}

pub mod CDOTA_Modifier_Rattletrap_CogPush {
    pub const m_flStartTime: usize = 0x12C8; // GameTime_t
    pub const push_length: usize = 0x12CC; // int32
    pub const push_duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Rattletrap_Cog_Barrier {
    pub const magic_barrier: usize = 0x12C8; // int32
    pub const m_nAbsorbRemaining: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Rattletrap_Cog_Leash {
    pub const leash: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Rattletrap_Cog_Thinker {
    pub const cogs_radius: usize = 0x12C8; // int32
    pub const extra_pull_buffer: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Rattletrap_Cog_Thinker_Leash {
    pub const cogs_radius: usize = 0x12C8; // int32
    pub const extra_pull_buffer: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Rattletrap_Cog_Thinker_Self_Bonuses {
    pub const cogs_radius: usize = 0x12C8; // int32
    pub const extra_pull_buffer: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Rattletrap_Cog_Thinker_Talent {
    pub const cogs_radius: usize = 0x12C8; // int32
    pub const extra_pull_buffer: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Rattletrap_Hookshot {
    pub const speed: usize = 0x12D0; // int32
    pub const stun_radius: usize = 0x12D4; // float32
    pub const damage: usize = 0x12D8; // float32
    pub const duration: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Rattletrap_JetPack {
    pub const m_nMovementSpeed: usize = 0x12C8; // int32
    pub const bonus_speed: usize = 0x12CC; // int32
    pub const turn_rate: usize = 0x12D0; // float32
    pub const m_flStartTime: usize = 0x12D4; // GameTime_t
    pub const tick_interval: usize = 0x12D8; // float32
    pub const m_flFacingTarget: usize = 0x12DC; // float32
    pub const m_nFXIndex: usize = 0x12E0; // ParticleIndex_t
    pub const m_flTurnHistory: usize = 0x12E8; // CUtlVector< float32 >
    pub const m_vecHeroesHitLastRicochet: usize = 0x1300; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_vecHeroesCredited: usize = 0x1318; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_vecHeroesHitCurrentRicochet: usize = 0x1330; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Rattletrap_Overclocking {
    pub const m_nOverclockFX: usize = 0x12C8; // ParticleIndex_t
    pub const bonus_movement_speed: usize = 0x12CC; // int32
    pub const bonus_attack_speed: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Rattletrap_RocketFlare {
    pub const projectile_vision_radius: usize = 0x12D0; // int32
    pub const projectile_vision_duration: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Rattletrap_RocketFlare_Overclock {
    pub const rocket_flare_interval: usize = 0x12C8; // float32
    pub const rocket_flare_offset_pct: usize = 0x12CC; // int32
    pub const rocket_flare_rockets: usize = 0x12D0; // int32
    pub const m_nRocketsFired: usize = 0x12D4; // int32
    pub const radius: usize = 0x12D8; // int32
    pub const m_vOriginalTarget: usize = 0x12DC; // Vector
}

pub mod CDOTA_Modifier_Rattletrap_RocketFlare_Slow {
    pub const slow_pct: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Razor_Arcana {
    pub const m_bIsMeleeAttack: usize = 0x12C8; // bool
    pub const m_bTauntQueued: usize = 0x12C9; // bool
}

pub mod CDOTA_Modifier_Razor_Arcana_EmpoweredState {
    pub const m_nEmpoweredBodyFX: usize = 0x12C8; // ParticleIndex_t
    pub const m_nEmpoweredWeaponFX: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Razor_Arcana_Kill_Effect {
    pub const m_nCasterFXIndex: usize = 0x12C8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Razor_EyeOfTheStorm {
    pub const radius: usize = 0x12C8; // int32
    pub const armor_reduction: usize = 0x12CC; // int32
    pub const damage: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Razor_EyeOfTheStorm_Passive {
    pub const passive_area_damage: usize = 0x12C8; // int32
    pub const passive_area_interval: usize = 0x12CC; // float32
    pub const radius: usize = 0x12D0; // int32
    pub const m_bHitNext: usize = 0x12D4; // bool
}

pub mod CDOTA_Modifier_Razor_PlasmaField_Slow {
    pub const slow_amount: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Razor_PlasmaField_Thinker {
    pub const m_fLastRadius: usize = 0x12C8; // float32
    pub const m_fCurRadius: usize = 0x12CC; // float32
    pub const m_fLastThink: usize = 0x12D0; // GameTime_t
    pub const m_bContracting: usize = 0x12D4; // bool
    pub const m_ViewerTimer: usize = 0x12D8; // CountdownTimer
    pub const m_nFXIndex: usize = 0x12F0; // ParticleIndex_t
    pub const m_EntitiesHit: usize = 0x12F8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const speed: usize = 0x1310; // int32
    pub const radius: usize = 0x1314; // int32
    pub const damage_min: usize = 0x1318; // float32
    pub const damage_max: usize = 0x131C; // float32
    pub const slow_min: usize = 0x1320; // int32
    pub const slow_max: usize = 0x1324; // int32
    pub const slow_duration: usize = 0x1328; // float32
    pub const m_bHasCreatedFx: usize = 0x132C; // bool
}

pub mod CDOTA_Modifier_Razor_StaticLink {
    pub const flSmoothness: usize = 0x12C8; // float32
    pub const drain_duration: usize = 0x12CC; // float32
    pub const drain_rate: usize = 0x12D0; // int32
    pub const drain_range: usize = 0x12D4; // int32
    pub const drain_range_buffer: usize = 0x12D8; // int32
    pub const m_hTarget: usize = 0x12DC; // CHandle< C_BaseEntity >
    pub const m_iTotalDrainAmount: usize = 0x12E0; // int32
    pub const pull_speed: usize = 0x12E4; // int32
    pub const min_pull_range: usize = 0x12E8; // int32
    pub const m_iLinkIndex: usize = 0x12EC; // ParticleIndex_t
    pub const m_pBuffPositive: usize = 0x12F0; // CDOTA_Buff*
    pub const m_pNegative: usize = 0x12F8; // CDOTA_Buff*
    pub const m_flLastThinkTime: usize = 0x1300; // GameTime_t
}

pub mod CDOTA_Modifier_Razor_StaticLink_Buff {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const drain_rate: usize = 0x12CC; // int32
    pub const drain_length: usize = 0x12D0; // float32
    pub const attack_speed_factor: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Razor_StaticLink_Debuff {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const attack_speed_factor: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Razor_UnstableCurrent_Aura {
    pub const self_movement_speed_pct: usize = 0x12C8; // int32
    pub const strike_pct_chance: usize = 0x12CC; // int32
    pub const strike_damage: usize = 0x12D0; // float32
    pub const strike_move_slow_pct: usize = 0x12D4; // int32
    pub const strike_search_radius: usize = 0x12D8; // int32
    pub const strike_target_count: usize = 0x12DC; // int32
    pub const strike_slow_duration: usize = 0x12E0; // float32
    pub const strike_internal_cd: usize = 0x12E4; // float32
}

pub mod CDOTA_Modifier_Razor_UnstableCurrent_Delayed_Damage {
    pub const strike_damage: usize = 0x12C8; // float32
    pub const strike_slow_duration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Razor_UnstableCurrent_Slow {
    pub const strike_move_slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Repair_Kit {
    pub const health_regen: usize = 0x12C8; // float32
    pub const armor_bonus: usize = 0x12CC; // int32
    pub const multishot_count: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Riki_Backstab {
    pub const fade_delay: usize = 0x12D8; // float32
    pub const backstab_angle: usize = 0x12DC; // int32
    pub const cleave: usize = 0x12E0; // int32
    pub const damage_multiplier: usize = 0x12E4; // float32
    pub const bonus_xp_kill: usize = 0x12E8; // float32
    pub const bonus_xp_assist: usize = 0x12EC; // float32
    pub const m_bBackstab: usize = 0x12F0; // bool
}

pub mod CDOTA_Modifier_Riki_Permanent_Invisibility {
    pub const movement_speed: usize = 0x12D8; // int32
    pub const fade_delay: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Riki_Poison_Dart {
    pub const wake_damage_limit: usize = 0x12C8; // int32
    pub const m_flDamageTaken: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Riki_Poison_Dart_Debuff {
    pub const movement_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Riki_SmokeScreen {
    pub const miss_rate: usize = 0x12C8; // int32
    pub const dodge_chance: usize = 0x12CC; // int32
    pub const block_targeting: usize = 0x12D0; // int32
    pub const armor_reduction: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Riki_SmokeScreenThinker {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Riki_TricksOfTheTrade_Phase {
    pub const m_nFxIndex: usize = 0x12C8; // ParticleIndex_t
    pub const m_hTarget: usize = 0x12CC; // CHandle< C_BaseEntity >
    pub const m_Timer: usize = 0x12D0; // CountdownTimer
    pub const m_hPreviousTarget: usize = 0x12E8; // CHandle< C_BaseEntity >
    pub const m_nSucceessiveHits: usize = 0x12EC; // int32
    pub const damage_pct: usize = 0x12F0; // int32
    pub const agility_pct: usize = 0x12F4; // int32
}

pub mod CDOTA_Modifier_Roshan_Bash {
    pub const bash_chance: usize = 0x12C8; // int32
    pub const bonus_damage: usize = 0x12CC; // int32
    pub const stun_duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Roshan_DevotionAura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Roshan_Moving {
    pub const movement_speed: usize = 0x12C8; // int32
    pub const m_hTarget: usize = 0x12CC; // CHandle< C_BaseEntity >
    pub const m_bFlipped: usize = 0x12D0; // bool
}

pub mod CDOTA_Modifier_Roshan_Slam {
    pub const slow_amount: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Royal_Jelly {
    pub const health_regen: usize = 0x12C8; // float32
    pub const mana_regen: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Royale_With_Cheese {
    pub const shield: usize = 0x12C8; // float32
    pub const regen: usize = 0x12CC; // float32
    pub const idle: usize = 0x12D0; // float32
    pub const m_nDamageAbsorbed: usize = 0x12D4; // int32
    pub const m_timeLastTick: usize = 0x12D8; // GameTime_t
    pub const m_timeLastDamage: usize = 0x12DC; // GameTime_t
    pub const nFXIndex: usize = 0x12E0; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Rubick_Arcane_Supremacy {
    pub const cast_range: usize = 0x12C8; // int32
    pub const spell_amp: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Rubick_FadeBolt {
    pub const radius: usize = 0x12C8; // int32
    pub const damage: usize = 0x12CC; // int32
    pub const jump_damage_reduction_pct: usize = 0x12D0; // int32
    pub const jump_delay: usize = 0x12D4; // float32
    pub const m_iCurJumpCount: usize = 0x12D8; // int32
    pub const m_vCurTargetLoc: usize = 0x12DC; // Vector
    pub const m_hHitEntities: usize = 0x12E8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Rubick_FadeBoltBuff {
    pub const attack_damage_bonus: usize = 0x12C8; // int32
    pub const attack_damage_reduction: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Rubick_FadeBoltDebuff {
    pub const attack_damage_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Rubick_NullField {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Rubick_NullField_Effect {
    pub const magic_damage_reduction_pct: usize = 0x12C8; // int32
    pub const m_bWasHidden: usize = 0x12CC; // bool
}

pub mod CDOTA_Modifier_Rubick_SpellSteal {
    pub const m_strActivityModifier: usize = 0x12C8; // CUtlString
    pub const stolen_debuff_amp: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Rubick_Telekinesis {
    pub const m_fStartTime: usize = 0x12C8; // GameTime_t
    pub const m_fEndTime: usize = 0x12CC; // GameTime_t
    pub const m_fTargetHeight: usize = 0x12D0; // float32
    pub const m_fCurHeight: usize = 0x12D4; // float32
    pub const m_vStartLoc: usize = 0x12D8; // Vector
    pub const m_vCurLoc: usize = 0x12E4; // Vector
    pub const max_land_distance: usize = 0x12F0; // int32
    pub const fall_duration: usize = 0x12F4; // float32
    pub const m_bOverrideDuration: usize = 0x12F8; // bool
    pub const m_flOverrideDuration: usize = 0x12FC; // float32
}

pub mod CDOTA_Modifier_Rune_Arcane {
    pub const m_iCooldownReductionPct: usize = 0x12C8; // int32
    pub const m_iCostReductionPct: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Rune_ExtraDamage {
    pub const m_iAdditionalDamagePct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Rune_Shield {
    pub const damage_absorb: usize = 0x12C8; // int32
    pub const m_bStartedTimer: usize = 0x12CC; // bool
    pub const m_nDamageAbsorbed: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_SandKing_CausticFinale {
    pub const caustic_finale_duration: usize = 0x12C8; // float32
    pub const m_InFlightAttackRecords: usize = 0x12D0; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_SandKing_CausticFinaleOrb {
    pub const caustic_finale_radius: usize = 0x12C8; // int32
    pub const caustic_finale_damage_base: usize = 0x12CC; // int32
    pub const caustic_finale_damage_pct: usize = 0x12D0; // int32
    pub const caustic_finale_slow: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_SandKing_Epicenter {
    pub const epicenter_radius_base: usize = 0x12C8; // int32
    pub const epicenter_radius_increment: usize = 0x12CC; // int32
    pub const epicenter_pulses: usize = 0x12D0; // int32
    pub const epicenter_radius: usize = 0x12D4; // int32[16]
    pub const m_iMaxPulses: usize = 0x1314; // int32
    pub const m_iPulseCount: usize = 0x1318; // int32
}

pub mod CDOTA_Modifier_SandKing_Epicenter_Slow {
    pub const epicenter_slow: usize = 0x12C8; // int32
    pub const epicenter_slow_as: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_SandKing_SandStorm {
    pub const sand_storm_radius: usize = 0x12D8; // int32
    pub const sand_storm_damage: usize = 0x12DC; // int32
    pub const damage_tick_rate: usize = 0x12E0; // float32
    pub const m_flLastDamageTime: usize = 0x12E4; // GameTime_t
    pub const m_nSandStormParticleIndex1: usize = 0x12E8; // ParticleIndex_t
    pub const m_nSandStormParticleIndex2: usize = 0x12EC; // ParticleIndex_t
    pub const m_hThinker: usize = 0x12F8; // CHandle< C_BaseEntity >
    pub const sand_storm_move_speed: usize = 0x12FC; // int32
    pub const vecSpawnPos: usize = 0x1300; // Vector
    pub const m_iExplosionQuadrant: usize = 0x130C; // int32
    pub const m_iExplosionDistance: usize = 0x1310; // int32
}

pub mod CDOTA_Modifier_SandKing_SandStorm_Slow {
    pub const slow: usize = 0x12C8; // int32
    pub const blind: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_SandKing_SandStorm_Slow_Aura_Thinker {
    pub const sand_storm_radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Sand_King_BurrowStrike {
    pub const m_vPopupLocation: usize = 0x12C8; // Vector
    pub const m_bTeleported: usize = 0x12D4; // bool
}

pub mod CDOTA_Modifier_Sand_King_Shard {
    pub const shard_move_distance: usize = 0x12C8; // float32
    pub const shard_interval: usize = 0x12CC; // float32
    pub const vLastPos: usize = 0x12D0; // Vector
    pub const m_fAccumulatedDistance: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_SatyrHellcaller_UnholyAura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_SatyrHellcaller_UnholyAura_Bonus {
    pub const health_regen: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_SatyrSoulstealer_ManaBurn {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_SatyrTrickster_Purge {
    pub const purge_rate: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_ScoutBonuses {
    pub const reveal_radius: usize = 0x12C8; // int32
    pub const increase_health: usize = 0x12CC; // float32
    pub const max_health: usize = 0x12D0; // int32
    pub const increase_armor: usize = 0x12D4; // float32
    pub const max_armor: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_ScoutVisible {
    pub const reveal_radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_ScriptedMotionController {
    pub const m_hHorizontalControlFunction: usize = 0x12C8; // HSCRIPT
    pub const m_hVerticalControlFunction: usize = 0x12D0; // HSCRIPT
    pub const m_hOnDestroyFunction: usize = 0x12D8; // HSCRIPT
    pub const m_parameterTable: usize = 0x12E0; // CVariantBase< CVariantDefaultAllocator >
}

pub mod CDOTA_Modifier_Seasonal_Diretide2020_HighFiveRequested {
    pub const acknowledge_range: usize = 0x12C8; // int32
    pub const think_interval: usize = 0x12CC; // float32
    pub const acknowledged_cooldown: usize = 0x12D0; // float32
    pub const m_bAcknowledged: usize = 0x12D4; // bool
    pub const m_bFirstThink: usize = 0x12D5; // bool
    pub const high_five_level: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Seasonal_Penguin {
    pub const m_fLifetimeGained: usize = 0x12C8; // float32
    pub const m_vTargetPos: usize = 0x12CC; // Vector
    pub const m_nCurrentSpeed: usize = 0x12D8; // int32
    pub const m_fLastBumpTime: usize = 0x12DC; // GameTime_t
    pub const m_vLastPos: usize = 0x12E0; // Vector
    pub const m_hLastHit: usize = 0x12EC; // CHandle< C_BaseEntity >
    pub const m_vDir: usize = 0x12F0; // Vector
    pub const m_vRunEndPos: usize = 0x12FC; // Vector
    pub const m_bIsInInitialRun: usize = 0x1308; // bool
    pub const m_vJumpEndPos: usize = 0x130C; // Vector
    pub const m_bPlayedVroomSinceLastCrash: usize = 0x1318; // bool
    pub const m_nVroomFX: usize = 0x131C; // ParticleIndex_t
    pub const m_fLastSpeedStepTime: usize = 0x1320; // GameTime_t
    pub const m_fLastSpeechTime: usize = 0x1324; // GameTime_t
    pub const m_nFXStackIndex: usize = 0x1328; // ParticleIndex_t
    pub const m_nBumpsSinceLastCrash: usize = 0x132C; // int32
    pub const m_hLastBumpingHero: usize = 0x1330; // CHandle< C_BaseEntity >
    pub const initial_speed: usize = 0x1334; // int32
    pub const max_speed: usize = 0x1338; // int32
    pub const speed_step: usize = 0x133C; // int32
    pub const speed_step_interval: usize = 0x1340; // float32
    pub const bump_delay: usize = 0x1344; // float32
    pub const bump_delay_absolute: usize = 0x1348; // float32
    pub const bump_collision_radius: usize = 0x134C; // int32
    pub const run_distance: usize = 0x1350; // int32
    pub const jump_distance: usize = 0x1354; // int32
    pub const speed_after_crash: usize = 0x1358; // int32
    pub const speed_gain_per_hero_bump: usize = 0x135C; // int32
    pub const max_gainable_lifetime: usize = 0x1360; // float32
    pub const lifetime_gain_per_hero_bump: usize = 0x1364; // float32
    pub const min_speech_repeat_time: usize = 0x1368; // float32
}

pub mod CDOTA_Modifier_Seasonal_Summon_CNY_Balloon_Thinker {
    pub const speed: usize = 0x12C8; // float32
    pub const min_distance_before_bounce: usize = 0x12CC; // int32
    pub const drag: usize = 0x12D0; // float32
    pub const max_height: usize = 0x12D4; // float32
    pub const max_vertical_move_time: usize = 0x12D8; // float32
    pub const bounce_turn_angle: usize = 0x12DC; // float32
    pub const bounce_turn_angle_tree: usize = 0x12E0; // float32
    pub const bounce_delay: usize = 0x12E4; // float32
    pub const m_vLastPos: usize = 0x12E8; // Vector
    pub const m_vDir: usize = 0x12F4; // Vector
    pub const m_flSpeed: usize = 0x1300; // float32
    pub const m_flDistRemaining: usize = 0x1304; // float32
    pub const m_flTreeTimeRemaining: usize = 0x1308; // float32
    pub const m_hLastHit: usize = 0x130C; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Seasonal_Summon_Common_Thinker {
    pub const grace_period: usize = 0x12C8; // float32
    pub const search_distance: usize = 0x12CC; // int32
    pub const think_interval: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Seasonal_Summon_Snowman_Thinker {
    pub const grace_period: usize = 0x12C8; // float32
    pub const search_distance: usize = 0x12CC; // int32
    pub const think_interval: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Seasonal_Summon_TI11_Balloon_Thinker {
    pub const speed: usize = 0x12C8; // float32
    pub const min_distance_before_bounce: usize = 0x12CC; // int32
    pub const drag: usize = 0x12D0; // float32
    pub const max_height: usize = 0x12D4; // float32
    pub const max_vertical_move_time: usize = 0x12D8; // float32
    pub const bounce_turn_angle: usize = 0x12DC; // float32
    pub const bounce_turn_angle_tree: usize = 0x12E0; // float32
    pub const bounce_delay: usize = 0x12E4; // float32
    pub const max_bounces: usize = 0x12E8; // int32
    pub const max_model_scale: usize = 0x12EC; // float32
    pub const power_growth_exponent: usize = 0x12F0; // float32
    pub const m_vLastPos: usize = 0x12F4; // Vector
    pub const m_vDir: usize = 0x1300; // Vector
    pub const m_flSpeed: usize = 0x130C; // float32
    pub const m_flDistRemaining: usize = 0x1310; // float32
    pub const m_flTreeTimeRemaining: usize = 0x1314; // float32
    pub const m_nTimesBounced: usize = 0x1318; // int32
    pub const m_flModelScale: usize = 0x131C; // float32
    pub const m_hLastHit: usize = 0x1320; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Seasonal_Summon_TI11_Balloon_Visuals {
    pub const model_scale_per_stack: usize = 0x12C8; // float32
    pub const m_bIsRadiant: usize = 0x12CC; // bool
}

pub mod CDOTA_Modifier_Seasonal_Summon_TI9_Balloon_Thinker {
    pub const speed: usize = 0x12C8; // float32
    pub const min_distance_before_bounce: usize = 0x12CC; // int32
    pub const drag: usize = 0x12D0; // float32
    pub const max_height: usize = 0x12D4; // float32
    pub const max_vertical_move_time: usize = 0x12D8; // float32
    pub const bounce_turn_angle: usize = 0x12DC; // float32
    pub const bounce_turn_angle_tree: usize = 0x12E0; // float32
    pub const bounce_delay: usize = 0x12E4; // float32
    pub const m_vLastPos: usize = 0x12E8; // Vector
    pub const m_vDir: usize = 0x12F4; // Vector
    pub const m_flSpeed: usize = 0x1300; // float32
    pub const m_flDistRemaining: usize = 0x1304; // float32
    pub const m_flTreeTimeRemaining: usize = 0x1308; // float32
    pub const m_hLastHit: usize = 0x130C; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Seasonal_TI10_DiscoBall_Channeling {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const m_nGesture: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Seasonal_TI10_HighFiveRequested {
    pub const acknowledge_range: usize = 0x12C8; // int32
    pub const think_interval: usize = 0x12CC; // float32
    pub const acknowledged_cooldown: usize = 0x12D0; // float32
    pub const m_bAcknowledged: usize = 0x12D4; // bool
    pub const m_bFirstThink: usize = 0x12D5; // bool
    pub const high_five_level: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Seasonal_TI10_Portal {
    pub const m_vSpawnPos: usize = 0x12C8; // Vector
    pub const m_vecEmissions: usize = 0x12D8; // CUtlVector< Vector >
    pub const m_bChatSent: usize = 0x12F0; // bool
    pub const m_nPhase: usize = 0x12F4; // int32
    pub const m_nNumEmitted: usize = 0x12F8; // int32
    pub const m_nNumDrops: usize = 0x12FC; // int32
    pub const m_flStartTime: usize = 0x1300; // GameTime_t
    pub const m_flLastEmitTime: usize = 0x1304; // GameTime_t
    pub const m_bIsOnRadiant: usize = 0x1308; // bool
    pub const m_bUseLargeEmit: usize = 0x1309; // bool
    pub const m_nOutcome: usize = 0x130C; // int32
    pub const PHASE_OPEN: usize = 0x1320; // int32
    pub const PHASE_EMIT: usize = 0x1324; // int32
    pub const PHASE_EMIT_DONE: usize = 0x1328; // int32
    pub const PHASE_CLOSE: usize = 0x132C; // int32
}

pub mod CDOTA_Modifier_Seasonal_TI10_SoccerBall {
    pub const m_vPost1: usize = 0x12C8; // Vector
    pub const m_vPost2: usize = 0x12D4; // Vector
    pub const m_vPostForward: usize = 0x12E0; // Vector
    pub const m_nPost1FXIndex: usize = 0x12EC; // ParticleIndex_t
    pub const m_nPost2FXIndex: usize = 0x12F0; // ParticleIndex_t
    pub const m_nPost3FXIndex: usize = 0x12F4; // ParticleIndex_t
    pub const PHASE_START: usize = 0x12F8; // int32
    pub const PHASE_GESTURE: usize = 0x12FC; // int32
    pub const PHASE_FX: usize = 0x1300; // int32
    pub const PHASE_MOTION: usize = 0x1304; // int32
    pub const PHASE_IMPACT: usize = 0x1308; // int32
    pub const PHASE_FAIL: usize = 0x130C; // int32
    pub const PHASE_CROSSED_GOAL: usize = 0x1310; // int32
    pub const PHASE_GOAL: usize = 0x1314; // int32
    pub const tick_interval: usize = 0x1318; // float32
    pub const forward_move_speed: usize = 0x131C; // float32
    pub const magnus_coefficient: usize = 0x1320; // float32
    pub const magnus_falloff: usize = 0x1324; // float32
    pub const drag_coefficient: usize = 0x1328; // float32
    pub const min_delay: usize = 0x132C; // float32
    pub const max_delay: usize = 0x1330; // float32
    pub const hit_radius: usize = 0x1334; // int32
    pub const m_nFXIndex: usize = 0x1338; // ParticleIndex_t
    pub const m_vVelocity: usize = 0x133C; // Vector
    pub const m_vSpawnPos: usize = 0x1348; // Vector
    pub const m_vTargetPos: usize = 0x1354; // Vector
    pub const m_nPhase: usize = 0x1360; // int32
    pub const m_nHits: usize = 0x1364; // int32
    pub const m_flMagnusValue: usize = 0x1368; // float32
    pub const m_flStartTime: usize = 0x136C; // GameTime_t
    pub const m_flDelay: usize = 0x1370; // float32
    pub const m_flStateChangeTime: usize = 0x1374; // GameTime_t
    pub const m_bFirstThink: usize = 0x1378; // bool
    pub const m_bGroundHit: usize = 0x1379; // bool
    pub const m_nTargetPlayerID: usize = 0x137C; // PlayerID_t
    pub const m_flCurTime: usize = 0x1380; // float32
    pub const m_flStartZ: usize = 0x1384; // float32
    pub const m_flAirTime: usize = 0x1388; // float32
    pub const m_flHeight: usize = 0x138C; // float32
}

pub mod CDOTA_Modifier_Seasonal_TI11_BubbleGun {
    pub const think_interval: usize = 0x12C8; // float32
    pub const max_charges: usize = 0x12CC; // int32
    pub const m_nChargesUsed: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Seasonal_TI11_CongaLineSlow {
    pub const m_flMaxSpeed: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Seasonal_TI11_DuelAccepted {
    pub const m_nDuelOpponent: usize = 0x12C8; // PlayerID_t
}

pub mod CDOTA_Modifier_Seasonal_TI11_RockPaperScissors {
    pub const challenge_duration: usize = 0x12C8; // float32
    pub const reveal_duration: usize = 0x12CC; // float32
    pub const reveal_delay: usize = 0x12D0; // float32
    pub const completed_cooldown: usize = 0x12D4; // float32
    pub const think_interval: usize = 0x12D8; // float32
    pub const acknowledge_range: usize = 0x12DC; // float32
    pub const m_flRevealTime: usize = 0x12E4; // GameTime_t
    pub const m_nOverheadFXIndex: usize = 0x12E8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Seasonal_TI11_RockPaperScissors_Playing {
    pub const input_duration: usize = 0x12C8; // float32
    pub const think_interval: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Seasonal_TI9_Shovel_BabyRoshan {
    pub const m_bFirstThink: usize = 0x12C8; // bool
    pub const m_flStartTime: usize = 0x12CC; // GameTime_t
    pub const m_vecStartingPos: usize = 0x12D0; // Vector
}

pub mod CDOTA_Modifier_Seasonal_TI9_Shovel_Stasis_Trap {
    pub const m_hHero: usize = 0x12D8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Seer_Stone {
    pub const cast_range_bonus: usize = 0x12C8; // int32
    pub const vision_bonus: usize = 0x12CC; // int32
    pub const mana_regen: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Seer_Stone_Truesight {
    pub const radius: usize = 0x12C8; // int32
    pub const duration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_SetScaleset {
    pub const m_Scaleset: usize = 0x12C8; // CUtlString
    pub const m_bActive: usize = 0x12D0; // bool
}

pub mod CDOTA_Modifier_ShadowAmulet_Fade {
    pub const fade_time: usize = 0x12C8; // float32
    pub const m_flFadeTime: usize = 0x12CC; // GameTime_t
    pub const m_bFaded: usize = 0x12D0; // bool
}

pub mod CDOTA_Modifier_ShadowShaman_SerpentWard {
    pub const scepter_range: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_ShadowShaman_Shackles {
    pub const tick_interval: usize = 0x12C8; // float32
    pub const total_damage: usize = 0x12CC; // float32
    pub const channel_time: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_ShadowShaman_Voodoo {
    pub const movespeed: usize = 0x12C8; // int32
    pub const damage_amp: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Shadow_Demon_Disruption {
    pub const m_flHealth: usize = 0x12C8; // float32
    pub const m_flDisseminate_Duration: usize = 0x12CC; // float32
    pub const m_hDisseminateAbility: usize = 0x12D0; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Shadow_Demon_Disseminate {
    pub const damage_reflection_pct: usize = 0x12C8; // int32
    pub const radius: usize = 0x12CC; // int32
    pub const m_flLastDispersionPulseEffectTime: usize = 0x12D0; // GameTime_t
}

pub mod CDOTA_Modifier_Shadow_Demon_Shadow_Poison {
    pub const stack_damage: usize = 0x12C8; // float32
    pub const bonus_stack_damage: usize = 0x12CC; // float32
    pub const max_multiply_stacks: usize = 0x12D0; // int32
    pub const m_nFXStackIndex: usize = 0x12D4; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Shadow_Demon_Soul_Catcher {
    pub const actual_health_lost: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Shadow_Demon_Soul_Catcher_Illusion {
    pub const hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Shadow_Demon_Soul_Catcher_SpellAmp {
    pub const num_heroes: usize = 0x12C8; // int32
    pub const bonus_spell_amp: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Shadowraze_Counter {
    pub const shadowraze_cooldown: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Shadowraze_Debuff {
    pub const movement_speed_pct: usize = 0x12C8; // int32
    pub const turn_rate_pct: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_SheepStick_Debuff {
    pub const sheep_movement_speed: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Shredder_Chakram_Debuff {
    pub const slow: usize = 0x12C8; // int32
    pub const slow_health_percentage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Shredder_Chakram_Thinker {
    pub const damage_per_second: usize = 0x12C8; // int32
    pub const mana_per_second: usize = 0x12CC; // float32
    pub const radius: usize = 0x12D0; // float32
    pub const break_distance: usize = 0x12D4; // float32
    pub const damage_interval: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Shredder_Flamethrower {
    pub const length: usize = 0x12C8; // int32
    pub const width: usize = 0x12CC; // int32
    pub const damage_per_second: usize = 0x12D0; // int32
    pub const m_nBeamFXIndex: usize = 0x12D4; // ParticleIndex_t
    pub const m_hBeamEnd: usize = 0x12D8; // CHandle< C_BaseEntity >
    pub const m_flLastHit: usize = 0x12DC; // GameTime_t
    pub const m_vecBurningTrees: usize = 0x12E0; // CUtlVector< C_BaseEntity* >
}

pub mod CDOTA_Modifier_Shredder_Flamethrower_Damage {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const m_nActiveFXIndex: usize = 0x12CC; // ParticleIndex_t
    pub const m_nActiveStatusFXIndex: usize = 0x12D0; // ParticleIndex_t
    pub const damage_per_second: usize = 0x12D4; // float32
    pub const building_dmg_pct: usize = 0x12D8; // int32
    pub const move_slow_pct: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Shredder_Flamethrower_TreeFire_Thinker {
    pub const m_vecTreeFires: usize = 0x12C8; // CUtlVector< CUtlPair< CHandle< C_BaseEntity >, GameTime_t > >
    pub const m_bCleanupWhenEmpty: usize = 0x12E0; // bool
}

pub mod CDOTA_Modifier_Shredder_ReactiveArmor_Bomb {
    pub const initial_shield: usize = 0x12C8; // float32
    pub const max_shield: usize = 0x12CC; // float32
    pub const shield_per_sec: usize = 0x12D0; // float32
    pub const shield_per_sec_per_enemy: usize = 0x12D4; // float32
    pub const duration: usize = 0x12D8; // float32
    pub const base_explosion: usize = 0x12DC; // float32
    pub const radius: usize = 0x12E0; // int32
    pub const explosion_radius: usize = 0x12E4; // int32
    pub const m_nDamageAbsorbed: usize = 0x12E8; // int32
    pub const m_timeLastTick: usize = 0x12EC; // GameTime_t
    pub const m_StartTime: usize = 0x12F0; // GameTime_t
    pub const m_nLastParticleTime: usize = 0x12F4; // int32
}

pub mod CDOTA_Modifier_Shredder_Reactive_Armor {
    pub const stack_limit: usize = 0x12C8; // int32
    pub const stack_duration: usize = 0x12CC; // float32
    pub const m_flStackDieTime: usize = 0x12D0; // GameTime_t
    pub const m_pFXIndex: usize = 0x12D4; // ParticleIndex_t[4]
}

pub mod CDOTA_Modifier_Shredder_Reactive_Armor_Stack {
    pub const bonus_armor: usize = 0x12C8; // float32
    pub const bonus_hp_regen: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Shredder_TimberChain {
    pub const speed: usize = 0x12C8; // int32
    pub const radius: usize = 0x12CC; // int32
    pub const damage: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Shredder_WhirlingDeath_Debuff {
    pub const stat_loss_pct: usize = 0x12C8; // int32
    pub const stat_loss_univ: usize = 0x12CC; // int32
    pub const iPrimaryAttribute: usize = 0x12D0; // int32
    pub const flStatLossStr: usize = 0x12D4; // float32
    pub const flStatLossAgi: usize = 0x12D8; // float32
    pub const flStatLossInt: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Silencer_CurseOfTheSilent {
    pub const damage: usize = 0x12C8; // int32
    pub const nFxIndex: usize = 0x12CC; // ParticleIndex_t
    pub const penalty_duration: usize = 0x12D0; // int32
    pub const penalty_multiplier: usize = 0x12D4; // float32
    pub const movespeed: usize = 0x12D8; // int32
    pub const undispellable: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Silencer_GlaivesOfWisdom {
    pub const intellect_damage_pct: usize = 0x12C8; // float32
    pub const m_InFlightAttackRecords: usize = 0x12D0; // CUtlVector< int16 >
    pub const m_InFlightSilenceAttackRecords: usize = 0x12E8; // CUtlVector< int16 >
    pub const m_pAttackCounterBuff: usize = 0x1300; // CDOTA_Buff*
}

pub mod CDOTA_Modifier_Silencer_GlaivesOfWisdom_Buff {
    pub const int_steal: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Silencer_GlaivesOfWisdom_Debuff {
    pub const int_steal: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Silencer_LastWord {
    pub const duration: usize = 0x12C8; // int32
    pub const spellcast: usize = 0x12CC; // bool
    pub const damage: usize = 0x12D0; // int32
    pub const debuff_duration: usize = 0x12D4; // float32
    pub const int_multiplier: usize = 0x12D8; // float32
    pub const scepter_bonus_damage: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Silver_Edge_WindWalk {
    pub const windwalk_bonus_damage: usize = 0x12D8; // int32
    pub const windwalk_movement_speed: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_SkeletonKing_HellfireBlast {
    pub const blast_dot_damage: usize = 0x12C8; // int32
    pub const blast_slow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_SkeletonKing_HellfireBlast_Skeleton_Buff {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
    pub const bonus_move_speed: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_SkeletonKing_MortalStrike {
    pub const crit_mult: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_SkeletonKing_Reincarnation {
    pub const reincarnate_time: usize = 0x12C8; // float32
    pub const scepter_aura_radius: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_SkeletonKing_Reincarnation_Scepter {
    pub const scepter_duration: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_SkeletonKing_Reincarnation_Scepter_Active {
    pub const scepter_move_speed_pct: usize = 0x12C8; // int32
    pub const scepter_attack_speed: usize = 0x12CC; // int32
    pub const m_hTarget: usize = 0x12D0; // CHandle< C_BaseEntity >
    pub const m_bPassive: usize = 0x12D4; // bool
    pub const m_bKillAtEnd: usize = 0x12D5; // bool
}

pub mod CDOTA_Modifier_SkeletonKing_Reincarnation_Scepter_RespawnTime {
    pub const scepter_respawn_pct: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_SkeletonKing_Reincarnation_Slow {
    pub const movespeed: usize = 0x12C8; // int32
    pub const attackslow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_SkeletonKing_VampiricAura {
    pub const vampiric_aura: usize = 0x12C8; // int32
    pub const creep_lifesteal_reduction_pct: usize = 0x12CC; // int32
    pub const skeleton_duration: usize = 0x12D0; // float32
    pub const max_skeleton_charges: usize = 0x12D4; // int32
    pub const m_iKillCounter: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_SkeletonKing_VampiricAura_Summon {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const talent_skeleton_damage: usize = 0x12CC; // int32
    pub const m_bRespawnReady: usize = 0x12D0; // bool
    pub const m_bRespawnConsumed: usize = 0x12D1; // bool
    pub const m_bKillParentOnDestroy: usize = 0x12D2; // bool
    pub const m_flRespawnTime: usize = 0x12D4; // GameTime_t
    pub const vampiric_aura: usize = 0x12D8; // int32
    pub const gold_bounty: usize = 0x12DC; // int32
    pub const xp_bounty: usize = 0x12E0; // int32
    pub const skeleton_building_damage_reduction: usize = 0x12E4; // int32
    pub const skeleton_bonus_hero_damage: usize = 0x12E8; // int32
}

pub mod CDOTA_Modifier_SkeletonKing_VampiricAura_Summon_Thinker {
    pub const m_iCount: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Skywrath_Mage_Ancient_Seal {
    pub const resist_debuff: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Skywrath_Mage_Arcane_Bolt_Lifesteal {
    pub const lifesteal_amt: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Skywrath_Mage_Concussive_Shot {
    pub const movement_speed_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Skywrath_Mage_Mystic_Flare {
    pub const radius: usize = 0x12C8; // int32
    pub const damage: usize = 0x12CC; // int32
    pub const duration: usize = 0x12D0; // float32
    pub const damage_interval: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Skywrath_Mage_Shard {
    pub const stack_duration: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Skywrath_Mage_Shard_Bonus {
    pub const bonus_intelligence: usize = 0x12C8; // int32
    pub const bonus_armor: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Skywrath_Mage_Shard_Bonus_Counter {
    pub const bonus_intelligence: usize = 0x12C8; // int32
    pub const bonus_armor: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Slardar_Amplify_Damage {
    pub const armor_reduction: usize = 0x12C8; // int32
    pub const scepter_delay: usize = 0x12CC; // float32
    pub const undispellable: usize = 0x12D0; // int32
    pub const puddle_radius: usize = 0x12D4; // int32
    pub const puddle_duration: usize = 0x12D8; // float32
    pub const m_vecLastPuddle: usize = 0x12DC; // Vector
    pub const m_hPuddle: usize = 0x12E8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Slardar_Bash_Active {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const duration: usize = 0x12CC; // float32
    pub const attack_count: usize = 0x12D0; // int32
    pub const river_damage: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Slardar_Puddle_Thinker {
    pub const puddle_radius: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Slardar_Slithereen_Crush {
    pub const crush_extra_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Slardar_Sprint {
    pub const bonus_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Slardar_Sprint_River {
    pub const river_speed: usize = 0x12C8; // int32
    pub const puddle_regen: usize = 0x12CC; // int32
    pub const puddle_armor: usize = 0x12D0; // int32
    pub const puddle_status_resistance: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Slark_DarkPact_Pulses {
    pub const radius: usize = 0x12C8; // int32
    pub const total_damage: usize = 0x12CC; // int32
    pub const total_pulses: usize = 0x12D0; // int32
    pub const self_damage_pct: usize = 0x12D4; // int32
    pub const pulse_interval: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Slark_Depth_Shroud {
    pub const m_nFXIndex: usize = 0x12D8; // ParticleIndex_t
    pub const bonus_movement_speed: usize = 0x12DC; // int32
    pub const bonus_regen: usize = 0x12E0; // float32
}

pub mod CDOTA_Modifier_Slark_Depth_Shroud_Thinker {
    pub const radius: usize = 0x12C8; // int32
    pub const m_hVisibleEntity: usize = 0x12CC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Slark_EssenceShift {
    pub const agi_gain: usize = 0x12C8; // int32
    pub const duration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Slark_EssenceShift_Debuff {
    pub const stat_loss: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Slark_Fish_Bait_Post {
    pub const attack_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Slark_Fish_Bait_Pre {
    pub const movement_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Slark_Pounce {
    pub const pounce_distance: usize = 0x12C8; // int32
    pub const pounce_distance_scepter: usize = 0x12CC; // int32
    pub const pounce_speed: usize = 0x12D0; // float32
    pub const pounce_acceleration: usize = 0x12D4; // float32
    pub const pounce_radius: usize = 0x12D8; // int32
    pub const pounce_damage: usize = 0x12DC; // int32
    pub const leash_duration: usize = 0x12E0; // float32
}

pub mod CDOTA_Modifier_Slark_Pounce_Leash {
    pub const leash_radius: usize = 0x12C8; // int32
    pub const vLeashLocation: usize = 0x12CC; // Vector
    pub const m_bEscaped: usize = 0x12D8; // bool
    pub const m_bFishBait: usize = 0x12D9; // bool
}

pub mod CDOTA_Modifier_Slark_ShadowDance {
    pub const m_hVisibleEntity: usize = 0x12D8; // CHandle< C_BaseEntity >
    pub const m_nFXIndex: usize = 0x12DC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Slark_ShadowDance_Aura {
    pub const scepter_aoe: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Slark_ShadowDance_Passive {
    pub const activation_delay: usize = 0x12C8; // float32
    pub const neutral_disable: usize = 0x12CC; // float32
    pub const m_bPendingRefresh: usize = 0x12D0; // bool
    pub const m_fPendingStateChangeTime: usize = 0x12D4; // GameTime_t
    pub const m_NeutralHitTimer: usize = 0x12D8; // CountdownTimer
}

pub mod CDOTA_Modifier_Slark_ShadowDance_PassiveRegen {
    pub const bonus_movement_speed: usize = 0x12C8; // int32
    pub const bonus_regen: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Slark_ShadowDance_Visual {
    pub const m_bScepter: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_Smoke_Of_Deceit {
    pub const bonus_movement_speed: usize = 0x12D8; // int32
    pub const visibility_radius: usize = 0x12DC; // int32
    pub const secondary_application_radius: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_Snapfire_FiresnapCookie_PreHop {
    pub const jump_duration: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Snapfire_GobbleUp_BellyHasUnit {
    pub const m_hGobbledUnit: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_nOriginalControllingUnit: usize = 0x12CC; // PlayerID_t
    pub const m_bUnitWasLaunched: usize = 0x12D0; // bool
    pub const max_time_in_belly: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Snapfire_LilShredder_Attack {
    pub const damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Snapfire_LilShredder_Attack_Reduction {
    pub const damage_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Snapfire_LilShredder_Buff {
    pub const m_InFlightAttackRecords: usize = 0x12D0; // CUtlVector< int16 >
    pub const m_nIsActive: usize = 0x12E8; // int32
    pub const m_bHasAttackTalent: usize = 0x12EC; // bool
    pub const m_bBonusAttack: usize = 0x12ED; // bool
    pub const attack_range_bonus: usize = 0x12F0; // int32
    pub const buffed_attacks: usize = 0x12F4; // int32
    pub const base_attack_time: usize = 0x12F8; // float32
    pub const attack_speed_bonus: usize = 0x12FC; // int32
    pub const armor_duration: usize = 0x1300; // float32
}

pub mod CDOTA_Modifier_Snapfire_LilShredder_Debuff {
    pub const armor_reduction_per_attack: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Snapfire_Magma_Burn_Slow {
    pub const m_nMoveSlowPct: usize = 0x12C8; // int32
    pub const m_fBurnDamagePerTick: usize = 0x12CC; // float32
    pub const move_slow_pct: usize = 0x12D0; // int32
    pub const burn_interval: usize = 0x12D4; // float32
    pub const burn_damage: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Snapfire_Magma_Thinker {
    pub const burn_interval: usize = 0x12C8; // float32
    pub const impact_radius: usize = 0x12CC; // int32
    pub const move_slow_pct: usize = 0x12D0; // int32
    pub const burn_linger_duration: usize = 0x12D4; // float32
    pub const m_nPathEffectIndex: usize = 0x12D8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Snapfire_MortimerKisses {
    pub const m_fIntervalPerRocket: usize = 0x12C8; // float32
    pub const m_flFacingTarget: usize = 0x12CC; // float32
    pub const m_nBeamFXIndex: usize = 0x12D0; // ParticleIndex_t
    pub const m_hBeamEnd: usize = 0x12D4; // CHandle< C_BaseEntity >
    pub const m_flCurDistance: usize = 0x12D8; // float32
    pub const m_vAimTarget: usize = 0x12DC; // Vector
    pub const m_fLastTurnAmount: usize = 0x12E8; // float32
    pub const m_nProjectilesLaunched: usize = 0x12EC; // int32
    pub const m_bDestroyOnNextThink: usize = 0x12F0; // bool
    pub const m_nProjectilesToLaunch: usize = 0x12F4; // int32
    pub const m_bHasProjectileTalent: usize = 0x12F8; // bool
    pub const projectile_count: usize = 0x12FC; // int32
    pub const projectile_speed: usize = 0x1300; // int32
    pub const projectile_width: usize = 0x1304; // int32
    pub const projectile_vision: usize = 0x1308; // int32
    pub const turn_rate: usize = 0x130C; // float32
    pub const min_range: usize = 0x1310; // int32
    pub const impact_radius: usize = 0x1314; // int32
    pub const min_lob_travel_time: usize = 0x1318; // float32
    pub const max_lob_travel_time: usize = 0x131C; // float32
    pub const delay_after_last_projectile: usize = 0x1320; // float32
}

pub mod CDOTA_Modifier_Snapfire_Scatterblast_Slow {
    pub const m_bIsPointBlank: usize = 0x12C8; // bool
    pub const movement_slow_pct: usize = 0x12CC; // int32
    pub const attack_slow_pct: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Snapfire_SpitCreep_ArcingUnit {
    pub const min_range: usize = 0x12C8; // int32
    pub const min_lob_travel_time: usize = 0x12CC; // float32
    pub const max_lob_travel_time: usize = 0x12D0; // float32
    pub const impact_radius: usize = 0x12D4; // int32
    pub const projectile_vision: usize = 0x12D8; // int32
    pub const stun_duration: usize = 0x12DC; // float32
    pub const min_height_above_lowest: usize = 0x12E0; // float32
    pub const min_height_above_highest: usize = 0x12E4; // float32
    pub const min_acceleration: usize = 0x12E8; // float32
    pub const max_acceleration: usize = 0x12EC; // float32
}

pub mod CDOTA_Modifier_Sniper_Assassinate_Crit {
    pub const scepter_crit_damage: usize = 0x12C8; // int32
    pub const scatter_damage_pct: usize = 0x12CC; // int32
    pub const is_secondary: usize = 0x12D0; // bool
}

pub mod CDOTA_Modifier_Sniper_Concussive_Grenade {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Sniper_Headshot {
    pub const proc_chance: usize = 0x12C8; // int32
    pub const slow_duration: usize = 0x12CC; // float32
    pub const knockback_distance: usize = 0x12D0; // float32
    pub const bonus_damage_creeps: usize = 0x12D4; // int32
    pub const bonus_attack_range: usize = 0x12D8; // int32
    pub const damage: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Sniper_Headshot_Slow {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Sniper_Shrapnel_Slow {
    pub const slow_movement_speed: usize = 0x12C8; // int32
    pub const shrapnel_damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Sniper_Shrapnel_Thinker {
    pub const radius: usize = 0x12C8; // int32
    pub const m_bExploded: usize = 0x12CC; // bool
}

pub mod CDOTA_Modifier_Sniper_TakeAim {
    pub const bonus_attack_range: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Sniper_TakeAim_Bonus {
    pub const slow: usize = 0x12C8; // int32
    pub const active_attack_range_bonus: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Soul_Ring_Buff {
    pub const m_iTempExtraMaxMana: usize = 0x12C8; // int32
    pub const m_iManaToRemove: usize = 0x12CC; // int32
    pub const duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Spawnlord_Aura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Spawnlord_Aura_Bonus {
    pub const lifesteal: usize = 0x12C8; // int32
    pub const hp_regen: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Spawnlord_Master_Freeze {
    pub const m_bFreezeAttack: usize = 0x12C8; // bool
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Spawnlord_Master_Freeze_Root {
    pub const max_ticks: usize = 0x12C8; // int32
    pub const damage: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Spawnlord_Master_Stomp {
    pub const armor_reduction_pct: usize = 0x12C8; // int32
    pub const m_flArmorReduction: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Special_Bonus_20_Bash {
    pub const value: usize = 0x12C8; // float32
    pub const value2: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Special_Bonus_Agility {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Agility_And_Intelligence {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_All_Stats {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Armor {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Attack_Base_Damage {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Attack_Damage {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Attack_Range {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Attack_Speed {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Attributes {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Base_Attack_Rate {
    pub const value: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Special_Bonus_Cast_Range {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Cast_Speed {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Cleave {
    pub const value: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Special_Bonus_Cooldown_Reduction {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Corruption {
    pub const value: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Special_Bonus_Corruption_Debuff {
    pub const value: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Special_Bonus_Crit {
    pub const crit_chance: usize = 0x12C8; // float32
    pub const crit_multiplier: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Special_Bonus_Day_Vision {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Evasion {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Exp_Boost {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Gold {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Gold_Income {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_HP {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_HP_Regen {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Haste {
    pub const value: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Special_Bonus_Intelligence {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Lifesteal {
    pub const value: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Special_Bonus_MP {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_MP_Regen {
    pub const value: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Special_Bonus_MP_Regen_Amp {
    pub const value: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Special_Bonus_Magic_Resistance {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Mana_Break {
    pub const value: usize = 0x12C8; // int32
    pub const burn_illusions_ranged: usize = 0x12CC; // int32
    pub const burn_illusions_melee: usize = 0x12D0; // int32
    pub const damage_per_burn: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Special_Bonus_Manaloss_Reduction {
    pub const value: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Special_Bonus_Movement_Speed {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Movement_Speed_Percentage {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Night_Vision {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Reincarnation {
    pub const value: usize = 0x12C8; // float32
    pub const reincarnate_time: usize = 0x12CC; // float32
    pub const m_fLastReincarnationTime: usize = 0x12D0; // GameTime_t
}

pub mod CDOTA_Modifier_Special_Bonus_Respawn_Reduction {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Spell_Amplify {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Spell_Block {
    pub const block_cooldown: usize = 0x12C8; // float32
    pub const m_CooldownTimer: usize = 0x12D0; // CountdownTimer
}

pub mod CDOTA_Modifier_Special_Bonus_Spell_Lifesteal {
    pub const value: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Special_Bonus_Status_Resistance {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Strength {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Unique_Beastmaster_5 {
    pub const bonus_ms: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Unique_Beastmaster_6 {
    pub const bonus_hp: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Bonus_Vision {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Special_Gyrocopter_Call_Down_Land_Mines {
    pub const radius: usize = 0x12C8; // int32
    pub const proximity_threshold: usize = 0x12CC; // float32
    pub const damage: usize = 0x12D0; // float32
    pub const activation_delay: usize = 0x12D4; // float32
    pub const bActivated: usize = 0x12D8; // bool
    pub const m_Timer: usize = 0x12E0; // CountdownTimer
}

pub mod CDOTA_Modifier_Special_Mars_Spear_Burning_Trail_Burn {
    pub const shard_dps: usize = 0x12C8; // int32
    pub const shard_move_slow_pct: usize = 0x12CC; // int32
    pub const shard_interval: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Special_Mars_Spear_Burning_Trail_Thinker {
    pub const m_vPathDir: usize = 0x12C8; // Vector
    pub const shard_trail_radius: usize = 0x12D4; // int32
    pub const shard_debuff_linger_duration: usize = 0x12D8; // float32
    pub const shard_interval: usize = 0x12DC; // float32
}

pub mod CDOTA_Modifier_Spectre_Arcana_Kill_Effect {
    pub const m_nCasterFXIndex: usize = 0x12C8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Spectre_Desolate {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const radius: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Spectre_Desolate_Blind {
    pub const blind_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Spectre_Dispersion {
    pub const damage_reflection_pct: usize = 0x12C8; // int32
    pub const min_radius: usize = 0x12CC; // int32
    pub const max_radius: usize = 0x12D0; // int32
    pub const activation_bonus_pct: usize = 0x12D4; // int32
    pub const m_flLastDispersionPulseEffectTime: usize = 0x12D8; // GameTime_t
    pub const m_fDamageCounter: usize = 0x12DC; // float32
    pub const m_fLastTime: usize = 0x12E0; // float32
}

pub mod CDOTA_Modifier_Spectre_Haunt {
    pub const m_fStartAttackTime: usize = 0x12C8; // GameTime_t
    pub const hTarget: usize = 0x12CC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Spectre_SpectralDagger {
    pub const path_radius: usize = 0x12C8; // int32
    pub const hero_path_duration: usize = 0x12CC; // float32
    pub const m_vLastPosition: usize = 0x12D0; // Vector
}

pub mod CDOTA_Modifier_Spectre_SpectralDaggerInPath {
    pub const bonus_movespeed: usize = 0x12C8; // int32
    pub const m_hTrackingTarget: usize = 0x12CC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Spectre_SpectralDaggerPath {
    pub const m_hUnitsInPath: usize = 0x12E0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const path_radius: usize = 0x12F8; // int32
    pub const vision_radius: usize = 0x12FC; // int32
    pub const dagger_radius: usize = 0x1300; // int32
    pub const buff_persistence: usize = 0x1304; // float32
    pub const dagger_grace_period: usize = 0x1308; // float32
    pub const dagger_path_duration: usize = 0x130C; // float32
}

pub mod CDOTA_Modifier_Spell_Prism {
    pub const bonus_all_stats: usize = 0x12C8; // int32
    pub const bonus_cooldown: usize = 0x12CC; // int32
    pub const mana_regen: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Spider_Legs {
    pub const bonus_movement_speed: usize = 0x12C8; // int32
    pub const turn_rate: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_SpiritBreaker_Bulldoze {
    pub const movement_speed: usize = 0x12C8; // int32
    pub const status_resistance: usize = 0x12CC; // int32
    pub const damage_barrier: usize = 0x12D0; // float32
    pub const m_flCurrentBarrier: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_SpiritBreaker_ChargeOfDarkness {
    pub const movement_speed: usize = 0x12C8; // int32
    pub const m_hTarget: usize = 0x12CC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_SpiritBreaker_ChargeOfDarknessTarget {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_iChargeTargetEntityIndex: usize = 0x12CC; // CEntityIndex
}

pub mod CDOTA_Modifier_SpiritBreaker_ChargeOfDarknessVision {
    pub const m_nFXIndex: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_SpiritBreaker_EmpoweringHaste {
    pub const bonus_movespeed_pct_self: usize = 0x12C8; // int32
    pub const bonus_movespeed_pct_allies: usize = 0x12CC; // int32
    pub const bonus_movespeed_pct_extra: usize = 0x12D0; // int32
    pub const duration: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_SpiritBreaker_EmpoweringHasteAura {
    pub const aura_radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_SpiritBreaker_GreaterBash {
    pub const chance_pct: usize = 0x12C8; // int32
    pub const damage: usize = 0x12CC; // float32
    pub const movespeed_duration: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_SpiritBreaker_GreaterBash_Speed {
    pub const bonus_movespeed_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_SpiritBreaker_NetherStrike {
    pub const damage: usize = 0x12C8; // int32
    pub const m_bStrikeLanded: usize = 0x12CC; // bool
    pub const m_vCastLocation: usize = 0x12D0; // Vector
    pub const m_hTarget: usize = 0x12DC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_SpiritBreaker_NetherStrike_Vision {
    pub const invis_threshold_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_SpiritBreaker_PlanarPocketAura {
    pub const radius: usize = 0x12C8; // int32
    pub const magic_resistance: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_SpiritBreaker_SpecialAttack {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Spirit_Bear_Attack_Damage {
    pub const bear_bat: usize = 0x12C8; // float32
    pub const bear_magic_resistance: usize = 0x12CC; // int32
    pub const bonus_bear_hp: usize = 0x12D0; // int32
    pub const bear_movespeed: usize = 0x12D4; // int32
    pub const bonus_bear_armor: usize = 0x12D8; // int32
    pub const hp_gain_per_druid_level: usize = 0x12DC; // int32
    pub const damage_gain_per_druid_level: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_Spirit_Gem {
    pub const m_flOriginalModelScale: usize = 0x12C8; // float32
    pub const m_nHealthBarOffsetOverride: usize = 0x12CC; // int32
    pub const m_nFXStackIndex: usize = 0x12D0; // ParticleIndex_t
    pub const m_fScaleGainPerGem: usize = 0x12D4; // float32
    pub const m_nLevel: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Spring2021_DefusalBomb {
    pub const BOMB_DURATION: usize = 0x12C8; // float32
    pub const BOMB_FAST_TICK_START: usize = 0x12CC; // float32
    pub const BOMB_DEFAULT_TICK: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Spring2021_TowerDeathRing_Thinker {
    pub const m_fLastRadius: usize = 0x12C8; // float32
    pub const m_fCurRadius: usize = 0x12CC; // float32
    pub const m_fLastThink: usize = 0x12D0; // GameTime_t
    pub const m_bWindingUp: usize = 0x12D4; // bool
    pub const m_ViewerTimer: usize = 0x12D8; // CountdownTimer
    pub const m_nFXIndex: usize = 0x12F0; // ParticleIndex_t
    pub const m_EntitiesHit: usize = 0x12F8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const speed: usize = 0x1310; // int32
    pub const radius: usize = 0x1314; // int32
    pub const damage: usize = 0x1318; // int32
    pub const windup_time: usize = 0x131C; // float32
}

pub mod CDOTA_Modifier_StaffOfWizardry {
    pub const bonus_intellect: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_StormSpirit_ElectricVortex_Pull {
    pub const m_vDestination: usize = 0x12C8; // Vector
    pub const electric_vortex_pull_distance: usize = 0x12D4; // int32
    pub const electric_vortex_pull_tether_range: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_StormSpirit_ElectricVortex_SelfSlow {
    pub const electric_vortex_self_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_StormSpirit_Electric_Rave {
    pub const overload_aoe: usize = 0x12C8; // int32
    pub const overload_damage: usize = 0x12CC; // int32
    pub const shard_attack_speed_bonus: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_StormSpirit_Overload {
    pub const overload_aoe: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_StormSpirit_OverloadSlow {
    pub const overload_move_slow: usize = 0x12C8; // int32
    pub const overload_attack_slow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_StormSpirit_StaticRemnantThinker {
    pub const static_remnant_radius: usize = 0x12C8; // int32
    pub const static_remnant_damage_radius: usize = 0x12CC; // int32
    pub const static_remnant_damage: usize = 0x12D0; // int32
    pub const static_remnant_delay: usize = 0x12D4; // float32
    pub const m_iSearchRadius: usize = 0x12D8; // int32
    pub const m_iDamageRadius: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Stormcrafter_Slow {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Sven_GodsStrength {
    pub const gods_strength_damage: usize = 0x12C8; // int32
    pub const bonus_slow_resistance: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Sven_GreatCleave {
    pub const cleave_starting_width: usize = 0x12C8; // int32
    pub const cleave_ending_width: usize = 0x12CC; // int32
    pub const cleave_distance: usize = 0x12D0; // int32
    pub const great_cleave_damage: usize = 0x12D4; // int32
    pub const strength_bonus: usize = 0x12D8; // int32
    pub const m_nLastCleaveRecord: usize = 0x12DC; // int16
    pub const m_nLastCleaveKills: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_Sven_Stormbolt_Hide {
    pub const m_nHandle: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Sven_Warcry {
    pub const movespeed: usize = 0x12C8; // int32
    pub const bonus_armor: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Sven_Warcry_Aura {
    pub const shard_radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Sven_Warcry_Passive {
    pub const shard_passive_armor: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_TangoHeal {
    pub const health_regen: usize = 0x12C8; // float32
    pub const superheal: usize = 0x12CC; // bool
    pub const m_fHealingDone: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Techies_LandMine {
    pub const radius: usize = 0x12D8; // int32
    pub const proximity_threshold: usize = 0x12DC; // float32
    pub const damage: usize = 0x12E0; // float32
    pub const activation_delay: usize = 0x12E4; // float32
    pub const outer_damage: usize = 0x12E8; // float32
    pub const min_distance: usize = 0x12EC; // int32
    pub const bActivated: usize = 0x12F0; // bool
    pub const m_Timer: usize = 0x12F8; // CountdownTimer
}

pub mod CDOTA_Modifier_Techies_LandMine_Burn {
    pub const mres_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Techies_Minefield_Sign_Scepter {
    pub const aura_radius: usize = 0x12C8; // int32
    pub const minefield_duration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Techies_Minefield_Sign_Scepter_Aura {
    pub const aura_radius: usize = 0x12C8; // float32
    pub const scepter_move_damage: usize = 0x12CC; // float32
    pub const scepter_move_amt: usize = 0x12D0; // float32
    pub const minefield_duration: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Techies_Minefield_Sign_Thinker {
    pub const aura_radius: usize = 0x12C8; // int32
    pub const trigger_radius: usize = 0x12CC; // float32
    pub const minefield_duration: usize = 0x12D0; // float32
    pub const m_bTriggered: usize = 0x12D4; // bool
}

pub mod CDOTA_Modifier_Techies_ReactiveTazer {
    pub const bonus_ms: usize = 0x12D0; // int32
    pub const stun_radius: usize = 0x12D4; // int32
    pub const stun_duration: usize = 0x12D8; // float32
    pub const damage_percent: usize = 0x12DC; // int32
    pub const m_nDamageRemaining: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_Techies_RemoteMine {
    pub const radius: usize = 0x12D8; // int32
    pub const damage: usize = 0x12DC; // int32
    pub const radius_scepter: usize = 0x12E0; // int32
    pub const damage_scepter: usize = 0x12E4; // int32
    pub const vision_radius: usize = 0x12E8; // int32
    pub const vision_duration: usize = 0x12EC; // float32
    pub const model_scale: usize = 0x12F0; // int32
    pub const bExploding: usize = 0x12F4; // bool
}

pub mod CDOTA_Modifier_Techies_StasisTrap {
    pub const activation_radius: usize = 0x12D8; // int32
    pub const stun_radius: usize = 0x12DC; // int32
    pub const stun_duration: usize = 0x12E0; // float32
    pub const explode_delay: usize = 0x12E4; // float32
    pub const activation_time: usize = 0x12E8; // float32
    pub const m_bActivated: usize = 0x12EC; // bool
    pub const m_bTriggered: usize = 0x12ED; // bool
}

pub mod CDOTA_Modifier_Techies_StickyBombThrow {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Techies_StickyBomb_Chase {
    pub const acceleration: usize = 0x12C8; // int32
    pub const m_nTeamNumber: usize = 0x12CC; // int32
    pub const speed: usize = 0x12D0; // float32
    pub const pre_chase_time: usize = 0x12D4; // float32
    pub const m_hAttachTarget: usize = 0x12D8; // CHandle< C_BaseEntity >
    pub const m_vStartPosition: usize = 0x12DC; // Vector
    pub const m_MoveTime: usize = 0x12E8; // CountdownTimer
}

pub mod CDOTA_Modifier_Techies_StickyBomb_Countdown {
    pub const m_vLastLoc: usize = 0x12C8; // Vector
    pub const m_hAttachTarget: usize = 0x12D4; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Techies_StickyBomb_Slow {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Techies_StickyBomb_Slow_Secondary {
    pub const secondary_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Techies_Suicide_RespawnTime {
    pub const m_nSuicideKillCount: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Teleporting {
    pub const m_fChannelTime: usize = 0x12C8; // float32
    pub const m_vStart: usize = 0x12CC; // Vector
    pub const m_vEnd: usize = 0x12D8; // Vector
}

pub mod CDOTA_Modifier_TemplarAssassin_Meld {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const launched_attack: usize = 0x12CC; // bool
    pub const m_hTarget: usize = 0x12D0; // CHandle< C_BaseEntity >
    pub const m_nAttackRecord: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_TemplarAssassin_MeldArmor {
    pub const bonus_armor: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_TemplarAssassin_Meld_Animation {
    pub const launched_attack: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_TemplarAssassin_PsiBlades {
    pub const bonus_attack_range: usize = 0x12C8; // int32
    pub const attack_spill_range: usize = 0x12CC; // float32
    pub const attack_spill_width: usize = 0x12D0; // int32
    pub const attack_spill_pct: usize = 0x12D4; // int32
    pub const attack_spill_penalty: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_TemplarAssassin_PsiBlades_Slow {
    pub const value: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_TemplarAssassin_RefractionAbsorb {
    pub const damage_threshold: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_TemplarAssassin_RefractionDamage {
    pub const m_InFlightAttackRecords: usize = 0x12C8; // CUtlVector< int16 >
    pub const bonus_damage: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_TemplarAssassin_Refraction_Holdout {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const damage_absorb: usize = 0x12CC; // int32
    pub const m_flDamageAbsorbed: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_TemplarAssassin_Trap {
    pub const bonus_vision: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_TemplarAssassin_Trap_Slow {
    pub const movement_speed_min: usize = 0x12C8; // int32
    pub const movement_speed_max: usize = 0x12CC; // int32
    pub const trap_max_charge_duration: usize = 0x12D0; // float32
    pub const min_silence_duration: usize = 0x12D4; // float32
    pub const max_silence_duration: usize = 0x12D8; // float32
    pub const stage: usize = 0x12DC; // float32
    pub const flDamagePerTick: usize = 0x12E0; // float32
}

pub mod CDOTA_Modifier_Tenderizer_Weaken {
    pub const weaken_per_hit: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Terrorblade_Arcana_Kill_Effect {
    pub const m_bFrozen: usize = 0x12C8; // bool
    pub const m_bFlail: usize = 0x12C9; // bool
}

pub mod CDOTA_Modifier_Terrorblade_Demon_Zeal {
    pub const berserk_bonus_attack_speed: usize = 0x12C8; // int32
    pub const berserk_bonus_movement_speed: usize = 0x12CC; // int32
    pub const berserk_bonus_armor: usize = 0x12D0; // int32
    pub const melee_bonus: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Terrorblade_Fear {
    pub const m_vOriginal: usize = 0x12C8; // Vector
}

pub mod CDOTA_Modifier_Terrorblade_Metamorphosis {
    pub const base_attack_time: usize = 0x12C8; // float32
    pub const bonus_range: usize = 0x12CC; // int32
    pub const m_iOriginalAttackCapabilities: usize = 0x12D0; // int32
    pub const bonus_damage: usize = 0x12D4; // int32
    pub const speed_loss: usize = 0x12D8; // int32
    pub const attack_projectile_speed_bonus: usize = 0x12DC; // int32
    pub const m_iszRangedAttackEffect: usize = 0x12E0; // CUtlSymbolLarge
    pub const m_iszOriginalRangedAttackEffect: usize = 0x12E8; // CUtlSymbolLarge
}

pub mod CDOTA_Modifier_Terrorblade_Metamorphosis_Fear_Thinker {
    pub const m_fLastRadius: usize = 0x12C8; // float32
    pub const m_fCurRadius: usize = 0x12CC; // float32
    pub const m_fLastThink: usize = 0x12D0; // GameTime_t
    pub const m_nFXIndex: usize = 0x12D4; // ParticleIndex_t
    pub const m_EntitiesHit: usize = 0x12D8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const scepter_speed: usize = 0x12F0; // int32
    pub const scepter_radius: usize = 0x12F4; // int32
    pub const m_bFirstThink: usize = 0x12F8; // bool
}

pub mod CDOTA_Modifier_Terrorblade_Reflection_Invulnerability {
    pub const hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Terrorblade_Reflection_Slow {
    pub const move_slow: usize = 0x12C8; // int32
    pub const attack_slow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Third_Eye {
    pub const truesight_radius: usize = 0x12C8; // int32
    pub const bonus_vision: usize = 0x12CC; // int32
    pub const bonus_all_stats: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Tidehunter_AnchorSmash {
    pub const damage_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Tidehunter_AnchorSmash_Caster {
    pub const attack_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Tidehunter_Anchor_Unit {
    pub const chain_length: usize = 0x12C8; // int32
    pub const attacks_to_destroy: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Tidehunter_DeadInTheWater {
    pub const max_movement_speed: usize = 0x12C8; // int32
    pub const chain_length: usize = 0x12CC; // int32
    pub const m_hAnchor: usize = 0x12D0; // CHandle< C_BaseEntity >
    pub const m_bDragging: usize = 0x12D4; // bool
}

pub mod CDOTA_Modifier_Tidehunter_Gush {
    pub const movement_speed: usize = 0x12C8; // int32
    pub const negative_armor: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Tidehunter_KrakenShell {
    pub const m_iDamageTaken: usize = 0x12C8; // int32
    pub const damage_reduction: usize = 0x12CC; // int32
    pub const damage_cleanse: usize = 0x12D0; // int32
    pub const bonus_reduction_per_stack: usize = 0x12D4; // int32
    pub const damage_reset_interval: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Tidehunter_SmashAttack {
    pub const attack_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Tinker_Defensive_Matrix {
    pub const damage_absorb: usize = 0x12C8; // int32
    pub const status_resistance: usize = 0x12CC; // int32
    pub const cooldown_reduction: usize = 0x12D0; // int32
    pub const m_bStartedTimer: usize = 0x12D4; // bool
    pub const m_nDamageAbsorbed: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Tinker_LaserBlind {
    pub const miss_rate: usize = 0x12C8; // int32
    pub const max_health_removed: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Tinker_MarchOfTheMachinesThinker {
    pub const collision_radius: usize = 0x12C8; // int32
    pub const radius: usize = 0x12CC; // int32
    pub const distance: usize = 0x12D0; // int32
    pub const distance_scepter: usize = 0x12D4; // int32
    pub const speed: usize = 0x12D8; // int32
    pub const machines_per_sec: usize = 0x12DC; // int32
    pub const m_vDir: usize = 0x12E0; // Vector
}

pub mod CDOTA_Modifier_Tinker_Warp_Grenade {
    pub const range_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Tiny_Avalanche {
    pub const m_pHeroesHit: usize = 0x12C8; // CUtlVector< C_DOTA_BaseNPC* >
    pub const radius: usize = 0x12E0; // int32
    pub const total_duration: usize = 0x12E4; // float32
    pub const stun_duration: usize = 0x12E8; // float32
    pub const tick_count: usize = 0x12EC; // int32
    pub const toss_multiplier: usize = 0x12F0; // float32
    pub const m_damage: usize = 0x12F4; // int32
    pub const m_nTicks: usize = 0x12F8; // int32
}

pub mod CDOTA_Modifier_Tiny_CraggyExterior {
    pub const stun_chance: usize = 0x12C8; // int32
    pub const stun_duration: usize = 0x12CC; // float32
    pub const damage: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Tiny_Grow {
    pub const bonus_armor: usize = 0x12C8; // int32
    pub const attack_speed_reduction: usize = 0x12CC; // int32
    pub const bonus_damage: usize = 0x12D0; // int32
    pub const tree_bonus_damage_pct: usize = 0x12D4; // float32
    pub const slow_resistance: usize = 0x12D8; // float32
    pub const m_nFXIndex: usize = 0x12DC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Tiny_Toss {
    pub const radius: usize = 0x12C8; // int32
    pub const toss_damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Tiny_TossTree_Bonus {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Tiny_TossTree_Slow {
    pub const movement_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Tiny_Tree_Channel {
    pub const tree_grab_radius: usize = 0x12C8; // int32
    pub const splash_radius: usize = 0x12CC; // int32
    pub const speed: usize = 0x12D0; // int32
    pub const interval: usize = 0x12D4; // float32
    pub const m_vPos: usize = 0x12D8; // Vector
}

pub mod CDOTA_Modifier_Tiny_Tree_Channel_Bonus {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Tiny_Tree_Grab {
    pub const attack_count: usize = 0x12C8; // int32
    pub const bonus_damage: usize = 0x12CC; // int32
    pub const bonus_damage_buildings: usize = 0x12D0; // int32
    pub const stun_duration: usize = 0x12D4; // float32
    pub const attack_range: usize = 0x12D8; // int32
    pub const m_nOriginalAttackCapability: usize = 0x12DC; // int32
    pub const bat_increase: usize = 0x12E0; // float32
    pub const speed_reduction: usize = 0x12E4; // int32
    pub const m_nBonusAttackRange: usize = 0x12E8; // int32
    pub const m_hStolenTree: usize = 0x12EC; // CHandle< C_BaseEntity >
    pub const m_nFXIndex: usize = 0x12F0; // ParticleIndex_t
    pub const m_nFXAttackIndex: usize = 0x12F4; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Tornado_Tempest {
    pub const near_radius: usize = 0x12C8; // int32
    pub const near_damage: usize = 0x12CC; // int32
    pub const far_radius: usize = 0x12D0; // int32
    pub const far_damage: usize = 0x12D4; // int32
    pub const tick_rate: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Tornado_Tempest_Debuff {
    pub const movespeed_slow: usize = 0x12C8; // int32
    pub const attackspeed_slow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Tower_Armor_Bonus {
    pub const bonus_armor: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Tower_Aura {
    pub const bonus_armor: usize = 0x12C8; // int32
    pub const m_bOverrideArmor: usize = 0x12CC; // bool
    pub const hp_regen: usize = 0x12D0; // float32
    pub const m_bOverrideRegen: usize = 0x12D4; // bool
    pub const m_nRadius: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Tower_Aura_Bonus {
    pub const bonus_armor: usize = 0x12C8; // int32
    pub const hp_regen: usize = 0x12CC; // float32
    pub const m_bOverrideArmor: usize = 0x12D0; // bool
    pub const m_bOverrideRegen: usize = 0x12D1; // bool
    pub const m_bSentFirstRefresh: usize = 0x12D2; // bool
}

pub mod CDOTA_Modifier_Treant_Bonus {
    pub const treant_hp_bonus: usize = 0x12C8; // int32
    pub const treant_damage_bonus: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Treant_Large_Bonus {
    pub const treant_large_hp_bonus: usize = 0x12C8; // int32
    pub const treant_large_damage_bonus: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Treant_LeechSeed {
    pub const damage_interval: usize = 0x12C8; // float32
    pub const leech_damage: usize = 0x12CC; // int32
    pub const movement_slow: usize = 0x12D0; // int32
    pub const radius: usize = 0x12D4; // int32
    pub const projectile_speed: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Treant_LeechSeed_Slow {
    pub const movement_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Treant_Lifebomb_Explode {
    pub const flStartTime: usize = 0x12C8; // GameTime_t
}

pub mod CDOTA_Modifier_Treant_LivingArmor {
    pub const heal_per_second: usize = 0x12C8; // int32
    pub const bonus_armor: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Treant_NaturesGrasp_Creation_Thinker {
    pub const creation_interval: usize = 0x12C8; // float32
    pub const vines_duration: usize = 0x12CC; // float32
    pub const vine_spawn_interval: usize = 0x12D0; // int32
    pub const m_vInitialLoc: usize = 0x12D4; // Vector
    pub const m_vFinalLoc: usize = 0x12E0; // Vector
    pub const m_iVineCount: usize = 0x12EC; // int32
    pub const m_iVinesIndex: usize = 0x12F0; // int32
    pub const m_bFoundTree: usize = 0x12F4; // bool
}

pub mod CDOTA_Modifier_Treant_NaturesGrasp_Damage {
    pub const movement_slow: usize = 0x12C8; // int32
    pub const damage_per_second: usize = 0x12CC; // int32
    pub const tick_interval: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Treant_NaturesGrasp_Latch_Thinker {
    pub const latch_range: usize = 0x12C8; // int32
    pub const latch_vision: usize = 0x12CC; // int32
    pub const initial_latch_delay: usize = 0x12D0; // float32
    pub const m_bBonus: usize = 0x12DC; // bool
}

pub mod CDOTA_Modifier_Treant_NaturesGuise {
    pub const m_RevealTimer: usize = 0x12C8; // CountdownTimer
    pub const m_InvisAttackTimer: usize = 0x12E0; // CountdownTimer
    pub const m_CooldownTimer: usize = 0x12F8; // CountdownTimer
}

pub mod CDOTA_Modifier_Treant_NaturesGuise_Root {
    pub const m_flEntangleDamage: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Treant_NaturesGuise_Tree_Walking {
    pub const movement_bonus: usize = 0x12C8; // int32
    pub const regen_amp: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Treant_Overgrowth {
    pub const damage: usize = 0x12C8; // float32
    pub const mute: usize = 0x12CC; // int32
    pub const bPurgable: usize = 0x12D0; // bool
}

pub mod CDOTA_Modifier_TrollWarlord_BattleTrance {
    pub const movement_speed: usize = 0x12C8; // int32
    pub const attack_speed: usize = 0x12CC; // int32
    pub const lifesteal: usize = 0x12D0; // int32
    pub const m_hTarget: usize = 0x12D4; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_TrollWarlord_BerserkersRage {
    pub const bonus_move_speed: usize = 0x12C8; // int32
    pub const bonus_range: usize = 0x12CC; // int32
    pub const bonus_hp: usize = 0x12D0; // int32
    pub const bonus_armor: usize = 0x12D4; // int32
    pub const ensnare_chance: usize = 0x12D8; // int32
    pub const ensnare_duration: usize = 0x12DC; // float32
    pub const base_attack_time: usize = 0x12E0; // float32
    pub const m_iOriginalAttackCapabilities: usize = 0x12E4; // int32
}

pub mod CDOTA_Modifier_TrollWarlord_BerserkersRage_Maim {
    pub const maim_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_TrollWarlord_Fervor {
    pub const attack_speed: usize = 0x12C8; // int32
    pub const max_stacks: usize = 0x12CC; // int32
    pub const extra_attack_chance_per_stack: usize = 0x12D0; // int32
    pub const base_chance: usize = 0x12D4; // int32
    pub const m_hUnit: usize = 0x12D8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_TrollWarlord_WhirlingAxes_Blind {
    pub const blind_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_TrollWarlord_WhirlingAxes_Slow {
    pub const movement_speed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_TrollWarlord_Whirling_Axes_Melee {
    pub const damage: usize = 0x12C8; // int32
    pub const hit_radius: usize = 0x12CC; // int32
    pub const axe_movement_speed: usize = 0x12D0; // int32
    pub const whirl_duration: usize = 0x12D4; // float32
    pub const max_range: usize = 0x12D8; // float32
    pub const blind_duration: usize = 0x12DC; // float32
    pub const m_flRotation: usize = 0x12E0; // float32
    pub const m_flAxeRadius: usize = 0x12E4; // float32
    pub const m_flDieTime: usize = 0x12E8; // GameTime_t
    pub const m_bPiercesMagicImmunity: usize = 0x12EC; // float32
    pub const m_nSwapIndex: usize = 0x12F0; // int32
    pub const m_bReturning: usize = 0x12F4; // bool
    pub const m_nAxeFXIndex: usize = 0x12F8; // ParticleIndex_t[2]
    pub const m_nHeroesHitForRelic: usize = 0x1300; // int32
    pub const hitEntities: usize = 0x1308; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_hAxes: usize = 0x1320; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Troll_Warlord_Rampage {
    pub const attack_speed: usize = 0x12C8; // int32
    pub const status_resistance: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Truesight_Aura {
    pub const m_nAuraRadius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Tusk_FrozenSigil {
    pub const aghs_shard_move_slow: usize = 0x12C8; // int32
    pub const aghs_shard_damage: usize = 0x12CC; // int32
    pub const attack_slow: usize = 0x12D0; // int32
    pub const turn_rate_slow: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Tusk_FrozenSigil_Aura {
    pub const sigil_radius: usize = 0x12C8; // int32
    pub const m_iIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Tusk_Tag_Team {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const slow_duration: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Tusk_Tag_Team_Attack_Slow {
    pub const attack_speed_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Tusk_Tag_Team_Attack_Slow_Aura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Tusk_Tag_Team_Aura {
    pub const m_iIndex: usize = 0x12C8; // ParticleIndex_t
    pub const radius: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Tusk_Tag_Team_Slow {
    pub const movement_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Tusk_WalrusKick_AirTime {
    pub const hp_threshold: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Tusk_WalrusKick_Slow {
    pub const move_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Tusk_WalrusPunch {
    pub const crit_multiplier: usize = 0x12C8; // int32
    pub const air_time: usize = 0x12CC; // float32
    pub const slow_duration: usize = 0x12D0; // float32
    pub const m_iszRangedAttackEffect: usize = 0x12D8; // CUtlSymbolLarge
    pub const m_hTarget: usize = 0x12E0; // CHandle< C_BaseEntity >
    pub const m_bWalrusPunch: usize = 0x12E4; // bool
    pub const m_InFlightAttackRecords: usize = 0x12E8; // CUtlVector< int16 >
    pub const m_nFXIndex: usize = 0x1300; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Tusk_WalrusPunch_AirTime {
    pub const hp_threshold: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Tusk_WalrusPunch_Slow {
    pub const move_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Tutorial_ForceAnimation {
    pub const m_nActivity: usize = 0x12C8; // int32
    pub const m_bLoop: usize = 0x12CC; // bool
    pub const m_bFacePlayer: usize = 0x12CD; // bool
}

pub mod CDOTA_Modifier_Tutorial_HideNPC {
    pub const m_bRemoveOnDeath: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_Tutorial_LastHitHelper {
    pub const m_bTargetFriendlies: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_Tutorial_LastHittable {
    pub const m_bIsFriend: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_Tutorial_SpeechBubble {
    pub const m_bFacePlayer: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_Twin_Gate_FX {
    pub const m_nPortalAmbientFX: usize = 0x12C8; // ParticleIndex_t
    pub const m_fPortalRadius: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Twin_Gate_Warp_Channel {
    pub const m_nfxTargetTp: usize = 0x12C8; // ParticleIndex_t
    pub const m_nfxTargetTp2: usize = 0x12CC; // ParticleIndex_t
    pub const m_nfxAmbientFx: usize = 0x12D0; // ParticleIndex_t
    pub const m_nfxPortal1: usize = 0x12D4; // ParticleIndex_t
    pub const m_nfxPortal2: usize = 0x12D8; // ParticleIndex_t
    pub const animation_rate: usize = 0x12DC; // float32
    pub const stop_distance: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_Underlord_Fear {
    pub const m_vOriginal: usize = 0x12C8; // Vector
}

pub mod CDOTA_Modifier_Underlord_Portal_Buff {
    pub const damage_reduction: usize = 0x12C8; // int32
    pub const bonus_ms: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Underlord_Portal_FX {
    pub const m_nPortalAmbientFX: usize = 0x12C8; // ParticleIndex_t
    pub const m_fPortalRadius: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Underlord_Portal_Warp_Channel {
    pub const m_nfxTargetTp: usize = 0x12C8; // ParticleIndex_t
    pub const m_nfxTargetTp2: usize = 0x12CC; // ParticleIndex_t
    pub const m_nfxAmbientFx: usize = 0x12D0; // ParticleIndex_t
    pub const m_nfxPortal1: usize = 0x12D4; // ParticleIndex_t
    pub const m_nfxPortal2: usize = 0x12D8; // ParticleIndex_t
    pub const animation_rate: usize = 0x12DC; // float32
    pub const stop_distance: usize = 0x12E0; // int32
}

pub mod CDOTA_Modifier_Undying_Decay_Buff {
    pub const str_steal: usize = 0x12C8; // int32
    pub const str_scale_up: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Undying_Decay_Debuff {
    pub const str_steal: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Undying_Decay_Shard {
    pub const shard_buff_duration: usize = 0x12C8; // float32
    pub const m_vecStackExpirationTimes: usize = 0x12D0; // CUtlVector< GameTime_t >
}

pub mod CDOTA_Modifier_Undying_FleshGolem {
    pub const str_percentage: usize = 0x12C8; // int32
    pub const movement_bonus: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Undying_FleshGolem_PlagueAura {
    pub const slow: usize = 0x12C8; // int32
    pub const damage: usize = 0x12CC; // int32
    pub const tick_rate: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Undying_FleshGolem_Slow {
    pub const damage_amp: usize = 0x12C8; // int32
    pub const slow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Undying_Tombstone_Bunker {
    pub const bunker_heal_pct: usize = 0x12C8; // float32
    pub const tombstone_grab_radius: usize = 0x12CC; // float32
    pub const tombstone_stun_penalty: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Undying_Tombstone_HP {
    pub const tombstone_health: usize = 0x12C8; // int32
    pub const zombie_damage_interval: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Undying_Tombstone_Zombie_Aura {
    pub const radius: usize = 0x12C8; // int32
    pub const zombie_interval: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Undying_Tombstone_Zombie_Deathlust {
    pub const bonus_move_speed: usize = 0x12C8; // int32
    pub const bonus_attack_speed: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Undying_Tombstone_Zombie_Deathstrike {
    pub const radius: usize = 0x12C8; // int32
    pub const health_threshold_pct: usize = 0x12CC; // float32
    pub const duration: usize = 0x12D0; // float32
    pub const m_hTombstoneSourceAbility: usize = 0x12D4; // CHandle< C_DOTABaseAbility >
    pub const m_hChaseUnit: usize = 0x12D8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Undying_Tombstone_Zombie_Deathstrike_Slow {
    pub const slow: usize = 0x12C8; // int32
    pub const m_flDecrementTime: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Undying_Tombstone_Zombie_Deathstrike_Slow_Counter {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_UpgradedBarricade {
    pub const armor_bonus_per_upgrade: usize = 0x12C8; // int32
    pub const hp_bonus_per_upgrade: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_UpgradedMortar {
    pub const bonus_per_upgrade: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_UpheavalUrn_Reincarnation {
    pub const reincarnate_time: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_UrnUpheaval {
    pub const m_iSlow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Ursa_Earthshock {
    pub const movement_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Ursa_Enrage {
    pub const damage_reduction: usize = 0x12C8; // int32
    pub const status_resistance: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Ursa_Fury_Swipes {
    pub const damage_per_stack: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Ursa_Fury_Swipes_Damage_Increase {
    pub const damage_per_stack: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Ursa_Overpower {
    pub const attack_speed_bonus_pct: usize = 0x12C8; // int32
    pub const slow_resist: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_VengefulSpirit_Command_Aura {
    pub const aura_radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_VengefulSpirit_Command_Aura_Effect {
    pub const bonus_base_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_VengefulSpirit_Command_Aura_Illusion {
    pub const hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_VengefulSpirit_Command_Negative_Aura {
    pub const aura_radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_VengefulSpirit_Command_Negative_Aura_Effect {
    pub const bonus_damage_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_VengefulSpirit_Hybrid_Special {
    pub const scepter_illusion_ms_bonus_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_VengefulSpirit_Nether_Swap_DamageReduction {
    pub const damage_reduction: usize = 0x12C8; // float32
    pub const nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_VengefulSpirit_WaveOfTerror {
    pub const armor_reduction: usize = 0x12C8; // int32
    pub const attack_reduction: usize = 0x12CC; // int32
    pub const damage_reduction_pct: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_VengefulSpirit_WaveOfTerror_Buff {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const bonus_armor: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Venom_Gland_Debuff {
    pub const damage: usize = 0x12C8; // int32
    pub const degen: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Venomancer_Latent_Poison {
    pub const duration_damage: usize = 0x12C8; // int32
    pub const movement_slow: usize = 0x12CC; // int32
    pub const explosion_damage: usize = 0x12D0; // int32
    pub const explosion_stun_duration: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Venomancer_NoxiousPlaguePrimary {
    pub const health_damage: usize = 0x12C8; // float32
    pub const debuff_radius: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Venomancer_NoxiousPlagueSecondary {
    pub const health_damage: usize = 0x12C8; // float32
    pub const attack_slow: usize = 0x12CC; // float32
    pub const movement_slow_max: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Venomancer_NoxiousPlague_Slow {
    pub const attack_slow: usize = 0x12C8; // float32
    pub const movement_slow_max: usize = 0x12CC; // float32
    pub const debuff_radius: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Venomancer_PoisonNova {
    pub const damage: usize = 0x12C8; // float32
    pub const magic_resist: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Venomancer_PoisonStingBase {
    pub const m_iDamage: usize = 0x12C8; // int32
    pub const movement_speed: usize = 0x12CC; // int32
    pub const m_nTicksRemaining: usize = 0x12D0; // int32
    pub const m_flAttackDamage: usize = 0x12D4; // float32
    pub const hp_regen_reduction: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Venomancer_PoisonStingWard {
    pub const m_iDamage: usize = 0x12C8; // int32
    pub const movement_speed: usize = 0x12CC; // int32
    pub const hp_regen_reduction: usize = 0x12D0; // int32
    pub const m_nTicksRemaining: usize = 0x12D4; // int32
    pub const m_flAttackDamage: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Venomancer_PoisonSting_Applier {
    pub const duration: usize = 0x12C8; // float32
    pub const damage: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Venomancer_VenomousGale {
    pub const movement_slow: usize = 0x12C8; // int32
    pub const tick_damage: usize = 0x12CC; // int32
    pub const tick_interval: usize = 0x12D0; // float32
    pub const duration: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Viper_BecomeUniversal {
    pub const bonus_magic_resistance: usize = 0x12C8; // int32
    pub const is_universal: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Viper_CorrosiveSkin {
    pub const bonus_magic_resistance: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Viper_CorrosiveSkin_Slow {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
    pub const damage: usize = 0x12CC; // int32
    pub const effect_multiplier_distance: usize = 0x12D0; // int32
    pub const effect_multiplier: usize = 0x12D4; // int32
    pub const m_bEffectMultiplied: usize = 0x12D8; // bool
}

pub mod CDOTA_Modifier_Viper_Nethertoxin {
    pub const min_damage: usize = 0x12C8; // int32
    pub const max_damage: usize = 0x12CC; // int32
    pub const max_duration: usize = 0x12D0; // float32
    pub const m_flDamageInterval: usize = 0x12D4; // float32
    pub const m_flTimeIncrement: usize = 0x12D8; // float32
}

pub mod CDOTA_Modifier_Viper_Nethertoxin_Thinker {
    pub const radius: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Viper_Nose_Dive {
    pub const m_vTargetPos: usize = 0x12C8; // Vector
    pub const m_vStart: usize = 0x12D4; // Vector
    pub const m_flZDelta: usize = 0x12E0; // float32
    pub const radius: usize = 0x12E4; // int32
    pub const corrosive_radius: usize = 0x12E8; // int32
    pub const effect_duration: usize = 0x12EC; // float32
    pub const dive_speed: usize = 0x12F0; // int32
    pub const start_height: usize = 0x12F4; // float32
}

pub mod CDOTA_Modifier_Viper_Nose_Dive_Effect {
    pub const movespeed_slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Viper_PoisonAttack {
    pub const duration: usize = 0x12C8; // float32
    pub const m_InFlightAttackRecords: usize = 0x12D0; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Viper_PoisonAttack_Slow {
    pub const damage: usize = 0x12C8; // float32
    pub const shard_armor_reduction: usize = 0x12CC; // int32
    pub const shard_building_dmg_pct: usize = 0x12D0; // int32
    pub const movement_speed: usize = 0x12D4; // int32
    pub const magic_resistance: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Viper_ViperStrike_Slow {
    pub const bonus_movement_speed: usize = 0x12C8; // int32
    pub const bonus_attack_speed: usize = 0x12CC; // int32
    pub const damage: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Visage_GraveChill_Buff {
    pub const movespeed_bonus: usize = 0x12C8; // int32
    pub const attackspeed_bonus: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Visage_GraveChill_Debuff {
    pub const movespeed_bonus: usize = 0x12C8; // int32
    pub const attackspeed_bonus: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Visage_GravekeepersCloak {
    pub const damage_reduction: usize = 0x12C8; // int32
    pub const max_layers: usize = 0x12CC; // int32
    pub const minimum_damage: usize = 0x12D0; // float32
    pub const recovery_time: usize = 0x12D4; // int32
    pub const radius: usize = 0x12D8; // int32
    pub const max_damage_reduction: usize = 0x12DC; // int32
    pub const m_nFXIndex: usize = 0x12E0; // ParticleIndex_t[4]
    pub const m_nFXIndexB: usize = 0x12F0; // ParticleIndex_t
    pub const vStacks: usize = 0x12F8; // CUtlVector< CDOTA_Modifier_Visage_GravekeepersCloak_Stack* >
}

pub mod CDOTA_Modifier_Visage_GravekeepersCloak_Secondary {
    pub const damage_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Visage_GravekeepersCloak_Stack {
    pub const m_bIsRecovering: usize = 0x12C8; // bool
    pub const m_nFXIndex: usize = 0x12CC; // int32[4]
}

pub mod CDOTA_Modifier_Visage_Scepter {
    pub const m_flReadyTime: usize = 0x12C8; // GameTime_t
}

pub mod CDOTA_Modifier_Visage_Silent_As_The_Grave {
    pub const m_bHasMadeAttack: usize = 0x12D8; // bool
}

pub mod CDOTA_Modifier_Visage_Silent_As_The_Grave_Bonus {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Visage_SoulAssumption {
    pub const radius: usize = 0x12C8; // int32
    pub const stack_limit: usize = 0x12CC; // int32
    pub const damage_limit: usize = 0x12D0; // int32
    pub const stack_duration: usize = 0x12D4; // float32
    pub const damage_min: usize = 0x12D8; // float32
    pub const damage_max: usize = 0x12DC; // float32
    pub const m_nFxIndexA: usize = 0x12E0; // ParticleIndex_t
    pub const m_nFxIndexB: usize = 0x12E4; // ParticleIndex_t
    pub const iCur_stack: usize = 0x12E8; // int32
    pub const m_fTotalDamage: usize = 0x1308; // float32
}

pub mod CDOTA_Modifier_Visage_SummonFamiliars_DamageCharge {
    pub const familiar_speed: usize = 0x12C8; // int32
    pub const m_nFXIndex: usize = 0x12CC; // int32
    pub const m_nFXIndexB: usize = 0x12D0; // int32
    pub const m_ctTimer: usize = 0x12D8; // CountdownTimer
}

pub mod CDOTA_Modifier_Visage_SummonFamiliars_StoneForm_Buff {
    pub const hp_regen: usize = 0x12C8; // float32
    pub const stun_radius: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Visage_SummonFamiliars_StoneForm_Thinker {
    pub const stun_radius: usize = 0x12C8; // int32
    pub const stun_damage: usize = 0x12CC; // int32
    pub const stun_delay: usize = 0x12D0; // float32
    pub const stun_duration: usize = 0x12D4; // float32
    pub const stone_duration: usize = 0x12D8; // float32
    pub const m_iIntervalCount: usize = 0x12DC; // int32
}

pub mod CDOTA_Modifier_Visage_SummonFamiliars_StoneForm_Timer {
    pub const stone_duration: usize = 0x12C8; // float32
    pub const m_fStartTime: usize = 0x12CC; // GameTime_t
}

pub mod CDOTA_Modifier_VoidSpirit_AetherRemnantThinker {
    pub const radius: usize = 0x12C8; // int32
    pub const m_vFacingDir: usize = 0x12CC; // Vector
    pub const m_hRemnant: usize = 0x12D8; // CHandle< C_BaseEntity >
    pub const m_bTriggered: usize = 0x12DC; // bool
    pub const m_flCurThink: usize = 0x12E0; // float32
}

pub mod CDOTA_Modifier_VoidSpirit_AetherRemnantUnit_Truesight {
    pub const m_nRadius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_VoidSpirit_AetherRemnant_Pull {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const m_nPlayerID: usize = 0x12CC; // PlayerID_t
    pub const think_interval: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_VoidSpirit_AetherRemnant_Unit {
    pub const m_vFacingDir: usize = 0x12C8; // Vector
    pub const m_hWatchPathThinkers: usize = 0x12D8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_nBeamFXIndex: usize = 0x12F0; // ParticleIndex_t
    pub const m_nViewerIDs: usize = 0x12F8; // CUtlVector< int32 >
    pub const remnant_watch_distance: usize = 0x1310; // int32
    pub const remnant_watch_radius: usize = 0x1314; // int32
    pub const duration: usize = 0x1318; // float32
    pub const watch_path_vision_radius: usize = 0x131C; // int32
    pub const impact_damage: usize = 0x1320; // int32
    pub const pull_duration: usize = 0x1324; // float32
    pub const activation_delay: usize = 0x1328; // float32
    pub const m_bPiercesCreeps: usize = 0x132C; // bool
    pub const m_hAlreadyHit: usize = 0x1330; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_VoidSpirit_AetherRemnant_WatchThinker {
    pub const remnant_watch_radius: usize = 0x12C8; // int32
    pub const m_flLastDamageTick: usize = 0x12CC; // GameTime_t
    pub const damage_tick_rate: usize = 0x12D0; // float32
    pub const flDamage: usize = 0x12D4; // float32
    pub const m_bPiercesCreeps: usize = 0x12D8; // bool
}

pub mod CDOTA_Modifier_VoidSpirit_AstralStep_Caster {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_VoidSpirit_AstralStep_Debuff {
    pub const m_nFXStackIndex: usize = 0x12C8; // ParticleIndex_t
    pub const movement_slow_pct: usize = 0x12CC; // int32
    pub const pop_damage: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_VoidSpirit_AstralStep_Intrinsic {
    pub const ability_chance_pct: usize = 0x12C8; // int32
    pub const attack_chance_pct: usize = 0x12CC; // int32
    pub const pop_damage_delay: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_VoidSpirit_Dissimilate_Phase {
    pub const m_vFirstPortalPos: usize = 0x12C8; // Vector
    pub const m_vecDestinations: usize = 0x12D8; // CUtlVector< Vector >
    pub const m_vecDestinationParticles: usize = 0x12F0; // CUtlVector< ParticleIndex_t >
    pub const m_vecDestinationParticles_EnemyTeam: usize = 0x1308; // CUtlVector< ParticleIndex_t >
    pub const m_nCurrentDestinationIndex: usize = 0x1320; // int32
    pub const m_nClosestIndexPosToClick: usize = 0x1324; // int32
    pub const m_nPortalHeightOffset: usize = 0x1328; // int32
    pub const m_szAlliesPortalFX: usize = 0x1330; // char*
    pub const m_szEnemiesPortalFX: usize = 0x1338; // char*
    pub const m_nPortalRadius: usize = 0x1340; // int32
    pub const m_nPortalPadding: usize = 0x1344; // int32
    pub const m_nFinalImpactDamageRadius: usize = 0x1348; // int32
    pub const m_nFinalImpactFXRadius: usize = 0x134C; // int32
    pub const destination_fx_radius: usize = 0x1350; // int32
    pub const damage_radius: usize = 0x1354; // int32
    pub const portals_per_ring: usize = 0x1358; // int32
    pub const angle_per_ring_portal: usize = 0x135C; // int32
    pub const first_ring_distance_offset: usize = 0x1360; // int32
    pub const debuff_duration: usize = 0x1364; // float32
}

pub mod CDOTA_Modifier_VoidSpirit_ResonantPulse_PhysicalBuff {
    pub const base_absorb_amount: usize = 0x12C8; // int32
    pub const m_nAbsorbRemaining: usize = 0x12CC; // int32
    pub const m_nMaxAbsorb: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_VoidSpirit_ResonantPulse_Ring {
    pub const m_fLastRadius: usize = 0x12C8; // float32
    pub const m_fCurRadius: usize = 0x12CC; // float32
    pub const m_fLastThink: usize = 0x12D0; // GameTime_t
    pub const m_nFXIndex: usize = 0x12D4; // ParticleIndex_t
    pub const m_EntitiesHit: usize = 0x12D8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const speed: usize = 0x12F0; // int32
    pub const radius: usize = 0x12F4; // int32
    pub const damage: usize = 0x12F8; // int32
}

pub mod CDOTA_Modifier_Voidwalker_Phased {
    pub const damage: usize = 0x12C8; // int32
    pub const max_damage_duration: usize = 0x12CC; // float32
    pub const attack_range_bonus: usize = 0x12D0; // int32
    pub const m_InFlightAttackRecords: usize = 0x12D8; // CUtlVector< int16 >
    pub const bAttackRange: usize = 0x12F0; // bool
    pub const m_flStartTime: usize = 0x12F4; // float32
    pub const m_flFadeTime: usize = 0x12F8; // float32
    pub const m_flDamageScale: usize = 0x12FC; // float32
    pub const duration: usize = 0x1300; // float32
}

pub mod CDOTA_Modifier_Warlock_FatalBonds {
    pub const m_FatalBondsEntities: usize = 0x12C8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const damage_share_percentage: usize = 0x12E0; // int32
    pub const imp_duration: usize = 0x12E4; // float32
}

pub mod CDOTA_Modifier_Warlock_Golem_Flaming_Fists {
    pub const damage: usize = 0x12C8; // float32
    pub const radius: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Warlock_Golem_Permanent_Immolation {
    pub const aura_radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Warlock_Golem_Permanent_Immolation_Debuff {
    pub const aura_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Warlock_Imp_AutoAttack {
    pub const m_hBestTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const bonus_hp: usize = 0x12CC; // float32
    pub const bonus_dmg: usize = 0x12D0; // float32
    pub const bonus_movespeed: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Warlock_Imp_ExplodeThinker {
    pub const explosion_radius: usize = 0x12C8; // float32
    pub const explosion_dmg: usize = 0x12CC; // float32
    pub const explosion_nonhero_reduce: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Warlock_Rain_Of_Chaos_Golem {
    pub const bonus_magic_resistance: usize = 0x12C8; // int32
    pub const bonus_armor: usize = 0x12CC; // int32
    pub const bonus_slow_resistance: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Warlock_Rain_Of_Chaos_Thinker {
    pub const stun_duration: usize = 0x12C8; // float32
    pub const bHasScepter: usize = 0x12CC; // bool
    pub const aoe: usize = 0x12D0; // int32
    pub const m_iCount: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Warlock_Shadow_Word {
    pub const tick_interval: usize = 0x12C8; // float32
    pub const damage: usize = 0x12CC; // int32
    pub const shard_movement_speed_pct: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Warlock_Upheaval {
    pub const m_iSlow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Warlock_Upheaval_Ally {
    pub const m_iAttackSpeed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_WarpineRaider_SeedShot {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_WarpineRaider_SeedShot_Slow {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Watch_Tower {
    pub const m_iCapturingTeam: usize = 0x12C8; // int32
    pub const m_flCaptureProgress: usize = 0x12CC; // float32
    pub const m_flBonusTime: usize = 0x12D0; // float32
    pub const m_bInitialSetupDone: usize = 0x12D4; // bool
    pub const m_iOriginalTeam: usize = 0x12D8; // int32
    pub const m_hPreviousHero: usize = 0x12DC; // CHandle< C_BaseEntity >
    pub const m_vecLastCreditedPlayerIDs: usize = 0x12E0; // CUtlVector< PlayerID_t >
    pub const m_flAccumulatedCaptureTime: usize = 0x12F8; // float32
    pub const m_iBonusCount: usize = 0x12FC; // int32
    pub const m_nFxOutpostAmbient: usize = 0x1300; // ParticleIndex_t
    pub const m_nFxOutpostInitialAmbient: usize = 0x1304; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Wave_Blast_Knockback {
    pub const disarm_duration: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Weaver_GeminateAttack {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_iAttacksRemaining: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Weaver_GeminateAttack_Bonus {
    pub const bonus_damage: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Weaver_Shukuchi {
    pub const radius: usize = 0x12D8; // int32
    pub const damage: usize = 0x12DC; // int32
    pub const speed: usize = 0x12E0; // int32
    pub const geminate_attack_mark_duration: usize = 0x12E4; // float32
    pub const m_hEntitiesAffected: usize = 0x12E8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Weaver_Shukuchi_GeminateAttackManager {
    pub const m_iAttacksRemaining: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Weaver_Swarm {
    pub const m_iMaxDist: usize = 0x12C8; // int32
    pub const m_iAttackCount: usize = 0x12CC; // int32
    pub const destroy_attacks: usize = 0x12D0; // int32
    pub const m_vDirection: usize = 0x12D4; // Vector
    pub const m_vStartLoc: usize = 0x12E0; // Vector
    pub const m_hAttachTarget: usize = 0x12EC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Weaver_Swarm_Debuff {
    pub const armor_reduction: usize = 0x12C8; // float32
    pub const damage: usize = 0x12CC; // int32
    pub const m_flCurrentArmorReduction: usize = 0x12D0; // float32
}

pub mod CDOTA_Modifier_Weaver_TimeLapse {
    pub const m_PositionIndex: usize = 0x12C8; // Vector[11]
    pub const m_flLife: usize = 0x134C; // float32[11]
    pub const m_flMana: usize = 0x1378; // float32[11]
    pub const m_bHasValidHistory: usize = 0x13A4; // bool
}

pub mod CDOTA_Modifier_Wildkin_Tornado {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Wind_Waker {
    pub const m_hBlocker: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_flStartTime: usize = 0x12CC; // GameTime_t
}

pub mod CDOTA_Modifier_Windrunner_Arcana_Kill_Effect {
    pub const m_bFrozen: usize = 0x12C8; // bool
}

pub mod CDOTA_Modifier_Windrunner_FocusFire {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const focusfire_damage_reduction: usize = 0x12CC; // int32
    pub const bActive: usize = 0x12D0; // bool
    pub const bonus_attack_speed: usize = 0x12D4; // int32
    pub const focusfire_fire_on_the_move: usize = 0x12D8; // int32
    pub const m_bPause: usize = 0x12DC; // bool
    pub const m_nNumArrowsLanded: usize = 0x12E0; // int32
    pub const m_nDamageDealt: usize = 0x12E4; // int32
    pub const m_nDamageDealtMax: usize = 0x12E8; // int32
    pub const m_nTargetInitialHP: usize = 0x12EC; // int32
    pub const m_bHeroKilled: usize = 0x12F0; // bool
    pub const m_bRecentFocusFireAttack: usize = 0x12F1; // bool
    pub const m_nFXFocusFire: usize = 0x12FC; // ParticleIndex_t
    pub const m_flLastFocusFireAttackTime: usize = 0x1300; // GameTime_t
}

pub mod CDOTA_Modifier_Windrunner_GaleForce_Aura {
    pub const radius: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Windrunner_Windrun {
    pub const radius: usize = 0x12C8; // int32
    pub const scepter_radius: usize = 0x12CC; // int32
    pub const movespeed_bonus_pct: usize = 0x12D0; // int32
    pub const m_bDispellable: usize = 0x12D4; // bool
    pub const physical_damage_pct: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Windrunner_Windrun_Invis_Thinker {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Windrunner_Windrun_Slow {
    pub const enemy_movespeed_bonus_pct: usize = 0x12C8; // float32
    pub const scepter_movespeed_bonus_pct: usize = 0x12CC; // float32
    pub const scepter_blind: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Winter_Wyvern_Arctic_Burn_Flight {
    pub const attack_point: usize = 0x12C8; // float32
    pub const attack_range_bonus: usize = 0x12CC; // int32
    pub const projectile_speed_bonus: usize = 0x12D0; // int32
    pub const night_vision_bonus: usize = 0x12D4; // int32
    pub const movement_scepter: usize = 0x12D8; // int32
    pub const m_iszRangedAttackEffect: usize = 0x12E0; // CUtlSymbolLarge
    pub const max_attacks: usize = 0x12E8; // int32
    pub const m_iNumAttacks: usize = 0x12EC; // int32
}

pub mod CDOTA_Modifier_Winter_Wyvern_Arctic_Burn_Frost_Attack {
    pub const m_InFlightAttackRecords: usize = 0x12C8; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Winter_Wyvern_Arctic_Burn_Slow {
    pub const percent_damage: usize = 0x12C8; // float32
    pub const move_slow: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Winter_Wyvern_Cold_Embrace {
    pub const heal_additive: usize = 0x12D0; // int32
    pub const heal_percentage: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Winter_Wyvern_Splinter_Blast_Slow {
    pub const bonus_movespeed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Winter_Wyvern_Winters_Curse {
    pub const m_hZombieTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_hDesiredTarget: usize = 0x12CC; // CHandle< C_BaseEntity >
    pub const damage_reduction: usize = 0x12D0; // int32
    pub const damage_amplification: usize = 0x12D4; // int32
    pub const bonus_attack_speed: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_Winter_Wyvern_Winters_Curse_Aura {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_nAlliesTaunted: usize = 0x12CC; // int32
    pub const m_bRelicTriggered: usize = 0x12D0; // bool
    pub const radius: usize = 0x12D4; // int32
    pub const damage_amplification: usize = 0x12D8; // int32
    pub const damage_reduction: usize = 0x12DC; // int32
    pub const m_flLastSeen: usize = 0x12E0; // GameTime_t
}

pub mod CDOTA_Modifier_Winter_Wyvern_Winters_Curse_Kill_Credit {
    pub const m_InFlightAttackRecords: usize = 0x12C8; // CUtlVector< int16 >
}

pub mod CDOTA_Modifier_Wisp_Overcharge {
    pub const bonus_attack_speed: usize = 0x12C8; // int32
    pub const bonus_spell_amp: usize = 0x12CC; // int32
    pub const shard_bonus_slow_resistance: usize = 0x12D0; // int32
    pub const hp_regen: usize = 0x12D4; // float32
}

pub mod CDOTA_Modifier_Wisp_Relocate_Return {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_vecReturnPosition: usize = 0x12CC; // Vector
    pub const return_time: usize = 0x12D8; // float32
    pub const m_nFXTimeRemaining: usize = 0x12DC; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Wisp_Relocate_Thinker {
    pub const cast_delay: usize = 0x12C8; // float32
    pub const m_nFXIndexEndTeam: usize = 0x12CC; // ParticleIndex_t
    pub const m_nFXIndexChannel: usize = 0x12D0; // ParticleIndex_t
}

pub mod CDOTA_Modifier_Wisp_Spirits {
    pub const creep_damage: usize = 0x12C8; // int32
    pub const hero_damage: usize = 0x12CC; // int32
    pub const hit_radius: usize = 0x12D0; // int32
    pub const hero_hit_radius: usize = 0x12D4; // int32
    pub const explode_radius: usize = 0x12D8; // int32
    pub const min_range: usize = 0x12DC; // int32
    pub const max_range: usize = 0x12E0; // int32
    pub const default_radius: usize = 0x12E4; // int32
    pub const spirit_amount: usize = 0x12E8; // int32
    pub const m_flRotation: usize = 0x12EC; // float32
    pub const m_flSpiritRadius: usize = 0x12F0; // float32
    pub const spirit_movement_rate: usize = 0x12F4; // int32
    pub const m_flNextSpawn: usize = 0x12F8; // GameTime_t
    pub const m_strSpiritsOutSwapAbility: usize = 0x1300; // CUtlString
}

pub mod CDOTA_Modifier_Wisp_Spirits_Slow {
    pub const scepter_slow_pct: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Wisp_Tentacles {
    pub const m_nFXIndex: usize = 0x12C8; // ParticleIndex_t
    pub const hCurrentEntity: usize = 0x12CC; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Wisp_Tether {
    pub const m_bInManaGained: usize = 0x12C8; // bool
    pub const m_hTarget: usize = 0x12CC; // CHandle< C_BaseEntity >
    pub const hStunnedEntities: usize = 0x12D0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const stun_duration: usize = 0x12E8; // float32
    pub const movespeed: usize = 0x12EC; // int32
    pub const self_bonus: usize = 0x12F0; // int32
    pub const m_bIsInRange: usize = 0x12F4; // bool
    pub const radius: usize = 0x12F8; // int32
    pub const latch_distance: usize = 0x12FC; // int32
    pub const m_flHealthHealed: usize = 0x1300; // float32
    pub const m_flManaHealed: usize = 0x1304; // float32
    pub const tether_heal_amp: usize = 0x1308; // float32
    pub const m_flHealMessageTime: usize = 0x130C; // GameTime_t
    pub const m_flManaMessageTime: usize = 0x1310; // GameTime_t
}

pub mod CDOTA_Modifier_Wisp_Tether_Haste {
    pub const movespeed: usize = 0x12C8; // float32
    pub const shard_bonus_spell_lifesteal: usize = 0x12CC; // float32
}

pub mod CDOTA_Modifier_Wisp_Tether_Slow {
    pub const slow: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Wisp_Tether_Spell_Lifesteal {
    pub const shard_bonus_spell_lifesteal: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_WitchDoctor_Cask_Thinker {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_WitchDoctor_DeathWard {
    pub const bonus_damage: usize = 0x12C8; // int32
    pub const m_iBounceRadius: usize = 0x12CC; // int32
    pub const bonus_accuracy: usize = 0x12D0; // int32
    pub const m_bFirstAttack: usize = 0x12D4; // bool
}

pub mod CDOTA_Modifier_WitchDoctor_DeathWard_Voodoo_Switcheroo_AttackSpeedReduction {
    pub const attack_speed_reduction: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_WitchDoctor_Maledict {
    pub const m_iStartHealth: usize = 0x12C8; // int32
    pub const bonus_damage: usize = 0x12CC; // int32
    pub const bonus_damage_threshold: usize = 0x12D0; // int32
    pub const ticks: usize = 0x12D4; // int32
    pub const iCurrentTick: usize = 0x12D8; // int32
}

pub mod CDOTA_Modifier_WitchDoctor_Voodoo_Restoration_Heal {
    pub const heal: usize = 0x12C8; // int32
    pub const mana_per_second: usize = 0x12CC; // float32
    pub const heal_interval: usize = 0x12D0; // float32
    pub const enemy_damage_pct: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_WitchDoctor_Voodoo_Switcheroo {
    pub const m_vSavedPos: usize = 0x12C8; // Vector
    pub const m_hSavedAbility: usize = 0x12D4; // CHandle< C_BaseEntity >
}

pub mod CDOTA_Modifier_Woodland_Striders_Active {
    pub const tree_duration: usize = 0x12C8; // float32
}

pub mod CDOTA_Modifier_Yasha_And_Kaya_Maim {
    pub const maim_slow_movement: usize = 0x12C8; // int32
    pub const maim_slow_attack: usize = 0x12CC; // int32
    pub const maim_slow_movement_range: usize = 0x12D0; // int32
    pub const maim_slow_attack_range: usize = 0x12D4; // int32
}

pub mod CDOTA_Modifier_Zombie_Berserk {
    pub const berserk_bonus_attack_speed: usize = 0x12C8; // int32
    pub const berserk_bonus_movement_speed: usize = 0x12CC; // int32
}

pub mod CDOTA_Modifier_Zuus_ArcLightning {
    pub const radius: usize = 0x12C8; // int32
    pub const jump_count: usize = 0x12CC; // int32
    pub const arc_damage: usize = 0x12D0; // int32
    pub const jump_delay: usize = 0x12D4; // float32
    pub const damage_health_pct: usize = 0x12D8; // int32
    pub const total_damage_pct: usize = 0x12DC; // int32
    pub const m_iCurJumpCount: usize = 0x12E0; // int32
    pub const trigger_spell_absorb: usize = 0x12E4; // bool
    pub const m_vCurTargetLoc: usize = 0x12E8; // Vector
    pub const m_hHitEntities: usize = 0x12F8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_Modifier_Zuus_Cloud {
    pub const m_CloudBoltTimer: usize = 0x12C8; // CountdownTimer
    pub const cloud_bolt_interval: usize = 0x12E0; // float32
    pub const cloud_radius: usize = 0x12E4; // int32
}

pub mod CDOTA_Modifier_Zuus_Heavenly_Jump_Boost {
    pub const postjump_movespeed: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Zuus_Lightning_Hands {
    pub const m_InFlightAttackRecords: usize = 0x12C8; // CUtlVector< int16 >
    pub const m_nFXIndex: usize = 0x12E0; // ParticleIndex_t
    pub const arc_lightning_damage_pct: usize = 0x12E4; // int32
    pub const arc_lightning_damage_illusion_pct: usize = 0x12E8; // int32
    pub const attack_range_bonus: usize = 0x12EC; // int32
}

pub mod CDOTA_Modifier_Zuus_Static_Field_Slow {
    pub const move_slow: usize = 0x12C8; // int32
    pub const aspd_slow: usize = 0x12CC; // int32
    pub const cast_slow: usize = 0x12D0; // int32
}

pub mod CDOTA_Modifier_Zuus_ThunderTrail_Debuff {
    pub const debuff_spell_amp: usize = 0x12C8; // int32
}

pub mod CDOTA_Modifier_Zuus_ThundergodsWrathThinker {
    pub const damage: usize = 0x12C8; // int32
    pub const damage_pct: usize = 0x12CC; // float32
    pub const sight_duration: usize = 0x12D0; // float32
    pub const second_strike_damage_percent: usize = 0x12D4; // int32
    pub const second_strike_delay: usize = 0x12D8; // float32
}

pub mod CDOTA_NPC_Observer_Ward {
    pub const m_iDuration: usize = 0x1750; // int32
    pub const m_pVisionRangeFX: usize = 0x1758; // CNewParticleEffect*
    pub const m_nPreviewViewer: usize = 0x1760; // int32
}

pub mod CDOTA_NPC_Observer_Ward_TrueSight {
    pub const m_iTrueSight: usize = 0x1768; // int32
    pub const m_hCasterEntity: usize = 0x176C; // CEntityHandle
    pub const m_hAbilityEntity: usize = 0x1770; // CEntityHandle
}

pub mod CDOTA_PlayerChallengeInfo {
    pub const nType: usize = 0x30; // int32
    pub const nQuestID: usize = 0x34; // int32
    pub const nQuestChallengeID: usize = 0x38; // int32
    pub const nTier: usize = 0x3C; // int32
    pub const nParam0: usize = 0x40; // int32
    pub const nParam1: usize = 0x44; // int32
    pub const nSlotID: usize = 0x48; // int32
    pub const nProgress: usize = 0x4C; // int32
    pub const nCompletionThreshold: usize = 0x50; // int32
    pub const nPlayerID: usize = 0x54; // PlayerID_t
    pub const nQueryIndex: usize = 0x58; // int32
    pub const nEventID: usize = 0x5C; // int32
    pub const nSequenceID: usize = 0x60; // int32
    pub const nCompleted: usize = 0x64; // int32
    pub const nRank: usize = 0x68; // int32
}

pub mod CDOTA_Unit_Announcer {
    pub const m_currentAnnouncer: usize = 0x1750; // CAnnouncerDescriptor
}

pub mod CDOTA_Unit_Hero_AbyssalUnderlord {
    pub const m_iRangeFX: usize = 0x1AB0; // ParticleIndex_t
}

pub mod CDOTA_Unit_Hero_ArcWarden {
    pub const m_nTalkFXIndex: usize = 0x1AB0; // ParticleIndex_t
    pub const m_nFXDeath: usize = 0x1AB4; // ParticleIndex_t
    pub const m_nTempestFX: usize = 0x1AB8; // ParticleIndex_t
}

pub mod CDOTA_Unit_Hero_Dawnbreaker {
    pub const m_nAttackState: usize = 0x1AB0; // int32
}

pub mod CDOTA_Unit_Hero_Grimstroke {
    pub const m_nFXDeath: usize = 0x1AB0; // ParticleIndex_t
}

pub mod CDOTA_modifier_aghsfort_juggernaut_omnislash {
    pub const m_hTarget: usize = 0x12C8; // CHandle< C_BaseEntity >
    pub const m_hLastTarget: usize = 0x12CC; // CHandle< C_BaseEntity >
    pub const m_nJumps: usize = 0x12D0; // int32
    pub const bonus_damage: usize = 0x12D4; // int32
    pub const bonus_attack_speed: usize = 0x12D8; // int32
    pub const omni_slash_radius: usize = 0x12DC; // int32
    pub const attack_rate_multiplier: usize = 0x12E0; // float32
    pub const upgraded_radius: usize = 0x12E4; // int32
    pub const m_iTotalDamage: usize = 0x12E8; // int32
    pub const m_iHeroDamage: usize = 0x12EC; // int32
    pub const m_bReflection: usize = 0x12F0; // bool
    pub const m_bFirstHit: usize = 0x12F1; // bool
    pub const m_bEndNext: usize = 0x12F2; // bool
    pub const m_fNextAttackTime: usize = 0x12F4; // GameTime_t
    pub const m_vCastOrigin: usize = 0x12F8; // Vector
    pub const m_hIllusions: usize = 0x1308; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod CDOTA_modifier_aghsfort_juggernaut_omnislash_Camera {
    pub const m_bIsDone: usize = 0x12C8; // bool
    pub const m_hFollowEnt: usize = 0x12CC; // CHandle< C_BaseEntity >
}

pub mod CDecalInfo {
    pub const m_flAnimationScale: usize = 0x0; // float32
    pub const m_flAnimationLifeSpan: usize = 0x4; // float32
    pub const m_flPlaceTime: usize = 0x8; // float32
    pub const m_flFadeStartTime: usize = 0xC; // float32
    pub const m_flFadeDuration: usize = 0x10; // float32
    pub const m_nVBSlot: usize = 0x14; // int32
    pub const m_nBoneIndex: usize = 0x18; // int32
    pub const m_pNext: usize = 0x28; // CDecalInfo*
    pub const m_pPrev: usize = 0x30; // CDecalInfo*
    pub const m_nDecalMaterialIndex: usize = 0x90; // int32
}

pub mod CDeferredLightBase {
    pub const m_LightColor: usize = 0x10; // Color
    pub const m_flIntensity: usize = 0x14; // float32
    pub const m_flLightSize: usize = 0x18; // float32
    pub const m_flSpotFoV: usize = 0x1C; // float32
    pub const m_vLightDirection: usize = 0x20; // QAngle
    pub const m_flStartFalloff: usize = 0x2C; // float32
    pub const m_flDistanceFalloff: usize = 0x30; // float32
    pub const m_nFlags: usize = 0x34; // uint32
    pub const m_ProjectedTextureName: usize = 0x38; // char[512]
}

pub mod CEffectData {
    pub const m_vOrigin: usize = 0x8; // Vector
    pub const m_vStart: usize = 0x14; // Vector
    pub const m_vNormal: usize = 0x20; // Vector
    pub const m_vAngles: usize = 0x2C; // QAngle
    pub const m_hEntity: usize = 0x38; // CEntityHandle
    pub const m_hOtherEntity: usize = 0x3C; // CEntityHandle
    pub const m_flScale: usize = 0x40; // float32
    pub const m_flMagnitude: usize = 0x44; // float32
    pub const m_flRadius: usize = 0x48; // float32
    pub const m_nSurfaceProp: usize = 0x4C; // CUtlStringToken
    pub const m_nEffectIndex: usize = 0x50; // CWeakHandle< InfoForResourceTypeIParticleSystemDefinition >
    pub const m_nDamageType: usize = 0x58; // uint32
    pub const m_nPenetrate: usize = 0x5C; // uint8
    pub const m_nMaterial: usize = 0x5E; // uint16
    pub const m_nHitBox: usize = 0x60; // uint16
    pub const m_nColor: usize = 0x62; // uint8
    pub const m_fFlags: usize = 0x63; // uint8
    pub const m_nAttachmentIndex: usize = 0x64; // AttachmentHandle_t
    pub const m_nAttachmentName: usize = 0x68; // CUtlStringToken
    pub const m_iEffectName: usize = 0x6C; // uint16
    pub const m_nExplosionType: usize = 0x6E; // uint8
}

pub mod CEntityIdentity {
    pub const m_nameStringableIndex: usize = 0x14; // int32
    pub const m_name: usize = 0x18; // CUtlSymbolLarge
    pub const m_designerName: usize = 0x20; // CUtlSymbolLarge
    pub const m_flags: usize = 0x30; // uint32
    pub const m_worldGroupId: usize = 0x38; // WorldGroupId_t
    pub const m_fDataObjectTypes: usize = 0x3C; // uint32
    pub const m_PathIndex: usize = 0x40; // ChangeAccessorFieldPathIndex_t
    pub const m_pPrev: usize = 0x58; // CEntityIdentity*
    pub const m_pNext: usize = 0x60; // CEntityIdentity*
    pub const m_pPrevByClass: usize = 0x68; // CEntityIdentity*
    pub const m_pNextByClass: usize = 0x70; // CEntityIdentity*
}

pub mod CEntityInstance {
    pub const m_iszPrivateVScripts: usize = 0x8; // CUtlSymbolLarge
    pub const m_pEntity: usize = 0x10; // CEntityIdentity*
    pub const m_CScriptComponent: usize = 0x28; // CScriptComponent*
}

pub mod CFlashlightEffect {
    pub const m_bIsOn: usize = 0x10; // bool
    pub const m_bMuzzleFlashEnabled: usize = 0x20; // bool
    pub const m_flMuzzleFlashBrightness: usize = 0x24; // float32
    pub const m_quatMuzzleFlashOrientation: usize = 0x30; // Quaternion
    pub const m_vecMuzzleFlashOrigin: usize = 0x40; // Vector
    pub const m_flFov: usize = 0x4C; // float32
    pub const m_flFarZ: usize = 0x50; // float32
    pub const m_flLinearAtten: usize = 0x54; // float32
    pub const m_bCastsShadows: usize = 0x58; // bool
    pub const m_flCurrentPullBackDist: usize = 0x5C; // float32
    pub const m_FlashlightTexture: usize = 0x60; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_MuzzleFlashTexture: usize = 0x68; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_textureName: usize = 0x70; // char[64]
}

pub mod CFoWBlockerRegion {
    pub const m_vecFowBlockers: usize = 0x538; // C_UtlVectorEmbeddedNetworkVar< FowBlocker_t >
    pub const m_flMinX: usize = 0x588; // float32
    pub const m_flMaxX: usize = 0x58C; // float32
    pub const m_flMinY: usize = 0x590; // float32
    pub const m_flMaxY: usize = 0x594; // float32
    pub const m_flGridSize: usize = 0x598; // float32
}

pub mod CGameSceneNode {
    pub const m_nodeToWorld: usize = 0x10; // CTransform
    pub const m_pOwner: usize = 0x30; // CEntityInstance*
    pub const m_pParent: usize = 0x38; // CGameSceneNode*
    pub const m_pChild: usize = 0x40; // CGameSceneNode*
    pub const m_pNextSibling: usize = 0x48; // CGameSceneNode*
    pub const m_hParent: usize = 0x70; // CGameSceneNodeHandle
    pub const m_vecOrigin: usize = 0x80; // CNetworkOriginCellCoordQuantizedVector
    pub const m_angRotation: usize = 0xB8; // QAngle
    pub const m_flScale: usize = 0xC4; // float32
    pub const m_vecAbsOrigin: usize = 0xC8; // Vector
    pub const m_angAbsRotation: usize = 0xD4; // QAngle
    pub const m_flAbsScale: usize = 0xE0; // float32
    pub const m_nParentAttachmentOrBone: usize = 0xE4; // int16
    pub const m_bDebugAbsOriginChanges: usize = 0xE6; // bool
    pub const m_bDormant: usize = 0xE7; // bool
    pub const m_bForceParentToBeNetworked: usize = 0xE8; // bool
    pub const m_bDirtyHierarchy: usize = 0x0; // bitfield:1
    pub const m_bDirtyBoneMergeInfo: usize = 0x0; // bitfield:1
    pub const m_bNetworkedPositionChanged: usize = 0x0; // bitfield:1
    pub const m_bNetworkedAnglesChanged: usize = 0x0; // bitfield:1
    pub const m_bNetworkedScaleChanged: usize = 0x0; // bitfield:1
    pub const m_bWillBeCallingPostDataUpdate: usize = 0x0; // bitfield:1
    pub const m_bNotifyBoneTransformsChanged: usize = 0x0; // bitfield:1
    pub const m_bBoneMergeFlex: usize = 0x0; // bitfield:1
    pub const m_nLatchAbsOrigin: usize = 0x0; // bitfield:2
    pub const m_bDirtyBoneMergeBoneToRoot: usize = 0x0; // bitfield:1
    pub const m_nHierarchicalDepth: usize = 0xEB; // uint8
    pub const m_nHierarchyType: usize = 0xEC; // uint8
    pub const m_nDoNotSetAnimTimeInInvalidatePhysicsCount: usize = 0xED; // uint8
    pub const m_name: usize = 0xF0; // CUtlStringToken
    pub const m_hierarchyAttachName: usize = 0x130; // CUtlStringToken
    pub const m_flZOffset: usize = 0x134; // float32
    pub const m_vRenderOrigin: usize = 0x138; // Vector
}

pub mod CGameSceneNodeHandle {
    pub const m_hOwner: usize = 0x8; // CEntityHandle
    pub const m_name: usize = 0xC; // CUtlStringToken
}

pub mod CGlobalLightBase {
    pub const m_bSpotLight: usize = 0x10; // bool
    pub const m_SpotLightOrigin: usize = 0x14; // Vector
    pub const m_SpotLightAngles: usize = 0x20; // QAngle
    pub const m_ShadowDirection: usize = 0x2C; // Vector
    pub const m_AmbientDirection: usize = 0x38; // Vector
    pub const m_SpecularDirection: usize = 0x44; // Vector
    pub const m_InspectorSpecularDirection: usize = 0x50; // Vector
    pub const m_flSpecularPower: usize = 0x5C; // float32
    pub const m_flSpecularIndependence: usize = 0x60; // float32
    pub const m_SpecularColor: usize = 0x64; // Color
    pub const m_bStartDisabled: usize = 0x68; // bool
    pub const m_bEnabled: usize = 0x69; // bool
    pub const m_LightColor: usize = 0x6A; // Color
    pub const m_AmbientColor1: usize = 0x6E; // Color
    pub const m_AmbientColor2: usize = 0x72; // Color
    pub const m_AmbientColor3: usize = 0x76; // Color
    pub const m_flSunDistance: usize = 0x7C; // float32
    pub const m_flFOV: usize = 0x80; // float32
    pub const m_flNearZ: usize = 0x84; // float32
    pub const m_flFarZ: usize = 0x88; // float32
    pub const m_bEnableShadows: usize = 0x8C; // bool
    pub const m_bOldEnableShadows: usize = 0x8D; // bool
    pub const m_bBackgroundClearNotRequired: usize = 0x8E; // bool
    pub const m_flCloudScale: usize = 0x90; // float32
    pub const m_flCloud1Speed: usize = 0x94; // float32
    pub const m_flCloud1Direction: usize = 0x98; // float32
    pub const m_flCloud2Speed: usize = 0x9C; // float32
    pub const m_flCloud2Direction: usize = 0xA0; // float32
    pub const m_flAmbientScale1: usize = 0xB0; // float32
    pub const m_flAmbientScale2: usize = 0xB4; // float32
    pub const m_flGroundScale: usize = 0xB8; // float32
    pub const m_flLightScale: usize = 0xBC; // float32
    pub const m_flFoWDarkness: usize = 0xC0; // float32
    pub const m_bEnableSeparateSkyboxFog: usize = 0xC4; // bool
    pub const m_vFowColor: usize = 0xC8; // Vector
    pub const m_ViewOrigin: usize = 0xD4; // Vector
    pub const m_ViewAngles: usize = 0xE0; // QAngle
    pub const m_flViewFoV: usize = 0xEC; // float32
    pub const m_WorldPoints: usize = 0xF0; // Vector[8]
    pub const m_vFogOffsetLayer0: usize = 0x4A8; // Vector2D
    pub const m_vFogOffsetLayer1: usize = 0x4B0; // Vector2D
    pub const m_hEnvWind: usize = 0x4B8; // CHandle< C_BaseEntity >
    pub const m_hEnvSky: usize = 0x4BC; // CHandle< C_BaseEntity >
    pub const m_fSmoothedAmount: usize = 0x4C0; // float32
    pub const m_fSlowSmoothedAmount: usize = 0x4C4; // float32
}

pub mod CGlowOverlay {
    pub const m_vPos: usize = 0x8; // Vector
    pub const m_bDirectional: usize = 0x14; // bool
    pub const m_vDirection: usize = 0x18; // Vector
    pub const m_bInSky: usize = 0x24; // bool
    pub const m_skyObstructionScale: usize = 0x28; // float32
    pub const m_Sprites: usize = 0x30; // CGlowSprite[4]
    pub const m_nSprites: usize = 0xB0; // int32
    pub const m_flProxyRadius: usize = 0xB4; // float32
    pub const m_flHDRColorScale: usize = 0xB8; // float32
    pub const m_flGlowObstructionScale: usize = 0xBC; // float32
    pub const m_bCacheGlowObstruction: usize = 0xC0; // bool
    pub const m_bCacheSkyObstruction: usize = 0xC1; // bool
    pub const m_bActivated: usize = 0xC2; // int16
    pub const m_ListIndex: usize = 0xC4; // uint16
    pub const m_queryHandle: usize = 0xC8; // int32
}

pub mod CGlowProperty {
    pub const m_fGlowColor: usize = 0x8; // Vector
    pub const m_iGlowType: usize = 0x30; // int32
    pub const m_iGlowTeam: usize = 0x34; // int32
    pub const m_nGlowRange: usize = 0x38; // int32
    pub const m_nGlowRangeMin: usize = 0x3C; // int32
    pub const m_glowColorOverride: usize = 0x40; // Color
    pub const m_bFlashing: usize = 0x44; // bool
    pub const m_flGlowTime: usize = 0x48; // float32
    pub const m_flGlowStartTime: usize = 0x4C; // float32
    pub const m_bGlowing: usize = 0x50; // bool
}

pub mod CGlowSprite {
    pub const m_vColor: usize = 0x0; // Vector
    pub const m_flHorzSize: usize = 0xC; // float32
    pub const m_flVertSize: usize = 0x10; // float32
    pub const m_hMaterial: usize = 0x18; // CStrongHandle< InfoForResourceTypeIMaterial2 >
}

pub mod CHeroStatueLiked {
    pub const m_iPlayerIDLiker: usize = 0x30; // PlayerID_t
    pub const m_iPlayerIDLiked: usize = 0x34; // PlayerID_t
}

pub mod CHeroesPerPlayer {
    pub const m_vecHeroIDs: usize = 0x30; // C_NetworkUtlVectorBase< int32 >
}

pub mod CHitboxComponent {
    pub const m_bvDisabledHitGroups: usize = 0x24; // uint32[1]
}

pub mod CInfoDynamicShadowHint {
    pub const m_bDisabled: usize = 0x538; // bool
    pub const m_flRange: usize = 0x53C; // float32
    pub const m_nImportance: usize = 0x540; // int32
    pub const m_nLightChoice: usize = 0x544; // int32
    pub const m_hLight: usize = 0x548; // CHandle< C_BaseEntity >
}

pub mod CInfoDynamicShadowHintBox {
    pub const m_vBoxMins: usize = 0x550; // Vector
    pub const m_vBoxMaxs: usize = 0x55C; // Vector
}

pub mod CInfoOffscreenPanoramaTexture {
    pub const m_bDisabled: usize = 0x538; // bool
    pub const m_nResolutionX: usize = 0x53C; // int32
    pub const m_nResolutionY: usize = 0x540; // int32
    pub const m_szLayoutFileName: usize = 0x548; // CUtlSymbolLarge
    pub const m_RenderAttrName: usize = 0x550; // CUtlSymbolLarge
    pub const m_TargetEntities: usize = 0x558; // C_NetworkUtlVectorBase< CHandle< C_BaseModelEntity > >
    pub const m_nTargetChangeCount: usize = 0x570; // int32
    pub const m_vecCSSClasses: usize = 0x578; // C_NetworkUtlVectorBase< CUtlSymbolLarge >
    pub const m_bCheckCSSClasses: usize = 0x6F0; // bool
}

pub mod CInfoWorldLayer {
    pub const m_pOutputOnEntitiesSpawned: usize = 0x538; // CEntityIOOutput
    pub const m_worldName: usize = 0x560; // CUtlSymbolLarge
    pub const m_layerName: usize = 0x568; // CUtlSymbolLarge
    pub const m_bWorldLayerVisible: usize = 0x570; // bool
    pub const m_bEntitiesSpawned: usize = 0x571; // bool
    pub const m_bCreateAsChildSpawnGroup: usize = 0x572; // bool
    pub const m_hLayerSpawnGroup: usize = 0x574; // uint32
    pub const m_bWorldLayerActuallyVisible: usize = 0x578; // bool
}

pub mod CIngameEvent_MuertaReleaseSpring2023 {
    pub const m_bMiniGameActive: usize = 0x18C0; // bool
    pub const m_vecTargetAssignments: usize = 0x18C1; // int8[10]
    pub const m_vecMiniGamePoints: usize = 0x18CB; // uint8[10]
    pub const m_vecMiniGameKills: usize = 0x18D5; // uint8[10]
    pub const m_activeGravestones: usize = 0x18F0; // CUtlVector< ParticleIndex_t >
}

pub mod CInterpolatedValue {
    pub const m_flStartTime: usize = 0x0; // float32
    pub const m_flEndTime: usize = 0x4; // float32
    pub const m_flStartValue: usize = 0x8; // float32
    pub const m_flEndValue: usize = 0xC; // float32
    pub const m_nInterpType: usize = 0x10; // int32
}

pub mod CLightComponent {
    pub const __m_pChainEntity: usize = 0x48; // CNetworkVarChainer
    pub const m_Color: usize = 0x85; // Color
    pub const m_SecondaryColor: usize = 0x89; // Color
    pub const m_flBrightness: usize = 0x90; // float32
    pub const m_flBrightnessScale: usize = 0x94; // float32
    pub const m_flBrightnessMult: usize = 0x98; // float32
    pub const m_flRange: usize = 0x9C; // float32
    pub const m_flFalloff: usize = 0xA0; // float32
    pub const m_flAttenuation0: usize = 0xA4; // float32
    pub const m_flAttenuation1: usize = 0xA8; // float32
    pub const m_flAttenuation2: usize = 0xAC; // float32
    pub const m_flTheta: usize = 0xB0; // float32
    pub const m_flPhi: usize = 0xB4; // float32
    pub const m_hLightCookie: usize = 0xB8; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_nCascades: usize = 0xC0; // int32
    pub const m_nCastShadows: usize = 0xC4; // int32
    pub const m_nShadowWidth: usize = 0xC8; // int32
    pub const m_nShadowHeight: usize = 0xCC; // int32
    pub const m_bRenderDiffuse: usize = 0xD0; // bool
    pub const m_nRenderSpecular: usize = 0xD4; // int32
    pub const m_bRenderTransmissive: usize = 0xD8; // bool
    pub const m_flOrthoLightWidth: usize = 0xDC; // float32
    pub const m_flOrthoLightHeight: usize = 0xE0; // float32
    pub const m_nStyle: usize = 0xE4; // int32
    pub const m_Pattern: usize = 0xE8; // CUtlString
    pub const m_nCascadeRenderStaticObjects: usize = 0xF0; // int32
    pub const m_flShadowCascadeCrossFade: usize = 0xF4; // float32
    pub const m_flShadowCascadeDistanceFade: usize = 0xF8; // float32
    pub const m_flShadowCascadeDistance0: usize = 0xFC; // float32
    pub const m_flShadowCascadeDistance1: usize = 0x100; // float32
    pub const m_flShadowCascadeDistance2: usize = 0x104; // float32
    pub const m_flShadowCascadeDistance3: usize = 0x108; // float32
    pub const m_nShadowCascadeResolution0: usize = 0x10C; // int32
    pub const m_nShadowCascadeResolution1: usize = 0x110; // int32
    pub const m_nShadowCascadeResolution2: usize = 0x114; // int32
    pub const m_nShadowCascadeResolution3: usize = 0x118; // int32
    pub const m_bUsesBakedShadowing: usize = 0x11C; // bool
    pub const m_nShadowPriority: usize = 0x120; // int32
    pub const m_nBakedShadowIndex: usize = 0x124; // int32
    pub const m_bRenderToCubemaps: usize = 0x128; // bool
    pub const m_LightGroups: usize = 0x130; // CUtlSymbolLarge
    pub const m_nDirectLight: usize = 0x138; // int32
    pub const m_nIndirectLight: usize = 0x13C; // int32
    pub const m_flFadeMinDist: usize = 0x140; // float32
    pub const m_flFadeMaxDist: usize = 0x144; // float32
    pub const m_flShadowFadeMinDist: usize = 0x148; // float32
    pub const m_flShadowFadeMaxDist: usize = 0x14C; // float32
    pub const m_bEnabled: usize = 0x150; // bool
    pub const m_bFlicker: usize = 0x151; // bool
    pub const m_bPrecomputedFieldsValid: usize = 0x152; // bool
    pub const m_vPrecomputedBoundsMins: usize = 0x154; // Vector
    pub const m_vPrecomputedBoundsMaxs: usize = 0x160; // Vector
    pub const m_vPrecomputedOBBOrigin: usize = 0x16C; // Vector
    pub const m_vPrecomputedOBBAngles: usize = 0x178; // QAngle
    pub const m_vPrecomputedOBBExtent: usize = 0x184; // Vector
    pub const m_flPrecomputedMaxRange: usize = 0x190; // float32
    pub const m_nFogLightingMode: usize = 0x194; // int32
    pub const m_flFogContributionStength: usize = 0x198; // float32
    pub const m_flNearClipPlane: usize = 0x19C; // float32
    pub const m_SkyColor: usize = 0x1A0; // Color
    pub const m_flSkyIntensity: usize = 0x1A4; // float32
    pub const m_SkyAmbientBounce: usize = 0x1A8; // Color
    pub const m_bUseSecondaryColor: usize = 0x1AC; // bool
    pub const m_bMixedShadows: usize = 0x1AD; // bool
    pub const m_flLightStyleStartTime: usize = 0x1B0; // GameTime_t
    pub const m_flCapsuleLength: usize = 0x1B4; // float32
    pub const m_flMinRoughness: usize = 0x1B8; // float32
}

pub mod CLightInfoBase {
    pub const m_origin2D: usize = 0x10; // Vector2D
    pub const m_Color: usize = 0x18; // Color[2]
    pub const m_LightScale: usize = 0x20; // float32[2]
    pub const m_AmbientColor: usize = 0x28; // Color[2]
    pub const m_AmbientScale: usize = 0x30; // float32[2]
    pub const m_ShadowColor: usize = 0x38; // Color[2]
    pub const m_ShadowSecondaryColor: usize = 0x40; // Color[2]
    pub const m_ShadowScale: usize = 0x48; // float32[2]
    pub const m_ShadowGroundScale: usize = 0x50; // float32[2]
    pub const m_SpecularColor: usize = 0x58; // Color[2]
    pub const m_flSpecularPower: usize = 0x60; // float32[2]
    pub const m_flSpecularIndependence: usize = 0x68; // float32[2]
    pub const m_SpecularDirection: usize = 0x70; // Vector[2]
    pub const m_InspectorSpecularDirection: usize = 0x88; // Vector[2]
    pub const m_LightDirection: usize = 0xA0; // Vector[2]
    pub const m_AmbientDirection: usize = 0xB8; // Vector[2]
    pub const m_FogColor: usize = 0xD0; // Color[2]
    pub const m_FogStart: usize = 0xD8; // float32[2]
    pub const m_FogEnd: usize = 0xE0; // float32[2]
    pub const m_HeightFogValue: usize = 0xE8; // float32[2]
    pub const m_HeightFogColor: usize = 0xF0; // Color[2]
    pub const m_FoWDarkness: usize = 0xF8; // float32[2]
    pub const m_FoWColorR: usize = 0x100; // float32[2]
    pub const m_FoWColorG: usize = 0x108; // float32[2]
    pub const m_FoWColorB: usize = 0x110; // float32[2]
    pub const m_InspectorViewFogColor: usize = 0x118; // Color[2]
    pub const m_windAngle: usize = 0x120; // QAngle
    pub const m_flWindAmount: usize = 0x12C; // float32[2]
    pub const m_flMinWind: usize = 0x134; // float32
    pub const m_flMaxWind: usize = 0x138; // float32
    pub const m_flMinGust: usize = 0x13C; // float32
    pub const m_flMaxGust: usize = 0x140; // float32
    pub const m_flMinGustDelay: usize = 0x144; // float32
    pub const m_flMaxGustDelay: usize = 0x148; // float32
    pub const m_flGustDuration: usize = 0x14C; // float32
    pub const m_flGustDirChange: usize = 0x150; // float32
    pub const m_skyboxAngle: usize = 0x154; // QAngle[2]
    pub const m_hSkyboxMaterial: usize = 0x170; // CStrongHandle< InfoForResourceTypeIMaterial2 >[2]
    pub const m_vSkyboxTintColor: usize = 0x180; // Color[2]
    pub const m_nSkyboxFogType: usize = 0x188; // uint8
    pub const m_flSkyboxAngularFogMaxEnd: usize = 0x18C; // float32
    pub const m_flSkyboxAngularFogMaxStart: usize = 0x190; // float32
    pub const m_flSkyboxAngularFogMinStart: usize = 0x194; // float32
    pub const m_flSkyboxAngularFogMinEnd: usize = 0x198; // float32
    pub const m_vAngularParams: usize = 0x19C; // Vector4D
    pub const m_vHeightFogColor: usize = 0x1AC; // Color[2]
    pub const m_flFogMaxZ: usize = 0x1B4; // float32
    pub const m_flFogDensity: usize = 0x1B8; // float32[2]
    pub const m_flFogFalloff: usize = 0x1C0; // float32
    pub const m_hFogTexture0: usize = 0x1C8; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_flFogLayer0Rotation: usize = 0x1D0; // float32
    pub const m_flFogLayer0Scale: usize = 0x1D4; // float32
    pub const m_flFoglayer0ScrollU: usize = 0x1D8; // float32[2]
    pub const m_flFoglayer0ScrollV: usize = 0x1E0; // float32[2]
    pub const m_hFogTexture1: usize = 0x1E8; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_flFogLayer1Rotation: usize = 0x1F0; // float32
    pub const m_flFogLayer1Scale: usize = 0x1F4; // float32
    pub const m_flFoglayer1ScrollU: usize = 0x1F8; // float32[2]
    pub const m_flFoglayer1ScrollV: usize = 0x200; // float32[2]
    pub const m_hFogTextureOpacity: usize = 0x208; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_hWaterFlowMapTexture: usize = 0x210; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_hFogFlowMapTexture: usize = 0x218; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_flFogExclusionInnerRadius: usize = 0x220; // float32
    pub const m_flFogExclusionHeightBias: usize = 0x224; // float32
    pub const m_flCausticSpeedScale: usize = 0x22C; // float32
    pub const m_flCausticAmplitudeScale: usize = 0x230; // float32
    pub const m_hColorWarp: usize = 0x238; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_flColorWarpBlendToFull: usize = 0x240; // float32
    pub const m_fInnerRadius: usize = 0x244; // float32
    pub const m_fOuterRadius: usize = 0x248; // float32
    pub const m_flLightning_specular_pow_scale_min: usize = 0x24C; // float32
    pub const m_flLightning_specular_pow_scale_max: usize = 0x250; // float32
    pub const m_lightningColor: usize = 0x254; // Color
    pub const m_flLightningIntensityMin: usize = 0x258; // float32
    pub const m_flLightningIntensityMax: usize = 0x25C; // float32
    pub const m_flLightningElevation: usize = 0x260; // float32
    pub const m_flLightningSpecularIntensity: usize = 0x264; // float32
    pub const m_flFarZOverride: usize = 0x268; // float32
    pub const m_flAmbientShadowAmount: usize = 0x26C; // float32
    pub const m_nWeatherType: usize = 0x270; // int32
    pub const m_WeatherEffect: usize = 0x278; // CUtlString
    pub const m_flLightning_period_min: usize = 0x280; // float32
    pub const m_flLightning_period_max: usize = 0x284; // float32
    pub const m_flLightning_duration_min: usize = 0x288; // float32
    pub const m_flLightning_duration_max: usize = 0x28C; // float32
    pub const m_flLightning_fluctuation_min: usize = 0x290; // float32
    pub const m_flLightning_fluctuation_max: usize = 0x294; // float32
    pub const m_pszLightningSound: usize = 0x298; // char[260]
    pub const m_flNextLightningStartTime: usize = 0x39C; // float32
    pub const m_flNextLightningEndTime: usize = 0x3A0; // float32
    pub const m_flLightningFluctuationTimeStart: usize = 0x3A4; // float32
    pub const m_flLightningFluctuationTimeEnd: usize = 0x3A8; // float32
    pub const m_flLightningNumFluctuations: usize = 0x3AC; // float32
    pub const m_flNextLightningSoundTime: usize = 0x3B0; // float32
    pub const m_bPlayLightingSound: usize = 0x3B4; // bool
    pub const m_flLightningEventMagnitude: usize = 0x3B8; // float32
    pub const m_flLightningScale: usize = 0x3BC; // float32
    pub const m_flLightningFluctuation: usize = 0x3C0; // float32
    pub const m_flLightningAngle: usize = 0x3C4; // float32
    pub const m_flLightningEventPercentage: usize = 0x3C8; // float32
}

pub mod CLogicRelay {
    pub const m_OnTrigger: usize = 0x538; // CEntityIOOutput
    pub const m_OnSpawn: usize = 0x560; // CEntityIOOutput
    pub const m_bDisabled: usize = 0x588; // bool
    pub const m_bWaitForRefire: usize = 0x589; // bool
    pub const m_bTriggerOnce: usize = 0x58A; // bool
    pub const m_bFastRetrigger: usize = 0x58B; // bool
    pub const m_bPassthoughCaller: usize = 0x58C; // bool
}

pub mod CModelState {
    pub const m_hModel: usize = 0xA0; // CStrongHandle< InfoForResourceTypeCModel >
    pub const m_ModelName: usize = 0xA8; // CUtlSymbolLarge
    pub const m_bClientClothCreationSuppressed: usize = 0xE8; // bool
    pub const m_MeshGroupMask: usize = 0x180; // uint64
    pub const m_nIdealMotionType: usize = 0x222; // int8
    pub const m_nForceLOD: usize = 0x223; // int8
    pub const m_nClothUpdateFlags: usize = 0x224; // int8
}

pub mod CNetworkedSequenceOperation {
    pub const m_hSequence: usize = 0x8; // HSequence
    pub const m_flPrevCycle: usize = 0xC; // float32
    pub const m_flCycle: usize = 0x10; // float32
    pub const m_flWeight: usize = 0x14; // CNetworkedQuantizedFloat
    pub const m_bSequenceChangeNetworked: usize = 0x1C; // bool
    pub const m_bDiscontinuity: usize = 0x1D; // bool
    pub const m_flPrevCycleFromDiscontinuity: usize = 0x20; // float32
    pub const m_flPrevCycleForAnimEventDetection: usize = 0x24; // float32
}

pub mod CPlayer_CameraServices {
    pub const m_PlayerFog: usize = 0x40; // C_fogplayerparams_t
    pub const m_hColorCorrectionCtrl: usize = 0x80; // CHandle< C_ColorCorrection >
    pub const m_hViewEntity: usize = 0x84; // CHandle< C_BaseEntity >
    pub const m_hTonemapController: usize = 0x88; // CHandle< C_TonemapController2 >
    pub const m_audio: usize = 0x90; // audioparams_t
    pub const m_PostProcessingVolumes: usize = 0x108; // C_NetworkUtlVectorBase< CHandle< C_PostProcessingVolume > >
    pub const m_flOldPlayerZ: usize = 0x120; // float32
    pub const m_flOldPlayerViewOffsetZ: usize = 0x124; // float32
    pub const m_CurrentFog: usize = 0x128; // fogparams_t
    pub const m_hOldFogController: usize = 0x190; // CHandle< C_FogController >
    pub const m_bOverrideFogColor: usize = 0x194; // bool[5]
    pub const m_OverrideFogColor: usize = 0x199; // Color[5]
    pub const m_bOverrideFogStartEnd: usize = 0x1AD; // bool[5]
    pub const m_fOverrideFogStart: usize = 0x1B4; // float32[5]
    pub const m_fOverrideFogEnd: usize = 0x1C8; // float32[5]
    pub const m_hActivePostProcessingVolume: usize = 0x1DC; // CHandle< C_PostProcessingVolume >
    pub const m_angDemoViewAngles: usize = 0x1E0; // QAngle
}

pub mod CPlayer_MovementServices {
    pub const m_nImpulse: usize = 0x40; // int32
    pub const m_nButtons: usize = 0x48; // CInButtonState
    pub const m_nQueuedButtonDownMask: usize = 0x68; // uint64
    pub const m_nQueuedButtonChangeMask: usize = 0x70; // uint64
    pub const m_nButtonDoublePressed: usize = 0x78; // uint64
    pub const m_pButtonPressedCmdNumber: usize = 0x80; // uint32[64]
    pub const m_nLastCommandNumberProcessed: usize = 0x180; // uint32
    pub const m_nToggleButtonDownMask: usize = 0x188; // uint64
    pub const m_flMaxspeed: usize = 0x190; // float32
    pub const m_arrForceSubtickMoveWhen: usize = 0x194; // float32[4]
    pub const m_flForwardMove: usize = 0x1A4; // float32
    pub const m_flLeftMove: usize = 0x1A8; // float32
    pub const m_flUpMove: usize = 0x1AC; // float32
    pub const m_vecLastMovementImpulses: usize = 0x1B0; // Vector
    pub const m_vecOldViewAngles: usize = 0x1BC; // QAngle
}

pub mod CPlayer_MovementServices_Humanoid {
    pub const m_flStepSoundTime: usize = 0x1D0; // float32
    pub const m_flFallVelocity: usize = 0x1D4; // float32
    pub const m_bInCrouch: usize = 0x1D8; // bool
    pub const m_nCrouchState: usize = 0x1DC; // uint32
    pub const m_flCrouchTransitionStartTime: usize = 0x1E0; // GameTime_t
    pub const m_bDucked: usize = 0x1E4; // bool
    pub const m_bDucking: usize = 0x1E5; // bool
    pub const m_bInDuckJump: usize = 0x1E6; // bool
    pub const m_groundNormal: usize = 0x1E8; // Vector
    pub const m_flSurfaceFriction: usize = 0x1F4; // float32
    pub const m_surfaceProps: usize = 0x1F8; // CUtlStringToken
    pub const m_nStepside: usize = 0x208; // int32
}

pub mod CPlayer_ObserverServices {
    pub const m_iObserverMode: usize = 0x40; // uint8
    pub const m_hObserverTarget: usize = 0x44; // CHandle< C_BaseEntity >
    pub const m_iObserverLastMode: usize = 0x48; // ObserverMode_t
    pub const m_bForcedObserverMode: usize = 0x4C; // bool
    pub const m_flObserverChaseDistance: usize = 0x50; // float32
    pub const m_flObserverChaseDistanceCalcTime: usize = 0x54; // GameTime_t
}

pub mod CPlayer_WeaponServices {
    pub const m_bAllowSwitchToNoWeapon: usize = 0x40; // bool
    pub const m_hMyWeapons: usize = 0x48; // C_NetworkUtlVectorBase< CHandle< C_BasePlayerWeapon > >
    pub const m_hActiveWeapon: usize = 0x60; // CHandle< C_BasePlayerWeapon >
    pub const m_hLastWeapon: usize = 0x64; // CHandle< C_BasePlayerWeapon >
    pub const m_iAmmo: usize = 0x68; // uint16[32]
}

pub mod CPointOffScreenIndicatorUi {
    pub const m_bBeenEnabled: usize = 0xA20; // bool
    pub const m_bHide: usize = 0xA21; // bool
    pub const m_flSeenTargetTime: usize = 0xA24; // float32
    pub const m_pTargetPanel: usize = 0xA28; // C_PointClientUIWorldPanel*
}

pub mod CPointTemplate {
    pub const m_iszWorldName: usize = 0x538; // CUtlSymbolLarge
    pub const m_iszSource2EntityLumpName: usize = 0x540; // CUtlSymbolLarge
    pub const m_iszEntityFilterName: usize = 0x548; // CUtlSymbolLarge
    pub const m_flTimeoutInterval: usize = 0x550; // float32
    pub const m_bAsynchronouslySpawnEntities: usize = 0x554; // bool
    pub const m_pOutputOnSpawned: usize = 0x558; // CEntityIOOutput
    pub const m_clientOnlyEntityBehavior: usize = 0x580; // PointTemplateClientOnlyEntityBehavior_t
    pub const m_ownerSpawnGroupType: usize = 0x584; // PointTemplateOwnerSpawnGroupType_t
    pub const m_createdSpawnGroupHandles: usize = 0x588; // CUtlVector< uint32 >
    pub const m_SpawnedEntityHandles: usize = 0x5A0; // CUtlVector< CEntityHandle >
    pub const m_ScriptSpawnCallback: usize = 0x5B8; // HSCRIPT
    pub const m_ScriptCallbackScope: usize = 0x5C0; // HSCRIPT
}

pub mod CPortraitData {
    pub const m_RenderList: usize = 0xDE0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_hHero: usize = 0xDF8; // CHandle< C_BaseEntity >
}

pub mod CPrecipitationVData {
    pub const m_szParticlePrecipitationEffect: usize = 0x28; // CResourceNameTyped< CWeakHandle< InfoForResourceTypeIParticleSystemDefinition > >
    pub const m_flInnerDistance: usize = 0x108; // float32
    pub const m_nAttachType: usize = 0x10C; // ParticleAttachment_t
    pub const m_bBatchSameVolumeType: usize = 0x110; // bool
    pub const m_nRTEnvCP: usize = 0x114; // int32
    pub const m_nRTEnvCPComponent: usize = 0x118; // int32
    pub const m_szModifier: usize = 0x120; // CUtlString
}

pub mod CProjectedTextureBase {
    pub const m_hTargetEntity: usize = 0xC; // CHandle< C_BaseEntity >
    pub const m_bState: usize = 0x10; // bool
    pub const m_bAlwaysUpdate: usize = 0x11; // bool
    pub const m_flLightFOV: usize = 0x14; // float32
    pub const m_bEnableShadows: usize = 0x18; // bool
    pub const m_bSimpleProjection: usize = 0x19; // bool
    pub const m_bLightOnlyTarget: usize = 0x1A; // bool
    pub const m_bLightWorld: usize = 0x1B; // bool
    pub const m_bCameraSpace: usize = 0x1C; // bool
    pub const m_flBrightnessScale: usize = 0x20; // float32
    pub const m_LightColor: usize = 0x24; // Color
    pub const m_flIntensity: usize = 0x28; // float32
    pub const m_flLinearAttenuation: usize = 0x2C; // float32
    pub const m_flQuadraticAttenuation: usize = 0x30; // float32
    pub const m_bVolumetric: usize = 0x34; // bool
    pub const m_flVolumetricIntensity: usize = 0x38; // float32
    pub const m_flNoiseStrength: usize = 0x3C; // float32
    pub const m_flFlashlightTime: usize = 0x40; // float32
    pub const m_nNumPlanes: usize = 0x44; // uint32
    pub const m_flPlaneOffset: usize = 0x48; // float32
    pub const m_flColorTransitionTime: usize = 0x4C; // float32
    pub const m_flAmbient: usize = 0x50; // float32
    pub const m_SpotlightTextureName: usize = 0x54; // char[512]
    pub const m_nSpotlightTextureFrame: usize = 0x254; // int32
    pub const m_nShadowQuality: usize = 0x258; // uint32
    pub const m_flNearZ: usize = 0x25C; // float32
    pub const m_flFarZ: usize = 0x260; // float32
    pub const m_flProjectionSize: usize = 0x264; // float32
    pub const m_flRotation: usize = 0x268; // float32
    pub const m_bFlipHorizontal: usize = 0x26C; // bool
}

pub mod CRenderComponent {
    pub const __m_pChainEntity: usize = 0x10; // CNetworkVarChainer
    pub const m_bIsRenderingWithViewModels: usize = 0x50; // bool
    pub const m_nSplitscreenFlags: usize = 0x54; // uint32
    pub const m_bEnableRendering: usize = 0x60; // bool
    pub const m_bInterpolationReadyToDraw: usize = 0xB0; // bool
}

pub mod CScriptComponent {
    pub const m_scriptClassName: usize = 0x30; // CUtlSymbolLarge
}

pub mod CSkeletonInstance {
    pub const m_modelState: usize = 0x160; // CModelState
    pub const m_bIsAnimationEnabled: usize = 0x390; // bool
    pub const m_bUseParentRenderBounds: usize = 0x391; // bool
    pub const m_bDisableSolidCollisionsForHierarchy: usize = 0x392; // bool
    pub const m_bDirtyMotionType: usize = 0x0; // bitfield:1
    pub const m_bIsGeneratingLatchedParentSpaceState: usize = 0x0; // bitfield:1
    pub const m_materialGroup: usize = 0x394; // CUtlStringToken
    pub const m_nHitboxSet: usize = 0x398; // uint8
}

pub mod CSkyboxReference {
    pub const m_worldGroupId: usize = 0x538; // WorldGroupId_t
    pub const m_hSkyCamera: usize = 0x53C; // CHandle< C_SkyCamera >
}

pub mod CTimeline {
    pub const m_flValues: usize = 0x10; // float32[64]
    pub const m_nValueCounts: usize = 0x110; // int32[64]
    pub const m_nBucketCount: usize = 0x210; // int32
    pub const m_flInterval: usize = 0x214; // float32
    pub const m_flFinalValue: usize = 0x218; // float32
    pub const m_nCompressionType: usize = 0x21C; // TimelineCompression_t
    pub const m_bStopped: usize = 0x220; // bool
}

pub mod C_BaseAnimatingController {
    pub const m_baseLayer: usize = 0x10; // CNetworkedSequenceOperation
    pub const m_bSequenceFinished: usize = 0x38; // bool
    pub const m_flGroundSpeed: usize = 0x3C; // float32
    pub const m_flLastEventCycle: usize = 0x40; // float32
    pub const m_flLastEventAnimTime: usize = 0x44; // float32
    pub const m_flPlaybackRate: usize = 0x48; // CNetworkedQuantizedFloat
    pub const m_flPrevAnimTime: usize = 0x50; // float32
    pub const m_flPoseParameter: usize = 0x54; // float32[24]
    pub const m_bClientSideAnimation: usize = 0xB4; // bool
    pub const m_bNetworkedAnimationInputsChanged: usize = 0xB5; // bool
    pub const m_nPrevNewSequenceParity: usize = 0xB6; // uint8
    pub const m_nPrevResetEventsParity: usize = 0xB7; // uint8
    pub const m_nNewSequenceParity: usize = 0xB8; // int32
    pub const m_nResetEventsParity: usize = 0xBC; // int32
    pub const m_nAnimLoopMode: usize = 0xC0; // AnimLoopMode_t
    pub const m_SequenceTransitioner: usize = 0xD8; // C_CSequenceTransitioner2
    pub const m_hLastAnimEventSequence: usize = 0x298; // HSequence
    pub const m_ClientSideAnimationListHandle: usize = 0x29C; // AnimationUpdateListHandle_t
}

pub mod C_BaseAnimatingOverlayController {
    pub const m_AnimOverlay: usize = 0x348; // C_UtlVectorEmbeddedNetworkVar< CAnimationLayer >
}

pub mod C_BaseButton {
    pub const m_glowEntity: usize = 0x7B8; // CHandle< C_BaseModelEntity >
    pub const m_usable: usize = 0x7BC; // bool
    pub const m_szDisplayText: usize = 0x7C0; // CUtlSymbolLarge
}

pub mod C_BaseClientUIEntity {
    pub const m_bEnabled: usize = 0x7C0; // bool
    pub const m_DialogXMLName: usize = 0x7C8; // CUtlSymbolLarge
    pub const m_PanelClassName: usize = 0x7D0; // CUtlSymbolLarge
    pub const m_PanelID: usize = 0x7D8; // CUtlSymbolLarge
}

pub mod C_BaseCombatCharacter {
    pub const m_hMyWearables: usize = 0x968; // C_NetworkUtlVectorBase< CHandle< C_EconWearable > >
    pub const m_bloodColor: usize = 0x980; // int32
    pub const m_leftFootAttachment: usize = 0x984; // AttachmentHandle_t
    pub const m_rightFootAttachment: usize = 0x985; // AttachmentHandle_t
    pub const m_nWaterWakeMode: usize = 0x988; // C_BaseCombatCharacter::WaterWakeMode_t
    pub const m_flWaterWorldZ: usize = 0x98C; // float32
    pub const m_flWaterNextTraceTime: usize = 0x990; // float32
    pub const m_flFieldOfView: usize = 0x994; // float32
}

pub mod C_BaseDoor {
    pub const m_bIsUsable: usize = 0x7B8; // bool
}

pub mod C_BaseEntity {
    pub const m_CBodyComponent: usize = 0x30; // CBodyComponent*
    pub const m_NetworkTransmitComponent: usize = 0x38; // CNetworkTransmitComponent
    pub const m_nLastThinkTick: usize = 0x308; // GameTick_t
    pub const m_pGameSceneNode: usize = 0x310; // CGameSceneNode*
    pub const m_pRenderComponent: usize = 0x318; // CRenderComponent*
    pub const m_pCollision: usize = 0x320; // CCollisionProperty*
    pub const m_iMaxHealth: usize = 0x328; // int32
    pub const m_iHealth: usize = 0x32C; // int32
    pub const m_lifeState: usize = 0x330; // uint8
    pub const m_takedamage: usize = 0x331; // DamageOptions_t
    pub const m_bTakesDamage: usize = 0x332; // bool
    pub const m_nTakeDamageFlags: usize = 0x334; // TakeDamageFlags_t
    pub const m_ubInterpolationFrame: usize = 0x338; // uint8
    pub const m_hSceneObjectController: usize = 0x33C; // CHandle< C_BaseEntity >
    pub const m_nNoInterpolationTick: usize = 0x340; // int32
    pub const m_nVisibilityNoInterpolationTick: usize = 0x344; // int32
    pub const m_flProxyRandomValue: usize = 0x348; // float32
    pub const m_iEFlags: usize = 0x34C; // int32
    pub const m_nWaterType: usize = 0x350; // uint8
    pub const m_bInterpolateEvenWithNoModel: usize = 0x351; // bool
    pub const m_bPredictionEligible: usize = 0x352; // bool
    pub const m_bApplyLayerMatchIDToModel: usize = 0x353; // bool
    pub const m_tokLayerMatchID: usize = 0x354; // CUtlStringToken
    pub const m_nSubclassID: usize = 0x358; // CUtlStringToken
    pub const m_nSimulationTick: usize = 0x368; // int32
    pub const m_iCurrentThinkContext: usize = 0x36C; // int32
    pub const m_aThinkFunctions: usize = 0x370; // CUtlVector< thinkfunc_t >
    pub const m_flAnimTime: usize = 0x388; // float32
    pub const m_flSimulationTime: usize = 0x38C; // float32
    pub const m_nSceneObjectOverrideFlags: usize = 0x390; // uint8
    pub const m_bHasSuccessfullyInterpolated: usize = 0x391; // bool
    pub const m_bHasAddedVarsToInterpolation: usize = 0x392; // bool
    pub const m_bRenderEvenWhenNotSuccessfullyInterpolated: usize = 0x393; // bool
    pub const m_nInterpolationLatchDirtyFlags: usize = 0x394; // int32[2]
    pub const m_ListEntry: usize = 0x39C; // uint16[11]
    pub const m_flCreateTime: usize = 0x3B4; // GameTime_t
    pub const m_flSpeed: usize = 0x3B8; // float32
    pub const m_EntClientFlags: usize = 0x3BC; // uint16
    pub const m_bClientSideRagdoll: usize = 0x3BE; // bool
    pub const m_iTeamNum: usize = 0x3BF; // uint8
    pub const m_spawnflags: usize = 0x3C0; // uint32
    pub const m_nNextThinkTick: usize = 0x3C4; // GameTick_t
    pub const m_fFlags: usize = 0x3C8; // uint32
    pub const m_vecAbsVelocity: usize = 0x3CC; // Vector
    pub const m_vecVelocity: usize = 0x3D8; // CNetworkVelocityVector
    pub const m_hEffectEntity: usize = 0x408; // CHandle< C_BaseEntity >
    pub const m_hOwnerEntity: usize = 0x40C; // CHandle< C_BaseEntity >
    pub const m_MoveCollide: usize = 0x410; // MoveCollide_t
    pub const m_MoveType: usize = 0x411; // MoveType_t
    pub const m_flWaterLevel: usize = 0x414; // float32
    pub const m_fEffects: usize = 0x418; // uint32
    pub const m_hGroundEntity: usize = 0x41C; // CHandle< C_BaseEntity >
    pub const m_flFriction: usize = 0x420; // float32
    pub const m_flElasticity: usize = 0x424; // float32
    pub const m_flGravityScale: usize = 0x428; // float32
    pub const m_flTimeScale: usize = 0x42C; // float32
    pub const m_bSimulatedEveryTick: usize = 0x430; // bool
    pub const m_bAnimatedEveryTick: usize = 0x431; // bool
    pub const m_flNavIgnoreUntilTime: usize = 0x434; // GameTime_t
    pub const m_hThink: usize = 0x438; // uint16
    pub const m_fBBoxVisFlags: usize = 0x448; // uint8
    pub const m_bPredictable: usize = 0x449; // bool
    pub const m_bRenderWithViewModels: usize = 0x44A; // bool
    pub const m_nSplitUserPlayerPredictionSlot: usize = 0x44C; // CSplitScreenSlot
    pub const m_nFirstPredictableCommand: usize = 0x450; // int32
    pub const m_nLastPredictableCommand: usize = 0x454; // int32
    pub const m_hOldMoveParent: usize = 0x458; // CHandle< C_BaseEntity >
    pub const m_Particles: usize = 0x460; // CParticleProperty
    pub const m_vecPredictedScriptFloats: usize = 0x488; // CUtlVector< float32 >
    pub const m_vecPredictedScriptFloatIDs: usize = 0x4A0; // CUtlVector< int32 >
    pub const m_nNextScriptVarRecordID: usize = 0x4D0; // int32
    pub const m_vecAngVelocity: usize = 0x4E0; // QAngle
    pub const m_DataChangeEventRef: usize = 0x4EC; // int32
    pub const m_dependencies: usize = 0x4F0; // CUtlVector< CEntityHandle >
    pub const m_nCreationTick: usize = 0x508; // int32
    pub const m_bAnimTimeChanged: usize = 0x521; // bool
    pub const m_bSimulationTimeChanged: usize = 0x522; // bool
    pub const m_sUniqueHammerID: usize = 0x530; // CUtlString
}

pub mod C_BaseFlex {
    pub const m_flexWeight: usize = 0x7F0; // C_NetworkUtlVectorBase< float32 >
    pub const m_vLookTargetPosition: usize = 0x808; // Vector
    pub const m_blinktoggle: usize = 0x820; // bool
    pub const m_nLastFlexUpdateFrameCount: usize = 0x880; // int32
    pub const m_CachedViewTarget: usize = 0x884; // Vector
    pub const m_nNextSceneEventId: usize = 0x890; // uint32
    pub const m_iBlink: usize = 0x894; // int32
    pub const m_blinktime: usize = 0x898; // float32
    pub const m_prevblinktoggle: usize = 0x89C; // bool
    pub const m_iMouthAttachment: usize = 0x89D; // AttachmentHandle_t
    pub const m_iEyeAttachment: usize = 0x89E; // AttachmentHandle_t
    pub const m_bResetFlexWeightsOnModelChange: usize = 0x89F; // bool
    pub const m_nEyeOcclusionRendererBone: usize = 0x8B8; // int32
    pub const m_mEyeOcclusionRendererCameraToBoneTransform: usize = 0x8BC; // matrix3x4_t
    pub const m_vEyeOcclusionRendererHalfExtent: usize = 0x8EC; // Vector
    pub const m_PhonemeClasses: usize = 0x908; // C_BaseFlex::Emphasized_Phoneme[3]
}

pub mod C_BaseFlex_Emphasized_Phoneme {
    pub const m_sClassName: usize = 0x0; // CUtlString
    pub const m_flAmount: usize = 0x18; // float32
    pub const m_bRequired: usize = 0x1C; // bool
    pub const m_bBasechecked: usize = 0x1D; // bool
    pub const m_bValid: usize = 0x1E; // bool
}

pub mod C_BaseModelEntity {
    pub const m_CRenderComponent: usize = 0x538; // CRenderComponent*
    pub const m_CHitboxComponent: usize = 0x540; // CHitboxComponent
    pub const m_bInitModelEffects: usize = 0x588; // bool
    pub const m_bIsStaticProp: usize = 0x589; // bool
    pub const m_iViewerID: usize = 0x58C; // int32
    pub const m_iTeamVisibilityBitmask: usize = 0x590; // int32
    pub const m_nLastAddDecal: usize = 0x594; // int32
    pub const m_nDecalsAdded: usize = 0x598; // int32
    pub const m_iOldHealth: usize = 0x59C; // int32
    pub const m_nRenderMode: usize = 0x5A0; // RenderMode_t
    pub const m_bVisibilityDirtyFlag: usize = 0x5A1; // bool
    pub const m_nRenderFX: usize = 0x5A2; // RenderFx_t
    pub const m_bAllowFadeInView: usize = 0x5A3; // bool
    pub const m_clrRender: usize = 0x5A4; // Color
    pub const m_vecRenderAttributes: usize = 0x5A8; // C_UtlVectorEmbeddedNetworkVar< EntityRenderAttribute_t >
    pub const m_LightGroup: usize = 0x610; // CUtlStringToken
    pub const m_bRenderToCubemaps: usize = 0x614; // bool
    pub const m_Collision: usize = 0x618; // CCollisionProperty
    pub const m_Glow: usize = 0x6C8; // CGlowProperty
    pub const m_flGlowBackfaceMult: usize = 0x720; // float32
    pub const m_fadeMinDist: usize = 0x724; // float32
    pub const m_fadeMaxDist: usize = 0x728; // float32
    pub const m_flFadeScale: usize = 0x72C; // float32
    pub const m_flShadowStrength: usize = 0x730; // float32
    pub const m_nObjectCulling: usize = 0x734; // uint8
    pub const m_nAddDecal: usize = 0x738; // int32
    pub const m_vDecalPosition: usize = 0x73C; // Vector
    pub const m_vDecalForwardAxis: usize = 0x748; // Vector
    pub const m_flDecalHealBloodRate: usize = 0x754; // float32
    pub const m_flDecalHealHeightRate: usize = 0x758; // float32
    pub const m_ConfigEntitiesToPropagateMaterialDecalsTo: usize = 0x760; // C_NetworkUtlVectorBase< CHandle< C_BaseModelEntity > >
    pub const m_vecViewOffset: usize = 0x778; // CNetworkViewOffsetVector
    pub const m_pClientAlphaProperty: usize = 0x7A8; // CClientAlphaProperty*
    pub const m_ClientOverrideTint: usize = 0x7B0; // Color
    pub const m_bUseClientOverrideTint: usize = 0x7B4; // bool
}

pub mod C_BasePlayerPawn {
    pub const m_pWeaponServices: usize = 0x9F8; // CPlayer_WeaponServices*
    pub const m_pItemServices: usize = 0xA00; // CPlayer_ItemServices*
    pub const m_pAutoaimServices: usize = 0xA08; // CPlayer_AutoaimServices*
    pub const m_pObserverServices: usize = 0xA10; // CPlayer_ObserverServices*
    pub const m_pWaterServices: usize = 0xA18; // CPlayer_WaterServices*
    pub const m_pUseServices: usize = 0xA20; // CPlayer_UseServices*
    pub const m_pFlashlightServices: usize = 0xA28; // CPlayer_FlashlightServices*
    pub const m_pCameraServices: usize = 0xA30; // CPlayer_CameraServices*
    pub const m_pMovementServices: usize = 0xA38; // CPlayer_MovementServices*
    pub const v_angle: usize = 0xA48; // QAngle
    pub const v_anglePrevious: usize = 0xA54; // QAngle
    pub const m_iHideHUD: usize = 0xA60; // uint32
    pub const m_skybox3d: usize = 0xA68; // sky3dparams_t
    pub const m_flDeathTime: usize = 0xAF8; // GameTime_t
    pub const m_vecPredictionError: usize = 0xAFC; // Vector
    pub const m_flPredictionErrorTime: usize = 0xB08; // GameTime_t
    pub const m_flFOVSensitivityAdjust: usize = 0xB0C; // float32
    pub const m_flMouseSensitivity: usize = 0xB10; // float32
    pub const m_vOldOrigin: usize = 0xB14; // Vector
    pub const m_flOldSimulationTime: usize = 0xB20; // float32
    pub const m_nLastExecutedCommandNumber: usize = 0xB24; // int32
    pub const m_nLastExecutedCommandTick: usize = 0xB28; // int32
    pub const m_hController: usize = 0xB2C; // CHandle< CBasePlayerController >
    pub const m_bIsSwappingToPredictableController: usize = 0xB30; // bool
}

pub mod C_BasePlayerWeapon {
    pub const m_nNextPrimaryAttackTick: usize = 0x7E0; // GameTick_t
    pub const m_flNextPrimaryAttackTickRatio: usize = 0x7E4; // float32
    pub const m_nNextSecondaryAttackTick: usize = 0x7E8; // GameTick_t
    pub const m_flNextSecondaryAttackTickRatio: usize = 0x7EC; // float32
    pub const m_iClip1: usize = 0x7F0; // int32
    pub const m_iClip2: usize = 0x7F4; // int32
    pub const m_pReserveAmmo: usize = 0x7F8; // int32[2]
}

pub mod C_BasePropDoor {
    pub const m_eDoorState: usize = 0xA68; // DoorState_t
    pub const m_modelChanged: usize = 0xA6C; // bool
    pub const m_bLocked: usize = 0xA6D; // bool
    pub const m_closedPosition: usize = 0xA70; // Vector
    pub const m_closedAngles: usize = 0xA7C; // QAngle
    pub const m_hMaster: usize = 0xA88; // CHandle< C_BasePropDoor >
    pub const m_vWhereToSetLightingOrigin: usize = 0xA8C; // Vector
}

pub mod C_BaseTrigger {
    pub const m_bDisabled: usize = 0x7B8; // bool
    pub const m_bClientSidePredicted: usize = 0x7B9; // bool
}

pub mod C_Beam {
    pub const m_flFrameRate: usize = 0x7B8; // float32
    pub const m_flHDRColorScale: usize = 0x7BC; // float32
    pub const m_flFireTime: usize = 0x7C0; // GameTime_t
    pub const m_flDamage: usize = 0x7C4; // float32
    pub const m_nNumBeamEnts: usize = 0x7C8; // uint8
    pub const m_queryHandleHalo: usize = 0x7CC; // int32
    pub const m_hBaseMaterial: usize = 0x7F0; // CStrongHandle< InfoForResourceTypeIMaterial2 >
    pub const m_nHaloIndex: usize = 0x7F8; // CStrongHandle< InfoForResourceTypeIMaterial2 >
    pub const m_nBeamType: usize = 0x800; // BeamType_t
    pub const m_nBeamFlags: usize = 0x804; // uint32
    pub const m_hAttachEntity: usize = 0x808; // CHandle< C_BaseEntity >[10]
    pub const m_nAttachIndex: usize = 0x830; // AttachmentHandle_t[10]
    pub const m_fWidth: usize = 0x83C; // float32
    pub const m_fEndWidth: usize = 0x840; // float32
    pub const m_fFadeLength: usize = 0x844; // float32
    pub const m_fHaloScale: usize = 0x848; // float32
    pub const m_fAmplitude: usize = 0x84C; // float32
    pub const m_fStartFrame: usize = 0x850; // float32
    pub const m_fSpeed: usize = 0x854; // float32
    pub const m_flFrame: usize = 0x858; // float32
    pub const m_nClipStyle: usize = 0x85C; // BeamClipStyle_t
    pub const m_bTurnedOff: usize = 0x860; // bool
    pub const m_vecEndPos: usize = 0x864; // Vector
    pub const m_hEndEntity: usize = 0x870; // CHandle< C_BaseEntity >
}

pub mod C_BodyComponentBaseAnimating {
    pub const m_animationController: usize = 0x470; // C_BaseAnimatingController
    pub const __m_pChainEntity: usize = 0x7B8; // CNetworkVarChainer
}

pub mod C_BodyComponentBaseAnimatingOverlay {
    pub const m_animationController: usize = 0x470; // C_BaseAnimatingOverlayController
    pub const __m_pChainEntity: usize = 0x828; // CNetworkVarChainer
}

pub mod C_BreakableProp {
    pub const m_OnBreak: usize = 0x828; // CEntityIOOutput
    pub const m_OnHealthChanged: usize = 0x850; // CEntityOutputTemplate< float32 >
    pub const m_OnTakeDamage: usize = 0x878; // CEntityIOOutput
    pub const m_impactEnergyScale: usize = 0x8A0; // float32
    pub const m_iMinHealthDmg: usize = 0x8A4; // int32
    pub const m_flPressureDelay: usize = 0x8A8; // float32
    pub const m_hBreaker: usize = 0x8AC; // CHandle< C_BaseEntity >
    pub const m_PerformanceMode: usize = 0x8B0; // PerformanceMode_t
    pub const m_flDmgModBullet: usize = 0x8B4; // float32
    pub const m_flDmgModClub: usize = 0x8B8; // float32
    pub const m_flDmgModExplosive: usize = 0x8BC; // float32
    pub const m_flDmgModFire: usize = 0x8C0; // float32
    pub const m_iszPhysicsDamageTableName: usize = 0x8C8; // CUtlSymbolLarge
    pub const m_iszBasePropData: usize = 0x8D0; // CUtlSymbolLarge
    pub const m_iInteractions: usize = 0x8D8; // int32
    pub const m_flPreventDamageBeforeTime: usize = 0x8DC; // GameTime_t
    pub const m_bHasBreakPiecesOrCommands: usize = 0x8E0; // bool
    pub const m_explodeDamage: usize = 0x8E4; // float32
    pub const m_explodeRadius: usize = 0x8E8; // float32
    pub const m_explosionDelay: usize = 0x8F0; // float32
    pub const m_explosionBuildupSound: usize = 0x8F8; // CUtlSymbolLarge
    pub const m_explosionCustomEffect: usize = 0x900; // CUtlSymbolLarge
    pub const m_explosionCustomSound: usize = 0x908; // CUtlSymbolLarge
    pub const m_explosionModifier: usize = 0x910; // CUtlSymbolLarge
    pub const m_hPhysicsAttacker: usize = 0x918; // CHandle< C_BasePlayerPawn >
    pub const m_flLastPhysicsInfluenceTime: usize = 0x91C; // GameTime_t
    pub const m_flDefaultFadeScale: usize = 0x920; // float32
    pub const m_hLastAttacker: usize = 0x924; // CHandle< C_BaseEntity >
    pub const m_hFlareEnt: usize = 0x928; // CHandle< C_BaseEntity >
    pub const m_noGhostCollision: usize = 0x92C; // bool
}

pub mod C_CSequenceTransitioner2 {
    pub const m_currentOp: usize = 0x8; // CNetworkedSequenceOperation
    pub const m_flCurrentPlaybackRate: usize = 0x30; // float32
    pub const m_flCurrentAnimTime: usize = 0x34; // float32
    pub const m_transitioningLayers: usize = 0x38; // TransitioningLayer_t[4]
    pub const m_pOwner: usize = 0x158; // C_BaseAnimatingController*
}

pub mod C_ClientRagdoll {
    pub const m_bFadeOut: usize = 0x978; // bool
    pub const m_bImportant: usize = 0x979; // bool
    pub const m_flEffectTime: usize = 0x97C; // GameTime_t
    pub const m_gibDespawnTime: usize = 0x980; // GameTime_t
    pub const m_iCurrentFriction: usize = 0x984; // int32
    pub const m_iMinFriction: usize = 0x988; // int32
    pub const m_iMaxFriction: usize = 0x98C; // int32
    pub const m_iFrictionAnimState: usize = 0x990; // int32
    pub const m_bReleaseRagdoll: usize = 0x994; // bool
    pub const m_iEyeAttachment: usize = 0x995; // AttachmentHandle_t
    pub const m_bFadingOut: usize = 0x996; // bool
    pub const m_flScaleEnd: usize = 0x998; // float32[10]
    pub const m_flScaleTimeStart: usize = 0x9C0; // GameTime_t[10]
    pub const m_flScaleTimeEnd: usize = 0x9E8; // GameTime_t[10]
}

pub mod C_ColorCorrection {
    pub const m_vecOrigin: usize = 0x538; // Vector
    pub const m_MinFalloff: usize = 0x544; // float32
    pub const m_MaxFalloff: usize = 0x548; // float32
    pub const m_flFadeInDuration: usize = 0x54C; // float32
    pub const m_flFadeOutDuration: usize = 0x550; // float32
    pub const m_flMaxWeight: usize = 0x554; // float32
    pub const m_flCurWeight: usize = 0x558; // float32
    pub const m_netlookupFilename: usize = 0x55C; // char[512]
    pub const m_bEnabled: usize = 0x75C; // bool
    pub const m_bMaster: usize = 0x75D; // bool
    pub const m_bClientSide: usize = 0x75E; // bool
    pub const m_bExclusive: usize = 0x75F; // bool
    pub const m_bEnabledOnClient: usize = 0x760; // bool[1]
    pub const m_flCurWeightOnClient: usize = 0x764; // float32[1]
    pub const m_bFadingIn: usize = 0x768; // bool[1]
    pub const m_flFadeStartWeight: usize = 0x76C; // float32[1]
    pub const m_flFadeStartTime: usize = 0x770; // float32[1]
    pub const m_flFadeDuration: usize = 0x774; // float32[1]
}

pub mod C_ColorCorrectionVolume {
    pub const m_LastEnterWeight: usize = 0x7C0; // float32
    pub const m_LastEnterTime: usize = 0x7C4; // float32
    pub const m_LastExitWeight: usize = 0x7C8; // float32
    pub const m_LastExitTime: usize = 0x7CC; // float32
    pub const m_bEnabled: usize = 0x7D0; // bool
    pub const m_MaxWeight: usize = 0x7D4; // float32
    pub const m_FadeDuration: usize = 0x7D8; // float32
    pub const m_Weight: usize = 0x7DC; // float32
    pub const m_lookupFilename: usize = 0x7E0; // char[512]
}

pub mod C_CommandContext {
    pub const needsprocessing: usize = 0x0; // bool
    pub const command_number: usize = 0x70; // int32
}

pub mod C_DOTAAmbientCreatureParticleZone {
    pub const m_szModelName: usize = 0x7B8; // char[64]
    pub const m_szAreaName: usize = 0x7F8; // char[256]
}

pub mod C_DOTAAppCheers {
    pub const m_nRadiantCheers: usize = 0x538; // int16
    pub const m_nRadiantBronzeCheers: usize = 0x53A; // int16
    pub const m_nRadiantSilverCheers: usize = 0x53C; // int16
    pub const m_nRadiantGoldCheers: usize = 0x53E; // int16
    pub const m_nRadiantCrowdLevel: usize = 0x540; // ECrowdLevel
    pub const m_nDireCheers: usize = 0x544; // int16
    pub const m_nDireBronzeCheers: usize = 0x546; // int16
    pub const m_nDireSilverCheers: usize = 0x548; // int16
    pub const m_nDireGoldCheers: usize = 0x54A; // int16
    pub const m_nDireCrowdLevel: usize = 0x54C; // ECrowdLevel
}

pub mod C_DOTABaseAbility {
    pub const m_bAuxCastState: usize = 0x540; // bool
    pub const m_iEnemyLevel: usize = 0x544; // int32
    pub const m_iMaxLevel: usize = 0x548; // int32
    pub const m_bCanLearn: usize = 0x54C; // bool
    pub const m_flUpgradeBlend: usize = 0x550; // float32
    pub const m_bUpgradedDuringQuickcastPreview: usize = 0x554; // bool
    pub const m_bConsiderOvershootInGetCastRange: usize = 0x555; // bool
    pub const m_bRefCountsModifiers: usize = 0x556; // bool
    pub const m_bHidden: usize = 0x557; // bool
    pub const m_bOldHidden: usize = 0x558; // bool
    pub const m_bActivated: usize = 0x559; // bool
    pub const m_bOldActivated: usize = 0x55A; // bool
    pub const m_nAbilityBarType: usize = 0x55C; // AbilityBarType_t
    pub const m_iDirtyButtons: usize = 0x560; // int32
    pub const m_bPerformDirtyParity: usize = 0x564; // bool
    pub const m_iLevel: usize = 0x568; // int32
    pub const m_bAbilityLevelDirty: usize = 0x56C; // bool
    pub const m_bToggleState: usize = 0x56D; // bool
    pub const m_flAbilityMuteDuration: usize = 0x570; // float32
    pub const m_bInAbilityPhase: usize = 0x574; // bool
    pub const m_fCooldown: usize = 0x578; // float32
    pub const m_flCooldownLength: usize = 0x57C; // float32
    pub const m_iManaCost: usize = 0x580; // int32
    pub const m_bAutoCastState: usize = 0x584; // bool
    pub const m_bAltCastState: usize = 0x585; // bool
    pub const m_flChannelStartTime: usize = 0x588; // GameTime_t
    pub const m_flCastStartTime: usize = 0x58C; // GameTime_t
    pub const m_bInIndefiniteCooldown: usize = 0x590; // bool
    pub const m_bFrozenCooldown: usize = 0x591; // bool
    pub const m_flOverrideCastPoint: usize = 0x594; // float32
    pub const m_bStolen: usize = 0x598; // bool
    pub const m_bStealable: usize = 0x599; // bool
    pub const m_bReplicated: usize = 0x59A; // bool
    pub const m_nAbilityCurrentCharges: usize = 0x59C; // int32
    pub const m_fAbilityChargeRestoreTimeRemaining: usize = 0x5A0; // float32
    pub const m_bUpgradeRecommended: usize = 0x5A4; // bool
    pub const m_nMaxLevelOverride: usize = 0x5A8; // int32
    pub const m_flLastCastClickTime: usize = 0x5AC; // float32
}

pub mod C_DOTABaseGameMode {
    pub const m_ForcedHUDSkin: usize = 0x538; // CUtlString
    pub const m_nCustomGameForceHeroSelectionId: usize = 0x540; // int32
    pub const m_bAlwaysShowPlayerInventory: usize = 0x544; // bool
    pub const m_bGoldSoundDisabled: usize = 0x545; // bool
    pub const m_bRecommendedItemsDisabled: usize = 0x546; // bool
    pub const m_bStickyItemDisabled: usize = 0x547; // bool
    pub const m_bStashPurchasingDisabled: usize = 0x548; // bool
    pub const m_bFogOfWarDisabled: usize = 0x549; // bool
    pub const m_bUseUnseenFOW: usize = 0x54A; // bool
    pub const m_bUseCustomBuybackCost: usize = 0x54B; // bool
    pub const m_bUseCustomBuybackCooldown: usize = 0x54C; // bool
    pub const m_bBuybackEnabled: usize = 0x54D; // bool
    pub const m_bUseTurboCouriers: usize = 0x54E; // bool
    pub const m_flCameraDistanceOverride: usize = 0x550; // float32
    pub const m_nCameraSmoothCountOverride: usize = 0x554; // int32
    pub const m_hOverrideSelectionEntity: usize = 0x558; // CHandle< C_DOTA_BaseNPC >
    pub const m_bTopBarTeamValuesOverride: usize = 0x55C; // bool
    pub const m_bTopBarTeamValuesVisible: usize = 0x55D; // bool
    pub const m_nTeamGoodGuysTopBarValue: usize = 0x560; // int32
    pub const m_nTeamBadGuysTopBarValue: usize = 0x564; // int32
    pub const m_bAlwaysShowPlayerNames: usize = 0x568; // bool
    pub const m_bUseCustomHeroLevels: usize = 0x569; // bool
    pub const m_nCustomXPRequiredToReachNextLevel: usize = 0x570; // C_NetworkUtlVectorBase< int32 >
    pub const m_bTowerBackdoorProtectionEnabled: usize = 0x588; // bool
    pub const m_bBotThinkingEnabled: usize = 0x589; // bool
    pub const m_bAnnouncerDisabled: usize = 0x58A; // bool
    pub const m_bAnnouncerGameModeAnnounceDisabled: usize = 0x58B; // bool
    pub const m_bDeathTipsDisabled: usize = 0x58C; // bool
    pub const m_bFilterPlayerHeroAvailability: usize = 0x58D; // bool
    pub const m_bKillingSpreeAnnouncerDisabled: usize = 0x58E; // bool
    pub const m_flFixedRespawnTime: usize = 0x590; // float32
    pub const m_flBuybackCostScale: usize = 0x594; // float32
    pub const m_flRespawnTimeScale: usize = 0x598; // float32
    pub const m_bLoseGoldOnDeath: usize = 0x59C; // bool
    pub const m_bKillableTombstones: usize = 0x59D; // bool
    pub const m_nHUDVisibilityBits: usize = 0x5A0; // uint32
    pub const m_flMinimumAttackSpeed: usize = 0x5A4; // float32
    pub const m_flMaximumAttackSpeed: usize = 0x5A8; // float32
    pub const m_bIsDaynightCycleDisabled: usize = 0x5AC; // bool
    pub const m_flDaynightCycleAdvanceRate: usize = 0x5B0; // float32
    pub const m_bAreWeatherEffectsDisabled: usize = 0x5B4; // bool
    pub const m_bDisableHudFlip: usize = 0x5B5; // bool
    pub const m_bEnableFriendlyBuildingMoveTo: usize = 0x5B6; // bool
    pub const m_bIsDeathOverlayDisabled: usize = 0x5B7; // bool
    pub const m_bIsHudCombatEventsDisabled: usize = 0x5B8; // bool
    pub const m_strDefaultStickyItem: usize = 0x5C0; // CUtlString
    pub const m_sCustomTerrainWeatherEffect: usize = 0x5C8; // CUtlString
    pub const m_strTPScrollSlotItemOverride: usize = 0x5D0; // CUtlString
    pub const m_flStrengthDamage: usize = 0x5D8; // float32
    pub const m_flStrengthHP: usize = 0x5DC; // float32
    pub const m_flStrengthHPRegen: usize = 0x5E0; // float32
    pub const m_flAgilityDamage: usize = 0x5E4; // float32
    pub const m_flAgilityArmor: usize = 0x5E8; // float32
    pub const m_flAgilityAttackSpeed: usize = 0x5EC; // float32
    pub const m_flAgilityMovementSpeedPercent: usize = 0x5F0; // float32
    pub const m_flIntelligenceDamage: usize = 0x5F4; // float32
    pub const m_flIntelligenceMana: usize = 0x5F8; // float32
    pub const m_flIntelligenceManaRegen: usize = 0x5FC; // float32
    pub const m_flIntelligenceMres: usize = 0x600; // float32
    pub const m_flIntelligenceSpellAmpPercent: usize = 0x604; // float32
    pub const m_flStrengthMagicResistancePercent: usize = 0x608; // float32
    pub const m_flAttributeAllDamage: usize = 0x60C; // float32
    pub const m_flDraftingHeroPickSelectTimeOverride: usize = 0x610; // float32
    pub const m_flDraftingBanningTimeOverride: usize = 0x614; // float32
    pub const m_bPauseEnabled: usize = 0x618; // bool
    pub const m_iCustomScanMaxCharges: usize = 0x61C; // int32
    pub const m_flCustomScanCooldown: usize = 0x620; // float32
    pub const m_flCustomGlyphCooldown: usize = 0x624; // float32
    pub const m_flCustomBackpackSwapCooldown: usize = 0x628; // float32
    pub const m_flCustomBackpackCooldownPercent: usize = 0x62C; // float32
    pub const m_bDefaultRuneSpawnLogic: usize = 0x630; // bool
    pub const m_bEnableFreeCourierMode: usize = 0x631; // bool
    pub const m_bAllowNeutralItemDrops: usize = 0x632; // bool
    pub const m_bEnableNeutralStash: usize = 0x633; // bool
    pub const m_bEnableNeutralStashTeamViewOnly: usize = 0x634; // bool
    pub const m_bEnableNeutralItemHideUndiscovered: usize = 0x635; // bool
    pub const m_bEnableSendToStash: usize = 0x636; // bool
    pub const m_bForceRightClickAttackDisabled: usize = 0x637; // bool
    pub const m_vecCustomShopInfo: usize = 0x638; // C_UtlVectorEmbeddedNetworkVar< CDOTACustomShopInfo >
    pub const m_bCanSellAnywhere: usize = 0x688; // bool
    pub const m_flCameraNearZ: usize = 0x68C; // float32
    pub const m_flCameraFarZ: usize = 0x690; // float32
    pub const m_nCustomRadiantScore: usize = 0x694; // int32
    pub const m_nCustomDireScore: usize = 0x698; // int32
    pub const m_bAbilityUpgradeWhitelistEnabled: usize = 0x69C; // bool
    pub const m_vecAbilityUpgradeWhitelist: usize = 0x6A0; // C_NetworkUtlVectorBase< AbilityID_t >
    pub const m_bGiveFreeTPOnDeath: usize = 0x6B8; // bool
    pub const m_nInnateMeleeDamageBlockPct: usize = 0x6BC; // int32
    pub const m_nInnateMeleeDamageBlockAmount: usize = 0x6C0; // int32
    pub const m_nInnateMeleeDamageBlockPerLevelAmount: usize = 0x6C4; // int32
    pub const m_flWaterRuneSpawnInterval: usize = 0x6C8; // float32
    pub const m_nHUDVisibilityBitsPrevious: usize = 0x6CC; // uint32
}

pub mod C_DOTACameraBounds {
    pub const m_vecBoundsMin: usize = 0x538; // Vector
    pub const m_vecBoundsMax: usize = 0x544; // Vector
}

pub mod C_DOTACheers {
    pub const m_nRadiantCheers: usize = 0x538; // int16
    pub const m_nDireCheers: usize = 0x53A; // int16
    pub const m_nRadiantPremiumCheers: usize = 0x53C; // int16
    pub const m_nDirePremiumCheers: usize = 0x53E; // int16
    pub const m_nRadiantCrowdLevel: usize = 0x540; // ECrowdLevel
    pub const m_nDireCrowdLevel: usize = 0x544; // ECrowdLevel
}

pub mod C_DOTACustomGameHeroPickRules {
    pub const m_Phase: usize = 0x538; // DOTACustomHeroPickRulesPhase_t
    pub const m_nNumBansPerTeam: usize = 0x53C; // int32
    pub const m_flEnterTime: usize = 0x540; // GameTime_t
    pub const m_nNumHeroesPicked: usize = 0x544; // int32
}

pub mod C_DOTAFogOfWarTempViewers {
    pub const m_FoWTempViewerVersion: usize = 0x538; // uint32
    pub const m_TempViewerInfo: usize = 0x540; // C_UtlVectorEmbeddedNetworkVar< TempViewerInfo_t >
    pub const m_dota_spectator_fog_of_war_last: usize = 0x590; // int32
}

pub mod C_DOTAGameManager {
    pub const __m_pChainEntity: usize = 0x30; // CNetworkVarChainer
    pub const m_bCustomGame: usize = 0x3D0; // bool
    pub const m_bEventGame: usize = 0x3D1; // bool
    pub const m_bGameModeWantsDefaultNeutralItemSchema: usize = 0x3D2; // bool
    pub const m_bGameModeFilteredAbilities: usize = 0x3D3; // bool
    pub const m_szAddOnGame: usize = 0x3D4; // char[128]
    pub const m_szAddOnMap: usize = 0x454; // char[128]
    pub const m_pTutorialLessonKeyValues: usize = 0x838; // KeyValues*
    pub const m_pDivisionKeyValues: usize = 0x840; // KeyValues*
    pub const m_pMatchGroupsKeyValues: usize = 0x848; // KeyValues*
    pub const m_pAnimationStatues: usize = 0x850; // KeyValues*
    pub const m_pBotScriptsDedicatedServer: usize = 0x858; // KeyValues*
    pub const m_pkvWardPlacementLocations: usize = 0x860; // KeyValues*
    pub const m_pRegionKeyValues: usize = 0x868; // KeyValues*
    pub const m_pSurveyQuestionData: usize = 0x870; // KeyValues*
    pub const m_AddonInfoKeyValues: usize = 0x878; // KeyValues3
    pub const m_pCountryKeyValues: usize = 0x8B0; // KeyValues*
    pub const m_bLoadedPortraits: usize = 0xE18; // bool[8]
    pub const m_pControlGroupsKeyValues: usize = 0xE20; // KeyValues*
    pub const m_CurrentHeroAvailable: usize = 0xE28; // bool[256]
}

pub mod C_DOTAGameManagerProxy {
    pub const m_pGameManager: usize = 0x538; // C_DOTAGameManager*
}

pub mod C_DOTAGamerules {
    pub const __m_pChainEntity: usize = 0x8; // CNetworkVarChainer
    pub const m_nTotalPausedTicks: usize = 0x30; // int32
    pub const m_nPauseStartTick: usize = 0x34; // int32
    pub const m_bGamePaused: usize = 0x38; // bool
    pub const m_iMiscHeroPickCounter: usize = 0x3C; // int32
    pub const m_hEndGameCinematicEntity: usize = 0x40; // CHandle< C_BaseEntity >
    pub const m_hOverlayHealthBarUnit: usize = 0x44; // CHandle< C_DOTA_BaseNPC >
    pub const m_nOverlayHealthBarType: usize = 0x48; // int32
    pub const m_bIsInCinematicMode: usize = 0x4C; // bool
    pub const m_bIsInClientSideCinematicMode: usize = 0x4D; // bool
    pub const m_bFreeCourierMode: usize = 0x4E; // bool
    pub const m_nStartingGold: usize = 0x50; // int32
    pub const m_nGoldPerTick: usize = 0x54; // int32
    pub const m_flGoldTickTime: usize = 0x58; // float32
    pub const m_bItemWhiteListChanged: usize = 0x5C; // bool
    pub const m_bEnableSuggestAbilities: usize = 0x5D; // bool
    pub const m_bEnableSuggestItems: usize = 0x5E; // bool
    pub const m_unFanfareGoodGuys: usize = 0x60; // uint32
    pub const m_unFanfareBadGuys: usize = 0x64; // uint32
    pub const m_iMapType: usize = 0x68; // int32
    pub const m_nServerGameState: usize = 0x6C; // int32
    pub const m_nServerHeroPickState: usize = 0x70; // DOTA_HeroPickState
    pub const m_nGameState: usize = 0x74; // int32
    pub const m_nHeroPickState: usize = 0x78; // DOTA_HeroPickState
    pub const m_bAlternateHeroGridsEnabled: usize = 0x7C; // bool
    pub const m_flStateTransitionTime: usize = 0x80; // GameTime_t
    pub const m_flOverride_dota_hero_selection_time: usize = 0x84; // float32
    pub const m_flOverride_dota_pregame_time: usize = 0x88; // float32
    pub const m_flOverride_dota_postgame_time: usize = 0x8C; // float32
    pub const m_flOverride_dota_strategy_time: usize = 0x90; // float32
    pub const m_flOverride_dota_team_showcase_duration: usize = 0x94; // float32
    pub const m_flOverride_dota_rune_spawn_time: usize = 0x98; // float32
    pub const m_flShowcaseTime: usize = 0x9C; // float32[15]
    pub const m_flNeutralInitialSpawnOffset: usize = 0xD8; // float32
    pub const m_iGameMode: usize = 0xDC; // int32
    pub const m_hGameModeEntity: usize = 0xE0; // CHandle< C_BaseEntity >
    pub const m_hCustomHeroPickRulesEntity: usize = 0xE4; // CHandle< C_BaseEntity >
    pub const m_flHeroPickStateTransitionTime: usize = 0xE8; // GameTime_t
    pub const m_iPlayerIDsInControl: usize = 0xF0; // uint64
    pub const m_bSameHeroSelectionEnabled: usize = 0xF8; // bool
    pub const m_bUseCustomHeroXPValue: usize = 0xF9; // bool
    pub const m_bUseBaseGoldBountyOnHeroes: usize = 0xFA; // bool
    pub const m_bUseUniversalShopMode: usize = 0xFB; // bool
    pub const m_bHideKillMessageHeaders: usize = 0xFC; // bool
    pub const m_flHeroMinimapIconScale: usize = 0x100; // float32
    pub const m_flCreepMinimapIconScale: usize = 0x104; // float32
    pub const m_bCreepSpawningEnabled: usize = 0x108; // bool
    pub const m_flRuneMinimapIconScale: usize = 0x10C; // float32
    pub const m_CustomVictoryMessage: usize = 0x110; // char[256]
    pub const m_flCustomGameEndDelay: usize = 0x210; // float32
    pub const m_flCustomGameSetupAutoLaunchDelay: usize = 0x214; // float32
    pub const m_flCustomGameSetupTimeout: usize = 0x218; // float32
    pub const m_flCustomVictoryMessageDuration: usize = 0x21C; // float32
    pub const m_flHeroSelectPenaltyTime: usize = 0x220; // float32
    pub const m_bCustomGameSetupAutoLaunchEnabled: usize = 0x224; // bool
    pub const m_bCustomGameTeamSelectionLocked: usize = 0x225; // bool
    pub const m_bCustomGameEnablePickRules: usize = 0x226; // bool
    pub const m_bCustomGameAllowHeroPickMusic: usize = 0x227; // bool
    pub const m_bCustomGameAllowMusicAtGameStart: usize = 0x228; // bool
    pub const m_bCustomGameAllowBattleMusic: usize = 0x229; // bool
    pub const m_bCustomGameDisableIK: usize = 0x22A; // bool
    pub const m_bCustomGameAllowSecondaryAbilitiesOnOtherUnits: usize = 0x22B; // bool
    pub const m_iCMModePickBanOrder: usize = 0x22C; // int32
    pub const m_iCDModePickBanOrder: usize = 0x230; // int32
    pub const m_nGGTeam: usize = 0x234; // int32
    pub const m_flGGEndsAtTime: usize = 0x238; // GameTime_t
    pub const m_bGGCalled: usize = 0x23C; // bool
    pub const m_bWhiteListEnabled: usize = 0x23D; // bool
    pub const m_bItemWhiteList: usize = 0x240; // uint64[4]
    pub const m_nLastHitUIMode: usize = 0x260; // int32
    pub const m_bHUDTimerTutorialMode: usize = 0x264; // bool
    pub const m_HeroPickMiscTimer: usize = 0x268; // CountdownTimer
    pub const m_ExtraTimeTimer: usize = 0x280; // CountdownTimer
    pub const m_fExtraTimeRemaining: usize = 0x298; // float32[2]
    pub const m_bHeroRespawnEnabled: usize = 0x2A0; // bool
    pub const m_bIsRandomingEnabled: usize = 0x2A1; // bool
    pub const m_iCaptainPlayerIDs: usize = 0x2A4; // PlayerID_t[2]
    pub const m_BannedHeroes: usize = 0x2AC; // int32[24]
    pub const m_SelectedHeroes: usize = 0x30C; // int32[24]
    pub const m_iActiveTeam: usize = 0x36C; // int32
    pub const m_iStartingTeam: usize = 0x370; // int32
    pub const m_iPenaltyLevelRadiant: usize = 0x374; // int32
    pub const m_iPenaltyLevelDire: usize = 0x378; // int32
    pub const m_bIsLoadingScenario: usize = 0x37C; // bool
    pub const m_vecNewBannedHeroes: usize = 0x380; // C_NetworkUtlVectorBase< int32 >
    pub const m_vecWhitelistedHeroes: usize = 0x398; // C_NetworkUtlVectorBase< int32 >
    pub const m_vecBlacklistedHeroes: usize = 0x3B0; // C_NetworkUtlVectorBase< int32 >
    pub const m_bHideBlacklistedHeroes: usize = 0x3C8; // bool
    pub const m_nSeriesType: usize = 0x3CC; // int32
    pub const m_nRadiantSeriesWins: usize = 0x3D0; // int32
    pub const m_nDireSeriesWins: usize = 0x3D4; // int32
    pub const m_vecAvailableHerosPerPlayerID: usize = 0x3D8; // C_UtlVectorEmbeddedNetworkVar< CHeroesPerPlayer >
    pub const m_vecLockedHerosByPlayerID: usize = 0x428; // C_UtlVectorEmbeddedNetworkVar< CHeroesPerPlayer >
    pub const m_vecDisabledRandomHerosByPlayerID: usize = 0x478; // C_UtlVectorEmbeddedNetworkVar< CHeroesPerPlayer >
    pub const m_CustomGameForceSelectHero: usize = 0x4C8; // int32[24]
    pub const m_flGoldTime: usize = 0x528; // float32
    pub const m_flXPTime: usize = 0x52C; // float32
    pub const m_flCreepSpawntime: usize = 0x530; // GameTime_t
    pub const m_flAnnounceStartTime: usize = 0x534; // float32
    pub const m_iGoodTomeCount: usize = 0x538; // int32
    pub const m_iBadTomeCount: usize = 0x53C; // int32
    pub const m_flPreGameStartTime: usize = 0x540; // GameTime_t
    pub const m_flGameStartTime: usize = 0x544; // GameTime_t
    pub const m_flGameEndTime: usize = 0x548; // GameTime_t
    pub const m_flGameLoadTime: usize = 0x54C; // float32
    pub const m_iCustomGameScore: usize = 0x550; // int32[2]
    pub const m_nCustomGameDifficulty: usize = 0x558; // int32
    pub const m_bEnemyModifiersEnabled: usize = 0x55C; // bool
    pub const m_iWaves: usize = 0x560; // int32
    pub const m_iCreepUpgradeState: usize = 0x564; // int32
    pub const m_fGoodGlyphCooldown: usize = 0x568; // GameTime_t
    pub const m_fBadGlyphCooldown: usize = 0x56C; // GameTime_t
    pub const m_flGlyphCooldowns: usize = 0x570; // GameTime_t[15]
    pub const m_fGoodRadarCooldown: usize = 0x5AC; // GameTime_t
    pub const m_fBadRadarCooldown: usize = 0x5B0; // GameTime_t
    pub const m_flRadarCooldowns: usize = 0x5B4; // GameTime_t[15]
    pub const m_iGoodRadarCharges: usize = 0x5F0; // int32
    pub const m_iBadRadarCharges: usize = 0x5F4; // int32
    pub const m_iRadarCharges: usize = 0x5F8; // int32[15]
    pub const m_flOutpostTimes: usize = 0x634; // float32[24]
    pub const m_bIsNightstalkerNight: usize = 0x694; // bool
    pub const m_bIsTemporaryNight: usize = 0x695; // bool
    pub const m_bIsTemporaryDay: usize = 0x696; // bool
    pub const m_nRiverType: usize = 0x698; // int32
    pub const m_nTeamFeaturedPlayerID: usize = 0x69C; // PlayerID_t[15]
    pub const m_nTeamVersusScreen: usize = 0x6D8; // item_definition_index_t[15]
    pub const m_flGoldRedistributeTime: usize = 0x714; // float32
    pub const m_nGoldToRedistribute: usize = 0x718; // int32[2]
    pub const m_flNextPreGameThink: usize = 0x720; // GameTime_t
    pub const m_flNextAllDraftGoldThink: usize = 0x724; // GameTime_t
    pub const m_flTimeEnteredState: usize = 0x728; // float64
    pub const m_unRiverAccountID: usize = 0x730; // uint32
    pub const m_ulRiverItemID: usize = 0x738; // itemid_t
    pub const m_vecItemStockInfo: usize = 0x780; // C_UtlVectorEmbeddedNetworkVar< CDOTA_ItemStockInfo >
    pub const m_AssassinMiniGameNetData: usize = 0x7D0; // DOTA_AssassinMinigameNetworkState
    pub const m_nGameWinner: usize = 0x7E0; // int32
    pub const m_unMatchID64: usize = 0x7E8; // MatchID_t
    pub const m_bMatchSignoutComplete: usize = 0x7F0; // bool
    pub const m_hSideShop1: usize = 0x7F4; // CHandle< C_BaseEntity >
    pub const m_hSideShop2: usize = 0x7F8; // CHandle< C_BaseEntity >
    pub const m_hSecretShop1: usize = 0x7FC; // CHandle< C_BaseEntity >
    pub const m_hSecretShop2: usize = 0x800; // CHandle< C_BaseEntity >
    pub const m_hTeamFountains: usize = 0x804; // CHandle< C_BaseEntity >[15]
    pub const m_hTeamForts: usize = 0x840; // CHandle< C_BaseEntity >[15]
    pub const m_hTeamShops: usize = 0x87C; // CHandle< C_BaseEntity >[15]
    pub const m_hAnnouncerGood: usize = 0x8B8; // CHandle< C_BaseEntity >
    pub const m_hAnnouncerBad: usize = 0x8BC; // CHandle< C_BaseEntity >
    pub const m_hAnnouncerSpectator: usize = 0x8C0; // CHandle< C_BaseEntity >
    pub const m_hAnnouncerGood_KillingSpree: usize = 0x8C4; // CHandle< C_BaseEntity >
    pub const m_hAnnouncerBad_KillingSpree: usize = 0x8C8; // CHandle< C_BaseEntity >
    pub const m_hAnnouncerSpectator_KillingSpree: usize = 0x8CC; // CHandle< C_BaseEntity >
    pub const m_arrTier1TowerDestroyed: usize = 0x8D0; // bool[15]
    pub const m_arrTier2TowerDestroyed: usize = 0x8DF; // bool[15]
    pub const m_arrTier3TowerDestroyed: usize = 0x8EE; // bool[15]
    pub const m_bTier1TowerDestroyed: usize = 0x8FD; // bool
    pub const m_bTier2TowerDestroyed: usize = 0x8FE; // bool
    pub const m_bTier3TowerDestroyed: usize = 0x8FF; // bool
    pub const m_fTimeOfDay: usize = 0x900; // float32
    pub const m_iNetTimeOfDay: usize = 0x904; // int32
    pub const m_nLoadedPlayers: usize = 0x908; // int32
    pub const m_nExpectedPlayers: usize = 0x90C; // int32
    pub const m_iMinimapDebugGridState: usize = 0x918; // int32
    pub const m_iFoWFrameNumber: usize = 0x998; // int32
    pub const m_vWeatherWindDirection: usize = 0x99C; // Vector
    pub const m_vecRequiredPlaytestHeroesStorage: usize = 0x9A8; // int32[24]
    pub const m_vecRecommendedPlaytestHeroesStorage: usize = 0xA08; // int32[24]
    pub const m_bAllRequiredPlaytestHeroesPicked: usize = 0xA68; // bool
    pub const m_nLastPlaytestPickPhase: usize = 0xA6C; // int32
    pub const m_nCustomGameFowTeamCount: usize = 0xA70; // int32
    pub const m_bUseAlternateABRules: usize = 0xA74; // bool
    pub const m_bLobbyIsAssociatedWithGame: usize = 0xA75; // bool
    pub const m_BotDebugTimer: usize = 0xA78; // CountdownTimer
    pub const m_BotDebugPushLane: usize = 0xA90; // uint8[18]
    pub const m_BotDebugDefendLane: usize = 0xAA2; // uint8[18]
    pub const m_BotDebugFarmLane: usize = 0xAB4; // uint8[6]
    pub const m_BotDebugRoam: usize = 0xABA; // uint8[8]
    pub const m_hBotDebugRoamTarget: usize = 0xAC4; // CHandle< C_BaseEntity >[2]
    pub const m_BotDebugRoshan: usize = 0xACC; // uint8[2]
    pub const m_nRoshanRespawnPhase: usize = 0xAD0; // ERoshanSpawnPhase
    pub const m_flRoshanRespawnPhaseEndTime: usize = 0xAD4; // GameTime_t
    pub const m_AbilityDraftAbilities: usize = 0xAD8; // C_UtlVectorEmbeddedNetworkVar< CDOTA_AbilityDraftAbilityState >
    pub const m_bAbilityDraftCurrentPlayerHasPicked: usize = 0xB28; // bool
    pub const m_nAbilityDraftPlayerTracker: usize = 0xB2C; // int32
    pub const m_nAbilityDraftRoundNumber: usize = 0xB30; // int32
    pub const m_nAbilityDraftAdvanceSteps: usize = 0xB34; // int32
    pub const m_nAbilityDraftPhase: usize = 0xB38; // int32
    pub const m_nAbilityDraftHeroesChosen: usize = 0xB3C; // int32[13]
    pub const m_bIsPlayerDraft: usize = 0xB70; // bool
    pub const m_ePlayerDraftState: usize = 0xB74; // DOTA_PlayerDraftState
    pub const m_vecPlayerDraftPickOrder: usize = 0xB78; // C_NetworkUtlVectorBase< uint8 >
    pub const m_nPlayerDraftPick: usize = 0xB90; // int32
    pub const m_nPlayerDraftActiveTeam: usize = 0xB94; // int32
    pub const m_flPlayerDraftTimeBank: usize = 0xB98; // float32[2]
    pub const m_vecARDMHeroes: usize = 0xBA0; // CUtlVector< KeyValues* >[2]
    pub const m_nARDMHeroesPrecached: usize = 0xBD0; // int32
    pub const m_fLastARDMPrecache: usize = 0xBD4; // float32
    pub const m_nAllDraftPhase: usize = 0xBD8; // int32
    pub const m_bAllDraftRadiantFirst: usize = 0xBDC; // bool
    pub const m_bAllowOverrideVPK: usize = 0xBDD; // bool
    pub const m_nARDMHeroesRemaining: usize = 0xBE0; // int32[2]
    pub const m_bUpdateHeroStatues: usize = 0xBE8; // bool
    pub const m_bExperimentalGameplay: usize = 0xC00; // bool
    pub const m_vecPlayerMMR: usize = 0xC40; // CUtlVector< int32 >
    pub const m_lobbyType: usize = 0xC58; // uint32
    pub const m_lobbyLeagueID: usize = 0xC5C; // LeagueID_t
    pub const m_lobbyGameName: usize = 0xC60; // char[256]
    pub const m_vecHeroStatueLiked: usize = 0xD60; // C_UtlVectorEmbeddedNetworkVar< CHeroStatueLiked >
    pub const m_CustomGameTeamMaxPlayers: usize = 0xDB0; // int32[15]
    pub const m_iMutations: usize = 0xDEC; // int32[5]
    pub const m_vecIngameEvents: usize = 0xE00; // C_NetworkUtlVectorBase< CHandle< C_IngameEvent_Base > >
    pub const m_nPrimaryIngameEventIndex: usize = 0xE18; // int8
    pub const m_hObsoleteIngameEvent: usize = 0xE1C; // CHandle< C_IngameEvent_Base >
    pub const m_nOfrendaPledges: usize = 0xE20; // uint32
    pub const m_nRadiantOfrendas: usize = 0xE24; // uint32
    pub const m_nDireOfrendas: usize = 0xE28; // uint32
    pub const m_bOfrendaEnabled: usize = 0xE2C; // bool
    pub const m_NeutralSpawnBoxes: usize = 0xE60; // C_NetworkUtlVectorBase< NeutralSpawnBoxes_t >
    pub const m_RegionTriggerBoxes: usize = 0xE78; // C_NetworkUtlVectorBase< RegionTriggerBoxes_t >
    pub const m_flLastPerfSampleTime: usize = 0x1E48; // float64
    pub const m_flLastPerfSampleSendTime: usize = 0x1E50; // float64
    pub const m_bDidSeeStrategyTime: usize = 0x1E58; // bool
    pub const m_flLastUnfocusedSleepTime: usize = 0xABB0; // float64
}

pub mod C_DOTAGamerulesProxy {
    pub const m_pGameRules: usize = 0x538; // C_DOTAGamerules*
}

pub mod C_DOTAPlayerController {
    pub const m_iMinimapMove: usize = 0x750; // int32
    pub const m_pClickBehaviorKeys: usize = 0x758; // KeyValues*
    pub const m_flCenterTime: usize = 0x760; // GameTime_t
    pub const m_iConfirmationIndex: usize = 0x764; // int32
    pub const m_bCenterOnHero: usize = 0x768; // bool
    pub const m_bHeroAssigned: usize = 0x769; // bool
    pub const m_nKeyBindHeroID: usize = 0x76C; // int32
    pub const m_bUsingCameraMan: usize = 0x770; // bool
    pub const m_bUsingAssistedCameraOperator: usize = 0x771; // bool
    pub const m_nPlayerAssistFlags: usize = 0x774; // int32
    pub const m_vLatestEvent: usize = 0x778; // Vector
    pub const m_hFreeDrawAbility: usize = 0x7A8; // CHandle< C_DOTABaseAbility >
    pub const m_vLastFreeDrawPosition: usize = 0x7AC; // Vector
    pub const m_nPlayerID: usize = 0x7B8; // PlayerID_t
    pub const m_hAssignedHero: usize = 0x7BC; // CHandle< C_BaseEntity >
    pub const m_hLastAssignedHero: usize = 0x7C0; // CHandle< C_BaseEntity >
    pub const m_hKillCamUnit: usize = 0x7C4; // CHandle< C_BaseEntity >
    pub const m_hPreviousKillCamUnit: usize = 0x7C8; // CHandle< C_BaseEntity >
    pub const m_flKillCamUnitReceivedTime: usize = 0x7CC; // float32
    pub const m_nRareLineClickCount: usize = 0x7D8; // int32
    pub const m_nRareLinesPlayed: usize = 0x7DC; // int32
    pub const m_nRareLineGroup: usize = 0x7E0; // int32
    pub const m_flLastRareLinePlayTime: usize = 0x7E4; // float32
    pub const m_flLastUnitOrdersSendTime: usize = 0x7E8; // float32
    pub const m_bTeleportRequiresHalt: usize = 0x7EC; // bool
    pub const m_bChannelRequiresHalt: usize = 0x7ED; // bool
    pub const m_bAutoPurchaseItems: usize = 0x7EE; // bool
    pub const m_bDisableHUDErrorMessages: usize = 0x7EF; // bool
    pub const m_iMouseDragStartX: usize = 0x7F0; // int32
    pub const m_iMouseDragStartY: usize = 0x7F4; // int32
    pub const m_nWeatherType: usize = 0x7F8; // int32
    pub const m_bDynamicWeatherSystemActive: usize = 0x7FC; // bool
    pub const m_bDynamicSoundHandled: usize = 0x7FD; // bool
    pub const m_flDynamicWeatherNextSwitchTime: usize = 0x800; // GameTime_t
    pub const m_flDynamicWeatherScaleFinishedTime: usize = 0x804; // GameTime_t
    pub const m_flDynamicWeatherIntensity: usize = 0x808; // float32
    pub const m_nXPRangeFXIndex: usize = 0x80C; // ParticleIndex_t
    pub const m_nVisionRangeFXIndex: usize = 0x810; // ParticleIndex_t
    pub const m_nSelectedControlGroup: usize = 0x814; // int32
    pub const m_nCachedCoachedTeam: usize = 0x818; // int32
    pub const m_hActiveAbility: usize = 0x81C; // CHandle< C_DOTABaseAbility >
    pub const m_unitorders: usize = 0x820; // CUtlVector< CUnitOrders >
    pub const m_nOutgoingOrderSequenceNumber: usize = 0x838; // int32
    pub const m_nServerOrderSequenceNumber: usize = 0x83C; // int32
    pub const m_nLastSentOutgoingOrderSequenceNumber: usize = 0x840; // int32
    pub const m_nSelectedUnits: usize = 0x848; // CUtlVector< CEntityIndex >
    pub const m_nWaypoints: usize = 0x860; // CUtlVector< ParticleIndex_t >
    pub const m_iActions: usize = 0x878; // int32
    pub const m_hQueryUnit: usize = 0x87C; // CHandle< C_DOTA_BaseNPC >
    pub const m_bInQuery: usize = 0x880; // bool
    pub const m_bSelectionChangedInDataUpdate: usize = 0x881; // bool
    pub const m_flQueryInhibitingActionTime: usize = 0x8E8; // GameTime_t
    pub const m_flQueryInhibitDuration: usize = 0x8EC; // float32
    pub const m_RingedEntities: usize = 0x8F0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_ActiveRingOwners: usize = 0x908; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_bOverridingQuery: usize = 0x920; // bool
    pub const m_flLastAutoRepeatTime: usize = 0x928; // float32
    pub const m_flConsumeDoubleclickTime: usize = 0x92C; // float32
    pub const m_LightInfoWeatherEffect: usize = 0x930; // CUtlString
    pub const m_bPreviousWasLightInfoWeather: usize = 0x938; // bool
    pub const m_MapDefaultWeatherEffect: usize = 0x940; // CUtlString
    pub const m_bMapUsesDynamicWeather: usize = 0x948; // bool
    pub const m_vecSuggestedWardLocationEffects: usize = 0x988; // CUtlVector< ParticleIndex_t >
    pub const m_pSmartCastNPC: usize = 0x9A0; // C_DOTA_BaseNPC*
    pub const m_nTeamSprayParticleIndex: usize = 0x9A8; // ParticleIndex_t
    pub const m_bIsNextCastOrderFromMouseClick: usize = 0x9AC; // bool
    pub const m_iCursor: usize = 0x9E0; // int32[2]
    pub const m_iSpectatorClickBehavior: usize = 0x9E8; // int32
    pub const m_flAspectRatio: usize = 0x9EC; // float32
    pub const m_hSpectatorQueryUnit: usize = 0x9F0; // CHandle< C_BaseEntity >
    pub const m_iStatsPanel: usize = 0x9F4; // int32
    pub const m_iShopPanel: usize = 0x9F8; // int32
    pub const m_iShopViewMode: usize = 0x9FC; // ShopItemViewMode_t
    pub const m_iStatsDropdownCategory: usize = 0xA00; // int32
    pub const m_iStatsDropdownSort: usize = 0xA04; // int32
    pub const m_szShopString: usize = 0xA08; // char[64]
    pub const m_vecClientQuickBuyState: usize = 0xA48; // C_UtlVectorEmbeddedNetworkVar< ClientQuickBuyItemState >
    pub const m_bInShowCaseMode: usize = 0xA98; // bool
    pub const m_flCameraZoomAmount: usize = 0xA9C; // float32
    pub const m_iHighPriorityScore: usize = 0xAA0; // int32
    pub const m_quickBuyItems: usize = 0xAA4; // AbilityID_t[9]
    pub const m_quickBuyIsPurchasable: usize = 0xAC8; // bool[9]
    pub const m_flFrameTime: usize = 0xAD4; // float32
    pub const m_flFrameTimeStdDev: usize = 0xAD8; // float32
    pub const m_flUnfilteredFrameTime: usize = 0xADC; // float32
    pub const m_NeutralCampAlarms: usize = 0xAE0; // CUtlVector< NeutralCampStackPullAlarm_t >
    pub const m_iPrevCursor: usize = 0xAF8; // int32[2]
    pub const m_iPositionHistoryTail: usize = 0xBA0; // int32
    pub const m_iMusicStatus: usize = 0xBA4; // int32
    pub const m_iPreviousMusicStatus: usize = 0xBA8; // int32
    pub const m_bRequestedInventory: usize = 0xBAC; // bool
    pub const m_flMusicOperatorVals: usize = 0xBB0; // float32[3]
    pub const m_iMusicOperatorVals: usize = 0xBBC; // int32[4]
    pub const m_ControlGroups: usize = 0xBD0; // CUtlVector< sControlGroupElem >[10]
    pub const m_pkvControlGroupKV: usize = 0xCC0; // KeyValues*
    pub const m_flAltHeldStartTime: usize = 0xD20; // float32
}

pub mod C_DOTAPropCustomTexture {
    pub const m_unTeamID: usize = 0xA58; // uint32
    pub const m_bSetupMaterialProxy: usize = 0xA68; // bool
}

pub mod C_DOTASceneEntity {
    pub const m_nCustomStackIndex: usize = 0x598; // int32
    pub const m_flVolume: usize = 0x59C; // float32
}

pub mod C_DOTASpectatorGraphManager {
    pub const __m_pChainEntity: usize = 0x28; // CNetworkVarChainer
    pub const m_bTrackingTeamStats: usize = 0x50; // bool
    pub const m_flStartTime: usize = 0x54; // GameTime_t
    pub const m_rgRadiantTotalEarnedGold: usize = 0x58; // int32[64]
    pub const m_rgDireTotalEarnedGold: usize = 0x158; // int32[64]
    pub const m_rgRadiantTotalEarnedXP: usize = 0x258; // int32[64]
    pub const m_rgDireTotalEarnedXP: usize = 0x358; // int32[64]
    pub const m_rgRadiantNetWorth: usize = 0x458; // int32[64]
    pub const m_rgDireNetWorth: usize = 0x558; // int32[64]
    pub const m_flTotalEarnedGoldStartTime: usize = 0x658; // GameTime_t
    pub const m_flTotalEarnedGoldEndTime: usize = 0x65C; // GameTime_t
    pub const m_nGoldGraphVersion: usize = 0x660; // int32
    pub const m_rgRadiantWinChance: usize = 0x664; // int32[64]
    pub const m_TeamStatsUpdateTimer: usize = 0x768; // CountdownTimer
    pub const m_HeroInventorySnapshotTimer: usize = 0x780; // CountdownTimer
    pub const m_vecPlayerSnapshots: usize = 0x798; // CUtlVector< sPlayerSnapshot >[24]
    pub const m_unDataChangedCount: usize = 0x9F0; // uint32
}

pub mod C_DOTASpectatorGraphManagerProxy {
    pub const m_pGraphManager: usize = 0x538; // C_DOTASpectatorGraphManager*
}

pub mod C_DOTATeam {
    pub const m_iHeroKills: usize = 0x5F0; // int32
    pub const m_iTowerKills: usize = 0x5F4; // int32
    pub const m_iBarracksKills: usize = 0x5F8; // int32
    pub const m_unTournamentTeamID: usize = 0x5FC; // uint32
    pub const m_ulTeamLogo: usize = 0x600; // uint64
    pub const m_ulTeamBaseLogo: usize = 0x608; // uint64
    pub const m_ulTeamBannerLogo: usize = 0x610; // uint64
    pub const m_bTeamComplete: usize = 0x618; // bool
    pub const m_bTeamIsHomeTeam: usize = 0x619; // bool
    pub const m_CustomHealthbarColor: usize = 0x61A; // Color
    pub const m_szTag: usize = 0x61E; // char[33]
}

pub mod C_DOTATurboHeroPickRules {
    pub const m_Phase: usize = 0x538; // DOTACustomHeroPickRulesPhase_t
}

pub mod C_DOTAWearableItem {
    pub const m_bOwnerModelChanged: usize = 0xAF8; // bool
    pub const m_bIsGeneratingEconItem: usize = 0xAF9; // bool
    pub const m_bIsItemVisibleOnGeneratedEntity: usize = 0xAFA; // bool
    pub const m_bHideWearableButDrawAdditionalWearables: usize = 0xAFB; // bool
    pub const m_hAdditionalWearable: usize = 0xAFC; // CHandle< CBaseAnimatingActivity >
    pub const m_hDrawWearable: usize = 0xB00; // CHandle< C_DOTAWearableItem >
    pub const m_bHiddenByCombiner: usize = 0xB04; // bool
    pub const m_bHiddenByEquipmentSlot: usize = 0xB05; // bool
    pub const m_bIsPortrait: usize = 0xB06; // bool
    pub const m_fZDelta: usize = 0xB08; // float32
    pub const m_combinerMaterialOverrideList: usize = 0xB10; // CUtlVector< CStrongHandleCopyable< InfoForResourceTypeIMaterial2 > >
    pub const m_bCombinerMaterialOverrideListChanged: usize = 0xB30; // bool
    pub const m_bRubickFollower: usize = 0xB31; // bool
    pub const m_bHasPlayerModel: usize = 0xB32; // bool
}

pub mod C_DOTAWorldParticleSystem {
    pub const m_nType: usize = 0x7B8; // int32
    pub const m_iClientEffectIndex: usize = 0x7BC; // ParticleIndex_t
    pub const m_szEffectName: usize = 0x7C0; // CUtlSymbolLarge
    pub const m_szTargetName: usize = 0x7C8; // CUtlSymbolLarge
    pub const m_szControlPoint: usize = 0x7D0; // CUtlSymbolLarge
    pub const m_hOverrideSequence: usize = 0x7D8; // HSequence
    pub const m_hOverrideModel: usize = 0x7E0; // CStrongHandle< InfoForResourceTypeCModel >
    pub const m_vModelScale: usize = 0x7E8; // Vector
    pub const m_nSkinOverride: usize = 0x7F4; // int32
    pub const m_bDayTime: usize = 0x7F8; // bool
    pub const m_bNightTime: usize = 0x7F9; // bool
    pub const m_bShowInFow: usize = 0x7FA; // bool
    pub const m_bShowWhileDynamicWeatherActive: usize = 0x7FB; // bool
    pub const m_bAnimateDuringGameplayPause: usize = 0x7FC; // bool
}

pub mod C_DOTA_Ability_AbyssalUnderlord_DarkRift {
    pub const teleport_delay: usize = 0x5B0; // float32
    pub const m_hTeleportTarget: usize = 0x5B4; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_AbyssalUnderlord_Dark_Portal {
    pub const duration: usize = 0x5B0; // float32
    pub const distance_from_fountain: usize = 0x5B4; // float32
}

pub mod C_DOTA_Ability_AbyssalUnderlord_Portal_Warp {
    pub const m_hTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const m_flChannelTime: usize = 0x5B4; // float32
}

pub mod C_DOTA_Ability_Aghanim_UrnUpheaval {
    pub const m_vPosition: usize = 0x5B0; // Vector
    pub const aoe: usize = 0x5BC; // float32
    pub const slow_rate: usize = 0x5C0; // float32
    pub const slow_rate_duration: usize = 0x5C4; // float32
    pub const duration: usize = 0x5C8; // float32
    pub const max_slow: usize = 0x5CC; // float32
    pub const burn_damage: usize = 0x5D0; // float32
    pub const m_flCurrentSlow: usize = 0x5D4; // float32
    pub const m_nFXIndex: usize = 0x5D8; // ParticleIndex_t
    pub const m_SlowTimer: usize = 0x5E0; // CountdownTimer
    pub const m_timer: usize = 0x5F8; // CountdownTimer
}

pub mod C_DOTA_Ability_AghsFort_BossDarkWillow_ShadowRealm {
    pub const projectile_damage: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_AghsFort_Creature_Phoenix_FireSpirits {
    pub const hp_cost_perc: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_AghsFort_Creature_Venomancer_PoisonNova {
    pub const m_nFXPreview: usize = 0x5B0; // ParticleIndex_t
    pub const radius: usize = 0x5B4; // int32
}

pub mod C_DOTA_Ability_AghsFort_EarthSpiritBoss_Magnetize {
    pub const cast_radius: usize = 0x5B0; // int32
    pub const rock_explosion_radius: usize = 0x5B4; // int32
    pub const damage_duration: usize = 0x5B8; // float32
}

pub mod C_DOTA_Ability_AghsFort_Juggernaut_BladeDance {
    pub const m_hBladeFuryThinker: usize = 0x5B0; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_AghsFort_Kunkka_GhostShip {
    pub const buff_duration: usize = 0x5B0; // float32
    pub const stun_duration: usize = 0x5B4; // float32
    pub const ghostship_width: usize = 0x5B8; // int32
    pub const ghostship_width_scepter: usize = 0x5BC; // int32
    pub const m_vFinalDestination: usize = 0x5C0; // Vector
    pub const m_vStartingPoint: usize = 0x5CC; // Vector
}

pub mod C_DOTA_Ability_AghsFort_Kunkka_Tidal_Wave {
    pub const m_WaveData: usize = 0x5B0; // WaveData_t[2]
}

pub mod C_DOTA_Ability_AghsFort_Kunkka_Torrent {
    pub const torrent_storm_chance: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_AghsFort_Kunkka_XMarksTheSpot {
    pub const m_hThinkerEntities: usize = 0x5B0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_AghsFort_Lich_ChainFrost {
    pub const jump_range: usize = 0x5B0; // int32
    pub const jumps: usize = 0x5B4; // int32
    pub const slow_duration: usize = 0x5B8; // float32
    pub const vision_radius: usize = 0x5BC; // int32
    pub const projectile_speed: usize = 0x5C0; // int32
}

pub mod C_DOTA_Ability_AghsFort_Lich_Ice_Spire {
    pub const aura_radius: usize = 0x5B0; // int32
    pub const duration: usize = 0x5B4; // float32
}

pub mod C_DOTA_Ability_AghsFort_Lich_Sinister_Gaze {
    pub const m_hShackleTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const m_vPullLocation: usize = 0x5B4; // Vector
    pub const m_flIntervalRate: usize = 0x5C0; // float32
}

pub mod C_DOTA_Ability_AghsFort_Phoenix_FireSpirits {
    pub const hp_cost_perc: usize = 0x5B0; // int32
    pub const spirit_count: usize = 0x5B4; // int32
}

pub mod C_DOTA_Ability_AghsFort_QueenOfPain_ScreamOfPain {
    pub const damage: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_AghsFort_QueenOfPain_ShadowStrike {
    pub const projectile_speed: usize = 0x5B0; // Vector
}

pub mod C_DOTA_Ability_AghsFort_QueenOfPain_SonicWave {
    pub const m_fStartTime: usize = 0x5B0; // GameTime_t
    pub const m_fTotalTime: usize = 0x5B4; // float32
    pub const m_nFXIndex: usize = 0x5B8; // ParticleIndex_t
    pub const m_nSonicTrailRadius: usize = 0x5BC; // int32
    pub const m_vLastTrailThinkerLocation: usize = 0x5C0; // Vector
    pub const bTrailStartedBurning: usize = 0x5CC; // bool
    pub const damage: usize = 0x5D0; // int32
    pub const starting_aoe: usize = 0x5D4; // int32
    pub const final_aoe: usize = 0x5D8; // int32
}

pub mod C_DOTA_Ability_AghsFort_RockGolem_Smash {
    pub const m_nPreviewFX: usize = 0x5B0; // ParticleIndex_t
    pub const m_vTargetLoc: usize = 0x5B4; // Vector
    pub const radius: usize = 0x5C8; // int32
}

pub mod C_DOTA_Ability_AghsFort_ShadowShaman_Shackles {
    pub const m_hShackleTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const nShackleFXIndex: usize = 0x5B4; // ParticleIndex_t
}

pub mod C_DOTA_Ability_AghsFort_Sniper_Concussive_Grenade {
    pub const m_iProjectile: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_AghsFort_StonehallGeneral_OverwhelmingOdds {
    pub const max_steps: usize = 0x5B0; // int32
    pub const m_vTarget: usize = 0x5B4; // Vector
    pub const m_vDir: usize = 0x5C0; // Vector
    pub const m_nSteps: usize = 0x5CC; // int32
    pub const m_flChannelStartTime: usize = 0x5D0; // GameTime_t
    pub const m_nFXIndex: usize = 0x5D4; // ParticleIndex_t
}

pub mod C_DOTA_Ability_AghsFort_TrapRoom_Hookshot {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
    pub const m_vProjectileVelocity: usize = 0x5B4; // Vector
    pub const m_bRetract: usize = 0x5C0; // bool
}

pub mod C_DOTA_Ability_AghsFort_Wave_Blast {
    pub const damage: usize = 0x5B0; // int32
    pub const knockback_duration: usize = 0x5B4; // float32
    pub const disarm_duration: usize = 0x5B8; // float32
}

pub mod C_DOTA_Ability_Aghsfort2_DrowRanger_Multishot {
    pub const m_vHitTargets: usize = 0x5B0; // CUtlVector< CUtlVector< CHandle< C_BaseEntity > > >
}

pub mod C_DOTA_Ability_Aghsfort2_DrowRanger_WaveOfSilence {
    pub const wave_length: usize = 0x5B0; // int32
    pub const wave_width: usize = 0x5B4; // int32
    pub const wave_speed: usize = 0x5B8; // float32
    pub const silence_duration: usize = 0x5BC; // float32
    pub const knockback_distance_max: usize = 0x5C0; // float32
    pub const knockback_duration: usize = 0x5C4; // float32
}

pub mod C_DOTA_Ability_Aghsfort_Aziyog_Underlord_Dark_Portal {
    pub const duration: usize = 0x5B0; // float32
}

pub mod C_DOTA_Ability_Aghsfort_Aziyog_Underlord_Portal_Warp {
    pub const m_hTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_Aghsfort_Bane_BrainSap {
    pub const cast_range: usize = 0x5B0; // int32
    pub const brain_sap_damage: usize = 0x5B4; // int32
}

pub mod C_DOTA_Ability_Aghsfort_Bane_FiendsGrip {
    pub const m_hGripTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const fiend_grip_damage: usize = 0x5B4; // int32
    pub const fiend_grip_spread_radius: usize = 0x5B8; // int32
    pub const channel_time: usize = 0x5BC; // float32
    pub const fiend_grip_tick_interval: usize = 0x5C0; // float32
    pub const m_flLastSpreadTime: usize = 0x5C4; // GameTime_t
}

pub mod C_DOTA_Ability_Aghsfort_Dawnbreaker_Solar_Guardian {
    pub const m_hThinker: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const m_bJumping: usize = 0x5B4; // bool
    pub const m_vecDeathThinkers: usize = 0x5B8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_hTeleportTarget: usize = 0x5D0; // CHandle< C_BaseEntity >
    pub const m_nTPFXIndex: usize = 0x5D4; // ParticleIndex_t
    pub const m_nAoEFXIndex: usize = 0x5D8; // ParticleIndex_t
}

pub mod C_DOTA_Ability_Aghsfort_Elemental_Wisp_Tether {
    pub const m_hTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const m_vProjectileLocation: usize = 0x5B4; // Vector
    pub const m_bProjectileActive: usize = 0x5C0; // bool
    pub const latch_distance: usize = 0x5C4; // int32
    pub const m_iProjectileIndex: usize = 0x5C8; // int32
}

pub mod C_DOTA_Ability_Aghsfort_Gyrocopter_Rocket_Barrage {
    pub const duration: usize = 0x5B0; // float32
}

pub mod C_DOTA_Ability_Aghsfort_Lina_DragonSlave {
    pub const dragon_slave_distance: usize = 0x5B0; // int32
    pub const m_vLastTrailThinkerLocation: usize = 0x5B4; // Vector
    pub const dragon_slave_damage: usize = 0x5C0; // int32
    pub const m_flLSADistance: usize = 0x5C4; // float32
    pub const m_nLSAPower: usize = 0x5C8; // int32
    pub const m_flLSADelay: usize = 0x5CC; // float32
}

pub mod C_DOTA_Ability_Aghsfort_Lina_LagunaBlade {
    pub const m_hHitEntities: usize = 0x5B0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_nMaxBounces: usize = 0x5C8; // int32
}

pub mod C_DOTA_Ability_Aghsfort_Magnataur_Empower {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
}

pub mod C_DOTA_Ability_Aghsfort_Magnataur_FriendlySkewer {
    pub const skewer_radius: usize = 0x5B0; // int32
    pub const skewer_speed: usize = 0x5B4; // int32
    pub const max_targets: usize = 0x5B8; // int32
    pub const range: usize = 0x5BC; // int32
    pub const tree_radius: usize = 0x5C0; // int32
    pub const affects_creeps: usize = 0x5C4; // int32
    pub const m_nTargetsHit: usize = 0x5C8; // int32
}

pub mod C_DOTA_Ability_Aghsfort_Magnataur_ReversePolarity {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
    pub const m_vPullLocation: usize = 0x5B4; // Vector
}

pub mod C_DOTA_Ability_Aghsfort_Magnataur_Shockwave {
    pub const m_nFXIndices: usize = 0x5B0; // CUtlVector< ParticleIndex_t >
}

pub mod C_DOTA_Ability_Aghsfort_Magnataur_Skewer {
    pub const skewer_radius: usize = 0x5B0; // int32
    pub const skewer_speed: usize = 0x5B4; // int32
    pub const range: usize = 0x5B8; // int32
    pub const tree_radius: usize = 0x5BC; // int32
    pub const m_vSkewerDirection: usize = 0x5C0; // Vector
    pub const m_nTargetsHit: usize = 0x5CC; // int32
}

pub mod C_DOTA_Ability_Aghsfort_PrimalBeast_Boss_Onslaught {
    pub const max_charge_time: usize = 0x5B0; // float32
}

pub mod C_DOTA_Ability_Aghsfort_PrimalBeast_Boss_Pummel {
    pub const m_hTarget: usize = 0x5B0; // C_DOTA_BaseNPC*
}

pub mod C_DOTA_Ability_Aghsfort_PrimalBeast_Boss_RoarAttack {
    pub const speed: usize = 0x5B0; // int32
    pub const projectile_count: usize = 0x5B4; // int32
    pub const rotation_angle: usize = 0x5B8; // float32
    pub const damage: usize = 0x5BC; // float32
    pub const radius: usize = 0x5C0; // float32
    pub const ctTimer: usize = 0x5C8; // CountdownTimer
    pub const m_vecStartRot: usize = 0x5E0; // Vector
    pub const m_vecEndRot: usize = 0x5EC; // Vector
}

pub mod C_DOTA_Ability_Aghsfort_PrimalBeast_Boss_TectonicShift {
    pub const damage: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_Aghsfort_Sand_King_BurrowStrike {
    pub const burrow_width: usize = 0x5B0; // int32
    pub const burrow_speed: usize = 0x5B4; // int32
    pub const burrow_anim_time: usize = 0x5B8; // float32
    pub const m_vStartLocation: usize = 0x5BC; // Vector
}

pub mod C_DOTA_Ability_Aghsfort_Sand_King_Epicenter {
    pub const m_nFXEpicenterIndex: usize = 0x5B0; // ParticleIndex_t
}

pub mod C_DOTA_Ability_Aghsfort_Sand_King_SandStorm {
    pub const m_iRadius: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_Aghsfort_Special_Magnataur_ReversePolarity_Polarity {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
    pub const m_vPullLocation: usize = 0x5B4; // Vector
    pub const pAbilityPrimaryPolarity: usize = 0x5C0; // C_DOTABaseAbility*
}

pub mod C_DOTA_Ability_Aghsfort_TuskBoss_IceShards {
    pub const m_iProjectile: usize = 0x5B0; // int32
    pub const shard_width: usize = 0x5B4; // int32
    pub const shard_damage: usize = 0x5B8; // int32
    pub const shard_count: usize = 0x5BC; // int32
    pub const shard_speed: usize = 0x5C0; // float32
    pub const shard_duration: usize = 0x5C4; // float32
    pub const shard_angle_step: usize = 0x5C8; // float32
    pub const shard_distance: usize = 0x5CC; // int32
    pub const m_vSpawnOrigin: usize = 0x5D0; // Vector
    pub const m_vDirection: usize = 0x5DC; // Vector
}

pub mod C_DOTA_Ability_Aghsfort_TuskBoss_Snowball {
    pub const snowball_windup_radius: usize = 0x5B0; // int32
    pub const snowball_radius: usize = 0x5B4; // int32
    pub const snowball_grow_rate: usize = 0x5B8; // int32
    pub const snowball_damage: usize = 0x5BC; // int32
    pub const snowball_damage_bonus: usize = 0x5C0; // int32
    pub const stun_duration: usize = 0x5C4; // float32
    pub const bonus_damage: usize = 0x5C8; // int32
    pub const bonus_stun: usize = 0x5CC; // float32
    pub const snowball_speed: usize = 0x5D0; // int32
    pub const snowball_duration: usize = 0x5D4; // float32
    pub const m_vProjectileLocation: usize = 0x5D8; // Vector
    pub const m_hSnowballedUnits: usize = 0x5E8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_nFXIndex: usize = 0x600; // ParticleIndex_t
    pub const ctSnowball: usize = 0x608; // CountdownTimer
    pub const m_bSpeakAlly: usize = 0x620; // bool
    pub const m_bIsExpired: usize = 0x621; // bool
    pub const m_bInWindup: usize = 0x622; // bool
    pub const m_hPrimaryTarget: usize = 0x624; // CHandle< C_BaseEntity >
    pub const m_nContainedValidUnits: usize = 0x628; // int32
    pub const m_bEndingSnowball: usize = 0x62C; // bool
}

pub mod C_DOTA_Ability_Aghsfort_Tusk_IceShards {
    pub const m_iProjectile: usize = 0x5B0; // int32
    pub const shard_width: usize = 0x5B4; // int32
    pub const shard_damage: usize = 0x5B8; // int32
    pub const shard_count: usize = 0x5BC; // int32
    pub const shard_speed: usize = 0x5C0; // float32
    pub const shard_duration: usize = 0x5C4; // float32
    pub const shard_angle_step: usize = 0x5C8; // float32
    pub const shard_distance: usize = 0x5CC; // int32
    pub const secondary_damage_pct: usize = 0x5D0; // int32
    pub const m_vSpawnOrigin: usize = 0x5D4; // Vector
    pub const m_vDirection: usize = 0x5E0; // Vector
}

pub mod C_DOTA_Ability_Aghsfort_Tusk_Snowball {
    pub const snowball_windup_radius: usize = 0x5B0; // int32
    pub const snowball_radius: usize = 0x5B4; // int32
    pub const snowball_grow_rate: usize = 0x5B8; // int32
    pub const snowball_damage: usize = 0x5BC; // int32
    pub const snowball_damage_bonus: usize = 0x5C0; // int32
    pub const stun_duration: usize = 0x5C4; // float32
    pub const bonus_damage: usize = 0x5C8; // int32
    pub const bonus_stun: usize = 0x5CC; // float32
    pub const snowball_speed: usize = 0x5D0; // int32
    pub const snowball_duration: usize = 0x5D4; // float32
    pub const m_vProjectileLocation: usize = 0x5D8; // Vector
    pub const m_hSnowballedUnits: usize = 0x5E8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_nFXIndex: usize = 0x600; // ParticleIndex_t
    pub const ctSnowball: usize = 0x608; // CountdownTimer
    pub const m_bSpeakAlly: usize = 0x620; // bool
    pub const m_bIsExpired: usize = 0x621; // bool
    pub const m_bInWindup: usize = 0x622; // bool
    pub const m_hPrimaryTarget: usize = 0x624; // CHandle< C_BaseEntity >
    pub const m_nContainedValidUnits: usize = 0x628; // int32
    pub const m_bEndingSnowball: usize = 0x62C; // bool
}

pub mod C_DOTA_Ability_Aghsfort_Tusk_Snowball_Dummy {
    pub const snowball_radius: usize = 0x5B0; // int32
    pub const snowball_grow_rate: usize = 0x5B4; // int32
    pub const snowball_damage: usize = 0x5B8; // int32
    pub const snowball_damage_bonus: usize = 0x5BC; // int32
    pub const stun_duration: usize = 0x5C0; // float32
    pub const bonus_damage: usize = 0x5C4; // int32
    pub const bonus_stun: usize = 0x5C8; // float32
    pub const snowball_speed: usize = 0x5CC; // int32
    pub const snowball_duration: usize = 0x5D0; // float32
    pub const m_bIsExpired: usize = 0x5D4; // bool
    pub const m_vProjectileLocation: usize = 0x5D8; // Vector
    pub const ctSnowball: usize = 0x5E8; // CountdownTimer
    pub const m_hSnowballedUnits: usize = 0x600; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_nFXIndex: usize = 0x618; // ParticleIndex_t
    pub const m_hPrimaryTarget: usize = 0x61C; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_Aghsfort_Viper_Aghsfort_ViperStrike {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
    pub const m_nTargetTeam: usize = 0x5B4; // int32
    pub const m_flNextViperStrikeTime: usize = 0x5B8; // float32
}

pub mod C_DOTA_Ability_Aghsfort_Weaver_Shukuchi {
    pub const duration: usize = 0x5B0; // float32
    pub const radius: usize = 0x5B4; // int32
    pub const damage: usize = 0x5B8; // int32
    pub const fade_time: usize = 0x5BC; // float32
}

pub mod C_DOTA_Ability_Aghsfort_Weaver_TimeLapse {
    pub const m_nNPCSpawnedID: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_Aghsfort_Wildwing_Tornado_Blast {
    pub const disable_duration: usize = 0x5B0; // float32
    pub const damage: usize = 0x5B4; // float32
    pub const m_nPreviewFX: usize = 0x5B8; // ParticleIndex_t
}

pub mod C_DOTA_Ability_Aghsfort_WitchDoctor_DeathWard {
    pub const m_iDamage: usize = 0x5B0; // int32
    pub const m_iBounceRadius: usize = 0x5B4; // int32
    pub const m_iProjectileSpeed: usize = 0x5B8; // int32
    pub const m_iBounces: usize = 0x5BC; // int32
    pub const m_hWard: usize = 0x5C0; // CHandle< C_BaseEntity >
    pub const m_fWardExpireTime: usize = 0x5C4; // GameTime_t
    pub const m_nFXIndex: usize = 0x5C8; // ParticleIndex_t
    pub const m_iAttackIndex: usize = 0x5CC; // int32
    pub const m_BounceInfo: usize = 0x5D0; // CUtlVector< sAghsfortWardBounceInfo >
}

pub mod C_DOTA_Ability_Aghsfort_WitchDoctor_DeathWard_No_Channel {
    pub const m_iDamage: usize = 0x5B0; // int32
    pub const m_iBounceRadius: usize = 0x5B4; // int32
    pub const m_iProjectileSpeed: usize = 0x5B8; // int32
    pub const m_iBounces: usize = 0x5BC; // int32
    pub const m_hWard: usize = 0x5C0; // CHandle< C_BaseEntity >
    pub const m_fWardExpireTime: usize = 0x5C4; // float32
    pub const m_nFXIndex: usize = 0x5C8; // ParticleIndex_t
    pub const m_iAttackIndex: usize = 0x5CC; // int32
    pub const m_BounceInfo: usize = 0x5D0; // CUtlVector< sAghsfortWardBounceInfo >
}

pub mod C_DOTA_Ability_Aghsfort_WitchDoctor_ParalyzingCask {
    pub const bounces: usize = 0x5B0; // int32
    pub const bounce_bonus_damage: usize = 0x5B4; // int32
    pub const damage: usize = 0x5B8; // int32
}

pub mod C_DOTA_Ability_Animation_Attack {
    pub const animation_time: usize = 0x5B0; // float32
}

pub mod C_DOTA_Ability_Animation_TailSpin {
    pub const animation_time: usize = 0x5B0; // float32
}

pub mod C_DOTA_Ability_ArcWarden_SparkWraith {
    pub const duration: usize = 0x5B0; // float32
    pub const activation_delay: usize = 0x5B4; // float32
    pub const wraith_vision_duration: usize = 0x5B8; // float32
    pub const wraith_vision_radius: usize = 0x5BC; // int32
    pub const spark_damage: usize = 0x5C0; // float32
    pub const creep_damage_bonus_pct: usize = 0x5C4; // int32
}

pub mod C_DOTA_Ability_ArcWarden_TempestDouble {
    pub const m_hDoubles: usize = 0x5B0; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_Axe_BattleHunger {
    pub const damage_per_second: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_Axe_CullingBlade {
    pub const m_bInterrupted: usize = 0x5B0; // bool
    pub const m_vProjectileLocation: usize = 0x5B4; // Vector
    pub const m_hTarget: usize = 0x5C0; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_Bane_BrainSap {
    pub const brain_sap_damage: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_Bane_FiendsGrip {
    pub const m_hGripTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const fiend_grip_damage: usize = 0x5B4; // int32
}

pub mod C_DOTA_Ability_Beastmaster_CallOfTheWild_Hawk {
    pub const m_HawkList: usize = 0x5B0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_flRotation: usize = 0x5C8; // float32
}

pub mod C_DOTA_Ability_Beastmaster_WildAxes {
    pub const axe_damage: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_BountyHunter_Track {
    pub const m_hTrap: usize = 0x5B0; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_Brewmaster_DrunkenBrawler {
    pub const m_iBrawlActive: usize = 0x5B0; // int32
    pub const m_bUpdateIcons: usize = 0x5B4; // bool
    pub const m_nDrunkenBrawlerFX: usize = 0x5B8; // ParticleIndex_t
}

pub mod C_DOTA_Ability_Brewmaster_HurlBoulder {
    pub const m_iBounces: usize = 0x5B0; // int32
    pub const m_hHitEntities: usize = 0x5B8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_Brewmaster_PrimalSplit {
    pub const m_hPrimary: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const m_hSecondary: usize = 0x5B4; // CHandle< C_BaseEntity >
    pub const m_hTertiary: usize = 0x5B8; // CHandle< C_BaseEntity >
    pub const m_hFourth: usize = 0x5BC; // CHandle< C_BaseEntity >
    pub const m_fHurlBoulder_CooldownTime: usize = 0x5C0; // GameTime_t
    pub const m_fHDispelMagic_CooldownTime: usize = 0x5C4; // GameTime_t
    pub const m_fCyclone_CooldownTime: usize = 0x5C8; // GameTime_t
    pub const m_fWindWalk_CooldownTime: usize = 0x5CC; // GameTime_t
    pub const m_fAstralPulse_CooldownTime: usize = 0x5D0; // GameTime_t
}

pub mod C_DOTA_Ability_Broodmother_SpinWeb {
    pub const m_hWebs: usize = 0x5B0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_Broodmother_StickySnare {
    pub const m_vEndpoint: usize = 0x5B0; // Vector
    pub const duration: usize = 0x5BC; // float32
    pub const width: usize = 0x5C0; // int32
    pub const m_bStolenSnareCheck: usize = 0x5C4; // bool
    pub const m_vecSnares: usize = 0x5E0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_ChaosKnight_Chaos_Bolt {
    pub const hAlreadyHitList: usize = 0x5B0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_ChaosKnight_Reality_Rift {
    pub const m_flPercentage: usize = 0x5B0; // float32
    pub const m_FXIndex: usize = 0x5B8; // CUtlVector< ParticleIndex_t >
    pub const m_hRiftIllusion: usize = 0x5D0; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_Chen_HolyPersuasion {
    pub const m_hDominatedUnits: usize = 0x5B0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_Clinkz_BurningBarrage {
    pub const m_vStartPos: usize = 0x5B0; // Vector
    pub const m_iArrowProjectile: usize = 0x5BC; // int32
    pub const m_nFXIndex: usize = 0x5C0; // int32
    pub const arrow_speed: usize = 0x5C4; // float32
}

pub mod C_DOTA_Ability_Clinkz_Burning_Army {
    pub const range: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_Clinkz_DeathPact {
    pub const m_nDevourFirstSlot: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_Creature_Fire_Breath {
    pub const speed: usize = 0x5B0; // int32
    pub const projectile_count: usize = 0x5B4; // int32
    pub const rotation_angle: usize = 0x5B8; // float32
    pub const damage: usize = 0x5BC; // float32
    pub const radius: usize = 0x5C0; // float32
    pub const ctTimer: usize = 0x5C8; // CountdownTimer
    pub const m_vecStartRot: usize = 0x5E0; // Vector
    pub const m_vecEndRot: usize = 0x5EC; // Vector
}

pub mod C_DOTA_Ability_Creature_Ice_Breath {
    pub const speed: usize = 0x5B0; // int32
    pub const projectile_count: usize = 0x5B4; // int32
    pub const rotation_angle: usize = 0x5B8; // float32
    pub const damage: usize = 0x5BC; // float32
    pub const radius: usize = 0x5C0; // float32
    pub const slow_duration: usize = 0x5C4; // float32
    pub const ctTimer: usize = 0x5C8; // CountdownTimer
    pub const m_vecStartRot: usize = 0x5E0; // Vector
    pub const m_vecEndRot: usize = 0x5EC; // Vector
}

pub mod C_DOTA_Ability_CrystalMaiden_CrystalNova {
    pub const nova_damage: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_DarkSeer_Surge {
    pub const m_bIsAltCastState: usize = 0x5B0; // bool
}

pub mod C_DOTA_Ability_DarkSeer_Vacuum {
    pub const m_vPullLocation: usize = 0x5B0; // Vector
}

pub mod C_DOTA_Ability_DarkSeer_WallOfReplica {
    pub const width: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_DarkWillow_Terrorize {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
}

pub mod C_DOTA_Ability_DataDriven {
    pub const m_bProcsMagicStick: usize = 0x5B0; // bool
    pub const m_bIsSharedWithTeammates: usize = 0x5B1; // bool
    pub const m_bCastFilterRejectCaster: usize = 0x5B2; // bool
    pub const m_fAOERadius: usize = 0x5B4; // float32
    pub const m_CastAnimation: usize = 0x5B8; // int32
    pub const m_ModifierKVDescriptions: usize = 0x5C0; // CUtlVector< KeyValues* >
    pub const m_pOnChannelFinishKV: usize = 0x5D8; // KeyValues*
    pub const m_pOnChannelSucceededKV: usize = 0x5E0; // KeyValues*
    pub const m_pOnChannelInterruptedKV: usize = 0x5E8; // KeyValues*
    pub const m_pOnOwnerSpawnedKV: usize = 0x5F0; // KeyValues*
    pub const m_pOnOwnerDiedKV: usize = 0x5F8; // KeyValues*
    pub const m_pOnUpgradeKV: usize = 0x600; // KeyValues*
    pub const m_pOnProjectileHitUnitKV: usize = 0x608; // KeyValues*
    pub const m_pOnProjectileFinishKV: usize = 0x610; // KeyValues*
    pub const m_pOnSpellStartKV: usize = 0x618; // KeyValues*
    pub const m_pOnAbilityPhaseStartKV: usize = 0x620; // KeyValues*
    pub const m_pOnAbilityPhaseInterruptedKV: usize = 0x628; // KeyValues*
    pub const m_pOnToggleOnKV: usize = 0x630; // KeyValues*
    pub const m_pOnToggleOffKV: usize = 0x638; // KeyValues*
    pub const m_pOnCreatedKV: usize = 0x640; // KeyValues*
}

pub mod C_DOTA_Ability_Dawnbreaker_Solar_Guardian {
    pub const m_hThinker: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const m_bJumping: usize = 0x5B4; // bool
    pub const m_hTeleportTarget: usize = 0x5B8; // CHandle< C_BaseEntity >
    pub const m_nTPFXIndex: usize = 0x5BC; // ParticleIndex_t
    pub const m_nAoEFXIndex: usize = 0x5C0; // ParticleIndex_t
}

pub mod C_DOTA_Ability_Dazzle_ShadowWave {
    pub const m_hHitEntities: usize = 0x5B0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const bounce_radius: usize = 0x5C8; // int32
    pub const damage_radius: usize = 0x5CC; // int32
    pub const damage: usize = 0x5D0; // int32
    pub const max_targets: usize = 0x5D4; // int32
    pub const scepter_heal_pct: usize = 0x5D8; // float32
}

pub mod C_DOTA_Ability_DeathProphet_CarrionSwarm {
    pub const start_radius: usize = 0x5B0; // int32
    pub const end_radius: usize = 0x5B4; // int32
    pub const m_fStartTime: usize = 0x5B8; // GameTime_t
    pub const m_fTotalTime: usize = 0x5BC; // float32
    pub const m_nProjectileHandle: usize = 0x5C0; // int32
    pub const m_nFXIndex: usize = 0x5C4; // ParticleIndex_t
}

pub mod C_DOTA_Ability_DeathProphet_Exorcism {
    pub const m_SpiritInfos: usize = 0x5B0; // CUtlVector< sSpiritInfo* >
}

pub mod C_DOTA_Ability_DeathProphet_Silence {
    pub const m_vPos: usize = 0x5B0; // Vector
}

pub mod C_DOTA_Ability_DeathProphet_SpiritSiphon {
    pub const m_vStartPos: usize = 0x5B0; // Vector
    pub const m_iArrowProjectile: usize = 0x5BC; // int32
    pub const m_nFXIndex: usize = 0x5C0; // int32
}

pub mod C_DOTA_Ability_DoomBringer_Devour {
    pub const m_nDevourFirstSlot: usize = 0x5B0; // int32
    pub const m_bIsAltCastState: usize = 0x5B4; // bool
}

pub mod C_DOTA_Ability_DragonKnight_BreatheFire {
    pub const start_radius: usize = 0x5B0; // int32
    pub const end_radius: usize = 0x5B4; // int32
    pub const damage: usize = 0x5B8; // float32
    pub const m_vStartPos: usize = 0x5BC; // Vector
    pub const m_fStartTime: usize = 0x5C8; // GameTime_t
    pub const m_fTotalTime: usize = 0x5CC; // float32
}

pub mod C_DOTA_Ability_DrowRanger_FrostArrows {
    pub const shard_burst_damage_per_stack: usize = 0x5B0; // int32
    pub const shard_burst_slow_duration: usize = 0x5B4; // float32
}

pub mod C_DOTA_Ability_DrowRanger_Multishot {
    pub const m_vStartPos: usize = 0x5B0; // Vector
    pub const m_iArrowProjectile: usize = 0x5BC; // int32
    pub const m_nFXIndex: usize = 0x5C0; // int32
    pub const arrow_speed: usize = 0x5C4; // float32
    pub const arrow_spread: usize = 0x5C8; // int32
    pub const m_vHitTargets0: usize = 0x5D0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_vHitTargets1: usize = 0x5E8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_vHitTargets2: usize = 0x600; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_vHitTargets3: usize = 0x618; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_vHitTargets4: usize = 0x630; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_vHitTargets5: usize = 0x648; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_vHitTargets6: usize = 0x660; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_DrowRanger_WaveOfSilence {
    pub const wave_width: usize = 0x5B0; // int32
    pub const wave_speed: usize = 0x5B4; // float32
    pub const silence_duration: usize = 0x5B8; // float32
    pub const knockback_distance_max: usize = 0x5BC; // float32
    pub const m_nHeroesHit: usize = 0x5C0; // int32
}

pub mod C_DOTA_Ability_Drow_Ranger_Glacier {
    pub const shard_width: usize = 0x5B0; // int32
    pub const shard_count: usize = 0x5B4; // int32
    pub const shard_duration: usize = 0x5B8; // float32
    pub const shard_angle_step: usize = 0x5BC; // float32
    pub const shard_distance: usize = 0x5C0; // int32
    pub const m_vSpawnOrigin: usize = 0x5C4; // Vector
    pub const m_vDirection: usize = 0x5D0; // Vector
    pub const m_vecShards: usize = 0x5E0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_EarthSpirit_BoulderSmash {
    pub const speed: usize = 0x5B0; // int32
    pub const rock_damage: usize = 0x5B4; // int32
    pub const creep_multiplier: usize = 0x5B8; // float32
    pub const radius: usize = 0x5BC; // int32
    pub const rock_search_aoe: usize = 0x5C0; // int32
    pub const unit_distance: usize = 0x5C4; // float32
    pub const rock_distance: usize = 0x5C8; // float32
    pub const m_nProjectileID: usize = 0x5CC; // int32
    pub const m_hCursorTarget: usize = 0x5D0; // CHandle< C_BaseEntity >
    pub const m_bUsedStone: usize = 0x5D4; // bool
    pub const m_hTarget: usize = 0x5D8; // CHandle< C_BaseEntity >
    pub const m_bTargetStone: usize = 0x5DC; // bool
}

pub mod C_DOTA_Ability_EarthSpirit_GeomagneticGrip {
    pub const m_hTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_EarthSpirit_Magnetize {
    pub const cast_radius: usize = 0x5B0; // int32
    pub const rock_explosion_radius: usize = 0x5B4; // int32
    pub const damage_duration: usize = 0x5B8; // float32
}

pub mod C_DOTA_Ability_EarthSpirit_RollingBoulder {
    pub const radius: usize = 0x5B0; // int32
    pub const speed: usize = 0x5B4; // int32
    pub const rock_speed: usize = 0x5B8; // int32
    pub const damage: usize = 0x5BC; // int32
    pub const damage_str: usize = 0x5C0; // int32
    pub const distance: usize = 0x5C4; // float32
    pub const rock_distance: usize = 0x5C8; // float32
    pub const rock_distance_multiplier: usize = 0x5CC; // float32
    pub const slow_duration: usize = 0x5D0; // float32
    pub const m_nFXIndex: usize = 0x5D4; // ParticleIndex_t
    pub const m_boulderSetposBool: usize = 0x5D8; // bool
    pub const m_nProjectileID: usize = 0x5DC; // int32
    pub const m_vStartingLocation: usize = 0x5E0; // Vector
    pub const m_vProjectileLocation: usize = 0x5EC; // Vector
    pub const m_vDir: usize = 0x5F8; // Vector
    pub const m_vVel: usize = 0x604; // Vector
    pub const m_bUsedStone: usize = 0x610; // bool
    pub const m_vRollDirection: usize = 0x614; // Vector
}

pub mod C_DOTA_Ability_Earthshaker_EchoSlam {
    pub const echo_slam_damage_range: usize = 0x5B0; // int32
    pub const echo_slam_echo_search_range: usize = 0x5B4; // int32
    pub const echo_slam_echo_range: usize = 0x5B8; // int32
    pub const echo_slam_echo_damage: usize = 0x5BC; // int32
    pub const echo_slam_initial_damage: usize = 0x5C0; // int32
}

pub mod C_DOTA_Ability_Elder_Titan_AncestralSpirit {
    pub const speed: usize = 0x5B0; // int32
    pub const radius: usize = 0x5B4; // int32
    pub const buff_duration: usize = 0x5B8; // float32
    pub const spirit_duration: usize = 0x5BC; // float32
    pub const m_nCreepsHit: usize = 0x5C0; // int32
    pub const m_nHeroesHit: usize = 0x5C4; // int32
    pub const m_bIsReturning: usize = 0x5C8; // bool
    pub const m_hAncestralSpirit: usize = 0x5CC; // CHandle< C_BaseEntity >
    pub const m_nReturnFXIndex: usize = 0x5D0; // ParticleIndex_t
    pub const m_strMoveSpiritSwapAbility: usize = 0x5D8; // CUtlString
}

pub mod C_DOTA_Ability_Elder_Titan_EchoStomp {
    pub const m_nFXIndexTitan: usize = 0x5B0; // ParticleIndex_t
    pub const m_nFXIndexSpirit: usize = 0x5B4; // ParticleIndex_t
    pub const m_nFXIndexTitanB: usize = 0x5B8; // ParticleIndex_t
    pub const m_nFXIndexSpiritB: usize = 0x5BC; // ParticleIndex_t
    pub const radius: usize = 0x5C0; // int32
    pub const stomp_damage: usize = 0x5C4; // int32
    pub const sleep_duration: usize = 0x5C8; // float32
    pub const cast_time: usize = 0x5CC; // float32
    pub const m_vecStompedHeroes: usize = 0x5D0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_vecStompedHeroes_BuffCounted: usize = 0x5E8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_bStompedInvisibleHero: usize = 0x600; // bool
}

pub mod C_DOTA_Ability_EmberSpirit_Activate_FireRemnant {
    pub const m_nProjectileID: usize = 0x5B0; // int32
    pub const m_vStartLocation: usize = 0x5B4; // Vector
    pub const m_vProjectileLocation: usize = 0x5C0; // Vector
    pub const m_ProjectileAngles: usize = 0x5CC; // QAngle
    pub const m_hRemnantToKill: usize = 0x5D8; // CHandle< C_BaseEntity >
    pub const m_bProjectileStarted: usize = 0x5DC; // bool
    pub const hAlreadyHitList: usize = 0x5E0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_EmberSpirit_FireRemnant {
    pub const m_vRemnantData: usize = 0x5B0; // CUtlVector< RemnantData_t >
}

pub mod C_DOTA_Ability_EmberSpirit_SleightOfFist {
    pub const m_vCastLoc: usize = 0x5B0; // Vector
    pub const m_nHeroesKilled: usize = 0x5BC; // int32
    pub const m_hAttackEntities: usize = 0x5C0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_nFXMarkerIndex: usize = 0x5D8; // ParticleIndex_t
}

pub mod C_DOTA_Ability_Enchantress_Little_Friends {
    pub const duration: usize = 0x5B0; // float32
    pub const radius: usize = 0x5B4; // int32
}

pub mod C_DOTA_Ability_Enigma_Malefice {
    pub const tick_rate: usize = 0x5B0; // float32
    pub const stun_instances: usize = 0x5B4; // int32
    pub const m_flDuration: usize = 0x5B8; // float32
}

pub mod C_DOTA_Ability_EnragedWildkin_Tornado {
    pub const m_hTornado: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const m_hNeutralTarget: usize = 0x5B4; // CHandle< C_BaseEntity >
    pub const m_nFXIndex: usize = 0x5B8; // ParticleIndex_t
}

pub mod C_DOTA_Ability_FacelessVoid_TimeWalk {
    pub const speed: usize = 0x5B0; // int32
    pub const range: usize = 0x5B4; // int32
    pub const radius: usize = 0x5B8; // int32
}

pub mod C_DOTA_Ability_FacelessVoid_TimeWalk_Reverse {
    pub const speed: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_Frostivus2018_Huskar_Life_Break {
    pub const m_vProjectileLocation: usize = 0x5B0; // Vector
    pub const m_hTarget: usize = 0x5BC; // CHandle< C_BaseEntity >
    pub const m_bInterrupted: usize = 0x5C0; // bool
    pub const max_damage: usize = 0x5C4; // int32
}

pub mod C_DOTA_Ability_Frostivus2018_Magnataur_Skewer {
    pub const skewer_radius: usize = 0x5B0; // int32
    pub const skewer_speed: usize = 0x5B4; // int32
    pub const range: usize = 0x5B8; // int32
    pub const tree_radius: usize = 0x5BC; // int32
    pub const m_nTargetsHit: usize = 0x5C0; // int32
}

pub mod C_DOTA_Ability_Frostivus2018_Puck_DreamCoil {
    pub const m_hThinker: usize = 0x5B0; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_Frostivus2018_Rubick_GhostShip {
    pub const buff_duration: usize = 0x5B0; // float32
    pub const stun_duration: usize = 0x5B4; // float32
    pub const ghostship_width: usize = 0x5B8; // int32
    pub const ghostship_speed: usize = 0x5BC; // int32
    pub const ghostship_distance: usize = 0x5C0; // int32
    pub const m_vFinalDestination: usize = 0x5C4; // Vector
    pub const m_vStartingPoint: usize = 0x5D0; // Vector
}

pub mod C_DOTA_Ability_Frostivus2018_TrollWarlord_BattleTrance {
    pub const trance_duration: usize = 0x5B0; // float32
}

pub mod C_DOTA_Ability_Frostivus2018_Windrunner_Shackleshot {
    pub const shackle_count: usize = 0x5B0; // int32
    pub const m_vArrowStartPos: usize = 0x5B4; // Vector
    pub const m_hTarget: usize = 0x5C0; // CHandle< C_BaseEntity >
    pub const m_hEntitiesAffected: usize = 0x5C8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_Furion_Sprout {
    pub const m_iShardCount: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_Furion_Teleportation {
    pub const m_nFXIndexStart: usize = 0x5B0; // ParticleIndex_t
    pub const m_nFXIndexEnd: usize = 0x5B4; // ParticleIndex_t
    pub const m_nFXIndexEndTeam: usize = 0x5B8; // ParticleIndex_t
}

pub mod C_DOTA_Ability_Greevil_Miniboss_Blue_IceVortex {
    pub const vision_aoe: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_Greevil_Miniboss_Purple_VenomousGale {
    pub const duration: usize = 0x5B0; // float32
}

pub mod C_DOTA_Ability_Holdout_Omnislash {
    pub const image_travel_speed: usize = 0x5B0; // int32
    pub const image_radius: usize = 0x5B4; // int32
    pub const jugg_travel_speed: usize = 0x5B8; // int32
    pub const juggcounter: usize = 0x5BC; // int32
    pub const range: usize = 0x5C0; // int32
    pub const m_bFirstProjectileFinished: usize = 0x5C4; // bool
    pub const m_vCastDir: usize = 0x5C8; // Vector
    pub const m_vPos: usize = 0x5D4; // Vector
    pub const m_vJuggStartLocation: usize = 0x5E0; // Vector
    pub const m_flRange: usize = 0x5EC; // float32
    pub const m_hEntities: usize = 0x5F0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_Hoodwink_AcornShot {
    pub const m_nAcornTree: usize = 0x5B0; // uint32
    pub const projectile_speed: usize = 0x5B4; // float32
    pub const bounce_delay: usize = 0x5B8; // float32
    pub const bounce_range: usize = 0x5BC; // int32
    pub const bounce_count: usize = 0x5C0; // int32
    pub const m_bIsAltCastState: usize = 0x5C4; // bool
}

pub mod C_DOTA_Ability_Hoodwink_Bushwhack {
    pub const trap_radius: usize = 0x5B0; // int32
    pub const m_nProjectileHandle: usize = 0x5B4; // int32
    pub const m_nFXIndex: usize = 0x5B8; // ParticleIndex_t
}

pub mod C_DOTA_Ability_Hoodwink_HuntersBoomerang {
    pub const radius: usize = 0x5B0; // int32
    pub const speed: usize = 0x5B4; // int32
    pub const damage: usize = 0x5B8; // int32
    pub const mark_duration: usize = 0x5BC; // float32
    pub const m_hTarget: usize = 0x5C0; // CHandle< C_BaseEntity >
    pub const m_vecHitEntities: usize = 0x5C8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_Hoodwink_Sharpshooter {
    pub const max_charge_time: usize = 0x5B0; // float32
    pub const m_vStartPos: usize = 0x5B4; // Vector
    pub const m_nFXIndex: usize = 0x5C0; // ParticleIndex_t
}

pub mod C_DOTA_Ability_Huskar_Life_Break {
    pub const m_vProjectileLocation: usize = 0x5B0; // Vector
    pub const m_hTarget: usize = 0x5BC; // CHandle< C_BaseEntity >
    pub const m_bInterrupted: usize = 0x5C0; // bool
}

pub mod C_DOTA_Ability_IceShaman_IncendiaryBomb {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
}

pub mod C_DOTA_Ability_Invoker_ChaosMeteor {
    pub const area_of_effect: usize = 0x5C0; // int32
    pub const damage_interval: usize = 0x5C4; // float32
    pub const vision_distance: usize = 0x5C8; // int32
    pub const end_vision_duration: usize = 0x5CC; // float32
    pub const main_damage: usize = 0x5D0; // float32
    pub const burn_duration: usize = 0x5D4; // float32
    pub const burn_dps: usize = 0x5D8; // float32
}

pub mod C_DOTA_Ability_Invoker_DeafeningBlast {
    pub const end_vision_duration: usize = 0x5C0; // float32
    pub const damage: usize = 0x5C4; // float32
    pub const knockback_duration: usize = 0x5C8; // float32
    pub const disarm_duration: usize = 0x5CC; // float32
}

pub mod C_DOTA_Ability_Invoker_Invoke {
    pub const max_invoked_spells: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_Invoker_Tornado {
    pub const vision_distance: usize = 0x5C0; // int32
    pub const end_vision_duration: usize = 0x5C4; // float32
    pub const lift_duration: usize = 0x5C8; // float32
    pub const base_damage: usize = 0x5CC; // float32
    pub const quas_damage: usize = 0x5D0; // float32
    pub const wex_damage: usize = 0x5D4; // float32
}

pub mod C_DOTA_Ability_Juggernaut_BladeFury {
    pub const blade_fury_damage: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_JungleSpirit_Volcano_Eruption {
    pub const m_iProjectile: usize = 0x5B8; // int32
    pub const primary_explosion_radius: usize = 0x5BC; // int32
    pub const secondary_explosion_radius: usize = 0x5C0; // int32
    pub const split_radius: usize = 0x5C4; // int32
    pub const szProjectileFXName: usize = 0x5C8; // char*
    pub const szGroundMarkerFXName: usize = 0x5D0; // char*
}

pub mod C_DOTA_Ability_KeeperOfTheLight_Illuminate {
    pub const m_fStartTime: usize = 0x5B0; // GameTime_t
    pub const m_iProjectile: usize = 0x5B4; // int32
    pub const m_nFXIndex: usize = 0x5B8; // ParticleIndex_t
    pub const m_nFXIndexB: usize = 0x5BC; // ParticleIndex_t
    pub const m_vPos: usize = 0x5C0; // Vector
    pub const total_damage: usize = 0x5CC; // int32
    pub const m_bStarted: usize = 0x5D0; // bool
}

pub mod C_DOTA_Ability_KeeperOfTheLight_SpiritFormIlluminate {
    pub const m_hThinker: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const max_channel_time: usize = 0x5B4; // float32
    pub const total_damage: usize = 0x5B8; // int32
    pub const m_fPower: usize = 0x5BC; // float32
    pub const m_fStartTime: usize = 0x5C0; // GameTime_t
    pub const m_nFXIndex: usize = 0x5C4; // ParticleIndex_t
}

pub mod C_DOTA_Ability_Kunkka_GhostShip {
    pub const buff_duration: usize = 0x5B0; // float32
    pub const stun_duration: usize = 0x5B4; // float32
    pub const ghostship_width: usize = 0x5B8; // int32
    pub const ghostship_width_scepter: usize = 0x5BC; // int32
    pub const m_vFinalDestination: usize = 0x5C0; // Vector
    pub const m_vStartingPoint: usize = 0x5CC; // Vector
}

pub mod C_DOTA_Ability_Kunkka_Tidal_Wave {
    pub const hAlreadyHitList: usize = 0x5B0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_vStart: usize = 0x5C8; // Vector
}

pub mod C_DOTA_Ability_Kunkka_XMarksTheSpot {
    pub const m_hThinker: usize = 0x5B0; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_Lich_ChainFrost {
    pub const jump_range: usize = 0x5B0; // int32
    pub const jumps: usize = 0x5B4; // int32
    pub const slow_duration: usize = 0x5B8; // float32
    pub const vision_radius: usize = 0x5BC; // int32
    pub const projectile_speed: usize = 0x5C0; // int32
    pub const initial_projectile_speed: usize = 0x5C4; // int32
}

pub mod C_DOTA_Ability_Lich_Ice_Spire {
    pub const aura_radius: usize = 0x5B0; // int32
    pub const duration: usize = 0x5B4; // float32
}

pub mod C_DOTA_Ability_Lich_Sinister_Gaze {
    pub const m_flLongestGazeDuration: usize = 0x5B0; // float32
    pub const m_hAffectedEntities: usize = 0x5B8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_Lina_DragonSlave {
    pub const dragon_slave_distance: usize = 0x5B0; // int32
    pub const dragon_slave_burn_duration: usize = 0x5B4; // float32
    pub const m_flDamage: usize = 0x5B8; // float32
}

pub mod C_DOTA_Ability_Lina_LagunaBlade {
    pub const m_iDamage: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_Lion_FingerOfDeath {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
}

pub mod C_DOTA_Ability_Lion_Impale {
    pub const width: usize = 0x5B0; // int32
    pub const duration: usize = 0x5B4; // float32
    pub const speed: usize = 0x5B8; // int32
    pub const length_buffer: usize = 0x5BC; // float32
    pub const range: usize = 0x5C0; // float32
    pub const m_iDefaultCastRange: usize = 0x5C4; // int32
    pub const pierces_immunity: usize = 0x5C8; // int32
    pub const m_hHitEntities: usize = 0x5D0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_Lion_ManaDrain {
    pub const m_Victims: usize = 0x5B0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_LoneDruid_Rabid {
    pub const rabid_duration: usize = 0x5B0; // float32
}

pub mod C_DOTA_Ability_LoneDruid_SpiritBear {
    pub const m_bLevelChanged: usize = 0x5B0; // bool
    pub const m_hBear: usize = 0x5B4; // CHandle< C_BaseEntity >
    pub const m_hPreBear: usize = 0x5B8; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_LoneDruid_SpiritBear_Return {
    pub const m_nFXOrigin: usize = 0x5B0; // ParticleIndex_t
}

pub mod C_DOTA_Ability_LoneDruid_TrueForm_BattleCry {
    pub const cry_duration: usize = 0x5B0; // float32
}

pub mod C_DOTA_Ability_Luna_MoonGlaive {
    pub const m_iAttackIndex: usize = 0x5B0; // int32
    pub const m_GlaiveInfo: usize = 0x5B8; // CUtlVector< sGlaiveInfo >
}

pub mod C_DOTA_Ability_Lycan_SummonWolves {
    pub const szUnitName: usize = 0x5B0; // char[260]
    pub const wolf_index: usize = 0x6B4; // int32
    pub const wolf_duration: usize = 0x6B8; // float32
    pub const m_hExistingUnits: usize = 0x6C0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_Magnataur_ReversePolarity {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
    pub const m_vPullLocation: usize = 0x5B4; // Vector
}

pub mod C_DOTA_Ability_Magnataur_Shockwave {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
    pub const hAlreadyHitList: usize = 0x5B8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const hAlreadyHitListReturning: usize = 0x5D0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_Magnataur_Skewer {
    pub const skewer_radius: usize = 0x5B0; // int32
    pub const skewer_speed: usize = 0x5B4; // int32
    pub const range: usize = 0x5B8; // int32
    pub const tree_radius: usize = 0x5BC; // int32
    pub const m_nTargetsHit: usize = 0x5C0; // int32
}

pub mod C_DOTA_Ability_Medusa_MysticSnake {
    pub const radius: usize = 0x5B0; // int32
    pub const snake_jumps: usize = 0x5B4; // int32
    pub const snake_damage: usize = 0x5B8; // int32
    pub const snake_damage_pct: usize = 0x5BC; // int32
    pub const snake_mana_steal: usize = 0x5C0; // int32
    pub const snake_scale: usize = 0x5C4; // int32
    pub const snake_speed_scale: usize = 0x5C8; // int32
    pub const initial_speed: usize = 0x5CC; // int32
    pub const return_speed: usize = 0x5D0; // int32
    pub const jump_delay: usize = 0x5D4; // float32
    pub const slow_duration: usize = 0x5D8; // float32
}

pub mod C_DOTA_Ability_Meepo_DividedWeStand {
    pub const m_nWhichDividedWeStand: usize = 0x5B0; // int32
    pub const m_nNumDividedWeStand: usize = 0x5B4; // int32
    pub const m_entPrimeDividedWeStand: usize = 0x5B8; // CHandle< C_DOTA_Ability_Meepo_DividedWeStand >
    pub const m_entNextDividedWeStand: usize = 0x5BC; // CHandle< C_DOTA_Ability_Meepo_DividedWeStand >
}

pub mod C_DOTA_Ability_Meepo_MegaMeepo {
    pub const hPreviousMeepo: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const hMegameepoFrame: usize = 0x5B4; // CHandle< C_BaseEntity >
    pub const hListOfMeepos: usize = 0x5B8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_bHasSwappedAbilities: usize = 0x5D0; // bool
}

pub mod C_DOTA_Ability_Meepo_Megameepo_Fling {
    pub const vision_radius: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_Mirana_Arrow {
    pub const scepter_radius: usize = 0x5B0; // int32
    pub const m_vStartPos: usize = 0x5B4; // Vector
    pub const m_nFXIndex: usize = 0x5C0; // ParticleIndex_t
    pub const hAlreadyHitList: usize = 0x5C8; // CUtlVector< CHandle< C_BaseEntity > >[2]
    pub const hStarfallList: usize = 0x5F8; // CUtlVector< CHandle< C_BaseEntity > >[2]
    pub const m_nActiveArrow: usize = 0x628; // int32
}

pub mod C_DOTA_Ability_Mirana_Leap {
    pub const m_bIsVectorTargeted: usize = 0x5B0; // bool
    pub const m_vEndpoint: usize = 0x5B4; // Vector
    pub const m_vLandPosition: usize = 0x5C0; // Vector
}

pub mod C_DOTA_Ability_Mirana_Starfall {
    pub const damage: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_MonkeyKing_Boundless_Strike {
    pub const strike_cast_range: usize = 0x5B0; // int32
    pub const strike_radius: usize = 0x5B4; // int32
    pub const m_nFXIndex: usize = 0x5B8; // ParticleIndex_t
    pub const m_bIsAltCastState: usize = 0x5BC; // bool
}

pub mod C_DOTA_Ability_MonkeyKing_FurArmy {
    pub const m_hThinker: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const m_nFXIndex: usize = 0x5B4; // ParticleIndex_t
    pub const num_first_soldiers: usize = 0x5B8; // int32
    pub const num_second_soldiers: usize = 0x5BC; // int32
    pub const m_bCreateMonkeys: usize = 0x5C0; // bool
    pub const m_flNextCreationTime: usize = 0x5C4; // GameTime_t
    pub const m_flScepterTime: usize = 0x5C8; // GameTime_t
    pub const m_vecSoldiers: usize = 0x5D0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_MonkeyKing_Spring {
    pub const m_vPos: usize = 0x5B0; // Vector
    pub const m_fStartChannelTime: usize = 0x5BC; // GameTime_t
    pub const m_hThinker: usize = 0x5C0; // CHandle< C_BaseEntity >
    pub const m_nFxIndex: usize = 0x5C4; // ParticleIndex_t
    pub const m_nRefCount: usize = 0x5C8; // int32
}

pub mod C_DOTA_Ability_MonkeyKing_TreeDance {
    pub const perched_jump_distance: usize = 0x5B0; // int32
    pub const ground_jump_distance: usize = 0x5B4; // int32
}

pub mod C_DOTA_Ability_Morphling_MorphReplicate {
    pub const m_flOldHealthPct: usize = 0x5B0; // float32
    pub const m_flOldManaPct: usize = 0x5B4; // float32
}

pub mod C_DOTA_Ability_Morphling_Replicate {
    pub const m_hTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const m_hScepterIllusion: usize = 0x5B4; // CHandle< C_BaseEntity >
    pub const m_bIsAltCastState: usize = 0x5B8; // bool
}

pub mod C_DOTA_Ability_Morty_Hop {
    pub const max_distance: usize = 0x5B0; // float32
    pub const m_vTarget: usize = 0x5B4; // Vector
    pub const m_iTrackerProjectile: usize = 0x5C0; // int32
}

pub mod C_DOTA_Ability_Muerta_DeadShot {
    pub const radius: usize = 0x5B0; // int32
    pub const ricochet_radius_start: usize = 0x5B4; // int32
    pub const ricochet_radius_end: usize = 0x5B8; // int32
}

pub mod C_DOTA_Ability_Muerta_PartingShot {
    pub const m_hAbilitySoulEntity: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const m_hTarget: usize = 0x5B4; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_NagaSiren_MirrorImage {
    pub const m_hIllusions: usize = 0x5B0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_Naga_Siren_Reel_In {
    pub const affectedEntities: usize = 0x5B0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_Necrolyte_Death_Pulse {
    pub const m_bIsDeathSeekerCast: usize = 0x5B8; // bool
}

pub mod C_DOTA_Ability_Nevermore_Requiem {
    pub const requiem_line_width_start: usize = 0x5B0; // int32
    pub const requiem_line_width_end: usize = 0x5B4; // int32
    pub const m_nCachedSouls: usize = 0x5B8; // int32
    pub const m_nFXIndex: usize = 0x5BC; // ParticleIndex_t
    pub const m_nKilleater_nLines: usize = 0x5C0; // int32
    pub const m_vecHeroesReqd: usize = 0x5C8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_Nevermore_Shadowraze {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
    pub const m_nFXIndexB: usize = 0x5B4; // ParticleIndex_t
}

pub mod C_DOTA_Ability_Nian_Apocalypse {
    pub const area_of_effect: usize = 0x5B0; // int32
    pub const m_nfxIndex_roar: usize = 0x5B4; // ParticleIndex_t
    pub const fire_interval: usize = 0x5B8; // float32
    pub const delay: usize = 0x5BC; // float32
    pub const target_range: usize = 0x5C0; // float32
    pub const m_ctTimer: usize = 0x5C8; // CountdownTimer
    pub const m_flTiming: usize = 0x5E0; // float32
}

pub mod C_DOTA_Ability_Nian_Hurricane {
    pub const min_distance: usize = 0x5B0; // int32
    pub const max_distance: usize = 0x5B4; // int32
    pub const torrent_count: usize = 0x5B8; // int32
    pub const fire_interval: usize = 0x5BC; // float32
    pub const pull_switch_interval: usize = 0x5C0; // float32
    pub const game_time_wind_activation: usize = 0x5C4; // float32
    pub const m_ctPullTimer: usize = 0x5C8; // CountdownTimer
    pub const m_ctTimer: usize = 0x5E0; // CountdownTimer
    pub const m_flTiming: usize = 0x5F8; // float32
    pub const m_bForward: usize = 0x5FC; // bool
    pub const m_bUseWind: usize = 0x5FD; // bool
    pub const m_nFXIndex: usize = 0x600; // ParticleIndex_t
    pub const m_nfxIndex_roar: usize = 0x604; // ParticleIndex_t
}

pub mod C_DOTA_Ability_Nian_Roar {
    pub const base_projectiles: usize = 0x5B0; // int32
    pub const max_projectiles: usize = 0x5B4; // int32
    pub const projectile_step: usize = 0x5B8; // int32
    pub const base_speed: usize = 0x5BC; // int32
    pub const speed_step: usize = 0x5C0; // int32
    pub const initial_radius: usize = 0x5C4; // int32
    pub const end_radius: usize = 0x5C8; // int32
    pub const damage: usize = 0x5CC; // int32
    pub const base_interval: usize = 0x5D0; // float32
    pub const interval_step: usize = 0x5D4; // float32
    pub const m_nCastCount: usize = 0x5D8; // int32
    pub const m_nProjectiles: usize = 0x5DC; // int32
    pub const m_nWaveCount: usize = 0x5E0; // int32
    pub const m_ctTimer: usize = 0x5E8; // CountdownTimer
    pub const m_flTiming: usize = 0x600; // float32
    pub const m_bScriptRoar: usize = 0x604; // bool
}

pub mod C_DOTA_Ability_Nian_Waterball {
    pub const m_ctTimer: usize = 0x5B0; // CountdownTimer
    pub const m_hEntities: usize = 0x5C8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_Nian_Whirlpool {
    pub const pool_count: usize = 0x5B0; // int32
    pub const min_distance: usize = 0x5B4; // int32
    pub const max_distance: usize = 0x5B8; // int32
    pub const pull_radius: usize = 0x5BC; // int32
    pub const fire_interval: usize = 0x5C0; // float32
    pub const m_ctTimer: usize = 0x5C8; // CountdownTimer
    pub const m_flTiming: usize = 0x5E0; // float32
}

pub mod C_DOTA_Ability_Nyx_Assassin_Impale {
    pub const hAlreadyHitList: usize = 0x5B0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const width: usize = 0x5C8; // int32
    pub const duration: usize = 0x5CC; // float32
    pub const length: usize = 0x5D0; // int32
    pub const speed: usize = 0x5D4; // int32
    pub const vOriginalCast: usize = 0x5D8; // Vector
}

pub mod C_DOTA_Ability_Obsidian_Destroyer_AstralImprisonment {
    pub const m_hImprisonedUnit: usize = 0x5B0; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_OgreBruiser_OgreSmash {
    pub const radius: usize = 0x5B0; // int32
    pub const hero_stun_duration: usize = 0x5B4; // float32
    pub const damage: usize = 0x5B8; // int32
    pub const damage_pct: usize = 0x5BC; // float32
    pub const flCooldown: usize = 0x5C0; // GameTime_t
}

pub mod C_DOTA_Ability_Ogre_Magi_Fireblast {
    pub const m_nMostRecentMulticastCount: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_Ogre_Magi_Ignite {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
    pub const m_nMostRecentMulticastCount: usize = 0x5B4; // int32
}

pub mod C_DOTA_Ability_Ogre_Magi_Unrefined_Fireblast {
    pub const m_nMostRecentMulticastCount: usize = 0x5B8; // int32
}

pub mod C_DOTA_Ability_Oracle_FatesEdict {
    pub const m_bTargetIsAlly: usize = 0x5B0; // bool
}

pub mod C_DOTA_Ability_Oracle_FortunesEnd {
    pub const damage: usize = 0x5B0; // int32
    pub const radius: usize = 0x5B4; // int32
    pub const bolt_speed: usize = 0x5B8; // int32
    pub const maximum_purge_duration: usize = 0x5BC; // float32
    pub const minimum_purge_duration: usize = 0x5C0; // float32
    pub const m_flStartTime: usize = 0x5C4; // GameTime_t
    pub const m_flDuration: usize = 0x5C8; // float32
    pub const m_flDamage: usize = 0x5CC; // float32
    pub const m_bAbsorbed: usize = 0x5D0; // bool
    pub const m_hTarget: usize = 0x5D4; // CHandle< C_BaseEntity >
    pub const m_nFXIndex: usize = 0x5D8; // ParticleIndex_t
}

pub mod C_DOTA_Ability_Oracle_PurifyingFlames {
    pub const m_bTargetIsAlly: usize = 0x5B0; // bool
    pub const m_hRecentTarget: usize = 0x5B4; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_Pangolier_Gyroshell {
    pub const m_nFxIndex: usize = 0x5B0; // ParticleIndex_t
}

pub mod C_DOTA_Ability_Pangolier_Rollup {
    pub const m_nFxIndex: usize = 0x5B0; // ParticleIndex_t
}

pub mod C_DOTA_Ability_Pangolier_Swashbuckle {
    pub const dash_speed: usize = 0x5B0; // int32
    pub const start_radius: usize = 0x5B4; // int32
    pub const end_radius: usize = 0x5B8; // int32
    pub const range: usize = 0x5BC; // int32
    pub const damage: usize = 0x5C0; // int32
}

pub mod C_DOTA_Ability_Phoenix_IcarusDive {
    pub const hp_cost_perc: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_Phoenix_LaunchFireSpirit {
    pub const spirit_speed: usize = 0x5B0; // int32
    pub const duration: usize = 0x5B4; // float32
    pub const radius: usize = 0x5B8; // int32
    pub const m_nFXIndex: usize = 0x5BC; // ParticleIndex_t
}

pub mod C_DOTA_Ability_PineCone_AcornShot {
    pub const m_nAcornTree: usize = 0x5B0; // uint32
    pub const projectile_speed: usize = 0x5B4; // float32
    pub const bounce_delay: usize = 0x5B8; // float32
    pub const bounce_range: usize = 0x5BC; // int32
    pub const bounce_count: usize = 0x5C0; // int32
}

pub mod C_DOTA_Ability_PrimalBeast_Onslaught {
    pub const max_charge_time: usize = 0x5B0; // float32
    pub const knockback_distance: usize = 0x5B4; // float32
    pub const knockback_damage: usize = 0x5B8; // int32
    pub const collision_radius: usize = 0x5BC; // int32
    pub const max_distance: usize = 0x5C0; // int32
    pub const m_vStartPos: usize = 0x5C4; // Vector
    pub const m_nFXIndex: usize = 0x5D0; // ParticleIndex_t
    pub const m_nProjectileID: usize = 0x5D4; // int32
}

pub mod C_DOTA_Ability_PrimalBeast_Pulverize {
    pub const m_hTarget: usize = 0x5B0; // C_DOTA_BaseNPC*
}

pub mod C_DOTA_Ability_PrimalBeast_Uproar {
    pub const m_iUproarStatus: usize = 0x5B0; // int32
    pub const m_bUpdateIcons: usize = 0x5B4; // bool
}

pub mod C_DOTA_Ability_Primal_Companion {
    pub const m_hCurrentBrewling: usize = 0x5B0; // CHandle< C_DOTA_BaseNPC >
}

pub mod C_DOTA_Ability_Puck_DreamCoil {
    pub const m_hThinker: usize = 0x5B0; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_Puck_IllusoryOrb {
    pub const m_iProjectile: usize = 0x5B0; // int32
    pub const m_ViewerTimer: usize = 0x5B8; // CountdownTimer
    pub const orb_vision: usize = 0x5D0; // int32
    pub const vision_duration: usize = 0x5D4; // float32
    pub const damage: usize = 0x5D8; // int32
}

pub mod C_DOTA_Ability_Pudge_FleshHeap {
    pub const m_iKills: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_Pudge_Rot {
    pub const rot_damage: usize = 0x5B0; // int32
    pub const m_flLastRotTime: usize = 0x5B4; // GameTime_t
}

pub mod C_DOTA_Ability_Pugna_LifeDrain {
    pub const m_hTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_Pugna_NetherWard {
    pub const vecAbilitiesUsed: usize = 0x5B0; // CUtlVector< sAbilityHistory >
}

pub mod C_DOTA_Ability_QueenOfPain_ScreamOfPain {
    pub const damage: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_QueenOfPain_ShadowStrike {
    pub const projectile_speed: usize = 0x5B0; // Vector
}

pub mod C_DOTA_Ability_QueenOfPain_SonicWave {
    pub const m_fStartTime: usize = 0x5B0; // GameTime_t
    pub const m_fTotalTime: usize = 0x5B4; // float32
    pub const m_nFXIndex: usize = 0x5B8; // ParticleIndex_t
    pub const starting_aoe: usize = 0x5BC; // int32
    pub const final_aoe: usize = 0x5C0; // int32
}

pub mod C_DOTA_Ability_Rattletrap_Hookshot {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
    pub const m_vProjectileVelocity: usize = 0x5B4; // Vector
    pub const m_bRetract: usize = 0x5C0; // bool
}

pub mod C_DOTA_Ability_Rattletrap_RocketFlare {
    pub const m_vecEnemyHeroesInFog: usize = 0x5B0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_Razor_StaticLink {
    pub const m_iLinkIndex: usize = 0x5B0; // int32
    pub const m_ViewerTimer: usize = 0x5B8; // CountdownTimer
    pub const vision_radius: usize = 0x5D0; // int32
    pub const vision_duration: usize = 0x5D4; // float32
    pub const m_bIsAltCastState: usize = 0x5D8; // bool
}

pub mod C_DOTA_Ability_Roshan_Teleport {
    pub const m_nFXIndexStart: usize = 0x5B0; // ParticleIndex_t
    pub const m_nFXIndexEnd: usize = 0x5B4; // ParticleIndex_t
    pub const m_nFXIndexEndTeam: usize = 0x5B8; // ParticleIndex_t
}

pub mod C_DOTA_Ability_Rubick_SpellSteal {
    pub const m_ActivityModifier: usize = 0x5B0; // char[256]
    pub const m_fStolenCastPoint: usize = 0x6B0; // float32
    pub const m_hStealTarget: usize = 0x6B4; // CHandle< C_BaseEntity >
    pub const m_hStealAbility: usize = 0x6B8; // CHandle< C_DOTABaseAbility >
}

pub mod C_DOTA_Ability_Rubick_Telekinesis {
    pub const m_vStartLocation: usize = 0x5B0; // Vector
    pub const m_vLandLocation: usize = 0x5BC; // Vector
    pub const m_flStartTime: usize = 0x5C8; // GameTime_t
    pub const m_pTarget: usize = 0x5D0; // C_DOTA_BaseNPC*
}

pub mod C_DOTA_Ability_SandKing_BurrowStrike {
    pub const burrow_width: usize = 0x5B0; // int32
    pub const burrow_speed: usize = 0x5B4; // int32
    pub const burrow_speed_scepter: usize = 0x5B8; // int32
    pub const burrow_anim_time: usize = 0x5BC; // float32
}

pub mod C_DOTA_Ability_SandKing_Epicenter {
    pub const m_nFXEpicenterIndex: usize = 0x5B0; // ParticleIndex_t
    pub const m_nFXIndex: usize = 0x5B4; // ParticleIndex_t
}

pub mod C_DOTA_Ability_SatyrSoulstealer_ManaBurn {
    pub const hNeutralTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const m_flTimeBuffer: usize = 0x5B4; // GameTime_t
}

pub mod C_DOTA_Ability_SatyrTrickster_Purge {
    pub const m_flTargetTime: usize = 0x5B0; // GameTime_t
}

pub mod C_DOTA_Ability_ShadowShaman_Serpentine {
    pub const length: usize = 0x5B0; // int32
    pub const count: usize = 0x5B4; // int32
}

pub mod C_DOTA_Ability_ShadowShaman_Shackles {
    pub const m_hShackleTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const nShackleFXIndex: usize = 0x5B4; // ParticleIndex_t
    pub const hWardList: usize = 0x5B8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_Shadow_Demon_Demonic_Cleanse {
    pub const m_bGrantedScepterCharges: usize = 0x5B0; // bool
}

pub mod C_DOTA_Ability_Shadow_Demon_Demonic_Purge {
    pub const m_bGrantedScepterCharges: usize = 0x5B0; // bool
}

pub mod C_DOTA_Ability_Shadow_Demon_Disruption {
    pub const m_hDisruptedUnit: usize = 0x5B0; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_Shredder_Chakram {
    pub const radius: usize = 0x5B0; // float32
    pub const speed: usize = 0x5B4; // float32
    pub const pass_slow_duration: usize = 0x5B8; // float32
    pub const pass_damage: usize = 0x5BC; // int32
    pub const m_vEndLocation: usize = 0x5C0; // Vector
    pub const m_fZCoord: usize = 0x5CC; // float32
    pub const m_bIsReturning: usize = 0x5D0; // bool
    pub const m_nFXIndex: usize = 0x5D4; // ParticleIndex_t
    pub const m_nFXIndexB: usize = 0x5D8; // ParticleIndex_t
    pub const m_nFXIndexC: usize = 0x5DC; // ParticleIndex_t
    pub const m_nProjectileIndex: usize = 0x5E0; // int32
    pub const m_hThinker: usize = 0x5E4; // CHandle< C_BaseEntity >
    pub const m_hReturnHits: usize = 0x5E8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_Shredder_TimberChain {
    pub const chain_radius: usize = 0x5B0; // int32
    pub const m_nFXIndex: usize = 0x5B4; // ParticleIndex_t
    pub const m_vProjectileVelocity: usize = 0x5B8; // Vector
    pub const m_bRetract: usize = 0x5C4; // bool
}

pub mod C_DOTA_Ability_Shredder_WhirlingDeath {
    pub const whirling_radius: usize = 0x5B0; // int32
    pub const whirling_damage: usize = 0x5B4; // int32
    pub const whirling_tick: usize = 0x5B8; // float32
    pub const duration: usize = 0x5BC; // float32
    pub const tree_damage_scale: usize = 0x5C0; // int32
}

pub mod C_DOTA_Ability_Silencer_GlaivesOfWisdom {
    pub const m_iAttackIndex: usize = 0x5B0; // int32
    pub const m_GlaiveInfo: usize = 0x5B8; // CUtlVector< sGlaiveInfoSilencer >
}

pub mod C_DOTA_Ability_SkeletonKing_Reincarnation {
    pub const m_bShouldUseResources: usize = 0x5B0; // bool
}

pub mod C_DOTA_Ability_Skywrath_Mage_Arcane_Bolt {
    pub const bolt_vision: usize = 0x5B0; // int32
    pub const vision_duration: usize = 0x5B4; // float32
    pub const m_flDamage: usize = 0x5B8; // float32
    pub const m_nFXIndex: usize = 0x5BC; // int32
}

pub mod C_DOTA_Ability_Skywrath_Mage_Concussive_Shot {
    pub const speed: usize = 0x5B0; // int32
    pub const slow_radius: usize = 0x5B4; // int32
    pub const damage: usize = 0x5B8; // int32
    pub const shot_vision: usize = 0x5BC; // int32
    pub const slow_duration: usize = 0x5C0; // float32
    pub const vision_duration: usize = 0x5C4; // float32
}

pub mod C_DOTA_Ability_Sniper_Assassinate {
    pub const m_hTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const m_iIndex: usize = 0x5B4; // ParticleIndex_t
    pub const cooldown_reduction_on_kill: usize = 0x5B8; // float32
}

pub mod C_DOTA_Ability_Sniper_Concussive_Grenade {
    pub const m_iProjectile: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_Spectre_SpectralDagger {
    pub const dagger_path_duration: usize = 0x5B0; // float32
    pub const hero_path_duration: usize = 0x5B4; // float32
    pub const m_fCreateInterval: usize = 0x5B8; // float32
    pub const m_fLastCreate: usize = 0x5BC; // GameTime_t
    pub const m_vecLastPosition: usize = 0x5C0; // Vector
    pub const m_bIsTrackingProjectile: usize = 0x5CC; // bool
    pub const m_hTrackingProjectileHits: usize = 0x5D0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_hUnitsHit: usize = 0x5E8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_hTrackingTarget: usize = 0x600; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_SpiritBreaker_ChargeOfDarkness {
    pub const m_vChargeStartPos: usize = 0x5B0; // Vector
}

pub mod C_DOTA_Ability_StormSpirit_BallLightning {
    pub const m_bHasAutoRemnantTalent: usize = 0x5B0; // bool
    pub const m_fAutoRemnantInterval: usize = 0x5B4; // float32
    pub const ball_lightning_initial_mana_base: usize = 0x5B8; // int32
    pub const ball_lightning_initial_mana_percentage: usize = 0x5BC; // float32
    pub const ball_lightning_travel_cost_base: usize = 0x5C0; // int32
    pub const ball_lightning_travel_cost_percent: usize = 0x5C4; // float32
    pub const m_iProjectileID: usize = 0x5C8; // int32
    pub const m_vStartLocation: usize = 0x5CC; // Vector
    pub const m_vProjectileLocation: usize = 0x5D8; // Vector
    pub const m_fDistanceAccumulator: usize = 0x5E4; // float32
    pub const m_fTalentDistanceAccumulator: usize = 0x5E8; // float32
    pub const scepter_remnant_interval: usize = 0x5EC; // int32
}

pub mod C_DOTA_Ability_StormSpirit_Overload {
    pub const m_iAttackIndex: usize = 0x5B0; // int32
    pub const m_GlaiveInfo: usize = 0x5B8; // CUtlVector< sGlaiveInfoStorm >
}

pub mod C_DOTA_Ability_StormSpirit_StaticRemnant {
    pub const m_vecCastPos: usize = 0x5B0; // Vector
}

pub mod C_DOTA_Ability_Sven_StormBolt {
    pub const vision_radius: usize = 0x5B0; // int32
    pub const m_bIsAltCastState: usize = 0x5B4; // bool
}

pub mod C_DOTA_Ability_Techies_Suicide {
    pub const m_unSuicideKills: usize = 0x5B0; // uint32
}

pub mod C_DOTA_Ability_TemplarAssassin_Trap_Teleport {
    pub const m_hTrap: usize = 0x5B0; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_Terrorblade_Reflection {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
}

pub mod C_DOTA_Ability_Tidehunter_ArmOfTheDeep {
    pub const hAlreadyHitList: usize = 0x5B0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const duration: usize = 0x5C8; // float32
    pub const speed: usize = 0x5CC; // int32
    pub const damage: usize = 0x5D0; // float32
    pub const range: usize = 0x5D4; // float32
}

pub mod C_DOTA_Ability_Tidehunter_DeadInTheWater {
    pub const radius: usize = 0x5B0; // int32
    pub const duration: usize = 0x5B4; // float32
    pub const chain_length: usize = 0x5B8; // int32
}

pub mod C_DOTA_Ability_Tidehunter_Gush {
    pub const gush_damage: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_Tidehunter_Ravage {
    pub const m_hEntsHit: usize = 0x5B0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_bAwardedKillEater: usize = 0x5C8; // bool
    pub const duration: usize = 0x5CC; // float32
}

pub mod C_DOTA_Ability_Tinker_HeatSeekingMissile {
    pub const m_nTargetsKilled: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_Tinker_Keen_Teleport {
    pub const m_hTeleportTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const m_nFXOrigin: usize = 0x5B4; // ParticleIndex_t
    pub const m_nFXDestination: usize = 0x5B8; // ParticleIndex_t
    pub const m_vDestination: usize = 0x5BC; // Vector
    pub const m_iMinDistance: usize = 0x5C8; // int32
    pub const m_flBaseTeleportTime: usize = 0x5CC; // float32
    pub const m_flExtraTeleportTime: usize = 0x5D0; // float32
}

pub mod C_DOTA_Ability_Tinker_Laser {
    pub const m_vProjectileLocation: usize = 0x5B0; // Vector
    pub const bBlocked: usize = 0x5BC; // bool
    pub const m_hHitEntities: usize = 0x5C0; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_Tinker_MarchOfTheMachines {
    pub const splash_radius: usize = 0x5B0; // int32
    pub const damage: usize = 0x5B4; // int32
}

pub mod C_DOTA_Ability_Tinker_Rearm {
    pub const m_vProjectileLocation: usize = 0x5B0; // Vector
}

pub mod C_DOTA_Ability_Tiny_Avalanche {
    pub const m_vTargetLoc: usize = 0x5B0; // Vector
}

pub mod C_DOTA_Ability_Tiny_Tree_Grab {
    pub const m_hStolenTree: usize = 0x5B0; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_TrollWarlord_BattleTrance {
    pub const trance_duration: usize = 0x5B0; // float32
}

pub mod C_DOTA_Ability_TrollWarlord_BerserkersRage {
    pub const m_iOriginalAttackCapabilities: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_TrollWarlord_WhirlingAxes_Ranged {
    pub const m_vStartPos: usize = 0x5B0; // Vector
    pub const m_iArrowProjectile: usize = 0x5BC; // int32
    pub const axe_width: usize = 0x5C0; // int32
    pub const axe_speed: usize = 0x5C4; // float32
    pub const axe_range: usize = 0x5C8; // float32
    pub const axe_spread: usize = 0x5CC; // int32
    pub const axe_count: usize = 0x5D0; // int32
    pub const m_hHitUnits: usize = 0x5D8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_TrollWarlord_Whirling_Axes_Melee {
    pub const m_hAxes: usize = 0x5B0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_nAxeIdx: usize = 0x5C8; // int32
}

pub mod C_DOTA_Ability_Tusk_IceShards {
    pub const m_iProjectile: usize = 0x5B0; // int32
    pub const shard_width: usize = 0x5B4; // int32
    pub const shard_damage: usize = 0x5B8; // int32
    pub const shard_count: usize = 0x5BC; // int32
    pub const shard_speed: usize = 0x5C0; // float32
    pub const shard_duration: usize = 0x5C4; // float32
    pub const shard_angle_step: usize = 0x5C8; // float32
    pub const shard_distance: usize = 0x5CC; // int32
    pub const m_vSpawnOrigin: usize = 0x5D0; // Vector
    pub const m_vDirection: usize = 0x5DC; // Vector
    pub const m_vecShards: usize = 0x5E8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_Tusk_Snowball {
    pub const snowball_windup_radius: usize = 0x5B0; // int32
    pub const snowball_radius: usize = 0x5B4; // int32
    pub const snowball_grow_rate: usize = 0x5B8; // int32
    pub const snowball_damage: usize = 0x5BC; // int32
    pub const snowball_damage_bonus: usize = 0x5C0; // int32
    pub const stun_duration: usize = 0x5C4; // float32
    pub const bonus_damage: usize = 0x5C8; // int32
    pub const bonus_stun: usize = 0x5CC; // float32
    pub const snowball_speed: usize = 0x5D0; // int32
    pub const snowball_speed_bonus: usize = 0x5D4; // int32
    pub const snowball_duration: usize = 0x5D8; // float32
    pub const m_vProjectileLocation: usize = 0x5DC; // Vector
    pub const m_hSnowballedUnits: usize = 0x5E8; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_nFXIndex: usize = 0x600; // ParticleIndex_t
    pub const ctSnowball: usize = 0x608; // CountdownTimer
    pub const m_bSpeakAlly: usize = 0x620; // bool
    pub const m_bIsExpired: usize = 0x621; // bool
    pub const m_bInWindup: usize = 0x622; // bool
    pub const m_hPrimaryTarget: usize = 0x624; // CHandle< C_BaseEntity >
    pub const m_nContainedValidUnits: usize = 0x628; // int32
    pub const m_bEndingSnowball: usize = 0x62C; // bool
}

pub mod C_DOTA_Ability_Twin_Gate_Portal_Warp {
    pub const m_hTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_Undying_Decay {
    pub const decay_damage: usize = 0x5B0; // int32
    pub const radius: usize = 0x5B4; // int32
    pub const decay_duration: usize = 0x5B8; // float32
    pub const str_steal: usize = 0x5BC; // int32
    pub const creep_damage_multiplier: usize = 0x5C0; // float32
}

pub mod C_DOTA_Ability_Undying_SoulRip {
    pub const damage_per_unit: usize = 0x5B0; // int32
    pub const radius: usize = 0x5B4; // int32
    pub const max_units: usize = 0x5B8; // int32
    pub const tombstone_heal: usize = 0x5BC; // int32
}

pub mod C_DOTA_Ability_Undying_Tombstone {
    pub const m_vZombies: usize = 0x5B0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const hTombstone: usize = 0x5C8; // CHandle< C_BaseEntity >
    pub const radius: usize = 0x5CC; // int32
    pub const duration: usize = 0x5D0; // float32
}

pub mod C_DOTA_Ability_VengefulSpirit_Command_Aura {
    pub const m_hScepterIllusion: usize = 0x5B0; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_VengefulSpirit_Magic_Missile {
    pub const m_bShouldBounce: usize = 0x5B0; // bool
}

pub mod C_DOTA_Ability_VengefulSpirit_Nether_Swap {
    pub const m_nFXIndex: usize = 0x5B0; // int32
    pub const damage: usize = 0x5B4; // int32
    pub const damage_reduction: usize = 0x5B8; // float32
    pub const damage_reduction_duration: usize = 0x5BC; // float32
}

pub mod C_DOTA_Ability_VengefulSpirit_WaveOfTerror {
    pub const wave_width: usize = 0x5B0; // int32
    pub const wave_speed: usize = 0x5B4; // float32
    pub const m_iProjectile: usize = 0x5B8; // int32
    pub const vision_aoe: usize = 0x5BC; // float32
    pub const vision_duration: usize = 0x5C0; // float32
    pub const m_nNumHeroesHit: usize = 0x5C4; // int32
    pub const m_ViewerTimer: usize = 0x5C8; // CountdownTimer
}

pub mod C_DOTA_Ability_Venomancer_Latent_Poison {
    pub const projectile_speed: usize = 0x5B0; // Vector
}

pub mod C_DOTA_Ability_Venomancer_PoisonSting {
    pub const radius: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_Venomancer_VenomousGale {
    pub const duration: usize = 0x5B0; // float32
}

pub mod C_DOTA_Ability_Viper_ViperStrike {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
}

pub mod C_DOTA_Ability_Visage_Silent_As_The_Grave {
    pub const invis_duration: usize = 0x5B0; // float32
}

pub mod C_DOTA_Ability_Visage_SoulAssumption {
    pub const m_fDamage: usize = 0x5B0; // float32
    pub const m_nFXIndex: usize = 0x5B4; // ParticleIndex_t
    pub const m_iForcedStacks: usize = 0x5B8; // int32
}

pub mod C_DOTA_Ability_Visage_SummonFamiliars {
    pub const szUnitName: usize = 0x5B0; // char[260]
    pub const m_hExistingUnits: usize = 0x6B8; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Ability_Warlock_RainOfChaos {
    pub const aoe: usize = 0x5B0; // int32
    pub const m_bHasScepter: usize = 0x5B4; // bool
}

pub mod C_DOTA_Ability_Warlock_Upheaval {
    pub const m_vPosition: usize = 0x5B0; // Vector
    pub const aoe: usize = 0x5BC; // float32
    pub const slow_per_second: usize = 0x5C0; // int32
    pub const aspd_per_second: usize = 0x5C4; // int32
    pub const max_slow: usize = 0x5C8; // int32
    pub const damage_per_second: usize = 0x5CC; // int32
    pub const max_damage: usize = 0x5D0; // int32
    pub const damage_tick_interval: usize = 0x5D4; // float32
    pub const m_timer: usize = 0x5D8; // CountdownTimer
    pub const m_shardTimer: usize = 0x5F0; // CountdownTimer
    pub const duration: usize = 0x608; // float32
    pub const m_flElapsedTime: usize = 0x60C; // GameTime_t
    pub const m_flCurrentSlow: usize = 0x610; // float32
    pub const m_nFXIndex: usize = 0x614; // ParticleIndex_t
    pub const m_bTargetCast: usize = 0x620; // bool
    pub const m_hTarget: usize = 0x624; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_WarpineRaider_SeedShot {
    pub const m_iBounces: usize = 0x5B0; // int32
    pub const bounces: usize = 0x5B4; // int32
}

pub mod C_DOTA_Ability_Weaver_Shukuchi {
    pub const duration: usize = 0x5B0; // float32
    pub const radius: usize = 0x5B4; // int32
    pub const damage: usize = 0x5B8; // int32
    pub const fade_time: usize = 0x5BC; // float32
}

pub mod C_DOTA_Ability_Weaver_TimeLapse {
    pub const m_nNPCSpawnedID: usize = 0x5B0; // int32
}

pub mod C_DOTA_Ability_Windrunner_GaleForce {
    pub const m_vEndpoint: usize = 0x5B0; // Vector
    pub const m_vFlowPosition: usize = 0x5BC; // Vector
    pub const duration: usize = 0x5C8; // float32
}

pub mod C_DOTA_Ability_Windrunner_Powershot {
    pub const m_fStartTime: usize = 0x5B8; // GameTime_t
    pub const m_fPower: usize = 0x5BC; // float32
    pub const m_iProjectile: usize = 0x5C0; // int32
    pub const damage_reduction: usize = 0x5C4; // float32
    pub const arrow_width: usize = 0x5C8; // int32
    pub const powershot_damage: usize = 0x5CC; // int32
    pub const tree_width: usize = 0x5D0; // float32
    pub const m_bAwardedKillEater: usize = 0x5D4; // bool
    pub const m_nHeroesHit: usize = 0x5D8; // int32
    pub const m_nFXIndex: usize = 0x5DC; // ParticleIndex_t
}

pub mod C_DOTA_Ability_Windrunner_Shackleshot {
    pub const shackle_count: usize = 0x5B0; // int32
    pub const m_vArrowAvgPos: usize = 0x5B4; // Vector
    pub const m_vArrowStartPos: usize = 0x5C0; // Vector
    pub const m_vArrowStartPos2: usize = 0x5CC; // Vector
    pub const m_vArrowStartPos3: usize = 0x5D8; // Vector
    pub const m_hTarget: usize = 0x5E4; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Ability_Wisp_Relocate {
    pub const m_nFXIndexEndTeam: usize = 0x5B0; // ParticleIndex_t
    pub const m_nFXIndexChannel: usize = 0x5B4; // ParticleIndex_t
    pub const cast_delay: usize = 0x5B8; // float32
    pub const return_time: usize = 0x5BC; // float32
}

pub mod C_DOTA_Ability_Wisp_Spirits {
    pub const m_SpiritDefs: usize = 0x5B0; // CUtlVector< sSpiritDef >
    pub const m_nWispDirection: usize = 0x5C8; // int32
    pub const spirit_amount: usize = 0x5CC; // int32
}

pub mod C_DOTA_Ability_Wisp_Tether {
    pub const m_hTarget: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const m_vProjectileLocation: usize = 0x5B4; // Vector
    pub const m_bProjectileActive: usize = 0x5C0; // bool
    pub const latch_distance: usize = 0x5C4; // int32
    pub const m_iProjectileIndex: usize = 0x5C8; // int32
}

pub mod C_DOTA_Ability_WitchDoctor_DeathWard {
    pub const m_hWard: usize = 0x5B0; // CHandle< C_BaseEntity >
    pub const m_iDamage: usize = 0x5B4; // int32
    pub const m_iBounceRadius: usize = 0x5B8; // int32
    pub const m_iProjectileSpeed: usize = 0x5BC; // int32
    pub const m_fWardExpireTime: usize = 0x5C0; // GameTime_t
    pub const m_nFXIndex: usize = 0x5C4; // ParticleIndex_t
    pub const m_iAttackIndex: usize = 0x5C8; // int32
    pub const m_BounceInfo: usize = 0x5D0; // CUtlVector< sBounceInfo >
}

pub mod C_DOTA_Ability_WitchDoctor_ParalyzingCask {
    pub const m_iBounces: usize = 0x5B0; // int32
    pub const bounces: usize = 0x5B4; // int32
    pub const bounce_bonus_damage: usize = 0x5B8; // int32
}

pub mod C_DOTA_Ability_Zuus_ThundergodsWrath {
    pub const m_nFXIndex: usize = 0x5B0; // ParticleIndex_t
    pub const m_bZeusHasArcana: usize = 0x5B4; // bool
}

pub mod C_DOTA_AghsFort_Ability_ArcWardenBoss_SparkWraith {
    pub const duration: usize = 0x5B0; // float32
    pub const activation_delay: usize = 0x5B4; // float32
    pub const wraith_vision_duration: usize = 0x5B8; // float32
    pub const wraith_vision_radius: usize = 0x5BC; // int32
    pub const spark_damage: usize = 0x5C0; // int32
}

pub mod C_DOTA_AghsFort_Ability_ArcWardenBoss_TempestDouble {
    pub const m_nNumDoubles: usize = 0x5B0; // int32
}

pub mod C_DOTA_AghsFort_Ability_Undying_Decay {
    pub const decay_damage: usize = 0x5B0; // int32
    pub const radius: usize = 0x5B4; // int32
    pub const decay_duration: usize = 0x5B8; // float32
    pub const str_steal: usize = 0x5BC; // float32
    pub const str_steal_captain_mult: usize = 0x5C0; // float32
    pub const str_steal_boss_mult: usize = 0x5C4; // float32
}

pub mod C_DOTA_AghsFort_Ability_Undying_SoulRip {
    pub const damage_per_unit: usize = 0x5B0; // int32
    pub const radius: usize = 0x5B4; // int32
    pub const max_units: usize = 0x5B8; // int32
    pub const tombstone_heal: usize = 0x5BC; // int32
    pub const damage_bonus_percent: usize = 0x5C0; // int32
}

pub mod C_DOTA_AghsFort_Ability_Undying_Tombstone {
    pub const m_vZombies: usize = 0x5B0; // CUtlVector< CHandle< C_BaseEntity > >
    pub const hTombstone: usize = 0x5C8; // CHandle< C_BaseEntity >
    pub const duration: usize = 0x5CC; // float32
}

pub mod C_DOTA_AghsFort_Unit_Undying_Zombie {
    pub const m_ctRespawn: usize = 0x1758; // CountdownTimer
    pub const m_pTombstone: usize = 0x1770; // C_DOTA_BaseNPC*
}

pub mod C_DOTA_Aghsfort_AbilityCrystalMaiden_FreezingField {
    pub const m_nWarningFX: usize = 0x5B0; // ParticleIndex_t
    pub const m_nRadiusFX: usize = 0x5B4; // ParticleIndex_t
}

pub mod C_DOTA_Aghsfort_Ability_Creature_Magnus_Push_Skewer {
    pub const skewer_radius: usize = 0x5B0; // int32
    pub const skewer_speed: usize = 0x5B4; // int32
    pub const range: usize = 0x5B8; // int32
    pub const tree_radius: usize = 0x5BC; // int32
}

pub mod C_DOTA_Aghsfort_Ability_CrystalMaiden_CrystalNova {
    pub const nova_damage: usize = 0x5B0; // int32
}

pub mod C_DOTA_Aghsfort_Ability_Hoodwink_HuntersBoomerang {
    pub const min_distance: usize = 0x5B0; // int32
}

pub mod C_DOTA_ArcanaDataEntity {
    pub const m_pArcanaData_CDOTA_ArcanaDataEntity_DrowRanger: usize = 0x538; // CDOTA_ArcanaDataEntity_DrowRanger*
    pub const m_pArcanaData_CDOTA_ArcanaDataEntity_FacelessVoid: usize = 0x540; // CDOTA_ArcanaDataEntity_FacelessVoid*
    pub const m_pArcanaData_CDOTA_ArcanaDataEntity_Razor: usize = 0x548; // CDOTA_ArcanaDataEntity_Razor*
}

pub mod C_DOTA_BaseNPC {
    pub const m_bIsPhantom: usize = 0xA30; // bool
    pub const m_iUnitType: usize = 0xA34; // uint32
    pub const m_bSelectionRingVisible: usize = 0xA48; // bool
    pub const m_iCurrentLevel: usize = 0xA4C; // int32
    pub const m_bIsAncient: usize = 0xA50; // bool
    pub const m_bIsBossCreature: usize = 0xA51; // bool
    pub const m_bStolenScepter: usize = 0xA52; // bool
    pub const m_bIsNeutralUnitType: usize = 0xA53; // bool
    pub const m_bSelectOnSpawn: usize = 0xA54; // bool
    pub const m_bCachedReplicatedMorphlingIllusion: usize = 0xA55; // bool
    pub const m_bIgnoreAddSummonedToSelection: usize = 0xA56; // bool
    pub const m_bConsideredHero: usize = 0xA57; // bool
    pub const m_bUsesConstantGesture: usize = 0xA58; // bool
    pub const m_bUseHeroAbilityNumbers: usize = 0xA59; // bool
    pub const m_bHasSharedAbilities: usize = 0xA5A; // bool
    pub const m_bIsSummoned: usize = 0xA5B; // bool
    pub const m_bCanBeDominated: usize = 0xA5C; // bool
    pub const m_bHasUpgradeableAbilities: usize = 0xA5D; // bool
    pub const m_flHealthThinkRegen: usize = 0xA60; // float32
    pub const m_iIsControllableByPlayer64: usize = 0xA68; // uint64
    pub const m_nHealthBarOffsetOverride: usize = 0xA70; // int32
    pub const m_bCanRespawn: usize = 0xA74; // bool
    pub const m_iAttackRange: usize = 0xA78; // int32
    pub const m_colorGemColor: usize = 0xA7C; // Color
    pub const m_bHasColorGem: usize = 0xA80; // bool
    pub const m_nFXDeniableIndex: usize = 0xA84; // ParticleIndex_t
    pub const m_iMoveSpeed: usize = 0xA88; // int32
    pub const m_iBaseAttackSpeed: usize = 0xA8C; // int32
    pub const m_flBaseAttackTime: usize = 0xA90; // float32
    pub const m_iUnitNameIndex: usize = 0xA94; // int32
    pub const m_iHealthBarOffset: usize = 0xA98; // int32
    pub const m_iHealthBarHighlightColor: usize = 0xA9C; // Color
    pub const m_flMana: usize = 0xAA0; // float32
    pub const m_flMaxMana: usize = 0xAA4; // float32
    pub const m_flManaThinkRegen: usize = 0xAA8; // float32
    pub const m_iBKBChargesUsed: usize = 0xAAC; // int32
    pub const m_iAeonChargesUsed: usize = 0xAB0; // int32
    pub const m_flRefresherUseTime: usize = 0xAB4; // GameTime_t
    pub const m_flRefresherLastCooldown: usize = 0xAB8; // float32
    pub const m_flLastDealtDamageTime: usize = 0xABC; // GameTime_t
    pub const m_iBotDebugData: usize = 0xAC0; // int32
    pub const m_bIsIllusion: usize = 0xAC4; // bool
    pub const m_bHasClientSeenIllusionModifier: usize = 0xAC5; // bool
    pub const m_hAbilities: usize = 0xAC8; // CHandle< C_BaseEntity >[35]
    pub const m_flInvisibilityLevel: usize = 0xB54; // float32
    pub const m_flHullRadius: usize = 0xB58; // float32
    pub const m_flCollisionPadding: usize = 0xB5C; // float32
    pub const m_flRingRadius: usize = 0xB60; // float32
    pub const m_flProjectileCollisionSize: usize = 0xB64; // float32
    pub const m_iszUnitName: usize = 0xB68; // CUtlSymbolLarge
    pub const m_iszParticleFolder: usize = 0xB70; // CUtlSymbolLarge
    pub const m_iszSoundSet: usize = 0xB78; // CUtlSymbolLarge
    pub const m_iszSelectionGroup: usize = 0xB80; // CUtlSymbolLarge
    pub const m_iszVoiceFile: usize = 0xB88; // CUtlSymbolLarge
    pub const m_iszGameSoundsFile: usize = 0xB90; // CUtlSymbolLarge
    pub const m_iszVoiceBackgroundSound: usize = 0xB98; // CUtlSymbolLarge
    pub const m_iszIdleSoundLoop: usize = 0xBA0; // CUtlSymbolLarge
    pub const m_szUnitLabel: usize = 0xBA8; // CUtlString
    pub const m_szScaleset: usize = 0xBB0; // CUtlString
    pub const m_nUnitLabelIndex: usize = 0xBB8; // uint8
    pub const m_strAnimationModifier: usize = 0xBC0; // CUtlString
    pub const m_TerrainSpecificFootstepEffect: usize = 0xBC8; // CUtlString
    pub const m_bUseCustomTerrainWeatherEffect: usize = 0xBD0; // bool
    pub const m_bHasClientSoundReplacement: usize = 0xBD1; // bool
    pub const m_bHasClientReplacementParticle: usize = 0xBD2; // bool
    pub const m_bResourcesLoaded: usize = 0xBDC; // bool
    pub const m_flTauntCooldown: usize = 0xBE0; // GameTime_t
    pub const m_flTauntCooldown2: usize = 0xBE4; // GameTime_t
    pub const m_iCurShop: usize = 0xBE8; // DOTA_SHOP_TYPE
    pub const m_szCurShopEntName: usize = 0xBF0; // CUtlSymbolLarge
    pub const m_iDayTimeVisionRange: usize = 0xBF8; // int32
    pub const m_iNightTimeVisionRange: usize = 0xBFC; // int32
    pub const m_iDamageMin: usize = 0xC00; // int32
    pub const m_iDamageMax: usize = 0xC04; // int32
    pub const m_iDamageBonus: usize = 0xC08; // int32
    pub const m_iTaggedAsVisibleByTeam: usize = 0xC0C; // int32
    pub const m_ModifierManager: usize = 0xC10; // CDOTA_ModifierManager
    pub const m_Inventory: usize = 0xEA0; // C_DOTA_UnitInventory
    pub const m_nUnitState64: usize = 0xF58; // uint64
    pub const m_nUnitDebuffState: usize = 0xF68; // uint64
    pub const m_bHasInventory: usize = 0xF70; // bool
    pub const m_iAcquisitionRange: usize = 0xF74; // int32
    pub const m_FoWViewID: usize = 0xF78; // int32
    pub const m_iPrevHealthPct: usize = 0xF7C; // int32
    pub const m_iPrevLifeState: usize = 0xF80; // int32
    pub const m_iPrevTeam: usize = 0xF84; // int32
    pub const m_bPrevProvidesVision: usize = 0xF88; // bool
    pub const m_nPrevControllableMask: usize = 0xF90; // uint64
    pub const m_TagTime: usize = 0xFA0; // CountdownTimer
    pub const m_ClickedTime: usize = 0xFB8; // CountdownTimer
    pub const m_flIdleRunPoseStartTime: usize = 0xFD0; // GameTime_t[2]
    pub const m_flIdleRunPoseDuration: usize = 0xFD8; // float32[2]
    pub const m_bAnimationTransitionActive: usize = 0xFE0; // bool
    pub const m_nAnimationTransitionPoseParameters: usize = 0xFE4; // int32[2]
    pub const m_nAnimationTransitionInjuredPoseParameters: usize = 0xFEC; // int32[2]
    pub const m_nAnimationTransitionAggressivePoseParameters: usize = 0xFF4; // int32[2]
    pub const m_flTimeSinceLastAbilityNag: usize = 0x1058; // GameTime_t
    pub const m_iAttackCapabilities: usize = 0x105C; // int32
    pub const m_iSpecialAbility: usize = 0x1060; // int32
    pub const m_iMoveCapabilities: usize = 0x1064; // int32
    pub const m_nPlayerOwnerID: usize = 0x1068; // PlayerID_t
    pub const m_flLastDamageTime: usize = 0x106C; // GameTime_t
    pub const m_flLastAttackTime: usize = 0x1070; // GameTime_t
    pub const m_iszMinimapIcon: usize = 0x1078; // CUtlSymbolLarge
    pub const m_flMinimapIconSize: usize = 0x1088; // float32
    pub const m_bMinimapDisableTint: usize = 0x108C; // bool
    pub const m_bMinimapDisableRotation: usize = 0x108D; // bool
    pub const m_colorHeroGlow: usize = 0x108E; // Color
    pub const m_iNearShopMask: usize = 0x1094; // int32
    pub const m_nPoseParameterTurn: usize = 0x1098; // int32
    pub const m_nPoseParameterRun: usize = 0x109C; // int32
    pub const m_flLean: usize = 0x10A0; // float32
    pub const m_flRun: usize = 0x10A4; // float32
    pub const m_anglediff: usize = 0x10A8; // int32
    pub const m_bInfoKeyActive: usize = 0x10AC; // bool
    pub const m_bNewUpdateAssetModifiersNetworked: usize = 0x10AD; // bool
    pub const m_nAssetModifiersLastUpdatedTick: usize = 0x10B0; // GameTick_t
    pub const m_bSuppressGlow: usize = 0x10B4; // bool
    pub const m_bWasSinking: usize = 0x10B5; // bool
    pub const m_flRangeDisplayDist: usize = 0x10B8; // float32
    pub const m_szDefaultIdle: usize = 0x12A0; // CUtlSymbolLarge
    pub const m_szDefaultRareIdle: usize = 0x12A8; // CUtlSymbolLarge
    pub const m_damagetimer: usize = 0x12B0; // CountdownTimer
    pub const m_vRenderOrigin: usize = 0x12C8; // Vector
    pub const m_fZDelta: usize = 0x12D4; // float32
    pub const m_flDeathTime: usize = 0x12D8; // GameTime_t
    pub const m_bBaseStatsChanged: usize = 0x12DC; // bool
    pub const m_bNeedsSoundEmitterRefresh: usize = 0x12DD; // bool
    pub const m_flPhysicalArmorValue: usize = 0x12E0; // float32
    pub const m_flMagicalResistanceValue: usize = 0x12E4; // float32
    pub const m_nPrevSequenceParity: usize = 0x12E8; // int32
    pub const m_flPrevInvisLevel: usize = 0x1390; // float32
    pub const m_nOriginalModelIndex: usize = 0x1398; // CStrongHandle< InfoForResourceTypeCModel >
    pub const m_nClientOriginalModelIndex: usize = 0x13A0; // CStrongHandle< InfoForResourceTypeCModel >
    pub const m_nClientReplacementModelIndex: usize = 0x13A8; // CStrongHandle< InfoForResourceTypeCModel >
    pub const m_bClientReplacementModelNeedsRebuild: usize = 0x13B0; // bool
    pub const m_nClientReplacementModelReloadCount: usize = 0x13B4; // int32
    pub const m_nUnitModelVariant: usize = 0x13B8; // int8
    pub const m_nUnitModelVariantCount: usize = 0x13B9; // int8
    pub const m_iPrevSequence: usize = 0x13BC; // int32
    pub const m_pLastWeatherEffectName: usize = 0x13C0; // char*
    pub const m_VoiceBackgroundSoundTimer: usize = 0x13D0; // CountdownTimer
    pub const m_bIsWaitingToSpawn: usize = 0x13E8; // bool
    pub const m_nTotalDamageTaken: usize = 0x13F0; // int64
    pub const m_flManaRegen: usize = 0x13F8; // float32
    pub const m_flHealthRegen: usize = 0x13FC; // float32
    pub const m_bIsMoving: usize = 0x1400; // bool
    pub const m_fRevealRadius: usize = 0x1404; // float32
    pub const m_bCanUseWards: usize = 0x1408; // bool
    pub const m_bCanUseAllItems: usize = 0x1409; // bool
    pub const m_iXPBounty: usize = 0x140C; // int32
    pub const m_iXPBountyExtra: usize = 0x1410; // int32
    pub const m_iGoldBountyMin: usize = 0x1414; // int32
    pub const m_iGoldBountyMax: usize = 0x1418; // int32
    pub const m_hOwnerNPC: usize = 0x141C; // CHandle< C_BaseEntity >
    pub const m_hGoalEntity: usize = 0x1420; // CHandle< C_BaseEntity >
    pub const m_hNeutralSpawner: usize = 0x1424; // CHandle< C_DOTA_NeutralSpawner >
    pub const m_hClientOverrideMaterial: usize = 0x1428; // CStrongHandle< InfoForResourceTypeIMaterial2 >
    pub const m_bCombinerMaterialOverrideListChanged: usize = 0x1430; // bool
    pub const m_nBaseModelMeshCount: usize = 0x1434; // int32
    pub const m_combinerMaterialOverrideList: usize = 0x1438; // CUtlVector< CStrongHandleCopyable< InfoForResourceTypeIMaterial2 > >
    pub const m_nArcanaLevel: usize = 0x1450; // int8
    pub const m_nDefaultArcanaLevel: usize = 0x1451; // int8
    pub const m_defaultColorGemColor: usize = 0x1452; // Color
    pub const m_bHasBuiltWearableSpawnList: usize = 0x1470; // bool
    pub const m_bHasCostume: usize = 0x1471; // bool
    pub const m_bRecreateParticleSystemsOnModifiersChange: usize = 0x1472; // bool
    pub const m_NetworkActivity: usize = 0x1590; // int32
    pub const m_PrevNetworkActivity: usize = 0x1594; // int32
    pub const m_NetworkSequenceIndex: usize = 0x1598; // int32
    pub const m_bShouldDoFlyHeightVisual: usize = 0x159C; // bool
    pub const m_flStartSequenceCycle: usize = 0x15A0; // float32
    pub const m_ActivityModifiers: usize = 0x15A8; // CUtlVector< CUtlSymbol >
    pub const m_hBackgroundSceneEnt: usize = 0x15C0; // CHandle< C_SceneEntity >
    pub const m_hSpeakingSceneEnt: usize = 0x15C4; // CHandle< C_SceneEntity >
    pub const m_hOldWearables: usize = 0x15D8; // CUtlVector< CHandle< C_EconWearable > >
    pub const m_hOldWearableSkins: usize = 0x15F0; // CUtlVector< CUtlStringToken >
    pub const m_CustomHealthLabel: usize = 0x1608; // char[256]
    pub const m_CustomHealthLabelColor: usize = 0x1708; // Color
    pub const m_nWearableDefIndex: usize = 0x170C; // item_definition_index_t
    pub const m_gibTintColor: usize = 0x1711; // Color
    pub const m_bForceMaterialCombine: usize = 0x1715; // bool
    pub const m_bShouldDrawParticlesWhileHidden: usize = 0x1716; // bool
    pub const m_bIsClientThinkPending: usize = 0x1717; // bool
    pub const m_bActivityModifiersDirty: usize = 0x1718; // bool
    pub const m_bUnitModelVariantChanged: usize = 0x1719; // bool
    pub const m_bWearablesAreTaunting: usize = 0x171A; // bool
    pub const m_bShowCannotBeDisabledIcon: usize = 0x1730; // bool
}

pub mod C_DOTA_BaseNPC_Building {
    pub const m_iDamageLevel: usize = 0x1750; // int32
    pub const m_nAmbientFXIndex: usize = 0x1754; // ParticleIndex_t
    pub const m_nTPFXIndex: usize = 0x1758; // ParticleIndex_t
    pub const m_nStatusFXIndex: usize = 0x175C; // ParticleIndex_t
    pub const m_nFXIndex: usize = 0x1760; // CStrongHandle< InfoForResourceTypeIParticleSystemDefinition >
    pub const m_nFXIndexDestruction: usize = 0x1768; // CStrongHandle< InfoForResourceTypeIParticleSystemDefinition >
    pub const m_angInitialAngles: usize = 0x1770; // QAngle
    pub const m_hHeroStatueSequence: usize = 0x177C; // HSequence
    pub const m_hConstantLayerSequence: usize = 0x1780; // HSequence
    pub const m_fHeroStatueCycle: usize = 0x1784; // float32
    pub const m_iHeroStatueStatusEffectIndex: usize = 0x1788; // int32
    pub const m_bHeroStatue: usize = 0x178C; // bool
    pub const m_bBattleCup: usize = 0x178D; // bool
    pub const m_HeroStatueInscription: usize = 0x178E; // char[32]
    pub const m_iHeroStatueOwnerPlayerID: usize = 0x17B0; // PlayerID_t
    pub const m_ParticleTintColor: usize = 0x17B4; // Color
}

pub mod C_DOTA_BaseNPC_Creature {
    pub const m_bIsCurrentlyChanneling: usize = 0x1758; // bool
    pub const m_flChannelCycle: usize = 0x175C; // float32
}

pub mod C_DOTA_BaseNPC_Creep {
    pub const m_flAim: usize = 0x1750; // float32
}

pub mod C_DOTA_BaseNPC_Creep_Neutral {
    pub const m_sDisplayName: usize = 0x1758; // CUtlString
}

pub mod C_DOTA_BaseNPC_HallofFame {
    pub const m_HallofFame: usize = 0x17B8; // int32
}

pub mod C_DOTA_BaseNPC_Healer {
    pub const m_iRangeFX: usize = 0x17B8; // ParticleIndex_t
}

pub mod C_DOTA_BaseNPC_Hero {
    pub const m_iCurrentXP: usize = 0x1758; // int32
    pub const m_iAbilityPoints: usize = 0x175C; // int32
    pub const m_flRespawnTime: usize = 0x1760; // GameTime_t
    pub const m_flRespawnTimePenalty: usize = 0x1764; // float32
    pub const m_flStrength: usize = 0x1768; // float32
    pub const m_flAgility: usize = 0x176C; // float32
    pub const m_flIntellect: usize = 0x1770; // float32
    pub const m_flStrengthTotal: usize = 0x1774; // float32
    pub const m_flAgilityTotal: usize = 0x1778; // float32
    pub const m_flIntellectTotal: usize = 0x177C; // float32
    pub const m_flDeathTime: usize = 0x1780; // GameTime_t
    pub const m_iRecentDamage: usize = 0x1784; // int32
    pub const m_fPainFactor: usize = 0x1788; // float32
    pub const m_fTargetPainFactor: usize = 0x178C; // float32
    pub const m_bLifeState: usize = 0x1790; // bool
    pub const m_nFXStunIndex: usize = 0x1794; // ParticleIndex_t
    pub const m_nFXSilenceIndex: usize = 0x1798; // ParticleIndex_t
    pub const m_nFXDeathIndex: usize = 0x179C; // ParticleIndex_t
    pub const m_iPlayerID: usize = 0x17C0; // PlayerID_t
    pub const m_hReplicatingOtherHeroModel: usize = 0x17C4; // CHandle< C_DOTA_BaseNPC_Hero >
    pub const m_bReincarnating: usize = 0x17C8; // bool
    pub const m_bCustomKillEffect: usize = 0x17C9; // bool
    pub const m_flSpawnedAt: usize = 0x17CC; // GameTime_t
    pub const m_bScriptDisableRespawns: usize = 0x17D0; // bool
    pub const m_iPrimaryAttribute: usize = 0x17D4; // int32
    pub const m_nLastDrawnHealth: usize = 0x17D8; // int32
    pub const m_flHurtAmount: usize = 0x17DC; // float32
    pub const m_flLastHurtTime: usize = 0x17E0; // GameTime_t
    pub const m_flHurtDecayRate: usize = 0x17E4; // float32
    pub const m_flLastHealTime: usize = 0x17E8; // GameTime_t
    pub const m_flLastTreeShakeTime: usize = 0x17EC; // GameTime_t
    pub const m_CenterOnHeroCooldownTimer: usize = 0x17F0; // CountdownTimer
    pub const m_CombinedModels: usize = 0x1A10; // CStrongHandle< InfoForResourceTypeCModel >[4]
    pub const m_nCurrentCombinedModelIndex: usize = 0x1A30; // int32
    pub const m_nPendingCombinedModelIndex: usize = 0x1A34; // int32
    pub const m_iHeroID: usize = 0x1A38; // int32
    pub const m_flCheckLegacyItemsAt: usize = 0x1A3C; // float32
    pub const m_bDisplayAdditionalHeroes: usize = 0x1A40; // bool
    pub const m_CombinedParticleModels: usize = 0x1A50; // CStrongHandle< InfoForResourceTypeCModel >[4]
    pub const m_vecAttachedParticleIndeces: usize = 0x1A70; // CUtlVector< ParticleIndex_t >
    pub const m_hPets: usize = 0x1A88; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_nKillStreakFX: usize = 0x1AA0; // ParticleIndex_t
    pub const m_nKillStreakFXTier: usize = 0x1AA4; // int32
    pub const m_bBuybackDisabled: usize = 0x0; // bitfield:1
    pub const m_bWasFrozen: usize = 0x0; // bitfield:1
    pub const m_bUpdateClientsideWearables: usize = 0x0; // bitfield:1
    pub const m_bForceBuildCombinedModel: usize = 0x0; // bitfield:1
    pub const m_bRecombineForMaterialsOnly: usize = 0x0; // bitfield:1
    pub const m_bBuildingCombinedModel: usize = 0x0; // bitfield:1
    pub const m_bInReloadEvent: usize = 0x0; // bitfield:1
    pub const m_bStoreOldVisibility: usize = 0x0; // bitfield:1
    pub const m_bResetVisibility: usize = 0x0; // bitfield:1
    pub const m_bStoredVisibility: usize = 0x0; // bitfield:1
}

pub mod C_DOTA_BaseNPC_HoldoutTower {
    pub const m_iTowerType: usize = 0x17F8; // DOTA_HOLDOUT_TOWER_TYPE
}

pub mod C_DOTA_BaseNPC_NeutralItemStash {
    pub const m_vecNewItemFX: usize = 0x17C8; // CUtlVector< ParticleIndex_t >
    pub const m_bHasFoundProps: usize = 0x17E0; // bool
    pub const m_bDidHaveNewItems: usize = 0x17E1; // bool
    pub const m_vecProps: usize = 0x17E8; // CUtlVector< CHandle< C_DynamicPropClientside > >
}

pub mod C_DOTA_BaseNPC_RotatableBuilding {
    pub const m_hObsoleteHeroStatueSequence: usize = 0x1750; // HSequence
    pub const m_hObsoleteConstantLayerSequence: usize = 0x1754; // HSequence
}

pub mod C_DOTA_BaseNPC_ShadowShaman_SerpentWard {
    pub const m_angle: usize = 0x1750; // QAngle
    pub const m_iPoseParameterAim: usize = 0x175C; // int32
}

pub mod C_DOTA_BaseNPC_Shop {
    pub const m_ShopType: usize = 0x17B8; // DOTA_SHOP_TYPE
    pub const m_nShopFX: usize = 0x17E0; // ParticleIndex_t
    pub const m_vShopFXOrigin: usize = 0x17E4; // Vector
    pub const m_flLastSpeech: usize = 0x17F0; // float32
}

pub mod C_DOTA_BaseNPC_Tower {
    pub const m_iRangeFX: usize = 0x17B8; // ParticleIndex_t
    pub const m_hTowerAttackTarget: usize = 0x17BC; // CHandle< C_BaseEntity >
    pub const m_hTowerHighFiveTarget: usize = 0x17C0; // CHandle< C_BaseEntity >
    pub const m_iPoseParameterAim: usize = 0x17C4; // int32
    pub const m_angDefaultCustomTowerAngle: usize = 0x17C8; // QAngle
    pub const m_flLastAimYaw: usize = 0x17D4; // float32
    pub const m_bClientSideCustomTower: usize = 0x17D8; // bool
    pub const m_IdleRareAnimationTime: usize = 0x17E0; // CountdownTimer
}

pub mod C_DOTA_BaseNPC_Tusk_Sigil {
    pub const m_angInitialAngles: usize = 0x1750; // QAngle
}

pub mod C_DOTA_BaseNPC_Venomancer_PlagueWard {
    pub const m_angle: usize = 0x1750; // QAngle
    pub const m_iPoseParameterAim: usize = 0x175C; // int32
}

pub mod C_DOTA_BaseNPC_Watch_Tower {
    pub const m_iRangeFX: usize = 0x17B8; // ParticleIndex_t
    pub const m_szOutpostName: usize = 0x17C0; // CUtlSymbolLarge
    pub const m_szInteractAbilityName: usize = 0x17C8; // CUtlSymbolLarge
}

pub mod C_DOTA_BinaryObject {
    pub const m_bActive: usize = 0x7E0; // bool
    pub const m_nBinaryID: usize = 0x7E4; // int32
}

pub mod C_DOTA_CDOTA_Item_BagOfGold_Caster_Only {
    pub const m_hThinker: usize = 0x648; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_CombatLogQueryProgress {
    pub const m_nPlayerID: usize = 0x28; // PlayerID_t
    pub const m_nQueryID: usize = 0x2C; // int32
    pub const m_nQueryRank: usize = 0x30; // int32
    pub const m_nMultiQueryID: usize = 0x34; // int32
}

pub mod C_DOTA_DataNonSpectator {
    pub const m_vecDataTeam: usize = 0x538; // C_UtlVectorEmbeddedNetworkVar< DataTeamPlayer_t >
    pub const m_bWorldTreeState: usize = 0x588; // uint64[256]
    pub const m_vecWorldTreeModelReplacements: usize = 0xD88; // C_UtlVectorEmbeddedNetworkVar< TreeModelReplacement_t >
    pub const m_vDesiredWardPlacement: usize = 0xDD8; // Vector2D[2]
    pub const m_nEnemyStartingPosition: usize = 0xDE8; // int32[5]
    pub const m_nTotalEventPoints: usize = 0xDFC; // int32
    pub const m_nCaptainInspectedHeroID: usize = 0xE00; // int32
    pub const m_nFeaturedPlayerID: usize = 0xE04; // int32
    pub const m_flSuggestedWardWeights: usize = 0xE08; // float32[20]
    pub const m_nSuggestedWardIndexes: usize = 0xE58; // uint8[20]
    pub const m_iSuggestedLanes: usize = 0xE6C; // int32[5]
    pub const m_iSuggestedLaneWeights: usize = 0xE80; // float32[5]
    pub const m_bSuggestedLaneRoam: usize = 0xE94; // bool[5]
    pub const m_bSuggestedLaneJungle: usize = 0xE99; // bool[5]
    pub const m_vecNeutralItemsTierInfo: usize = 0xEA0; // C_UtlVectorEmbeddedNetworkVar< TierNeutralInfo_t >
    pub const m_vecNeutralItemsEarned: usize = 0xEF0; // C_NetworkUtlVectorBase< AbilityID_t >
    pub const m_vecNeutralStashItems: usize = 0xF08; // C_NetworkUtlVectorBase< CHandle< C_BaseEntity > >
    pub const m_vecNeutralItemsConsumed: usize = 0xF20; // C_NetworkUtlVectorBase< AbilityID_t >
    pub const m_PingConfirmationStates: usize = 0xF38; // C_UtlVectorEmbeddedNetworkVar< PingConfirmationState_t >
    pub const m_vecKnownClearCamps: usize = 0xF88; // C_NetworkUtlVectorBase< CHandle< C_BaseEntity > >
    pub const m_vPossibleWardPlacement: usize = 0xFA0; // Vector2D[100]
    pub const m_vPossibleWardRadii: usize = 0x12C0; // float32[100]
}

pub mod C_DOTA_DataSpectator {
    pub const m_hPowerupRune_1: usize = 0x1450; // CHandle< C_BaseEntity >
    pub const m_hPowerupRune_2: usize = 0x1454; // CHandle< C_BaseEntity >
    pub const m_hBountyRune_1: usize = 0x1458; // CHandle< C_BaseEntity >
    pub const m_hBountyRune_2: usize = 0x145C; // CHandle< C_BaseEntity >
    pub const m_hBountyRune_3: usize = 0x1460; // CHandle< C_BaseEntity >
    pub const m_hBountyRune_4: usize = 0x1464; // CHandle< C_BaseEntity >
    pub const m_hXPRune_1: usize = 0x1468; // CHandle< C_BaseEntity >
    pub const m_hXPRune_2: usize = 0x146C; // CHandle< C_BaseEntity >
    pub const m_iNetWorth: usize = 0x1470; // int32[24]
    pub const m_fRadiantWinProbability: usize = 0x14D0; // float32
    pub const m_iGoldSpentOnSupport: usize = 0x14D4; // int32[24]
    pub const m_iHeroDamage: usize = 0x1534; // int32[24]
    pub const m_nWardsPurchased: usize = 0x1594; // int32[24]
    pub const m_nWardsPlaced: usize = 0x15F4; // int32[24]
    pub const m_nWardsDestroyed: usize = 0x1654; // int32[24]
    pub const m_nRunesActivated: usize = 0x16B4; // int32[24]
    pub const m_nCampsStacked: usize = 0x1714; // int32[24]
    pub const m_ThreatLevelInfos: usize = 0x1778; // C_UtlVectorEmbeddedNetworkVar< DOTAThreatLevelInfo_t >
}

pub mod C_DOTA_DisplacementVisibility {
    pub const m_HiddenDisplacement: usize = 0x538; // int32
}

pub mod C_DOTA_GuildBannerDynamic {
    pub const m_bRespawnClientEntity: usize = 0xA58; // bool
    pub const m_bPlaySpawnAnimation: usize = 0xA59; // bool
    pub const m_unGuildTier: usize = 0xA5A; // uint8
    pub const m_unPrimaryColor: usize = 0xA5B; // uint8
    pub const m_unSecondaryColor: usize = 0xA5C; // uint8
    pub const m_unPattern: usize = 0xA5D; // uint8
    pub const m_unLogo: usize = 0xA60; // uint64
    pub const m_unGuildID: usize = 0xA68; // GuildID_t
    pub const m_unGuildFlags: usize = 0xA6C; // uint32
    pub const m_bUsePanelCache: usize = 0xA70; // bool
    pub const m_hClientEntity: usize = 0xA74; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Hero_Recorder {
    pub const m_bStartRecording: usize = 0x538; // bool
    pub const m_hHero: usize = 0x53C; // CHandle< C_DOTA_BaseNPC >
    pub const m_hPlayer: usize = 0x540; // CHandle< C_DOTAPlayerController >
    pub const m_bRecording: usize = 0x544; // bool
    pub const m_bLastStartRecording: usize = 0x545; // bool
    pub const m_flLastCycle: usize = 0x548; // float32
    pub const m_nCompletedCycles: usize = 0x54C; // int32
    pub const m_nFramesThisCycle: usize = 0x550; // int32
    pub const m_nRecordedFrames: usize = 0x554; // int32
    pub const m_flHeroAdvanceTime: usize = 0x558; // float32
    pub const m_flStartTime: usize = 0x55C; // float32
    pub const m_flCycles: usize = 0x560; // CUtlVector< float32 >
    pub const m_pBatchFiles: usize = 0x578; // CUtlVector< CUtlString* >
}

pub mod C_DOTA_Item {
    pub const m_bCombinable: usize = 0x5B0; // bool
    pub const m_bPermanent: usize = 0x5B1; // bool
    pub const m_bStackable: usize = 0x5B2; // bool
    pub const m_iStackableMax: usize = 0x5B4; // int32
    pub const m_bRecipe: usize = 0x5B8; // bool
    pub const m_bRecipeConsumesCharges: usize = 0x5B9; // bool
    pub const m_iSharability: usize = 0x5BC; // int32
    pub const m_bDroppable: usize = 0x5C0; // bool
    pub const m_bPurchasable: usize = 0x5C1; // bool
    pub const m_bSellable: usize = 0x5C2; // bool
    pub const m_bInitiallySellable: usize = 0x5C3; // bool
    pub const m_bForceUnsellable: usize = 0x5C4; // bool
    pub const m_bRequiresCharges: usize = 0x5C5; // bool
    pub const m_bDisplayCharges: usize = 0x5C6; // bool
    pub const m_bHideCharges: usize = 0x5C7; // bool
    pub const m_bKillable: usize = 0x5C8; // bool
    pub const m_bGloballyCombinable: usize = 0x5C9; // bool
    pub const m_bDisassemblable: usize = 0x5CA; // bool
    pub const m_bIsNeutralDrop: usize = 0x5CB; // bool
    pub const m_nNeutralDropTeam: usize = 0x5CC; // int32
    pub const m_bAlertable: usize = 0x5D0; // bool
    pub const m_iInitialCharges: usize = 0x5D4; // int32
    pub const m_bCastOnPickup: usize = 0x5D8; // bool
    pub const m_bOnlyPlayerHeroPickup: usize = 0x5D9; // bool
    pub const m_bCreepHeroPickup: usize = 0x5DA; // bool
    pub const m_iValuelessCharges: usize = 0x5DC; // int32
    pub const m_iCurrentCharges: usize = 0x5E0; // int32
    pub const m_iSecondaryCharges: usize = 0x5E4; // int32
    pub const m_iMaxCharges: usize = 0x5E8; // int32
    pub const m_bCombineLocked: usize = 0x5EC; // bool
    pub const m_flPurchaseTime: usize = 0x5F0; // GameTime_t
    pub const m_flAssembledTime: usize = 0x5F4; // GameTime_t
    pub const m_bPurchasedWhileDead: usize = 0x5F8; // bool
    pub const m_bCanBeUsedOutOfInventory: usize = 0x5F9; // bool
    pub const m_bItemEnabled: usize = 0x5FA; // bool
    pub const m_flEnableTime: usize = 0x5FC; // GameTime_t
    pub const m_flReclaimTime: usize = 0x600; // GameTime_t
    pub const m_bDisplayOwnership: usize = 0x604; // bool
    pub const m_bShowOnMinimap: usize = 0x605; // bool
    pub const m_flMinimapIconSize: usize = 0x608; // float32
    pub const m_bIsUpgradeable: usize = 0x60C; // bool
    pub const m_nUpgradeProgress: usize = 0x610; // int32
    pub const m_nUpgradeGoal: usize = 0x614; // int32
    pub const m_hOldOwnerEntity: usize = 0x618; // CHandle< C_BaseEntity >
    pub const m_iOldCharges: usize = 0x61C; // int32
    pub const m_iPlayerOwnerID: usize = 0x620; // PlayerID_t
    pub const m_vecPreGameTransferPlayerIDs: usize = 0x628; // C_NetworkUtlVectorBase< uint8 >
}

pub mod C_DOTA_Item_AeonDisk {
    pub const m_nUses: usize = 0x648; // int32
    pub const max_level: usize = 0x64C; // int32
}

pub mod C_DOTA_Item_Ancient_Janggo {
    pub const radius: usize = 0x648; // int32
}

pub mod C_DOTA_Item_Arcane_Scout {
    pub const m_hScout: usize = 0x648; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Item_Armlet {
    pub const toggle_cooldown: usize = 0x648; // float32
}

pub mod C_DOTA_Item_Barricade {
    pub const m_hScout: usize = 0x648; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Item_Black_King_Bar {
    pub const m_bActive: usize = 0x648; // bool
}

pub mod C_DOTA_Item_BootsOfTravel {
    pub const m_flTeleportTimeOverride: usize = 0x648; // float32
    pub const m_bPendingSellExcessScrolls: usize = 0x64C; // bool
    pub const m_bQueueSellScrolls: usize = 0x64D; // bool
}

pub mod C_DOTA_Item_Boots_Of_Bearing {
    pub const radius: usize = 0x648; // int32
}

pub mod C_DOTA_Item_Bottomless_Chalice {
    pub const m_iStoredRuneType: usize = 0x648; // int32
    pub const m_fStoredRuneTime: usize = 0x64C; // GameTime_t
    pub const stored_rune_duration: usize = 0x650; // float32
}

pub mod C_DOTA_Item_Bracer {
    pub const counter: usize = 0x648; // int32
}

pub mod C_DOTA_Item_DataDriven {
    pub const m_bProcsMagicStick: usize = 0x648; // bool
    pub const m_bIsSharedWithTeammates: usize = 0x649; // bool
    pub const m_bCastFilterRejectCaster: usize = 0x64A; // bool
    pub const m_fAnimationPlaybackRate: usize = 0x64C; // float32
    pub const m_fAOERadius: usize = 0x650; // float32
    pub const m_CastAnimation: usize = 0x654; // int32
    pub const m_ModifierKVDescriptions: usize = 0x658; // CUtlVector< KeyValues* >
    pub const m_pOnChannelFinishKV: usize = 0x670; // KeyValues*
    pub const m_pOnChannelSucceededKV: usize = 0x678; // KeyValues*
    pub const m_pOnChannelInterruptedKV: usize = 0x680; // KeyValues*
    pub const m_pOnOwnerSpawnedKV: usize = 0x688; // KeyValues*
    pub const m_pOnOwnerDiedKV: usize = 0x690; // KeyValues*
    pub const m_pOnProjectileHitUnitKV: usize = 0x698; // KeyValues*
    pub const m_pOnProjectileFinishKV: usize = 0x6A0; // KeyValues*
    pub const m_pOnSpellStartKV: usize = 0x6A8; // KeyValues*
    pub const m_pOnAbilityPhaseStartKV: usize = 0x6B0; // KeyValues*
    pub const m_pOnToggleOnKV: usize = 0x6B8; // KeyValues*
    pub const m_pOnToggleOffKV: usize = 0x6C0; // KeyValues*
    pub const m_pOnEquipKV: usize = 0x6C8; // KeyValues*
    pub const m_pOnUnequipKV: usize = 0x6D0; // KeyValues*
    pub const m_pOnCreatedKV: usize = 0x6D8; // KeyValues*
}

pub mod C_DOTA_Item_EagleEye {
    pub const m_iTempViewer: usize = 0x648; // int32
    pub const m_iTeam: usize = 0x64C; // int32
    pub const m_nFXIndex: usize = 0x650; // int32
}

pub mod C_DOTA_Item_EmptyBottle {
    pub const m_iStoredRuneType: usize = 0x648; // int32
    pub const m_fStoredRuneTime: usize = 0x64C; // GameTime_t
    pub const rune_expire_time: usize = 0x650; // float32
}

pub mod C_DOTA_Item_GemOfTrueSight {
    pub const m_iTempViewer: usize = 0x648; // int32
    pub const m_iTeam: usize = 0x64C; // int32
    pub const m_nFXIndex: usize = 0x650; // int32
}

pub mod C_DOTA_Item_Grandmasters_Glaive {
    pub const m_iStat: usize = 0x648; // int32
}

pub mod C_DOTA_Item_MeteorHammer {
    pub const m_nFXIndex: usize = 0x648; // ParticleIndex_t
    pub const m_nFXIndexB: usize = 0x64C; // ParticleIndex_t
}

pub mod C_DOTA_Item_Muertas_Gun {
    pub const radius: usize = 0x648; // int32
    pub const ricochet_radius_start: usize = 0x64C; // int32
    pub const ricochet_radius_end: usize = 0x650; // int32
    pub const m_iTempViewerRadiant: usize = 0x654; // int32
    pub const m_iTempViewerDire: usize = 0x658; // int32
}

pub mod C_DOTA_Item_Necronomicon {
    pub const m_hWarrior: usize = 0x648; // CHandle< C_BaseEntity >
    pub const m_hArcher: usize = 0x64C; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Item_NullTalisman {
    pub const counter: usize = 0x648; // int32
}

pub mod C_DOTA_Item_Orb_Of_Revelations {
    pub const m_ViewerTimer: usize = 0x648; // CountdownTimer
    pub const m_iProjectile: usize = 0x660; // int32
}

pub mod C_DOTA_Item_Overflowing_Elixir {
    pub const m_iStoredRuneType: usize = 0x648; // int32
    pub const m_fStoredRuneTime: usize = 0x64C; // GameTime_t
}

pub mod C_DOTA_Item_Physical {
    pub const m_hItem: usize = 0x7E0; // CHandle< C_DOTA_Item >
    pub const m_bIsLowPriorityHoverItem: usize = 0x7E4; // bool
    pub const m_hOldItem: usize = 0x7E8; // CHandle< C_DOTA_Item >
    pub const m_pszParticleName: usize = 0x7F0; // char*
    pub const m_nFXIndex: usize = 0x7F8; // ParticleIndex_t
    pub const m_bShowingTooltip: usize = 0x7FC; // bool
    pub const m_bShowingSimpleTooltip: usize = 0x7FD; // bool
}

pub mod C_DOTA_Item_PowerTreads {
    pub const m_iStat: usize = 0x648; // int32
}

pub mod C_DOTA_Item_RiverPainter {
    pub const m_iRiverPaintColor: usize = 0x648; // int32
}

pub mod C_DOTA_Item_Rune {
    pub const m_iRuneType: usize = 0x7E0; // int32
    pub const m_flRuneTime: usize = 0x7E4; // float32
    pub const m_nMapLocationTeam: usize = 0x7E8; // int32
    pub const m_szLocation: usize = 0x7EC; // char[512]
    pub const m_iOldRuneType: usize = 0x9EC; // int32
    pub const m_bShowingTooltip: usize = 0x9F0; // bool
}

pub mod C_DOTA_Item_RuneSpawner {
    pub const m_nRuneType: usize = 0x7E0; // DOTA_RUNES
    pub const m_flLastSpawnTime: usize = 0x7E4; // float32
    pub const m_flNextSpawnTime: usize = 0x7E8; // float32
}

pub mod C_DOTA_Item_RuneSpawner_Bounty {
    pub const m_nRuneType: usize = 0x7E0; // DOTA_RUNES
    pub const m_flLastSpawnTime: usize = 0x7E4; // float32
    pub const m_flNextSpawnTime: usize = 0x7E8; // float32
}

pub mod C_DOTA_Item_RuneSpawner_Powerup {
    pub const m_nRuneType: usize = 0x7E0; // DOTA_RUNES
    pub const m_flLastSpawnTime: usize = 0x7E4; // float32
    pub const m_flNextSpawnTime: usize = 0x7E8; // float32
    pub const m_bNextRuneIsWater: usize = 0x7EC; // bool
}

pub mod C_DOTA_Item_RuneSpawner_XP {
    pub const m_nRuneType: usize = 0x7E0; // DOTA_RUNES
    pub const m_flLastSpawnTime: usize = 0x7E4; // float32
    pub const m_flNextSpawnTime: usize = 0x7E8; // float32
}

pub mod C_DOTA_Item_SheepStick {
    pub const upgrade_radius: usize = 0x648; // int32
}

pub mod C_DOTA_Item_SpookyTeleportScroll {
    pub const m_hTeleportTarget: usize = 0x648; // CHandle< C_BaseEntity >
    pub const m_nFXOrigin: usize = 0x64C; // ParticleIndex_t
    pub const m_nFXDestination: usize = 0x650; // ParticleIndex_t
    pub const m_vDestination: usize = 0x654; // Vector
    pub const m_iMinDistance: usize = 0x660; // int32
    pub const m_flBaseTeleportTime: usize = 0x664; // float32
    pub const m_flExtraTeleportTime: usize = 0x668; // float32
}

pub mod C_DOTA_Item_Spring2021_DefusalBomb {
    pub const m_bInBombSite: usize = 0x648; // bool
}

pub mod C_DOTA_Item_TeleportScroll {
    pub const m_hTeleportTarget: usize = 0x648; // CHandle< C_BaseEntity >
    pub const m_nFXOrigin: usize = 0x64C; // ParticleIndex_t
    pub const m_nFXDestination: usize = 0x650; // ParticleIndex_t
    pub const m_vDestination: usize = 0x654; // Vector
    pub const m_iMinDistance: usize = 0x660; // int32
    pub const m_flBaseTeleportTime: usize = 0x664; // float32
    pub const m_flExtraTeleportTime: usize = 0x668; // float32
}

pub mod C_DOTA_Item_TierToken {
    pub const m_vecChoices: usize = 0x648; // AbilityID_t[5]
}

pub mod C_DOTA_Item_Tombstone {
    pub const m_flTimer: usize = 0x648; // float32
    pub const m_nFXIndex: usize = 0x64C; // ParticleIndex_t
    pub const m_nFXIndex2: usize = 0x650; // ParticleIndex_t
}

pub mod C_DOTA_Item_TranquilBoots {
    pub const break_count: usize = 0x648; // int32
    pub const m_DamageList: usize = 0x650; // CUtlVector< GameTime_t >
}

pub mod C_DOTA_Item_TranquilBoots2 {
    pub const break_count: usize = 0x648; // int32
    pub const m_DamageList: usize = 0x650; // CUtlVector< GameTime_t >
}

pub mod C_DOTA_Item_TurtleShell {
    pub const duration: usize = 0x648; // float32
}

pub mod C_DOTA_Item_Vambrace {
    pub const m_iStat: usize = 0x648; // int32
}

pub mod C_DOTA_Item_WraithBand {
    pub const counter: usize = 0x648; // int32
}

pub mod C_DOTA_Item_WraithPact {
    pub const m_hWard: usize = 0x648; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_LinearProjectileInfo {
    pub const m_transform: usize = 0x20; // CTransform
    pub const m_iHandle: usize = 0x40; // int32
    pub const m_vInitPosition: usize = 0x44; // Vector
    pub const m_vPosition: usize = 0x50; // Vector
    pub const m_vOriginalVelocity: usize = 0x5C; // Vector
    pub const m_vVelocity: usize = 0x68; // Vector
    pub const m_vAcceleration: usize = 0x74; // Vector
    pub const m_flMaxSpeed: usize = 0x80; // float32
    pub const m_flFowRadius: usize = 0x84; // float32
    pub const m_bStickyFoWReveal: usize = 0x88; // bool
    pub const m_flDistance: usize = 0x8C; // float32
    pub const m_hSource: usize = 0x90; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_MapTree {
    pub const m_bInitialized: usize = 0x818; // bool
}

pub mod C_DOTA_NPC_Aghsfort_WitchDoctor_Ward {
    pub const m_nFXSkullIndex: usize = 0x1750; // ParticleIndex_t
    pub const m_nTargetType: usize = 0x1754; // int32
    pub const m_nTargetFlags: usize = 0x1758; // int32
}

pub mod C_DOTA_NPC_Lantern {
    pub const m_szInteractAbilityName: usize = 0x17B8; // CUtlSymbolLarge
    pub const m_pVisionRangeFX: usize = 0x17C0; // CNewParticleEffect*
    pub const m_iFxIndex: usize = 0x17C8; // ParticleIndex_t
    pub const m_nPreviewViewer: usize = 0x17CC; // int32
    pub const m_nTeamNumberShown: usize = 0x17D0; // int32
}

pub mod C_DOTA_NPC_TechiesMines {
    pub const m_iRangeFX: usize = 0x1750; // ParticleIndex_t
}

pub mod C_DOTA_NPC_WitchDoctor_Ward {
    pub const m_nFXSkullIndex: usize = 0x1750; // ParticleIndex_t
    pub const m_nTargetType: usize = 0x1754; // int32
    pub const m_nTargetFlags: usize = 0x1758; // int32
}

pub mod C_DOTA_NeutralSpawner {
    pub const m_Type: usize = 0x538; // int32
}

pub mod C_DOTA_PhantomAssassin_Gravestone {
    pub const m_nVictimPlayerID: usize = 0x1750; // PlayerID_t
}

pub mod C_DOTA_PlayerResource {
    pub const m_bWasDataUpdateCreated: usize = 0x548; // bool
    pub const m_vecPlayerTeamData: usize = 0x550; // C_UtlVectorEmbeddedNetworkVar< PlayerResourcePlayerTeamData_t >
    pub const m_vecPlayerData: usize = 0x5A0; // C_UtlVectorEmbeddedNetworkVar< PlayerResourcePlayerData_t >
    pub const m_vecBrodcasterData: usize = 0x5F0; // C_UtlVectorEmbeddedNetworkVar< PlayerResourceBroadcasterData_t >
    pub const m_vecEventsForDisplay: usize = 0x640; // C_NetworkUtlVectorBase< uint32 >
    pub const m_nPrimaryEventIndex: usize = 0x658; // int8
    pub const m_nObsoleteEventIDAssociatedWithEventData: usize = 0x65C; // uint32
    pub const m_playerIDToPlayer: usize = 0x660; // CHandle< C_BaseEntity >[64]
    pub const m_iszName: usize = 0x760; // CUtlSymbolLarge[64]
    pub const m_iszHTMLSafeName: usize = 0x960; // CUtlSymbolLarge[64]
    pub const m_iszFilteredHTMLSafeName: usize = 0xB60; // CUtlSymbolLarge[64]
    pub const m_bDirtySuggestedItems: usize = 0x2560; // bool
    pub const m_bDirtyEstimatedMatchDuration: usize = 0x2561; // bool
    pub const m_bDirtySelection: usize = 0x2562; // bool
    pub const m_bHasWorldTreesChanged: usize = 0x2563; // bool
    pub const m_bWorldTreeModelsChanged: usize = 0x2564; // bool
    pub const m_bSwapWillingness: usize = 0x2565; // bool[24]
    pub const m_hTeamCouriers: usize = 0x25C0; // CUtlVector< CHandle< C_DOTA_Unit_Courier > >[15]
    pub const m_hPlayerCouriers: usize = 0x2728; // CUtlVector< CHandle< C_DOTA_Unit_Courier > >[64]
    pub const m_vecOnstageHomeTeams: usize = 0x2D28; // CUtlVector< uint32 >
    pub const m_pPlayerIDToOnstageSlot: usize = 0x2D40; // PlayerSeatAssignment_t*[24]
    pub const m_vecOnstagePlayerSeats: usize = 0x2E00; // CUtlVector< PlayerSeatAssignment_t >
    pub const m_nEventNPCReplaced: usize = 0x2E18; // int32
    pub const m_nEventPlayerInfo: usize = 0x2E1C; // int32
    pub const m_nInventoryUpdated: usize = 0x2E20; // int32
}

pub mod C_DOTA_PortraitBuilding {
    pub const m_nAmbientFXIndex: usize = 0x7E0; // ParticleIndex_t
    pub const m_nFXIndex: usize = 0x7E8; // CStrongHandle< InfoForResourceTypeIParticleSystemDefinition >
    pub const m_ParticleTintColor: usize = 0x7F0; // Color
}

pub mod C_DOTA_PortraitEntity {
    pub const m_PetIdleTimer: usize = 0x1750; // CountdownTimer
    pub const m_nMouthFX: usize = 0x1768; // ParticleIndex_t
    pub const m_nMouthControlPoint: usize = 0x176C; // int32
    pub const m_iPortraitParticle: usize = 0x1770; // ParticleIndex_t
    pub const m_PortraitActivity: usize = 0x1774; // int32
    pub const m_CustomActivityModifiers: usize = 0x1778; // CUtlVector< CUtlSymbol >
    pub const m_bIsSimulationActive: usize = 0x1790; // bool
    pub const m_hAppearanceFromNPC: usize = 0x1794; // CEntityHandle
}

pub mod C_DOTA_RoshanSpawner {
    pub const m_iKillCount: usize = 0x538; // int32
    pub const m_vRoshanAltLocation: usize = 0x53C; // Vector
}

pub mod C_DOTA_Samurai_Tabi {
    pub const m_iStat: usize = 0x648; // int32
}

pub mod C_DOTA_SimpleObstruction {
    pub const m_bEnabled: usize = 0x538; // bool
    pub const m_bBlockFoW: usize = 0x539; // bool
    pub const m_unOccluderID: usize = 0x53C; // uint32
    pub const m_bBlockingGridNav: usize = 0x540; // bool
    pub const m_bPrevEnabled: usize = 0x541; // bool
}

pub mod C_DOTA_TempTree {
    pub const m_fExpireTime: usize = 0x810; // GameTime_t
    pub const m_vecTreeCircleCenter: usize = 0x814; // Vector
    pub const m_bCanApplyTreeCostume: usize = 0x820; // bool
    pub const m_bIsMangoTree: usize = 0x821; // bool
    pub const m_Particles: usize = 0x828; // CUtlVector< ParticleIndex_t >
    pub const m_bIsPartOfFowSystem: usize = 0x840; // bool
    pub const m_bHasClientSideAnimation: usize = 0x841; // bool
}

pub mod C_DOTA_TrackingProjectileInfo {
    pub const m_iHandle: usize = 0x0; // int32
    pub const m_iOriginalMoveSpeed: usize = 0x4; // int32
    pub const m_iMoveSpeed: usize = 0x8; // int32
    pub const m_vLocation: usize = 0xC; // Vector
    pub const m_hSource: usize = 0x18; // CHandle< C_BaseEntity >
    pub const m_hTarget: usize = 0x1C; // CHandle< C_BaseEntity >
    pub const m_hAbility: usize = 0x20; // CHandle< C_BaseEntity >
    pub const m_vTargetLocation: usize = 0x24; // Vector
    pub const m_bDodgeable: usize = 0x30; // bool
    pub const m_bIsAttack: usize = 0x31; // bool
    pub const m_bIsEvaded: usize = 0x32; // bool
    pub const m_flExpireTime: usize = 0x34; // GameTime_t
    pub const m_flMaxImpactTime: usize = 0x38; // GameTime_t
}

pub mod C_DOTA_UnitInventory {
    pub const m_SharedCooldownList: usize = 0x8; // CUtlVector< sSharedCooldownInfo >
    pub const m_hItems: usize = 0x20; // CHandle< C_BaseEntity >[19]
    pub const m_bItemQueried: usize = 0x6C; // bool[19]
    pub const m_iParity: usize = 0x80; // int32
    pub const m_hInventoryParent: usize = 0x84; // CHandle< C_BaseEntity >
    pub const m_bIsActive: usize = 0x88; // bool
    pub const m_bStashEnabled: usize = 0x89; // bool
    pub const m_hTransientCastItem: usize = 0x8C; // CHandle< C_BaseEntity >
    pub const m_bSendChangedMsg: usize = 0xA8; // bool
}

pub mod C_DOTA_Unit_AghsFort_Creature_DungeonBat {
    pub const m_nFXAmbient: usize = 0x1760; // ParticleIndex_t
}

pub mod C_DOTA_Unit_Aghsfort_Aziyog_Underlord_Portal {
    pub const m_hOtherPortal: usize = 0x1750; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Unit_Brewmaster_PrimalEarth {
    pub const m_nFXEarthAmbient1: usize = 0x1758; // ParticleIndex_t
    pub const m_nFXEarthAmbient2: usize = 0x175C; // ParticleIndex_t
}

pub mod C_DOTA_Unit_Brewmaster_PrimalFire {
    pub const m_nFXAmbient: usize = 0x1758; // ParticleIndex_t
}

pub mod C_DOTA_Unit_Brewmaster_PrimalStorm {
    pub const m_nFXStormAmbient1: usize = 0x1758; // ParticleIndex_t
    pub const m_nFXStormAmbient2: usize = 0x175C; // ParticleIndex_t
}

pub mod C_DOTA_Unit_Brewmaster_PrimalVoid {
    pub const m_nFXAmbient: usize = 0x1758; // ParticleIndex_t
}

pub mod C_DOTA_Unit_Broodmother_Web {
    pub const m_nFXIndex: usize = 0x1750; // ParticleIndex_t
    pub const m_vecOrigin: usize = 0x1754; // Vector
}

pub mod C_DOTA_Unit_Courier {
    pub const m_bUnitRespawned: usize = 0x1750; // bool
    pub const m_bPreUpdateFlyingCourier: usize = 0x1751; // bool
    pub const m_nSoleControllingPlayer: usize = 0x1754; // PlayerID_t
    pub const m_bFlyingCourier: usize = 0x1758; // bool
    pub const m_flRespawnTime: usize = 0x175C; // GameTime_t
    pub const m_nCourierState: usize = 0x1760; // CourierState_t
    pub const m_hCourierStateEntity: usize = 0x1764; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Unit_Diretide_Portal {
    pub const m_hPartnerPortal: usize = 0x17B8; // CHandle< C_BaseEntity >
    pub const m_nRuneType: usize = 0x17BC; // int32
    pub const m_nOriginalTeamNumber: usize = 0x17C0; // int32
    pub const m_nActiveFXIndex: usize = 0x17C4; // ParticleIndex_t
    pub const m_nRuneFXIndex: usize = 0x17C8; // ParticleIndex_t
}

pub mod C_DOTA_Unit_Hero_CrystalMaiden {
    pub const m_iParticleMouthIndex: usize = 0x1AB0; // int32
    pub const m_iParticleHandRIndex: usize = 0x1AB4; // int32
}

pub mod C_DOTA_Unit_Hero_DarkSeer {
    pub const m_nNormalPunchBuffIndex: usize = 0x1AB0; // ParticleIndex_t
}

pub mod C_DOTA_Unit_Hero_Earthshaker {
    pub const m_nFXDeath: usize = 0x1AB0; // ParticleIndex_t
}

pub mod C_DOTA_Unit_Hero_Juggernaut {
    pub const m_unOmniKills: usize = 0x1AB0; // uint32
}

pub mod C_DOTA_Unit_Hero_Kunkka {
    pub const m_nFXIndex: usize = 0x1AB0; // int32
}

pub mod C_DOTA_Unit_Hero_Legion_Commander {
    pub const m_unDuelsWon: usize = 0x1AB0; // uint32
}

pub mod C_DOTA_Unit_Hero_Lion {
    pub const m_unFingerPrestigeKills: usize = 0x1AB0; // uint32
}

pub mod C_DOTA_Unit_Hero_Meepo {
    pub const m_nWhichMeepo: usize = 0x1AB0; // int32
}

pub mod C_DOTA_Unit_Hero_MonkeyKing {
    pub const mb_MonkeyHasArcana: usize = 0x1AB0; // bool
    pub const m_nTreeDisguise: usize = 0x1AB4; // uint32
    pub const m_nPerchedTree: usize = 0x1AB8; // uint32
    pub const m_hTreeDisguiseEnt: usize = 0x1ABC; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Unit_Hero_Muerta {
    pub const m_hDoubleShotTarget: usize = 0x1AB0; // CHandle< C_BaseEntity >
    pub const m_qStartAngle: usize = 0x1AB4; // QAngle
}

pub mod C_DOTA_Unit_Hero_Nevermore {
    pub const m_nFXDeath: usize = 0x1AB0; // ParticleIndex_t
}

pub mod C_DOTA_Unit_Hero_Obsidian_Destroyer {
    pub const m_nFXDeath: usize = 0x1AB0; // ParticleIndex_t
}

pub mod C_DOTA_Unit_Hero_Oracle {
    pub const m_nFXDeath: usize = 0x1AB0; // ParticleIndex_t
}

pub mod C_DOTA_Unit_Hero_PhantomAssassin {
    pub const m_nFXDeath: usize = 0x1AB0; // ParticleIndex_t
    pub const m_nArcanaLevel: usize = 0x1AB4; // uint32
}

pub mod C_DOTA_Unit_Hero_PhantomLancer {
    pub const m_bInventoryEnabled: usize = 0x1AB0; // bool
}

pub mod C_DOTA_Unit_Hero_Razor {
    pub const m_iPoseParameterAim: usize = 0x1AB0; // int32
    pub const m_iPoseParameterRange: usize = 0x1AB4; // int32
    pub const m_nTargetAngle: usize = 0x1AB8; // int32
    pub const m_nTargetRange: usize = 0x1ABC; // int32
}

pub mod C_DOTA_Unit_Hero_Rubick {
    pub const m_stolenAbilityColorHSV1: usize = 0x1AB0; // Vector
    pub const m_stolenAbilityColorHSV2: usize = 0x1ABC; // Vector
    pub const m_stolenAbilityFXColorHSV: usize = 0x1AC8; // Vector
    pub const m_bHasInitializedAbilityColors: usize = 0x1AD4; // bool
    pub const m_startAbilityColorHSV1: usize = 0x1AD8; // Vector
    pub const m_startAbilityColorHSV2: usize = 0x1AE4; // Vector
    pub const m_startAbilityFXColorHSV: usize = 0x1AF0; // Vector
    pub const m_currAbilityColorHSV1: usize = 0x1AFC; // Vector
    pub const m_currAbilityColorHSV2: usize = 0x1B08; // Vector
    pub const m_currAbilityFXColorHSV: usize = 0x1B14; // Vector
    pub const m_flStartTime: usize = 0x1B20; // GameTime_t
}

pub mod C_DOTA_Unit_Hero_SkeletonKing {
    pub const m_nSkeletonWarriors: usize = 0x1AB0; // int32
    pub const m_vecInitialHeroKillBitmask: usize = 0x1AB4; // int32[5]
    pub const m_vecInGameHeroKillBitmask: usize = 0x1AC8; // int32[5]
}

pub mod C_DOTA_Unit_Hero_Spectre {
    pub const m_unCurrentArcanaKillStreak: usize = 0x1AB0; // uint8
    pub const m_unBestArcanaKillStreak: usize = 0x1AB1; // uint8
    pub const m_nVictimPlayerID: usize = 0x1AB4; // PlayerID_t
    pub const m_bArcanaKillstreakRecordBroken: usize = 0x1AB8; // bool
}

pub mod C_DOTA_Unit_Hero_Techies {
    pub const m_bLastDeathFromSuicide: usize = 0x1AB0; // bool
}

pub mod C_DOTA_Unit_Hero_Terrorblade {
    pub const m_nFXDeath: usize = 0x1AB0; // ParticleIndex_t
    pub const m_szResponseCriteria: usize = 0x1AB8; // CUtlString
    pub const m_nArcanaColor: usize = 0x1AC0; // uint32
}

pub mod C_DOTA_Unit_Hero_Tiny {
    pub const m_hTreeWearable: usize = 0x1AB0; // CHandle< C_BaseEntity >
    pub const m_nFXIndexScepterAmbient: usize = 0x1AB4; // ParticleIndex_t
    pub const m_hIllusionOwner: usize = 0x1AB8; // CHandle< C_DOTA_BaseNPC >
    pub const m_bIllusionHasTree: usize = 0x1ABC; // bool
}

pub mod C_DOTA_Unit_Hero_Windrunner {
    pub const m_nTargetAngle: usize = 0x1AB0; // int32
    pub const m_iPoseParameterAim: usize = 0x1AB4; // int32
    pub const m_nFXDeath: usize = 0x1AB8; // ParticleIndex_t
}

pub mod C_DOTA_Unit_Hero_Winter_Wyvern {
    pub const m_nFXDeath: usize = 0x1AB0; // ParticleIndex_t
}

pub mod C_DOTA_Unit_Hero_Wisp {
    pub const m_nAmbientFXIndex: usize = 0x1AB0; // ParticleIndex_t
    pub const m_nStunnedFXIndex: usize = 0x1AB4; // ParticleIndex_t
    pub const m_nTalkFXIndex: usize = 0x1AB8; // ParticleIndex_t
    pub const m_nIllusionFXIndex: usize = 0x1ABC; // ParticleIndex_t
    pub const m_bParticleHexed: usize = 0x1AC0; // bool
    pub const m_bParticleStunned: usize = 0x1AC1; // bool
    pub const m_bDetermineAmbientEffect: usize = 0x1AC2; // bool
    pub const m_flPrevHealth: usize = 0x1AC4; // float32
}

pub mod C_DOTA_Unit_LoopingSound {
    pub const m_nPrevLoopingSoundParity: usize = 0x1758; // int32
    pub const m_pszNetworkedSoundLoop: usize = 0x175C; // char[256]
    pub const m_nLoopingSoundParity: usize = 0x185C; // int32
}

pub mod C_DOTA_Unit_Miniboss {
    pub const m_nTimesSpawned: usize = 0x1750; // int32
    pub const m_nTempViewer: usize = 0x1754; // int32
    pub const m_hAttackingHeroes: usize = 0x1758; // CUtlVector< CHandle< C_BaseEntity > >
}

pub mod C_DOTA_Unit_Roshan {
    pub const m_iLastHealthPercent: usize = 0x1750; // int32
    pub const m_nFXIndex: usize = 0x1754; // ParticleIndex_t
    pub const m_hAttackingHeroes: usize = 0x1758; // CUtlVector< CHandle< C_BaseEntity > >
    pub const m_bGoldenRoshan: usize = 0x1770; // bool
    pub const m_bIsNightTimeMode: usize = 0x1771; // bool
}

pub mod C_DOTA_Unit_Scout {
    pub const m_bUnitRespawned: usize = 0x1750; // bool
    pub const m_nSoleControllingPlayer: usize = 0x1754; // PlayerID_t
    pub const m_flRespawnTime: usize = 0x1758; // GameTime_t
    pub const m_nScoutState: usize = 0x175C; // ScoutState_t
    pub const m_hScoutStateEntity: usize = 0x1760; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Unit_SpiritBear {
    pub const m_bShouldRespawn: usize = 0x1AB0; // bool
    pub const m_bStolen: usize = 0x1AB1; // bool
}

pub mod C_DOTA_Unit_TargetDummy {
    pub const m_flDamageTaken: usize = 0x1AB0; // float32
    pub const m_flLastHit: usize = 0x1AB4; // float32
    pub const m_flStartDamageTime: usize = 0x1AB8; // GameTime_t
    pub const m_flLastTargetDummyDamageTime: usize = 0x1ABC; // GameTime_t
    pub const m_bIsMoving: usize = 0x1AC0; // bool
}

pub mod C_DOTA_Unit_Tidehunter_Anchor {
    pub const m_hTarget: usize = 0x1750; // CHandle< C_BaseEntity >
    pub const m_vProjectilePosition: usize = 0x1754; // Vector
}

pub mod C_DOTA_Unit_Twin_Gate {
    pub const m_hOtherPortal: usize = 0x17B8; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Unit_Underlord_Portal {
    pub const m_hOtherPortal: usize = 0x17B8; // CHandle< C_BaseEntity >
}

pub mod C_DOTA_Unit_Undying_Zombie {
    pub const m_ctRespawn: usize = 0x1758; // CountdownTimer
    pub const m_pTombstone: usize = 0x1770; // C_DOTA_BaseNPC*
}

pub mod C_DOTA_Unit_VisageFamiliar {
    pub const m_nFXAmbient: usize = 0x1758; // ParticleIndex_t
}

pub mod C_DotaQuest {
    pub const m_pszQuestTitle: usize = 0x538; // char[256]
    pub const m_pszQuestText: usize = 0x638; // char[256]
    pub const m_nQuestType: usize = 0x738; // int32
    pub const m_hSubquests: usize = 0x73C; // CHandle< C_DotaSubquestBase >[8]
    pub const m_bHidden: usize = 0x75C; // bool
    pub const m_bCompleted: usize = 0x75D; // bool
    pub const m_bWinIfCompleted: usize = 0x75E; // bool
    pub const m_bLoseIfCompleted: usize = 0x75F; // bool
    pub const m_pszGameEndText: usize = 0x760; // char[256]
    pub const m_pnTextReplaceValuesCDotaQuest: usize = 0x860; // int32[4]
    pub const m_pszTextReplaceString: usize = 0x870; // char[64]
    pub const m_nTextReplaceValueVersion: usize = 0x8B0; // int32
    pub const m_bWasCompleted: usize = 0x8B4; // bool
}

pub mod C_DotaSubquestBase {
    pub const m_pszSubquestText: usize = 0x538; // char[256]
    pub const m_bHidden: usize = 0x638; // bool
    pub const m_bCompleted: usize = 0x639; // bool
    pub const m_bShowProgressBar: usize = 0x63A; // bool
    pub const m_nProgressBarHueShift: usize = 0x63C; // int32
    pub const m_pnTextReplaceValuesCDotaSubquestBase: usize = 0x640; // int32[2]
    pub const m_pszTextReplaceString: usize = 0x648; // char[64]
    pub const m_nTextReplaceValueVersion: usize = 0x688; // int32
    pub const m_bWasCompleted: usize = 0x68C; // bool
}

pub mod C_DotaTree {
    pub const m_unOccluderID: usize = 0x2C; // uint32
}

pub mod C_DotaTutorialNetworker {
    pub const m_nTutorialState: usize = 0x538; // int32
    pub const m_nTaskProgress: usize = 0x53C; // int32
    pub const m_nTaskSteps: usize = 0x540; // int32
    pub const m_nTaskSecondsRemianing: usize = 0x544; // int32
    pub const m_nUIState: usize = 0x548; // int32
    pub const m_nShopState: usize = 0x54C; // int32
    pub const m_TargetLocation: usize = 0x550; // Vector
    pub const m_TargetEntity: usize = 0x55C; // CHandle< C_BaseEntity >
    pub const m_SpeechBubbles: usize = 0x560; // C_UtlVectorEmbeddedNetworkVar< C_SpeechBubbleInfo >
    pub const m_nLocationID: usize = 0xAC0; // int32
    pub const m_GuideStr: usize = 0xAC4; // char[256]
    pub const m_QuickBuyStr: usize = 0xBC4; // char[256]
    pub const m_nPreTutorialState: usize = 0xCC4; // int32
    pub const m_nPreUIState: usize = 0xCC8; // int32
    pub const m_nPreShopState: usize = 0xCCC; // int32
    pub const m_vecPrevTargetLocation: usize = 0xCD0; // Vector
    pub const m_hPrevTargetEntity: usize = 0xCDC; // CHandle< C_BaseEntity >
}

pub mod C_DynamicLight {
    pub const m_Flags: usize = 0x7B8; // uint8
    pub const m_LightStyle: usize = 0x7B9; // uint8
    pub const m_Radius: usize = 0x7BC; // float32
    pub const m_Exponent: usize = 0x7C0; // int32
    pub const m_InnerAngle: usize = 0x7C4; // float32
    pub const m_OuterAngle: usize = 0x7C8; // float32
    pub const m_SpotRadius: usize = 0x7CC; // float32
}

pub mod C_DynamicProp {
    pub const m_bRandomAnimator: usize = 0x930; // bool
    pub const m_flNextRandAnim: usize = 0x934; // GameTime_t
    pub const m_flMinRandAnimDuration: usize = 0x938; // float32
    pub const m_flMaxRandAnimDuration: usize = 0x93C; // float32
    pub const m_bUseHitboxesForRenderBox: usize = 0x940; // bool
    pub const m_bUseAnimGraph: usize = 0x941; // bool
    pub const m_pOutputAnimBegun: usize = 0x948; // CEntityIOOutput
    pub const m_pOutputAnimOver: usize = 0x970; // CEntityIOOutput
    pub const m_pOutputAnimLoopCycleOver: usize = 0x998; // CEntityIOOutput
    pub const m_OnAnimReachedStart: usize = 0x9C0; // CEntityIOOutput
    pub const m_OnAnimReachedEnd: usize = 0x9E8; // CEntityIOOutput
    pub const m_iszDefaultAnim: usize = 0xA10; // CUtlSymbolLarge
    pub const m_nDefaultAnimLoopMode: usize = 0xA18; // AnimLoopMode_t
    pub const m_bAnimateOnServer: usize = 0xA1C; // bool
    pub const m_bRandomizeCycle: usize = 0xA1D; // bool
    pub const m_bStartDisabled: usize = 0xA1E; // bool
    pub const m_bScriptedMovement: usize = 0xA1F; // bool
    pub const m_bFiredStartEndOutput: usize = 0xA20; // bool
    pub const m_bForceNpcExclude: usize = 0xA21; // bool
    pub const m_bCreateNonSolid: usize = 0xA22; // bool
    pub const m_bIsOverrideProp: usize = 0xA23; // bool
    pub const m_iInitialGlowState: usize = 0xA24; // int32
    pub const m_nGlowRange: usize = 0xA28; // int32
    pub const m_nGlowRangeMin: usize = 0xA2C; // int32
    pub const m_glowColor: usize = 0xA30; // Color
    pub const m_nGlowTeam: usize = 0xA34; // int32
    pub const m_iCachedFrameCount: usize = 0xA38; // int32
    pub const m_vecCachedRenderMins: usize = 0xA3C; // Vector
    pub const m_vecCachedRenderMaxs: usize = 0xA48; // Vector
}

pub mod C_DynamicPropClientside {
    pub const m_bSetupMaterialProxy: usize = 0xA60; // bool
    pub const m_bNoInterpolate: usize = 0xA61; // bool
}

pub mod C_EconEntity {
    pub const m_AttributeManager: usize = 0x978; // CAttributeContainer
    pub const m_bClientside: usize = 0xAA0; // bool
    pub const m_nDisableMode: usize = 0xAA4; // EconEntityParticleDisableMode_t
    pub const m_bParticleSystemsCreated: usize = 0xAA8; // bool
    pub const m_bForceDestroyAttachedParticlesImmediately: usize = 0xAA9; // bool
    pub const m_vecAttachedParticles: usize = 0xAB0; // CUtlVector< C_EconEntity::AttachedParticleInfo_t >
    pub const m_hViewmodelAttachment: usize = 0xAC8; // CHandle< CBaseAnimatingActivity >
    pub const m_iOldTeam: usize = 0xACC; // int32
    pub const m_bAttachmentDirty: usize = 0xAD0; // bool
    pub const m_iOldStyle: usize = 0xAD1; // style_index_t
    pub const m_hOldProvidee: usize = 0xAD4; // CHandle< C_BaseEntity >
    pub const m_vecAttachedModels: usize = 0xAD8; // CUtlVector< C_EconEntity::AttachedModelData_t >
}

pub mod C_EconEntity_AttachedModelData_t {
    pub const m_iModelDisplayFlags: usize = 0x0; // int32
}

pub mod C_EconEntity_AttachedParticleInfo_t {
    pub const m_nAttachedParticleIndex: usize = 0x0; // ParticleIndex_t
    pub const m_customType: usize = 0x4; // CUtlStringToken
    pub const m_bShouldDestroyImmediately: usize = 0x8; // bool
}

pub mod C_EconItemAttribute {
    pub const m_iAttributeDefinitionIndex: usize = 0x30; // attrib_definition_index_t
    pub const m_flValue: usize = 0x34; // float32
}

pub mod C_EconItemView {
    pub const m_iItemDefinitionIndex: usize = 0x8; // item_definition_index_t
    pub const m_iEntityQuality: usize = 0xC; // int32
    pub const m_iEntityLevel: usize = 0x10; // uint32
    pub const m_iItemID: usize = 0x18; // itemid_t
    pub const m_iAccountID: usize = 0x20; // uint32
    pub const m_iInventoryPosition: usize = 0x24; // uint32
    pub const m_bInitialized: usize = 0x30; // bool
    pub const m_nOverrideStyle: usize = 0x31; // style_index_t
    pub const m_bIsStoreItem: usize = 0x32; // bool
    pub const m_bIsTradeItem: usize = 0x33; // bool
    pub const m_bHasComputedAttachedParticles: usize = 0x34; // bool
    pub const m_bHasAttachedParticles: usize = 0x35; // bool
    pub const m_iEntityQuantity: usize = 0x38; // int32
    pub const m_unClientFlags: usize = 0x3C; // uint8
    pub const m_unOverrideOrigin: usize = 0x40; // eEconItemOrigin
    pub const m_pszGrayedOutReason: usize = 0x50; // char*
    pub const m_AttributeList: usize = 0x58; // CAttributeList
}

pub mod C_EntityDissolve {
    pub const m_flStartTime: usize = 0x7C0; // GameTime_t
    pub const m_flFadeInStart: usize = 0x7C4; // float32
    pub const m_flFadeInLength: usize = 0x7C8; // float32
    pub const m_flFadeOutModelStart: usize = 0x7CC; // float32
    pub const m_flFadeOutModelLength: usize = 0x7D0; // float32
    pub const m_flFadeOutStart: usize = 0x7D4; // float32
    pub const m_flFadeOutLength: usize = 0x7D8; // float32
    pub const m_flNextSparkTime: usize = 0x7DC; // GameTime_t
    pub const m_nDissolveType: usize = 0x7E0; // EntityDisolveType_t
    pub const m_vDissolverOrigin: usize = 0x7E4; // Vector
    pub const m_nMagnitude: usize = 0x7F0; // uint32
    pub const m_bCoreExplode: usize = 0x7F4; // bool
    pub const m_bLinkedToServerEnt: usize = 0x7F5; // bool
}

pub mod C_EntityFlame {
    pub const m_hEntAttached: usize = 0x538; // CHandle< C_BaseEntity >
    pub const m_hOldAttached: usize = 0x560; // CHandle< C_BaseEntity >
    pub const m_bCheapEffect: usize = 0x564; // bool
}

pub mod C_EnvCombinedLightProbeVolume {
    pub const m_Color: usize = 0x15A0; // Color
    pub const m_flBrightness: usize = 0x15A4; // float32
    pub const m_hCubemapTexture: usize = 0x15A8; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_bCustomCubemapTexture: usize = 0x15B0; // bool
    pub const m_hLightProbeTexture: usize = 0x15B8; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_hLightProbeDirectLightIndicesTexture: usize = 0x15C0; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_hLightProbeDirectLightScalarsTexture: usize = 0x15C8; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_hLightProbeDirectLightShadowsTexture: usize = 0x15D0; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_vBoxMins: usize = 0x15D8; // Vector
    pub const m_vBoxMaxs: usize = 0x15E4; // Vector
    pub const m_LightGroups: usize = 0x15F0; // CUtlSymbolLarge
    pub const m_bMoveable: usize = 0x15F8; // bool
    pub const m_nHandshake: usize = 0x15FC; // int32
    pub const m_nEnvCubeMapArrayIndex: usize = 0x1600; // int32
    pub const m_nPriority: usize = 0x1604; // int32
    pub const m_bStartDisabled: usize = 0x1608; // bool
    pub const m_flEdgeFadeDist: usize = 0x160C; // float32
    pub const m_vEdgeFadeDists: usize = 0x1610; // Vector
    pub const m_nLightProbeSizeX: usize = 0x161C; // int32
    pub const m_nLightProbeSizeY: usize = 0x1620; // int32
    pub const m_nLightProbeSizeZ: usize = 0x1624; // int32
    pub const m_nLightProbeAtlasX: usize = 0x1628; // int32
    pub const m_nLightProbeAtlasY: usize = 0x162C; // int32
    pub const m_nLightProbeAtlasZ: usize = 0x1630; // int32
    pub const m_bEnabled: usize = 0x1649; // bool
}

pub mod C_EnvCubemap {
    pub const m_hCubemapTexture: usize = 0x5C0; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_bCustomCubemapTexture: usize = 0x5C8; // bool
    pub const m_flInfluenceRadius: usize = 0x5CC; // float32
    pub const m_vBoxProjectMins: usize = 0x5D0; // Vector
    pub const m_vBoxProjectMaxs: usize = 0x5DC; // Vector
    pub const m_LightGroups: usize = 0x5E8; // CUtlSymbolLarge
    pub const m_bMoveable: usize = 0x5F0; // bool
    pub const m_nHandshake: usize = 0x5F4; // int32
    pub const m_nEnvCubeMapArrayIndex: usize = 0x5F8; // int32
    pub const m_nPriority: usize = 0x5FC; // int32
    pub const m_flEdgeFadeDist: usize = 0x600; // float32
    pub const m_vEdgeFadeDists: usize = 0x604; // Vector
    pub const m_flDiffuseScale: usize = 0x610; // float32
    pub const m_bStartDisabled: usize = 0x614; // bool
    pub const m_bDefaultEnvMap: usize = 0x615; // bool
    pub const m_bDefaultSpecEnvMap: usize = 0x616; // bool
    pub const m_bIndoorCubeMap: usize = 0x617; // bool
    pub const m_bCopyDiffuseFromDefaultCubemap: usize = 0x618; // bool
    pub const m_bEnabled: usize = 0x628; // bool
}

pub mod C_EnvCubemapFog {
    pub const m_flEndDistance: usize = 0x538; // float32
    pub const m_flStartDistance: usize = 0x53C; // float32
    pub const m_flFogFalloffExponent: usize = 0x540; // float32
    pub const m_bHeightFogEnabled: usize = 0x544; // bool
    pub const m_flFogHeightWidth: usize = 0x548; // float32
    pub const m_flFogHeightEnd: usize = 0x54C; // float32
    pub const m_flFogHeightStart: usize = 0x550; // float32
    pub const m_flFogHeightExponent: usize = 0x554; // float32
    pub const m_flLODBias: usize = 0x558; // float32
    pub const m_bActive: usize = 0x55C; // bool
    pub const m_bStartDisabled: usize = 0x55D; // bool
    pub const m_flFogMaxOpacity: usize = 0x560; // float32
    pub const m_nCubemapSourceType: usize = 0x564; // int32
    pub const m_hSkyMaterial: usize = 0x568; // CStrongHandle< InfoForResourceTypeIMaterial2 >
    pub const m_iszSkyEntity: usize = 0x570; // CUtlSymbolLarge
    pub const m_hFogCubemapTexture: usize = 0x578; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_bHasHeightFogEnd: usize = 0x580; // bool
    pub const m_bFirstTime: usize = 0x581; // bool
}

pub mod C_EnvDecal {
    pub const m_hDecalMaterial: usize = 0x7B8; // CStrongHandle< InfoForResourceTypeIMaterial2 >
    pub const m_flWidth: usize = 0x7C0; // float32
    pub const m_flHeight: usize = 0x7C4; // float32
    pub const m_flDepth: usize = 0x7C8; // float32
    pub const m_nRenderOrder: usize = 0x7CC; // uint32
    pub const m_bProjectOnWorld: usize = 0x7D0; // bool
    pub const m_bProjectOnCharacters: usize = 0x7D1; // bool
    pub const m_bProjectOnWater: usize = 0x7D2; // bool
    pub const m_flDepthSortBias: usize = 0x7D4; // float32
}

pub mod C_EnvDetailController {
    pub const m_flFadeStartDist: usize = 0x538; // float32
    pub const m_flFadeEndDist: usize = 0x53C; // float32
}

pub mod C_EnvLightProbeVolume {
    pub const m_hLightProbeTexture: usize = 0x1518; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_hLightProbeDirectLightIndicesTexture: usize = 0x1520; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_hLightProbeDirectLightScalarsTexture: usize = 0x1528; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_hLightProbeDirectLightShadowsTexture: usize = 0x1530; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_vBoxMins: usize = 0x1538; // Vector
    pub const m_vBoxMaxs: usize = 0x1544; // Vector
    pub const m_LightGroups: usize = 0x1550; // CUtlSymbolLarge
    pub const m_bMoveable: usize = 0x1558; // bool
    pub const m_nHandshake: usize = 0x155C; // int32
    pub const m_nPriority: usize = 0x1560; // int32
    pub const m_bStartDisabled: usize = 0x1564; // bool
    pub const m_nLightProbeSizeX: usize = 0x1568; // int32
    pub const m_nLightProbeSizeY: usize = 0x156C; // int32
    pub const m_nLightProbeSizeZ: usize = 0x1570; // int32
    pub const m_nLightProbeAtlasX: usize = 0x1574; // int32
    pub const m_nLightProbeAtlasY: usize = 0x1578; // int32
    pub const m_nLightProbeAtlasZ: usize = 0x157C; // int32
    pub const m_bEnabled: usize = 0x1589; // bool
}

pub mod C_EnvParticleGlow {
    pub const m_flAlphaScale: usize = 0xD68; // float32
    pub const m_flRadiusScale: usize = 0xD6C; // float32
    pub const m_flSelfIllumScale: usize = 0xD70; // float32
    pub const m_ColorTint: usize = 0xD74; // Color
    pub const m_hTextureOverride: usize = 0xD78; // CStrongHandle< InfoForResourceTypeCTextureBase >
}

pub mod C_EnvScreenOverlay {
    pub const m_iszOverlayNames: usize = 0x538; // CUtlSymbolLarge[10]
    pub const m_flOverlayTimes: usize = 0x588; // float32[10]
    pub const m_flStartTime: usize = 0x5B0; // GameTime_t
    pub const m_iDesiredOverlay: usize = 0x5B4; // int32
    pub const m_bIsActive: usize = 0x5B8; // bool
    pub const m_bWasActive: usize = 0x5B9; // bool
    pub const m_iCachedDesiredOverlay: usize = 0x5BC; // int32
    pub const m_iCurrentOverlay: usize = 0x5C0; // int32
    pub const m_flCurrentOverlayTime: usize = 0x5C4; // GameTime_t
}

pub mod C_EnvSky {
    pub const m_hSkyMaterial: usize = 0x7B8; // CStrongHandle< InfoForResourceTypeIMaterial2 >
    pub const m_hSkyMaterialLightingOnly: usize = 0x7C0; // CStrongHandle< InfoForResourceTypeIMaterial2 >
    pub const m_bStartDisabled: usize = 0x7C8; // bool
    pub const m_vTintColor: usize = 0x7C9; // Color
    pub const m_vTintColorLightingOnly: usize = 0x7CD; // Color
    pub const m_flBrightnessScale: usize = 0x7D4; // float32
    pub const m_nFogType: usize = 0x7D8; // int32
    pub const m_flFogMinStart: usize = 0x7DC; // float32
    pub const m_flFogMinEnd: usize = 0x7E0; // float32
    pub const m_flFogMaxStart: usize = 0x7E4; // float32
    pub const m_flFogMaxEnd: usize = 0x7E8; // float32
    pub const m_bEnabled: usize = 0x7EC; // bool
}

pub mod C_EnvVolumetricFogController {
    pub const m_flScattering: usize = 0x538; // float32
    pub const m_flAnisotropy: usize = 0x53C; // float32
    pub const m_flFadeSpeed: usize = 0x540; // float32
    pub const m_flDrawDistance: usize = 0x544; // float32
    pub const m_flFadeInStart: usize = 0x548; // float32
    pub const m_flFadeInEnd: usize = 0x54C; // float32
    pub const m_flIndirectStrength: usize = 0x550; // float32
    pub const m_nIndirectTextureDimX: usize = 0x554; // int32
    pub const m_nIndirectTextureDimY: usize = 0x558; // int32
    pub const m_nIndirectTextureDimZ: usize = 0x55C; // int32
    pub const m_vBoxMins: usize = 0x560; // Vector
    pub const m_vBoxMaxs: usize = 0x56C; // Vector
    pub const m_bActive: usize = 0x578; // bool
    pub const m_flStartAnisoTime: usize = 0x57C; // GameTime_t
    pub const m_flStartScatterTime: usize = 0x580; // GameTime_t
    pub const m_flStartDrawDistanceTime: usize = 0x584; // GameTime_t
    pub const m_flStartAnisotropy: usize = 0x588; // float32
    pub const m_flStartScattering: usize = 0x58C; // float32
    pub const m_flStartDrawDistance: usize = 0x590; // float32
    pub const m_flDefaultAnisotropy: usize = 0x594; // float32
    pub const m_flDefaultScattering: usize = 0x598; // float32
    pub const m_flDefaultDrawDistance: usize = 0x59C; // float32
    pub const m_bStartDisabled: usize = 0x5A0; // bool
    pub const m_bEnableIndirect: usize = 0x5A1; // bool
    pub const m_bIsMaster: usize = 0x5A2; // bool
    pub const m_hFogIndirectTexture: usize = 0x5A8; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_nForceRefreshCount: usize = 0x5B0; // int32
    pub const m_bFirstTime: usize = 0x5B4; // bool
}

pub mod C_EnvVolumetricFogVolume {
    pub const m_bActive: usize = 0x538; // bool
    pub const m_vBoxMins: usize = 0x53C; // Vector
    pub const m_vBoxMaxs: usize = 0x548; // Vector
    pub const m_bStartDisabled: usize = 0x554; // bool
    pub const m_flStrength: usize = 0x558; // float32
    pub const m_nFalloffShape: usize = 0x55C; // int32
    pub const m_flFalloffExponent: usize = 0x560; // float32
}

pub mod C_EnvWind {
    pub const m_EnvWindShared: usize = 0x538; // C_EnvWindShared
}

pub mod C_EnvWindClientside {
    pub const m_EnvWindShared: usize = 0x538; // C_EnvWindShared
}

pub mod C_EnvWindShared {
    pub const m_flStartTime: usize = 0x8; // GameTime_t
    pub const m_iWindSeed: usize = 0xC; // uint32
    pub const m_iMinWind: usize = 0x10; // uint16
    pub const m_iMaxWind: usize = 0x12; // uint16
    pub const m_windRadius: usize = 0x14; // int32
    pub const m_iMinGust: usize = 0x18; // uint16
    pub const m_iMaxGust: usize = 0x1A; // uint16
    pub const m_flMinGustDelay: usize = 0x1C; // float32
    pub const m_flMaxGustDelay: usize = 0x20; // float32
    pub const m_flGustDuration: usize = 0x24; // float32
    pub const m_iGustDirChange: usize = 0x28; // uint16
    pub const m_location: usize = 0x2C; // Vector
    pub const m_iszGustSound: usize = 0x38; // int32
    pub const m_iWindDir: usize = 0x3C; // int32
    pub const m_flWindSpeed: usize = 0x40; // float32
    pub const m_currentWindVector: usize = 0x44; // Vector
    pub const m_CurrentSwayVector: usize = 0x50; // Vector
    pub const m_PrevSwayVector: usize = 0x5C; // Vector
    pub const m_iInitialWindDir: usize = 0x68; // uint16
    pub const m_flInitialWindSpeed: usize = 0x6C; // float32
    pub const m_flVariationTime: usize = 0x70; // GameTime_t
    pub const m_flSwayTime: usize = 0x74; // GameTime_t
    pub const m_flSimTime: usize = 0x78; // GameTime_t
    pub const m_flSwitchTime: usize = 0x7C; // GameTime_t
    pub const m_flAveWindSpeed: usize = 0x80; // float32
    pub const m_bGusting: usize = 0x84; // bool
    pub const m_flWindAngleVariation: usize = 0x88; // float32
    pub const m_flWindSpeedVariation: usize = 0x8C; // float32
    pub const m_iEntIndex: usize = 0x90; // CEntityIndex
}

pub mod C_EnvWindShared_WindAveEvent_t {
    pub const m_flStartWindSpeed: usize = 0x0; // float32
    pub const m_flAveWindSpeed: usize = 0x4; // float32
}

pub mod C_EnvWindShared_WindVariationEvent_t {
    pub const m_flWindAngleVariation: usize = 0x0; // float32
    pub const m_flWindSpeedVariation: usize = 0x4; // float32
}

pub mod C_FoWRevealerEntity {
    pub const m_unViewerTeam: usize = 0x538; // uint32
    pub const m_nVisionRange: usize = 0x53C; // int32
}

pub mod C_FogController {
    pub const m_fog: usize = 0x538; // fogparams_t
    pub const m_bUseAngles: usize = 0x5A0; // bool
    pub const m_iChangedVariables: usize = 0x5A4; // int32
}

pub mod C_FuncElectrifiedVolume {
    pub const m_nAmbientEffect: usize = 0x7B8; // ParticleIndex_t
    pub const m_EffectName: usize = 0x7C0; // CUtlSymbolLarge
    pub const m_bState: usize = 0x7C8; // bool
}

pub mod C_FuncTrackTrain {
    pub const m_nLongAxis: usize = 0x7B8; // int32
    pub const m_flRadius: usize = 0x7BC; // float32
    pub const m_flLineLength: usize = 0x7C0; // float32
}

pub mod C_GlobalLight {
    pub const m_WindClothForceHandle: usize = 0xA60; // uint16
}

pub mod C_InfoPlayerStartDota {
    pub const m_bDisabled: usize = 0x538; // bool
}

pub mod C_InfoVisibilityBox {
    pub const m_nMode: usize = 0x53C; // int32
    pub const m_vBoxSize: usize = 0x540; // Vector
    pub const m_bEnabled: usize = 0x54C; // bool
}

pub mod C_IngameEvent_Base {
    pub const m_bInitialized: usize = 0x548; // bool
    pub const m_CompendiumChallengeEventID: usize = 0x54C; // int32[24]
    pub const m_CompendiumChallengeSequenceID: usize = 0x5AC; // int32[24]
    pub const m_CompendiumChallengeCoinReward: usize = 0x60C; // int32[24]
    pub const m_CompendiumChallengeCoinSplash: usize = 0x66C; // int32[24]
    pub const m_CompendiumChallengePointReward: usize = 0x6CC; // int32[24]
    pub const m_CompendiumChallengeCompleted: usize = 0x72C; // bool[24]
    pub const m_CompendiumChallengeFailed: usize = 0x744; // bool[24]
    pub const m_CompendiumChallengeProgress: usize = 0x75C; // int32[24]
    pub const m_QueryIDForProgress: usize = 0x7BC; // int32[24]
    pub const m_SubChallenges: usize = 0x820; // C_UtlVectorEmbeddedNetworkVar< CDOTASubChallengeInfo >
    pub const m_CompendiumCoinWager: usize = 0x870; // int32[10]
    pub const m_CompendiumTokenWagerItemID: usize = 0x898; // itemid_t[10]
    pub const m_CompendiumWagerTokenBonusPct: usize = 0x8E8; // int32[10]
    pub const m_CompendiumCoinWagerResults: usize = 0x910; // int32[10]
    pub const m_CompendiumRankWagers: usize = 0x938; // int32[10]
    pub const m_flWagerTimer: usize = 0x960; // float32
    pub const m_flWagerEndTime: usize = 0x964; // GameTime_t
    pub const m_CompendiumChallengeInfo: usize = 0x968; // C_UtlVectorEmbeddedNetworkVar< CDOTA_PlayerChallengeInfo >
    pub const m_PlayerQueryIDs: usize = 0x1428; // C_UtlVectorEmbeddedNetworkVar< C_DOTA_CombatLogQueryProgress >
    pub const m_ProgressForQueryID: usize = 0x1478; // int32[100]
    pub const m_GoalForQueryID: usize = 0x1608; // int32[100]
    pub const m_PlayerQuestRankPreviouslyCompleted: usize = 0x1798; // int32[10]
    pub const m_PlayerQuestRankCompleted: usize = 0x17C0; // int32[10]
    pub const m_PlayerBountyTarget: usize = 0x17E8; // PlayerID_t[10]
    pub const m_flPlayerBountyTimestamp: usize = 0x1810; // GameTime_t[10]
    pub const m_PlayerBountyCount: usize = 0x1838; // int32[10]
    pub const m_PlayerBountyTimestamp_Obsolete: usize = 0x1860; // int32[24]
}

pub mod C_LightEntity {
    pub const m_CLightComponent: usize = 0x7B8; // CLightComponent*
}

pub mod C_LocalTempEntity {
    pub const flags: usize = 0x7F8; // int32
    pub const die: usize = 0x7FC; // GameTime_t
    pub const m_flFrameMax: usize = 0x800; // float32
    pub const x: usize = 0x804; // float32
    pub const y: usize = 0x808; // float32
    pub const fadeSpeed: usize = 0x80C; // float32
    pub const bounceFactor: usize = 0x810; // float32
    pub const hitSound: usize = 0x814; // int32
    pub const priority: usize = 0x818; // int32
    pub const tentOffset: usize = 0x81C; // Vector
    pub const m_vecTempEntAngVelocity: usize = 0x828; // QAngle
    pub const tempent_renderamt: usize = 0x834; // int32
    pub const m_vecNormal: usize = 0x838; // Vector
    pub const m_flSpriteScale: usize = 0x844; // float32
    pub const m_nFlickerFrame: usize = 0x848; // int32
    pub const m_flFrameRate: usize = 0x84C; // float32
    pub const m_flFrame: usize = 0x850; // float32
    pub const m_pszImpactEffect: usize = 0x858; // char*
    pub const m_pszParticleEffect: usize = 0x860; // char*
    pub const m_bParticleCollision: usize = 0x868; // bool
    pub const m_iLastCollisionFrame: usize = 0x86C; // int32
    pub const m_vLastCollisionOrigin: usize = 0x870; // Vector
    pub const m_vecTempEntVelocity: usize = 0x87C; // Vector
    pub const m_vecPrevAbsOrigin: usize = 0x888; // Vector
    pub const m_vecTempEntAcceleration: usize = 0x894; // Vector
}

pub mod C_NextBotCombatCharacter {
    pub const m_shadowTimer: usize = 0x9F8; // CountdownTimer
    pub const m_bInFrustum: usize = 0xA10; // bool
    pub const m_nInFrustumFrame: usize = 0xA14; // int32
    pub const m_flFrustumDistanceSqr: usize = 0xA18; // float32
    pub const m_nLod: usize = 0xA1C; // uint8
}

pub mod C_ParticleSystem {
    pub const m_szSnapshotFileName: usize = 0x7B8; // char[512]
    pub const m_bActive: usize = 0x9B8; // bool
    pub const m_bFrozen: usize = 0x9B9; // bool
    pub const m_flFreezeTransitionDuration: usize = 0x9BC; // float32
    pub const m_nStopType: usize = 0x9C0; // int32
    pub const m_bAnimateDuringGameplayPause: usize = 0x9C4; // bool
    pub const m_iEffectIndex: usize = 0x9C8; // CStrongHandle< InfoForResourceTypeIParticleSystemDefinition >
    pub const m_flStartTime: usize = 0x9D0; // GameTime_t
    pub const m_flPreSimTime: usize = 0x9D4; // float32
    pub const m_vServerControlPoints: usize = 0x9D8; // Vector[4]
    pub const m_iServerControlPointAssignments: usize = 0xA08; // uint8[4]
    pub const m_hControlPointEnts: usize = 0xA0C; // CHandle< C_BaseEntity >[64]
    pub const m_bNoSave: usize = 0xB0C; // bool
    pub const m_bNoFreeze: usize = 0xB0D; // bool
    pub const m_bNoRamp: usize = 0xB0E; // bool
    pub const m_bStartActive: usize = 0xB0F; // bool
    pub const m_iszEffectName: usize = 0xB10; // CUtlSymbolLarge
    pub const m_iszControlPointNames: usize = 0xB18; // CUtlSymbolLarge[64]
    pub const m_nDataCP: usize = 0xD18; // int32
    pub const m_vecDataCPValue: usize = 0xD1C; // Vector
    pub const m_nTintCP: usize = 0xD28; // int32
    pub const m_clrTint: usize = 0xD2C; // Color
    pub const m_bOldActive: usize = 0xD50; // bool
    pub const m_bOldFrozen: usize = 0xD51; // bool
}

pub mod C_PathParticleRope {
    pub const m_bStartActive: usize = 0x538; // bool
    pub const m_flMaxSimulationTime: usize = 0x53C; // float32
    pub const m_iszEffectName: usize = 0x540; // CUtlSymbolLarge
    pub const m_PathNodes_Name: usize = 0x548; // CUtlVector< CUtlSymbolLarge >
    pub const m_flParticleSpacing: usize = 0x560; // float32
    pub const m_flSlack: usize = 0x564; // float32
    pub const m_flRadius: usize = 0x568; // float32
    pub const m_ColorTint: usize = 0x56C; // Color
    pub const m_nEffectState: usize = 0x570; // int32
    pub const m_iEffectIndex: usize = 0x578; // CStrongHandle< InfoForResourceTypeIParticleSystemDefinition >
    pub const m_PathNodes_Position: usize = 0x580; // C_NetworkUtlVectorBase< Vector >
    pub const m_PathNodes_TangentIn: usize = 0x598; // C_NetworkUtlVectorBase< Vector >
    pub const m_PathNodes_TangentOut: usize = 0x5B0; // C_NetworkUtlVectorBase< Vector >
    pub const m_PathNodes_Color: usize = 0x5C8; // C_NetworkUtlVectorBase< Vector >
    pub const m_PathNodes_PinEnabled: usize = 0x5E0; // C_NetworkUtlVectorBase< bool >
    pub const m_PathNodes_RadiusScale: usize = 0x5F8; // C_NetworkUtlVectorBase< float32 >
}

pub mod C_PhysPropClientside {
    pub const m_flTouchDelta: usize = 0x930; // GameTime_t
    pub const m_fDeathTime: usize = 0x934; // GameTime_t
    pub const m_impactEnergyScale: usize = 0x938; // float32
    pub const m_inertiaScale: usize = 0x93C; // float32
    pub const m_flDmgModBullet: usize = 0x940; // float32
    pub const m_flDmgModClub: usize = 0x944; // float32
    pub const m_flDmgModExplosive: usize = 0x948; // float32
    pub const m_flDmgModFire: usize = 0x94C; // float32
    pub const m_iszPhysicsDamageTableName: usize = 0x950; // CUtlSymbolLarge
    pub const m_iszBasePropData: usize = 0x958; // CUtlSymbolLarge
    pub const m_iInteractions: usize = 0x960; // int32
    pub const m_bHasBreakPiecesOrCommands: usize = 0x964; // bool
    pub const m_vecDamagePosition: usize = 0x968; // Vector
    pub const m_vecDamageDirection: usize = 0x974; // Vector
    pub const m_nDamageType: usize = 0x980; // int32
}

pub mod C_PhysicsProp {
    pub const m_bAwake: usize = 0x930; // bool
}

pub mod C_PlayerCosmeticPropClientside {
    pub const m_iPlayerNum: usize = 0xA68; // int32
    pub const m_iCosmeticType: usize = 0xA6C; // int32
    pub const m_szProxyTextureName: usize = 0xA70; // char[260]
    pub const m_hProxyTexture: usize = 0xB78; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_bGeneratedShowcaseProps: usize = 0xC20; // bool
    pub const m_vecShowcaseProps: usize = 0xC28; // CUtlVector< C_PlayerCosmeticPropClientside* >
    pub const m_pShowcaseItem: usize = 0xC40; // C_EconItemView*
}

pub mod C_PlayerVisibility {
    pub const m_flVisibilityStrength: usize = 0x538; // float32
    pub const m_flFogDistanceMultiplier: usize = 0x53C; // float32
    pub const m_flFogMaxDensityMultiplier: usize = 0x540; // float32
    pub const m_flFadeTime: usize = 0x544; // float32
    pub const m_bStartDisabled: usize = 0x548; // bool
    pub const m_bIsEnabled: usize = 0x549; // bool
}

pub mod C_PointCamera {
    pub const m_FOV: usize = 0x538; // float32
    pub const m_Resolution: usize = 0x53C; // float32
    pub const m_bFogEnable: usize = 0x540; // bool
    pub const m_FogColor: usize = 0x541; // Color
    pub const m_flFogStart: usize = 0x548; // float32
    pub const m_flFogEnd: usize = 0x54C; // float32
    pub const m_flFogMaxDensity: usize = 0x550; // float32
    pub const m_bActive: usize = 0x554; // bool
    pub const m_bUseScreenAspectRatio: usize = 0x555; // bool
    pub const m_flAspectRatio: usize = 0x558; // float32
    pub const m_bNoSky: usize = 0x55C; // bool
    pub const m_fBrightness: usize = 0x560; // float32
    pub const m_flZFar: usize = 0x564; // float32
    pub const m_flZNear: usize = 0x568; // float32
    pub const m_bCanHLTVUse: usize = 0x56C; // bool
    pub const m_flOverrideShadowFarZ: usize = 0x570; // float32
    pub const m_bDofEnabled: usize = 0x574; // bool
    pub const m_flDofNearBlurry: usize = 0x578; // float32
    pub const m_flDofNearCrisp: usize = 0x57C; // float32
    pub const m_flDofFarCrisp: usize = 0x580; // float32
    pub const m_flDofFarBlurry: usize = 0x584; // float32
    pub const m_flDofTiltToGround: usize = 0x588; // float32
    pub const m_TargetFOV: usize = 0x58C; // float32
    pub const m_DegreesPerSecond: usize = 0x590; // float32
    pub const m_bIsOn: usize = 0x594; // bool
    pub const m_pNext: usize = 0x598; // C_PointCamera*
}

pub mod C_PointCameraVFOV {
    pub const m_flVerticalFOV: usize = 0x5A0; // float32
}

pub mod C_PointClientUIDialog {
    pub const m_hActivator: usize = 0x7E8; // CHandle< C_BaseEntity >
    pub const m_bStartEnabled: usize = 0x7EC; // bool
}

pub mod C_PointClientUIHUD {
    pub const m_bCheckCSSClasses: usize = 0x7F0; // bool
    pub const m_bIgnoreInput: usize = 0x978; // bool
    pub const m_flWidth: usize = 0x97C; // float32
    pub const m_flHeight: usize = 0x980; // float32
    pub const m_flDPI: usize = 0x984; // float32
    pub const m_flInteractDistance: usize = 0x988; // float32
    pub const m_flDepthOffset: usize = 0x98C; // float32
    pub const m_unOwnerContext: usize = 0x990; // uint32
    pub const m_unHorizontalAlign: usize = 0x994; // uint32
    pub const m_unVerticalAlign: usize = 0x998; // uint32
    pub const m_unOrientation: usize = 0x99C; // uint32
    pub const m_bAllowInteractionFromAllSceneWorlds: usize = 0x9A0; // bool
    pub const m_vecCSSClasses: usize = 0x9A8; // C_NetworkUtlVectorBase< CUtlSymbolLarge >
}

pub mod C_PointClientUIWorldPanel {
    pub const m_bForceRecreateNextUpdate: usize = 0x7F0; // bool
    pub const m_bMoveViewToPlayerNextThink: usize = 0x7F1; // bool
    pub const m_bCheckCSSClasses: usize = 0x7F2; // bool
    pub const m_anchorDeltaTransform: usize = 0x800; // CTransform
    pub const m_pOffScreenIndicator: usize = 0x9A0; // CPointOffScreenIndicatorUi*
    pub const m_bIgnoreInput: usize = 0x9C8; // bool
    pub const m_bLit: usize = 0x9C9; // bool
    pub const m_bFollowPlayerAcrossTeleport: usize = 0x9CA; // bool
    pub const m_flWidth: usize = 0x9CC; // float32
    pub const m_flHeight: usize = 0x9D0; // float32
    pub const m_flDPI: usize = 0x9D4; // float32
    pub const m_flInteractDistance: usize = 0x9D8; // float32
    pub const m_flDepthOffset: usize = 0x9DC; // float32
    pub const m_unOwnerContext: usize = 0x9E0; // uint32
    pub const m_unHorizontalAlign: usize = 0x9E4; // uint32
    pub const m_unVerticalAlign: usize = 0x9E8; // uint32
    pub const m_unOrientation: usize = 0x9EC; // uint32
    pub const m_bAllowInteractionFromAllSceneWorlds: usize = 0x9F0; // bool
    pub const m_vecCSSClasses: usize = 0x9F8; // C_NetworkUtlVectorBase< CUtlSymbolLarge >
    pub const m_bOpaque: usize = 0xA10; // bool
    pub const m_bNoDepth: usize = 0xA11; // bool
    pub const m_bRenderBackface: usize = 0xA12; // bool
    pub const m_bUseOffScreenIndicator: usize = 0xA13; // bool
    pub const m_bExcludeFromSaveGames: usize = 0xA14; // bool
    pub const m_bGrabbable: usize = 0xA15; // bool
    pub const m_bOnlyRenderToTexture: usize = 0xA16; // bool
    pub const m_bDisableMipGen: usize = 0xA17; // bool
    pub const m_nExplicitImageLayout: usize = 0xA18; // int32
}

pub mod C_PointClientUIWorldTextPanel {
    pub const m_messageText: usize = 0xA20; // char[512]
}

pub mod C_PointCommentaryNode {
    pub const m_bActive: usize = 0x7E8; // bool
    pub const m_bWasActive: usize = 0x7E9; // bool
    pub const m_flEndTime: usize = 0x7EC; // GameTime_t
    pub const m_flStartTime: usize = 0x7F0; // GameTime_t
    pub const m_flStartTimeInCommentary: usize = 0x7F4; // float32
    pub const m_iszCommentaryFile: usize = 0x7F8; // CUtlSymbolLarge
    pub const m_iszTitle: usize = 0x800; // CUtlSymbolLarge
    pub const m_iszSpeakers: usize = 0x808; // CUtlSymbolLarge
    pub const m_iNodeNumber: usize = 0x810; // int32
    pub const m_iNodeNumberMax: usize = 0x814; // int32
    pub const m_bListenedTo: usize = 0x818; // bool
    pub const m_hViewPosition: usize = 0x828; // CHandle< C_BaseEntity >
    pub const m_bRestartAfterRestore: usize = 0x82C; // bool
}

pub mod C_PointValueRemapper {
    pub const m_bDisabled: usize = 0x538; // bool
    pub const m_bDisabledOld: usize = 0x539; // bool
    pub const m_bUpdateOnClient: usize = 0x53A; // bool
    pub const m_nInputType: usize = 0x53C; // ValueRemapperInputType_t
    pub const m_hRemapLineStart: usize = 0x540; // CHandle< C_BaseEntity >
    pub const m_hRemapLineEnd: usize = 0x544; // CHandle< C_BaseEntity >
    pub const m_flMaximumChangePerSecond: usize = 0x548; // float32
    pub const m_flDisengageDistance: usize = 0x54C; // float32
    pub const m_flEngageDistance: usize = 0x550; // float32
    pub const m_bRequiresUseKey: usize = 0x554; // bool
    pub const m_nOutputType: usize = 0x558; // ValueRemapperOutputType_t
    pub const m_hOutputEntities: usize = 0x560; // C_NetworkUtlVectorBase< CHandle< C_BaseEntity > >
    pub const m_nHapticsType: usize = 0x578; // ValueRemapperHapticsType_t
    pub const m_nMomentumType: usize = 0x57C; // ValueRemapperMomentumType_t
    pub const m_flMomentumModifier: usize = 0x580; // float32
    pub const m_flSnapValue: usize = 0x584; // float32
    pub const m_flCurrentMomentum: usize = 0x588; // float32
    pub const m_nRatchetType: usize = 0x58C; // ValueRemapperRatchetType_t
    pub const m_flRatchetOffset: usize = 0x590; // float32
    pub const m_flInputOffset: usize = 0x594; // float32
    pub const m_bEngaged: usize = 0x598; // bool
    pub const m_bFirstUpdate: usize = 0x599; // bool
    pub const m_flPreviousValue: usize = 0x59C; // float32
    pub const m_flPreviousUpdateTickTime: usize = 0x5A0; // GameTime_t
    pub const m_vecPreviousTestPoint: usize = 0x5A4; // Vector
}

pub mod C_PointWorldText {
    pub const m_bForceRecreateNextUpdate: usize = 0x7C0; // bool
    pub const m_messageText: usize = 0x7D0; // char[512]
    pub const m_FontName: usize = 0x9D0; // char[64]
    pub const m_bEnabled: usize = 0xA10; // bool
    pub const m_bFullbright: usize = 0xA11; // bool
    pub const m_flWorldUnitsPerPx: usize = 0xA14; // float32
    pub const m_flFontSize: usize = 0xA18; // float32
    pub const m_flDepthOffset: usize = 0xA1C; // float32
    pub const m_Color: usize = 0xA20; // Color
    pub const m_nJustifyHorizontal: usize = 0xA24; // PointWorldTextJustifyHorizontal_t
    pub const m_nJustifyVertical: usize = 0xA28; // PointWorldTextJustifyVertical_t
    pub const m_nReorientMode: usize = 0xA2C; // PointWorldTextReorientMode_t
}

pub mod C_PortraitHero {
    pub const m_nHeroID: usize = 0x1750; // int32
    pub const m_actQueuedActivity: usize = 0x1754; // int32
    pub const m_szQueuedActivityModifier: usize = 0x1758; // char[64]
}

pub mod C_PortraitWorldLight {
    pub const m_flLatitude: usize = 0xA80; // float32
    pub const m_flLongitude: usize = 0xA84; // float32
    pub const m_flAdditionalRadius: usize = 0xA88; // float32
    pub const m_flBoundsRadiusMultiplier: usize = 0xA8C; // float32
    pub const m_hTarget: usize = 0xA90; // CHandle< C_PortraitWorldUnit >
    pub const m_bShowGizmos: usize = 0xA94; // bool
    pub const m_hitboxSetName: usize = 0xA98; // CUtlStringToken
    pub const m_vPreviousCenter: usize = 0xA9C; // Vector
    pub const m_vCenterVelocity: usize = 0xAA8; // Vector
    pub const m_flPreviousBoundingSphereRadius: usize = 0xAB4; // float32
    pub const m_flBoundingSphereRadiusChangeSpeed: usize = 0xAB8; // float32
    pub const m_bPreviousValuesInitialized: usize = 0xABC; // bool
}

pub mod C_PortraitWorldUnit {
    pub const m_bSuppressIntroEffects: usize = 0x1750; // bool
    pub const m_bIsAlternateLoadout: usize = 0x1751; // bool
    pub const m_bSpawnBackgroundModels: usize = 0x1752; // bool
    pub const m_bDeferredPortrait: usize = 0x1753; // bool
    pub const m_bShowParticleAssetModifiers: usize = 0x1754; // bool
    pub const m_bIgnorePortraitInfo: usize = 0x1755; // bool
    pub const m_bFlyingCourier: usize = 0x1756; // bool
    pub const m_bDisableDefaultModifiers: usize = 0x1757; // bool
    pub const m_nEffigyStatusEffect: usize = 0x1758; // int32
    pub const m_effigySequenceName: usize = 0x1760; // CUtlSymbolLarge
    pub const m_flStartingAnimationCycle: usize = 0x1768; // float32
    pub const m_flAnimationPlaybackSpeed: usize = 0x176C; // float32
    pub const m_flRareLoadoutAnimChance: usize = 0x1770; // float32
    pub const m_vecActivityModifiers: usize = 0x1778; // CUtlVector< CUtlSymbolLarge >
    pub const m_environment: usize = 0x1790; // DOTAPortraitEnvironmentType_t
    pub const m_nStartupBehavior: usize = 0x1798; // StartupBehavior_t
    pub const m_cameraName: usize = 0x1980; // CUtlSymbolLarge
    pub const m_nPortraitParticle: usize = 0x19D8; // ParticleIndex_t
    pub const m_nCourierType: usize = 0x19DC; // int32
}

pub mod C_PostProcessingVolume {
    pub const m_hPostSettings: usize = 0x7D0; // CStrongHandle< InfoForResourceTypeCPostProcessingResource >
    pub const m_flFadeDuration: usize = 0x7D8; // float32
    pub const m_flMinLogExposure: usize = 0x7DC; // float32
    pub const m_flMaxLogExposure: usize = 0x7E0; // float32
    pub const m_flMinExposure: usize = 0x7E4; // float32
    pub const m_flMaxExposure: usize = 0x7E8; // float32
    pub const m_flExposureCompensation: usize = 0x7EC; // float32
    pub const m_flExposureFadeSpeedUp: usize = 0x7F0; // float32
    pub const m_flExposureFadeSpeedDown: usize = 0x7F4; // float32
    pub const m_flTonemapEVSmoothingRange: usize = 0x7F8; // float32
    pub const m_bMaster: usize = 0x7FC; // bool
    pub const m_bExposureControl: usize = 0x7FD; // bool
}

pub mod C_RagdollManager {
    pub const m_iCurrentMaxRagdollCount: usize = 0x538; // int8
}

pub mod C_RagdollProp {
    pub const m_ragPos: usize = 0x980; // C_NetworkUtlVectorBase< Vector >
    pub const m_ragAngles: usize = 0x998; // C_NetworkUtlVectorBase< QAngle >
    pub const m_flBlendWeight: usize = 0x9B0; // float32
    pub const m_hRagdollSource: usize = 0x9B4; // CHandle< C_BaseEntity >
    pub const m_iEyeAttachment: usize = 0x9B8; // AttachmentHandle_t
    pub const m_flBlendWeightCurrent: usize = 0x9BC; // float32
    pub const m_parentPhysicsBoneIndices: usize = 0x9C0; // CUtlVector< int32 >
    pub const m_worldSpaceBoneComputationOrder: usize = 0x9D8; // CUtlVector< int32 >
}

pub mod C_RagdollPropAttached {
    pub const m_boneIndexAttached: usize = 0x9F0; // uint32
    pub const m_ragdollAttachedObjectIndex: usize = 0x9F4; // uint32
    pub const m_attachmentPointBoneSpace: usize = 0x9F8; // Vector
    pub const m_attachmentPointRagdollSpace: usize = 0xA04; // Vector
    pub const m_vecOffset: usize = 0xA10; // Vector
    pub const m_parentTime: usize = 0xA1C; // float32
    pub const m_bHasParent: usize = 0xA20; // bool
}

pub mod C_RopeKeyframe {
    pub const m_LinksTouchingSomething: usize = 0x7C0; // CBitVec< 10 >
    pub const m_nLinksTouchingSomething: usize = 0x7C4; // int32
    pub const m_bApplyWind: usize = 0x7C8; // bool
    pub const m_fPrevLockedPoints: usize = 0x7CC; // int32
    pub const m_iForcePointMoveCounter: usize = 0x7D0; // int32
    pub const m_bPrevEndPointPos: usize = 0x7D4; // bool[2]
    pub const m_vPrevEndPointPos: usize = 0x7D8; // Vector[2]
    pub const m_flCurScroll: usize = 0x7F0; // float32
    pub const m_flScrollSpeed: usize = 0x7F4; // float32
    pub const m_RopeFlags: usize = 0x7F8; // uint16
    pub const m_iRopeMaterialModelIndex: usize = 0x800; // CStrongHandle< InfoForResourceTypeIMaterial2 >
    pub const m_LightValues: usize = 0xA78; // Vector[10]
    pub const m_nSegments: usize = 0xAF0; // uint8
    pub const m_hStartPoint: usize = 0xAF4; // CHandle< C_BaseEntity >
    pub const m_hEndPoint: usize = 0xAF8; // CHandle< C_BaseEntity >
    pub const m_iStartAttachment: usize = 0xAFC; // AttachmentHandle_t
    pub const m_iEndAttachment: usize = 0xAFD; // AttachmentHandle_t
    pub const m_Subdiv: usize = 0xAFE; // uint8
    pub const m_RopeLength: usize = 0xB00; // int16
    pub const m_Slack: usize = 0xB02; // int16
    pub const m_TextureScale: usize = 0xB04; // float32
    pub const m_fLockedPoints: usize = 0xB08; // uint8
    pub const m_nChangeCount: usize = 0xB09; // uint8
    pub const m_Width: usize = 0xB0C; // float32
    pub const m_PhysicsDelegate: usize = 0xB10; // C_RopeKeyframe::CPhysicsDelegate
    pub const m_hMaterial: usize = 0xB20; // CStrongHandle< InfoForResourceTypeIMaterial2 >
    pub const m_TextureHeight: usize = 0xB28; // int32
    pub const m_vecImpulse: usize = 0xB2C; // Vector
    pub const m_vecPreviousImpulse: usize = 0xB38; // Vector
    pub const m_flCurrentGustTimer: usize = 0xB44; // float32
    pub const m_flCurrentGustLifetime: usize = 0xB48; // float32
    pub const m_flTimeToNextGust: usize = 0xB4C; // float32
    pub const m_vWindDir: usize = 0xB50; // Vector
    pub const m_vColorMod: usize = 0xB5C; // Vector
    pub const m_vCachedEndPointAttachmentPos: usize = 0xB68; // Vector[2]
    pub const m_vCachedEndPointAttachmentAngle: usize = 0xB80; // QAngle[2]
    pub const m_bConstrainBetweenEndpoints: usize = 0xB98; // bool
    pub const m_bEndPointAttachmentPositionsDirty: usize = 0x0; // bitfield:1
    pub const m_bEndPointAttachmentAnglesDirty: usize = 0x0; // bitfield:1
    pub const m_bNewDataThisFrame: usize = 0x0; // bitfield:1
    pub const m_bPhysicsInitted: usize = 0x0; // bitfield:1
}

pub mod C_RopeKeyframe_CPhysicsDelegate {
    pub const m_pKeyframe: usize = 0x8; // C_RopeKeyframe*
}

pub mod C_SceneEntity {
    pub const m_bIsPlayingBack: usize = 0x540; // bool
    pub const m_bPaused: usize = 0x541; // bool
    pub const m_bMultiplayer: usize = 0x542; // bool
    pub const m_bAutogenerated: usize = 0x543; // bool
    pub const m_flForceClientTime: usize = 0x544; // float32
    pub const m_nSceneStringIndex: usize = 0x548; // uint16
    pub const m_bClientOnly: usize = 0x54A; // bool
    pub const m_hOwner: usize = 0x54C; // CHandle< C_BaseFlex >
    pub const m_hActorList: usize = 0x550; // C_NetworkUtlVectorBase< CHandle< C_BaseFlex > >
    pub const m_bWasPlaying: usize = 0x568; // bool
    pub const m_QueuedEvents: usize = 0x578; // CUtlVector< C_SceneEntity::QueuedEvents_t >
    pub const m_flCurrentTime: usize = 0x590; // float32
}

pub mod C_SceneEntity_QueuedEvents_t {
    pub const starttime: usize = 0x0; // float32
}

pub mod C_SkyCamera {
    pub const m_skyboxData: usize = 0x538; // sky3dparams_t
    pub const m_skyboxSlotToken: usize = 0x5C8; // CUtlStringToken
    pub const m_bUseAngles: usize = 0x5CC; // bool
    pub const m_pNext: usize = 0x5D0; // C_SkyCamera*
}

pub mod C_SoundAreaEntityBase {
    pub const m_bDisabled: usize = 0x538; // bool
    pub const m_bWasEnabled: usize = 0x540; // bool
    pub const m_iszSoundAreaType: usize = 0x548; // CUtlSymbolLarge
    pub const m_vPos: usize = 0x550; // Vector
}

pub mod C_SoundAreaEntityOrientedBox {
    pub const m_vMin: usize = 0x560; // Vector
    pub const m_vMax: usize = 0x56C; // Vector
}

pub mod C_SoundAreaEntitySphere {
    pub const m_flRadius: usize = 0x560; // float32
}

pub mod C_SoundOpvarSetPointBase {
    pub const m_iszStackName: usize = 0x538; // CUtlSymbolLarge
    pub const m_iszOperatorName: usize = 0x540; // CUtlSymbolLarge
    pub const m_iszOpvarName: usize = 0x548; // CUtlSymbolLarge
    pub const m_iOpvarIndex: usize = 0x550; // int32
    pub const m_bUseAutoCompare: usize = 0x554; // bool
}

pub mod C_SpeechBubbleInfo {
    pub const m_LocalizationStr: usize = 0x30; // char[256]
    pub const m_hNPC: usize = 0x130; // CHandle< C_BaseEntity >
    pub const m_flStartTime: usize = 0x134; // GameTime_t
    pub const m_flDuration: usize = 0x138; // float32
    pub const m_unOffsetX: usize = 0x13C; // uint32
    pub const m_unOffsetY: usize = 0x140; // uint32
    pub const m_unCount: usize = 0x144; // uint16
}

pub mod C_SpeechBubbleManager {
    pub const m_SpeechBubbles: usize = 0x538; // C_UtlVectorEmbeddedNetworkVar< C_SpeechBubbleInfo >
    pub const m_nLastCountInQueue: usize = 0x12D8; // uint32[4]
}

pub mod C_Sprite {
    pub const m_hSpriteMaterial: usize = 0x7D0; // CStrongHandle< InfoForResourceTypeIMaterial2 >
    pub const m_hAttachedToEntity: usize = 0x7D8; // CHandle< C_BaseEntity >
    pub const m_nAttachment: usize = 0x7DC; // AttachmentHandle_t
    pub const m_flSpriteFramerate: usize = 0x7E0; // float32
    pub const m_flFrame: usize = 0x7E4; // float32
    pub const m_flDieTime: usize = 0x7E8; // GameTime_t
    pub const m_nBrightness: usize = 0x7F8; // uint32
    pub const m_flBrightnessDuration: usize = 0x7FC; // float32
    pub const m_flSpriteScale: usize = 0x800; // float32
    pub const m_flScaleDuration: usize = 0x804; // float32
    pub const m_bWorldSpaceScale: usize = 0x808; // bool
    pub const m_flGlowProxySize: usize = 0x80C; // float32
    pub const m_flHDRColorScale: usize = 0x810; // float32
    pub const m_flLastTime: usize = 0x814; // GameTime_t
    pub const m_flMaxFrame: usize = 0x818; // float32
    pub const m_flStartScale: usize = 0x81C; // float32
    pub const m_flDestScale: usize = 0x820; // float32
    pub const m_flScaleTimeStart: usize = 0x824; // GameTime_t
    pub const m_nStartBrightness: usize = 0x828; // int32
    pub const m_nDestBrightness: usize = 0x82C; // int32
    pub const m_flBrightnessTimeStart: usize = 0x830; // GameTime_t
    pub const m_hOldSpriteMaterial: usize = 0x838; // CWeakHandle< InfoForResourceTypeIMaterial2 >
    pub const m_nSpriteWidth: usize = 0x8E0; // int32
    pub const m_nSpriteHeight: usize = 0x8E4; // int32
}

pub mod C_Team {
    pub const m_aPlayerControllers: usize = 0x538; // C_NetworkUtlVectorBase< CHandle< CBasePlayerController > >
    pub const m_aPlayers: usize = 0x550; // C_NetworkUtlVectorBase< CHandle< C_BasePlayerPawn > >
    pub const m_iScore: usize = 0x568; // int32
    pub const m_szTeamname: usize = 0x56C; // char[129]
}

pub mod C_TextureBasedAnimatable {
    pub const m_bLoop: usize = 0x7B8; // bool
    pub const m_flFPS: usize = 0x7BC; // float32
    pub const m_hPositionKeys: usize = 0x7C0; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_hRotationKeys: usize = 0x7C8; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_vAnimationBoundsMin: usize = 0x7D0; // Vector
    pub const m_vAnimationBoundsMax: usize = 0x7DC; // Vector
    pub const m_flStartTime: usize = 0x7E8; // float32
    pub const m_flStartFrame: usize = 0x7EC; // float32
}

pub mod C_TonemapController2 {
    pub const m_flAutoExposureMin: usize = 0x538; // float32
    pub const m_flAutoExposureMax: usize = 0x53C; // float32
    pub const m_flTonemapPercentTarget: usize = 0x540; // float32
    pub const m_flTonemapPercentBrightPixels: usize = 0x544; // float32
    pub const m_flTonemapMinAvgLum: usize = 0x548; // float32
    pub const m_flExposureAdaptationSpeedUp: usize = 0x54C; // float32
    pub const m_flExposureAdaptationSpeedDown: usize = 0x550; // float32
    pub const m_flTonemapEVSmoothingRange: usize = 0x554; // float32
}

pub mod C_World {
    pub const m_skyBoxFaces: usize = 0x7B8; // CStrongHandle< InfoForResourceTypeIMaterial2 >[6]
    pub const m_hHeightFogTexture: usize = 0x7E8; // CStrongHandle< InfoForResourceTypeCTextureBase >
    pub const m_hHeightFogMaskTexture: usize = 0x7F0; // CStrongHandle< InfoForResourceTypeCTextureBase >
}

pub mod C_fogplayerparams_t {
    pub const m_hCtrl: usize = 0x8; // CHandle< C_FogController >
    pub const m_flTransitionTime: usize = 0xC; // float32
    pub const m_OldColor: usize = 0x10; // Color
    pub const m_flOldStart: usize = 0x14; // float32
    pub const m_flOldEnd: usize = 0x18; // float32
    pub const m_flOldMaxDensity: usize = 0x1C; // float32
    pub const m_flOldHDRColorScale: usize = 0x20; // float32
    pub const m_flOldFarZ: usize = 0x24; // float32
    pub const m_NewColor: usize = 0x28; // Color
    pub const m_flNewStart: usize = 0x2C; // float32
    pub const m_flNewEnd: usize = 0x30; // float32
    pub const m_flNewMaxDensity: usize = 0x34; // float32
    pub const m_flNewHDRColorScale: usize = 0x38; // float32
    pub const m_flNewFarZ: usize = 0x3C; // float32
}

pub mod CandyShopCandyType_t {
    pub const m_unCandyTypeID: usize = 0x0; // CandyShopCandyTypeID_t
    pub const m_sLocName: usize = 0x8; // CUtlString
    pub const m_sImage: usize = 0x10; // CPanoramaImageName
}

pub mod CandyShopRewardOption_t {
    pub const m_unRewardOptionID: usize = 0x0; // CandyShopRewardOptionID_t
    pub const m_sRewardClass: usize = 0x8; // CUtlString
    pub const m_unRewardOptionMaxCount: usize = 0x10; // uint32
    pub const m_unWeight: usize = 0x14; // uint32
    pub const m_unCandyPrice: usize = 0x18; // uint32
    pub const m_eOptionType: usize = 0x1C; // ECandyShopRewardOptionType
    pub const m_unSingleItemDef: usize = 0x20; // item_definition_index_t
    pub const m_sLootList: usize = 0x28; // CUtlString
    pub const m_eEvent: usize = 0x30; // EEvent
    pub const m_unEventActionID: usize = 0x34; // uint32
    pub const m_unEventPoints: usize = 0x38; // uint32
}

pub mod CandyShopRewardSlot_t {
    pub const m_sSlotClass: usize = 0x0; // CUtlString
    pub const m_vecSlotRewardOptions: usize = 0x8; // CUtlVector< CandyShopRewardOption_t >
}

pub mod ClientQuickBuyItemState {
    pub const nItemType: usize = 0x30; // int16
    pub const bPurchasable: usize = 0x32; // bool
}

pub mod CountdownTimer {
    pub const m_duration: usize = 0x8; // float32
    pub const m_timestamp: usize = 0xC; // GameTime_t
    pub const m_timescale: usize = 0x10; // float32
    pub const m_nWorldGroupId: usize = 0x14; // WorldGroupId_t
}

pub mod DOTAThreatLevelInfo_t {
    pub const m_flKillability: usize = 0x30; // float32
    pub const m_nEntIndex: usize = 0x34; // int32
}

pub mod DOTA_AssassinMinigameNetworkState {
    pub const nAssassinState: usize = 0x8; // uint16
    pub const nVictimHeroID: usize = 0xA; // uint16
}

pub mod DataTeamPlayer_t {
    pub const m_iReliableGold: usize = 0x30; // int32
    pub const m_iUnreliableGold: usize = 0x34; // int32
    pub const m_iStartingPosition: usize = 0x38; // int32
    pub const m_iTotalEarnedGold: usize = 0x3C; // int32
    pub const m_iTotalEarnedXP: usize = 0x40; // int32
    pub const m_iSharedGold: usize = 0x44; // int32
    pub const m_iHeroKillGold: usize = 0x48; // int32
    pub const m_iCreepKillGold: usize = 0x4C; // int32
    pub const m_iNeutralKillGold: usize = 0x50; // int32
    pub const m_iCourierGold: usize = 0x54; // int32
    pub const m_iBountyGold: usize = 0x58; // int32
    pub const m_iRoshanGold: usize = 0x5C; // int32
    pub const m_iBuildingGold: usize = 0x60; // int32
    pub const m_iOtherGold: usize = 0x64; // int32
    pub const m_iComebackGold: usize = 0x68; // int32
    pub const m_iExperimentalGold: usize = 0x6C; // int32
    pub const m_iExperimental2Gold: usize = 0x70; // int32
    pub const m_iCreepDenyGold: usize = 0x74; // int32
    pub const m_iTPScrollsPurchased: usize = 0x78; // int32
    pub const m_flCustomStats: usize = 0x7C; // float32
    pub const m_iIncomeGold: usize = 0x80; // int32
    pub const m_iWardKillGold: usize = 0x84; // int32
    pub const m_iAbilityGold: usize = 0x88; // int32
    pub const m_iNetWorth: usize = 0x8C; // int32
    pub const m_iDenyCount: usize = 0x90; // int32
    pub const m_iLastHitCount: usize = 0x94; // int32
    pub const m_iLastHitStreak: usize = 0x98; // int32
    pub const m_iLastHitMultikill: usize = 0x9C; // int32
    pub const m_iNearbyCreepDeathCount: usize = 0xA0; // int32
    pub const m_iClaimedDenyCount: usize = 0xA4; // int32
    pub const m_iClaimedMissCount: usize = 0xA8; // int32
    pub const m_iMissCount: usize = 0xAC; // int32
    pub const m_nPossibleHeroSelection: usize = 0xB0; // int32
    pub const m_iMetaLevel: usize = 0xB4; // uint16
    pub const m_iMetaExperience: usize = 0xB6; // uint16
    pub const m_iMetaExperienceAwarded: usize = 0xB8; // uint16
    pub const m_flBuybackCooldownTime: usize = 0xBC; // GameTime_t
    pub const m_flBuybackGoldLimitTime: usize = 0xC0; // GameTime_t
    pub const m_flBuybackCostTime: usize = 0xC4; // float32
    pub const m_flCustomBuybackCooldown: usize = 0xC8; // float32
    pub const m_fStuns: usize = 0xCC; // float32
    pub const m_fHealing: usize = 0xD0; // float32
    pub const m_fRegeneratedHealth: usize = 0xD4; // float32
    pub const m_iTowerKills: usize = 0xD8; // int32
    pub const m_iRoshanKills: usize = 0xDC; // int32
    pub const m_hCameraTarget: usize = 0xE0; // CHandle< C_BaseEntity >
    pub const m_hOverrideSelectionEntity: usize = 0xE4; // CHandle< C_BaseEntity >
    pub const m_iObserverWardsPlaced: usize = 0xE8; // int32
    pub const m_iSentryWardsPlaced: usize = 0xEC; // int32
    pub const m_iCreepsStacked: usize = 0xF0; // int32
    pub const m_iCampsStacked: usize = 0xF4; // int32
    pub const m_iRunePickups: usize = 0xF8; // int32
    pub const m_iGoldSpentOnSupport: usize = 0xFC; // int32
    pub const m_iHeroDamage: usize = 0x100; // int32
    pub const m_iTowerDamage: usize = 0x104; // int32
    pub const m_iWardsPurchased: usize = 0x108; // int32
    pub const m_iWardsDestroyed: usize = 0x10C; // int32
    pub const m_PreGameInventory: usize = 0x110; // C_DOTA_UnitInventory
    pub const m_nKillsPerOpposingTeamMember: usize = 0x1C0; // int32[24]
    pub const m_iSuggestedAbilities: usize = 0x220; // AbilityID_t[5]
    pub const m_fSuggestedAbilityWeights: usize = 0x234; // float32[5]
    pub const m_iSuggestedPregameItems: usize = 0x248; // AbilityID_t[15]
    pub const m_iSuggestedItemSequences: usize = 0x284; // AbilityID_t[30]
    pub const m_iSuggestedWeightedItems: usize = 0x2FC; // WeightedAbilitySuggestion_t[15]
    pub const m_iSuggestedTopNItems: usize = 0x374; // WeightedAbilitySuggestion_t[3]
    pub const m_iSuggestedNeutralItems: usize = 0x38C; // WeightedAbilitySuggestion_t[25]
    pub const m_iSuggestedHeroes: usize = 0x454; // uint32[10]
    pub const m_flSuggestedHeroesWeights: usize = 0x47C; // float32[10]
    pub const m_iDamageByTypeReceivedPreReduction: usize = 0x4A4; // int32[3]
    pub const m_iDamageByTypeReceivedPostReduction: usize = 0x4B0; // int32[3]
    pub const m_iOutgoingDamageByTypePreReduction: usize = 0x4BC; // int32[3]
    pub const m_iOutgoingDamageByTypePostReduction: usize = 0x4C8; // int32[3]
    pub const m_iCommandsIssued: usize = 0x4D4; // int32
    pub const m_iGoldSpentOnConsumables: usize = 0x4D8; // int32
    pub const m_iGoldSpentOnItems: usize = 0x4DC; // int32
    pub const m_iGoldSpentOnBuybacks: usize = 0x4E0; // int32
    pub const m_iGoldLostToDeath: usize = 0x4E4; // int32
    pub const m_bIsNewPlayer: usize = 0x4E8; // bool
    pub const m_bIsGuidePlayer: usize = 0x4E9; // bool
    pub const m_iPlayerSteamID: usize = 0x4F0; // uint64
    pub const m_iSmokesUsed: usize = 0x4F8; // int32
    pub const m_iNeutralTokensFound: usize = 0x4FC; // int32
    pub const m_iWatchersTaken: usize = 0x500; // int32
    pub const m_iLotusesTaken: usize = 0x504; // int32
    pub const m_iTormentorKills: usize = 0x508; // int32
    pub const m_iCourierKills: usize = 0x50C; // int32
}

pub mod EngineCountdownTimer {
    pub const m_duration: usize = 0x8; // float32
    pub const m_timestamp: usize = 0xC; // float32
    pub const m_timescale: usize = 0x10; // float32
}

pub mod EntityRenderAttribute_t {
    pub const m_ID: usize = 0x30; // CUtlStringToken
    pub const m_Values: usize = 0x34; // Vector4D
}

pub mod FowBlocker_t {
    pub const m_flMinX: usize = 0x30; // float32
    pub const m_flMaxX: usize = 0x34; // float32
    pub const m_flMinY: usize = 0x38; // float32
    pub const m_flMaxY: usize = 0x3C; // float32
    pub const m_fowBlockerShape: usize = 0x40; // FowBlockerShape_t
    pub const m_bClearRegion: usize = 0x44; // bool
}

pub mod InGamePredictionData_t {
    pub const m_nID: usize = 0x28; // uint32
    pub const m_nValue: usize = 0x2C; // uint32
    pub const m_nRawValue: usize = 0x30; // uint32
    pub const m_nValueState: usize = 0x34; // uint8
    pub const m_bValueIsMask: usize = 0x35; // bool
}

pub mod IntervalTimer {
    pub const m_timestamp: usize = 0x8; // GameTime_t
    pub const m_nWorldGroupId: usize = 0xC; // WorldGroupId_t
}

pub mod Modifier_Pluck_Famango_Channel {
    pub const famango_rate: usize = 0x12C8; // float32
    pub const m_hMangoTree: usize = 0x12CC; // CHandle< C_BaseEntity >
}

pub mod NagaSiren_SongOfTheSiren_Healing {
    pub const heal_pct: usize = 0x12C8; // int32
}

pub mod NeutralCampStackPullAlarm_t {
    pub const m_vPosition: usize = 0x0; // Vector
    pub const m_flPullAlarmTime: usize = 0xC; // float32
    pub const m_flStackAlarmTime: usize = 0x10; // float32
    pub const m_nCampIndex: usize = 0x14; // int32
    pub const m_nAlarmTypes: usize = 0x18; // int32
}

pub mod PhysicsRagdollPose_t {
    pub const __m_pChainEntity: usize = 0x8; // CNetworkVarChainer
    pub const m_Transforms: usize = 0x30; // C_NetworkUtlVectorBase< CTransform >
    pub const m_hOwner: usize = 0x48; // CHandle< C_BaseEntity >
    pub const m_bDirty: usize = 0x68; // bool
}

pub mod PingConfirmationState_t {
    pub const m_nInitiatingPlayerID: usize = 0x30; // int32
    pub const m_PingWheelMessageID: usize = 0x34; // int32
    pub const m_flActiveUntilTime: usize = 0x38; // float32
    pub const m_vLocation: usize = 0x3C; // Vector2D
    pub const m_nPingedEntityEntIndex: usize = 0x44; // int32
    pub const m_nID: usize = 0x48; // int32
    pub const m_IconType: usize = 0x4C; // PingConfirmationIconType
    pub const m_nAgreeState: usize = 0x50; // int32[24]
}

pub mod PlayerResourceBroadcasterData_t {
    pub const m_iszBroadcasterChannelDescription: usize = 0x30; // CUtlSymbolLarge
    pub const m_iszBroadcasterChannelCountryCode: usize = 0x38; // CUtlSymbolLarge
    pub const m_iszBroadcasterChannelLanguageCode: usize = 0x40; // CUtlSymbolLarge
}

pub mod PlayerResourcePlayerData_t {
    pub const m_bIsValid: usize = 0x30; // bool
    pub const m_iszPlayerName: usize = 0x38; // CUtlSymbolLarge
    pub const m_iPlayerTeam: usize = 0x40; // int32
    pub const m_bFullyJoinedServer: usize = 0x44; // bool
    pub const m_bFakeClient: usize = 0x45; // bool
    pub const m_bIsBroadcaster: usize = 0x46; // bool
    pub const m_iBroadcasterChannel: usize = 0x48; // uint32
    pub const m_iBroadcasterChannelSlot: usize = 0x4C; // uint32
    pub const m_bIsBroadcasterChannelCameraman: usize = 0x50; // bool
    pub const m_iConnectionState: usize = 0x54; // int32
    pub const m_iPlayerSteamID: usize = 0x58; // uint64
    pub const m_eCoachTeam: usize = 0x60; // DOTATeam_t
    pub const m_vecPrivateCoachPlayerIDs: usize = 0x68; // C_NetworkUtlVectorBase< PlayerID_t >
    pub const m_unCoachRating: usize = 0x80; // uint32
    pub const m_eLiveSpectatorTeam: usize = 0x84; // DOTATeam_t
    pub const m_nLiveSpectatorSpectatedHeroIndex: usize = 0x88; // int32
    pub const m_bIsPlusSubscriber: usize = 0x8C; // bool
    pub const m_bWasMVPLastGame: usize = 0x8D; // bool
    pub const m_nCavernCrawlMapVariant: usize = 0x8E; // CavernCrawlMapVariant_t
    pub const m_eAccoladeType: usize = 0x90; // int32[3]
    pub const m_unAccoladeData: usize = 0xA0; // uint64[3]
    pub const m_iRankTier: usize = 0xB8; // int32
    pub const m_iTitle: usize = 0xBC; // int32
    pub const m_unFavTeamPacked: usize = 0xC0; // uint64
    pub const m_nPlayerSlot: usize = 0xC8; // CPlayerSlot
    pub const m_bIsBot: usize = 0xCC; // bool
    pub const m_bHasNeutralTier: usize = 0xCD; // bool[5]
    pub const m_bHasRedeemedNeutralTier: usize = 0xD2; // bool[5]
    pub const m_nCommLevel: usize = 0xD8; // int32
    pub const m_nBehaviorLevel: usize = 0xDC; // int32
    pub const m_flLastCommsTime: usize = 0xE0; // float32
}

pub mod PlayerResourcePlayerEventData_t {
    pub const m_iEventID: usize = 0x30; // uint32
    pub const m_iEventPoints: usize = 0x34; // uint32
    pub const m_iEventPremiumPoints: usize = 0x38; // uint32
    pub const m_iEventEffectsMask: usize = 0x3C; // uint32
    pub const m_iEventRanks: usize = 0x40; // uint16
    pub const m_bIsEventOwned: usize = 0x42; // bool
    pub const m_iFavoriteTeam: usize = 0x44; // uint32
    pub const m_iFavoriteTeamQuality: usize = 0x48; // uint16
    pub const m_iAvailableSalutes: usize = 0x4A; // uint8
    pub const m_iSaluteAmountIndex: usize = 0x4B; // uint8
    pub const m_iEventWagerStreak: usize = 0x4C; // uint32
    pub const m_iEventTeleportFXLevel: usize = 0x50; // uint8
    pub const m_nCandyPointsReason: usize = 0x54; // int32[5]
    pub const m_vecPeriodicResources: usize = 0x68; // C_UtlVectorEmbeddedNetworkVar< PlayerResourcePlayerPeriodicResourceData_t >
    pub const m_iObsoleteSaluteAmounts: usize = 0xB8; // uint8
    pub const m_iObsoleteEventArcanaPeriodicResourceRemaining: usize = 0xBC; // uint32
    pub const m_iObsoleteEventArcanaPeriodicResourceMax: usize = 0xC0; // uint32
    pub const m_iObsoleteEventWagerTokensRemaining: usize = 0xC4; // uint32
    pub const m_iObsoleteEventWagerTokensMax: usize = 0xC8; // uint32
    pub const m_iObsoleteEventBountiesRemaining: usize = 0xCC; // uint32
    pub const m_iObsoleteRankWagersAvailable: usize = 0xD0; // uint32
    pub const m_iObsoleteRankWagersMax: usize = 0xD4; // uint32
    pub const m_iObsoleteEventPointAdjustmentsRemaining: usize = 0xD8; // uint32
}

pub mod PlayerResourcePlayerPeriodicResourceData_t {
    pub const m_unPeriodicResourceID: usize = 0x30; // PeriodicResourceID_t
    pub const m_unPeriodicResourceMax: usize = 0x34; // uint32
    pub const m_unPeriodicResourceRemaining: usize = 0x38; // uint32
}

pub mod PlayerResourcePlayerTeamData_t {
    pub const m_ThreatLevelInfos: usize = 0x30; // C_UtlVectorEmbeddedNetworkVar< DOTAThreatLevelInfo_t >
    pub const m_nSelectedHeroID: usize = 0x80; // int32
    pub const m_iKills: usize = 0x84; // int32
    pub const m_iAssists: usize = 0x88; // int32
    pub const m_iDeaths: usize = 0x8C; // int32
    pub const m_iStreak: usize = 0x90; // int32
    pub const m_iLevel: usize = 0x94; // int32
    pub const m_iCustomIntParam: usize = 0x98; // int32
    pub const m_iRespawnSeconds: usize = 0x9C; // int32
    pub const m_flLastBuybackTime: usize = 0xA0; // GameTime_t
    pub const m_iLastBuybackTime_Obsolete: usize = 0xA4; // int32
    pub const m_hSelectedHero: usize = 0xA8; // CHandle< C_BaseEntity >
    pub const m_bAFK: usize = 0xAC; // bool
    pub const m_nSuggestedHeroes: usize = 0xB0; // int32[4]
    pub const m_bBanSuggestedHeroes: usize = 0xC0; // bool[4]
    pub const m_iTimedRewardDrops: usize = 0xC4; // item_definition_index_t
    pub const m_iTimedRewardDropOrigins: usize = 0xC8; // int32
    pub const m_iTimedRewardCrates: usize = 0xCC; // item_definition_index_t
    pub const m_iTimedRewardEvents: usize = 0xD0; // item_definition_index_t
    pub const m_unCompendiumLevel: usize = 0xD4; // uint16
    pub const m_bCanRepick: usize = 0xD6; // bool
    pub const m_bCanEarnRewards: usize = 0xD7; // bool
    pub const m_bHasRandomed: usize = 0xD8; // bool
    pub const m_nRandomedHeroID: usize = 0xDC; // int32
    pub const m_bBattleBonusActive: usize = 0xE0; // bool
    pub const m_iBattleBonusRate: usize = 0xE2; // uint16
    pub const m_iCustomBuybackCost: usize = 0xE4; // int32
    pub const m_CustomPlayerColor: usize = 0xE8; // Color
    pub const m_bQualifiesForPAContractReward: usize = 0xEC; // bool
    pub const m_bHasPredictedVictory: usize = 0xED; // bool
    pub const m_UnitShareMasks: usize = 0xF0; // int32
    pub const m_iTeamSlot: usize = 0xF4; // int32
    pub const m_iBattleCupWinStreak: usize = 0xF8; // uint8
    pub const m_iBattleCupWinDate: usize = 0x100; // uint64
    pub const m_iBattleCupSkillLevel: usize = 0x108; // uint16
    pub const m_iBattleCupTeamID: usize = 0x10C; // uint32
    pub const m_iBattleCupTournamentID: usize = 0x110; // uint32
    pub const m_iBattleCupDivision: usize = 0x114; // uint8
    pub const m_flTeamFightParticipation: usize = 0x118; // float32
    pub const m_iFirstBloodClaimed: usize = 0x11C; // int32
    pub const m_iFirstBloodGiven: usize = 0x120; // int32
    pub const m_unPickOrder: usize = 0x124; // uint32
    pub const m_flTimeOfLastSaluteSent: usize = 0x128; // GameTime_t
    pub const m_vecPlayerEventData: usize = 0x130; // C_UtlVectorEmbeddedNetworkVar< PlayerResourcePlayerEventData_t >
    pub const m_unSelectedHeroBadgeXP: usize = 0x180; // uint32
    pub const m_iBountyRunes: usize = 0x184; // uint8
    pub const m_iPowerRunes: usize = 0x185; // uint8
    pub const m_iWaterRunes: usize = 0x186; // uint8
    pub const m_iOutpostsCaptured: usize = 0x187; // uint8
    pub const m_unGuildTier: usize = 0x188; // uint8
    pub const m_unGuildLevel: usize = 0x18A; // uint16
    pub const m_unGuildPrimaryColor: usize = 0x18C; // uint8
    pub const m_unGuildSecondaryColor: usize = 0x18D; // uint8
    pub const m_unGuildPattern: usize = 0x18E; // uint8
    pub const m_unGuildLogo: usize = 0x190; // uint64
    pub const m_unGuildFlags: usize = 0x198; // uint32
    pub const m_bIsPartyGuild: usize = 0x19C; // bool
    pub const m_unGuildID: usize = 0x1A0; // GuildID_t
    pub const m_unHeroStickerDefIndex: usize = 0x1A4; // item_definition_index_t
    pub const m_eHeroStickerQuality: usize = 0x1A8; // uint8
    pub const m_eLaneSelectionFlags: usize = 0x1A9; // uint8
    pub const m_nPlayerDraftPreferredRoles: usize = 0x1AA; // uint8
    pub const m_nPlayerDraftPreferredTeam: usize = 0x1AB; // int8
    pub const m_iObsoleteEventPoints: usize = 0x1AC; // uint32
    pub const m_iObsoleteEventPremiumPoints: usize = 0x1B0; // uint32
    pub const m_iObsoleteEventWagerTokensRemaining: usize = 0x1B4; // uint32
    pub const m_iObsoleteEventWagerTokensMax: usize = 0x1B8; // uint32
    pub const m_iObsoleteEventEffectsMask: usize = 0x1BC; // uint32
    pub const m_iObsoleteEventRanks: usize = 0x1C0; // uint16
    pub const m_bObsoleteIsEventOwned: usize = 0x1C2; // bool
    pub const m_iObsoleteRankWagersAvailable: usize = 0x1C4; // uint32
    pub const m_iObsoleteRankWagersMax: usize = 0x1C8; // uint32
    pub const m_iObsoleteEventPointAdjustmentsRemaining: usize = 0x1CC; // uint32
    pub const m_iObsoleteAvailableSalutes: usize = 0x1D0; // uint32
    pub const m_iObsoleteSaluteAmounts: usize = 0x1D4; // uint32
}

pub mod PlayerSeatAssignment_t {
    pub const unAccountID: usize = 0x0; // uint32
    pub const unSeat: usize = 0x4; // uint32
    pub const unReversedSeat: usize = 0x8; // uint32
    pub const unTeamID: usize = 0xC; // uint32
}

pub mod PortraitWorldLightConfig_t {
    pub const m_strName: usize = 0x0; // CUtlString
    pub const m_flLatitude: usize = 0x8; // float32
    pub const m_flLongitude: usize = 0xC; // float32
    pub const m_flIntensity: usize = 0x10; // float32
    pub const m_flAdditionalRadius: usize = 0x14; // float32
    pub const m_flSpotFov: usize = 0x18; // float32
    pub const m_flAnimatedNoiseFrequency: usize = 0x1C; // float32
    pub const m_flAnimatedNoiseMinValue: usize = 0x20; // float32
    pub const m_color: usize = 0x24; // Color
    pub const m_bShowGizmos: usize = 0x28; // bool
    pub const m_bShadows: usize = 0x29; // bool
}

pub mod RoadToTIQuestDefinition_t {
    pub const m_unID: usize = 0x0; // RoadToTIQuestID_t
    pub const m_eQuestType: usize = 0x4; // ERoadToTIQuestType
    pub const m_unPeriod: usize = 0x8; // uint32
    pub const m_unMatchID: usize = 0x10; // MatchID_t
    pub const m_unSeriesID: usize = 0x18; // uint32
    pub const m_unLeagueID: usize = 0x1C; // uint32
    pub const m_unPlayerID: usize = 0x20; // uint32
    pub const m_unTeamID: usize = 0x24; // uint32
    pub const m_vecHeroes: usize = 0x28; // CUtlVector< uint8 >
    pub const m_bDeveloper: usize = 0x40; // bool
}

pub mod SZooSetAnnotation_t {
    pub const m_strSetTag: usize = 0x0; // CUtlString
}

pub mod SZooSetAnnotationsConfig_t {
    pub const m_allAnnotations: usize = 0x0; // CUtlVector< SZooSetAnnotations_t >
}

pub mod SZooSetAnnotations_t {
    pub const m_strSetName: usize = 0x0; // CUtlString
    pub const m_annotations: usize = 0x8; // CUtlVector< SZooSetAnnotation_t >
}

pub mod TempViewerInfo_t {
    pub const m_nGridX: usize = 0x30; // int32
    pub const m_nGridY: usize = 0x34; // int32
    pub const m_nRadius: usize = 0x38; // int32
    pub const m_nViewerType: usize = 0x3C; // int8
    pub const m_bObstructedVision: usize = 0x3D; // bool
    pub const m_bValid: usize = 0x3E; // bool
    pub const m_bDirty: usize = 0x3F; // bool
    pub const flEndTime: usize = 0x40; // GameTime_t
    pub const nFoWID: usize = 0x44; // int32
    pub const hOwner: usize = 0x48; // CHandle< C_BaseEntity >
}

pub mod TierNeutralInfo_t {
    pub const m_nTier: usize = 0x30; // int32
    pub const m_nAbilityID: usize = 0x38; // C_NetworkUtlVectorBase< AbilityID_t >
}

pub mod TimedEvent {
    pub const m_TimeBetweenEvents: usize = 0x0; // float32
    pub const m_fNextEvent: usize = 0x4; // float32
}

pub mod TransitioningLayer_t {
    pub const m_op: usize = 0x8; // CNetworkedSequenceOperation
    pub const m_flStartAnimTime: usize = 0x30; // float32
    pub const m_flStartWeight: usize = 0x34; // float32
    pub const m_flAnimTime: usize = 0x38; // float32
    pub const m_nOrder: usize = 0x3C; // int32
    pub const m_flPlaybackRate: usize = 0x40; // float32
    pub const m_flFadeOutDuration: usize = 0x44; // float32
}

pub mod TreeModelReplacement_t {
    pub const m_nBinaryObjectID: usize = 0x30; // int32
    pub const m_szModel: usize = 0x34; // char[512]
}

pub mod VPhysicsCollisionAttribute_t {
    pub const m_nInteractsAs: usize = 0x8; // uint64
    pub const m_nInteractsWith: usize = 0x10; // uint64
    pub const m_nInteractsExclude: usize = 0x18; // uint64
    pub const m_nEntityId: usize = 0x20; // uint32
    pub const m_nOwnerId: usize = 0x24; // uint32
    pub const m_nHierarchyId: usize = 0x28; // uint16
    pub const m_nCollisionGroup: usize = 0x2A; // uint8
    pub const m_nCollisionFunctionMask: usize = 0x2B; // uint8
}

pub mod audioparams_t {
    pub const localSound: usize = 0x8; // Vector[8]
    pub const soundscapeIndex: usize = 0x68; // int32
    pub const localBits: usize = 0x6C; // uint8
    pub const soundscapeEntityListIndex: usize = 0x70; // int32
    pub const soundEventHash: usize = 0x74; // uint32
}

pub mod fogparams_t {
    pub const dirPrimary: usize = 0x8; // Vector
    pub const colorPrimary: usize = 0x14; // Color
    pub const colorSecondary: usize = 0x18; // Color
    pub const colorPrimaryLerpTo: usize = 0x1C; // Color
    pub const colorSecondaryLerpTo: usize = 0x20; // Color
    pub const start: usize = 0x24; // float32
    pub const end: usize = 0x28; // float32
    pub const farz: usize = 0x2C; // float32
    pub const maxdensity: usize = 0x30; // float32
    pub const exponent: usize = 0x34; // float32
    pub const HDRColorScale: usize = 0x38; // float32
    pub const skyboxFogFactor: usize = 0x3C; // float32
    pub const skyboxFogFactorLerpTo: usize = 0x40; // float32
    pub const startLerpTo: usize = 0x44; // float32
    pub const endLerpTo: usize = 0x48; // float32
    pub const maxdensityLerpTo: usize = 0x4C; // float32
    pub const lerptime: usize = 0x50; // GameTime_t
    pub const duration: usize = 0x54; // float32
    pub const blendtobackground: usize = 0x58; // float32
    pub const scattering: usize = 0x5C; // float32
    pub const locallightscale: usize = 0x60; // float32
    pub const enable: usize = 0x64; // bool
    pub const blend: usize = 0x65; // bool
    pub const m_bNoReflectionFog: usize = 0x66; // bool
    pub const m_bPadding: usize = 0x67; // bool
}

pub mod sControlGroupElem {
    pub const m_UnitName: usize = 0x0; // char[260]
    pub const m_UnitLabel: usize = 0x104; // char[260]
    pub const m_unUnitLabelIndex: usize = 0x208; // uint8
    pub const m_hEntity: usize = 0x20C; // CHandle< C_BaseEntity >
    pub const m_bIsIllusion: usize = 0x210; // bool
    pub const m_IllusionLabel: usize = 0x211; // char[260]
}

pub mod sky3dparams_t {
    pub const scale: usize = 0x8; // int16
    pub const origin: usize = 0xC; // Vector
    pub const bClip3DSkyBoxNearToWorldFar: usize = 0x18; // bool
    pub const flClip3DSkyBoxNearToWorldFarOffset: usize = 0x1C; // float32
    pub const fog: usize = 0x20; // fogparams_t
    pub const m_nWorldGroupID: usize = 0x88; // WorldGroupId_t
}