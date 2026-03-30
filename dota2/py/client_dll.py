# Generated using https://github.com/ikhsanprasetyo/source2-dumper
# 2026-03-30 13:31:39.632920200 +07:00

class Schemas:
    # Module: client.dll
    class ClientDll:
        class C_BaseCombatCharacter__WaterWakeMode_t:
            WATER_WAKE_NONE = 0x0
            WATER_WAKE_IDLE = 0x1
            WATER_WAKE_WALKING = 0x2
            WATER_WAKE_RUNNING = 0x3
            WATER_WAKE_WATER_OVERHEAD = 0x4
        class PulseBestOutflowRules_t:
            SORT_BY_NUMBER_OF_VALID_CRITERIA = 0x0
            SORT_BY_OUTFLOW_INDEX = 0x1
        class PulseCursorCancelPriority_t:
            None_ = 0x0
            CancelOnSucceeded = 0x1
            SoftCancel = 0x2
            HardCancel = 0x3
        class CDOTA_BaseNPC_AghsFort_Watch_Tower__ExitDirection_t:
            EXIT_DIRECTION_LEFT = 0x0
            EXIT_DIRECTION_TOP = 0x1
            EXIT_DIRECTION_RIGHT = 0x2
        class PulseMethodCallMode_t:
            SYNC_WAIT_FOR_COMPLETION = 0x0
            ASYNC_FIRE_AND_FORGET = 0x1
        class VisualNovelDialogueLineType_t:
            k_eVisualNovelDialogueLineType_Invalid = 0x0
            k_eVisualNovelDialogueLineType_Text = 0x1
            k_eVisualNovelDialogueLineType_PopupText = 0x2
            k_eVisualNovelDialogueLineType_Comic = 0x3
            k_eVisualNovelDialogueLineType_ExternalComic = 0x4
            k_eVisualNovelDialogueLineType_Video = 0x5
            k_eVisualNovelDialogueLineType_ConditionalIf = 0x6
            k_eVisualNovelDialogueLineType_ConditionalElse = 0x7
        class EOverworldEncounterRewardStyle:
            k_eOverworldEncounterRewardStyle_Invalid = 0x0
            k_eOverworldEncounterRewardStyle_Choice = 0x1
            k_eOverworldEncounterRewardStyle_Random = 0x2
            k_eOverworldEncounterRewardStyle_HighScore = 0x3
            k_eOverworldEncounterRewardStyle_ClaimCredits = 0x4
            k_eOverworldEncounterRewardStyle_Quest = 0x5
            k_eOverworldEncounterRewardStyle_ClaimCreditsRange = 0x6
            k_eOverworldEncounterRewardStyle_Custom = 0xA
            k_eOverworldEncounterRewardStyle_RepeatableChoice = 0xB
        class ESurvivorsEnemySpawnBehavior:
            INVALID_SPAWN_BEHAVIOR = 0xFFFFFFFFFFFFFFFF
            FIXED_DIRECTION = 0x0
            FIXED_DIRECTION_PERPENDICULAR_WALL = 0x1
            OVAL_AROUND_PLAYER = 0x2
            STATIC_IN_MAP = 0x3
            STATIC_IN_MAP_IGNORE_PLAYER_RADIUS = 0x4
            RANDOM_DIRECTION = 0x5
            SPAWNER_ORIGIN_RADIUS = 0x6
        class EOverworldTokenType:
            k_eOverworldTokenType_Invalid = 0x0
            k_eOverworldTokenType_Generic = 0x1
            k_eOverworldTokenType_Hidden = 0x2
            k_eOverworldTokenType_Premium = 0x3
            k_eOverworldTokenType_Scrap = 0x4
        class CMsgDOTARequestMatches_SkillLevel:
            CMsgDOTARequestMatches_SkillLevel_Any = 0x0
            CMsgDOTARequestMatches_SkillLevel_Normal = 0x1
            CMsgDOTARequestMatches_SkillLevel_High = 0x2
            CMsgDOTARequestMatches_SkillLevel_VeryHigh = 0x3
        class StartupBehavior_t:
            UNIT_STARTUP_BEHAVIOR_DEFAULT = 0x0
            UNIT_STARTUP_BEHAVIOR_TAUNT = 0x1
        class ETeamFanContentAssetType:
            k_eFanContentAssetType_LogoPNG = 0x1
            k_eFanContentAssetType_LogoSVG = 0x2
            k_eFanContentAssetType_Logo3D = 0x3
            k_eFanContentAssetType_Players = 0x4
            k_eFanContentAssetType_Sprays = 0x5
            k_eFanContentAssetType_Wallpapers = 0x6
            k_eFanContentAssetType_Emoticons = 0x7
            k_eFanContentAssetType_VoiceLines = 0x8
            k_eFanContentAssetType_Localization = 0x9
            k_eFanContentAssetType_Banner = 0xA
            k_eFanContentAssetType_BaseLogo = 0xB
        class SurvivorsAttributeType_t:
            k_eSurvivorsAttribute_Invalid = 0x0
            k_eSurvivorsAttribute_MaxHP = 0x1
            k_eSurvivorsAttribute_HPRegen = 0x2
            k_eSurvivorsAttribute_LifeSteal = 0x3
            k_eSurvivorsAttribute_Damage = 0x4
            k_eSurvivorsAttribute_DamageMultiplier = 0x5
            k_eSurvivorsAttribute_Damage_Physical = 0x6
            k_eSurvivorsAttribute_DamageMultiplier_Physical = 0x7
            k_eSurvivorsAttribute_Damage_Magical = 0x8
            k_eSurvivorsAttribute_DamageMultiplier_Magical = 0x9
            k_eSurvivorsAttribute_CriticalStrike_Physical_Chance = 0xA
            k_eSurvivorsAttribute_CriticalStrike_Physical_DamageMultiplier = 0xB
            k_eSurvivorsAttribute_MovementSpeed = 0xC
            k_eSurvivorsAttribute_Cooldown = 0xD
            k_eSurvivorsAttribute_CooldownReductionMultiplier = 0xE
            k_eSurvivorsAttribute_Range = 0xF
            k_eSurvivorsAttribute_Armor = 0x10
            k_eSurvivorsAttribute_DamageReflection = 0x11
            k_eSurvivorsAttribute_Dodge = 0x12
            k_eSurvivorsAttribute_ExpMultiplier = 0x13
            k_eSurvivorsAttribute_PickupRadius = 0x14
            k_eSurvivorsAttribute_Luck = 0x15
            k_eSurvivorsAttribute_Lives = 0x16
            k_eSurvivorsAttribute_LimitBreak = 0x17
            k_eSurvivorsAttribute_DashSpeed = 0x18
            k_eSurvivorsAttribute_DashDuration = 0x19
            k_eSurvivorsAttribute_DashCooldown = 0x1A
            k_eSurvivorsAttribute_NumDashes = 0x1B
            k_eSurvivorsAttribute_TargetCount = 0x1C
            k_eSurvivorsAttribute_StunDuration = 0x1D
            k_eSurvivorsAttribute_ProjectileRadius = 0x1E
            k_eSurvivorsAttribute_ProjectileBounces = 0x1F
            k_eSurvivorsAttribute_AoEIncrease = 0x20
            k_eSurvivorsAttribute_KnockbackDistance = 0x21
            k_eSurvivorsAttribute_FreezeDuration = 0x22
            k_eSurvivorsAttribute_VulnerabilityDuration = 0x23
            k_eSurvivorsAttribute_VulnerabilityDamagePercent = 0x24
            k_eSurvivorsAttribute_VulnerabilityStunOnHitDuration = 0x25
            k_eSurvivorsAttribute_ModifierDamagePerTick = 0x26
            k_eSurvivorsAttribute_ProjectileSpeed = 0x27
            k_eSurvivorsAttribute_ProjectileAttackInterval = 0x28
            k_eSurvivorsAttribute_Width = 0x29
            k_eSurvivorsAttribute_Length = 0x2A
            k_eSurvivorsAttribute_Radius = 0x2B
            k_eSurvivorsAttribute_InitialTickDelay = 0x2C
            k_eSurvivorsAttribute_DamageTicks = 0x2D
            k_eSurvivorsAttribute_TimeBetweenTicks = 0x2E
            k_eSurvivorsAttribute_LifeTime = 0x2F
            k_eSurvivorsAttribute_ArmingTime = 0x30
            k_eSurvivorsAttribute_TriggerTime = 0x31
            k_eSurvivorsAttribute_MaxRerolls = 0x32
            k_eSurvivorsAttribute_ProjectileHitCount = 0x33
            k_eSurvivorsAttribute_ProjectileHorizontalOffset = 0x34
            k_eSurvivorsAttribute_SplashDamagePercentage = 0x35
            k_eSurvivorsAttribute_Directions = 0x36
            k_eSurvivorsAttribute_Duration = 0x37
            k_eSurvivorsAttribute_DurationExtension = 0x38
            k_eSurvivorsAttribute_StampedeMovementSpeed = 0x39
            k_eSurvivorsAttribute_ActiveAbilitySlots = 0x3A
            k_eSurvivorsAttribute_KnockbackDistanceMultiplier = 0x3B
            k_eSurvivorsAttribute_FreezeSplashRadius = 0x3C
            k_eSurvivorsAttribute_CriticalStrike_Projectile_Chance = 0x3D
            k_eSurvivorsAttribute_CriticalStrike_Projectile_DamageMultiplier = 0x3E
            k_eSurvivorsAttribute_Projectile_BonusMagicDamage = 0x3F
            k_eSurvivorsAttribute_MagicDamageWeakensPhysicalDamageResistance_Duration = 0x40
            k_eSurvivorsAttribute_MagicDamageWeakensPhysicalDamageResistance_Percentage = 0x41
            k_eSurvivorsAttribute_BonusPhysicalDamage = 0x42
            k_eSurvivorsAttribute_IncomingDamageAmplification = 0x43
            k_eSurvivorsAttribute_IncomingDamageAmplificationDuration = 0x44
            k_eSurvivorsAttribute_FreezeStrength = 0x45
            k_eSurvivorsAttribute_ProjectileDamagePercent = 0x46
            k_eSurvivorsAttribute_ProjectileAttack = 0x47
            k_eSurvivorsAttribute_DurationEffect = 0x48
            k_eSurvivorsAttribute_SlowStrength = 0x49
            k_eSurvivorsAttribute_SlowDuration = 0x4A
            k_eSurvivorsAttribute_Count = 0x4B
        class ESurvivorsEnemyMovementCapability:
            ENEMY_MOVEMENT_CAPABILITY_INVALID = 0xFFFFFFFFFFFFFFFF
            ENEMY_MOVEMENT_CAPABILITY_GROUND = 0x0
            ENEMY_MOVEMENT_CAPABILITY_FLYING = 0x1
        class EOverworldIntroProgressState:
            k_eOverworldIntroProgressState_StoryRecap = 0x0
            k_eOverworldIntroProgressState_GetStarted = 0x1
            k_eOverworldIntroProgressState_HowToPlay = 0x2
            k_eOverworldIntroProgressState_FreeToken = 0x3
            k_eOverworldIntroProgressState_RevealInventory = 0x4
            k_eOverworldIntroProgressState_UnlockNode = 0x5
            k_eOverworldIntroProgressState_ShowLearnMore = 0x6
            k_eOverworldIntroProgressState_Complete = 0x7
        class ETalentContentAssetType:
            k_eTalentContentAssetType_Photo = 0x1
            k_eTalentContentAssetType_Autograph = 0x2
            k_eTalentContentAssetType_Voicelines = 0x3
        class EShowcaseType:
            k_eShowcaseType_Invalid = 0x0
            k_eShowcaseType_Profile = 0x1
            k_eShowcaseType_MiniProfile = 0x2
            k_eShowcaseType_DefaultProfile = 0x3
            k_eShowcaseType_DefaultMiniProfile = 0x4
        class DOTA_WatchReplayType:
            DOTA_WATCH_REPLAY_NORMAL = 0x0
            DOTA_WATCH_REPLAY_HIGHLIGHTS = 0x1
        class ESurvivorsBossAbility:
            INVALID = 0xFFFFFFFFFFFFFFFF
            BURNING_GROUND = 0x0
            RADIATE_RAYS = 0x1
            DEMON_PORTALS = 0x2
            MAGIC_MISSILE = 0x3
        class ETournamentEvent:
            k_ETournamentEvent_None = 0x0
            k_ETournamentEvent_TournamentCreated = 0x1
            k_ETournamentEvent_TournamentsMerged = 0x2
            k_ETournamentEvent_GameOutcome = 0x3
            k_ETournamentEvent_TeamGivenBye = 0x4
            k_ETournamentEvent_TournamentCanceledByAdmin = 0x5
            k_ETournamentEvent_TeamAbandoned = 0x6
            k_ETournamentEvent_ScheduledGameStarted = 0x7
            k_ETournamentEvent_Canceled = 0x8
            k_ETournamentEvent_TeamParticipationTimedOut_EntryFeeRefund = 0x9
            k_ETournamentEvent_TeamParticipationTimedOut_EntryFeeForfeit = 0xA
            k_ETournamentEvent_TeamParticipationTimedOut_GrantedVictory = 0xB
        class PortraitScale_t:
            PORTRAIT_SCALE_INVALID = 0xFFFFFFFFFFFFFFFF
            PORTRAIT_SCALE_LOADOUT = 0x0
            PORTRAIT_SCALE_ALTERNATE_LOADOUT = 0x1
            PORTRAIT_SCALE_WORLD = 0x2
            PORTRAIT_SCALE_SPECTATOR_LOADOUT = 0x3
            PORTRAIT_SCALE_VERSUS_LOADOUT = 0x4
        class ENewSettingsBadge:
            eNewSettingsBadge_New = 0x0
            eNewSettingsBadge_Updated = 0x1
        class EDOTAGroupMergeResult:
            k_EDOTAGroupMergeResult_OK = 0x0
            k_EDOTAGroupMergeResult_FAILED_GENERIC = 0x1
            k_EDOTAGroupMergeResult_NOT_LEADER = 0x2
            k_EDOTAGroupMergeResult_TOO_MANY_PLAYERS = 0x3
            k_EDOTAGroupMergeResult_TOO_MANY_COACHES = 0x4
            k_EDOTAGroupMergeResult_ENGINE_MISMATCH = 0x5
            k_EDOTAGroupMergeResult_NO_SUCH_GROUP = 0x6
            k_EDOTAGroupMergeResult_OTHER_GROUP_NOT_OPEN = 0x7
            k_EDOTAGroupMergeResult_ALREADY_INVITED = 0x8
            k_EDOTAGroupMergeResult_NOT_INVITED = 0x9
        class EPartyBeaconType:
            k_EPartyBeaconType_Available = 0x0
            k_EPartyBeaconType_Joinable = 0x1
        class EOverworldFortuneReward:
            k_eOverworldFortuneReward_Invalid = 0x0
            k_eOverworldFortuneReward_RandomToken = 0x1
            k_eOverworldFortuneReward_ExtraToken = 0x2
            k_eOverworldFortuneReward_CandyBag = 0x3
            k_eOverworldFortuneReward_DiscountCoin = 0x4
            k_eOverworldFortuneReward_Treasure = 0x5
        class EDOTAFightingGameAILevel:
            FG_AI_LEVEL_NONE = 0xFFFFFFFFFFFFFFFF
            FG_AI_LEVEL_EASY = 0x0
            FG_AI_LEVEL_MEDIUM = 0x1
            FG_AI_LEVEL_HARD = 0x2
        class EShowcaseAuditAction:
            k_eShowcaseAuditAction_Invalid = 0x0
            k_eShowcaseAuditAction_ShowcaseChanged = 0x1
            k_eShowcaseAuditAction_AdminShowcaseReset = 0x2
            k_eShowcaseAuditAction_AdminShowcaseAccountLocked = 0x3
            k_eShowcaseAuditAction_AdminShowcaseExonerated = 0x4
            k_eShowcaseAuditAction_AdminShowcaseConvicted = 0x5
            k_eShowcaseAuditAction_AdminModerationApproved = 0x6
            k_eShowcaseAuditAction_AdminModerationRejected = 0x7
        class EItemEditorReservationResult:
            k_EItemEditorReservationResult_OK = 0x1
            k_EItemEditorReservationResult_AlreadyExists = 0x2
            k_EItemEditorReservationResult_Reserved = 0x3
            k_EItemEditorReservationResult_TimedOut = 0x4
        class CMsgClientToGCUpdateComicBookStat_Type:
            CMsgClientToGCUpdateComicBookStat_Type_HighestPageRead = 0x1
            CMsgClientToGCUpdateComicBookStat_Type_SecondsSpentReading = 0x2
            CMsgClientToGCUpdateComicBookStat_Type_HighestPercentRead = 0x3
        class EHeroSelectionText:
            k_EHeroSelectionText_Invalid = 0xFFFFFFFFFFFFFFFF
            k_EHeroSelectionText_None = 0x0
            k_EHeroSelectionText_ChooseHero = 0x1
            k_EHeroSelectionText_AllDraft_Planning_YouFirst = 0x2
            k_EHeroSelectionText_AllDraft_Planning_TheyFirst = 0x3
            k_EHeroSelectionText_AllDraft_Banning = 0x4
            k_EHeroSelectionText_AllDraft_Ban_Waiting = 0x5
            k_EHeroSelectionText_AllDraft_PickTwo = 0x6
            k_EHeroSelectionText_AllDraft_PickOneMore = 0x7
            k_EHeroSelectionText_AllDraft_PickOne = 0x8
            k_EHeroSelectionText_AllDraft_WaitingRadiant = 0x9
            k_EHeroSelectionText_AllDraft_WaitingDire = 0xA
            k_EHeroSelectionText_AllDraft_TeammateRandomed = 0xB
            k_EHeroSelectionText_AllDraft_YouPicking_LosingGold = 0xC
            k_EHeroSelectionText_AllDraft_TheyPicking_LosingGold = 0xD
            k_EHeroSelectionText_CaptainsMode_ChooseCaptain = 0xE
            k_EHeroSelectionText_CaptainsMode_WaitingForChooseCaptain = 0xF
            k_EHeroSelectionText_CaptainsMode_YouSelect = 0x10
            k_EHeroSelectionText_CaptainsMode_TheySelect = 0x11
            k_EHeroSelectionText_CaptainsMode_YouBan = 0x12
            k_EHeroSelectionText_CaptainsMode_TheyBan = 0x13
            k_EHeroSelectionText_RandomDraft_HeroReview = 0x14
            k_EHeroSelectionText_RandomDraft_RoundDisplay = 0x15
            k_EHeroSelectionText_RandomDraft_Waiting = 0x16
            k_EHeroSelectionText_EventGame_BanPhase = 0x17
        class EOverworldProgressionType:
            k_eOverworldProgressionType_NodesAndPaths = 0x0
            k_eOverworldProgressionType_NodesPathsAndRooms = 0x1
        class EBingoAuditAction:
            k_eBingoAuditAction_Invalid = 0x0
            k_eBingoAuditAction_DevModifyTokens = 0x1
            k_eBingoAuditAction_DevClearInventory = 0x2
            k_eBingoAuditAction_DevRerollCard = 0x3
            k_eBingoAuditAction_ShuffleCard = 0x4
            k_eBingoAuditAction_RerollSquare = 0x5
            k_eBingoAuditAction_UpgradeSquare = 0x6
            k_eBingoAuditAction_ClaimRow = 0x7
            k_eBingoAuditAction_EventActionTokenGrant = 0x8
            k_eBingoAuditAction_SupportGrantTokens = 0x9
            k_eBingoAuditAction_SupportStatThresholdFixup = 0xA
        class DOTAVisualNovelDialogueEffect_t:
            VN_DIALOGUE_EFFECT_NONE = 0x0
            VN_DIALOGUE_EFFECT_ENTRANCE_SLIDE_SLOW = 0x1
            VN_DIALOGUE_EFFECT_ENTRANCE_SLIDE_FAST = 0x2
            VN_DIALOGUE_EFFECT_ENTRANCE_RISING = 0x4
            VN_DIALOGUE_EFFECT_ENTRANCE_FALLING = 0x8
            VN_DIALOGUE_EFFECT_ENTRANCE_LEFT = 0x10
            VN_DIALOGUE_EFFECT_ENTRANCE_RIGHT = 0x20
            VN_DIALOGUE_EFFECT_EXIT_INSTANT = 0x40
            VN_DIALOGUE_EFFECT_EXIT_SLIDE_SLOW = 0x80
            VN_DIALOGUE_EFFECT_EXIT_SLIDE_FAST = 0x100
            VN_DIALOGUE_EFFECT_EXIT_FALLING = 0x200
            VN_DIALOGUE_EFFECT_EXIT_RISING = 0x400
            VN_DIALOGUE_EFFECT_WINDOW_SHAKE = 0x800
            VN_DIALOGUE_EFFECT_PORTRAIT_SHAKE = 0x1000
            VN_DIALOGUE_EFFECT_PORTRAIT_WEAVE = 0x2000
            VN_DIALOGUE_EFFECT_PORTRAIT_FLIP = 0x4000
            VN_DIALOGUE_EFFECT_TEXT_INSTANT = 0x8000
            VN_DIALOGUE_EFFECT_TEXT_FAST = 0x10000
            VN_DIALOGUE_EFFECT_TEXT_BOLD = 0x20000
            VN_DIALOGUE_EFFECT_TEXT_SMALL = 0x40000
            VN_DIALOGUE_EFFECT_NO_WAIT = 0x80000
            VN_DIALOGUE_EFFECT_ACTOR_ANGER = 0x100000
            VN_DIALOGUE_EFFECT_ACTOR_ANXIETY = 0x200000
            VN_DIALOGUE_EFFECT_ACTOR_SWEATDROP = 0x400000
            VN_DIALOGUE_EFFECT_POPUP_WET = 0x800000
            VN_DIALOGUE_EFFECT_POPUP_FOCUS_BOTTOM = 0x1000000
            VN_DIALOGUE_EFFECT_POPUP_FOCUS_TOP = 0x2000000
            VN_DIALOGUE_EFFECT_PORTRAIT_BIG = 0x4000000
            VN_DIALOGUE_EFFECT_CLEAR_STAGE = 0x8000000
            VN_DIALOGUE_EFFECT_PORTRAIT_JITTER = 0x10000000
            VN_DIALOGUE_EFFECT_PORTRAIT_BORDER_FROZEN = 0x20000000
            VN_DIALOGUE_EFFECT_PORTRAIT_SMALL = 0x40000000
            VN_DIALOGUE_EFFECT_PORTRAIT_NO_BLUR_OTHER = 0x80000000
        class EArtyHitboxType:
            k_eAABB = 0x0
            k_eCircle = 0x1
            k_eLine = 0x2
            k_eRay = 0x3
        class EShowcaseItemState:
            k_eShowcaseItemState_Ok = 0x0
            k_eShowcaseItemState_MinorModifications = 0x1
            k_eShowcaseItemState_ValidityUnknown = 0x2
            k_eShowcaseItemState_PartiallyInvalid = 0x3
            k_eShowcaseItemState_Invalid = 0x4
            k_eShowcaseItemState_Failure = 0x5
        class ESupportEventRequestResult:
            k_ESupportEventRequestResult_Success = 0x0
            k_ESupportEventRequestResult_Timeout = 0x1
            k_ESupportEventRequestResult_CantLockSOCache = 0x2
            k_ESupportEventRequestResult_ItemNotInInventory = 0x3
            k_ESupportEventRequestResult_InvalidItemDef = 0x4
            k_ESupportEventRequestResult_InvalidEvent = 0x5
            k_ESupportEventRequestResult_EventExpired = 0x6
            k_ESupportEventRequestResult_InvalidSupportAccount = 0x7
            k_ESupportEventRequestResult_InvalidSupportMessage = 0x8
            k_ESupportEventRequestResult_InvalidEventPoints = 0x9
            k_ESupportEventRequestResult_InvalidPremiumPoints = 0xA
            k_ESupportEventRequestResult_InvalidActionID = 0xB
            k_ESupportEventRequestResult_InvalidActionScore = 0xC
            k_ESupportEventRequestResult_TransactionFailed = 0xD
        class ETournamentTemplate:
            k_ETournamentTemplate_None = 0x0
            k_ETournamentTemplate_AutomatedWin3 = 0x1
        class EDevEventRequestResult:
            k_EDevEventRequestResult_Success = 0x0
            k_EDevEventRequestResult_NotAllowed = 0x1
            k_EDevEventRequestResult_InvalidEvent = 0x2
            k_EDevEventRequestResult_SqlFailure = 0x3
            k_EDevEventRequestResult_Timeout = 0x4
            k_EDevEventRequestResult_LockFailure = 0x5
            k_EDevEventRequestResult_SDOLoadFailure = 0x6
        class EGuildChatType:
            k_EGuildChatType_Unspecified = 0x0
            k_EGuildChatType_SteamChatGroup = 0x1
            k_EGuildChatType_GC = 0x2
        class EArtyOrderFlag:
            k_EArtyOrderFlag_None = 0x0
            k_EArtyOrderFlag_MoveLeft = 0x1
            k_EArtyOrderFlag_MoveRight = 0x2
            k_EArtyOrderFlag_AimUp = 0x4
            k_EArtyOrderFlag_AimDown = 0x8
            k_EArtyOrderFlag_NextWeapon = 0x10
            k_EArtyOrderFlag_PrevWeapon = 0x20
            k_EArtyOrderFlag_FireStart = 0x40
            k_EArtyOrderFlag_FireStop = 0x80
            k_EArtyOrderFlag_FineControl = 0x100
            k_EArtyOrderFlag_PowerUp = 0x200
            k_EArtyOrderFlag_PowerDown = 0x400
            k_EArtyOrderFlag_MoveUp = 0x800
            k_EArtyOrderFlag_MoveDown = 0x1000
        class ECandyShopRewardOptionType:
            k_eCandyShopRewardOptionType_Invalid = 0xFFFFFFFFFFFFFFFF
            k_eCandyShopRewardOptionType_SingleItem = 0x0
            k_eCandyShopRewardOptionType_LootList = 0x1
            k_eCandyShopRewardOptionType_SingleEventAction = 0x2
            k_eCandyShopRewardOptionType_EventPoints = 0x3
        class ETournamentTeamState:
            k_ETournamentTeamState_Unknown = 0x0
            k_ETournamentTeamState_Node1 = 0x1
            k_ETournamentTeamState_NodeMax = 0x400
            k_ETournamentTeamState_Eliminated = 0x36B3
            k_ETournamentTeamState_Forfeited = 0x36B4
            k_ETournamentTeamState_Finished1st = 0x3A99
            k_ETournamentTeamState_Finished2nd = 0x3A9A
            k_ETournamentTeamState_Finished3rd = 0x3A9B
            k_ETournamentTeamState_Finished4th = 0x3A9C
            k_ETournamentTeamState_Finished5th = 0x3A9D
            k_ETournamentTeamState_Finished6th = 0x3A9E
            k_ETournamentTeamState_Finished7th = 0x3A9F
            k_ETournamentTeamState_Finished8th = 0x3AA0
            k_ETournamentTeamState_Finished9th = 0x3AA1
            k_ETournamentTeamState_Finished10th = 0x3AA2
            k_ETournamentTeamState_Finished11th = 0x3AA3
            k_ETournamentTeamState_Finished12th = 0x3AA4
            k_ETournamentTeamState_Finished13th = 0x3AA5
            k_ETournamentTeamState_Finished14th = 0x3AA6
            k_ETournamentTeamState_Finished15th = 0x3AA7
            k_ETournamentTeamState_Finished16th = 0x3AA8
        class EFightingGameInvulnerabilityFlags:
            FG_INVULNERABILITY_NONE = 0x0
            FG_INVULNERABILITY_STRIKE = 0x1
            FG_INVULNERABILITY_PROJECTILE = 0x2
        class EArtyLayer:
            k_eDefault = 0x1
            k_eTerrain = 0x2
            k_eShots = 0x4
            k_eRays = 0x8
            k_eFX = 0x10
            k_ePhysical = 0x7
            k_eFullObjects = 0x3
            k_eAllLayers = 0xFFFFFFFF
        class SteamUGCQuery:
            RankedByVote = 0x0
            RankedByPublicationDate = 0x1
            AcceptedForGameRankedByAcceptanceDate = 0x2
            RankedByTrend = 0x3
            FavoritedByFriendsRankedByPublicationDate = 0x4
            CreatedByFriendsRankedByPublicationDate = 0x5
            RankedByNumTimesReported = 0x6
            CreatedByFollowedUsersRankedByPublicationDate = 0x7
            NotYetRated = 0x8
            RankedByTotalVotesAsc = 0x9
            RankedByVotesUp = 0xA
            RankedByTextSearch = 0xB
            RankedByTotalUniqueSubscriptions = 0xC
            RankedByPlaytimeTrend = 0xD
            RankedByTotalPlaytime = 0xE
            RankedByAveragePlaytimeTrend = 0xF
            RankedByLifetimeAveragePlaytime = 0x10
            RankedByPlaytimeSessionsTrend = 0x11
            RankedByLifetimePlaytimeSessions = 0x12
        class EOverworldNodeAlertState:
            k_eOverworldNodeAlertState_Unset = 0x0
            k_eOverworldNodeAlertState_Show = 0x1
            k_eOverworldNodeAlertState_Hide = 0x2
        class DOTAKeybindCommand_t:
            DOTA_KEYBIND_NONE = 0x0
            DOTA_KEYBIND_FIRST = 0x1
            DOTA_KEYBIND_CAMERA_DOWN = 0x2
            DOTA_KEYBIND_CAMERA_LEFT = 0x3
            DOTA_KEYBIND_CAMERA_RIGHT = 0x4
            DOTA_KEYBIND_CAMERA_GRIP = 0x5
            DOTA_KEYBIND_CAMERA_YAW_GRIP = 0x6
            DOTA_KEYBIND_CAMERA_SAVED_POSITION_1 = 0x7
            DOTA_KEYBIND_CAMERA_SAVED_POSITION_2 = 0x8
            DOTA_KEYBIND_CAMERA_SAVED_POSITION_3 = 0x9
            DOTA_KEYBIND_CAMERA_SAVED_POSITION_4 = 0xA
            DOTA_KEYBIND_CAMERA_SAVED_POSITION_5 = 0xB
            DOTA_KEYBIND_CAMERA_SAVED_POSITION_6 = 0xC
            DOTA_KEYBIND_CAMERA_SAVED_POSITION_7 = 0xD
            DOTA_KEYBIND_CAMERA_SAVED_POSITION_8 = 0xE
            DOTA_KEYBIND_CAMERA_SAVED_POSITION_9 = 0xF
            DOTA_KEYBIND_CAMERA_SAVED_POSITION_10 = 0x10
            DOTA_KEYBIND_HERO_ATTACK = 0x11
            DOTA_KEYBIND_HERO_MOVE = 0x12
            DOTA_KEYBIND_HERO_MOVE_DIRECTION = 0x13
            DOTA_KEYBIND_PATROL = 0x14
            DOTA_KEYBIND_HERO_STOP = 0x15
            DOTA_KEYBIND_HERO_HOLD = 0x16
            DOTA_KEYBIND_HERO_SELECT = 0x17
            DOTA_KEYBIND_COURIER_SELECT = 0x18
            DOTA_KEYBIND_COURIER_DELIVER = 0x19
            DOTA_KEYBIND_COURIER_BURST = 0x1A
            DOTA_KEYBIND_COURIER_SHIELD = 0x1B
            DOTA_KEYBIND_PAUSE = 0x1C
            DOTA_SELECT_ALL = 0x1D
            DOTA_SELECT_ALL_OTHERS = 0x1E
            DOTA_RECENT_EVENT = 0x1F
            DOTA_KEYBIND_CHAT_TEAM = 0x20
            DOTA_KEYBIND_CHAT_GLOBAL = 0x21
            DOTA_KEYBIND_CHAT_TEAM2 = 0x22
            DOTA_KEYBIND_CHAT_GLOBAL2 = 0x23
            DOTA_KEYBIND_CHAT_VOICE_PARTY = 0x24
            DOTA_KEYBIND_CHAT_VOICE_TEAM = 0x25
            DOTA_KEYBIND_CHAT_WHEEL = 0x26
            DOTA_KEYBIND_CHAT_WHEEL2 = 0x27
            DOTA_KEYBIND_CHAT_WHEEL_CARE = 0x28
            DOTA_KEYBIND_CHAT_WHEEL_BACK = 0x29
            DOTA_KEYBIND_CHAT_WHEEL_NEED_WARDS = 0x2A
            DOTA_KEYBIND_CHAT_WHEEL_STUN = 0x2B
            DOTA_KEYBIND_CHAT_WHEEL_HELP = 0x2C
            DOTA_KEYBIND_CHAT_WHEEL_GET_PUSH = 0x2D
            DOTA_KEYBIND_CHAT_WHEEL_GOOD_JOB = 0x2E
            DOTA_KEYBIND_CHAT_WHEEL_MISSING = 0x2F
            DOTA_KEYBIND_CHAT_WHEEL_MISSING_TOP = 0x30
            DOTA_KEYBIND_CHAT_WHEEL_MISSING_MIDDLE = 0x31
            DOTA_KEYBIND_CHAT_WHEEL_MISSING_BOTTOM = 0x32
            DOTA_KEYBIND_HERO_CHAT_WHEEL = 0x33
            DOTA_KEYBIND_SPRAY_WHEEL = 0x34
            DOTA_KEYBIND_ABILITY_PRIMARY1 = 0x35
            DOTA_KEYBIND_ABILITY_PRIMARY2 = 0x36
            DOTA_KEYBIND_ABILITY_PRIMARY3 = 0x37
            DOTA_KEYBIND_ABILITY_SECONDARY1 = 0x38
            DOTA_KEYBIND_ABILITY_SECONDARY2 = 0x39
            DOTA_KEYBIND_ABILITY_ULTIMATE = 0x3A
            DOTA_KEYBIND_TALENT_UPGRADE_LEFT = 0x3B
            DOTA_KEYBIND_TALENT_UPGRADE_RIGHT = 0x3C
            DOTA_KEYBIND_TALENT_UPGRADE_ATTRIBUTE = 0x3D
            DOTA_KEYBIND_NEUTRAL_ITEM_SELECT1 = 0x3E
            DOTA_KEYBIND_NEUTRAL_ITEM_SELECT2 = 0x3F
            DOTA_KEYBIND_NEUTRAL_ITEM_SELECT3 = 0x40
            DOTA_KEYBIND_NEUTRAL_ITEM_SELECT4 = 0x41
            DOTA_KEYBIND_NEUTRAL_ITEM_SELECT5 = 0x42
            DOTA_KEYBIND_ABILITY_PRIMARY1_QUICKCAST = 0x43
            DOTA_KEYBIND_ABILITY_PRIMARY2_QUICKCAST = 0x44
            DOTA_KEYBIND_ABILITY_PRIMARY3_QUICKCAST = 0x45
            DOTA_KEYBIND_ABILITY_SECONDARY1_QUICKCAST = 0x46
            DOTA_KEYBIND_ABILITY_SECONDARY2_QUICKCAST = 0x47
            DOTA_KEYBIND_ABILITY_ULTIMATE_QUICKCAST = 0x48
            DOTA_KEYBIND_ABILITY_PRIMARY1_EXPLICIT_AUTOCAST = 0x49
            DOTA_KEYBIND_ABILITY_PRIMARY2_EXPLICIT_AUTOCAST = 0x4A
            DOTA_KEYBIND_ABILITY_PRIMARY3_EXPLICIT_AUTOCAST = 0x4B
            DOTA_KEYBIND_ABILITY_SECONDARY1_EXPLICIT_AUTOCAST = 0x4C
            DOTA_KEYBIND_ABILITY_SECONDARY2_EXPLICIT_AUTOCAST = 0x4D
            DOTA_KEYBIND_ABILITY_ULTIMATE_EXPLICIT_AUTOCAST = 0x4E
            DOTA_KEYBIND_ABILITY_PRIMARY1_QUICKCAST_AUTOCAST = 0x4F
            DOTA_KEYBIND_ABILITY_PRIMARY2_QUICKCAST_AUTOCAST = 0x50
            DOTA_KEYBIND_ABILITY_PRIMARY3_QUICKCAST_AUTOCAST = 0x51
            DOTA_KEYBIND_ABILITY_SECONDARY1_QUICKCAST_AUTOCAST = 0x52
            DOTA_KEYBIND_ABILITY_SECONDARY2_QUICKCAST_AUTOCAST = 0x53
            DOTA_KEYBIND_ABILITY_ULTIMATE_QUICKCAST_AUTOCAST = 0x54
            DOTA_KEYBIND_ABILITY_PRIMARY1_AUTOMATIC_AUTOCAST = 0x55
            DOTA_KEYBIND_ABILITY_PRIMARY2_AUTOMATIC_AUTOCAST = 0x56
            DOTA_KEYBIND_ABILITY_PRIMARY3_AUTOMATIC_AUTOCAST = 0x57
            DOTA_KEYBIND_ABILITY_SECONDARY1_AUTOMATIC_AUTOCAST = 0x58
            DOTA_KEYBIND_ABILITY_SECONDARY2_AUTOMATIC_AUTOCAST = 0x59
            DOTA_KEYBIND_ABILITY_ULTIMATE_AUTOMATIC_AUTOCAST = 0x5A
            DOTA_KEYBIND_INVENTORY1 = 0x5B
            DOTA_KEYBIND_INVENTORY2 = 0x5C
            DOTA_KEYBIND_INVENTORY3 = 0x5D
            DOTA_KEYBIND_INVENTORY4 = 0x5E
            DOTA_KEYBIND_INVENTORY5 = 0x5F
            DOTA_KEYBIND_INVENTORY6 = 0x60
            DOTA_KEYBIND_INVENTORYTP = 0x61
            DOTA_KEYBIND_INVENTORYNEUTRAL = 0x62
            DOTA_KEYBIND_INVENTORY1_QUICKCAST = 0x63
            DOTA_KEYBIND_INVENTORY2_QUICKCAST = 0x64
            DOTA_KEYBIND_INVENTORY3_QUICKCAST = 0x65
            DOTA_KEYBIND_INVENTORY4_QUICKCAST = 0x66
            DOTA_KEYBIND_INVENTORY5_QUICKCAST = 0x67
            DOTA_KEYBIND_INVENTORY6_QUICKCAST = 0x68
            DOTA_KEYBIND_INVENTORYTP_QUICKCAST = 0x69
            DOTA_KEYBIND_INVENTORYNEUTRAL_QUICKCAST = 0x6A
            DOTA_KEYBIND_INVENTORY1_AUTOCAST = 0x6B
            DOTA_KEYBIND_INVENTORY2_AUTOCAST = 0x6C
            DOTA_KEYBIND_INVENTORY3_AUTOCAST = 0x6D
            DOTA_KEYBIND_INVENTORY4_AUTOCAST = 0x6E
            DOTA_KEYBIND_INVENTORY5_AUTOCAST = 0x6F
            DOTA_KEYBIND_INVENTORY6_AUTOCAST = 0x70
            DOTA_KEYBIND_INVENTORYTP_AUTOCAST = 0x71
            DOTA_KEYBIND_INVENTORYNEUTRAL_AUTOCAST = 0x72
            DOTA_KEYBIND_INVENTORY1_QUICKAUTOCAST = 0x73
            DOTA_KEYBIND_INVENTORY2_QUICKAUTOCAST = 0x74
            DOTA_KEYBIND_INVENTORY3_QUICKAUTOCAST = 0x75
            DOTA_KEYBIND_INVENTORY4_QUICKAUTOCAST = 0x76
            DOTA_KEYBIND_INVENTORY5_QUICKAUTOCAST = 0x77
            DOTA_KEYBIND_INVENTORY6_QUICKAUTOCAST = 0x78
            DOTA_KEYBIND_INVENTORYTP_QUICKAUTOCAST = 0x79
            DOTA_KEYBIND_INVENTORYNEUTRAL_QUICKAUTOCAST = 0x7A
            DOTA_KEYBIND_CONTROL_GROUP1 = 0x7B
            DOTA_KEYBIND_CONTROL_GROUP2 = 0x7C
            DOTA_KEYBIND_CONTROL_GROUP3 = 0x7D
            DOTA_KEYBIND_CONTROL_GROUP4 = 0x7E
            DOTA_KEYBIND_CONTROL_GROUP5 = 0x7F
            DOTA_KEYBIND_CONTROL_GROUP6 = 0x80
            DOTA_KEYBIND_CONTROL_GROUP7 = 0x81
            DOTA_KEYBIND_CONTROL_GROUP8 = 0x82
            DOTA_KEYBIND_CONTROL_GROUP9 = 0x83
            DOTA_KEYBIND_CONTROL_GROUP10 = 0x84
            DOTA_KEYBIND_CONTROL_GROUPCYCLE = 0x85
            DOTA_KEYBIND_SELECT_ALLY1 = 0x86
            DOTA_KEYBIND_SELECT_ALLY2 = 0x87
            DOTA_KEYBIND_SELECT_ALLY3 = 0x88
            DOTA_KEYBIND_SELECT_ALLY4 = 0x89
            DOTA_KEYBIND_SELECT_ALLY5 = 0x8A
            DOTA_KEYBIND_SHOP_TOGGLE = 0x8B
            DOTA_KEYBIND_SCOREBOARD_TOGGLE = 0x8C
            DOTA_KEYBIND_COMBATLOG_TOGGLE = 0x8D
            DOTA_KEYBIND_SCREENSHOT = 0x8E
            DOTA_KEYBIND_ESCAPE = 0x8F
            DOTA_KEYBIND_CONSOLE = 0x90
            DOTA_KEYBIND_DEATH_SUMMARY = 0x91
            DOTA_KEYBIND_LEARN_ABILITIES = 0x92
            DOTA_KEYBIND_LEARN_STATS = 0x93
            DOTA_KEYBIND_ACTIVATE_GLYPH = 0x94
            DOTA_KEYBIND_ACTIVATE_RADAR = 0x95
            DOTA_KEYBIND_PURCHASE_QUICKBUY = 0x96
            DOTA_KEYBIND_PURCHASE_STICKY = 0x97
            DOTA_KEYBIND_TOGGLE_BUYBACK_PROTECTION = 0x98
            DOTA_KEYBIND_GRAB_STASH_ITEMS = 0x99
            DOTA_KEYBIND_TOGGLE_AUTOATTACK = 0x9A
            DOTA_KEYBIND_TOGGLE_OVERLAYMAP = 0x9B
            DOTA_KEYBIND_OVERLAYMAP_INPUTKEY = 0x9C
            DOTA_KEYBIND_FILTER_ENEMY = 0x9D
            DOTA_KEYBIND_FILTER_ALLY = 0x9E
            DOTA_KEYBIND_FILTER_HERO = 0x9F
            DOTA_KEYBIND_FILTER_NONHERO = 0xA0
            DOTA_KEYBIND_TAUNT = 0xA1
            DOTA_KEYBIND_SHOP_CONSUMABLES = 0xA2
            DOTA_KEYBIND_SHOP_ATTRIBUTES = 0xA3
            DOTA_KEYBIND_SHOP_ARMAMENTS = 0xA4
            DOTA_KEYBIND_SHOP_ARCANE = 0xA5
            DOTA_KEYBIND_SHOP_BASICS = 0xA6
            DOTA_KEYBIND_SHOP_SUPPORT = 0xA7
            DOTA_KEYBIND_SHOP_CASTER = 0xA8
            DOTA_KEYBIND_SHOP_WEAPONS = 0xA9
            DOTA_KEYBIND_SHOP_ARMOR = 0xAA
            DOTA_KEYBIND_SHOP_ARTIFACTS = 0xAB
            DOTA_KEYBIND_SHOP_SIDE_PAGE_1 = 0xAC
            DOTA_KEYBIND_SHOP_SIDE_PAGE_2 = 0xAD
            DOTA_KEYBIND_SHOP_SECRET = 0xAE
            DOTA_KEYBIND_SHOP_SEARCHBOX = 0xAF
            DOTA_KEYBIND_SHOP_SLOT_1 = 0xB0
            DOTA_KEYBIND_SHOP_SLOT_2 = 0xB1
            DOTA_KEYBIND_SHOP_SLOT_3 = 0xB2
            DOTA_KEYBIND_SHOP_SLOT_4 = 0xB3
            DOTA_KEYBIND_SHOP_SLOT_5 = 0xB4
            DOTA_KEYBIND_SHOP_SLOT_6 = 0xB5
            DOTA_KEYBIND_SHOP_SLOT_7 = 0xB6
            DOTA_KEYBIND_SHOP_SLOT_8 = 0xB7
            DOTA_KEYBIND_SHOP_SLOT_9 = 0xB8
            DOTA_KEYBIND_SHOP_SLOT_10 = 0xB9
            DOTA_KEYBIND_SHOP_SLOT_11 = 0xBA
            DOTA_KEYBIND_SHOP_SLOT_12 = 0xBB
            DOTA_KEYBIND_SHOP_SLOT_13 = 0xBC
            DOTA_KEYBIND_SHOP_SLOT_14 = 0xBD
            DOTA_KEYBIND_SPEC_CAMERA_UP = 0xBE
            DOTA_KEYBIND_SPEC_CAMERA_DOWN = 0xBF
            DOTA_KEYBIND_SPEC_CAMERA_LEFT = 0xC0
            DOTA_KEYBIND_SPEC_CAMERA_RIGHT = 0xC1
            DOTA_KEYBIND_SPEC_CAMERA_GRIP = 0xC2
            DOTA_KEYBIND_SPEC_CAMERA_SAVED_POSITION_1 = 0xC3
            DOTA_KEYBIND_SPEC_CAMERA_SAVED_POSITION_2 = 0xC4
            DOTA_KEYBIND_SPEC_CAMERA_SAVED_POSITION_3 = 0xC5
            DOTA_KEYBIND_SPEC_CAMERA_SAVED_POSITION_4 = 0xC6
            DOTA_KEYBIND_SPEC_CAMERA_SAVED_POSITION_5 = 0xC7
            DOTA_KEYBIND_SPEC_CAMERA_SAVED_POSITION_6 = 0xC8
            DOTA_KEYBIND_SPEC_CAMERA_SAVED_POSITION_7 = 0xC9
            DOTA_KEYBIND_SPEC_CAMERA_SAVED_POSITION_8 = 0xCA
            DOTA_KEYBIND_SPEC_CAMERA_SAVED_POSITION_9 = 0xCB
            DOTA_KEYBIND_SPEC_CAMERA_SAVED_POSITION_10 = 0xCC
            DOTA_KEYBIND_SPEC_UNIT_SELECT = 0xCD
            DOTA_KEYBIND_SPEC_HERO_SELECT = 0xCE
            DOTA_KEYBIND_SPEC_PAUSE = 0xCF
            DOTA_KEYBIND_SPEC_CHAT = 0xD0
            DOTA_KEYBIND_SPEC_SCOREBOARD = 0xD1
            DOTA_KEYBIND_SPEC_INCREASE_REPLAY_SPEED = 0xD2
            DOTA_KEYBIND_SPEC_DECREASE_REPLAY_SPEED = 0xD3
            DOTA_KEYBIND_SPEC_STATS_ITEM = 0xD4
            DOTA_KEYBIND_SPEC_STATS_GOLD = 0xD5
            DOTA_KEYBIND_SPEC_STATS_XP = 0xD6
            DOTA_KEYBIND_SPEC_STATS_FANTASY = 0xD7
            DOTA_KEYBIND_SPEC_STATS_WINCHANCE = 0xD8
            DOTA_KEYBIND_SPEC_FOW_TOGGLEBOTH = 0xD9
            DOTA_KEYBIND_SPEC_FOW_TOGGLERADIENT = 0xDA
            DOTA_KEYBIND_SPEC_FOW_TOGGLEDIRE = 0xDB
            DOTA_KEYBIND_SPEC_OPEN_BROADCASTER_MENU = 0xDC
            DOTA_KEYBIND_SPEC_DROPDOWN_KDA = 0xDD
            DOTA_KEYBIND_SPEC_DROPDOWN_LASTHITS_DENIES = 0xDE
            DOTA_KEYBIND_SPEC_DROPDOWN_LEVEL = 0xDF
            DOTA_KEYBIND_SPEC_DROPDOWN_XP_PER_MIN = 0xE0
            DOTA_KEYBIND_SPEC_DROPDOWN_GOLD = 0xE1
            DOTA_KEYBIND_SPEC_DROPDOWN_TOTALGOLD = 0xE2
            DOTA_KEYBIND_SPEC_DROPDOWN_GOLD_PER_MIN = 0xE3
            DOTA_KEYBIND_SPEC_DROPDOWN_BUYBACK = 0xE4
            DOTA_KEYBIND_SPEC_DROPDOWN_NETWORTH = 0xE5
            DOTA_KEYBIND_SPEC_DROPDOWN_FANTASY = 0xE6
            DOTA_KEYBIND_SPEC_DROPDOWN_SORT = 0xE7
            DOTA_KEYBIND_SPEC_DROPDOWN_CLOSE = 0xE8
            DOTA_KEYBIND_SPEC_FOCUS_PLAYER_1 = 0xE9
            DOTA_KEYBIND_SPEC_FOCUS_PLAYER_2 = 0xEA
            DOTA_KEYBIND_SPEC_FOCUS_PLAYER_3 = 0xEB
            DOTA_KEYBIND_SPEC_FOCUS_PLAYER_4 = 0xEC
            DOTA_KEYBIND_SPEC_FOCUS_PLAYER_5 = 0xED
            DOTA_KEYBIND_SPEC_FOCUS_PLAYER_6 = 0xEE
            DOTA_KEYBIND_SPEC_FOCUS_PLAYER_7 = 0xEF
            DOTA_KEYBIND_SPEC_FOCUS_PLAYER_8 = 0xF0
            DOTA_KEYBIND_SPEC_FOCUS_PLAYER_9 = 0xF1
            DOTA_KEYBIND_SPEC_FOCUS_PLAYER_10 = 0xF2
            DOTA_KEYBIND_SPEC_COACH_VIEWTOGGLE = 0xF3
            DOTA_KEYBIND_INSPECTHEROINWORLD = 0xF4
            DOTA_KEYBIND_CAMERA_ZOOM_IN = 0xF5
            DOTA_KEYBIND_CAMERA_ZOOM_OUT = 0xF6
            DOTA_KEYBIND_CONTROL_GROUPCYCLEPREV = 0xF7
            DOTA_KEYBIND_DOTA_ALT = 0xF8
            DOTA_KEYBIND_DOTA_ALTERNATIVE_CAST_SWITCH = 0xF9
            DOTA_KEYBIND_COUNT = 0xFA
        class EOverworldFortuneRequirement:
            k_eOverworldFortuneRequirement_Invalid = 0x0
            k_eOverworldFortuneRequirement_PlayGame = 0x1
            k_eOverworldFortuneRequirement_WinGame = 0x2
            k_eOverworldFortuneRequirement_WinGameStrengthHero = 0x3
            k_eOverworldFortuneRequirement_WinGameIntelligenceHero = 0x4
            k_eOverworldFortuneRequirement_WinGameAgilityHero = 0x5
            k_eOverworldFortuneRequirement_WinGameUniversalHero = 0x6
        class EGuildAuditAction:
            k_EGuildAuditAction_Invalid = 0x0
            k_EGuildAuditAction_GuildCreated = 0x1
            k_EGuildAuditAction_GuildLanguageChanged = 0x2
            k_EGuildAuditAction_GuildFlagsChanged = 0x3
            k_EGuildAuditAction_GuildMemberJoined = 0x5
            k_EGuildAuditAction_GuildMemberLeft = 0x6
            k_EGuildAuditAction_GuildMemberKicked = 0x7
            k_EGuildAuditAction_GuildMemberRoleChanged = 0x8
            k_EGuildAuditAction_GuildLogoChanged = 0x9
            k_EGuildAuditAction_GuildRegionChanged = 0xA
            k_EGuildAuditAction_GuildDescriptionChanged = 0xB
            k_EGuildAuditAction_GuildPrimaryColorChanged = 0xC
            k_EGuildAuditAction_GuildSecondaryColorChanged = 0xD
            k_EGuildAuditAction_GuildPatternChanged = 0xE
            k_EGuildAuditAction_AdminClearedLogo = 0xF
            k_EGuildAuditAction_GuildRequiredRankChanged = 0x10
            k_EGuildAuditAction_GuildMotDChanged = 0x12
            k_EGuildAuditAction_AdminResetName = 0x13
            k_EGuildAuditAction_AdminResetTag = 0x14
            k_EGuildAuditAction_AdminLock = 0x15
            k_EGuildAuditAction_GuildNameChanged = 0x16
            k_EGuildAuditAction_GuildTagChanged = 0x17
            k_EGuildAuditAction_AdminPermitted = 0x18
            k_EGuildAuditAction_AdminBlocked = 0x19
            k_EGuildAuditAction_AdminBannedUser = 0x1A
            k_EGuildAuditAction_AdminExonerated = 0x1B
        class EWeekendTourneyRichPresenceEvent:
            k_EWeekendTourneyRichPresenceEvent_None = 0x0
            k_EWeekendTourneyRichPresenceEvent_StartedMatch = 0x1
            k_EWeekendTourneyRichPresenceEvent_WonMatch = 0x2
            k_EWeekendTourneyRichPresenceEvent_Eliminated = 0x3
        class EArtyTeam:
            k_eNoTeam = 0x0
            k_eYou = 0x1
            k_eThem = 0x2
            k_eNeutral = 0x4
            k_ePlayers = 0x3
            k_eAllTeams = 0xFF
        class ESurvivorsUpgradeRarity:
            RARITY_INVALID = 0xFFFFFFFFFFFFFFFF
            RARITY_COMMON = 0x0
            RARITY_COUNT = 0x1
        class EFishingGameFishCategory:
            k_eFishingGameFishCategory_Invalid = 0x0
            k_eFishingGameFishCategory_Trash = 0x1
            k_eFishingGameFishCategory_Common = 0x2
            k_eFishingGameFishCategory_Uncommon = 0x3
            k_eFishingGameFishCategory_Rare = 0x4
            k_eFishingGameFishCategory_UltraRare = 0x5
        class ESurvivorsAttackTargeting:
            INVALID_TARGET = 0xFFFFFFFFFFFFFFFF
            CLOSEST_TARGET = 0x0
            RANDOM_TARGET_IN_RANGE = 0x1
            STRONGEST_TARGET_IN_RANGE = 0x2
            RANDOM_TARGET_AT_RANGE = 0x3
            PLAYER_FACING = 0x4
        class ECoachTeammateRating:
            k_ECoachTeammateRating_None = 0x0
            k_ECoachTeammateRating_Positive = 0x1
            k_ECoachTeammateRating_Negative = 0x2
            k_ECoachTeammateRating_Abusive = 0x3
        class EShmupEventType:
            k_eShmupEventType_Invalid = 0xFFFFFFFFFFFFFFFF
            k_eShmupEventType_SpawnEnemy = 0x0
            k_eShmupEventType_UI = 0x1
        class DOTAVisualNovelTextColor_t:
            VN_TEXT_COLOR_DEFAULT = 0x0
            VN_TEXT_COLOR_GRAY = 0x1
            VN_TEXT_COLOR_BLUE = 0x2
            VN_TEXT_COLOR_LIGHT_BLUE = 0x3
            VN_TEXT_COLOR_GREEN = 0x4
            VN_TEXT_COLOR_RED = 0x5
            VN_TEXT_COLOR_LIGHT_GRAY = 0x6
            VN_TEXT_COLOR_DARK_GRAY = 0x7
            VN_TEXT_COLOR_DARK_RED = 0x8
            VN_TEXT_COLOR_ORANGE = 0x9
            VN_TEXT_COLOR_WHITE = 0xA
            VN_TEXT_COLOR_PURPLE = 0xB
            VN_TEXT_COLOR_YELLOW = 0xC
        class ECandyShopAuditAction:
            k_ECandyShopAuditAction_Invalid = 0x0
            k_ECandyShopAuditAction_SupportModify = 0x1
            k_ECandyShopAuditAction_PurchaseReward = 0x2
            k_ECandyShopAuditAction_OpenBags = 0x3
            k_ECandyShopAuditAction_RerollRewards = 0x4
            k_ECandyShopAuditAction_DoVariableExchange = 0x5
            k_ECandyShopAuditAction_DoExchange = 0x6
            k_ECandyShopAuditAction_DEPRECATED_EventActionGrantInventorySizeIncrease = 0x7
            k_ECandyShopAuditAction_EventActionGrantRerollChargesIncrease = 0x8
            k_ECandyShopAuditAction_EventActionGrantUpgrade_InventorySize = 0x64
            k_ECandyShopAuditAction_EventActionGrantUpgrade_RewardShelf = 0x65
            k_ECandyShopAuditAction_EventActionGrantUpgrade_ExtraExchangeRecipe = 0x66
        class EShmupPathEventType:
            k_eShmupPathEventType_Invalid = 0xFFFFFFFFFFFFFFFF
            k_eShmupPathEventType_Speed = 0x0
            k_eShmupPathEventType_Shoot = 0x1
            k_eShmupPathEventType_StorePlayerPosition = 0x2
            k_eShmupPathEventType_DestroySelf = 0x3
        class PortraitUnitDirection_t:
            PORTRAIT_UNIT_DIRECTION_INVALID = 0xFFFFFFFFFFFFFFFF
            PORTRAIT_UNIT_DIRECTION_FORWARD = 0x0
            PORTRAIT_UNIT_DIRECTION_BACKWARD = 0x1
        class ESurvivorsBossState:
            INVALID = 0xFFFFFFFFFFFFFFFF
            IDLE = 0x0
            CASTING_BURNING_GROUND = 0x1
            START_CASTING_RADIATE_RAYS = 0x2
            CASTING_RADIATE_RAYS = 0x3
            END_CASTING_RADIATE_RAYS = 0x4
            CASTING_DEMON_PORTALS = 0x5
            START_DESTROYING_TOWER = 0x6
            END_DESTROYING_TOWER = 0x7
            CASTING_MAGIC_MISSILE = 0x8
        class EArtyGraphicsType:
            k_eSprite = 0x0
            k_eAnimatedSprite = 0x1
            k_eProgressBar = 0x2
            k_eUnit = 0x3
            k_eHero = 0x4
            k_eParticle = 0x5
        class DOTA_2013PassportSelectionIndices:
            PP13_SEL_ALLSTAR_PLAYER_0 = 0x0
            PP13_SEL_ALLSTAR_PLAYER_1 = 0x1
            PP13_SEL_ALLSTAR_PLAYER_2 = 0x2
            PP13_SEL_ALLSTAR_PLAYER_3 = 0x3
            PP13_SEL_ALLSTAR_PLAYER_4 = 0x4
            PP13_SEL_ALLSTAR_PLAYER_5 = 0x5
            PP13_SEL_ALLSTAR_PLAYER_6 = 0x6
            PP13_SEL_ALLSTAR_PLAYER_7 = 0x7
            PP13_SEL_ALLSTAR_PLAYER_8 = 0x8
            PP13_SEL_ALLSTAR_PLAYER_9 = 0x9
            PP13_SEL_QUALPRED_WEST_0 = 0xA
            PP13_SEL_QUALPRED_WEST_1 = 0xB
            PP13_SEL_QUALPRED_WEST_2 = 0xC
            PP13_SEL_QUALPRED_WEST_3 = 0xD
            PP13_SEL_QUALPRED_WEST_4 = 0xE
            PP13_SEL_QUALPRED_WEST_5 = 0xF
            PP13_SEL_QUALPRED_WEST_6 = 0x10
            PP13_SEL_QUALPRED_WEST_7 = 0x11
            PP13_SEL_QUALPRED_WEST_8 = 0x12
            PP13_SEL_QUALPRED_WEST_9 = 0x13
            PP13_SEL_QUALPRED_WEST_10 = 0x14
            PP13_SEL_QUALPRED_WEST_11 = 0x15
            PP13_SEL_QUALPRED_WEST_12 = 0x16
            PP13_SEL_QUALPRED_WEST_13 = 0x17
            PP13_SEL_QUALPRED_WEST_14 = 0x18
            PP13_SEL_QUALPRED_EAST_0 = 0x19
            PP13_SEL_QUALPRED_EAST_1 = 0x1A
            PP13_SEL_QUALPRED_EAST_2 = 0x1B
            PP13_SEL_QUALPRED_EAST_3 = 0x1C
            PP13_SEL_QUALPRED_EAST_4 = 0x1D
            PP13_SEL_QUALPRED_EAST_5 = 0x1E
            PP13_SEL_QUALPRED_EAST_6 = 0x1F
            PP13_SEL_QUALPRED_EAST_7 = 0x20
            PP13_SEL_QUALPRED_EAST_8 = 0x21
            PP13_SEL_QUALPRED_EAST_9 = 0x22
            PP13_SEL_QUALPRED_EAST_10 = 0x23
            PP13_SEL_QUALPRED_EAST_11 = 0x24
            PP13_SEL_QUALPRED_EAST_12 = 0x25
            PP13_SEL_QUALPRED_EAST_13 = 0x26
            PP13_SEL_QUALPRED_EAST_14 = 0x27
            PP13_SEL_TEAMCUP_TEAM = 0x28
            PP13_SEL_TEAMCUP_PLAYER = 0x29
            PP13_SEL_TEAMCUP_TEAM_LOCK = 0x2A
            PP13_SEL_TEAMCUP_PLAYER_LOCK = 0x2B
            PP13_SEL_EVENTPRED_0 = 0x2C
            PP13_SEL_EVENTPRED_1 = 0x2D
            PP13_SEL_EVENTPRED_2 = 0x2E
            PP13_SEL_EVENTPRED_3 = 0x2F
            PP13_SEL_EVENTPRED_4 = 0x30
            PP13_SEL_EVENTPRED_5 = 0x31
            PP13_SEL_EVENTPRED_6 = 0x32
            PP13_SEL_EVENTPRED_7 = 0x33
            PP13_SEL_EVENTPRED_8 = 0x34
            PP13_SEL_EVENTPRED_9 = 0x35
            PP13_SEL_EVENTPRED_10 = 0x36
            PP13_SEL_EVENTPRED_11 = 0x37
            PP13_SEL_EVENTPRED_12 = 0x38
            PP13_SEL_EVENTPRED_13 = 0x39
            PP13_SEL_EVENTPRED_14 = 0x3A
            PP13_SEL_EVENTPRED_15 = 0x3B
            PP13_SEL_EVENTPRED_16 = 0x3C
            PP13_SEL_EVENTPRED_17 = 0x3D
            PP13_SEL_EVENTPRED_18 = 0x3E
            PP13_SEL_EVENTPRED_19 = 0x3F
            PP13_SEL_EVENTPRED_20 = 0x40
            PP13_SEL_EVENTPRED_21 = 0x41
            PP13_SEL_EVENTPRED_22 = 0x42
            PP13_SEL_EVENTPRED_23 = 0x43
            PP13_SEL_EVENTPRED_24 = 0x44
            PP13_SEL_EVENTPRED_25 = 0x45
            PP13_SEL_EVENTPRED_26 = 0x46
            PP13_SEL_EVENTPRED_27 = 0x47
            PP13_SEL_EVENTPRED_28 = 0x48
            PP13_SEL_EVENTPRED_29 = 0x49
            PP13_SEL_EVENTPRED_30 = 0x4A
            PP13_SEL_EVENTPRED_31 = 0x4B
            PP13_SEL_EVENTPRED_32 = 0x4C
            PP13_SEL_EVENTPRED_33 = 0x4D
            PP13_SEL_EVENTPRED_34 = 0x4E
            PP13_SEL_EVENTPRED_35 = 0x4F
            PP13_SEL_EVENTPRED_36 = 0x50
            PP13_SEL_EVENTPRED_37 = 0x51
            PP13_SEL_EVENTPRED_38 = 0x52
            PP13_SEL_EVENTPRED_39 = 0x53
            PP13_SEL_EVENTPRED_40 = 0x54
            PP13_SEL_EVENTPRED_41 = 0x55
            PP13_SEL_EVENTPRED_42 = 0x56
            PP13_SEL_EVENTPRED_43 = 0x57
            PP13_SEL_SOLO_0 = 0x58
            PP13_SEL_SOLO_1 = 0x59
            PP13_SEL_SOLO_2 = 0x5A
            PP13_SEL_SOLO_3 = 0x5B
            PP13_SEL_SOLO_4 = 0x5C
            PP13_SEL_SOLO_5 = 0x5D
            PP13_SEL_SOLO_6 = 0x5E
            PP13_SEL_SOLO_7 = 0x5F
        class SteamUGCMatchingUGCType:
            Items = 0x0
            Items_Mtx = 0x1
            Items_ReadyToUse = 0x2
            Collections = 0x3
            Artwork = 0x4
            Videos = 0x5
            Screenshots = 0x6
            AllGuides = 0x7
            WebGuides = 0x8
            IntegratedGuides = 0x9
            UsableInGame = 0xA
            ControllerBindings = 0xB
            GameManagedItems = 0xC
            All = 0xFFFFFFFFFFFFFFFF
        class EStartFindingMatchResult:
            k_EStartFindingMatchResult_Invalid = 0x0
            k_EStartFindingMatchResult_OK = 0x1
            k_EStartFindingMatchResult_AlreadySearching = 0x2
            k_EStartFindingMatchResult_FailGeneric = 0x64
            k_EStartFindingMatchResult_FailedIgnore = 0x65
            k_EStartFindingMatchResult_MatchmakingDisabled = 0x66
            k_EStartFindingMatchResult_RegionOffline = 0x67
            k_EStartFindingMatchResult_MatchmakingCooldown = 0x68
            k_EStartFindingMatchResult_ClientOutOfDate = 0x69
            k_EStartFindingMatchResult_CompetitiveNoLowPriority = 0x6A
            k_EStartFindingMatchResult_CompetitiveNotUnlocked = 0x6B
            k_EStartFindingMatchResult_GameModeNotUnlocked = 0x6C
            k_EStartFindingMatchResult_CompetitiveNotEnoughPlayTime = 0x6D
            k_EStartFindingMatchResult_MissingInitialSkill = 0x6E
            k_EStartFindingMatchResult_CompetitiveRankSpreadTooLarge = 0x6F
            k_EStartFindingMatchResult_MemberAlreadyInLobby = 0x70
            k_EStartFindingMatchResult_MemberNotVACVerified = 0x71
            k_EStartFindingMatchResult_WeekendTourneyBadPartySize = 0x72
            k_EStartFindingMatchResult_WeekendTourneyTeamBuyInTooSmall = 0x73
            k_EStartFindingMatchResult_WeekendTourneyIndividualBuyInTooLarge = 0x74
            k_EStartFindingMatchResult_WeekendTourneyTeamBuyInTooLarge = 0x75
            k_EStartFindingMatchResult_MemberMissingEventOwnership = 0x76
            k_EStartFindingMatchResult_WeekendTourneyNotUnlocked = 0x77
            k_EStartFindingMatchResult_WeekendTourneyRecentParticipation = 0x78
            k_EStartFindingMatchResult_MemberMissingAnchoredPhoneNumber = 0x79
            k_EStartFindingMatchResult_NotMemberOfClan = 0x7A
            k_EStartFindingMatchResult_CoachesChallengeBadPartySize = 0x7B
            k_EStartFindingMatchResult_CoachesChallengeRequirementsNotMet = 0x7C
            k_EStartFindingMatchResult_InvalidRoleSelections = 0x7D
            k_EStartFindingMatchResult_PhoneNumberDiscrepancy = 0x7E
            k_EStartFindingMatchResult_NoQueuePoints = 0x7F
            k_EStartFindingMatchResult_MemberMissingGauntletFlag = 0x80
            k_EStartFindingMatchResult_MemberGauntletTooRecent = 0x81
            k_EStartFindingMatchResult_DifficultyNotUnlocked = 0x82
            k_EStartFindingMatchResult_CoachesNotAllowedInParty = 0x83
            k_EStartFindingMatchResult_MatchmakingBusy = 0x84
            k_EStartFindingMatchResult_SteamChinaBanned = 0x85
            k_EStartFindingMatchResult_SteamChinaInvalidMixedParty = 0x86
            k_EStartFindingMatchResult_RestrictedFromRanked = 0x87
            k_EStartFindingMatchResult_RankPreventsParties = 0x88
            k_EStartFindingMatchResult_RegisteredNameRequired = 0x89
        class EShmupBulletPattern:
            k_eShmupBulletPattern_Invalid = 0xFFFFFFFFFFFFFFFF
            k_eShmupBulletPattern_Radial = 0x0
            k_eShmupBulletPattern_HorizontalLeft = 0x1
            k_eShmupBulletPattern_HorizontalRight = 0x2
            k_eShmupBulletPattern_VerticalInwards = 0x3
            k_eShmupBulletPattern_VerticalOutwards = 0x4
            k_eShmupBulletPattern_FixedDirection = 0x5
            k_eShmupBulletPattern_Player = 0x6
            k_eShmupBulletPattern_PlayerSpreadRandom = 0x7
            k_eShmupBulletPattern_PlayerSpreadEven = 0x8
        class EDOTATriviaAnswerResult:
            k_EDOTATriviaAnswerResult_Success = 0x0
            k_EDOTATriviaAnswerResult_InvalidQuestion = 0x1
            k_EDOTATriviaAnswerResult_InvalidAnswer = 0x2
            k_EDOTATriviaAnswerResult_QuestionLocked = 0x3
            k_EDOTATriviaAnswerResult_AlreadyAnswered = 0x4
            k_EDOTATriviaAnswerResult_TriviaDisabled = 0x5
        class ETournamentNodeState:
            k_ETournamentNodeState_Unknown = 0x0
            k_ETournamentNodeState_Canceled = 0x1
            k_ETournamentNodeState_TeamsNotYetAssigned = 0x2
            k_ETournamentNodeState_InBetweenGames = 0x3
            k_ETournamentNodeState_GameInProgress = 0x4
            k_ETournamentNodeState_A_Won = 0x5
            k_ETournamentNodeState_B_Won = 0x6
            k_ETournamentNodeState_A_WonByForfeit = 0x7
            k_ETournamentNodeState_B_WonByForfeit = 0x8
            k_ETournamentNodeState_A_Bye = 0x9
            k_ETournamentNodeState_A_Abandoned = 0xA
            k_ETournamentNodeState_ServerFailure = 0xB
            k_ETournamentNodeState_A_TimeoutForfeit = 0xC
            k_ETournamentNodeState_A_TimeoutRefund = 0xD
        class EShowcaseHeroPlusFlag:
            k_eShowcaseHeroPlusFlag_None = 0x0
            k_eShowcaseHeroPlusFlag_BadgePosTop = 0x1
            k_eShowcaseHeroPlusFlag_BadgePosBottom = 0x2
            k_eShowcaseHeroPlusFlag_BadgePosLeft = 0x4
            k_eShowcaseHeroPlusFlag_BadgePosRight = 0x8
            k_eShowcaseHeroPlusFlag_ShowRelics = 0x10
        class ETalentContentAssetStatus:
            k_eTalentContentAssetStatus_None = 0x0
            k_eTalentContentAssetStatus_Approved = 0x1
            k_eTalentContentAssetStatus_Rejected = 0x2
        class EUnderDraftResponse:
            k_eInternalError = 0x0
            k_eSuccess = 0x1
            k_eNoGold = 0x2
            k_eInvalidSlot = 0x3
            k_eNoBenchSpace = 0x4
            k_eNoTickets = 0x5
            k_eEventNotOwned = 0x6
            k_eInvalidReward = 0x7
            k_eHasBigReward = 0x8
            k_eNoGCConnection = 0x9
            k_eTooBusy = 0xA
            k_eCantRollBack = 0xB
        class DOTAKeybindTemplate_t:
            DOTA_KEYBIND_TEMPLATE_ARROW = 0x0
            DOTA_KEYBIND_TEMPLATE_WASD = 0x1
            DOTA_KEYBIND_TEMPLATE_LEGACY = 0x2
            DOTA_KEYBIND_TEMPLATE_MMO = 0x3
            DOTA_KEYBIND_TEMPLATE_LOL = 0x4
            DOTA_KEYBIND_TEMPLATE_HON = 0x5
            DOTA_KEYBIND_TEMPLATE_SMITE = 0x6
            DOTA_KEYBIND_TEMPLATE_COUNT = 0x7
        class ECandyShopRewardType:
            k_eCandyShopRewardType_None = 0x0
            k_eCandyShopRewardType_Item = 0x1
            k_eCandyShopRewardType_EventAction = 0x2
            k_eCandyShopRewardType_EventPoints = 0x3
        class ETournamentGameState:
            k_ETournamentGameState_Unknown = 0x0
            k_ETournamentGameState_Canceled = 0x1
            k_ETournamentGameState_Scheduled = 0x2
            k_ETournamentGameState_Active = 0x3
            k_ETournamentGameState_RadVictory = 0x14
            k_ETournamentGameState_DireVictory = 0x15
            k_ETournamentGameState_RadVictoryByForfeit = 0x16
            k_ETournamentGameState_DireVictoryByForfeit = 0x17
            k_ETournamentGameState_ServerFailure = 0x28
            k_ETournamentGameState_NotNeeded = 0x29
        class EOverworldFortuneModifier:
            k_eOverworldFortuneModifier_Invalid = 0x0
            k_eOverworldFortuneModifier_DoubleReward = 0x1
            k_eOverworldFortuneModifier_TripleReward = 0x2
            k_eOverworldFortuneModifier_OneChance = 0x3
            k_eOverworldFortuneModifier_ReceiveAdditionalRandomReward = 0x4
            k_eOverworldFortuneModifier_RerollFortune = 0x5
        class SteamUniverse:
            Invalid = 0x0
            Internal = 0x3
            Dev = 0x4
            Beta = 0x2
            Public = 0x1
        class ESurvivorsEnemySeparationLayer:
            OFF = 0xFFFFFFFFFFFFFFFF
            SMALL = 0x1
            LARGE = 0x2
            ELITE = 0x3
        class EGuildEventAuditAction:
            k_EGuildEventAuditAction_Invalid = 0x0
            k_EGuildEventAuditAction_DevGrant = 0x1
            k_EGuildEventAuditAction_CompleteContract = 0x2
            k_EGuildEventAuditAction_CompleteChallenge = 0x3
            k_EGuildEventAuditAction_CompleteMatch_Winner = 0x4
            k_EGuildEventAuditAction_ChallengeProgress = 0x5
            k_EGuildEventAuditAction_CompleteMatch_Loser = 0x6
            k_EGuildEventAuditAction_WeeklyLeaderboard = 0x7
            k_EGuildEventAuditAction_ManualGrant = 0x8
        class EPrivateCoachingSessionState:
            k_ePrivateCoachingSessionState_Invalid = 0x0
            k_ePrivateCoachingSessionState_SearchingForCoach = 0x1
            k_ePrivateCoachingSessionState_CoachAssigned = 0x2
            k_ePrivateCoachingSessionState_Finished = 0x3
            k_ePrivateCoachingSessionState_Expired = 0x4
            k_ePrivateCoachingSessionState_Abandoned = 0x5
        class ETeamInviteResult:
            TEAM_INVITE_SUCCESS = 0x0
            TEAM_INVITE_FAILURE_INVITE_REJECTED = 0x1
            TEAM_INVITE_FAILURE_INVITE_TIMEOUT = 0x2
            TEAM_INVITE_ERROR_TEAM_AT_MEMBER_LIMIT = 0x3
            TEAM_INVITE_ERROR_TEAM_LOCKED = 0x4
            TEAM_INVITE_ERROR_INVITEE_NOT_AVAILABLE = 0x5
            TEAM_INVITE_ERROR_INVITEE_BUSY = 0x6
            TEAM_INVITE_ERROR_INVITEE_ALREADY_MEMBER = 0x7
            TEAM_INVITE_ERROR_INVITEE_AT_TEAM_LIMIT = 0x8
            TEAM_INVITE_ERROR_INVITEE_INSUFFICIENT_PLAY_TIME = 0x9
            TEAM_INVITE_ERROR_INVITER_INVALID_ACCOUNT_TYPE = 0xA
            TEAM_INVITE_ERROR_INVITER_NOT_ADMIN = 0xB
            TEAM_INVITE_ERROR_INCORRECT_USER_RESPONDED = 0xC
            TEAM_INVITE_ERROR_UNSPECIFIED = 0xD
        class PlayerOrderIssuer_t:
            DOTA_ORDER_ISSUER_SELECTED_UNITS = 0x0
            DOTA_ORDER_ISSUER_CURRENT_UNIT_ONLY = 0x1
            DOTA_ORDER_ISSUER_HERO_ONLY = 0x2
            DOTA_ORDER_ISSUER_PASSED_UNIT_ONLY = 0x3
        class EShowcaseItemFlag_Hero:
            k_eShowcaseItemFlag_Hero_None = 0x0
            k_eShowcaseItemFlag_Hero_ShowPedestal = 0x1
            k_eShowcaseItemFlag_Hero_UseCurrentLoadout = 0x2
            k_eShowcaseItemFlag_Hero_ShowHeroCard = 0x4
            k_eShowcaseItemFlag_Hero_HeroCardHideName = 0x8
            k_eShowcaseItemFlag_Hero_HeroCardUseMovie = 0x10
        class ESurvivorsAreaAttackOrigin:
            INVALID_ORIGIN = 0xFFFFFFFFFFFFFFFF
            PLAYER_ORIGIN = 0x0
            RANDOM_ENEMY_ORIGIN = 0x1
        class PortraitDisplayMode_t:
            PORTRAIT_DISPLAY_MODE_INVALID = 0xFFFFFFFFFFFFFFFF
            PORTRAIT_DISPLAY_MODE_LOADOUT = 0x0
            PORTRAIT_DISPLAY_MODE_LOADOUT_DIRE = 0x1
            PORTRAIT_DISPLAY_MODE_LOADOUT_SMALL = 0x2
            PORTRAIT_DISPLAY_MODE_TREASURE_SMALL = 0x3
        class ETeamFanContentAssetStatus:
            k_eFanContentAssetStatus_None = 0x0
            k_eFanContentAssetStatus_Approved = 0x1
            k_eFanContentAssetStatus_Rejected = 0x2
        class EPurchaseHeroRelicResult:
            k_EPurchaseHeroRelicResult_Success = 0x0
            k_EPurchaseHeroRelicResult_FailedToSend = 0x1
            k_EPurchaseHeroRelicResult_NotEnoughPoints = 0x2
            k_EPurchaseHeroRelicResult_InternalServerError = 0x3
            k_EPurchaseHeroRelicResult_PurchaseNotAllowed = 0x4
            k_EPurchaseHeroRelicResult_InvalidRelic = 0x5
            k_EPurchaseHeroRelicResult_AlreadyOwned = 0x6
            k_EPurchaseHeroRelicResult_InvalidRarity = 0x7
        class ESurvivorsEnemySpawnPositionsLayer:
            INVALID = 0xFFFFFFFFFFFFFFFF
            ENEMY_MAIN = 0x1
            DESTRUCTIBLE_MAIN = 0x2
            ENEMY_BOSS_ROOM = 0x3
        class ESurvivorsEnemyMovementBehavior:
            ENEMY_MOVEMENT_BEHAVIOR_INVALID = 0xFFFFFFFFFFFFFFFF
            ENEMY_MOVEMENT_BEHAVIOR_TRACKING = 0x0
            ENEMY_MOVEMENT_BEHAVIOR_LINEAR = 0x1
            ENEMY_MOVEMENT_BEHAVIOR_LINEAR_SIN_WAVE = 0x2
            ENEMY_MOVEMENT_BEHAVIOR_STATIONARY = 0x3
        class EPrivateCoachingSessionMemberFlag:
            k_EPrivateCoachingSessionMemberFlag_Requester = 0x1
            k_EPrivateCoachingSessionMemberFlag_Coach = 0x2
            k_EPrivateCoachingSessionMemberFlag_LeftSession = 0x4
        class ERoadToTIQuestType:
            k_RoadToTIQuestType_Match = 0x0
            k_RoadToTIQuestType_Player = 0x1
        class OrderQueueBehavior_t:
            DOTA_ORDER_QUEUE_DEFAULT = 0x0
            DOTA_ORDER_QUEUE_NEVER = 0x1
            DOTA_ORDER_QUEUE_ALWAYS = 0x2
        class EDOTADraftTriviaAnswerResult:
            k_EDOTADraftTriviaAnswerResult_Success = 0x0
            k_EDOTADraftTriviaAnswerResult_InvalidMatchID = 0x1
            k_EDOTADraftTriviaAnswerResult_AlreadyAnswered = 0x2
            k_EDOTADraftTriviaAnswerResult_InternalError = 0x3
            k_EDOTADraftTriviaAnswerResult_TriviaDisabled = 0x4
            k_EDOTADraftTriviaAnswerResult_GCDown = 0x5
        class EPlayerCoachMatchFlag:
            k_EPlayerCoachMatchFlag_EligibleForRewards = 0x1
            k_EPlayerCoachMatchFlag_PrivateCoach = 0x2
        class NeutralCampStackPullAlarmType_t:
            DOTA_NEUTRAL_CAMP_STACK_PULL_ALARM_TYPE_STACK = 0x1
            DOTA_NEUTRAL_CAMP_STACK_PULL_ALARM_TYPE_PULL = 0x2
        class SurvivorsAttackIndicatorShape_t:
            k_eSurvivorsShape_Undefined = 0x0
            k_eSurvivorsShape_Circle = 0x1
            k_eSurvivorsShape_Rectangle = 0x2
        class EFlappySkywrathInputAction:
            Jump = 0x0
            Dash = 0x1
            Dive = 0x2
            Glide = 0x3
            COUNT = 0x4
            None_ = 0xFFFFFFFFFFFFFFFF
        class ETalentContentStatus:
            TALENT_CONTENT_STATUS_INVALID = 0x0
            TALENT_CONTENT_STATUS_PENDING = 0x1
            TALENT_CONTENT_STATUS_EVALUATED = 0x2
            TALENT_CONTENT_STATUS_REJECTED = 0x3
            TALENT_CONTENT_STATUS_APPROVED = 0x4
        class EFightingGameActionID:
            INVALID_ACTION_DEFINITION = 0xFFFFFFFFFFFFFFFF
            IDLE_ACTION_DEFINITION = 0x0
            BLOCKSTUN_ACTION_DEFINITION = 0x1
            HITSTUN_ACTION_DEFINITION = 0x2
            KNOCKED_DOWN_ACTION_DEFINITION = 0x3
            DASH_ACTION_DEFINITION = 0x4
            BACKDASH_ACTION_DEFINITION = 0x5
            GUARDBREAK_ACTION_DEFINITION = 0x6
            VICTORY_ACTION_DEFINITION = 0x7
            DEFEAT_ACTION_DEFINITION = 0x8
            JAB_ACTION_DEFINITION = 0x9
            JAB_2_ACTION_DEFINITION = 0xA
            JAB_3_ACTION_DEFINITION = 0xB
            JAB_4_ACTION_DEFINITION = 0xC
            CROSS_ACTION_DEFINITION = 0xD
            CROSS_2_ACTION_DEFINITION = 0xE
            CROSS_3_ACTION_DEFINITION = 0xF
            HEAVY_ACTION_DEFINITION = 0x10
            SWEEP_ACTION_DEFINITION = 0x11
            SWEEP_2_ACTION_DEFINITION = 0x12
            KICK_1_ACTION_DEFINITION = 0x13
            KICK_2_ACTION_DEFINITION = 0x14
            KICK_3_ACTION_DEFINITION = 0x15
            FINISHER_ACTION_DEFINITION = 0x16
            FINISHER_2_ACTION_DEFINITION = 0x17
            PROJECTILE_ACTION_DEFINITION = 0x18
            WALRUS_PUNCH_ACTION_DEFINITION = 0x19
            SWAP_ACTION_DEFINITION = 0x1A
            SWAP_RECOVERY_ACTION_DEFINITION = 0x1B
            QUILLSPRAY_START_ACTION_DEFINITION = 0x1C
            QUILLSPRAY_2_ACTION_DEFINITION = 0x1D
            QUILLSPRAY_3_ACTION_DEFINITION = 0x1E
            QUILLSPRAY_4_ACTION_DEFINITION = 0x1F
            QUILLSPRAY_FINISH_ACTION_DEFINITION = 0x20
            UNLEASH_ACTION_DEFINITION = 0x21
            DASH_STRIKE_ACTION_DEFINITION = 0x22
            DASH_STRIKE_2_ACTION_DEFINITION = 0x23
            STARBREAKER_1_ACTION_DEFINITION = 0x24
            STARBREAKER_2_ACTION_DEFINITION = 0x25
            STARBREAKER_3_ACTION_DEFINITION = 0x26
            LUMINOSITY_ACTION_DEFINITION = 0x27
        class EShowcaseItemFlag:
            k_eShowcaseItemFlag_None = 0x0
            k_eShowcaseItemFlag_FlipHorizontally = 0x1
        class EFightingGameButtonBit:
            kNONE_BIT = 0x0
            kBUTTON_FORWARD_BIT = 0x1
            kBUTTON_BACK_BIT = 0x2
            kBUTTON_DOWN_BIT = 0x4
            kBUTTON_UP_BIT = 0x8
            kBUTTON_ATTACK_BIT = 0x10
            kBUTTON_SPECIAL_BIT = 0x20
        class ETournamentState:
            k_ETournamentState_Unknown = 0x0
            k_ETournamentState_CanceledByAdmin = 0x1
            k_ETournamentState_Completed = 0x2
            k_ETournamentState_Merged = 0x3
            k_ETournamentState_ServerFailure = 0x4
            k_ETournamentState_TeamAbandoned = 0x5
            k_ETournamentState_TeamTimeoutForfeit = 0x6
            k_ETournamentState_TeamTimeoutRefund = 0x7
            k_ETournamentState_ServerFailureGrantedVictory = 0x8
            k_ETournamentState_TeamTimeoutGrantedVictory = 0x9
            k_ETournamentState_InProgress = 0x64
            k_ETournamentState_WaitingToMerge = 0x65
        class PlayerUltimateStateOrTime_t:
            PLAYER_ULTIMATE_STATE_READY = 0x0
            PLAYER_ULTIMATE_STATE_NO_MANA = 0xFFFFFFFFFFFFFFFF
            PLAYER_ULTIMATE_STATE_NOT_LEVELED = 0xFFFFFFFFFFFFFFFE
            PLAYER_ULTIMATE_STATE_HIDDEN = 0xFFFFFFFFFFFFFFFD
        class EArtyGameObjectType:
            k_eTypeObject = 0x0
            k_eTypeShot = 0x1
            k_eTypeTrail = 0x2
            k_eTypeCannon = 0x3
            k_eTypePlayer = 0x4
            k_eTypeEnemy = 0x5
            k_eTypeFX = 0x6
            k_eTypeUI = 0x7
        class ETeamFanContentStatus:
            TEAM_FAN_CONTENT_STATUS_INVALID = 0x0
            TEAM_FAN_CONTENT_STATUS_PENDING = 0x1
            TEAM_FAN_CONTENT_STATUS_EVALUATED = 0x2
        class EOverworldCharacterVisibility:
            CompleteNode = 0x1
            ActiveNode = 0x2
            LockedNode = 0x4
            AlwaysVisible = 0x8
        class EMatch3LevelFlags:
            k_eMatch3LevelFlag_None = 0x0
            k_eMatch3LevelFlag_Boss = 0x1
            k_eMatch3LevelFlag_Friendly = 0x2
        class P2P_Messages:
            p2p_TextMessage = 0x100
            p2p_Voice = 0x101
            p2p_Ping = 0x102
            p2p_VRAvatarPosition = 0x103
            p2p_WatchSynchronization = 0x104
            p2p_FightingGame_GameData = 0x105
            p2p_FightingGame_Connection = 0x106
        class EOverworldNodeFlags:
            Premium = 0x1
            MainQuest = 0x2
            SideQuest = 0x4
            DelayStyles = 0x8
            Shortcut = 0x10
            InvisibleUntilNearby = 0x20
            Secret = 0x40
            FinalNode = 0x80
            Invisible = 0x100
        class DotaCustomUIType_t:
            DOTA_CUSTOM_UI_TYPE_HUD = 0x0
            DOTA_CUSTOM_UI_TYPE_HERO_SELECTION = 0x1
            DOTA_CUSTOM_UI_TYPE_PREGAME_STRATEGY = 0x2
            DOTA_CUSTOM_UI_TYPE_GAME_INFO = 0x3
            DOTA_CUSTOM_UI_TYPE_GAME_SETUP = 0x4
            DOTA_CUSTOM_UI_TYPE_FLYOUT_SCOREBOARD = 0x5
            DOTA_CUSTOM_UI_TYPE_HUD_TOP_BAR = 0x6
            DOTA_CUSTOM_UI_TYPE_END_SCREEN = 0x7
            DOTA_CUSTOM_UI_TYPE_COUNT = 0x8
            DOTA_CUSTOM_UI_TYPE_INVALID = 0xFFFFFFFFFFFFFFFF
        class C_DOTA_BaseNPC_Shop:
            pass
        class CDOTA_Item_SeedsOfSerenity:
            pass
        class C_DOTA_Item_Recipe_Possessed_Mask:
            pass
        class C_DOTA_Item_Arcane_Ring:
            pass
        class C_DOTA_Item_Grove_Bow:
            pass
        class C_DOTA_Item_Revenants_Brooch:
            pass
        class C_DOTA_Ability_Dawnbreaker_BreakOfDawn:
            pass
        class CDOTA_Ability_AbyssalUnderlord_Cancel_DarkRift:
            pass
        class C_DOTA_Ability_Brewmaster_Primal_Split_Fire_Phase:
            pass
        class C_DOTA_Ability_DragonKnight_WyrmsWrath:
            pass
        class CDOTA_Ability_Morphling_EbbAndFlow:
            pass
        class C_DOTA_Ability_Nevermore_Shadowraze1:
            pass
        class CDOTA_Ability_FelBeast_Haunt:
            pass
        class CDOTA_Modifier_AghsFort_Arcanist_Potion:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Luna_8:
            pass
        class CDOTA_Modifier_Special_Bonus_Reincarnation:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Dazzle_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_60:
            pass
        class C_DOTA_Ability_Special_Bonus_Gold_lvl20_l:
            pass
        class CDOTA_Modifier_Hurricane_Pike_Range:
            pass
        class CDOTA_Modifier_Item_Lotus_Orb_Active:
            pass
        class CDOTA_Modifier_Item_EagleHorn:
            pass
        class CDOTA_Modifier_Item_Gauntlets:
            pass
        class CDOTA_Modifier_Magnus_Strength_Of_Joelrak:
            pass
        class CDOTA_Modifier_Wisp_Equilibrium:
            pass
        class CDOTA_Modifier_Lycan_Innate_Spirit_Wolves_Self:
            pass
        class CDOTA_Modifier_Spectre_Arcana:
            pass
        class CDOTA_Modifier_Omniknight_Marty:
            pass
        class CDOTA_Modifier_Sniper_Headshot:
            pass
        class CDOTA_Modifier_Tiny_Tree_Channel_Bonus:
            pass
        class CDOTA_Modifier_Razor_EyeOfTheStorm:
            pass
        class CDOTA_Modifier_Nevermore_Necromastery_Fear:
            pass
        class CDOTA_Modifier_Bane_Nightmare_Invulnerable:
            pass
        class CDOTA_Modifier_AntiMage_Counterspell:
            pass
        class C_DOTAGameManager:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Item_Recipe_Mysterious_Hat:
            pass
        class C_DOTA_Ability_Phoenix_IcarusDive:
            pass
        class CDOTA_Ability_Shadow_Demon_Disruption:
            pass
        class C_DOTA_Ability_Huskar_Berserkers_Blood:
            pass
        class C_DOTA_Ability_Necrolyte_Death_Seeker:
            pass
        class InGamePredictionData_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Modifier_AghsFort_Creature_Phoenix_Sun:
            pass
        class CDOTA_Modifier_Seasonal_TI9_Shovel_BabyRoshan:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Meepo_4:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Riki_1:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lion_10:
            pass
        class CDOTA_Modifier_Eul_Wind_Waker_Thinker:
            pass
        class CDOTA_Modifier_Keen_Optic:
            pass
        class CDOTA_Modifier_Grimstroke_InkCreature_Latched:
            pass
        class CDOTA_Modifier_TrollWarlord_Whirling_Axes_Melee:
            pass
        class CDOTA_Modifier_Treant_NaturesGuise_DamageCooldown:
            pass
        class CDOTA_Modifier_Omniknight_Purification_Recast:
            pass
        class CDOTA_Modifier_DragonKnight_DragonBlood:
            pass
        class CDOTA_Modifier_Morphling_Accumulation:
            pass
        class CDOTA_Modifier_Sven_Warcry:
            pass
        class CDOTA_Modifier_Pet:
            pass
        class CDOTA_BaseNPC_JungleVarmint:
            pass
        class C_DOTAPropCustomTexture:
            pass
        class C_DOTAWearableItem:
            pass
        class C_DOTA_Item_Recipe_DivineRapier:
            pass
        class CDOTA_Ability_Magnus_Magnetic_Horn:
            pass
        class C_DOTA_Ability_Obsidian_Destroyer_Equilibrium:
            pass
        class CDOTA_Item_Furion_Gold_Bag:
            pass
        class C_DOTA_Ability_Tidehunter_LeviathansCatch:
            pass
        class C_DOTA_Ability_Puck_IllusoryOrb:
            pass
        class CDOTA_Modifier_Neutral_Upgrade:
            pass
        class C_DOTA_Ability_HarpyScout_TakeOff:
            pass
        class CDOTA_Ability_Furbolg_Enrage_Damage:
            pass
        class CDOTA_Modifier_Aghsfort_Elemental_Wisp_Tether:
            pass
        class CDOTA_Modifier_AghsFort_RestorativeFlower:
            pass
        class CDOTA_Modifier_Special_Bonus_Spell_Block:
            pass
        class C_DOTA_Ability_Special_Bonus_Respawn_Reduction_25:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Speed_120:
            pass
        class CDOTA_Modifier_Item_UnstableWand_Critter:
            pass
        class CDOTA_Modifier_PrimalBeast_Onslaught_Movement:
            pass
        class CDOTA_Modifier_VoidSpirit_AstralStep_Caster:
            pass
        class CDOTA_Modifier_Tusk_IceShard:
            pass
        class CDOTA_Modifier_Alchemist_Scepter_Bonus_Damage:
            pass
        class CDOTA_Modifier_Alchemist_UnstableConcoction:
            pass
        class CDOTA_Modifier_SpiritBreaker_PlanarPocketAura:
            pass
        class CDOTA_Modifier_Nian_Tail_Swipe_AirTime:
            pass
        class CDOTA_Modifier_Lion_FingerOfDeath:
            pass
        class CDOTA_Modifier_Necrolyte_Sadist:
            pass
        class CDOTA_Modifier_Lich_FrostArmor:
            pass
        class CDOTA_Modifier_Kunkka_Admirals_Rum:
            pass
        class CDOTA_Modifier_Filler_Buff_Icon:
            pass
        class CPulseCell_WaitForCursorsWithTag:
            pass
        class C_DOTA_Unit_Hero_Life_Stealer:
            pass
        class C_DOTA_Ability_Pangolier_LuckyShot:
            pass
        class C_DOTA_Ability_MonkeyKing_UnTransform:
            pass
        class CDOTA_Ability_Winter_Wyvern_EldwurmsEdda:
            pass
        class C_DOTA_Ability_Warlock_Shadow_Word:
            pass
        class CDOTA_Ability_Roshan_GrabThrow:
            pass
        class C_DOTA_Ability_Razor_UnstableCurrent:
            pass
        class CDOTA_Ability_Juggernaut_Innate_Mask_Spin_Crit:
            pass
        class CDOTA_Modifier_BigThunderLizard_Wardrums:
            pass
        class CDOTA_Ability_AghsFort_Ascension_Firefly:
            pass
        class CDOTA_Modifier_Zombie_Berserk:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Queen_Of_Pain_6:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Bane_9:
            pass
        class C_DOTA_Ability_Special_Bonus_Intelligence_14:
            pass
        class CDOTA_Modifier_Mirror_Shield_Delay:
            pass
        class CDOTA_Modifier_Item_Essence_Distiller_Heal:
            pass
        class CDOTA_Modifier_DarkWillow_ShadowRealm_FadeTime:
            pass
        class CDOTA_Modifier_MonkeyKing_FurArmy_SoldierHidden:
            pass
        class CDOTA_Modifier_MonkeyKing_Strike_Stun:
            pass
        class CDOTA_Modifier_Phoenix_Sun:
            pass
        class CDOTA_Modifier_EarthSpirit_RollingBoulder_Caster:
            pass
        class CDOTA_Modifier_EmberSpirit_SlightOfFist_ChargeCounter:
            pass
        class CDOTA_Modifier_Disruptor_Glimpse_Thinker:
            pass
        class CDOTA_Modifier_Spectre_SpectralDaggerInPath:
            pass
        class CDOTA_Modifier_Life_Stealer_Ghoul_Frenzy:
            pass
        class CDOTA_Modifier_FacelessVoid_Chronosphere_Freeze:
            pass
        class CDOTA_Modifier_Earthshaker_Fissure_Shard_Pathing:
            pass
        class CDOTA_Modifier_AncientApparition_ColdFeet_Freeze:
            pass
        class C_SceneEntity__QueuedEvents_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Unit_Hero_Bane:
            pass
        class C_DOTA_NPC_Lantern:
            pass
        class C_DOTA_Item_PocketRoshan:
            pass
        class C_DOTA_Ability_Rubick_Empty2:
            pass
        class C_DOTA_Ability_Invoker_Invoke:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_ArenaOfBloodHPRegen:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Shadow_Demon_9:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Invoker_5:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lion_5:
            pass
        class CDOTA_Modifier_Special_Bonus_Attack_Speed:
            pass
        class C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_8:
            pass
        class CDOTA_Modifier_Partisans_Brand:
            pass
        class CDOTA_Modifier_AghanimsShard_WaitForUpgradeSelected:
            pass
        class CDOTA_Modifier_Item_AsceticCapBuff:
            pass
        class CDOTA_Modifier_Item_Mango_Tree:
            pass
        class CDOTA_Modifier_Magnataur_ReversePolarity_Stats:
            pass
        class CDOTA_Modifier_Lycan_Fear:
            pass
        class CDOTA_Modifier_Beastmaster_Hawk_Perch_Perching:
            pass
        class CDOTA_Modifier_Windrunner_ShackleShot_SelfDamageBuff:
            pass
        class CDOTA_Modifier_StormSpirit_OverloadSlow:
            pass
        class CDOTA_Modifier_Sven_Variant_Shieldbreaker:
            pass
        class CDOTA_Modifier_Juggernaut_Vaulted_Strike:
            pass
        class C_DOTA_Unit_Hero_Winter_Wyvern:
            pass
        class C_DOTA_Unit_Hero_EmberSpirit:
            pass
        class C_DOTA_Unit_Hero_TemplarAssassin:
            pass
        class C_DOTA_Item_Sorcerers_Staff:
            pass
        class C_DOTA_Item_Recipe_Essence_Distiller:
            pass
        class C_DOTA_Item_NetherRaiment:
            pass
        class CDOTA_Item_Recipe_BootsOfTravel_2:
            pass
        class CDOTA_Ability_Centaur_Work_Horse:
            pass
        class C_DOTA_Ability_Brewmaster_FireBrewling_FireStance:
            pass
        class C_DOTA_Ability_Warlock_Upheaval:
            pass
        class C_DOTA_Ability_Frogmen_CongregationOfTheDeep:
            pass
        class CDOTA_Modifier_Creep_Piercing:
            pass
        class CDOTA_Modifier_ForestTrollHighPriest_ManaAura:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Kunkka_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Amplify_8:
            pass
        class FowBlocker_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Modifier_Snapfire_Magma_Burn_Slow:
            pass
        class CDOTA_ModifierTreant_Innate_Attack_Damage:
            pass
        class CDOTA_Modifier_Lycan_Apex_Predator_Aura:
            pass
        class CDOTA_Modifier_Furion_Arboreal_Might_Armor:
            pass
        class CDOTA_Modifier_Viper_ViperStrike_Debuff:
            pass
        class CDOTA_Modifier_Miniboss_Minion_FollowingMovement:
            pass
        class CDOTA_Modifier_Ursa_Enrage:
            pass
        class CDOTA_Modifier_Tinker_Eureka:
            pass
        class CDOTA_Modifier_LegacyChargeCounter:
            pass
        class CDOTA_BaseNPC_Seasonal_TI9_Balloon:
            pass
        class C_DOTA_Item_Enhancement_Manic:
            pass
        class C_DOTA_Item_Enhancement_Keen_Eyed:
            pass
        class CDOTA_Ability_EarthSpirit_Petrify:
            pass
        class C_DOTA_Ability_EmberSpirit_SleightOfFist:
            pass
        class C_DOTA_Ability_Chaos_Knight_Phantasmagoria:
            pass
        class CDOTA_Ability_Miniboss_Minion_Unyielding_Shield:
            pass
        class CDOTA_Ability_Roshan_SpellBlock:
            pass
        class C_DOTA_Ability_Tidehunter_Ravage:
            pass
        class C_DOTA_Ability_StormSpirit_ElectricVortex:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Underlord_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Evasion_12:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Base_Damage_50:
            pass
        class CDOTA_Modifier_Item_EyeOfTheVizier:
            pass
        class CDOTA_Modifier_Item_Chipped_Vest:
            pass
        class CDOTA_Modifier_Item_Ward_Maker:
            pass
        class CDOTA_Modifier_Pangolier_Swashbuckle_Attack:
            pass
        class CDOTA_Modifier_MonkeyKing_FurArmy_Soldier:
            pass
        class CDOTA_Modifier_Terrorblade_Metamorphosis_Transform_Aura_Applier:
            pass
        class CDOTA_Modifier_Bristleback_QuillSprayStack:
            pass
        class CDOTA_Modifier_Treant_LivingArmorPermanent:
            pass
        class CDOTA_Modifier_Omniknight_GuardianAngel:
            pass
        class CDOTA_Modifier_Dark_Seer_Innate_MasterTactician:
            pass
        class CDOTA_Modifier_Lion_Impale:
            pass
        class CDOTA_Modifier_Lich_Sinister_Gaze:
            pass
        class CBaseTriggerAPI:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Item_Recipe_Cloak_Of_Flames:
            pass
        class C_DOTA_Item_Chipped_Vest:
            pass
        class CDOTA_Item_Pavise:
            pass
        class CDOTA_Item_Recipe_Guardian_Greaves:
            pass
        class C_DOTA_Item_Recipe_Boots_Of_Bearing:
            pass
        class C_DOTA_Item_Shivas_Guard:
            pass
        class C_DOTA_Item_RobeOfMagi:
            pass
        class C_DOTA_Item_Recipe_BootsOfTravel:
            pass
        class C_DOTA_Ability_Visage_GravekeepersCloak:
            pass
        class C_DOTA_Ability_LoneDruid_SpiritBear:
            pass
        class C_DOTA_Ability_Brewmaster_Pulverize:
            pass
        class C_DOTA_Ability_Kunkka_Tidal_Wave:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_20:
            pass
        class C_DOTABotChallengeGameMode:
            pass
        class CDOTA_Modifier_Item_Urn_Heal:
            pass
        class CDOTA_Modifier_Primalbeast_Trample:
            pass
        class CDOTA_Modifier_VoidSpirit_AstralStep_ChargeCounter:
            pass
        class CDOTA_Modifier_KeeperOfTheLight_Will_O_Wisp:
            pass
        class CDOTA_Modifier_Brewmaster_Void_Brawler_Slow:
            pass
        class CDOTA_Modifier_DoomBringer_Doom_Aura_Enemy:
            pass
        class CDOTA_Modifier_Rattletrap_Overheated:
            pass
        class CDOTA_Modifier_FacelessVoid_Chronosphere_SelfBuff:
            pass
        class CDOTA_Modifier_FacelessVoid_TimeDilation_Distortion_Aura:
            pass
        class CDOTA_Modifier_FacelessVoid_Backtrack:
            pass
        class CDOTA_Modifier_DrowRanger_ArcanaKill_Delay:
            pass
        class CDOTA_Modifier_Sand_King_Shard:
            pass
        class CDOTA_Modifier_BattleCupEffigy:
            pass
        class C_DOTA_BaseNPC_SDKTower:
            pass
        class CDOTA_Item_Stonefeather_Satchel:
            pass
        class C_DOTA_Item_Doubloon:
            pass
        class C_DOTA_Item_Recipe_Mage_Slayer:
            pass
        class C_DOTA_Ability_Ringmaster_EmptySouvenir:
            pass
        class C_DOTA_Ability_Invoker_Quas:
            pass
        class CDOTA_Ability_Life_Stealer_Control:
            pass
        class C_DOTA_Ability_Nevermore_Presence:
            pass
        class CDOTA_Modifier_Greevil_Miniboss_Green_Overgrowth:
            pass
        class CDOTA_Modifier_AghsFort_Firefly_Burn:
            pass
        class C_DOTA_Ability_Special_Bonus_Armor_3:
            pass
        class CDOTA_Modifier_Item_Vampire_Fangs:
            pass
        class CDOTA_Modifier_Item_Lotus_Orb_ChannelCheck:
            pass
        class CDOTA_Modifier_Item_Pipe_Aura:
            pass
        class CDOTA_Modifier_item_Splintmail:
            pass
        class CDOTA_Modifier_Snapfire_Scatterblast_Slow:
            pass
        class CDOTA_Modifier_Pangolier_ShieldCrash_Jump:
            pass
        class CDOTA_Modifier_Banana:
            pass
        class CDOTA_Modifier_Skywrath_Mage_Staff_Of_The_Scion:
            pass
        class CDOTA_Modifier_Jakiro_LiquidFire_Burn:
            pass
        class CDOTA_Modifier_Jakiro_DualBreath_Burn:
            pass
        class CDOTA_Modifier_Leshrac_Decrepify_Aura:
            pass
        class CDOTA_Modifier_Ursa_Overpower:
            pass
        class CDOTA_Modifier_BookOfAgility:
            pass
        class CDOTA_Modifier_Buyback_Gold_Penalty:
            pass
        class CDOTA_Unit_Hero_Centaur:
            pass
        class C_DOTA_Unit_Hero_Pudge:
            pass
        class C_DOTA_BaseNPC_HoldoutTower_HeavySlow:
            pass
        class C_DOTA_BaseNPC_Healer:
            pass
        class CDOTA_Item_Harmonizer:
            pass
        class CDOTA_Item_Guardian_Greaves:
            pass
        class CDOTA_Ability_Techies_StickyBomb:
            pass
        class C_DOTA_Ability_Rubick_Might_And_Magus:
            pass
        class C_DOTA_Ability_Spectre_Spectral:
            pass
        class C_DOTA_Ability_Rattletrap_Innate_Cogs_Small_Spread:
            pass
        class C_DOTA_Ability_SandKing_BurrowStrike:
            pass
        class CDOTA_Modifier_Mutation_StationaryDamageReduction_Aura:
            pass
        class CDOTA_Modifier_AghsFort_Ascension_Silence:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Grimstroke_7:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Undying:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_13:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Speed_140:
            pass
        class C_DOTA_DataDire:
            pass
        class CDOTA_Modifier_Necronomicon_Warrior_ManaBurn:
            pass
        class CDOTA_Modifier_Item_Crimson_Guard:
            pass
        class CDOTA_Modifier_Manta:
            pass
        class CDOTA_Modifier_Slark_Depth_Shroud:
            pass
        class CDOTA_Modifier_Viper_CorrosiveSkin:
            pass
        class CDOTA_Modifier_Pudge_Dismember:
            pass
        class C_SpeechBubbleInfo:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_EnvWindShared:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_SkyCamera:
            pass
        class CPulseCell_Base:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class C_DOTA_Item_Timeless_Relic:
            pass
        class CDOTA_Ability_Magnataur_Skewer:
            pass
        class C_DOTA_Ability_NagaSiren_SongOfTheSiren:
            pass
        class C_DOTA_Ability_Undying_Tombstone_Zombie_DeathStrike:
            pass
        class C_DOTA_Ability_LoneDruid_TrueForm_BattleCry:
            pass
        class C_DOTA_Ability_LoneDruid_SpiritBear_Defender:
            pass
        class CDOTA_Ability_DarkSeer_Surge:
            pass
        class C_DOTA_Ability_QueenOfPain_SonicWave:
            pass
        class C_DOTA_Ability_Tinker_Keen_Teleport:
            pass
        class C_DOTA_Ability_Zuus_ArcLightning:
            pass
        class C_FuncRotating:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Spirit_Breaker_4:
            pass
        class CDOTA_Modifier_Special_Bonus_HP:
            pass
        class C_DOTA_Ability_Special_Bonus_Night_Vision_600:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_450:
            pass
        class CDOTA_Modifier_Roshans_Banner_HP:
            pass
        class CDOTA_Modifier_Item_Veil_Of_Discord:
            pass
        class CDOTA_Modifier_Item_BlinkDagger:
            pass
        class CDOTA_Modifier_Invoker_Quas_Intrinsic:
            pass
        class CDOTA_Modifier_Life_Stealer_Infest:
            pass
        class CDOTA_Modifier_Viper_ViperStrike_Slow:
            pass
        class CDOTA_Modifier_Attached_Unit:
            pass
        class CDOTA_Modifier_AncientApparition_IceBlast:
            pass
        class C_SoundOpvarSetPointBase:
            pass
        class CDOTA_Unit_Hero_Marci:
            pass
        class CDOTA_Item_Nether_Shawl:
            pass
        class C_DOTA_Item_Recipe_Veil_Of_Discord:
            pass
        class CDOTA_Ability_Magnataur_Empower:
            pass
        class C_DOTA_Ability_Nyx_Assassin_SpikedCarapace:
            pass
        class C_DOTA_Ability_Jakiro_IcePath:
            pass
        class C_DOTA_Ability_Life_Stealer_Consume:
            pass
        class CDOTA_Modifier_Greevil_Miniboss_Blue_IceVortex:
            pass
        class C_EnvCubemapFog:
            pass
        class C_DOTA_Ability_Special_Bonus_Lifesteal_15:
            pass
        class CDOTA_Modifier_Item_Foragers_Stats:
            pass
        class CDOTA_Modifier_Item_SuperArcane_Blink:
            pass
        class CDOTA_Modifier_Item_Apex:
            pass
        class CDOTA_Modifier_Item_RingOfAquila_Aura:
            pass
        class CDOTA_Modifier_Item_DustofAppearance:
            pass
        class CDOTA_Modifier_Phoenix_SunRay:
            pass
        class CDOTA_Modifier_EarthSpirit_Geogmagnetic_Grip_Debuff:
            pass
        class CDOTA_Modifier_Centaur_Mount_Toss:
            pass
        class CDOTA_Modifier_Medusa_StoneGaze_Facing:
            pass
        class CDOTA_Modifier_NagaSiren_RipTide_Passive:
            pass
        class CDOTA_Modifier_Disruptor_ElectromagneticRepulsion_Passive:
            pass
        class C_EnvParticleGlow:
            pass
        class CDOTA_Item_Recipe_Swift_Blink:
            pass
        class C_DOTA_Item_Fluffy_Hat:
            pass
        class C_DOTA_Item_VoidwalkerScythe:
            pass
        class C_DOTA_Ability_Kez_Shodo_Sai_Parry_Cancel:
            pass
        class C_DOTA_Ability_Phoenix_IcarusDiveStop:
            pass
        class CDOTA_Ability_Disruptor_ElectromagneticRepulsion:
            pass
        class C_DOTA_Ability_Invoker_ForgeSpirit:
            pass
        class C_DOTA_Jungle_Varmint_Dive:
            pass
        class CDOTA_Ability_Creature_Flamestrike:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lion_11:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Drow_Ranger_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Exp_Boost_15:
            pass
        class C_FoWRevealerEntity:
            pass
        class CDOTA_Modifier_Item_Nemesis_Curse_Debuff:
            pass
        class CDOTA_Modifier_Item_MaskOfDispair:
            pass
        class CDOTA_Modifier_Item_LesserCritical:
            pass
        class CDOTA_Modifier_Arc_Warden_Magnetic_Field_Thinker_Rune_Magnet:
            pass
        class CDOTA_Modifier_Elder_Titan_EarthSplitter:
            pass
        class CDOTA_Modifier_Abaddon_Frostmourne:
            pass
        class CDOTA_Modifier_Wisp_Overcharge:
            pass
        class CDOTA_Modifier_Nyx_Assassin_Vendetta:
            pass
        class CDOTA_Modifier_Brewmaster_SpellImmunity:
            pass
        class CDOTA_Modifier_Brewmaster_CinderBrew_Animation:
            pass
        class CDOTA_Modifier_Broodmother_Tough:
            pass
        class CDOTA_Modifier_PhantomAssassin_BlurActive:
            pass
        class CDOTA_Modifier_Beastmaster_DrumsOfSlom:
            pass
        class CDOTA_Modifier_Roshan_Grab_Thrown:
            pass
        class CDOTA_Modifier_Riki_SmokeScreenThinker:
            pass
        class CDOTA_Modifier_VengefulSpirit_WaveOfTerror:
            pass
        class CDOTA_Modifier_FillerThinker:
            pass
        class CDOTA_Modifier_Stacking_Multiple_Buff_Base:
            pass
        class C_EconEntity__AttachedModelData_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CPulse_ResumePoint:
            pass
        class C_NextBotCombatCharacter:
            pass
        class CTriggerFan:
            pass
        class CDOTA_Item_Apex:
            pass
        class C_DOTA_Ability_Largo_Frogstomp:
            pass
        class C_DOTA_Ability_Ringmaster_DarkCarnivalSouvenirs:
            pass
        class CDOTA_Ability_Snapfire_Boomstick:
            pass
        class CDOTA_Ability_Mars_Dauntless:
            pass
        class C_DOTA_Ability_Winter_Wyvern_Essence_Of_The_Blueheart:
            pass
        class CDOTA_Ability_Elder_Titan_EchoStomp_Spirit:
            pass
        class C_DOTA_Ability_KeeperOfTheLight_Recall:
            pass
        class CDOTA_Modifier_Seasonal_TI11_CongaLineSlow:
            pass
        class CDOTA_AghsFort_BossPreview:
            pass
        class C_DOTA_Aghsfort_Ability_CrystalMaiden_CrystalNova:
            pass
        class CDOTA_Modifier_Seasonal_PartyHat:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Base_Damage_35:
            pass
        class CDOTA_Modifier_Item_Enhancement_Feverish:
            pass
        class CDOTA_Modifier_Item_Harpoon_EchoSabre_Component:
            pass
        class CDOTA_Modifier_Item_Mind_Breaker:
            pass
        class CDOTA_Modifier_Item_Faerie_Fire:
            pass
        class CDOTA_Modifier_MjollnirChain:
            pass
        class CDOTA_Modifier_InvisibilityEdge_WindWalk:
            pass
        class CDOTA_Modifier_Item_Perseverance:
            pass
        class CDOTA_Modifier_Kez_SwitchWeapon_Aghs:
            pass
        class CDOTA_Modifier_Shredder_Flamethrower:
            pass
        class CDOTA_Modifier_Rubick_Curiosity:
            pass
        class CDOTA_Modifier_NetherStrike_GreaterBash_KnockbackAmp:
            pass
        class CDOTA_Modifier_Ursa_Damage_Increase:
            pass
        class CDOTA_Modifier_CrystalMaiden_FreezingField:
            pass
        class C_DOTA_Unit_Hero_Lich:
            pass
        class C_DOTASceneEntity:
            pass
        class C_DOTA_Item_TiaraOfSelemene:
            pass
        class CDOTA_Item_Recipe_HydrasBreath:
            pass
        class C_DOTA_Item_VoidStone:
            pass
        class C_DOTA_Ability_Medusa_Cold_Blooded:
            pass
        class C_DOTA_Ability_Treant_LivingArmor:
            pass
        class C_DOTA_Item_Vermillion_Robe:
            pass
        class CDOTA_Item_Mask_Spin_Crit:
            pass
        class C_DOTA_Ability_Greevil_Miniboss_Sight:
            pass
        class C_FuncElectrifiedVolume:
            pass
        class C_DOTASpectatorGraphManager:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lion_2:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lich_5:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Tiny_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Pangolier_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Exp_Boost_25:
            pass
        class C_DOTA_Ability_Special_Bonus_Agility_81:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_Regen_2:
            pass
        class CDOTA_Modifier_Item_RefresherShard:
            pass
        class CDOTA_Modifier_Kez_Katana_Shard_Debuff:
            pass
        class CDOTA_Modifier_AbyssalUnderlord_AtrophyAura_Permanent_HeroDmgBuff:
            pass
        class CDOTA_Modifier_Slark_EssenceShift_Permanent_Buff:
            pass
        class CDOTA_Modifier_Nyx_Assassin_Vendetta_Break:
            pass
        class CDOTA_Modifier_Undying_FleshGolem_Slow:
            pass
        class CDOTA_Modifier_Invoker_EMP:
            pass
        class CDOTA_Modifier_Jakiro_Macropyre_Burn:
            pass
        class CDOTA_Modifier_Enchantress_Enchant_Intrinsic_Aura:
            pass
        class CDOTA_Modifier_PhantomAssassin_StiflingDagger_Recast:
            pass
        class CDOTA_Modifier_Item_GrisGris_GoldCounter:
            pass
        class CDOTA_Modifier_Tidehunter_AnchorSmash:
            pass
        class CDOTA_Modifier_Sven_Stormbolt_Hide:
            pass
        class CDOTA_Modifier_Lua_Horizontal_Motion:
            pass
        class C_DOTA_NPC_BroodmotherStickyWeb:
            pass
        class C_DOTA_Item_Arcane_Blink:
            pass
        class CDOTA_Item_RefresherOrb_Shard:
            pass
        class C_DOTA_Item_Recipe_GreaterCritical:
            pass
        class C_DOTA_Ability_Largo_Encore:
            pass
        class C_DOTA_Ability_Bristleback_Hairball:
            pass
        class C_DOTA_Ability_Enchantress_Little_Friends:
            pass
        class C_DOTA_Ability_Lich_ChainFrost:
            pass
        class CDOTA_Ability_Kunkka_Admirals_Rum:
            pass
        class CDOTA_Modifier_SatyrTrickster_Purge:
            pass
        class C_EnvVolumetricFogVolume:
            pass
        class CDOTA_Modifier_Special_Bonus_Status_Resistance:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_50:
            pass
        class CDOTA_Modifier_Item_Harpoon:
            pass
        class CDOTA_Modifier_Yasha_And_Kaya_Maim:
            pass
        class CDOTA_Modifier_Kez_RaptorDance_Immune:
            pass
        class CDOTA_Modifier_Kez_Echo_Slash_Echo_Damage:
            pass
        class CDOTA_Modifier_Phoenix_IcarusDive:
            pass
        class CDOTA_Modifier_Legion_Commander_OverwhelmingOdds:
            pass
        class CDOTA_Modifier_Mangataur_Shockwave_Intrinsic:
            pass
        class CDOTA_Modifier_Brewmaster_DrunkenHaze:
            pass
        class CDOTA_Modifier_Batrider_FlamingLasso:
            pass
        class CDOTA_Modifier_Night_Stalker_Heart_Of_Darkness:
            pass
        class CDOTA_Modifier_Warlock_FatalBonds:
            pass
        class CDOTA_Modifier_Roshan_Grab_Self:
            pass
        class CDOTA_Modifier_Axe_Culling_Blade_Charge:
            pass
        class CDOTA_Modifier_Axe_BattleHunger:
            pass
        class CDOTA_Modifier_Juggernaut_Bladeform_Linger:
            pass
        class CDOTA_Modifier_Rune_SuperArcane:
            pass
        class CIngameEvent_Diretide2020:
            pass
        class CPulseCell_PlaySequence:
            pass
        class C_DOTA_Item_Keen_Optic:
            pass
        class C_DOTA_Item_BladeOfAlacrity:
            pass
        class C_DOTA_Ability_Treant_Innate_Attack_Damage:
            pass
        class CDOTA_Ability_Shadow_Demon_Shadow_Poison_Release:
            pass
        class C_DOTA_Ability_Invoker_ColdSnap:
            pass
        class CDOTA_Ability_Broodmother_StickySnare:
            pass
        class C_DOTA_Ability_Lifestealer_CorpseEater:
            pass
        class C_DOTA_Ability_Necrolyte_Death_Pulse:
            pass
        class CDOTA_Modifier_PolarFurbolgUrsaWarrior_ThunderClap:
            pass
        class CDOTA_Modifier_Boss_DarkWillow_Bedlam:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Magnus:
            pass
        class CDOTA_Modifier_Special_Bonus_Unique_Treant_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Juggernaut_2:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_Regen_8:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_700:
            pass
        class CDOTA_Modifier_Item_Bottomless_Chalice:
            pass
        class CDOTA_Modifier_Item_Blood_Grenade_Debuff:
            pass
        class CDOTA_Modifier_Item_Blood_Grenade:
            pass
        class CDOTA_Modifier_MonkeyKing_CloudRunStart:
            pass
        class CDOTA_Modifier_Banana_Knockback:
            pass
        class CDOTA_Modifier_EmberSpirit_FlameGuard:
            pass
        class CDOTA_Modifier_Techies_SnareTrap:
            pass
        class CDOTA_Modifier_Tidehunter_SmashAttack:
            pass
        class CDOTA_Modifier_Morphling_Replicate_MorphedIllusionsEffect:
            pass
        class CDOTA_Modifier_Fountain_Truesight_Aura:
            pass
        class C_BaseEntityAPI:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_PortraitHero:
            pass
        class C_DOTA_Item_Bloodthorn:
            pass
        class CDOTA_Item_Recipe_GlimmerCape:
            pass
        class C_DOTA_Ability_PrimalBeast_Uproar:
            pass
        class C_DOTA_Ability_Rubick_Telekinesis:
            pass
        class C_DOTA_Ability_Dazzle_Weave:
            pass
        class C_DOTA_Ability_Greevil_Miniboss_Orange_DragonSlave:
            pass
        class C_DOTA_Ability_KoboldTaskmaster_SpeedAura:
            pass
        class CDOTA_Ability_Seasonal_TI11_CongaLine:
            pass
        class CDOTA_Modifier_Wave_Blast_Knockback:
            pass
        class CDOTA_Modifier_Ascension_Meteoric_Land:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Death_Prophet_5:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Tinker_7:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Tiny_3:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Sand_King_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Ancient_Apparition_6:
            pass
        class C_DOTA_Ability_Special_Bonus_Agility_12:
            pass
        class CDOTA_Modifier_ManaDraught_Regeneration:
            pass
        class CDOTA_Modifier_Gungnir_Debuff:
            pass
        class CDOTA_Modifier_Item_SentryWard:
            pass
        class CDOTA_Modifier_AbyssalUnderlord_Underling_Spawn_Thinker:
            pass
        class CDOTA_Modifier_Magnataur_Shockwave_Pull:
            pass
        class CDOTA_Modifier_Slark_SaltwaterShiv:
            pass
        class CDOTA_Modifier_BountyHunter_WindWalk:
            pass
        class CDOTA_Modifier_Broodmother_SpawnSpiderite:
            pass
        class CDOTA_Modifier_PhantomLancer_Juxtapose_Cache:
            pass
        class CDOTA_Unit_Side_Gunner:
            pass
        class C_DOTA_PortraitBuilding:
            pass
        class C_DOTA_Item_RuneSpawner_XP:
            pass
        class C_DOTA_Item_Recipe_Falcon_Blade:
            pass
        class CDOTA_Item_Recipe_Nether_Shawl:
            pass
        class C_DOTA_Item_Spell_Prism:
            pass
        class C_DOTA_Item_Quarterstaff:
            pass
        class C_DOTA_Ability_Rattletrap_PowerCogs:
            pass
        class CDOTA_Modifier_Greevil_Miniboss_Blue_ColdFeet:
            pass
        class CDOTA_Modifier_Mutation_Spellcast:
            pass
        class C_DOTASpectatorGraphManagerProxy:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Bane_3:
            pass
        class CDOTA_Modifier_Special_Bonus_Attributes:
            pass
        class CDOTA_Modifier_Item_Faded_Broach:
            pass
        class CDOTA_Modifier_Item_Cornucopia:
            pass
        class CDOTA_Modifier_Item_Bloodstone_Active:
            pass
        class CDOTA_Modifier_Special_Mars_Spear_Burning_Trail_Thinker:
            pass
        class CDOTA_Modifier_EmberSpirit_Immolation_Debuff:
            pass
        class CDOTA_Modifier_NagaSiren_SongOfTheSiren_Aura:
            pass
        class CDOTA_Modifier_Treant_NaturesGuise_NearTreeDisplay:
            pass
        class CDOTA_Modifier_Chaos_Knight_Phantasmagoria:
            pass
        class CDOTA_Modifier_PhantomLancer_JuxtaposeIllusion:
            pass
        class CDOTA_Modifier_Bloodseeker_Thirst:
            pass
        class CDOTA_Modifier_Shadowraze_Counter:
            pass
        class CDOTA_Modifier_Filler_LastDitch:
            pass
        class CPulseCell_LerpCameraSettings:
            pass
        class C_DOTA_Unit_Hero_MonkeyKing:
            pass
        class C_DOTA_BaseNPC_ShadowShaman_SerpentWard:
            pass
        class C_DOTA_Item_Recipe_Butterfly:
            pass
        class CDOTA_Ability_Techies_RemoteMines_SelfDetonate:
            pass
        class C_DOTA_Ability_Bristleback_QuillSpray:
            pass
        class C_DOTA_Ability_TrollWarlord_Innate_Masterful:
            pass
        class C_DOTA_Ability_LoneDruid_SpiritBear_Fetch:
            pass
        class C_DOTA_Ability_LoneDruid_SpiritLink:
            pass
        class C_DOTA_Ability_Luna_MoonGlaive:
            pass
        class C_DOTA_Ability_Beastmaster_Mark_Of_The_Beast:
            pass
        class CDOTA_Ability_AncientApparition_IceAge:
            pass
        class C_DOTA_Ability_BigThunderLizard_Slam:
            pass
        class CPointOffScreenIndicatorUi:
            pass
        class CDOTA_Modifier_Creature_Flamestrike_Ground:
            pass
        class CDOTA_Modifier_LootDrop_Thinker:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Hoodwink_AcornShotDamage:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Riki_6:
            pass
        class CDOTA_Modifier_MetamorphicMandible_Active:
            pass
        class CDOTA_Modifier_Aghanims_Shard:
            pass
        class CDOTA_Modifier_Necronomicon_Archer_Purge:
            pass
        class CDOTA_Modifier_Largo_Song_Speed_Burst_SlowRes:
            pass
        class CDOTA_Modifier_Dawnbreaker_Converge:
            pass
        class CDOTA_Modifier_Mars_Immovable:
            pass
        class CDOTA_Modifier_Medusa_VenomedVolley:
            pass
        class CDOTA_Modifier_Slark_ShadowDance_Visual:
            pass
        class CDOTA_Modifier_Wisp_Tether_Stun_Tracker:
            pass
        class CDOTA_Modifier_Shadow_Demon_Disseminate_HPLoss:
            pass
        class CDOTA_Modifier_Jakiro_DualBreath_Thinker:
            pass
        class CDOTA_Modifier_Rattletrap_Hookshot:
            pass
        class CDOTA_Modifier_Miniboss_Minion_Deflecting_Shield_Buff:
            pass
        class CDOTA_Modifier_Illusion:
            pass
        class CDOTA_Unit_Falconers_Glove_Hawk:
            pass
        class CDOTA_BaseNPC_Effigy_AghsFort:
            pass
        class C_DOTA_Item_Dynamite_Jacket:
            pass
        class CDOTA_Item_Paladin_Sword:
            pass
        class C_DOTA_Ability_Dawnbreaker_Fire_Wreath:
            pass
        class C_DOTA_Ability_Troll_Warlord_Rampage:
            pass
        class C_DOTA_Ability_Meepo_DividedWeStand:
            pass
        class C_DOTA_Ability_Brewmaster_DrunkenBrawler:
            pass
        class C_DOTA_Ability_Leshrac_Split_Earth:
            pass
        class C_DOTA_Ability_DeathProphet_CarrionSwarm:
            pass
        class C_DOTA_Ability_StormSpirit_BallLightning:
            pass
        class CDOTA_Modifier_BlueDragonspawnOverseer_DevotionAura:
            pass
        class CDOTA_Modifier_SkeletonKing_Reincarnation_Scepter_Active:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lich_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Amplify_10:
            pass
        class C_DOTA_Ability_Special_Bonus_Magic_Resistance_40:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_150:
            pass
        class CDOTA_Modifier_Item_ForagersHealth:
            pass
        class CDOTA_Modifier_Dawnbreaker_Solar_Guardian_Evasion:
            pass
        class CDOTA_Modifier_EarthSpirit_Polarization_Damage_Timer:
            pass
        class CDOTA_Modifier_Ogre_Magi_Bloodlust:
            pass
        class CDOTA_Modifier_ChaosKnight_Chaos_Strike_Debuff:
            pass
        class CDOTA_Modifier_BountyHunter_Big_Game_Hunter_Tracks:
            pass
        class CDOTA_Modifier_Phantom_Assassin_Blur_Visual:
            pass
        class CDOTA_Modifier_Nian_Tail_Swipe_Wall:
            pass
        class CDOTA_Modifier_Razor_StaticLink_Buff:
            pass
        class C_PostProcessingVolume:
            pass
        class C_DOTA_Item_Enhancement_Crude:
            pass
        class C_DOTA_Item_Orb_Of_Frost:
            pass
        class C_DOTA_Item_Maelstrom:
            pass
        class C_DOTA_Item_Dagon_Upgraded5:
            pass
        class C_DOTA_Ability_Visage_SummonFamiliars:
            pass
        class CDOTA_Ability_NagaSiren_SlithereenCutlass:
            pass
        class CDOTA_Ability_Life_Stealer_Empty2:
            pass
        class C_DOTA_Ability_Dazzle_Good_Juju:
            pass
        class CDOTA_Ability_Miniboss_Unyielding_Shield:
            pass
        class CDOTA_Ability_Roshan_Slam:
            pass
        class C_DOTA_Ability_Courier_ReturnToBase:
            pass
        class C_DOTA_Ability_Greevil_Miniboss_White_Degen_Aura:
            pass
        class C_DOTA_Ability_Greevil_Miniboss_White_Purification:
            pass
        class CDOTA_Ability_PineCone_ShieldBash:
            pass
        class CDOTA_Modifier_UpgradedMortar:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lion_4:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Enchantress_4:
            pass
        class CDOTA_Modifier_Special_Bonus_Spell_Amplify:
            pass
        class C_DOTA_Ability_Special_Bonus_Exp_Boost_35:
            pass
        class C_DOTA_Ability_Special_Bonus_Corruption_3:
            pass
        class CDOTA_Modifier_Item_Ex_Machina:
            pass
        class CDOTA_Modifier_Item_Ballista:
            pass
        class CDOTA_Modifier_VoidSpirit_AetherRemnant_WatchThinker:
            pass
        class CDOTA_Modifier_Skywrath_Mage_Shard:
            pass
        class CDOTA_Modifier_Invoker_ExortInstance:
            pass
        class CDOTA_Modifier_Dazzle_NothlProjection_SoulClone:
            pass
        class CDOTA_Modifier_Tiny_Toss_ChargeCounter:
            pass
        class CDOTA_Modifier_VengefulSpirit_Restitution_Revival:
            pass
        class CDOTA_Modifier_StormSpirit_ElectricVortex_NoStack:
            pass
        class C_BaseModelEntity__Emphasized_Phoneme:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Unit_Hero_Axe:
            pass
        class C_DOTA_Ability_Kez_TalonToss:
            pass
        class C_DOTA_Ability_Phoenix_SunRayToggleMove:
            pass
        class C_DOTA_Ability_Tusk_FrozenSigil:
            pass
        class CDOTA_Modifier_BerserkerTroll_Break:
            pass
        class CDOTA_Modifier_AghsFort_DragonPotion:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lich_6:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_10:
            pass
        class CDOTA_Modifier_Item_Titan_Sliver:
            pass
        class CDOTA_Modifier_Item_MeteorHammer_Burn:
            pass
        class CDOTA_Modifier_Ringmaster_Wheel_Mesmerize_Facing:
            pass
        class CDOTA_Modifier_Grimstroke_Ink_Trail:
            pass
        class CDOTA_Modifier_Oracle_DivinersDeck_TheWorld:
            pass
        class CDOTA_Modifier_Medusa_StoneGaze_Tracker:
            pass
        class CDOTA_Modifier_Brewmaster_Cyclone:
            pass
        class CDOTA_Modifier_Earthshaker_Fissure_Shard:
            pass
        class CDOTA_Modifier_SandKing_SandStorm_Slow:
            pass
        class CPulseCell_PickBestOutflowSelector:
            pass
        class C_DOTA_Unit_Hero_Windrunner:
            pass
        class CIngameEvent_MonsterHunter_DummyModifierHolder:
            pass
        class C_DOTA_Item_Recipe_Nether_Raiment:
            pass
        class C_DOTA_Ability_Pangolier_GyroshellStop:
            pass
        class C_DOTA_Ability_Shredder_WhirlingDeath:
            pass
        class C_DOTA_Ability_Invoker_InvokableElement:
            pass
        class C_DOTA_Ability_Enchantress_Impetus:
            pass
        class C_DOTA_Ability_Rattletrap_Hookshot:
            pass
        class C_DOTA_Ability_QueenOfPain_ScreamOfPain:
            pass
        class C_DOTA_Ability_Warlock_Golem_Permanent_Immolation:
            pass
        class C_DOTA_Ability_Courier_ReturnStashItems:
            pass
        class C_DOTA_Ability_Kunkka_GhostShip:
            pass
        class CInfoFan:
            pass
        class CDOTA_Modifier_UrnUpheaval:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Meepo_3:
            pass
        class C_DOTA_Ability_Special_Bonus_All_Stats_8:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Block_18:
            pass
        class C_DOTA_Ability_Special_Bonus_Undefined:
            pass
        class CPulseCell_ShmupWaitForDuration:
            pass
        class CDOTA_Modifier_Dawnbreaker_Unbreakable:
            pass
        class CDOTA_Modifier_Undying_Decay_Buff:
            pass
        class CDOTA_Modifier_Invoker_GhostWalk_Enemy:
            pass
        class CDOTA_Modifier_Broodmother_Spiders_Milk_Healing:
            pass
        class CDOTA_Modifier_DragonKnight_DragonForm:
            pass
        class CDOTA_Modifier_Lich_Chain_Frost_Frostbound:
            pass
        class CDOTA_Modifier_Bane_Ichor_Of_Nyctasha_Debuff:
            pass
        class CDOTA_Modifier_Bane_FiendsGrip_Cast_Illusion:
            pass
        class C_DOTA_Unit_Hero_Zuus:
            pass
        class CDOTA_Item_SuperSwift_Blink:
            pass
        class C_DOTA_Item_Vampire_Fangs:
            pass
        class CDOTA_Item_GreatFamango:
            pass
        class C_DOTA_Ability_TrollWarlord_Innate_Unflinching:
            pass
        class C_DOTA_Ability_Animation_LeftClawSwipe:
            pass
        class C_DOTA_Ability_Kunkka_Torrent:
            pass
        class C_DOTA_Ability_Greevil_Miniboss_Green_LivingArmor:
            pass
        class CDOTA_Modifier_AghsFort_Creature_Venomancer_PoisonNova:
            pass
        class CDOTA_Ability_AghsFort_ExplosiveBarrel:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Ember_Spirit_1:
            pass
        class C_DOTA_Ability_Special_Bonus_Evasion_40:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_Regen_16:
            pass
        class CDOTA_Modifier_Item_Enhancement_Keen_Eyed:
            pass
        class CDOTA_Modifier_Orb_Of_Frost_Debuff:
            pass
        class CDOTA_Modifier_Soul_Ring_Buff:
            pass
        class CDOTA_Modifier_Item_PlateMail:
            pass
        class CDOTA_Modifier_Skywrath_Mage_Shield_Barrier:
            pass
        class CDOTA_Modifier_Lycan_Shapeshift_Thinker:
            pass
        class CDOTA_Modifier_MeltingStrike_Debuff:
            pass
        class CDOTA_Modifier_Dazzle_NothlBoon_Shield:
            pass
        class CDOTA_Modifier_Nian_GreaterBash_Speed:
            pass
        class CDOTA_Modifier_Slardar_Puddle_Thinker:
            pass
        class CDOTA_Modifier_Building_DispelsSmoke:
            pass
        class C_DOTA_Item_Vanguard:
            pass
        class C_DOTA_Item_SobiMask:
            pass
        class C_DOTA_Item_BladesOfAttack:
            pass
        class C_DOTA_Ability_Silencer_LastWord:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Drow_Ranger_6:
            pass
        class CDOTA_Modifier_Item_Pavise_Shield:
            pass
        class CDOTA_Modifier_Hurricane_Pike_Active:
            pass
        class CDOTA_Modifier_AbyssalUnderlord_AtrophyAura:
            pass
        class CDOTA_Modifier_Tusk_WalrusPunch:
            pass
        class CDOTA_Modifier_Obsidian_Destroyer_Sanity_Eclipse_Thinker:
            pass
        class CDOTA_Modifier_Warlock_Shadow_Word:
            pass
        class CDOTA_Modifier_Razor_EyeOfTheStorm_Armor:
            pass
        class CDOTA_Modifier_Axe_CullingBlade_Boost:
            pass
        class CDOTA_Modifier_Hexed:
            pass
        class C_DOTA_Unit_Hero_Medusa:
            pass
        class C_DOTA_NPC_DataDriven:
            pass
        class C_DOTA_Ability_Grimstroke_Ink_Trail:
            pass
        class C_DOTA_Ability_Techies_ReactiveTazer_Stop:
            pass
        class C_DOTA_Beastmaster_Axe:
            pass
        class C_DOTA_Ability_Riki_TricksOfTheTrade:
            pass
        class CDOTA_Modifier_DoNotCastRock:
            pass
        class CDOTA_Modifier_Spawnlord_Master_Freeze_Root:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Medusa_2:
            pass
        class C_DOTA_Ability_Special_Bonus_24_Crit_2:
            pass
        class CDOTA_Modifier_Item_Broom_Handle:
            pass
        class CDOTA_Modifier_Item_Ocean_Heart:
            pass
        class CDOTA_Modifier_Shredder_WhirlingDeath_Debuff:
            pass
        class CDOTA_Modifier_NightStalker_Void_ZoneThinker:
            pass
        class CDOTA_Modifier_FacelessVoid_TimeDilation_SelfBuff:
            pass
        class CDOTA_Modifier_Roshan_Knockback_Timeout:
            pass
        class CDOTA_Modifier_ShadowShaman_SerpentWard_Intrinsic:
            pass
        class CDOTA_Modifier_Lina_LagunaBlade:
            pass
        class CDOTA_Modifier_SkeletonKing_Reincarnation_Scepter:
            pass
        class CDOTA_Modifier_Rune_Super_Invisibility:
            pass
        class C_DOTA_Unit_TemplarAssassin_PsionicTrap:
            pass
        class C_DOTA_Item_Whisper_Of_The_Dread:
            pass
        class C_DOTA_Item_HydrasBreath:
            pass
        class C_DOTA_Item_Recipe_Holy_Locket:
            pass
        class CDOTA_Item_GreaterFamango:
            pass
        class C_DOTA_Ability_Kez_Kazurai_Katana:
            pass
        class C_DOTA_Ability_Visage_SummonFamiliars_StoneForm:
            pass
        class C_DOTA_Ability_NagaSiren_Crit:
            pass
        class CDOTA_Ability_Nyx_Assassin_Unburrow:
            pass
        class CDOTA_Ability_Obsidian_Destroyer_AstralImprisonment:
            pass
        class CDOTA_Ability_Broodmother_Spiders_Milk:
            pass
        class CDOTA_Ability_Dark_Seer_Innate_MasterTactician:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Beastmaster_6:
            pass
        class CDOTA_Ability_Frogmen_WaterBubble_Medium:
            pass
        class CDOTA_Ability_BerserkerTroll_Break:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Sand_King_5:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Puck_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Lifesteal_30:
            pass
        class C_DOTA_Ability_Special_Bonus_Vision_200:
            pass
        class C_DOTA_Ability_Special_Bonus_Magic_Resistance_8:
            pass
        class CBaseTrackedStatsEntity:
            pass
        class CDOTA_Modifier_Item_Orb_of_Pestilence:
            pass
        class CDOTA_Modifier_Item_GreaterCritical:
            pass
        class CDOTA_Modifier_Hoodwink_Caltrops:
            pass
        class CDOTA_Modifier_EarthSpirit_Magnetize:
            pass
        class CDOTA_Modifier_Shadow_Demon_Disruption_Bonus_Damage:
            pass
        class CDOTA_Modifier_Brewmaster_Void_Void_Strike:
            pass
        class CDOTA_Modifier_Venomancer_PoisonNova:
            pass
        class CDOTA_Modifier_Venomancer_PoisonSting_Applier:
            pass
        class CDOTA_Modifier_Zuus_ArcLightningSlow:
            pass
        class CDOTA_Modifier_DrowRanger_WaveOfSilence:
            pass
        class CDOTA_Modifier_Nevermore_Requiem_InvisBreak:
            pass
        class CDOTA_Modifier_AntiMage_Empowered_ManaBreak:
            pass
        class C_BreakableProp:
            pass
        class C_DOTA_Item_Recipe_Ancient_Janggo:
            pass
        class C_DOTA_Item_TalismanOfEvasion:
            pass
        class C_DOTA_Ability_ArcWarden_SparkWraith:
            pass
        class CDOTA_Ability_Winter_Wyvern_Cold_Embrace:
            pass
        class C_DOTA_Ability_Invoker_Empty2:
            pass
        class C_DOTA_Ability_Chen_HolyPersuasion:
            pass
        class CDOTA_Ability_Broodmother_SpawnSpiderite:
            pass
        class CDOTA_Ability_AncientApparition_ColdFeet:
            pass
        class CDOTA_Modifier_Greevil_Miniboss_Orange_LightStrikeArray_Thinker:
            pass
        class CDOTA_Modifier_GnollAssassin_EnvenomedWeapon_Poison:
            pass
        class CDOTA_Modifier_Watch_Tower_Invulnerable:
            pass
        class C_DOTA_Aghsfort_Ability_Creature_Magnus_Push_Skewer:
            pass
        class CDOTA_Modifier_Seasonal_TI9_MonkeyPoop:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Huskar_5:
            pass
        class CDOTA_Modifier_Special_Bonus_Cooldown_Reduction:
            pass
        class C_DOTA_Ability_Special_Bonus_Armor_7:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Speed_250:
            pass
        class CDOTA_Modifier_Item_Demonicon:
            pass
        class CDOTA_Modifier_Muerta_DeadShot_Slow:
            pass
        class CDOTA_Modifier_Underlord_Fear:
            pass
        class CDOTA_Modifier_Tusk_WalrusPunch_Slow:
            pass
        class CDOTA_Modifier_Magnataur_Solid_Core:
            pass
        class CDOTA_Modifier_TrollWarlord_BattleTrance:
            pass
        class CDOTA_Modifier_BountyHunter_Lookout:
            pass
        class CDOTA_Modifier_Broodmother_Spider_HP:
            pass
        class CDOTA_Modifier_Enchantress_Untouchable_Slow:
            pass
        class CDOTA_Modifier_Tidehunter_LeviathansCatch_Counter:
            pass
        class CDOTA_Modifier_Windrunner_EasyBreezy:
            pass
        class CDOTA_Modifier_Skeleton_King_Innate_VampiricSpirit_Aura:
            pass
        class CDOTA_Modifier_Juggernaut_Omnislash_Invulnerability:
            pass
        class CDOTA_Modifier_TeamShowcase_Global:
            pass
        class CDOTA_Modifier_Tutorial_SpeechBubble:
            pass
        class C_DOTA_Unit_ZeusCloud:
            pass
        class CIngameEvent_MuertaReleaseSpring2023_DummyModifierHolder:
            pass
        class C_DOTA_Item_Recipe_Elven_Tunic:
            pass
        class C_DOTA_Item_Mjollnir:
            pass
        class C_DOTA_Ability_Chen_TestOfFaith:
            pass
        class CDOTA_Modifier_Greevil_Miniboss_Black_Nightmare_Invulnerable:
            pass
        class CPrecipitationVData:
            pass
        class CDOTA_Modifier_AghsFort_Creature_Venomancer_PoisonSting_Applier:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_VoidSpirit_7:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_VoidSpirit_3:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Medusa_5:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Rubick_7:
            pass
        class CDOTA_Modifier_Special_Bonus_Tree_Walking:
            pass
        class CDOTA_Modifier_Item_Ring_Of_Basilius_Effect:
            pass
        class CDOTA_Modifier_Kez_SwitchWeapon_Flutter_Katana:
            pass
        class CDOTA_Modifier_VoidSpirit_ResonantPulse_Ring:
            pass
        class CDOTA_Modifier_Terrorblade_Transforming:
            pass
        class CDOTA_Modifier_Magnataur_Horn_Toss:
            pass
        class CDOTA_Modifier_Slark_Fish_Bait_Pre:
            pass
        class CDOTA_Modifier_Fountain_Fury_Swipes_Damage_Increase:
            pass
        class CDOTA_Modifier_VengefulSpirit_Arcana_Kill_Effect:
            pass
        class CDOTA_Modifier_BookOfStrength:
            pass
        class CDOTA_Modifier_Break:
            pass
        class CPulseCell_WaitForObservable:
            pass
        class C_DOTA_Unit_Hero_Furion:
            pass
        class C_DOTA_Item_Recipe_AbyssalBlade:
            pass
        class CDOTA_Ability_Snapfire_FiresnapCookie:
            pass
        class CDOTA_Ability_Mars_ArenaOfBlood:
            pass
        class C_DOTA_Ability_Grimstroke_Scepter:
            pass
        class CDOTA_Ability_Shadow_Demon_Demonic_Purge:
            pass
        class C_DOTA_Ability_Dazzle_Rain_Of_Vermin:
            pass
        class C_DOTA_Ability_Razor_StormSurge:
            pass
        class C_DOTA_Ability_Mirana_Selemenes_Faithful:
            pass
        class CDOTA_Modifier_Greevil_Miniboss_White_Degen_Aura_Effect:
            pass
        class CDOTA_Modifier_PushWave_Movement:
            pass
        class C_SoundAreaEntitySphere:
            pass
        class CDOTAInGamePredictionState:
            pass
        class CDOTA_Modifier_Aghsfort_Walrus_Pudge_Harpoon:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Pangolier_6:
            pass
        class C_DOTA_Ability_Special_Bonus_Lifesteal_35:
            pass
        class CDOTA_Modifier_Bloodthorn_Debuff:
            pass
        class CDOTA_Modifier_Grimstroke_Shard_Buff:
            pass
        class CDOTA_Modifier_Oracle_FortunesEndPurge:
            pass
        class CDOTA_Modifier_TrollWarlord_BattleTrance_Aura:
            pass
        class CDOTA_Modifier_Wisp_Tentacles:
            pass
        class CDOTA_Modifier_Ogre_Magi_Bloodlust_Autocast:
            pass
        class CDOTA_Modifier_Gyrocopter_Homing_Missile:
            pass
        class CDOTA_Modifier_Chen_Zealot:
            pass
        class CDOTA_Modifier_Life_Stealer_Assimilate_Effect:
            pass
        class CDOTA_Modifier_Nian_Flag_Trapped:
            pass
        class CDOTA_Modifier_Mirana_CelestialQuiver:
            pass
        class CDOTA_Modifier_Knockback:
            pass
        class CDOTA_Modifier_Armor_Per_Barracks:
            pass
        class CPulseCell_Step_EntFire:
            pass
        class C_DOTA_Item_Yasha_And_Kaya:
            pass
        class C_DOTA_Item_Recipe_HeavensHalberd:
            pass
        class C_DOTA_Item_Recipe_HelmOfTheDominator_2:
            pass
        class C_DOTA_Ability_Ogre_Magi_Smash:
            pass
        class C_DOTA_Ability_Nian_Tail_Swipe:
            pass
        class C_DOTA_Ability_Ursa_Fury_Swipes:
            pass
        class C_DOTA_Ability_Zuus_Heavenly_Jump:
            pass
        class CDOTA_Modifier_OgreSmash_Swing:
            pass
        class CDOTA_Ability_Spawnlord_Master_Bash:
            pass
        class CDOTA_Modifier_GnollAssassin_EnvenomedWeapon:
            pass
        class C_BaseButton:
            pass
        class CDOTA_Modifier_AghsFort_Watch_Tower:
            pass
        class CDOTA_Modifier_Ascension_Bulwark:
            pass
        class C_DOTA_Ability_Lesser_NightCrawler_Pounce:
            pass
        class CDOTA_Ability_Seasonal_TI9_Instruments:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Warlock_2:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Spectre_5:
            pass
        class CDOTA_Modifier_Special_Bonus_Unique_Morphling_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Exp_Boost_20:
            pass
        class C_DOTA_DataRadiant:
            pass
        class CDOTA_Modifier_Item_Urn_Damage:
            pass
        class CDOTA_Modifier_Kez_Shodo_Sai_Mark:
            pass
        class CDOTA_Modifier_DeathProphet_Silence_Debuff:
            pass
        class CDOTA_Modifier_Enigma_Event_Horizon_Aura_Effect:
            pass
        class CDOTA_Modifier_AntiMage_Mana_Thirst:
            pass
        class CDOTA_Modifier_Dominated:
            pass
        class CDOTA_Modifier_MagicImmune:
            pass
        class CIngameEvent_FV2023:
            pass
        class CHitboxComponent:
            pass
        class C_DOTA_BaseNPC_Creep:
            pass
        class CDOTA_Item_Lotus_Orb:
            pass
        class CDOTA_Item_Recipe_ForceStaff:
            pass
        class C_DOTA_Ability_TrollWarlord_BerserkersRage:
            pass
        class C_DOTA_Ability_NightStalker_Void:
            pass
        class C_DOTA_Ability_Earthshaker_Aftershock:
            pass
        class CDOTA_Ability_Juggernaut_Innate_Mask_Crit_Lifesteal:
            pass
        class CDOTA_Modifier_AncientRockGolem_Weakening_Aura:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Luna_1:
            pass
        class C_DOTA_Ability_Special_Bonus_Cleave_35:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_275:
            pass
        class CDOTA_Modifier_Arc_Warden_Scepter:
            pass
        class CDOTA_Modifier_ArcWarden_Flux:
            pass
        class CDOTA_Modifier_Batrider_FlamingLasso_Damage:
            pass
        class CDOTA_Ability_Batrider_Flamebreak_Knockback:
            pass
        class CDOTA_Modifier_Broodmother_SpawnSpiderlings_Slow:
            pass
        class CDOTA_Modifier_Clinkz_SearingArrows:
            pass
        class CDOTA_Modifier_Viper_Nethertoxin:
            pass
        class CDOTA_Modifier_Pugna_Decrepify:
            pass
        class CDOTA_Modifier_Windrunner_Windrun_Invis_Thinker:
            pass
        class CDOTA_Modifier_Morphling_Scepter:
            pass
        class C_DOTA_BaseNPC_Clinkz_Skeleton_Army:
            pass
        class C_DOTA_BaseNPC_Launchpad:
            pass
        class CDOTA_Item_IdolOfScreeauk:
            pass
        class C_DOTA_Item_Butterfly:
            pass
        class C_DOTA_Ability_Medusa_MysticSnake:
            pass
        class C_DOTA_Ability_Visage_Lurker:
            pass
        class C_DOTA_Ability_Meepo_Fling:
            pass
        class CDOTA_Ability_Shadow_Demon_Disseminate:
            pass
        class C_DOTA_Ability_Batrider_StickyNapalm_Application_Damage:
            pass
        class C_DOTA_Ability_TemplarAssassin_SelfTrap:
            pass
        class C_DOTA_Ability_CrystalMaiden_CrystalNova:
            pass
        class CDOTA_Ability_Scout_Bonuses:
            pass
        class CDOTA_Modifier_Wildkin_Tornado:
            pass
        class CDOTA_Modifier_Mutation_KillstreakPower_Aura:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_Regen_14:
            pass
        class C_DOTA_Ability_Special_Bonus_Cleave_60:
            pass
        class C_DOTA_Ability_Special_Bonus_Base_Attack_Rate_1:
            pass
        class CDOTA_Modifier_Item_Imp_Claw:
            pass
        class CDOTA_Modifier_Item_CrellasCrozier_Debuff:
            pass
        class CDOTA_Modifier_Elder_Titan_EarthSplitter_Caster:
            pass
        class CDOTA_Modifier_Skywrath_Mage_Arcana:
            pass
        class CDOTA_Modifier_Tusk_IceShard_Slow_Aura:
            pass
        class CDOTA_Modifier_Silencer_GlaivesOfWisdom_SilenceDebuffCounter:
            pass
        class C_DOTA_Unit_Hero_Brewmaster:
            pass
        class CDOTA_BaseNPC_AghsFort_Watch_Tower:
            pass
        class C_DOTAAmbientCreatureParticleZone:
            pass
        class C_DOTA_Item_Caster_Rapier:
            pass
        class C_DOTA_Ability_ArcWarden_Runic_Infusion:
            pass
        class C_DOTA_Ability_Legion_Commander_PressTheAttack:
            pass
        class C_DOTA_Ability_SkeletonKing_VampiricAura:
            pass
        class C_DOTA_Ability_BackdoorProtectionInBase:
            pass
        class CDOTA_Ability_ForestTrollHighPriest_HealAmp_Aura:
            pass
        class CDOTA_Modifier_EnragedWildkin_Hurricane:
            pass
        class CDOTA_Modifier_GreaterClarity:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Underlord_9:
            pass
        class C_DOTA_Ability_Special_Bonus_Magic_Resistance_15:
            pass
        class CDOTA_Modifier_Item_Disperser_Movespeed_Buff:
            pass
        class CDOTA_Modifier_Shadow_Demon_Disruption_ChargeCounter:
            pass
        class CDOTA_Modifier_Lycan_Howl:
            pass
        class CDOTA_Modifier_Obsidian_Destroyer_Ominous_Discernment:
            pass
        class CDOTA_Modifier_DarkSeer_Surge_Trail_Thinker:
            pass
        class CDOTA_Modifier_Bloodseeker_Bloodbath_Thinker:
            pass
        class CPathQueryComponent:
            pass
        class C_DOTA_Unit_Hero_Mirana:
            pass
        class C_DOTA_Item_Penta_Edged_Sword:
            pass
        class C_DOTA_Item_Paintball:
            pass
        class C_DOTA_Item_Recipe_MaskOfMadness:
            pass
        class C_DOTA_Item_PlateMail:
            pass
        class CDOTA_Ability_Ringmaster_WeightedPie:
            pass
        class C_DOTA_Ability_Techies_Spoons_Stash:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Rubick_3:
            pass
        class C_DOTA_Ability_Clinkz_WindWalk:
            pass
        class C_DOTA_Ability_Necrolyte_Sadist:
            pass
        class C_DOTA_Ability_AntiMage_Counterspell_Ally:
            pass
        class CLogicRelay:
            pass
        class CDOTA_Ability_AghsFort_Ascension_Invis:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Techies_3:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Naga_Siren_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Movement_Speed_65:
            pass
        class C_DOTA_Ability_Special_Bonus_Cast_Speed_30:
            pass
        class CDOTA_Modifier_Ringmaster_CrystalBall_Truesight_Aura:
            pass
        class CDOTA_Modifier_Mars_Bulwark_Soldier_Thinker:
            pass
        class CDOTA_Modifier_Earth_Spirit_StoneCaller_ChargeCounter:
            pass
        class CDOTA_Modifier_Shredder_Exposure_Therapy:
            pass
        class CDOTA_Modifier_KeeperOfTheLight_Recall:
            pass
        class CDOTA_Modifier_LoneDruid_SpiritBear_Fetch_Damage:
            pass
        class CDOTA_Modifier_Luna_LunarBlessingAura:
            pass
        class CDOTA_Modifier_VengefulSpirit_Command_Aura_Illusion:
            pass
        class C_IngameEvent_NB2020:
            pass
        class SequenceHistory_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CPlayer_ItemServices:
            pass
        class CPulse_OutflowConnection:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Item_WeightedDice:
            pass
        class C_DOTA_Item_PyrrhicCloak:
            pass
        class C_DOTA_Item_ShadowAmulet:
            pass
        class C_DOTA_Ability_Primal_Beast_Innate_Slow_Resistance_Per_Time:
            pass
        class C_DOTA_Ability_Naga_Siren_Reel_In:
            pass
        class C_DOTA_Ability_Alchemist_Berserk_Potion:
            pass
        class C_DOTA_Ability_TemplarAssassin_Meld:
            pass
        class C_DOTA_Ability_Warlock_Eldritch_Summoning:
            pass
        class C_DOTA_Ability_DrowRanger_Trueshot:
            pass
        class C_DOTA_Ability_Storm_Spirit_Electric_Rave:
            pass
        class CDOTA_Ability_Aghsfort_Bonus_Pudge_MeatHook:
            pass
        class CDOTA_Modifier_AghsFort_EchoSlamPotion_Debuff:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Vengeful_Spirit_1:
            pass
        class C_DOTA_Ability_Special_Bonus_Cast_Range_400:
            pass
        class CDOTA_Modifier_Item_Enhancement_Brawny:
            pass
        class CDOTA_Modifier_Ringmaster_WeightedPie_Debuff:
            pass
        class CDOTA_Modifier_Ringmaster_FunhouseMirror_ModelScale:
            pass
        class CDOTA_Modifier_TrollWarlord_BerserkersRage:
            pass
        class CDOTA_Modifier_Visage_TalentBaseDamage:
            pass
        class CDOTA_Modifier_Wisp_Tether_Haste:
            pass
        class CDOTA_Modifier_BountyHunter_Track:
            pass
        class CDOTA_Modifier_DragonKnight_BreatheFire_Reduction:
            pass
        class CDOTA_Modifier_Windrunner_GaleForce_Aura:
            pass
        class CDOTA_Modifier_Morphling_ScepterStatsDrain_All_Debuff:
            pass
        class CDOTA_Modifier_TrueSightAll:
            pass
        class C_DOTA_Item_InvisibilityEdge:
            pass
        class C_DOTA_Ability_ChaosKnight_Phantasm:
            pass
        class C_DOTA_Ability_Kunkka_Return:
            pass
        class C_DOTA_TempTree:
            pass
        class CDOTA_Ability_Plus_GuildBanner:
            pass
        class CDOTA_Modifier_PudgeMiniboss_ArmorCorruptionStack:
            pass
        class C_DOTA_Ability_AghsFort_StonehallGeneral_OverwhelmingOdds:
            pass
        class CDOTA_Modifier_Lesser_NightCrawler_Pounce:
            pass
        class CDOTA_Ability_Seasonal_Summon_Dragon:
            pass
        class C_DOTA_Ability_Special_Bonus_Respawn_Reduction_45:
            pass
        class C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_30:
            pass
        class C_DOTA_Ability_Special_Bonus_Mana_Break_35:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_200:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_350:
            pass
        class CDOTA_Modifier_Item_Timeless_Relic:
            pass
        class CDOTA_Modifier_Nullifier_Mute:
            pass
        class CDOTA_Modifier_MaelstromChain:
            pass
        class CDOTA_Modifier_Aegis_Regen:
            pass
        class CDOTA_Modifier_Item_Reaver:
            pass
        class CDOTA_Modifier_Kez_FalconRush:
            pass
        class CDOTA_Modifier_MonkeyKing_BounceLeap:
            pass
        class CDOTA_Modifier_Oracle_DivinersDeck_TheLovers:
            pass
        class CDOTA_Modifier_Bristleback_Warpath:
            pass
        class CDOTA_Modifier_Rubick_CuriosityFromHeroes_Tracker:
            pass
        class CDOTA_Modifier_Rubick_FadeBolt:
            pass
        class CDOTA_Modifier_Treant_NaturesGrasp_Damage_Bonus:
            pass
        class CDOTA_Modifier_Obsidian_Destroyer_ObjurgationCooldown:
            pass
        class CDOTA_Modifier_Enchantress_Enchant_Intrinsic:
            pass
        class CDOTA_Modifier_Luna_MoonGlaive:
            pass
        class CDOTA_Modifier_Warlock_Upheaval_Ally:
            pass
        class CDOTA_Modifier_TeamShowcase_Showcase:
            pass
        class CDOTA_BaseNPC_CustomEffigy:
            pass
        class C_DOTA_Item_Armlet:
            pass
        class C_DOTA_Item_PointBooster:
            pass
        class C_DOTA_Ability_Bristleback_ViscousNasalGoo:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Undying_3:
            pass
        class C_DOTA_Ability_Obsidian_Destroyer_EssenceAura:
            pass
        class C_DOTA_Ability_Obsidian_Destroyer_ArcaneOrb:
            pass
        class C_DOTA_Ability_Invoker_Tornado:
            pass
        class C_DOTA_Ability_Dazzle_Poison_Touch:
            pass
        class CDOTA_Ability_PhantomLancer_IllusoryArmaments:
            pass
        class C_DOTA_Ability_Lua:
            pass
        class CDOTA_Modifier_BlackDragon_SplashAttack:
            pass
        class C_DOTA_Ability_Launchpad:
            pass
        class CDOTA_Modifier_Stacking_Gold_Rate_Boost:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Earth_Spirit_2:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_30:
            pass
        class CDOTA_Modifier_Item_Trickster_Cloak_Invis:
            pass
        class CDOTA_Modifier_Item_Enchanted_Quiver:
            pass
        class CDOTA_Modifier_Item_Radiance:
            pass
        class CDOTA_Modifier_Item_WardTrueSight:
            pass
        class CDOTA_Modifier_Item_PoorMansShield:
            pass
        class CDOTA_Modifier_Largo_CatchyLick_Intrinsic:
            pass
        class CDOTA_Modifier_Largo_FroglingBand2_Frogling_Leave:
            pass
        class CDOTA_Modifier_Kez_GrapplingClaw_Movement:
            pass
        class CDOTA_Modifier_Ringmaster_Spotlight_Thinker:
            pass
        class CDOTA_Modifier_Wisp_Tether_Attack_Damage_Penalty:
            pass
        class CDOTA_Modifier_Chaos_Knight_Chaos_Bolt_Additional_Debuff_Delay:
            pass
        class CDOTA_Modifier_Gyrocopter_Flak_Cannon_Scepter:
            pass
        class CDOTA_Modifier_TemplarAssassin_PsiBlades_Slow:
            pass
        class CDOTA_Modifier_Tinker_LaserBlind:
            pass
        class CDOTA_Modifier_AntiMage_Puritan_Slow:
            pass
        class CIngameEvent_Fall2021:
            pass
        class CDOTA_Unit_Hero_Alchemist:
            pass
        class C_DOTA_Unit_Hero_PhantomLancer:
            pass
        class C_DOTA_Item_Recipe_Desolator:
            pass
        class C_DOTA_Item_Recipe_HelmOfTheOverlord:
            pass
        class C_DOTA_Ability_EmberSpirit_FireRemnant:
            pass
        class CDOTA_Ability_Magnus_Strength_Of_Joelrak:
            pass
        class C_DOTA_Ability_Life_Stealer_Rage:
            pass
        class C_DOTA_Ability_DragonKnight_ElderDragonForm:
            pass
        class C_DOTA_Ability_Juggernaut_Bladeform:
            pass
        class C_EnvVolumetricFogController:
            pass
        class CDOTA_Modifier_AghsFort_TreantMiniboss_NaturesGuise_NearTreeDisplay:
            pass
        class C_DOTA_Ability_Seasonal_Festive_Firework:
            pass
        class CDOTA_Modifier_Seasonal_Penguin:
            pass
        class CDOTA_Ability_ShootFirework:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Silencer_4:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Pugna_5:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Riki_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Earthshaker_6:
            pass
        class C_DOTA_Ability_Special_Bonus_Night_Vision_1000:
            pass
        class C_DOTA_Ability_Special_Bonus_Magic_Resistance_80:
            pass
        class C_DOTA_Ability_Special_Bonus_Magic_Resistance_12:
            pass
        class CDOTA_Modifier_Item_Enhancement_Alert:
            pass
        class CDOTA_Modifier_Item_Giants_Ring:
            pass
        class CDOTA_Modifier_Orb_Of_Revelations:
            pass
        class CDOTA_Modifier_Item_Arcane_Blink:
            pass
        class CDOTA_Modifier_ShadowAmulet_Fade:
            pass
        class CDOTA_Modifier_MonkeyKing_Transfiguration_Hidden:
            pass
        class CDOTA_Modifier_Visage_GravekeepersCloak_Secondary:
            pass
        class CDOTA_Modifier_Rubick_Telekinesis:
            pass
        class CDOTA_Modifier_Silencer_CurseOfTheSilent_Intrinsic:
            pass
        class CDOTA_Modifier_Weaver_Rewoven:
            pass
        class CDOTA_Modifier_Huskar_Inner_Fire_Disarm:
            pass
        class CDOTA_Modifier_Slardar_Puddle:
            pass
        class CDOTA_Modifier_Tiny_CraggyExterior_Debuff:
            pass
        class CDOTA_Modifier_Razor_UnstableCurrent:
            pass
        class CPulseGraphDef:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class C_DynamicPropClientside:
            pass
        class C_DOTA_Item_Recipe_Mind_Breaker:
            pass
        class CDOTA_Item_Recipe_Kaya_And_Sange:
            pass
        class CDOTA_Item_Famango:
            pass
        class C_DOTA_Item_Radiance:
            pass
        class C_DOTA_Item_Bracer:
            pass
        class C_DOTA_Ability_Abyssal_Underlord_Raid_Boss:
            pass
        class C_DOTA_Ability_Elder_Titan_AncestralSpirit:
            pass
        class C_DOTA_Ability_KeeperOfTheLight_Illuminate:
            pass
        class C_DOTA_Ability_Lone_Druid_Bear_Necessities:
            pass
        class CDOTA_Ability_Gyrocopter_Side_Gunner_SpawnAbility:
            pass
        class C_DOTA_Ability_Jakiro_Liquid_Ice:
            pass
        class C_DOTA_Ability_Courier_TransferItems:
            pass
        class C_DOTA_Ability_Riki_Innate_Backstab:
            pass
        class CDOTA_Modifier_Aghsfort_Hoodwink_ArcingBoomerang:
            pass
        class CDOTA_Ability_AghsFort_RestorativeFlower:
            pass
        class CDOTA_Modifier_AghsFort_Ascension_Invis:
            pass
        class CDOTA_Modifier_AghsFort_Ascension_Silence_Display:
            pass
        class C_DOTA_Ability_Special_Bonus_Gold_Income_150:
            pass
        class CDOTA_Modifier_Kez_ShodoSai_Parry:
            pass
        class CDOTA_Modifier_Muerta_TheCalling_Silence:
            pass
        class CDOTA_Modifier_Furion_Teleport_Shield:
            pass
        class CDOTA_Modifier_Razor_PlasmaField_Slow:
            pass
        class C_IngameEvent_FM2016:
            pass
        class C_EnvDetailController:
            pass
        class C_DOTA_Item_VindicatorsAxe:
            pass
        class CDOTA_Item_Recipe_Ballista:
            pass
        class C_DOTA_Item_AeonDisk:
            pass
        class C_DOTA_Item_HelmOfTheDominator:
            pass
        class C_DOTA_Item_Recipe_UltimateScepter:
            pass
        class C_DOTA_Ability_Kez_FalconRush:
            pass
        class C_DOTA_Ability_Oracle_FortunesEnd:
            pass
        class CDOTA_Ability_Elder_Titan_FundamentalFury:
            pass
        class C_DOTA_Ability_KeeperOfTheLight_BlindingLight:
            pass
        class C_DOTA_Ability_Rubick_Hidden3:
            pass
        class C_DOTA_Ability_DeathProphet_Witchcraft:
            pass
        class C_EnvWindVolume:
            pass
        class C_DOTA_Ability_Special_Bonus_Lifesteal_12:
            pass
        class C_DOTA_Ability_Special_Bonus_Night_Vision_400:
            pass
        class CDOTA_Modifier_Holy_Locket_ActiveBuff:
            pass
        class CDOTA_Modifier_Black_King_Bar_Immune:
            pass
        class CDOTA_Modifier_Hoodwink_Caltrops_Debuff:
            pass
        class CDOTA_Modifier_EarthSpirit_StoneThinker:
            pass
        class CDOTA_Modifier_EmberSpirit_FireRemnant:
            pass
        class CDOTA_Modifier_Elder_Titan_NaturalOrder_Armor:
            pass
        class CDOTA_Modifier_Broodmother_PoisonStingDebuff:
            pass
        class CDOTA_Modifier_Tinker_Innate_Keen_Teleport_Gold_On_Death:
            pass
        class CDOTA_Modifier_Riki_BlinkStrike:
            pass
        class CDOTA_Modifier_Lua_Motion_Both:
            pass
        class CDOTA_Modifier_FixedNumberOfHitsToKill:
            pass
        class CBasePlayerControllerAPI:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Unit_Hero_Invoker:
            pass
        class C_CrownfallShmupEnemy:
            pass
        class C_DOTA_GuildBannerDynamic:
            pass
        class CEnvSoundscapeAlias_snd_soundscape:
            pass
        class C_DOTA_Item_Enhancement_Quickened:
            pass
        class CDOTA_Item_Recipe_Essence_Ring:
            pass
        class C_DOTA_Item_Recipe_MantaStyle:
            pass
        class C_DOTA_Item_NullTalisman:
            pass
        class C_DOTA_Ability_Brewmaster_EarthBrewling_EarthStance:
            pass
        class C_DOTA_Ability_Invoker_ChaosMeteor:
            pass
        class C_DOTA_Ability_Jakiro_DoubleTrouble:
            pass
        class C_DOTA_Ability_BountyHunter_Cutpurse:
            pass
        class C_DOTA_Ability_Windrunner_FocusFire_End:
            pass
        class CDOTA_Ability_SkeletonKing_SpectralBlade:
            pass
        class C_DOTA_Ability_Greevil_Miniboss_Purple_VenomousGale:
            pass
        class CDOTA_Modifier_HillTroll_Rally:
            pass
        class CDOTA_Ability_EnragedWildkin_Hurricane:
            pass
        class CDOTA_Modifier_Tornado_Tempest:
            pass
        class C_DOTA_Ability_Spawnlord_Master_Freeze:
            pass
        class CDOTA_Modifier_AghsFort_BossWinterWyvern_Cold_Embrace_Thinker:
            pass
        class CDOTA_Modifier_AghsFort_AmoebaBoss_Summoned_Knockback:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Riki_2:
            pass
        class CDOTA_Modifier_Special_Bonus_Mana_Break:
            pass
        class C_DOTA_Ability_Special_Bonus_Intelligence_30:
            pass
        class CDOTA_Modifier_Item_Enhancement_Evolved:
            pass
        class CDOTA_Modifier_Elder_Titan_AncestralSpirit:
            pass
        class CDOTA_Modifier_Abaddon_Withering_Mist_Debuff:
            pass
        class CDOTA_Modifier_Magnataur_Skewer_Slow:
            pass
        class CDOTA_Modifier_Centaur_DoubleEdge_Buff:
            pass
        class CDOTA_Modifier_Visage_SummonFamiliars_In_Formation:
            pass
        class CDOTA_Modifier_Lycan_SummonWolves_GeistForm:
            pass
        class CDOTA_Modifier_Spectre_Arcana_Kill_Effect:
            pass
        class CDOTA_Modifier_BountyHunter_WindWalk_Slow:
            pass
        class CDOTA_Modifier_Windrunner_TailwindIntrinsic:
            pass
        class CDOTA_Modifier_Bloodseeker_Thirst_Vision:
            pass
        class CDOTA_Modifier_VengefulSpirit_WaveOfTerror_Buff:
            pass
        class CDOTA_Modifier_Nevermore_Presence_Aura:
            pass
        class CDOTA_Modifier_MP_Regen:
            pass
        class CDOTA_ArcanaDataEntity_DrowRanger:
            pass
        class C_CSequenceTransitioner2:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_GameRulesProxy:
            pass
        class CRenderComponent:
            pass
        class CDOTA_Item_Recipe_Quickening_Charm:
            pass
        class C_DOTA_Item_Arcane_Boots:
            pass
        class C_DOTA_Ability_Gyrocopter_Flak_Cannon:
            pass
        class C_DOTA_Ability_DragonKnight_DragonTail:
            pass
        class C_DOTA_Ability_FacelessVoid_TimeLock:
            pass
        class C_DOTA_Ability_CrystalMaiden_FreezingField:
            pass
        class CDOTA_Ability_SkeletonKing_BoneGuard_DamageTracker:
            pass
        class C_Team:
            pass
        class C_DOTA_Ability_AghsFort_Creature_Venomancer_PoisonNova:
            pass
        class CDOTA_Ability_Aghsfort_Reward_HPAura:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Meepo_2:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Enigma_2:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_40:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Range_60:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Base_Damage_20:
            pass
        class CDOTA_Modifier_Item_Rabbits_Foot:
            pass
        class CDOTA_Modifier_Item_Enhancement_Restorative:
            pass
        class CDOTA_Modifier_Item_Illusionsts_Cape:
            pass
        class CDOTA_Modifier_Item_Blight_Stone:
            pass
        class CDOTA_Modifier_Item_HandOfMidas:
            pass
        class CDOTA_Modifier_Dark_Willow_Pixie_Dust:
            pass
        class CDOTA_Modifier_MonkeyKing_Bounce:
            pass
        class CDOTA_Modifier_Techies_MutuallyAssuredDestruction:
            pass
        class CDOTA_Modifier_Naga_Siren_Reel_In:
            pass
        class CDOTA_Modifier_SpiritBreaker_ChargeOfDarkness_Untargetable:
            pass
        class CDOTA_Modifier_Enchantress_Little_Friends_Aura:
            pass
        class CDOTA_Modifier_Drow_Ranger_Multishot:
            pass
        class CDOTA_Modifier_Creep_Haste:
            pass
        class C_DOTA_Item_Divine_Regalia_Broken:
            pass
        class C_DOTA_Item_Enhancement_Tough:
            pass
        class C_DOTA_Item_Royal_jelly:
            pass
        class C_DOTA_Item_Necronomicon_Level3:
            pass
        class C_DOTA_Ability_Nyx_Assassin_Impale:
            pass
        class C_DOTA_Ability_PhantomLancer_Juxtapose:
            pass
        class C_PathParticleRopeAlias_path_particle_rope_clientside:
            pass
        class CPointChildModifier:
            pass
        class CDOTA_Modifier_Morty_Hop:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Techies_4:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lich_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Cast_Range_275:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Range_325:
            pass
        class CDOTA_Modifier_Item_Eternal_Shroud_Barrier:
            pass
        class CDOTA_Modifier_EmberSpirit_SearingChains:
            pass
        class CDOTA_Modifier_Centaur_Return_Counter:
            pass
        class CDOTA_Modifier_Wisp_Relocate_Return:
            pass
        class CDOTA_Modifier_Invoker_SunStrike:
            pass
        class CDOTA_Modifier_Clinkz_Burning_Army_Thinker:
            pass
        class CDOTA_Modifier_Beastmaster_InnerBeast:
            pass
        class CDOTA_Modifier_Miniboss_Fortification:
            pass
        class CDOTA_Modifier_Miniboss_UnyieldingShield:
            pass
        class CDOTA_Modifier_SkeletonKing_HellfireBlast_Skeleton_Buff:
            pass
        class CDOTA_Modifier_Nevermore_Requiem_Fear:
            pass
        class C_BaseAnimatingController:
            pass
        class C_DOTA_Unit_Undying_Zombie:
            pass
        class C_TriggerMultiple:
            pass
        class C_DOTA_Item_Tome_Of_Knowledge:
            pass
        class CDOTA_Item_Silver_Edge:
            pass
        class CDOTA_Ability_Snapfire_MortimerKisses:
            pass
        class CDOTA_Ability_Brewmaster_Void_Astral_Pull:
            pass
        class C_DOTA_Ability_Chen_TestOfFaithTeleport:
            pass
        class C_DOTA_Ability_Omniknight_GuardianAngel:
            pass
        class C_DOTA_Ability_Tinker_Eureka:
            pass
        class CDOTA_Modifier_Frogmen_Riverborn_Aura:
            pass
        class CDOTA_Modifier_BlackDragon_Fireball_Thinker:
            pass
        class CDOTA_Modifier_Creature_IceSlam_Thinker:
            pass
        class CDOTA_Modifier_Seasonal_TI9_Shovel_Stasis_Trap:
            pass
        class C_DOTA_Ability_Special_Bonus_Exp_Boost_40:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Range_200:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Range_100:
            pass
        class C_DOTA_Ability_Special_Bonus_Status_Resistance_10:
            pass
        class C_DOTA_Ability_Special_Bonus_Agility_10:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_125:
            pass
        class CDOTA_Modifier_Mechanical_Arm_Counter:
            pass
        class CDOTA_Modifier_Item_EmptyBottle:
            pass
        class CDOTA_Modifier_Muerta_TheCallingAuraSlow:
            pass
        class CDOTA_Modifier_Shadow_Demon_Disruption:
            pass
        class CDOTA_Modifier_Lycan_SummonWolves_PermanentInvisibility:
            pass
        class CDOTA_Modifier_SpiritBreaker_ChargeOfDarknessVision:
            pass
        class CDOTA_Modifier_Venomancer_Snakebite:
            pass
        class CDOTA_Modifier_CDOTA_Modifier_CrystalMaiden_IceRink_Movement_Ricochet:
            pass
        class CDOTA_Modifier_Earthshaker_Echoslam_Heating_Up:
            pass
        class C_BodyComponentBaseAnimatingOverlay:
            pass
        class CDOTA_NPC_Observer_Ward:
            pass
        class C_DOTA_Item_Enhancement_Alert:
            pass
        class C_DOTA_Item_Witch_Blade:
            pass
        class CDOTA_Item_Recipe_Dragon_Scale:
            pass
        class CDOTA_Item_Recipe_Silver_Edge:
            pass
        class CDOTA_Ability_PrimalBeast_Rock_Throw:
            pass
        class CDOTA_Ability_Winter_Wyvern_Winters_Curse:
            pass
        class C_DOTA_Ability_Animation_Attack:
            pass
        class C_DOTA_Ability_Razor_PlasmaField:
            pass
        class CDOTA_Ability_Pudge_FleshHeap:
            pass
        class C_DOTA_Ability_FlagBearer_Creep_Aura_Effect:
            pass
        class CDOTA_Modifier_Aghsfort_Elemental_Wisp_Tether_Haste:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Broodmother_5:
            pass
        class CDOTA_Modifier_Item_RingOfTarrasque:
            pass
        class CDOTA_Modifier_Item_Arcane_Boots:
            pass
        class CDOTA_Modifier_Item_Boots_Of_Bearing_Active:
            pass
        class CDOTA_Modifier_ForceStaff:
            pass
        class CDOTA_Modifier_Snapfire_LilShredder_Buff:
            pass
        class CDOTA_Modifier_Tusk_Snowball_Visible:
            pass
        class CDOTA_Modifier_Ogre_Magi_Fireblast_Attack_Proc:
            pass
        class CDOTA_Modifier_LoneDruid_SpiritBear_Fetch:
            pass
        class CDOTA_Modifier_Lycan_Wolf_Uncontrollable:
            pass
        class CDOTA_Modifier_Viper_PoisonAttack_Slow:
            pass
        class CDOTA_Modifier_Necrolyte_ReapersScythe:
            pass
        class CDOTA_Modifier_Earthshaker_EnchantTotem_Leap:
            pass
        class CDOTA_Unit_Hero_AbyssalUnderlord:
            pass
        class C_DOTA_Item_Tier4Token:
            pass
        class CDOTA_Item_Riftshadow_Prism:
            pass
        class C_DOTA_Item_Falcon_Blade:
            pass
        class C_DOTA_Item_PhaseBoots:
            pass
        class C_DOTA_Ability_MonkeyKing_Transfiguration:
            pass
        class C_DOTA_Ability_Jakiro_Macropyre:
            pass
        class C_DOTA_Ability_Nian_Roar:
            pass
        class C_DOTA_Ability_Bane_BrainSap:
            pass
        class CDOTA_Ability_AncientApparition_ChillingTouch:
            pass
        class CDOTA_Modifier_Watch_Tower_Capturing:
            pass
        class CDOTA_Modifier_Healing_Campfire_Heal:
            pass
        class CDOTA_Modifier_Item_Angels_Demise:
            pass
        class CDOTA_Modifier_Aether_Lens:
            pass
        class CDOTA_Modifier_Item_Ward_Dispenser:
            pass
        class CDOTA_Modifier_Item_RingOfProtection:
            pass
        class CDOTA_Modifier_Broodmother_SpinWebInvisibleApplier:
            pass
        class CDOTA_Modifier_Beastmaster_PrimalRoar_Push:
            pass
        class CDOTA_Modifier_Animation_LeftClawSwipe:
            pass
        class CDOTA_Unit_Hero_Largo:
            pass
        class C_ColorCorrectionVolume:
            pass
        class CDOTA_Item_Recipe_Pirate_Hat:
            pass
        class C_DOTA_Item_MagicWand:
            pass
        class C_DOTA_Ability_Wisp_Empty2:
            pass
        class C_DOTA_Ability_Rubick_NullField:
            pass
        class C_DOTA_Ability_Brewmaster_PermanentImmolation:
            pass
        class C_DOTA_Ability_Invoker_SunStrike_AD:
            pass
        class C_DOTA_Ability_SpiritBreaker_Bulldoze:
            pass
        class CDOTA_Ability_Batrider_Smoldering_Resin:
            pass
        class C_DOTA_Ability_Huskar_Burning_Spear:
            pass
        class C_DOTA_Ability_Viper_ViperStrike:
            pass
        class C_DOTA_Ability_Necrolyte_ReapersScythe:
            pass
        class CDOTA_Modifier_BlackDrake_MagicAmplification:
            pass
        class CDOTA_Modifier_Twin_Gate_FX:
            pass
        class CDOTA_Modifier_AghsFort_TorrentEffectPotion_Torrent_Slow:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Ember_Spirit_2:
            pass
        class C_DOTA_Ability_Special_Bonus_Intelligence_6:
            pass
        class C_DOTA_Ability_Special_Bonus_All_Stats_6:
            pass
        class C_DOTA_Ability_Special_Bonus_Cleave_100:
            pass
        class CDOTA_Modifier_Item_Headdress:
            pass
        class CDOTA_Modifier_Item_Vladmir:
            pass
        class CDOTA_Modifier_Slark_SaltwaterShiv_Slow:
            pass
        class CDOTA_Modifier_Invoker_EMP_Pull_Thinker:
            pass
        class CDOTA_Modifier_Omniknight_Degen_Aura:
            pass
        class CDOTA_Modifier_Prosperous_Soul:
            pass
        class CPlayer_MovementServices:
            pass
        class CDOTA_Unit_Hero_Dawnbreaker:
            pass
        class CInfoDynamicShadowHintBox:
            pass
        class C_DOTA_Item_Mind_Breaker:
            pass
        class CDOTA_Item_Solar_Crest:
            pass
        class C_DOTA_Ability_Luna_LunarBlessing:
            pass
        class C_DOTA_Ability_Viper_Innate_Corrosive_Skin_Vile:
            pass
        class CDOTA_Modifier_PushWave:
            pass
        class CBaseAnimGraphController:
            pass
        class CDOTA_Modifier_Seasonal_Firecrackers:
            pass
        class C_DOTA_Ability_Special_Bonus_Exp_Boost_30:
            pass
        class C_DOTA_Ability_Special_Bonus_Armor_20:
            pass
        class C_DOTA_Ability_Special_Bonus_All_Stats_10:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_Regen_Amp_10:
            pass
        class C_DOTA_Ability_Special_Bonus_Reincarnation_250:
            pass
        class C_DOTA_Ability_Special_Bonus_Cleave_40:
            pass
        class CDOTAPlayer_MovementServices:
            pass
        class CDOTA_Modifier_Item_Chasm_Stone:
            pass
        class CDOTA_Modifier_Item_Swift_Blink:
            pass
        class CDOTA_Modifier_Ringmaster_Unicycle_Cycle:
            pass
        class CDOTA_Modifier_Ringmaster_TheBox_Buff:
            pass
        class CDOTA_Modifier_Hoodwink_AcornShot_DelayThinker:
            pass
        class CDOTA_Modifier_Skywrath_Mage_Shard_Bonus:
            pass
        class CDOTA_Modifier_SpiritBreaker_HerdMentality_Aura:
            pass
        class CDOTA_Modifier_QueenOfPain_Innate_Spell_Reflect:
            pass
        class CDOTA_Modifier_Sniper_Shrapnel_Slow:
            pass
        class C_ColorCorrection:
            pass
        class CBuoyancyHelper:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Unit_AghsFort_SpectralTusk_Tombstone:
            pass
        class C_DOTA_Ability_MonkeyKing_Boundless_Strike:
            pass
        class C_DOTA_Ability_Visage_SoulAssumption:
            pass
        class C_DOTA_Ability_Undying_SoulRip:
            pass
        class C_DOTA_Ability_Chen_DivineFavor:
            pass
        class C_DOTA_Ability_Beastmaster_WildAxes:
            pass
        class CDOTA_Ability_Sniper_TakeAimStop:
            pass
        class C_PhysBox:
            pass
        class C_DOTA_Ability_AghanimsFortress_SkeletonKing_VampiricAura:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Zeus:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Drow_Ranger_1:
            pass
        class CDOTA_Modifier_Special_Bonus_Corruption:
            pass
        class C_DOTA_Ability_Special_Bonus_20_Crit_15:
            pass
        class CDOTA_Modifier_Item_Revenants_Brooch_Active:
            pass
        class CDOTA_Modifier_DarkWillow_ShadowRealm_Buff_Attack_Logic:
            pass
        class CDOTA_Modifier_Shredder_TwistedChakram:
            pass
        class CDOTA_Modifier_Treant_NaturesGrasp_Latch_Thinker:
            pass
        class CDOTA_Modifier_Invoker_WexInstance:
            pass
        class CDOTA_Modifier_Dazzle_NothlProjection_ProjectileReturn:
            pass
        class CDOTA_Modifier_QueenOfPain_SonicWave_Damage:
            pass
        class CDOTA_Modifier_Innate_Riki_Backstab:
            pass
        class CDOTA_Modifier_Razor_Link_Vision:
            pass
        class C_DOTA_PortraitTree:
            pass
        class C_DOTA_Item_Light_Collector:
            pass
        class CDOTA_Item_Recipe_Quicksilver_Amulet:
            pass
        class CDOTA_Item_Recipe_Seer_Stone:
            pass
        class C_DOTA_Item_TeleportScroll:
            pass
        class C_DOTA_Item_BootsOfTravel:
            pass
        class C_DOTA_Ability_MonkeyKing_Spring_Early:
            pass
        class C_DOTA_Ability_Phoenix_SunRayToggleMoveEmpty:
            pass
        class C_DOTA_Ability_Phoenix_SunRayStop:
            pass
        class CDOTA_Ability_DoomBringer_Empty2:
            pass
        class C_DOTA_Ability_Beastmaster_Hawk_Perch:
            pass
        class C_DOTA_Ability_Tinker_Innate_Keen_Teleport_Gold_On_Death:
            pass
        class C_DOTA_Ability_Nevermore_Shadowraze3:
            pass
        class CDOTA_Modifier_Spawnlord_Aura_Bonus:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Lone_Druid_8:
            pass
        class C_DOTA_Ability_Special_Bonus_Cast_Range_150:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Range_275:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_12:
            pass
        class CDOTA_Modifier_Item_Book_Of_Shadows:
            pass
        class CDOTA_Modifier_Item_Orb_of_Pestilence_Slow:
            pass
        class CDOTA_Modifier_Item_Fallen_Sky_Land:
            pass
        class CDOTA_Modifier_Ringmaster_WeightedPie_Blind:
            pass
        class CDOTA_Modifier_AbyssalUnderlord_Firestorm_Burn:
            pass
        class CDOTA_Modifier_Medusa_SplitShot:
            pass
        class CDOTA_Modifier_SpiritBreaker_SpecialAttack:
            pass
        class CDOTA_Modifier_DoomBringer_Devour_Hero_Debuff:
            pass
        class CDOTA_Modifier_DoomBringer_Devour_Intrinsic:
            pass
        class CDOTA_Modifier_Jakiro_Macropyre_Ice_EdgeThinker:
            pass
        class CDOTA_Modifier_Dazzle_Innate_Weave_Armor:
            pass
        class CDOTA_Modifier_Courier_Passive_Bonus:
            pass
        class CDOTA_Modifier_Enigma_BlackHoleThinker:
            pass
        class CDOTA_Modifier_Windrunner_Windrun:
            pass
        class CDOTA_Modifier_NotOnMinimap:
            pass
        class CDOTA_Modifier_Item_Editor:
            pass
        class CDOTA_Modifier_BackdoorProtection:
            pass
        class CDOTA_Item_OgreSealTotem:
            pass
        class CDOTA_Item_RiverPainter3:
            pass
        class C_DOTA_Ability_Nyx_Assassin_Vendetta:
            pass
        class C_DOTA_Ability_Meepo_FairShare:
            pass
        class C_DOTA_Ability_Morphling_MorphReplicate:
            pass
        class CDOTA_Ability_Bloodseeker_BloodMist:
            pass
        class CDOTA_Modifier_Greevil_Miniboss_Yellow_IonShell:
            pass
        class CFilterMultiple:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Invoker_2:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Storm_Spirit_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Dazzle_1:
            pass
        class CDOTA_Modifier_FlaskHealing:
            pass
        class CDOTA_Modifier_Obsidian_Destroyer_Equilibrium_Buff:
            pass
        class CDOTA_Modifier_Batrider_FlamingLasso_Self:
            pass
        class CDOTA_Modifier_Life_Stealer_InfestDot:
            pass
        class CDOTA_Modifier_Furion_Sprout_Tether:
            pass
        class CDOTA_Modifier_Beastmaster_Axe_Invulnerable:
            pass
        class CDOTA_Modifier_AncientApparition_IceVortexThinker:
            pass
        class C_IngameEvent_FM2015:
            pass
        class CPulseCell_FireCursors:
            pass
        class CDOTA_Unit_Hero_PrimalBeast:
            pass
        class C_DOTA_Unit_Hero_Riki:
            pass
        class C_DOTA_Item_Force_Field:
            pass
        class C_DOTA_Item_Vladmir:
            pass
        class C_DOTA_Ability_Magnataur_Solid_Core:
            pass
        class C_DOTA_Ability_Visage_Silent_As_The_Grave:
            pass
        class C_DOTA_Ability_Silencer_GlobalSilence:
            pass
        class CDOTA_Modifier_Frogmen_Riverborn_Aura_Bonus:
            pass
        class CDOTA_Modifier_Watch_Tower:
            pass
        class CEnvSoundscape:
            pass
        class C_SoundEventEntityAlias_snd_event_point:
            pass
        class C_FogController:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Broodmother_1:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Phantom_Assassin_4:
            pass
        class CDOTA_Modifier_Item_MantaStyle:
            pass
        class CDOTA_Modifier_Dawnbreaker_Hearthfire:
            pass
        class CDOTA_Modifier_Special_Bonus_Unique_Elder_Titan_4:
            pass
        class CDOTA_Modifier_Meepo_MegaMeepo_Self:
            pass
        class CDOTA_Modifier_Lone_Druid_Spirit_Bear_Self_Item_Restrictions:
            pass
        class CDOTA_Modifier_Gyrocopter_Chop_Shop:
            pass
        class CDOTA_Modifier_Jakiro_Macropyre_Thinker:
            pass
        class CDOTA_Modifier_Sniper_Assassinate_CastRange:
            pass
        class CDOTA_Modifier_PhantomLancer_Juxtapose_Thinker:
            pass
        class CDOTA_Modifier_Juggernaut_Bladeform:
            pass
        class CDOTA_Modifier_FountainAura:
            pass
        class C_SoundOpvarSetOBBWindEntity:
            pass
        class C_DOTA_Unit_Hero_Lion:
            pass
        class C_NetTestBaseCombatCharacter:
            pass
        class C_DOTA_Item_OgreAxe:
            pass
        class C_DOTA_Item_Recipe_PowerTreads:
            pass
        class C_DOTA_Ability_EarthSpirit_RollingBoulder:
            pass
        class C_DOTA_Ability_NagaSiren_Ensnare:
            pass
        class C_DOTA_Ability_SpiritBreaker_GreaterBash:
            pass
        class C_DOTA_Ability_Dazzle_Innate_Weave:
            pass
        class C_DOTA_Ability_PhantomAssassin_CoupdeGrace:
            pass
        class C_DOTA_Ability_BlueDragonspawnOverseer_DevotionAura:
            pass
        class CDOTA_Modifier_Seasonal_Summon_TI11_Balloon_Visuals:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Naga_Siren_6:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Luna_5:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Templar_Assassin_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Enigma_5:
            pass
        class C_DOTA_Ability_Special_Bonus_All_Stats_15:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_Regen_25:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_325:
            pass
        class CDOTA_Modifier_Item_Rattlecage:
            pass
        class CDOTA_Modifier_Riki_Poison_Dart:
            pass
        class CDOTA_Modifier_PhantomLancer_PhantomEdge_Boost:
            pass
        class CDOTA_Modifier_CrystalMaiden_BrillianceAuraEffect:
            pass
        class CBodyComponentPoint:
            pass
        class CDOTA_Ability_Techies_LandMines:
            pass
        class CDOTA_Ability_Dazzle_NothlProjection:
            pass
        class C_DOTA_Ability_Sniper_Assassinate:
            pass
        class C_DOTA_Ability_Mirana_Leap:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_15:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Speed_90:
            pass
        class CPlayerTrackedStatsEntity:
            pass
        class CDOTA_Modifier_Illusionsts_Cape_Marker:
            pass
        class CDOTA_Modifier_Item_WraithPact_DeathAura:
            pass
        class CDOTA_Modifier_Item_Mekansm_Aura:
            pass
        class CDOTA_Modifier_Winter_Wyvern_Essence_Of_The_Blueheart:
            pass
        class CDOTA_Modifier_Underlord_Portal_Buff:
            pass
        class CDOTA_Modifier_Slark_EssenceShift_Buff:
            pass
        class CDOTA_Modifier_Slark_Pounce:
            pass
        class CDOTA_Modifier_Visage_SummonFamiliars_StoneForm_Timer:
            pass
        class CDOTA_Modifier_Obsidian_Destroyer_EssenceAura:
            pass
        class CDOTA_Modifier_Invoker_SunStrike_Cataclysm:
            pass
        class CDOTA_Modifier_SpiritBreaker_GreaterBash:
            pass
        class CDOTA_Modifier_Weaver_Rewoven_Stacks:
            pass
        class CDOTA_Modifier_Life_Stealer_Infest_Enemy_Hero:
            pass
        class CDOTA_Modifier_SkeletonKing_BoneGuard_DamageTracker:
            pass
        class C_DOTA_Unit_Hero_Shadow_Demon:
            pass
        class C_DOTA_Unit_Poogie:
            pass
        class C_DOTA_Unit_AghsFort_Creature_Batrider:
            pass
        class CDOTAPropConsumableBanner:
            pass
        class CDOTA_Item_GlimmerCape:
            pass
        class C_DOTA_Item_Dagon_Upgraded2:
            pass
        class CDOTA_Item_Battlefury:
            pass
        class CDOTA_Ability_Tusk_WalrusKick:
            pass
        class CDOTA_Ability_Filler_Tooltip:
            pass
        class C_DOTA_Ability_Creep_Siege:
            pass
        class C_EconItemView:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Doom_2:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Amplify_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Armor_30:
            pass
        class C_DOTA_Ability_Special_Bonus_Strength_11:
            pass
        class C_DOTA_Ability_Special_Bonus_Mana_Reduction_11:
            pass
        class CDOTA_Modifier_Item_Blade_Mail_Reflect:
            pass
        class CDOTA_Modifier_Item_BootsOfElven:
            pass
        class CDOTA_Modifier_Item_BootsOfTravel_2:
            pass
        class CDOTA_Modifier_Largo_Frogstomp_Debuff:
            pass
        class CDOTA_Modifier_Hoodwink_Sharpshooter_Recoil_EndAnim:
            pass
        class CDOTA_Modifier_ArcWarden_MagneticField_Evasion:
            pass
        class CDOTA_Modifier_EarthSpirit_GeomagneticGrip:
            pass
        class CDOTA_Modifier_Bristleback_Bristleback:
            pass
        class CDOTA_Modifier_Brewmaster_DrunkenBrawler_Slow:
            pass
        class CDOTA_Modifier_Silencer_GlaivesOfWisdom_BuffCounter:
            pass
        class CDOTA_Modifier_Chen_HolyPersuasion:
            pass
        class CDOTA_Modifier_Venomancer_NoxiousPlagueSecondary:
            pass
        class CDOTA_Modifier_Crystal_Maiden_GlacialGuard:
            pass
        class CDOTA_Modifier_Sven_Vanquisher:
            pass
        class CDOTA_Modifier_Juggernaut_Duelist:
            pass
        class CPulseCell_Timeline__TimelineEvent_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class C_DOTA_Unit_Hero_Earthshaker:
            pass
        class C_DOTA_Item_Recipe_VoidwalkerScythe:
            pass
        class CDOTA_Ability_Shadow_Demon_Demonic_Cleanse:
            pass
        class C_DOTA_Ability_Furion_Teleportation:
            pass
        class C_DOTA_Ability_TemplarAssassin_PsionicTrap:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Lina_2:
            pass
        class CDOTA_Ability_AghsFort_Capture:
            pass
        class CDOTA_Modifier_AghsFort_Creature_SpikedCarapace:
            pass
        class C_DOTA_Ability_Seasonal_Throw_Snowball:
            pass
        class C_DOTA_Ability_Special_Bonus_Cleave_20:
            pass
        class CDOTA_Modifier_Item_Enhancement_Tough:
            pass
        class CDOTA_Modifier_Item_Spark_Of_Courage:
            pass
        class CDOTA_Modifier_Item_Occult_Bracelet_Stack:
            pass
        class CDOTA_Modifier_Item_VoidwalkerScythe:
            pass
        class CDOTA_Modifier_Kez_Echo_Slash_Slow:
            pass
        class CDOTA_Modifier_Ringmaster_WhoopeeCushion_Debuff:
            pass
        class CDOTA_Modifier_Snapfire_MortimerKisses:
            pass
        class CDOTA_Modifier_Mars_Scepter_Damage_Slow:
            pass
        class CDOTA_Modifier_Winter_Wyvern_Winters_Curse:
            pass
        class CDOTA_Modifier_Winter_Wyvern_Arctic_Burn_Frost_Attack:
            pass
        class CDOTA_Modifier_EarthSpirit_Polarization:
            pass
        class CDOTA_Modifier_Tusk_WalrusKick_Slow:
            pass
        class CDOTA_Modifier_LoneDruid_SpiritBear_Entangle_Effect:
            pass
        class CDOTA_Modifier_Obsidian_Destroyer_ArcaneOrb:
            pass
        class CDOTA_Modifier_Venomancer_PoisonSting:
            pass
        class CDOTA_Modifier_Tinker_Warp_Grenade:
            pass
        class CDOTA_Modifier_Tidehunter_Blubber:
            pass
        class CDOTA_Modifier_Morphling_EbbAndFlow_Intrinsic:
            pass
        class CDOTA_Modifier_Item_Mask_Spin_Crit_Consumed:
            pass
        class CIngameEvent_10thAnniversary:
            pass
        class CPulseCell_IntervalTimer__CursorState_t:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class CPulseCell_BaseRequirement:
            pass
        class C_DOTA_Item_Recipe_PhaseBoots:
            pass
        class C_DOTA_Ability_Terrorblade_Terror_Wave:
            pass
        class C_DOTA_Ability_Brewmaster_Cyclone:
            pass
        class C_DOTA_Ability_Jakiro_Liquid_Fire:
            pass
        class CDOTA_Ability_Huskar_Blood_Magic:
            pass
        class C_DOTA_Ability_Warlock_Imp_Explode:
            pass
        class C_DOTA_Ability_Tinker_Rearm:
            pass
        class CDOTA_Ability_Axe_CounterHelix:
            pass
        class C_DOTA_Ability_Seasonal_Summon_Snowman:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Medusa:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Zeus_6:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Pangolier_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Respawn_Reduction_50:
            pass
        class C_DOTA_Ability_Special_Bonus_Strength_18:
            pass
        class CDOTA_Modifier_Item_GaleGuard:
            pass
        class CDOTA_Modifier_Item_Bottomless_Chalice_Regen:
            pass
        class CDOTA_Modifier_ArcaneBoots_ManaRegen:
            pass
        class CDOTA_Modifier_Gem_Active_TrueSight:
            pass
        class CDOTA_Modifier_Visage_SummonFamiliars_Talents:
            pass
        class CDOTA_Modifier_Treant_SuperBloom:
            pass
        class CDOTA_Modifier_Brewmaster_LiquidCourage:
            pass
        class CDOTA_Modifier_WitchDoctor_Maledict:
            pass
        class CDOTA_Modifier_Mirana_SolarFlare:
            pass
        class CDOTA_Modifier_SandKing_SandStorm_Slow_Aura_Thinker:
            pass
        class CPulseCell_BaseState:
            pass
        class OutflowWithRequirements_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class C_DOTA_Unit_Tidehunter_Anchor:
            pass
        class CDOTA_Unit_AghsFort_Creature_DragonKnight:
            pass
        class C_DOTA_Item_Phoenix_Ash:
            pass
        class CDOTA_Item_Recipe_Aether_Lens:
            pass
        class CDOTA_Ability_Necronomicon_Archer_Purge:
            pass
        class C_DOTA_Item_GhostScepter:
            pass
        class CDOTA_Ability_Courier_AutoDeliver:
            pass
        class C_DOTA_Ability_Riki_Permanent_Invisibility:
            pass
        class C_DOTA_Ability_Zuus_LightningBolt:
            pass
        class CDOTA_Ability_Axe_CullingBlade:
            pass
        class CDOTA_Modifier_Seasonal_TI9_Monkey_Thinker:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Outworld_Devourer_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_35:
            pass
        class CDOTA_Modifier_EldwurmsEdda_Intrinsic:
            pass
        class CDOTA_Modifier_Item_Enhancement_Vampiric:
            pass
        class CDOTA_Modifier_Item_Enhancement_Quickened:
            pass
        class CDOTA_Modifier_PrimalBeast_Onslaught_Windup:
            pass
        class CDOTA_Modifier_Spectre_Haunt_Fear:
            pass
        class CDOTA_Modifier_DarkSeer_WallOfReplica_Illusion:
            pass
        class CDOTA_Modifier_Nian_Roar:
            pass
        class CDOTA_Modifier_Tiny_Tree_Grab_Damage:
            pass
        class CDOTA_Modifier_AncientApparition_ChillingTouch:
            pass
        class CPulseCell_IsRequirementValid:
            pass
        class C_InfoPlayerStartGoodGuys:
            pass
        class CDOTA_Item_SerratedShiv:
            pass
        class C_DOTA_Item_Crown:
            pass
        class C_DOTA_Item_Black_King_Bar:
            pass
        class C_DOTA_Item_Slippers:
            pass
        class C_DOTA_Ability_KeeperOfTheLight_ManaLeak:
            pass
        class C_SoundEventPathCornerEntity:
            pass
        class C_InfoVisibilityBox:
            pass
        class C_DotaQuestBase:
            pass
        class CDOTA_Modifier_Aghsfort_Reward_MagicResistAura_Bonus:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Viper_2:
            pass
        class CDOTA_Modifier_Special_Bonus_Cast_Speed:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Juggernaut_3:
            pass
        class CDOTA_Modifier_Item_Harpoon_Pull:
            pass
        class CDOTA_Modifier_Item_Heart:
            pass
        class CDOTA_Modifier_Item_MysticStaff:
            pass
        class CDOTA_Modifier_Techies_ReactiveTazer_Disarm:
            pass
        class CDOTA_Modifier_Medusa_GorgonGrasp_Root:
            pass
        class CDOTA_Modifier_Slark_DarkPact:
            pass
        class CDOTA_Modifier_Necrolyte_Sadist_Counter:
            pass
        class CDOTA_Modifier_StormSpirit_ElectricVortex_Pull:
            pass
        class CDOTA_Modifier_StormSpirit_ElectricVortex_SelfSlow:
            pass
        class CPulseCell_Value_Gradient:
            pass
        class CDOTA_Item_Recipe_Harpoon:
            pass
        class C_DOTA_Item_Orb_of_Pestilence:
            pass
        class C_DOTA_Item_Essence_Distiller:
            pass
        class C_DOTA_Item_Recipe_Spirit_Vessel:
            pass
        class CDOTA_Ability_BlackDragon_DragonhideAura:
            pass
        class CDOTA_Ability_GiantWolf_Intimidate:
            pass
        class CDOTA_Modifier_BlackDrake_MagicAmplification_Aura:
            pass
        class CDOTA_Modifier_Neutral_SpellImmunity:
            pass
        class C_DOTA_Item_GreaterClarity:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Invoker_13:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Storm_Spirit_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Gold_Income_420:
            pass
        class C_DOTA_Ability_Special_Bonus_Movement_Speed_35:
            pass
        class CDOTA_Modifier_Item_Harpoon_InternalCD:
            pass
        class CDOTA_Modifier_Item_Kaya_And_Sange:
            pass
        class CDOTA_Buff_Wards:
            pass
        class CDOTA_Modifier_Techies_Suicide_Leap:
            pass
        class CDOTA_Modifier_SpiritBreaker_Bulldoze:
            pass
        class CDOTA_Modifier_Spectre_Desolate:
            pass
        class CDOTA_Modifier_Threads_Of_Fate:
            pass
        class CDOTA_Modifier_DarkSeer_WallOfReplica:
            pass
        class CDOTA_Modifier_Pugna_LifeDrain_SpellAmp:
            pass
        class CDOTA_Modifier_Venomancer_Sepsis:
            pass
        class CDOTA_Modifier_Tidehunter_LeviathansCatch_Stacks:
            pass
        class CDOTA_Modifier_FountainAuraBuff:
            pass
        class CDOTA_Modifier_CameraFollow:
            pass
        class CDOTA_ItemStockInfo:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class IntervalTimer:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class audioparams_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_CrownfallShmupCamera:
            pass
        class C_DOTA_Item_RiverPainter:
            pass
        class C_DOTA_Ability_Jakiro_DualBreath:
            pass
        class C_DOTA_Item_GrisGris:
            pass
        class C_DOTA_Ability_Tinker_Laser:
            pass
        class CDOTA_Ability_Morphling_Hybrid:
            pass
        class CDOTA_Modifier_Neutral_Creep_Lost:
            pass
        class CDOTA_Modifier_Radar_Thinker:
            pass
        class C_PathParticleRope:
            pass
        class C_DOTA_Ability_Special_Bonus_Agility_15:
            pass
        class C_DOTA_Ability_Special_Bonus_Intelligence_8:
            pass
        class CDOTA_Modifier_VoidSpirit_AstralStep_Debuff:
            pass
        class CDOTA_Modifier_Special_Mars_Spear_Burning_Trail_Burn:
            pass
        class CDOTA_Modifier_Winter_Wyvern_Eldwurm_Scholar:
            pass
        class CDOTA_Modifier_CDOTA_Ability_Oracle_Clairvoyant_Cure:
            pass
        class CDOTA_Modifier_Centaur_Stampede_Slow:
            pass
        class CDOTA_Modifier_Medusa_ManaShield:
            pass
        class CDOTA_Modifier_Visage_Stone_Form_Self_Cast_Cooldown_Manager:
            pass
        class CDOTA_Modifier_LoneDruid_SpiritBear_TowerKillTracker:
            pass
        class CDOTA_Modifier_IonShell_Slow:
            pass
        class CDOTA_Modifier_TemplarAssassin_Trap:
            pass
        class CDOTA_Modifier_DeathProphet_Witchcraft:
            pass
        class CDOTA_Modifier_Tiny_TossTree_Bonus:
            pass
        class CDOTA_Modifier_Morphling_ScepterStatsDrain_Agility_Debuff:
            pass
        class CDOTA_Modifier_Axe_BatleHunger_Self_Movespeed:
            pass
        class CIngameEvent_Spring2021:
            pass
        class CDOTAPropArenaOfBloodWarrior:
            pass
        class C_DOTA_RoshanSpawner:
            pass
        class C_DOTA_Item_Diffusal_Blade:
            pass
        class C_DOTA_Ability_Muerta_Ofrenda:
            pass
        class C_DOTA_Ability_Pangolier_RollupStop:
            pass
        class C_DOTA_Ability_Invoker_EMP:
            pass
        class C_DOTA_Ability_CrystalMaiden_BrillianceAura:
            pass
        class C_DOTA_Ability_DrowRanger_Marksmanship:
            pass
        class CDOTA_Modifier_GraniteGolem_HPAura:
            pass
        class CDOTA_Modifier_PineCone_AcornShot_TreeThinker:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Dark_Seer_3:
            pass
        class CDOTA_Modifier_ConjuersCatalyst_Overheat:
            pass
        class CDOTA_Modifier_Item_Desolator:
            pass
        class CDOTA_Modifier_Item_OrchidMalevolence:
            pass
        class CDOTA_Modifier_GhostScepter_Active:
            pass
        class CDOTA_Modifier_Item_BootsOfTravel:
            pass
        class CDOTA_Modifier_Largo_CatchyLick_Rune:
            pass
        class CDOTA_Modifier_Rubick_Telekinesis_Stun:
            pass
        class CDOTA_Modifier_Weaver_TimeLapse:
            pass
        class CDOTA_Modifier_Turbo_Courier_Haste:
            pass
        class CDOTA_Modifier_ShadowShaman_SerpentWardInvulnAura:
            pass
        class CDOTA_Modifier_Morphling_Replicate_Manager:
            pass
        class CDOTA_Modifier_DrowRanger_Arcana:
            pass
        class C_DOTA_Item_Bullwhip:
            pass
        class C_DOTA_Item_RingOfTarrasque:
            pass
        class C_DOTA_Ability_Chen_Zealot:
            pass
        class CDOTA_Ability_Venomancer_Snakebite:
            pass
        class C_DOTA_Ability_CrystalMaiden_Frostbite:
            pass
        class CDOTA_Ability_Seasonal_TI11_Balloon:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_DarkWillow_2:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Broodmother_2:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Riki_8:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Zeus_2:
            pass
        class CDOTA_Modifier_Item_Possessed_Mask:
            pass
        class CDOTA_Modifier_Item_PlaneswalkersCloak:
            pass
        class CDOTA_Modifier_Item_RingOfRegeneration:
            pass
        class CDOTA_Modifier_Muerta_DeadShot_Fear:
            pass
        class CDOTA_Modifier_Snapfire_MortimerKisses_VisionSource:
            pass
        class CDOTA_Modifier_FacelessVoid_Chronosphere_Speed:
            pass
        class CDOTA_Modifier_QueenOfPain_Masochist:
            pass
        class CDOTA_Modifier_Earthshaker_Echoslam_Debuff:
            pass
        class CTimeline:
            pass
        class CPulseCursorFuncs:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Item_Enhancement_Titanic:
            pass
        class C_DOTA_Item_AsceticCap:
            pass
        class CDOTA_Item_Recipe_Illusionsts_Cape:
            pass
        class CDOTA_Item_Recipe_Mind_Breaker2:
            pass
        class C_DOTA_Item_Recipe_UltimateScepter_2:
            pass
        class C_DOTA_Ability_DoomBringer_Empty1:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Batrider_4:
            pass
        class C_DOTA_Ability_Omniknight_Purification:
            pass
        class C_DOTA_Ability_FacelessVoid_TimeDilation:
            pass
        class CDOTA_Ability_VengefulSpirit_SoulStrike:
            pass
        class C_DOTA_Ability_DrowRanger_Silence:
            pass
        class C_DOTA_Ability_AntiMage_Puritan:
            pass
        class CDOTA_Ability_AntiMage_Scepter:
            pass
        class C_DOTA_Ability_Greevil_Miniboss_Purple_PlagueWard:
            pass
        class CDOTA_Modifier_Tornado_ExpirationTime:
            pass
        class CDOTA_Modifier_CentaurKhan_EnduranceAura_Bonus:
            pass
        class C_TonemapController2:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Earth_Spirit_6:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Lone_Druid_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Range_150:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_Regen_35:
            pass
        class CDOTA_Modifier_Item_Foragers_Mana:
            pass
        class CDOTA_Modifier_Item_Swift_Blink_Buff:
            pass
        class CDOTA_Modifier_Item_Assault_Cuirass_Positive:
            pass
        class CDOTA_Modifier_Item_RefresherOrb:
            pass
        class CDOTA_Modifier_Dawnbreaker_Celestial_Hammer_Movement:
            pass
        class CDOTA_Modifier_Disruptor_Thunder_Strike_Speed:
            pass
        class CDOTA_Modifier_Undying_FleshGolem:
            pass
        class CDOTA_Modifier_Brewmaster_CinderBrew:
            pass
        class CDOTA_Modifier_Rattletrap_RocketFlare_Slow:
            pass
        class CDOTA_Modifier_FacelessVoid_Timelock_TimeWalk_Proc_Marker:
            pass
        class CDOTA_Modifier_Miniboss_Alleviation:
            pass
        class CDOTA_Modifier_Windrunner_UnfocusedFire:
            pass
        class CDOTA_Modifier_SandKing_CausticFinaleOrb:
            pass
        class CountdownTimer:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class PulseNodeDynamicOutflows_t__DynamicOutflow_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CDOTA_BaseNPC_Seasonal_TI11_Balloon_Dire:
            pass
        class C_DOTA_Item_Enhancement_Fleetfooted:
            pass
        class C_DOTA_Item_EagleEye:
            pass
        class C_DOTA_Ability_Centaur_HoofStomp:
            pass
        class C_DOTA_Ability_Rattletrap_Overclocking:
            pass
        class CDOTA_Ability_Beastmaster_SummonRazorback:
            pass
        class C_DOTA_Ability_Tidehunter_Gush:
            pass
        class CDOTA_Ability_Axe_One_Man_Army:
            pass
        class C_DOTA_Ability_Greevil_Miniboss_Black_BrainSap:
            pass
        class C_DOTA_Ability_Special_Bonus_Strength_14:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_Regen_8:
            pass
        class DataTeamPlayer_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Modifier_Item_Phoenix_Ash:
            pass
        class CDOTA_Modifier_Item_RingOfAquila_Aura_Bonus:
            pass
        class CDOTA_Modifier_Item_SuperBlinkDagger:
            pass
        class CDOTA_Modifier_Item_Bracer:
            pass
        class CDOTA_Modifier_PrimalBeast_Uproar_Roared_Self:
            pass
        class CDOTA_Modifier_Dawnbreaker_Luminosity_Blaze_Buff:
            pass
        class CDOTA_Modifier_Dawnbreaker_Celestial_Hammer_Thinker:
            pass
        class CDOTA_Modifier_Winter_Wyvern_Winters_Curse_Aura:
            pass
        class CDOTA_Modifier_Underlord_Portal_Fire_Effect:
            pass
        class CDOTA_Modifier_Elder_Titan_AncestralSpirit_Buff:
            pass
        class CDOTA_Modifier_Broodmother_IncapacitatingBite:
            pass
        class CDOTA_Modifier_DeathProphet_SpiritSiphon_Debuff:
            pass
        class CDOTA_Modifier_Sniper_Shrapnel_ChargeCounter:
            pass
        class CDOTA_Modifier_Zuus_Static_Field:
            pass
        class CIngameEvent_TI2022:
            pass
        class C_DOTA_Item_Psychic_Headband:
            pass
        class C_DOTA_Item_UltimateScepter:
            pass
        class C_DOTA_Item_DustofAppearance:
            pass
        class C_DOTA_Item_PlaneswalkersCloak:
            pass
        class C_DOTA_Ability_Dawnbreaker_Converge:
            pass
        class CDOTA_Ability_Mars_Bulwark:
            pass
        class C_DOTA_Ability_LoneDruid_SpiritBear_Demolish:
            pass
        class C_DOTA_Ability_Brewmaster_Primal_Split_Cancel:
            pass
        class CDOTA_Ability_Dark_Seer_Quick_Wit:
            pass
        class CDOTA_Ability_AncientApparition_IceBlast_Release:
            pass
        class CDOTA_Ability_HillTroll_Rally:
            pass
        class CDOTA_Modifier_Ghost_FrostAttack_Slow:
            pass
        class CDOTA_Modifier_AghsFort_PoisonNova_Creature_Thinker:
            pass
        class C_DOTA_Ability_Special_Bonus_Cleave_25:
            pass
        class C_DOTABaseAbility:
            pass
        class CDOTA_Modifier_Desolator_2_Corruption:
            pass
        class CDOTA_Modifier_Item_Essence_Distiller_Vision:
            pass
        class CDOTA_Modifier_Item_RingOfAquila:
            pass
        class CDOTA_Modifier_EmberSpirit_SleightOfFist_Caster_EchoDamagePenalty:
            pass
        class CDOTA_Modifier_NightStalker_HunterInTheNight:
            pass
        class CDOTA_Modifier_Lich_Ice_Spire_Debuff:
            pass
        class CBasePulseGraphInstance:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_DarkWillow_Creature:
            pass
        class CDOTA_BaseNPC_Seasonal_TI11_Balloon:
            pass
        class C_DOTA_Ability_KeeperOfTheLight_ChakraMagic:
            pass
        class C_DOTA_Ability_ChaosKnight_Reality_Rift:
            pass
        class CDOTA_Ability_Phantom_Assassin_Immaterial:
            pass
        class C_DOTA_Ability_Courier_TakeStashAndTransferItems:
            pass
        class C_DOTA_Ability_Slardar_Amplify_Damage:
            pass
        class C_DOTA_Ability_DrowRanger_WaveOfSilence:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Hoodwink_SharpshooterPureDamage:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_65:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_10:
            pass
        class CDOTA_Modifier_Huskar_Burning_Spear_Self:
            pass
        class CDOTA_Modifier_Nian_GreaterBash:
            pass
        class CDOTA_Modifier_Shadow_Shaman_Ally_Voodoo_Invuln:
            pass
        class CDOTA_Modifier_Antimage_DampenMagic:
            pass
        class FilterHealth:
            pass
        class CDOTA_Item_Enhancement_Evolved:
            pass
        class C_DOTA_Item_Recipe_Sphere:
            pass
        class C_DOTA_Item_EmptyBottle:
            pass
        class CDOTA_Ability_Obsidian_Destroyer_Ominous_Discernment:
            pass
        class C_DOTA_Ability_TemplarAssassin_Refraction:
            pass
        class C_DOTA_Ability_AntiMage_ManaVoid:
            pass
        class CDOTA_Modifier_FrostbittenGolem_TimeWarpAura:
            pass
        class C_PointClientUIHUD:
            pass
        class C_DOTA_Ability_Aghsfort_Aziyog_Underlord_Portal_Warp:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Wisp_10:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Bloodseeker_5:
            pass
        class CDOTA_Modifier_Largo_CatchyLick_Buff:
            pass
        class CDOTA_Modifier_Marci_Unleash_FlurryPulse_Debuff:
            pass
        class CDOTA_Modifier_Hoodwink_Decoy_Invisibility:
            pass
        class CDOTA_Modifier_Visage_Silent_As_The_GravePassive:
            pass
        class CDOTA_Modifier_Wisp_EssenceConduction:
            pass
        class CDOTA_Modifier_Chen_DivineFavor_Aura:
            pass
        class CDOTA_Modifier_NightStalker_Darkness:
            pass
        class CDOTA_Modifier_Enchantress_Enchant:
            pass
        class CDOTA_Modifier_Rattletrap_CogPush:
            pass
        class CDOTA_Modifier_Enigma_Malefice:
            pass
        class CDOTA_Modifier_Puck_PhaseShift:
            pass
        class CPulseCell_Inflow_GraphHook:
            pass
        class CAmbientCreatures:
            pass
        class C_DOTA_Item_Trickster_Cloak:
            pass
        class CDOTA_Item_Pirate_Hat:
            pass
        class C_DOTA_Item_Nullifier:
            pass
        class C_DOTA_Item_Heart:
            pass
        class C_DOTA_Ability_Ringmaster_TameTheBeasts_Crack:
            pass
        class CDOTA_Ability_Grimstroke_InkCreature:
            pass
        class C_DOTA_Ability_Lycan_SummonWolves:
            pass
        class CDOTA_Ability_SpiritBreaker_BullRush:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Batrider_7:
            pass
        class C_DOTA_Ability_Sniper_KeenScope:
            pass
        class C_DOTA_Ability_Windrunner_Windrun:
            pass
        class CDOTA_Modifier_Filler_Tooltip:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_VoidSpirit_1:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lycan_7:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Phantom_Assassin_3:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_Regen_14:
            pass
        class CDOTA_Modifier_Item_Bullwhip:
            pass
        class CDOTA_Modifier_Item_Fluffy_Hat:
            pass
        class CDOTA_Modifier_Item_UltimateOrb:
            pass
        class CDOTA_Modifier_Ringmaster_WhoopeeCushion_Thinker:
            pass
        class CDOTA_Modifier_DarkWillow_BrambleMaze:
            pass
        class CDOTA_Modifier_Oracle_FatesEdict:
            pass
        class CDOTA_Modifier_Nyx_Assassin_Burrow:
            pass
        class CDOTA_Modifier_Disruptor_StaticStorm:
            pass
        class CDOTA_Modifier_Undying_Tombstone_Bunker:
            pass
        class CDOTA_Modifier_Brewmaster_DrunkenBrawler_BrewedUp:
            pass
        class CDOTA_Modifier_DoomBringer_Doom_Aura_Self:
            pass
        class CDOTA_Modifier_DoomBringer_Devour_Hero_Ability:
            pass
        class CDOTA_Modifier_Broodmother_SpinWeb_Thinker:
            pass
        class CDOTA_Modifier_Morphling_ScepterStatsDrain_Intelligence_Buff:
            pass
        class CDOTA_Modifier_Skeleton_King_Arcana:
            pass
        class CDOTA_Modifier_Invulnerable_Hidden:
            pass
        class SignatureOutflow_Resume:
            pass
        class C_DOTA_Item_PigletPole:
            pass
        class CDOTA_Item_Dragon_Scale:
            pass
        class C_DOTA_Item_Recipe_Armlet:
            pass
        class C_DOTA_Ability_NightStalker_Darkness:
            pass
        class C_DOTA_Ability_Tidehunter_AnchorSmash:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Wisp_8:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Razor_4:
            pass
        class CDOTA_Modifier_Item_Necronomicon_Mana_Aura_2:
            pass
        class CDOTA_Modifier_Grimstroke_InkCreature_Spawning:
            pass
        class CDOTA_Modifier_Oracle_RainOfDestiny_Aura:
            pass
        class CDOTA_Modifier_Elder_Titan_EarthSplitterScepter:
            pass
        class CDOTA_Modifier_Shredder_Chakram_Disarm:
            pass
        class CDOTA_Modifier_Treant_LivingArmor:
            pass
        class CDOTA_Modifier_LoneDruid_SpiritBear_Demolish:
            pass
        class CDOTA_Modifier_Lycan_Wolf_Bite_Lifesteal:
            pass
        class CDOTA_Modifier_Broodmother_SpawnSpideriteDebuff:
            pass
        class CDOTA_Modifier_Leshrac_Lightning_Storm_Scepter_Thinker:
            pass
        class CDOTA_Modifier_Dazzle_Good_Juju:
            pass
        class CDOTA_Modifier_Warlock_Rain_Of_Chaos_Death_Trigger:
            pass
        class CDOTA_Modifier_Juggernaut_Healing_Ward_Aura:
            pass
        class CDOTA_Modifier_Muerta_PartingShot_PhysicalBodyDebuff:
            pass
        class CDOTA_Modifier_Fear:
            pass
        class CPathSimpleAPI:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_InfoPlayerStartBadGuys:
            pass
        class C_DOTA_Ability_QueenOfPain_Masochist:
            pass
        class C_DOTA_Ability_Creep_Piercing:
            pass
        class CDOTA_Modifier_Ghost_FrostAttack:
            pass
        class CDOTA_Modifier_Outpost_Summoning:
            pass
        class CDOTA_Modifier_Seasonal_TI11_CongaLineDancer:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Clockwerk_3:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Bloodseeker_2:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Furion_3:
            pass
        class CDOTA_Modifier_Special_Bonus_MP:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Juggernaut_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Exp_Boost_50:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_18:
            pass
        class C_DOTA_Ability_Special_Bonus_Agility_13:
            pass
        class C_DOTA_Ability_Special_Bonus_Movement_Speed_Percentage_5:
            pass
        class CDOTA_Modifier_FlayersBotaActive:
            pass
        class CDOTA_Modifier_Item_Spellslinger:
            pass
        class CDOTA_Modifier_Item_Mage_Slayer_Debuff:
            pass
        class CDOTA_Modifier_EchoSabre_Debuff:
            pass
        class CDOTA_Modifier_Largo_Groovin:
            pass
        class CDOTA_Modifier_Muerta_OfrendaIntrinsic:
            pass
        class CDOTA_Modifier_VoidSpirit_AetherRemnant_Pull:
            pass
        class CDOTA_Modifier_ObliterateSoldier:
            pass
        class CDOTA_Modifier_Elder_Titan_AncestralSpirit_Invincibility:
            pass
        class CDOTA_Modifier_Undying_Tombstone_Zombie_Deathstrike_Slow_Counter:
            pass
        class CDOTA_Modifier_Meepo_Earthbind:
            pass
        class CDOTA_Modifier_Gyrocopter_Call_Down_Slow:
            pass
        class CDOTA_Modifier_Jakiro_LiquidFire:
            pass
        class CDOTA_Modifier_BountyHunter_WindWalk_Fade:
            pass
        class CDOTA_Modifier_Warlock_Rain_Of_Chaos_Golem:
            pass
        class CDOTA_Modifier_Slardar_Amplify_Damage:
            pass
        class CDOTA_Modifier_Lich_FrostArmor_Slow:
            pass
        class CDOTA_Modifier_PhantomLancer_SpiritLance:
            pass
        class C_DOTA_Unit_Fountain:
            pass
        class C_PointCommentaryNode:
            pass
        class C_DOTA_Item_MadstoneBundle:
            pass
        class C_DOTA_Item_Recipe_Necronomicon_2:
            pass
        class C_DOTA_Item_Recipe_MagicWand:
            pass
        class C_DOTA_Item_TranquilBoots2:
            pass
        class CDOTA_Ability_Marci_Grapple:
            pass
        class C_DOTA_Ability_Winter_Wyvern_Accelerated_Learning:
            pass
        class CDOTA_Ability_Chaos_Knight_FundamentalForging:
            pass
        class C_DOTA_Ability_Omniknight_Degen_Aura:
            pass
        class CDOTA_Modifier_BlueDragonspawnOverseer_Evasion:
            pass
        class CDOTA_Modifier_ForestTrollHighPriest_ManaAura_Bonus:
            pass
        class C_DOTA_Ability_GiantWolf_CriticalStrike:
            pass
        class CDOTA_Modifier_Neutral_Sleep_AI:
            pass
        class CSpriteOriented:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Nyx:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Huskar_7:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_120:
            pass
        class CDOTA_Modifier_Item_Grove_Bow_Debuff:
            pass
        class CDOTA_Modifier_Item_Skadi:
            pass
        class CDOTA_Modifier_Armlet_UnholyStrength:
            pass
        class CDOTA_Modifier_Item_HelmOfIronWill:
            pass
        class CDOTA_Modifier_Kez_SwitchWeapon_Flutter_Sai:
            pass
        class CDOTA_Modifier_Hoodwink_Scurry_Active:
            pass
        class CDOTA_Modifier_ArcWarden_MagneticField_Thinker_Attack_Speed:
            pass
        class CDOTA_Modifier_Elder_Titan_NaturalOrder_MagicResistance:
            pass
        class CDOTA_Modifier_Undying_Tombstone_Zombie_Deathlust:
            pass
        class CDOTA_Modifier_Obsidian_Destroyer_Equilibrium_Buff_Counter:
            pass
        class CDOTA_Modifier_Spectre_Desolate_Blind:
            pass
        class CDOTA_Modifier_Clinkz_DeathPact_Permanent_Buff:
            pass
        class CDOTA_Modifier_CallOfTheWild_Boar_BonusDamage:
            pass
        class CDOTA_Modifier_Sniper_Headshot_Slow:
            pass
        class C_DOTA_NPC_Treant_EyesInTheForest:
            pass
        class CDOTA_Unit_Hero_AncientApparition:
            pass
        class C_DOTA_Item_Avianas_Feather:
            pass
        class C_DOTA_Item_Tenderizer:
            pass
        class CDOTA_Ability_Marci_Unleash:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Batrider_5:
            pass
        class C_DOTA_Ability_Earthshaker_EnchantTotem:
            pass
        class CDOTA_Modifier_Aghsfort_Aziyog_Underlord_Firestorm_Thinker:
            pass
        class CDOTA_Modifier_Seasonal_FestiveFirework:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Undying_2:
            pass
        class C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_50:
            pass
        class C_DOTA_Ability_Special_Bonus_Intelligence_15:
            pass
        class CDOTA_Modifier_Item_LanceOfPursuit:
            pass
        class CDOTA_Modifier_Item_Overwhelming_Blink_Debuff:
            pass
        class CDOTA_Modifier_Item_Hood_Of_Defiance_Barrier:
            pass
        class CDOTA_Modifier_Item_Black_King_Bar:
            pass
        class CDOTA_Modifier_Legion_Commander_Outfight_Them_Buff:
            pass
        class CDOTA_Modifier_Meepo_Fling_Slow:
            pass
        class CDOTA_Modifier_Chaos_Knight_Reins_Of_Chaos:
            pass
        class CDOTA_Modifier_Morphling_Morph_Str_Intrinsic:
            pass
        class C_DOTA_Unit_Hero_DeathProphet:
            pass
        class C_DOTA_Unit_Hero_ShadowShaman:
            pass
        class C_DOTA_BaseNPC_Fort:
            pass
        class C_DOTA_Ability_Necronomicon_Warrior_LastWill:
            pass
        class C_DOTA_Ability_Legion_Commander_OverwhelmingOdds:
            pass
        class C_DOTA_Ability_Invoker_DeafeningBlast:
            pass
        class C_DOTA_Ability_Dark_Seer_Aggrandize:
            pass
        class C_DOTA_Ability_Venomancer_PoisonNova:
            pass
        class C_DOTA_Ability_Courier_DequeuePickupFromStash:
            pass
        class C_PointClientUIWorldTextPanel:
            pass
        class CDOTA_Modifier_Plus_HighFiveRequested:
            pass
        class CDOTA_Modifier_AghsFort_Ascension_PlasmaField_Thinker:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Tinker_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Arc_Warden_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Gold_Income_90:
            pass
        class CDOTA_Modifier_OgreSealTotem_Active:
            pass
        class CDOTA_Modifier_Item_Consecrated_Wraps_Auto_Barrier:
            pass
        class CDOTA_Modifier_Oracle_DivinersDeck_Death_Next:
            pass
        class CDOTA_Modifier_Invoker_Tornado_TwisterDamage:
            pass
        class CDOTA_Modifier_SpiritBreaker_GreaterBash_Break:
            pass
        class CDOTA_Modifier_Tinker_Rearmor:
            pass
        class CDOTA_Modifier_Tidehunter_DeadInTheWater:
            pass
        class CDOTA_Modifier_Lich_FrostArmor_Autocast:
            pass
        class CDOTA_Modifier_Axe_CounterHelix:
            pass
        class C_DOTA_BaseNPC_Invoker_Forged_Spirit:
            pass
        class CDOTAPropPlusPlayerGuildBanner:
            pass
        class C_DOTA_BaseNPC_Creep_Lane:
            pass
        class CDOTA_Item_Recipe_Ex_Machina:
            pass
        class CDOTA_Item_RiverPainter5:
            pass
        class C_DOTA_Item_UltimateScepter_Roshan:
            pass
        class C_DOTA_Item_Tango:
            pass
        class CDOTA_Ability_Ringmaster_StrongmanTonic:
            pass
        class C_DOTA_Ability_Pudge_Rot:
            pass
        class CDOTA_Modifier_FrostbittenGolem_TimeWarpAura_Bonus:
            pass
        class CDOTA_Modifier_XP_Fountain_Aura:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Enigma_6:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Abaddon:
            pass
        class C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_40:
            pass
        class CDOTA_Modifier_Item_Witch_Blade_Slow:
            pass
        class CDOTA_Modifier_Largo_FroglingBand2_Frogling:
            pass
        class CDOTA_Modifier_Hoodwink_HuntersQuiver_Mark:
            pass
        class CDOTA_Modifier_MonkeyKing_CloudRun:
            pass
        class CDOTA_Modifier_Treant_NaturesGuise_Root:
            pass
        class CDOTA_Modifier_Invoker_QuasInstance:
            pass
        class CDOTA_Modifier_Weaver_GeminateAttack_Bonus:
            pass
        class CDOTA_BountyHunter_Cutpurse:
            pass
        class CDOTA_Modifier_Furion_Sprout_Tether_Aura:
            pass
        class CDOTA_Modifier_Necrolyte_ReapersScythe_RespawnTime:
            pass
        class CDOTA_Modifier_Bane_Nightmare:
            pass
        class CDOTA_Modifier_Rune_Arcane:
            pass
        class C_DOTA_Unit_Hero_SkeletonKing:
            pass
        class C_DOTA_AncientDecorationProp:
            pass
        class CDOTA_Item_Tombstone_Drop:
            pass
        class C_DOTA_Item_Recipe_Ceremonial_Robe:
            pass
        class C_DOTA_Item_DivineRapier:
            pass
        class CDOTA_Ability_LegionCommander_Intimidate:
            pass
        class CDOTA_Ability_Elder_Titan_MoveSpirit:
            pass
        class C_DOTA_Ability_Courier_Burst:
            pass
        class C_DOTA_Ability_Morphling_Replicate:
            pass
        class CDOTA_Modifier_Aghsfort_Reward_ArmorAura:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Phantom_Lancer_5:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Bane_8:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Amplify_6:
            pass
        class CDOTA_Modifier_Item_PyrrhicCloak:
            pass
        class CDOTA_Modifier_Pangolier_GyroshellBounce:
            pass
        class CDOTA_Modifier_Shredder_TimberChain:
            pass
        class CDOTA_Modifier_KeeperOfTheLight_Illuminate:
            pass
        class CDOTA_Modifier_Brewmaster_WindWalk:
            pass
        class CDOTA_Modifier_Silencer_GlaivesOfWisdom_DebuffCounter:
            pass
        class CDOTA_Modifier_DeathProphet_SpiritSiphon_ChargeCounter:
            pass
        class CDOTA_Modifier_Tiny_Tree_Channel:
            pass
        class CDOTA_Modifier_VengefulSpirit_WaveOfTerror_Fear:
            pass
        class CDOTA_Modifier_AncientApparition_IceAge:
            pass
        class C_DOTA_Unit_Hero_Tinker:
            pass
        class C_World:
            pass
        class C_DOTA_Item_Recipe_Penta_Edged_Sword:
            pass
        class CDOTA_Item_Recipe_Orb_of_Pestilence:
            pass
        class C_DOTA_Ability_ChaosKnight_Chaos_Bolt:
            pass
        class C_DOTA_Ability_Brewmaster_DrunkenHaze:
            pass
        class C_DOTA_Ability_Spectre_Reality:
            pass
        class C_DOTA_Ability_Rattletrap_JetPack:
            pass
        class C_DOTA_Ability_Pugna_LifeDrain:
            pass
        class C_DOTA_Ability_Mirana_Arrow:
            pass
        class CDOTA_Modifier_PineCone_ShieldBash_Slow:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Viper_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Range_300:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_252:
            pass
        class CDOTA_Modifier_Item_Harmonizer:
            pass
        class CDOTA_Modifier_Item_Enhancement_Timeless:
            pass
        class CDOTA_Modifier_Item_Philosophers_Stone:
            pass
        class CDOTA_Modifier_KeeperOfTheLight_BlindingLight:
            pass
        class CDOTA_Modifier_Silencer_GlaivesOfWisdom_Buff:
            pass
        class CDOTA_Modifier_SpiritBreaker_NetherStrike:
            pass
        class CDOTA_Modifier_Huskar_Burning_Spear_Debuff:
            pass
        class CDOTA_Modifier_Miniboss_Alleviation_Active_Aura:
            pass
        class CDOTA_Modifier_Sven_Warcry_Aura:
            pass
        class C_EconItemAttribute:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Item_Enhancement_Feverish:
            pass
        class C_DOTA_Item_Repair_kit:
            pass
        class C_DOTA_Item_Recipe_Phoenix_Ash:
            pass
        class C_DOTA_Item_Recipe_Crellas_Crozier:
            pass
        class C_DOTA_Ability_Ringmaster_TheBox:
            pass
        class C_DOTA_Ability_Undying_TombstoneGrab:
            pass
        class C_DOTA_Ability_Templar_Assassin_MeditationEnd:
            pass
        class C_DOTA_Ability_Tiny_Rocksteady:
            pass
        class CDOTA_Ability_Nevermore_Necromastery:
            pass
        class CDOTA_Modifier_ARDM_NewHero:
            pass
        class CDOTA_Modifier_Mutation_Cooldown_Reduction:
            pass
        class CDOTA_Modifier_Mutation_Cooldown_Reduction_Team_Aura:
            pass
        class C_DOTA_Aghsfort_Ability_Hoodwink_HuntersBoomerang:
            pass
        class CDOTA_Modifier_Unwavering_Condition:
            pass
        class CDOTA_Modifier_Item_Pirate_Hat:
            pass
        class CDOTA_Modifier_Item_Bloodthorn:
            pass
        class CDOTA_Modifier_MonkeyKing_Strike_Crit:
            pass
        class CDOTA_Modifier_Abyssal_Underlord_Raid_Boss:
            pass
        class CDOTA_Modifier_Visage_Lurker:
            pass
        class CDOTA_Modifier_LoneDruid_SpiritBear_Entangle:
            pass
        class CDOTA_Modifier_Furion_SpiritOfTheForest:
            pass
        class CDOTA_Modifier_Plague_Wards_Bonus_Range:
            pass
        class CDOTA_Modifier_ShadowShaman_Chicken_Speed:
            pass
        class CDOTA_Modifier_Bloodseeker_Rupture_ChargeCounter:
            pass
        class CDOTA_Modifier_Antimage_DampenMagic_Aura_Strong:
            pass
        class CDOTA_Modifier_Creep_Slow:
            pass
        class CPulseCell_Inflow_BaseEntrypoint:
            pass
        class C_DOTA_Unit_Hero_Disruptor:
            pass
        class C_DOTA_Item_Enhancement_Mystical:
            pass
        class C_DOTA_Item_Recipe_Tenderizer:
            pass
        class C_DOTA_Ability_Lycan_Wolf_Bite:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Batrider_2:
            pass
        class C_DOTA_Ability_Sven_Warcry:
            pass
        class CDOTA_Modifer_Furbolg_Enrage_Damage:
            pass
        class C_DOTA_Ability_IceShaman_IncendiaryBomb:
            pass
        class CDOTA_Ability_AghsFort_Dragon_Potion:
            pass
        class CDOTA_Modifier_AghsFort_Ascension_PlasmaField_Slow:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Invoker_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Furion:
            pass
        class CDOTA_Modifier_Ringmaster_TheBox_Visual:
            pass
        class CDOTA_Modifier_DarkWillow_Debuff_Fear:
            pass
        class CDOTA_Modifier_Terrorblade_Dark_Unity:
            pass
        class CDOTA_Modifier_EmberSpirit_SleightOfFist_Caster:
            pass
        class CDOTA_Modifier_Visage_SoulAssumption:
            pass
        class CDOTA_Modifier_Meepo_Fling_Held_Caster:
            pass
        class CDOTA_Modifier_Brewmaster_PrimalSplit_Scepter_UI:
            pass
        class CDOTA_Modifier_Invoker_IceWall_AllyBuff:
            pass
        class CDOTA_Modifier_SpiritBreaker_EmpoweringHasteAura:
            pass
        class CDOTA_Modifier_Enigma_Splitting_Image:
            pass
        class CDOTA_Modifier_Tiny_Tree_Grab:
            pass
        class CDOTA_Modifier_Kunkka_GhostShip_Loaded:
            pass
        class CDOTA_Modifier_Skeleton_King_Scepter_Tracker:
            pass
        class CPulseCell_WaitForCursorsWithTagBase:
            pass
        class C_DOTA_Unit_Hero_Oracle:
            pass
        class C_DOTA_Item_Ethereal_Blade:
            pass
        class C_DOTA_Item_UltimateOrb:
            pass
        class C_DOTA_Ability_Enchantress_NaturesAttendants:
            pass
        class C_DOTA_Ability_Omniknight_Innate_HealingHammer:
            pass
        class C_DOTA_Ability_Tinker_Defensive_Matrix:
            pass
        class C_DOTA_Ability_Pudge_GraftFlesh:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Luna_7:
            pass
        class PingConfirmationState_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Modifier_Item_Lance_of_Pursuit_Slow:
            pass
        class CDOTA_Modifier_Item_Spy_Gadget_Aura:
            pass
        class CDOTA_Modifier_Item_Vambrace:
            pass
        class CDOTA_Modifier_EmberSpirit_Immolation_Aura:
            pass
        class CDOTA_Modifier_EmberSpirit_SleightOfFist_Marker:
            pass
        class CDOTA_Modifier_Visage_GraveChill_Buff:
            pass
        class CDOTA_Modifier_Ogre_Magi_DumbLuck:
            pass
        class CDOTA_Modifier_Obsidian_Destroyer_Equilibrium:
            pass
        class CDOTA_Modifier_Obsidian_Destroyer_StackingBonusMana:
            pass
        class CDOTA_Modifier_Viper_BecomeUniversal:
            pass
        class CDOTA_Modifier_CrystalMaiden_CrystalNova:
            pass
        class CDOTA_Modifier_Mirana_MoonlightShadow_KillTracker:
            pass
        class CDOTA_VR_Modifier_Statue_Mode:
            pass
        class CDOTACustomShopItemInfo:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Item_EldwurmsEdda:
            pass
        class CDOTA_Item_Minotaur_Horn:
            pass
        class C_DOTA_Item_Recipe_Ironwood_tree:
            pass
        class C_DOTA_Item_Aether_Lens:
            pass
        class C_DOTA_Item_RingOfAquila:
            pass
        class C_DOTA_Item_Soul_Booster:
            pass
        class C_DOTA_Ability_Brewmaster_ThunderClap:
            pass
        class CDOTA_Modifier_BlueDragonspawnOverseer_DevotionAura_Bonus:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Clinkz_9:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_250:
            pass
        class C_DOTA_Ability_Special_Bonus_Armor_10:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Speed_40:
            pass
        class CDOTA_Modifier_Item_Cloak_Of_Flames:
            pass
        class CDOTA_Modifier_Item_WraithPact:
            pass
        class CDOTA_Modifier_Manta_Phase:
            pass
        class CDOTA_Modifier_Marci_Guardian_Buff:
            pass
        class CDOTA_Modifier_Hoodwink_AcornShot_Slow:
            pass
        class CDOTA_Modifier_AbyssalUnderlord_AtrophyAura_CreepDmgBuff:
            pass
        class CDOTA_Modifier_EarthSpirit_Boulder_Smash_Debuff:
            pass
        class CDOTA_Modifier_SpiritBreaker_GreaterBash_CascadingBashesTracker:
            pass
        class CDOTA_Modifier_Life_Stealer_Infest_Creep:
            pass
        class CDOTA_Modifier_Furion_WrathOfNature_BuffCounter:
            pass
        class CDOTA_Modifier_PhantomAssassin_CoupdeGrace:
            pass
        class CDOTA_Modifier_Roshan_RevengeRoar_Disarm:
            pass
        class CDOTA_Modifier_Tinker_Rearm:
            pass
        class CDOTA_Modifier_VengefulSpirit_Restitution_EnemyDebuff:
            pass
        class C_fogplayerparams_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Item_Mysterious_Hat:
            pass
        class CDOTA_Item_Ocean_Heart:
            pass
        class C_DOTA_Ability_Ringmaster_Spotlight:
            pass
        class C_DOTA_Ability_Disruptor_KineticField:
            pass
        class C_DOTA_Ability_Chaos_Knight_Reins_Of_Chaos:
            pass
        class C_DOTA_Ability_Morphling_Ebb:
            pass
        class C_DOTA_Ability_Crystal_Maiden_Crystal_Clone:
            pass
        class C_DOTA_Ability_Throw_Snowball:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Base_Damage_45:
            pass
        class C_DOTA_Ability_Special_Bonus_Armor_2:
            pass
        class CFoWBlockerRegion:
            pass
        class CDOTA_Modifier_Item_UnstableWand:
            pass
        class CDOTA_Modifier_Item_Repair_Kit:
            pass
        class CDOTA_Modifier_Desolator_Corruption:
            pass
        class CDOTA_Modifier_Item_Ring_Of_Basilius:
            pass
        class CDOTA_Modifier_Largo_CroakOfGenius_Buff:
            pass
        class CDOTA_Modifier_VoidSpirit_AstralStep_Intrinsic:
            pass
        class CDOTA_Modifier_ArcWarden_SparkWraith_Purge:
            pass
        class CDOTA_Modifier_NyxAssassin_NeuroSting_Debuff:
            pass
        class CDOTA_Modifier_Undying_Decay_DebuffCounter:
            pass
        class CDOTA_Modifier_Treant_LivingArmorPassive:
            pass
        class CDOTA_Modifier_FacelessVoid_TimeLock:
            pass
        class CDOTA_Modifier_Riki_TricksOfTheTrade_Phase:
            pass
        class CDOTA_Modifier_PhantomLancer_Dopplewalk_Phase:
            pass
        class CGameSceneNode:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Item_Mage_Slayer:
            pass
        class C_DOTA_Item_Recipe_Fluffy_Hat:
            pass
        class CDOTA_Item_Trusty_Shovel:
            pass
        class C_DOTA_Item_Recipe_Bloodstone:
            pass
        class CDOTA_Ability_Magnataur_ReversePolarity:
            pass
        class C_DOTA_Ability_Furion_ForceOfNature:
            pass
        class C_DOTA_Ability_Lina_LagunaBlade:
            pass
        class CDOTA_Modifier_ForestTrollHighPriest_HealAutocast:
            pass
        class CDOTA_Modifier_Effigy_AghsFort:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_700:
            pass
        class CDOTA_Modifier_Item_BootsOfSpeed:
            pass
        class CDOTA_Modifier_Muerta_PartingShot_Knockback:
            pass
        class CDOTA_Modifier_Slark_DarkPact_Pulses:
            pass
        class CDOTA_Modifier_Visage_Familiar_Innate:
            pass
        class CDOTA_Modifier_Brewmaster_PrimalSplit_Scepter:
            pass
        class CDOTA_Modifier_Enchantress_Untouchable:
            pass
        class CDOTA_Modifier_Furion_Sprout_Healing_Aura:
            pass
        class CDOTA_Modifier_Roshan_Grab:
            pass
        class CDOTA_Modifier_Lion_Innate_ToHellAndBack:
            pass
        class CDOTA_Modifier_Ursa_Bear_Down:
            pass
        class CDOTA_Modifier_Earthshaker_Aftershock:
            pass
        class CPlayer_ObserverServices:
            pass
        class C_DOTA_Unit_Hero_Treant:
            pass
        class C_DOTA_BaseNPC_Barracks:
            pass
        class CDOTAPlayerPawn:
            pass
        class CDOTA_Item_Greater_Mango:
            pass
        class C_DOTA_Item_Recipe_Dagon:
            pass
        class C_DOTA_Ability_Ringmaster_SummonUnicycle:
            pass
        class CDOTA_Ability_Winter_Wyvern_Splinter_Blast:
            pass
        class C_DOTA_Ability_Visage_GraveChill:
            pass
        class C_DOTA_Ability_Weaver_TheSwarm:
            pass
        class C_SoundAreaEntityBase:
            pass
        class C_PlayerVisibility:
            pass
        class CDOTA_Modifier_AghsFort_ShadowShaman_Shackles:
            pass
        class CDOTA_Modifier_Special_Bonus_HP_Regen:
            pass
        class C_DOTA_Ability_Special_Bonus_Movement_Speed_10:
            pass
        class CDOTA_Modifier_Giant_Maul:
            pass
        class CDOTA_Modifier_Item_Fallen_Sky_Burn:
            pass
        class CDOTA_Modifier_Item_MeteorHammer_Land:
            pass
        class CDOTA_Modifier_Item_Ancient_Janggo_Active:
            pass
        class CDOTA_Modifier_Item_Blade_Mail:
            pass
        class CDOTA_Modifier_Item_DivineRapier:
            pass
        class CDOTA_Modifier_Item_NetherRaiment:
            pass
        class CDOTA_Modifier_Kez_Sai:
            pass
        class CDOTA_Modifier_Terrorblade_Arcana_Kill_Effect:
            pass
        class CDOTA_Modifier_Keeper_of_the_Light_BrightSpeed:
            pass
        class CDOTA_Modifier_Silencer_GlaivesOfWisdom:
            pass
        class CDOTA_Modifier_Spectre_Dispersion_Boost:
            pass
        class CDOTA_Modifier_Warlock_Upheaval:
            pass
        class CAttributeManager__cached_attribute_float_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Unit_Hero_Hoodwink:
            pass
        class C_DOTA_Unit_Hero_Wisp:
            pass
        class C_DOTA_BaseNPC_Warlock_Golem:
            pass
        class C_PortraitWorldUnit:
            pass
        class C_BasePlayerWeapon:
            pass
        class C_DOTA_Item_Orb_Of_Corrosion:
            pass
        class CDOTA_Item_Ballista:
            pass
        class C_DOTA_Item_HelmOfIronWill:
            pass
        class C_DOTA_Ability_Hoodwink_HuntersBoomerang:
            pass
        class CDOTA_Ability_Life_Stealer_Assimilate:
            pass
        class C_DOTA_Ability_Viper_PoisonAttack:
            pass
        class CDOTA_Ability_Roshan_InherentBuffs:
            pass
        class CRagdollManager:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Keeper_of_the_Light_14:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Earth_Spirit_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Movement_Speed_90:
            pass
        class C_DOTA_Ability_Special_Bonus_Movement_Speed_45:
            pass
        class CDOTA_Modifier_Item_Disperser_Attack_Debuff:
            pass
        class CDOTA_Modifier_Item_PointBooster:
            pass
        class CDOTA_Modifier_Item_Circlet:
            pass
        class CDOTA_Modifier_Marci_Bodyguarded:
            pass
        class CDOTA_Modifier_Mars_ArenaOfBlood_Leash:
            pass
        class CDOTA_Modifier_MonkeyKing_QuadrupleTap_Bonuses:
            pass
        class CDOTA_Modifier_Invoker_Quas_Intrinsic_DoT:
            pass
        class CDOTA_Modifier_Riki_Backstab:
            pass
        class CDOTA_Modifier_Kunkka_Torrent_Slow:
            pass
        class CDOTA_Modifier_Morphling_Morph_Agi:
            pass
        class CDOTA_Modifier_Mirana_Leap_ChargeCounter:
            pass
        class CDOTA_Modifier_AntiMage_ManaBreak:
            pass
        class CDOTA_Modifier_Tower_Armor_Bonus:
            pass
        class C_EnvSky:
            pass
        class C_DOTA_Item_Rune:
            pass
        class C_DOTA_Item_Enhancement_Greedy:
            pass
        class C_DOTA_Item_Devastator:
            pass
        class CDOTA_Ability_Hoodwink_Decoy:
            pass
        class C_DOTA_Ability_Riki_SmokeScreen:
            pass
        class C_DOTA_Ability_StormSpirit_Galvanized:
            pass
        class C_DOTA_Ability_Juggernaut_Trinity:
            pass
        class CDOTA_Modifier_731_Teaser_Stun:
            pass
        class C_DotaSubquestEntityDeath:
            pass
        class CDOTA_Modifier_Aghsfort_Walrus_Pudge_Harpoon_PathingFix:
            pass
        class CDOTA_Modifier_ItemWiggle_Thinker:
            pass
        class CDOTA_Ability_Seasonal_TI9_Banner:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lifestealer_3:
            pass
        class CDOTA_Modifier_Special_Bonus_Day_Vision:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_Regen_7:
            pass
        class CDOTA_Modifier_Item_Diffusal_Blade:
            pass
        class CDOTA_Modifier_Hoodwink_Sharpshooter_Debuff:
            pass
        class CDOTA_Modifier_Phoenix_Dying_Light_Debuff:
            pass
        class CDOTA_Modifier_Slark_ShadowDance_Aura:
            pass
        class CDOTA_Modifier_Invoker_DeafeningBlast_Disarm:
            pass
        class CDOTA_Modifier_Life_Stealer_UnfetteredFury:
            pass
        class CDOTA_Modifier_Drow_Ranger_Glaicer_Intrinsic:
            pass
        class CIngameEvent_Frostivus2024:
            pass
        class CPulse_InvokeBinding:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CDOTA_Item_Clumsy_Net:
            pass
        class CDOTA_Item_Recipe_Solar_Crest:
            pass
        class C_DOTA_Ability_Brewmaster_StormBrewling_StormStance:
            pass
        class C_DOTA_Ability_Invoker_ColdSnap_AD:
            pass
        class C_DOTA_Ability_Morphling_AdaptiveStrike_Agi:
            pass
        class C_DOTA_Ability_Lina_LightStrikeArray:
            pass
        class CDOTA_Modifier_MudGolem_CloakAura:
            pass
        class C_DOTA_Ability_BlackDrake_MagicAmplification_Aura:
            pass
        class CDOTA_Modifier_KoboldTaskmaster_SpeedAura:
            pass
        class CDOTA_Modifier_Jungle_Varmint:
            pass
        class C_EnvWindController:
            pass
        class C_DOTA_Ability_AghsFort_Waveblaster_Leap:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Ember_Spirit_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Movement_Speed_Percentage_8:
            pass
        class CDOTA_Modifier_Item_Dormant_CurioPermanent:
            pass
        class CDOTA_Modifier_Item_Essence_Ring:
            pass
        class CDOTA_Modifier_Nullifier:
            pass
        class CDOTA_Modifier_Item_WindLace:
            pass
        class CDOTA_Modifier_Item_OgreAxe:
            pass
        class CDOTA_Modifier_Largo_Song_Speed_Burst:
            pass
        class CDOTA_Modifier_Kez_RavensVeil_Buff:
            pass
        class CDOTA_Modifier_Techies_StickyBomb_Slow_Secondary:
            pass
        class CDOTA_Modifier_Slark_Depth_Shroud_Thinker:
            pass
        class CDOTA_Modifier_Luna_LucentBeam_Damage_Buff_Counter:
            pass
        class CDOTA_Modifier_Viper_Nethertoxin_Thinker:
            pass
        class CDOTA_Modifier_PhantomAssassin_Fan_Of_Knives:
            pass
        class CDOTA_Modifier_Tiny_Toss_Land_Debuff:
            pass
        class CDOTA_Modifier_Morphling_Waveform_ChargeCounter:
            pass
        class CDOTA_Modifier_DrowRanger_Trueshot:
            pass
        class CDOTA_Modifier_Rooted:
            pass
        class CDOTA_Modifier_Generic_Hidden:
            pass
        class InventoryQuickBuyState_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_GameRules:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Item_Recipe_Consecrated_Wraps:
            pass
        class C_DOTA_Item_Recipe_Buckler:
            pass
        class CDOTA_Ability_Techies_RemoteMines:
            pass
        class C_DOTA_Ability_Broodmother_WebWalk:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lion_6:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Vengeful_Spirit_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Movement_Speed_100:
            pass
        class C_DOTAPortraitWorldCallbackHandler:
            pass
        class CDOTA_Modifier_Hurricane_Pike_Active_Alternate:
            pass
        class CDOTA_Modifier_Hoodwink_Hunters_Mark:
            pass
        class CDOTA_Modifier_Oracle_FortunesEndChannelTarget:
            pass
        class CDOTA_Modifier_Underlord_Portal_Fire:
            pass
        class CDOTA_Modifier_Bristleback_Prickly:
            pass
        class CDOTA_Modifier_Chen_HandOfGod_InvulnAura:
            pass
        class CDOTA_Modifier_Warlock_Golem_Permanent_Immolation:
            pass
        class CDOTA_Modifier_Ursa_Fury_Swipes:
            pass
        class CDOTA_Modifier_Enigma_Innate_EventHorizon:
            pass
        class CDOTA_Modifier_PhantomLancer_Juxtapose:
            pass
        class CDOTA_Modifier_DrowRanger_FrostArrows_Hypothermia:
            pass
        class CDOTA_Modifier_Activity_Modifier:
            pass
        class C_DOTA_BaseNPC_Building:
            pass
        class CDOTA_Item_Polliwog_Charm:
            pass
        class C_DOTA_Item_Recipe_Gungir:
            pass
        class C_DOTA_Item_Recipe_Yasha_And_Kaya:
            pass
        class C_DOTA_Ability_Kez_SwitchWeapons:
            pass
        class CDOTA_Ability_Meepo_Poof:
            pass
        class C_DOTA_Ability_SpiritBreaker_NetherStrike:
            pass
        class C_DOTA_Ability_Clinkz_Tar_Bomb:
            pass
        class CDOTA_Item_Tidehunter_Fish:
            pass
        class C_PointWorldText:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Chen_7:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Phantom_Assassin_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Gold_Income_60:
            pass
        class C_DOTA_Ability_Special_Bonus_Agility_14:
            pass
        class CDOTA_UI_ScenePanelAPI:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Modifier_Item_Shivas_Guard_Thinker:
            pass
        class CDOTA_Modifier_Item_VoidStone:
            pass
        class CDOTA_Modifier_Pangolier_Fortune_Favors_The_Bold_Effect:
            pass
        class CDOTA_Modifier_EmberSpirit_FireRemnant_RemnantTracker:
            pass
        class CDOTA_Modifier_Elder_Titan_Fundamental_Fury:
            pass
        class CDOTA_Modifier_Nyx_Assassin_SpikedCarapace:
            pass
        class CDOTA_Modifier_Luna_MoonGlaive_Shield:
            pass
        class CDOTA_Modifier_TemplarAssassin_PsionicTrap_Counter:
            pass
        class CDOTA_Modifier_Beastmaster_PrimalRoar_Slow:
            pass
        class CDOTA_Modifier_Razor_Arcana:
            pass
        class C_IngameEvent_TI9:
            pass
        class CDOTASubChallengeInfo:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Item_Yasha:
            pass
        class CDOTA_Ability_Pangolier_Fortune_Favors_The_Bold:
            pass
        class C_DOTA_Ability_Wisp_Overcharge:
            pass
        class CDOTA_Ability_Lycan_SummonWolves_Bash:
            pass
        class C_DOTA_Ability_Greevil_Miniboss_Red_Earthshock:
            pass
        class C_RopeKeyframe:
            pass
        class C_BaseToggle:
            pass
        class C_EnvCubemapBox:
            pass
        class CDOTA_Modifier_AghsFort_Creature_Impale:
            pass
        class CDOTA_Modifier_Special_Bonus_Attack_Base_Damage:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Dazzle_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Immunity:
            pass
        class CDOTA_Modifier_AttributeBonus:
            pass
        class CDOTA_Modifier_Item_Blood_Grenade_Flight_Thinker:
            pass
        class CDOTA_Modifier_Pangolier_LuckyShot_Silence:
            pass
        class CDOTA_Modifier_Techies_Spoons_Stash:
            pass
        class CDOTA_Modifier_Elder_Titan_NaturalOrder_Aura_Armor:
            pass
        class CDOTA_Modifier_Medusa_GorgonGrasp_Thinker:
            pass
        class CDOTA_Modifier_Meepo_Poof_Damage_Sharing:
            pass
        class CDOTA_Modifier_Chen_Divine_Favor_Teleport:
            pass
        class CDOTA_Modifier_Chen_Penitence_Attack_Speed_Buff:
            pass
        class CDOTA_Modifier_DarkSeer_Normal_Punch_Illusion_Thinker:
            pass
        class CDOTA_Modifier_FacelessVoid_TimeLock_Freeze:
            pass
        class CDOTA_Modifier_Axe_CullingBlade_Permanent:
            pass
        class CDOTA_Modifier_Pudge_Meat_Hook_Followthrough:
            pass
        class CDOTA_Modifier_HeroStatue:
            pass
        class CDOTA_Item_Quicksilver_Amulet:
            pass
        class C_DOTA_Item_Recipe_MonkeyKingBar:
            pass
        class CDOTA_Item_ObserverWard:
            pass
        class C_DOTA_Ability_Ringmaster_TameTheBeasts:
            pass
        class C_DOTA_Ability_TrollWarlord_WhirlingAxes_Ranged:
            pass
        class C_DOTA_Ability_Invoker_Alacrity:
            pass
        class C_DOTA_Ability_Nian_Hurricane:
            pass
        class CDOTA_Ability_Miniboss_Fortification:
            pass
        class CDOTA_Ability_Courier_Morph:
            pass
        class C_DOTA_Ability_DrowRanger_Multishot:
            pass
        class CDOTA_Modifier_Greevil_Miniboss_Casting:
            pass
        class CDOTA_Modifier_Kobold_Disarm:
            pass
        class C_DOTA_Ability_VengefulSpirit_Command_Aura:
            pass
        class CDOTA_Modifier_AghsFort_RockGolem_Avalanche:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Jakiro_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Lone_Druid_2:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_251:
            pass
        class CDOTA_Ability_Special_Bonus_Corruption_25:
            pass
        class C_DOTA_DataNonSpectator:
            pass
        class CDOTA_Modifier_Item_GunpowderGauntlets:
            pass
        class CDOTA_Modifier_Stormcrafter_Slow:
            pass
        class CDOTA_Modifier_Item_Essence_Distiller_Damage:
            pass
        class CDOTA_Modifier_Item_Necronomicon:
            pass
        class CDOTA_Modifier_Abaddon_BorrowedTime_ImmolationAura:
            pass
        class CDOTA_Modifier_Slark_SaltwaterShiv_Effect:
            pass
        class CDOTA_Modifier_Undying_Decay_Shard:
            pass
        class CDOTA_Modifier_Lycan_FeralImpulse:
            pass
        class CDOTA_Modifier_Broodmother_StickySnare_Root:
            pass
        class CDOTA_Modifier_QueenOfPain_SonicWave_Effect:
            pass
        class CDOTA_Modifier_Tinker_Turret_Debuff:
            pass
        class CDOTA_Modifier_Kunkka_Ghost_Ship_Fire_Cannons:
            pass
        class CDOTA_Modifier_Razor_PlasmaField_Thinker:
            pass
        class CDOTA_Modifier_Pudge_Meat_Hook_PathingFix:
            pass
        class CDOTA_Modifier_Invisible:
            pass
        class CDOTA_Modifier_BaseBlocker_Thinker:
            pass
        class C_RopeKeyframe__CPhysicsDelegate:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CAdditionalWearable:
            pass
        class CInfoDynamicShadowHint:
            pass
        class CDOTA_Item_SuperOverwhelming_Blink:
            pass
        class C_DOTA_Ability_Warlock_Golem_Flaming_Fists:
            pass
        class C_DOTA_Ability_Nian_Leap:
            pass
        class C_DOTA_Ability_Lion_ManaDrain:
            pass
        class C_DOTA_Ability_Mirana_MoonlightShadow:
            pass
        class CPathNode:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Wisp_6:
            pass
        class C_DOTA_Ability_Special_Bonus_Movement_Speed_25:
            pass
        class CDOTA_Modifier_Whisper_Of_The_Dread:
            pass
        class CDOTA_Modifier_Woodland_Striders_Active:
            pass
        class CDOTA_Modifier_Item_Sphere_Target:
            pass
        class CDOTA_Modifier_Item_SacredRelic:
            pass
        class CDOTA_Modifier_Legion_Commander_Outfight_Them_AuraMarker:
            pass
        class CDOTA_Modifier_Elder_Titan_EchoStomp:
            pass
        class CDOTA_Modifier_Shadow_Demon_Menace_Crushed:
            pass
        class CDOTA_Modifier_Huskar_Burning_Spear_Counter:
            pass
        class CDOTA_Modifier_DeathProphet_SpiritSiphon:
            pass
        class CDOTA_Modifier_Zuus_ArcLightning:
            pass
        class CDOTA_Modifier_Sven_GreatCleave:
            pass
        class CDOTA_Unit_Announcer_Killing_Spree:
            pass
        class C_DOTA_Unit_Hero_Sven:
            pass
        class C_FuncMoveLinear:
            pass
        class C_DOTA_Item_Pupils_gift:
            pass
        class C_DOTA_Item_Recipe_Assault_Cuirass:
            pass
        class C_DOTA_Item_SacredRelic:
            pass
        class CDOTA_Ability_VoidSpirit_Variant_Equilibrium:
            pass
        class C_DOTA_Ability_Terrorblade_ConjureImage:
            pass
        class CDOTA_Ability_AbyssalUnderlord_AtrophyAura:
            pass
        class C_DOTA_Ability_Rubick_TelekinesisLand:
            pass
        class CDOTA_Ability_Furbolg_Enrage_AttackSpeed:
            pass
        class C_DOTA_BinaryObject:
            pass
        class CDOTA_Ability_AghsFort_Lifestealer_Enraged_Pulse:
            pass
        class CDOTA_Ability_Aghsfort_Walrus_Pudge_Harpoon:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Mirana_6:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_175:
            pass
        class TierNeutralInfo_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Modifier_Muerta_PierceTheVeil:
            pass
        class CDOTA_Modifier_Oracle_FalsePromise:
            pass
        class CDOTA_Modifier_Oracle_FortunesEnd_PurgeRepeatedly:
            pass
        class CDOTA_Modifier_Abaddon_Withering_Mist:
            pass
        class CDOTA_Modifier_Tusk_Tag_Team_Attack_Slow_Aura:
            pass
        class CDOTA_Modifier_Invoker_Alacrity:
            pass
        class CDOTA_Modifier_Chen_TestOfFaith_Teleport:
            pass
        class CDOTA_Modifier_BountyHunter_Jinada:
            pass
        class CDOTA_Modifier_Rattletrap_RocketFlare:
            pass
        class CDOTA_Modifier_Item_Vermillion_Robe:
            pass
        class CDOTA_Modifier_Nian_Leap:
            pass
        class CDOTA_Modifier_Nevermore_Requiem_Aura:
            pass
        class CDOTA_Modifier_Pudge_Swallow_Effect:
            pass
        class CDOTA_Unit_Frogling_Event:
            pass
        class CDOTA_Item_Foragers_Stats:
            pass
        class C_DOTA_Item_Princes_Knife:
            pass
        class CDOTA_Item_SentryWard:
            pass
        class CDOTA_Item_BootsOfTravel_2:
            pass
        class C_DOTA_Item_Circlet:
            pass
        class CDOTA_Ability_VoidSpirit_Dissimilate:
            pass
        class C_DOTA_Ability_Shredder_ReturnChakram:
            pass
        class CDOTA_Ability_Disruptor_KineticFence:
            pass
        class CDOTA_Ability_Treant_SuperBloom:
            pass
        class C_DOTA_Ability_Viper_CorrosiveSkin:
            pass
        class CDOTA_Item_Recipe_Vermillion_Robe:
            pass
        class CDOTA_Modifier_ForestTrollHighPriest_HealAmp_Bonus:
            pass
        class CDOTA_Modifier_GraniteGolem_HPAura_Bonus:
            pass
        class CDOTA_Modifier_MudGolem_RockDestroy:
            pass
        class CServerOnlyModelEntity:
            pass
        class CDOTA_Ability_AghsFort_EchoSlamPotion:
            pass
        class CDOTA_Modifier_AghsFort_Ascension_Invis_Warning:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Underlord_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Lifesteal_100:
            pass
        class CDOTA_Modifier_DaggerOfRistul_Buff:
            pass
        class CDOTA_Modifier_Item_SeedsOfSerenity_Active:
            pass
        class CDOTA_Modifier_Item_Tree_Processor:
            pass
        class CDOTA_Modifier_Item_Panic_Button:
            pass
        class CDOTA_Modifier_Clumsy_Net_Ensnare:
            pass
        class CDOTA_Modifier_Item_Vanguard:
            pass
        class CDOTA_Modifier_Ward_RecentlySpawned:
            pass
        class CDOTA_Modifier_Item_Claymore:
            pass
        class CDOTA_Modifier_Marci_Unleash_Pulse_Silence:
            pass
        class CDOTA_Modifier_Shredder_Chakram_Debuff_2:
            pass
        class CDOTA_Modifier_Centaur_Innate_Rawhide:
            pass
        class CDOTA_Modifier_TrollWarlord_Axe_Invulnerable:
            pass
        class CDOTA_Modifier_Meepo_Geostrike_Debuff:
            pass
        class CDOTA_Modifier_ForgedSpirit_MeltingStrike:
            pass
        class CDOTA_Modifier_Batrider_Smoldering_Resin_Debuff:
            pass
        class CDOTA_Modifier_Furion_OneWithNature:
            pass
        class CDOTA_Modifier_Furion_Sprout_Blind:
            pass
        class CDOTA_Modifier_Tidehunter_Ravage:
            pass
        class CDOTA_Modifier_SandKing_Epicenter_Slow:
            pass
        class CPulseCell_IntervalTimer:
            pass
        class C_DOTA_Unit_Hero_Skywrath_Mage:
            pass
        class C_DOTA_Item_Kobold_Cup:
            pass
        class CDOTA_Item_Recipe_Hurricane_Pike:
            pass
        class C_DOTA_Item_Urn_Of_Shadows:
            pass
        class CDOTA_Item_SuperBlinkDagger:
            pass
        class C_DOTA_Ability_EmberSpirit_SearingChains:
            pass
        class C_DOTA_Ability_Ogre_Magi_Fireblast:
            pass
        class C_DOTA_Ability_Silencer_BrainDrain:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Beastmaster_5:
            pass
        class C_DOTA_Ability_Kunkka_Tidebringer:
            pass
        class CDOTA_Modifier_Gold_Bag_Launch:
            pass
        class C_SoundEventOBBEntity:
            pass
        class CDOTA_Modifier_Aghsfort_Aziyog_Underlord_Portal_Warp_Channel_Soundstop:
            pass
        class CDOTA_Modifier_Creature_Flamestrike:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Bane_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Axe_4:
            pass
        class CDOTA_Modifier_Stacking_Exp_Rate_Boost:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Abaddon_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Gold_Income_120:
            pass
        class C_DOTA_Hero_Recorder:
            pass
        class CDOTA_Modifier_Item_HydrasBreath_Poison:
            pass
        class CDOTA_Modifier_Muerta_PartingShot_SoulClone:
            pass
        class CDOTA_Modifier_Hoodwink_Sharpshooter_Windup:
            pass
        class CDOTA_Modifier_VoidSpirit_Dissimilate_Phase:
            pass
        class CDOTA_Modifier_Abaddon_AphoticShield:
            pass
        class CDOTA_Modifier_FacelessVoid_TimeWalk:
            pass
        class CIngameEvent_MuertaReleaseSpring2023:
            pass
        class CPulseTestScriptLib:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Ability_Special_Bonus_Unique_Hoodwink_AcornShot_ArmorCorruption:
            pass
        class CDOTA_Ability_VoidSpirit_IntrinsicEdge:
            pass
        class CDOTA_Ability_Slark_ShadowDance:
            pass
        class C_DOTA_Ability_NightStalker_HunterInTheNight:
            pass
        class C_DOTA_Ability_NightStalker_CripplingFear:
            pass
        class C_DOTA_Ability_Clinkz_Infernal_Shred:
            pass
        class C_DOTA_Ability_Lion_Impale:
            pass
        class C_DOTA_Ability_AntiMage_Blink_Fake:
            pass
        class CDOTA_Item_BookAgility:
            pass
        class CDOTA_Modifier_Special_Bonus_Manaloss_Reduction:
            pass
        class CDOTA_Ability_Special_Bonus_Spell_AoE_25:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_Regen_10:
            pass
        class CDOTA_Modifier_Item_Soul_Booster:
            pass
        class CDOTA_Modifier_Muerta_PartingShot_ProjectileReturn:
            pass
        class CDOTA_Modifier_Marci_Unleash:
            pass
        class CDOTA_Modifier_VoidSpirit_Variant_Equilibrium:
            pass
        class CDOTA_Modifier_Pangolier_ShieldCrash_Buff:
            pass
        class CDOTA_Modifier_MonkeyKing_QuadrupleTap:
            pass
        class CDOTA_Modifier_Brewmaster_Void_Astral_Pull_Movement:
            pass
        class CDOTA_Modifier_Brewmaster_Primal_Companion:
            pass
        class CDOTA_Modifier_Huskar_Life_Break_Taunt:
            pass
        class CDOTA_Modifier_Enchantress_Enchant_Slow:
            pass
        class CDOTA_Modifier_Dazzle_Poison_Touch:
            pass
        class CDOTA_Modifier_Kunkka_GhostShip_Knockback:
            pass
        class CDOTA_Modifier_AncientApparition_ChillingTouch_Slow:
            pass
        class CPulseCell_BaseLerp:
            pass
        class C_DOTA_Unit_Brewmaster_PrimalStorm:
            pass
        class C_DOTA_Unit_Hero_Beastmaster_Beasts:
            pass
        class C_BasePropDoor:
            pass
        class C_DOTAWorldParticleSystem:
            pass
        class CDOTA_Item_Forage_Base:
            pass
        class C_DOTA_Item_CripplingCrossbow:
            pass
        class C_DOTA_Item_MagicStick:
            pass
        class C_DOTA_Ability_Largo_AmphibianRhapsody_GoodVibrations:
            pass
        class C_DOTA_Ability_Batrider_Stoked:
            pass
        class C_DOTA_Ability_DragonKnight_Fireball:
            pass
        class C_DOTA_Ability_Warlock_RainOfChaos:
            pass
        class C_DOTA_Ability_Nian_Apocalypse:
            pass
        class CDOTA_Modifier_Mutation_KillstreakPower:
            pass
        class CDOTA_Ability_AghsFort_Ascension_PlasmaField:
            pass
        class C_DOTA_Ability_Special_Bonus_Agility_16:
            pass
        class C_DOTA_Ability_Special_Bonus_Agility_8:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_125:
            pass
        class CPulseCell_ShmupWaitForDuration__CursorState_t:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class CDOTA_Modifier_Item_DaggerOfRistul:
            pass
        class CDOTA_Modifier_Item_Arcane_Ring:
            pass
        class CDOTA_Modifier_ElixerHealing:
            pass
        class CDOTA_Modifier_Item_Kaya:
            pass
        class CDOTA_Modifier_Item_Orb_of_Venom_Slow:
            pass
        class CDOTA_Modifier_Item_HelmOfTheDominator:
            pass
        class CDOTA_Modifier_AbyssalUnderlord_DarkRift_Bonus_Health:
            pass
        class CDOTA_Modifier_TrollWarlord_BerserkersRage_Maim:
            pass
        class CDOTA_Modifier_Visage_Stone_Form_PreCastRoot:
            pass
        class CDOTA_Modifier_KeeperOfTheLight_Recall_Movespeed:
            pass
        class CDOTA_Modifier_Gyrocopter_Rocket_Barrage:
            pass
        class CDOTA_Modifier_Weaver_Shukuchi:
            pass
        class CDOTA_Modifier_Broodmother_WebWalk_Slow:
            pass
        class CDOTA_Modifier_TemplarAssassin_RefractionDamage:
            pass
        class CDOTA_Modifier_Special_Bonus_Unique_Beastmaster_5_Aura:
            pass
        class CDOTA_Modifier_CallOfTheWild_Boar_PoisonBase:
            pass
        class CDOTA_Modifier_Tidehunter_LeviathansCatch:
            pass
        class CDOTA_Modifier_CrystalMaiden_Let_It_Go_Slow:
            pass
        class CDOTA_Modifier_DisableTauntAnimationCancel:
            pass
        class CDOTA_Modifier_Kill:
            pass
        class C_DOTA_ArcanaDataEntity:
            pass
        class C_DOTA_Ability_BotChallenge_SkeletonKing_BoneGuard:
            pass
        class CDOTA_Item_Recipe_Enchanted_Quiver:
            pass
        class CDOTA_Ability_Abaddon_Frostmourne:
            pass
        class C_DOTA_Ability_TrollWarlord_Fervor:
            pass
        class C_DOTA_Ability_LoneDruid_TrueForm:
            pass
        class C_DOTA_Ability_Brewmaster_DispelMagic:
            pass
        class C_DOTA_Ability_DoomBringer_InfernalBlade:
            pass
        class CDOTA_Ability_AncientApparition_IceVortex:
            pass
        class CDOTA_Modifier_OgreMagi_FrostArmor:
            pass
        class CDOTA_Modifier_Aghsfort_Reward_HPAura_Bonus:
            pass
        class CDOTA_Modifier_Creature_Full_Avoidance:
            pass
        class CDOTA_Modifier_Corpselord_Revive:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lich_8:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Morphling_1:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Crystal_Maiden_6:
            pass
        class C_DOTA_Ability_Special_Bonus_Cleave_30:
            pass
        class C_DOTA_Ability_Special_Bonus_Gold_lvl10_l:
            pass
        class DOTAThreatLevelInfo_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Modifier_Item_Flicker:
            pass
        class CDOTA_Modifier_Item_Holy_Locket_Aura:
            pass
        class CDOTA_Modifier_Item_Satanic_Unholy:
            pass
        class CDOTA_Modifier_Item_Buckler:
            pass
        class CDOTA_Modifier_Item_SheepStick:
            pass
        class CDOTA_Modifier_Kez_ShodoSai_Mark_Slow:
            pass
        class CDOTA_Modifier_Mars_ArenaOfBlood_Buff:
            pass
        class CDOTA_Modifier_Terrorblade_Dark_Unity_Aura:
            pass
        class CDOTA_Modifier_Abyssal_Underling_Archer_AoE_Aura:
            pass
        class CDOTA_Modifier_Shredder_Reactive_Armor_Stack:
            pass
        class CDOTA_Modifier_DoomBringer_Devour:
            pass
        class CDOTA_Modifier_Night_Stalker_MidnightFeast:
            pass
        class CDOTA_Modifier_Lich_FrostShield_Slow:
            pass
        class CDOTA_Unit_Elder_Titan_AncestralSpirit:
            pass
        class C_DOTA_Unit_Hero_KeeperOfTheLight:
            pass
        class CDOTA_Item_Recipe_EchoSabre:
            pass
        class CDOTA_Ability_Ringmaster_CrystalBall:
            pass
        class C_DOTA_Ability_Slark_EssenceShift:
            pass
        class C_DOTA_Ability_LoneDruid_SpiritBear_Return:
            pass
        class CDOTA_Ability_DoomBringer_Lvl_Pain:
            pass
        class C_DOTA_Ability_PhantomAssassin_Fan_Of_Knives:
            pass
        class C_DOTA_Ability_QueenOfPain_Innate_Spell_Reflect:
            pass
        class CDOTA_Ability_Beastmaster_DrumsOfSlom_Stop:
            pass
        class C_DOTA_Ability_Courier_GoToSideShop2:
            pass
        class CDOTA_Modifier_Mutation_Vampire:
            pass
        class CChoreoInfoTarget:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Morphling_10:
            pass
        class CDOTA_Modifier_Special_Bonus_Magic_Resistance:
            pass
        class C_DOTA_Ability_Special_Bonus_Movement_Speed_Percentage_6:
            pass
        class C_DOTACameraBounds:
            pass
        class CDOTA_Modifier_Item_Caster_Rapier:
            pass
        class CDOTA_Modifier_Item_Crimson_Guard_Extra:
            pass
        class CDOTA_Modifier_Kez_Katana:
            pass
        class CDOTA_Modifier_Abaddon_BorrowedTime_Passive:
            pass
        class CDOTA_Modifier_TrollWarlord_SwitchStance:
            pass
        class CDOTA_Modifier_Alchemist_Self_Corrosive_Weaponry:
            pass
        class CDOTA_Modifier_SpiritBreaker_BullRush_Intrinsic:
            pass
        class CDOTA_Modifier_FacelessVoid_TimeZone_Effect:
            pass
        class CDOTA_Modifier_FacelessVoid_Backtrack_Temp:
            pass
        class CTakeDamageResultAPI:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CNetworkedSequenceOperation:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Item_Chasm_Stone:
            pass
        class C_DOTA_Ability_Muerta_PartingShot:
            pass
        class C_DOTA_Ability_DarkWillow_BrambleMaze:
            pass
        class C_DOTA_Ability_Invoker_GhostWalk:
            pass
        class C_DOTA_Ability_ShadowShaman_Serpentine:
            pass
        class C_DOTA_Ability_Tidehunter_Blubber:
            pass
        class CDOTA_Modifier_AghsFort_RockGolem_Avalanche_Stun:
            pass
        class CDOTA_Modifier_Aghsfort_Reward_CritAura_Bonus:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Crystal_Maiden_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Range_250:
            pass
        class C_DOTA_Ability_Special_Bonus_All_Stats_7:
            pass
        class CDOTA_DB_Page_StickerEntity:
            pass
        class CDOTA_Modifier_Item_DefiantShell_Attack:
            pass
        class CDOTA_Modifier_Item_Phylactery:
            pass
        class CDOTA_Modifier_Item_Boots_Of_Bearing_Aura:
            pass
        class CDOTA_Modifier_MonkeyKing_TreeJump_Hidden:
            pass
        class CDOTA_Modifier_Weaver_Shukuchi_GeminateAttackManager:
            pass
        class CDOTA_Modifier_Furion_Treant_HeroDamage:
            pass
        class CDOTA_Modifier_Beastmaster_WildAxes:
            pass
        class CDOTA_Modifier_Nian_EruptionPendingThinker:
            pass
        class CDOTA_Modifier_Shadow_Shaman_Voodoo_Hands:
            pass
        class CDOTA_Modifier_Windrunner_Powershot_Slow:
            pass
        class CDOTA_Modifier_VengefulSpirit_Revenge_Tracker:
            pass
        class CDOTA_Modifier_StormSpirit_StaticRemnant_Talent:
            pass
        class CDOTA_Modifier_Bane_Enfeeble:
            pass
        class CDOTA_Modifier_IceSlide:
            pass
        class CDOTA_Modifier_Rune_FlyingHaste:
            pass
        class C_DOTANewPlayerPoolGameMode:
            pass
        class CEntityInstance:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Unit_Hero_Visage:
            pass
        class C_DOTA_Unit_Hero_Venomancer:
            pass
        class C_DOTA_Unit_Hero_Enigma:
            pass
        class C_DOTA_Item_Nemesis_Curse:
            pass
        class C_DOTA_Item_Recipe_Eternal_Shroud:
            pass
        class CDOTA_Item_Recipe_Paladin_Sword:
            pass
        class C_DOTA_Item_Elixer:
            pass
        class C_DOTA_Item_Headdress:
            pass
        class C_DOTA_Item_Recipe_TranquilBoots:
            pass
        class CDOTA_Ability_Tusk_Tag_Team:
            pass
        class C_DOTA_Ability_Brewmaster_WindWalk:
            pass
        class C_DOTA_Ability_Gyrocopter_Mounting_Platform:
            pass
        class C_DOTA_Ability_Lion_Voodoo:
            pass
        class C_DOTA_Ability_WitchDoctor_VoodooRestoration:
            pass
        class C_DOTA_Ability_ShadowShaman_Shackles:
            pass
        class CDOTA_Modifier_XP_Fountain_Building:
            pass
        class CDOTA_Ability_Seasonal_TI11_RockPaperScissors:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Bloodseeker_6:
            pass
        class CDOTA_Modifier_Special_Bonus_Agility:
            pass
        class C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_12:
            pass
        class CDOTA_Modifier_Item_Ninja_Gear:
            pass
        class CDOTA_Modifier_Kez_TalonToss_Damage:
            pass
        class CDOTA_Modifier_Oracle_DivinersDeck_Intrinsic:
            pass
        class CDOTA_Modifier_Phoenix_Sun_Debuff:
            pass
        class CDOTA_Modifier_EmberSpirit_FlameGuard_Debuff:
            pass
        class CDOTA_Modifier_Legion_Commander_Outfight_Them:
            pass
        class CDOTA_Modifier_Huskar_Cauterize_Delay:
            pass
        class CDOTA_Modifier_Warlock_Rain_Of_Chaos_Thinker:
            pass
        class CDOTA_Modifier_Slardar_SeabornSentinel_Passive:
            pass
        class CDOTA_Modifier_Sniper_Assassinate:
            pass
        class CDOTA_Modifier_Earthshaker_Arcana:
            pass
        class CDOTA_Modifier_Juggernaut_Omnislash:
            pass
        class C_IngameEvent_TI8:
            pass
        class C_DOTA_Ability_Grimstroke_Dark_Portrait:
            pass
        class CDOTA_Ability_Magnataur_Horn_Toss:
            pass
        class C_DOTA_Ability_Visage_SummonFamiliars_Recall:
            pass
        class CDOTA_Ability_NagaSiren_Eelskin:
            pass
        class C_DOTA_Ability_Windrunner_Tailwind:
            pass
        class CDOTA_Modifier_BigThunderLizard_Wardrums_Aura:
            pass
        class C_BaseModelEntity:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Nevermore_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_75:
            pass
        class DOTATeleportInfo_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Modifier_Item_PyrrhicCloak_Target:
            pass
        class CDOTA_Modifier_Item_Enchanted_Mango:
            pass
        class CDOTA_Modifier_Primal_Beast_Trample_Haste:
            pass
        class CDOTA_Modifier_Dawnbreaker_Fire_Wreath_Smash_Stun:
            pass
        class CDOTA_Modifier_LegionCommander_DuelAura:
            pass
        class CDOTA_Modifier_Medusa_StoneGaze_Stone:
            pass
        class CDOTA_Modifier_Medusa_Mystic_Snake_Slow:
            pass
        class CDOTA_Modifier_NagaSiren_Ensnare:
            pass
        class CDOTA_Modifier_Nyx_Assassin_ManaBurn:
            pass
        class CDOTA_Modifier_Disruptor_StaticStormThinker:
            pass
        class CDOTA_Modifier_Shadow_Demon_Purge_Slow:
            pass
        class CDOTA_Modifier_Life_Stealer_Feast:
            pass
        class CDOTA_Modifier_PhantomAssassin_StiflingDagger_Caster:
            pass
        class CDOTA_Modifier_Windrunner_FocusFire:
            pass
        class C_SoundOpvarSetAutoRoomEntity:
            pass
        class CDOTA_Item_Recipe_TranquilBoots2:
            pass
        class CDOTA_Ability_Techies_SnareTrap:
            pass
        class C_DOTA_Ability_Shredder_Flamethrower:
            pass
        class CDOTA_Ability_LoneDruid_Entangle:
            pass
        class C_DOTA_Ability_DoomBringer_Devour:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Batrider_6:
            pass
        class CDOTA_Modifier_SatyrHellcaller_UnholyAura_Bonus:
            pass
        class CDOTA_Modifier_Mutation_DeathExplosion_TeamAura:
            pass
        class C_EnvCombinedLightProbeVolume:
            pass
        class CDOTA_Modifier_Aghsfort_Elemental_Wisp_Tether_Slow:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Slark_7:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Slardar_7:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Antimage_2:
            pass
        class CDOTA_Modifier_Special_Bonus_MP_Regen:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Amplify_7:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_400:
            pass
        class CDOTA_Modifier_Item_Shivas_Guard_Blast:
            pass
        class CDOTA_Modifier_Underlord_Portal_Warp_Channel:
            pass
        class CDOTA_Modifier_Techies_LandMine_Burn:
            pass
        class CDOTA_Modifier_Ogre_Magi_Arcana:
            pass
        class CDOTA_Modifier_SpiritBreaker_BullsHit:
            pass
        class CDOTA_Modifier_DeathProphet_SpiritSiphon_Slow:
            pass
        class CDOTA_Modifier_Nian_Intrinsic:
            pass
        class CDOTA_Modifier_Slardar_Sprint:
            pass
        class CDOTA_Modifier_Pudge_Swallow:
            pass
        class CDOTA_Modifier_Antimage_Persectur:
            pass
        class C_MultiplayRules:
            pass
        class CPlayer_AutoaimServices:
            pass
        class C_DOTA_Item_DragonLance:
            pass
        class C_DOTA_Ability_Muerta_PierceTheVeil:
            pass
        class CDOTA_Ability_Abyssal_Underling_Warrior_ManaBurn:
            pass
        class C_DOTA_Ability_Viper_Innate_Corrosive_Skin_Virulent:
            pass
        class C_DOTA_Ability_ShadowShamanVoodoo:
            pass
        class C_DOTA_Aghsfort_AbilityCrystalMaiden_FreezingField:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Elder_Titan:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Slark_6:
            pass
        class CDOTA_Modifier_Special_Bonus_20_Bash:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Mirana_2:
            pass
        class CDOTA_Ability_Special_Bonus_Spell_AoE_50:
            pass
        class NagaSiren_SongOfTheSiren_Healing:
            pass
        class CDOTA_Modifier_Chaos_Knight_Illusions_Damage_Reduction_Aura:
            pass
        class CDOTA_Modifier_ForgedSpirit_Stats:
            pass
        class CDOTA_Modifier_Rattletrap_RocketFlare_Overclock:
            pass
        class CDOTA_Modifier_FacelessVoid_TimeZone_Projectile_Effect:
            pass
        class CDOTA_Modifier_Ursa_Fury_Swipes_Damage_Increase:
            pass
        class CDOTA_Modifier_Shadow_Shaman_Fowl_Play_Damage_Reduction:
            pass
        class CDOTA_Modifier_Zuus_ThundergodsWrathThinker:
            pass
        class CDOTA_Modifier_Drow_Ranger_Glacier_Shard:
            pass
        class CDOTACourierController:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_LightDirectionalEntity:
            pass
        class C_DOTA_Item_Giant_Maul:
            pass
        class C_DOTA_Item_Recipe_Orb_Of_Corrosion:
            pass
        class C_DOTA_Item_Recipe_Pipe:
            pass
        class C_DOTA_Item_Recipe_LesserCritical:
            pass
        class C_DOTA_Item_EnergyBooster:
            pass
        class CDOTA_Modifier_Aghsfort_Wildwing_Tornado_Blast_Debuff:
            pass
        class C_DOTA_Ability_Special_Bonus_Evasion_8:
            pass
        class C_DOTA_Ability_Special_Bonus_Haste:
            pass
        class CDOTA_Modifier_Item_Conjurers_Catalyst:
            pass
        class CDOTA_Modifier_Item_InvisibilityEdge:
            pass
        class CDOTA_Modifier_Largo_Encore:
            pass
        class CDOTA_Modifier_Hoodwink_Camouflage:
            pass
        class CDOTA_Modifier_Mars_ArenaOfBlood_Animation:
            pass
        class CDOTA_Modifier_Meepo_MegaMeepo_Frame_Invulnerable:
            pass
        class CDOTA_Modifier_LoneDruid_TrueForm_BattleCry:
            pass
        class CDOTA_Modifier_LoneDruid_SpiritBear_AttackCheck:
            pass
        class CDOTA_Modifier_Brewmaster_ThunderClap:
            pass
        class CDOTA_Modifier_Invoker_DeafeningBlast_Knockback:
            pass
        class CDOTA_Modifier_Spectre_Spectral_Dagger_Illusions:
            pass
        class CDOTA_Modifier_Chen_Zealot_Buff:
            pass
        class CDOTA_Modifier_Clinkz_Strafe_Debuff:
            pass
        class CDOTA_Modifier_Earthshaker_Fissure_Thinker:
            pass
        class CDOTA_Modifier_SkeletonKing_Reincarnation_SpawnSkeletons:
            pass
        class CDOTA_Modifier_AncientApparition_IceVortex:
            pass
        class CDOTACustomShopInfo:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_BaseEntity:
            pass
        class ActiveModelConfig_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Item_Gossamer_Cape:
            pass
        class CDOTA_Item_PogoStick:
            pass
        class C_DOTA_Item_Recipe_Urn_Of_Shadows:
            pass
        class C_DOTA_Ability_Pangolier_Swashbuckle:
            pass
        class CDOTA_Ability_Abaddon_BorrowedTime:
            pass
        class CDOTA_Ability_Warlock_Black_Grimoire:
            pass
        class C_DOTA_Ability_Bane_Ichor_Of_Nyctasha:
            pass
        class CDOTA_Item_Aghsfort_BootsOfTravel_2:
            pass
        class CDOTA_Modifier_AghsFort_Firefly:
            pass
        class CDOTA_Ability_Seasonal_Summon_CNY_Balloon:
            pass
        class C_DOTA_Ability_Special_Bonus_Armor_15:
            pass
        class C_DOTA_Ability_Special_Bonus_Strength_12:
            pass
        class CDOTA_Modifier_Item_Greater_Faerie_Fire:
            pass
        class CDOTA_Modifier_Item_MagicStick_Hero:
            pass
        class CDOTA_Modifier_TangoHeal:
            pass
        class CDOTA_Modifier_Muerta_PierceTheVeil_MagicImmunityDamageCancel:
            pass
        class CDOTA_Modifier_Disruptor_Glimpse:
            pass
        class CDOTA_Modifier_Undying_Tombstone_Bunker_Friendly:
            pass
        class CDOTA_Modifier_Gyrocopter_Call_Down:
            pass
        class CDOTA_Modifier_Rattletrap_Cog_Thinker_Talent:
            pass
        class CDOTA_Modifier_Beastmaster_DrumsOfSlom_Aura:
            pass
        class CDOTA_Modifier_NianCharge:
            pass
        class CDOTA_Modifier_Roshan_RevengeRoar_Aura:
            pass
        class CDOTA_Modifier_Roshan_Devotion:
            pass
        class CDOTA_Modifier_WitchDoctor_Cask_Thinker:
            pass
        class CDOTA_Modifier_FountainInvulnerabilityBuff:
            pass
        class CPulseCell_Value_Curve:
            pass
        class C_DOTA_Unit_SpiritBear:
            pass
        class C_DOTA_Unit_Hero_AntiMage:
            pass
        class C_BasePlayerPawn:
            pass
        class C_DOTA_Item_Enhancement_Fierce:
            pass
        class C_DOTA_Item_Recipe_Necronomicon:
            pass
        class C_DOTA_Item_Recipe_PoorMansShield:
            pass
        class C_DOTA_Ability_PrimalBeast_Onslaught:
            pass
        class C_DOTA_Ability_Life_Stealer_Infest:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Visage_3:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Shadow_Shaman_1:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_200:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_Regen_10:
            pass
        class CDOTA_Modifier_BotChallenge_SkeletonKing_BoneGuard_DamageTracker:
            pass
        class CDOTA_Modifier_Silver_Edge_Debuff:
            pass
        class CDOTA_Modifier_Ringmaster_WhoopeeCushion_Active:
            pass
        class CDOTA_Modifier_Dawnbreaker_Solar_Guardian_Disable:
            pass
        class CDOTA_Modifier_Hoodwink_AcornShot_BonusDamage:
            pass
        class CDOTA_Modifier_Undying_CeaselessDirge_Buff:
            pass
        class CDOTA_Modifier_Undying_Tombstone_Zombie_Deathstrike:
            pass
        class CDOTA_Modifier_Meepo_Divided_We_Stand_SupportGroup:
            pass
        class CDOTA_Modifier_Invoker_Tornado:
            pass
        class CDOTA_Modifier_Spectre_Spectral:
            pass
        class CDOTA_Modifier_DarkSeer_Surge:
            pass
        class CDOTA_Modifier_Slardar_SeabornSentinel_River:
            pass
        class CDOTA_Modifier_Sniper_Assassinate_Crit:
            pass
        class CDOTA_Modifier_Enigma_MidnightPulse_Damage:
            pass
        class CDOTA_Modifier_Bashed:
            pass
        class C_SoundOpvarSetAABBEntity:
            pass
        class C_DOTA_Unit_Aghsfort_Aziyog_Underlord_Portal:
            pass
        class C_DOTA_Item_Foragers_Kit:
            pass
        class C_DOTA_Item_Enhancement_Audacious:
            pass
        class C_DOTA_Item_Recipe_Kaya:
            pass
        class C_DOTA_Item_GemOfTrueSight:
            pass
        class C_DOTA_Ability_Kez_RaptorDance:
            pass
        class CDOTA_Ability_Elder_Titan_ReturnSpirit:
            pass
        class C_DOTA_Ability_Rubick_SpellSteal:
            pass
        class C_DOTA_Ability_Omniknight_Innate_Degen_Aura_Radius:
            pass
        class CDOTA_Ability_Omniknight_Angelic_Flight:
            pass
        class C_DOTA_Ability_Pugna_Decrepify:
            pass
        class C_DOTA_Ability_ShadowShaman_EtherShock:
            pass
        class C_DOTA_Ability_Mirana_Nightveil:
            pass
        class C_DOTA_Ability_Bane_NightmareEnd:
            pass
        class C_DOTA_Ability_BlackDragon_SplashAttack:
            pass
        class CDOTA_Modifier_AghsFort_Morphling_Waveform:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Zeus_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_55:
            pass
        class CDOTA_Modifier_Item_Craggy_Coat:
            pass
        class CDOTA_Modifier_Item_CrellasCrozier_Movespeed_Buff:
            pass
        class CDOTA_Modifier_Primalbeast_Trample_Attack_Bonus:
            pass
        class CDOTA_Modifier_Spectre_SpectralDaggerPath:
            pass
        class CDOTA_Modifier_Jakiro_LiquidIce_Debuff:
            pass
        class CDOTA_Modifier_PhantomAssassin_Fan_Of_Knives_Thinker:
            pass
        class CDOTA_Modifier_Nevermore_Requiem_Slow:
            pass
        class CQuickBuyController:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Unit_Hero_Snapfire:
            pass
        class C_DOTA_BaseNPC_Filler:
            pass
        class C_DOTA_Item_Recipe_Bullwhip:
            pass
        class C_DOTA_Item_Recipe_Book_Of_Shadows:
            pass
        class C_DOTA_Item_Buckler:
            pass
        class C_DOTA_Item_LesserCritical:
            pass
        class C_DOTA_Item_Tango_Single:
            pass
        class C_DOTA_Ability_Largo_CatchyLick:
            pass
        class C_DOTA_Ability_ArcWarden_Scepter:
            pass
        class C_DOTA_Ability_Silencer_Irrepressible:
            pass
        class CDOTA_Ability_Alchemist_UnstableConcoctionThrow:
            pass
        class C_DOTA_Ability_Enigma_Innate_EventHorizon:
            pass
        class CDOTA_Ability_VengefulSpirit_Magic_Missile:
            pass
        class CDOTA_Modifier_Greevil_Miniboss_Green_LivingArmor:
            pass
        class C_DOTA_Ability_XP_Fountain:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Undying_7:
            pass
        class CDOTA_Modifier_Special_Bonus_Spell_Lifesteal:
            pass
        class CDOTA_Modifier_Special_Bonus_Cleave:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Omniknight_6:
            pass
        class C_DOTA_Ability_Special_Bonus_Agility_20:
            pass
        class C_DOTA_Ability_Special_Bonus_Strength_7:
            pass
        class CDOTA_Modifier_Item_SeedsOfSerenity_Active_Aura:
            pass
        class CDOTA_Modifier_Mars_ArenaOfBlood_BuffAura:
            pass
        class CDOTA_Modifier_Abyssal_Underling_Warrior_Sight:
            pass
        class CDOTA_Modifier_Medusa_Cold_Blooded:
            pass
        class CDOTA_Modifier_Visage_Silent_As_The_Grave:
            pass
        class CDOTA_Modifier_LoneDruid_SpiritLink:
            pass
        class CDOTA_Modifier_Gyrocopter_Call_Down_Thinker_Tracking:
            pass
        class CDOTA_Modifier_Chen_Divine_Favor_Armor:
            pass
        class CDOTA_Modifier_TemplarAssassin_MeldArmor:
            pass
        class CDOTA_Modifier_Lion_FingerOfDeath_Kill_Counter:
            pass
        class CDOTA_Modifier_Kunkka_Tidebringer_ArmorPiercing:
            pass
        class CDOTA_Modifier_AncientApparition_ChillingTouch_SuperSlow:
            pass
        class CDOTA_Modifier_ChangeTreeModel_Thinker:
            pass
        class CDOTA_Modifier_Bonus_Mres:
            pass
        class C_DOTA_Item_Blitz_Knuckles:
            pass
        class C_DOTA_Item_Recipe_Dagon2:
            pass
        class C_DOTA_Item_Recipe_Cyclone:
            pass
        class C_DOTA_Item_StaffOfWizardry:
            pass
        class C_DOTA_Ability_EarthSpirit_BoulderSmash:
            pass
        class C_DOTA_Ability_Shredder_TwistedChakram:
            pass
        class C_DOTA_Ability_Wisp_Tether:
            pass
        class C_DOTA_Ability_Meepo_Earthbind:
            pass
        class CDOTA_Ability_Shadow_Demon_Shadow_Poison:
            pass
        class C_DOTA_Ability_Batrider_FlamingLasso:
            pass
        class C_DOTA_Ability_Lich_Ice_Spire:
            pass
        class CEnvSoundscapeProxy:
            pass
        class C_SoundEventEntity:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Tusk_4:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Troll_Warlord_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Centaur_4:
            pass
        class CDOTA_Modifier_Item_Stonefeather_Satchel:
            pass
        class CDOTA_Modifier_Item_Searing_Signet:
            pass
        class CDOTA_Modifier_Muerta_TheCalling_Aura_HPRegen:
            pass
        class CDOTA_Modifier_Techies_StickyBomb_Chase:
            pass
        class CDOTA_Modifier_Undying_Tombstone_Zombie_Aura:
            pass
        class CDOTA_Modifier_Weaver_Threads_Of_Fate_Established:
            pass
        class CDOTA_Modifier_Huskar_Blood_Magic:
            pass
        class CDOTA_Modifier_Furion_WrathOfNature_Thinker:
            pass
        class CDOTA_Modifier_Nian_Torrent_Thinker:
            pass
        class CDOTA_Modifier_Puck_Coil_Break_Stun:
            pass
        class CDOTA_ModifierManager:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CPulseCell_Inflow_EventHandler:
            pass
        class C_DOTA_Unit_IngisFatuus:
            pass
        class C_DOTA_BaseNPC_XP_Fountain:
            pass
        class C_LightOrthoEntity:
            pass
        class C_DOTA_Item_Enhancement_Nimble:
            pass
        class C_DOTA_Item_Recipe_MaskOfDispair:
            pass
        class C_DOTA_Item_BeltOfStrength:
            pass
        class C_DOTA_Ability_Invoker_DeafeningBlast_AD:
            pass
        class C_DOTA_Ability_Spectre_ShadowStep:
            pass
        class C_DOTA_Ability_Earthshaker_Slugger:
            pass
        class C_DOTA_Ability_Sven_StormBolt:
            pass
        class CDOTA_Ability_Frogmen_WaterBubble_Small:
            pass
        class CDOTA_Modifier_Flagbearer_Creep_Aura_Effect:
            pass
        class CDOTA_Modifier_WarpineRaider_SeedShot_Slow:
            pass
        class CDOTA_Modifier_Mutation_DeathExplosion:
            pass
        class CDOTA_Item_Book:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Jakiro_6:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Clockwerk:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Templar_Assassin:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Dark_Seer_13:
            pass
        class CDOTA_Modifier_Special_Bonus_TrueStrike:
            pass
        class C_DOTA_Ability_Special_Bonus_Strength_20:
            pass
        class C_DOTA_Ability_Special_Bonus_Strength_10:
            pass
        class C_DOTA_Ability_Special_Bonus_Mana_Reduction_8:
            pass
        class CDOTA_Modifier_Item_SeedsOfSerenity:
            pass
        class CDOTA_Modifier_Ringmaster_Spotlight_Debuff:
            pass
        class CDOTA_Modifier_Tusk_Drinking_Buddies_Buff:
            pass
        class CDOTA_Modifier_Tusk_Tag_Team_Slow:
            pass
        class CDOTA_Modifier_Slark_EssenceShift_Permanent_Debuff:
            pass
        class CDOTA_Modifier_Meepo_Innate_ToughChoices:
            pass
        class CDOTA_Modifier_Broodmother_SpinWeb:
            pass
        class CDOTA_Modifier_Warlock_Golem_Flaming_Fists:
            pass
        class CDOTA_Modifier_WitchDoctor_Voodoo_Restoration_Aura:
            pass
        class CDOTA_Modifier_Riki_BlinkStrike_Slow:
            pass
        class CDOTA_Modifier_Morphling_EbbAndFlow:
            pass
        class CDOTA_Modifier_Crystal_Maiden_Crystal_Clone_Statue:
            pass
        class CDOTA_Modifier_SkeletonKing_Reincarnation_Slow:
            pass
        class CPulseCell_BaseFlow:
            pass
        class C_DOTA_Unit_Hero_Clinkz:
            pass
        class CDOTA_Ability_Grimstroke_SoulChain:
            pass
        class C_DOTA_Ability_Techies_MutuallyAssuredDestruction:
            pass
        class C_DOTA_Ability_FacelessVoid_Backtrack:
            pass
        class C_DOTA_Item_UpgradedMortar:
            pass
        class C_DOTA_Ability_Special_Bonus_Magic_Resistance_6:
            pass
        class C_DOTA_Item:
            pass
        class CDOTA_Modifer_Item_WeightedDice:
            pass
        class CDOTA_Modifier_Outworld_Staff:
            pass
        class CDOTA_Modifier_Item_SerratedShiv:
            pass
        class CDOTA_Modifier_Item_HydrasBreath_ProcDamage:
            pass
        class CDOTA_Modifier_Muerta_PierceTheVeil_Buff:
            pass
        class CDOTA_Modifier_Mars_ArenaOfBlood_Marker:
            pass
        class CDOTA_Modifier_Grimstroke_SoulChain:
            pass
        class CDOTA_Modifier_ArcWarden_SparkWraith_Thinker:
            pass
        class CDOTA_Modifier_Skywrath_Mage_Ancient_Seal:
            pass
        class CDOTA_Modifier_Undying_CeaselessDirge:
            pass
        class CDOTA_Modifier_Ogre_Magi_Item_Multicast:
            pass
        class CDOTA_Modifier_Batrider_Stoked:
            pass
        class CDOTA_Modifier_QueenOfPain_Arcana_Kill_Effect:
            pass
        class CDOTA_Modifier_Morphling_Syntropy_Slow:
            pass
        class CDOTA_Modifier_DrowRanger_WaveOfSilence_Knockback:
            pass
        class CDOTA_Modifier_StormSpirit_Overload_Passive:
            pass
        class CDOTA_Modifier_Rune_DoubleDamage:
            pass
        class CDOTA_Unit_Hero_Void_Spirit:
            pass
        class C_DOTA_Unit_Hero_TrollWarlord:
            pass
        class C_DOTA_Unit_Nian:
            pass
        class CDOTA_Item_SuperArcane_Blink:
            pass
        class C_DOTA_Item_MaskOfDispair:
            pass
        class C_DOTA_Item_Blight_Stone:
            pass
        class C_DOTA_Item_Claymore:
            pass
        class CDOTA_Ability_VoidSpirit_AstralStep:
            pass
        class C_DOTA_Ability_BlueDragonspawnSorcerer_Evasion:
            pass
        class CDOTA_Ability_Spawnlord_Aura:
            pass
        class CDOTA_Modifier_PineCone_AcornShot_BonusDamage:
            pass
        class CDOTA_Ability_AghsFort_Spectre_ActiveDispersion:
            pass
        class CDOTA_Ability_Ascension_AcidBlood:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Medusa_3:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Treant_7:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Luna_2:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Pudge_1:
            pass
        class C_DOTA_Ability_Special_Bonus_Magic_Resistance_20:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_250:
            pass
        class CDOTA_Modifier_Item_Pipe_Barrier:
            pass
        class CDOTA_Modifier_Item_Battlefury:
            pass
        class CDOTA_Modifier_Item_Shawl:
            pass
        class CDOTA_Modifier_NyxAssassin_NeuroSting:
            pass
        class CDOTA_Modifier_SpiritBreaker_ChargeOfDarkness:
            pass
        class CDOTA_Modifier_DragonKnight_Fireball_Burn:
            pass
        class CDOTA_Modifier_AntiMage_Empowered_ManaBreak_Debuff:
            pass
        class CSkeletonInstance:
            pass
        class CEntityComponent:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_BaseNPC_Watch_Tower:
            pass
        class CDOTA_Item_Recipe_Iron_Talon:
            pass
        class C_DOTA_Item_Gauntlets:
            pass
        class C_DOTA_Ability_Ringmaster_WhoopeeCushion:
            pass
        class CDOTA_Modifier_Furbolg_Enrage_Damage_OnDeath:
            pass
        class CDOTA_Modifier_Mutation_DeathExplosion_Aura:
            pass
        class C_DOTA_Ability_Aghsfort_Aziyog_Underlord_Dark_Portal:
            pass
        class CDOTA_Modifier_Special_Bonus_Spell_AoE:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_150:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_25:
            pass
        class C_DOTA_Ability_Special_Bonus_Magic_Resistance_100:
            pass
        class CDOTA_Modifier_BottleRegeneration:
            pass
        class CDOTA_Modifier_Hoodwink_Sharpshooter_Recoil:
            pass
        class CDOTA_Modifier_EmberSpirit_FireRemnant_Timer:
            pass
        class CDOTA_Modifier_Lycan_SummonWolves_Spirited:
            pass
        class CDOTA_Modifier_Broodmother_IncapacitatingBiteOrb:
            pass
        class CDOTA_Modifier_Beastmaster_Mark_Of_The_Beast:
            pass
        class CDOTA_Modifier_Puck_Puckish:
            pass
        class CDOTA_Modifier_PhantomLancer_IllusoryArmaments:
            pass
        class CDOTA_Modifier_DrowRanger_Marksmanship_Aura_Bonus:
            pass
        class CDOTA_Modifier_Oracle_Prognosticate:
            pass
        class C_DOTA_PortraitEntity_FullBody:
            pass
        class CDOTA_Item_Recipe_Nullifier:
            pass
        class CDOTA_Item_Recipe_Octarine_Core:
            pass
        class C_DOTA_Item_MantaStyle:
            pass
        class C_DOTA_Ability_PrimalBeast_Pulverize:
            pass
        class C_DOTA_Ability_Oracle_FalsePromise:
            pass
        class C_DOTA_Ability_Slark_Fish_Bait:
            pass
        class C_DOTA_Ability_Nyxth_Sense:
            pass
        class C_DOTA_Ability_Treant_NaturesGuise:
            pass
        class C_DOTA_Ability_Enigma_DemonicConversion:
            pass
        class C_DOTA_Ability_ShadowShaman_MassSerpentWard:
            pass
        class C_DOTA_Ability_StormSpirit_StaticRemnant:
            pass
        class CDOTA_Modifier_KoboldTunneler_ProspectingAura_Money:
            pass
        class CDOTA_Modifier_SatyrSoulstealer_ManaBurn:
            pass
        class C_DOTA_Ability_EnragedWildkin_Tornado:
            pass
        class CDOTA_Modifier_Lotus_Pool_Pickup:
            pass
        class C_DOTA_Ability_Special_Bonus_Lifesteal_10:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_253:
            pass
        class CDOTA_Modifier_Item_Consecrated_Wraps:
            pass
        class CDOTA_Modifier_Silver_Edge_WindWalk:
            pass
        class CDOTA_Modifier_Item_EnergyBooster:
            pass
        class CDOTA_Modifier_Ringmaster_TheBox_Rescale:
            pass
        class CDOTA_Modifier_Oracle_Clairvoyant_Curse:
            pass
        class CDOTA_Modifier_WitchDoctor_DeathWard:
            pass
        class CDOTA_Modifier_Riki_BlinkStrike_ChargeCounter:
            pass
        class CDOTA_Modifier_Puck_PhaseShift_AttackBonus:
            pass
        class CDOTA_Modifier_Kunkka_XMarksTheSpot_Thinker:
            pass
        class CDOTA_Modifier_Lina_FierySoul:
            pass
        class CPulseCell_Outflow_CycleShuffled__InstanceState_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CPulseCell_BaseLerp__CursorState_t:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class CDOTA_Item_Woodland_Striders:
            pass
        class C_DOTA_Item_DemonEdge:
            pass
        class C_DOTA_Ability_Bear_Empty1:
            pass
        class C_DOTA_Ability_LoneDruid_TrueForm_Druid:
            pass
        class C_DOTA_Ability_QueenOfPain_ShadowStrike:
            pass
        class C_DOTA_Ability_AncientApparition_Innate_Absolute_Zero:
            pass
        class CDOTA_Modifier_Launchpad_Aura:
            pass
        class CDOTA_Modifier_Aghsfort_Minor_Stats_Upgrade:
            pass
        class C_DOTA_Ability_Throw_Coal:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Underlord:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_66:
            pass
        class C_DOTA_Ability_Special_Bonus_Reincarnation_200:
            pass
        class CDOTA_Modifier_Item_AsceticCap:
            pass
        class CDOTA_Modifier_Item_Assault_Cuirass_Positive_Aura:
            pass
        class CDOTA_Modifier_LoneDruid_TrueForm:
            pass
        class CDOTA_Modifier_Invoker_ColdSnap_Freeze:
            pass
        class CDOTA_Modifier_Furion_WrathOfNature_Spawn:
            pass
        class CDOTA_Modifier_Dazzle_Shallow_Grave:
            pass
        class CDOTA_Modifier_Dazzle_Rain_Of_Vermin:
            pass
        class CDOTA_Modifier_QueenOfPain_ShadowStrike:
            pass
        class CDOTA_Modifier_Special_Bonus_Unique_Beastmaster_6:
            pass
        class CDOTA_Modifier_Warlock_EldritchSummoning:
            pass
        class CDOTA_Modifier_Nian_Flag_Trap_Thinker:
            pass
        class CDOTA_Modifier_Necrophos_Death_Seeker_Ethereal:
            pass
        class CDOTA_Modifier_Riki_Poison_Dart_Debuff:
            pass
        class CDOTA_Modifier_SkeletonKing_SpectralBladeIntrinsic:
            pass
        class CDOTA_Modifier_SkeletonKing_HellfireBlast:
            pass
        class C_DOTA_Unit_Hero_Muerta:
            pass
        class C_DOTA_Unit_Hero_Viper:
            pass
        class C_DOTA_BaseNPC_Tower:
            pass
        class C_DOTA_Ability_Ringmaster_FunhouseMirror:
            pass
        class C_DOTA_Ability_Wisp_Tether_Break:
            pass
        class C_DOTA_Ability_SpiritBreaker_EmpoweringHaste:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Batrider_3:
            pass
        class C_DOTA_Ability_VengefulSpirit_WaveOfTerror:
            pass
        class C_DOTA_Ability_Sven_Wrath_Of_God:
            pass
        class C_DOTA_Ability_AncientGolem_Rockslide:
            pass
        class CDOTA_Modifier_DarkTrollWarlord_Ensnare:
            pass
        class CDOTA_Modifier_PudgeMiniboss_ArmorCorruptionDebuff:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Hoodwink_SharpshooterPierceHeroes:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Slardar:
            pass
        class C_DOTA_Ability_Special_Bonus_Magic_Resistance_5:
            pass
        class C_DotaTutorialNetworker:
            pass
        class CDOTA_Modifier_Item_SpecialistsArray:
            pass
        class CDOTA_Modifier_Item_Witless_shako:
            pass
        class CDOTA_Modifier_Item_Royal_Jelly:
            pass
        class CDOTA_Modifier_ClarityPotion:
            pass
        class CDOTA_Modifier_Hoodwink_HuntersQuiver:
            pass
        class CDOTA_Modifier_Underlord_Portal_FX:
            pass
        class CDOTA_Modifier_Leshrac_Split_Earth_Thinker:
            pass
        class CDOTA_Modifier_Drow_Ranger_Glacier_Hilltop_Aura:
            pass
        class CPulseAnimFuncs:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Item_Satanic:
            pass
        class C_DOTA_Item_GreaterCritical:
            pass
        class CDOTA_Ability_Miniboss_Radiance:
            pass
        class CDOTA_Modifier_Greevil_Miniboss_Yellow_Surge:
            pass
        class CDOTA_Modifier_IceShaman_IncendiaryBomb:
            pass
        class C_BaseClientUIEntity:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Pudge_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Omniknight_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Ancient_Apparition_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Movement_Speed_Percentage_12:
            pass
        class CDOTA_Modifier_Penta_Edged_Sword_Maim:
            pass
        class CDOTA_Modifier_Pogo_Stick_Active:
            pass
        class CDOTA_Modifier_Item_Ironwood_tree:
            pass
        class CDOTA_Modifier_Item_Skadi_Slow:
            pass
        class CDOTA_Modifier_Item_Bloodstone_Drained:
            pass
        class CDOTA_Modifier_Item_TalismanOfEvasion:
            pass
        class CDOTA_Modifier_Item_StoutShield:
            pass
        class CDOTA_Modifier_Item_VitalityBooster:
            pass
        class CDOTA_Modifier_Tusk_Tag_Team:
            pass
        class CDOTA_Modifier_Bristleback_QuillSpray_Thinker:
            pass
        class CDOTA_Modifier_Wisp_Tether_Slow:
            pass
        class CDOTA_Modifier_Disruptor_StaticStormMute:
            pass
        class CDOTA_Modifier_Dazzle_Weave_Armor:
            pass
        class CDOTA_Modifier_PhantomAssassin_Blur:
            pass
        class CDOTA_Modifier_Tiny_Avalanche:
            pass
        class CDOTA_Modifier_NoHealthBar:
            pass
        class CPulseCell_WaitForCursorsWithTagBase__CursorState_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CPulseArraylib:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_TriggerLerpObject:
            pass
        class C_DOTA_Item_Phylactery:
            pass
        class CDOTA_Ability_Wisp_Equilibrium:
            pass
        class CDOTA_Ability_LoneDruid_Innate_GiftBearer:
            pass
        class C_DOTA_Ability_LoneDruid_SavageRoar_Bear:
            pass
        class CDOTA_Ability_Bloodseeker_Bloodbath:
            pass
        class CDOTA_Ability_Plus_HighFive:
            pass
        class CDOTA_Item_Aghsfort_BootsOfTravel:
            pass
        class CDOTA_Modifier_Aghsfort_AggroOverride:
            pass
        class CDOTA_Ability_AghsFort_Ascension_Silence:
            pass
        class CDOTA_Modifier_Aghsfort_TempBuff_CorpseExplosion_Aura:
            pass
        class CDOTA_Modifier_Item_Falcon_Blade:
            pass
        class CDOTA_Modifier_Item_Octarine_Core:
            pass
        class CDOTA_Modifier_Item_OblivionStaff:
            pass
        class CDOTA_Modifier_Largo_FroglingBand2_Frogling_Vanish:
            pass
        class CDOTA_Modifier_Mars_Spear_Stun:
            pass
        class CDOTA_Modifier_Pangolier_LuckyShot_Disarm:
            pass
        class CDOTA_Modifier_Winter_Wyvern_Splinter_Blast_Slow:
            pass
        class CDOTA_Modifier_Terrorblade_Reflection_Slow:
            pass
        class CDOTA_Modifier_Slark_ShadowDance:
            pass
        class CDOTA_Modifier_Batrider_Firefly:
            pass
        class CDOTA_Modifier_Pugna_Oblivion_Savant:
            pass
        class CDOTA_Modifier_Beastmaster_Summon_Raptor:
            pass
        class CDOTA_Modifier_VengefulSpirit_Command_Aura:
            pass
        class CDOTA_Modifier_Sand_King_Scorpion_Strike_Attack_Bonus:
            pass
        class CDOTA_Modifier_Bane_Ichor_Of_Nyctasha:
            pass
        class CDOTA_Modifier_AntiMage_Thirst_Effect:
            pass
        class CDOTA_Modifier_Disarmed:
            pass
        class CPointTemplateAPI:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Unit_Hero_Tidehunter:
            pass
        class C_DOTA_Unit_Hero_SandKing:
            pass
        class C_DynamicPropAlias_cable_dynamic:
            pass
        class CBaseProp:
            pass
        class CDOTA_Item_Recipe_Apex:
            pass
        class CDOTA_Item_Ninja_Gear:
            pass
        class CDOTA_Item_Recipe_Trusty_Shovel:
            pass
        class C_DOTA_Ability_Hoodwink_Sharpshooter_Release:
            pass
        class CDOTA_Ability_VoidSpirit_ResonantPulse:
            pass
        class CDOTA_Ability_Mars_Immovable:
            pass
        class C_DOTA_Ability_Phoenix_BlindingSun:
            pass
        class CDOTA_Ability_Treant_EyesInTheForest:
            pass
        class C_DOTA_Ability_Rattletrap_JetPack_Toggle:
            pass
        class C_DOTA_Ability_Enigma_Event_Horizon:
            pass
        class CDOTA_Modifier_MonsterHunterEvent:
            pass
        class CDOTA_Ability_BigThunderLizard_Wardrums:
            pass
        class CDOTA_Modifier_WarpineRaider_SeedShot:
            pass
        class CInfoOffscreenPanoramaTexture:
            pass
        class CDOTA_Modifier_AghsFort_Spectre_ActiveDispersion_Thinker:
            pass
        class CDOTA_Modifier_AghsFort_Ravage_Potion:
            pass
        class C_DOTA_Ability_Special_Bonus_Exp_Boost_60:
            pass
        class C_DOTA_Ability_Special_Bonus_Mana_Break_40:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_250:
            pass
        class CDOTA_Modifier_Item_Force_Field:
            pass
        class CDOTA_Modifier_Muerta_Revenant_HealingThinker:
            pass
        class CDOTA_Modifier_VoidSpirit_Dissimilate_Invis:
            pass
        class CDOTA_Modifier_Abyssal_Underling_Warrior_LastWill:
            pass
        class CDOTA_Modifier_Techies_Minefield_Sign_Aura:
            pass
        class CDOTA_Modifier_Elder_Titan_AncestralSpirit_CastTime:
            pass
        class CDOTA_Modifier_Treant_NaturesGuise_Tree_Walking:
            pass
        class CDOTA_Modifier_Spectre_SpectralDaggerPath_ActivityModifierOnly:
            pass
        class CDOTA_Dazzle_NothlProjection_PhysicalBodyDebuff:
            pass
        class CDOTA_Modifier_FacelessVoid_Arcana_Kill_Effect:
            pass
        class CDOTA_Modifier_Teleporting:
            pass
        class C_DOTA_Unit_Hero_DrowRanger:
            pass
        class C_DOTA_Item_DuelistGloves:
            pass
        class C_DOTA_Item_Recipe_Sorcerers_Staff:
            pass
        class C_DOTA_Item_Blood_Grenade:
            pass
        class C_DOTA_Ability_Hoodwink_Caltrops:
            pass
        class C_DOTA_Ability_Phoenix_LaunchFireSpirit:
            pass
        class C_DOTA_Ability_Kunkka_XMarksTheSpot:
            pass
        class CDOTA_Modifier_AlphaWolf_CommandAura_Bonus:
            pass
        class C_DOTA_Ability_Aghsfort_Elemental_Wisp_Tether:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Kunkka_2:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Crystal_Maiden_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Furion_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Axe_6:
            pass
        class C_DOTA_Ability_Special_Bonus_Gold_Income_180:
            pass
        class CDOTA_Modifier_Marci_Unleash_Flurry:
            pass
        class CDOTA_Modifier_VoidSpirit_IntrinsicEdge:
            pass
        class CDOTA_Modifier_NagaSiren_SongOfTheSiren_Healing_Aura:
            pass
        class CDOTA_Modifier_Rubick_FadeBoltDebuff:
            pass
        class CDOTA_Modifier_Meepo_Poof_Casting:
            pass
        class CDOTA_Modifier_Weaver_Swarm:
            pass
        class CDOTA_Modifier_Windrunner_Arcana_Kill_Effect:
            pass
        class CDOTA_Modifier_Kunkka_GhostShip_DamageAbsorb:
            pass
        class CDOTA_Modifier_StormSpirit_Enemy_Overload:
            pass
        class CDOTA_Modifier_Lua_Vertical_Motion:
            pass
        class CDOTA_Modifier_PreventTaunts:
            pass
        class C_BodyComponentBaseAnimating:
            pass
        class CAttributeManager:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class SignatureOutflow_Continue:
            pass
        class C_DOTA_NPC_Techies_Minefield_Sign:
            pass
        class C_DOTA_Unit_Hero_Luna:
            pass
        class C_DOTA_NPC_WitchDoctor_Ward:
            pass
        class C_Artillery_PortraitWorldUnit:
            pass
        class C_DOTA_Tiny_ScepterTree:
            pass
        class C_DOTA_Item_Physical:
            pass
        class C_DOTA_Item_Eaglehorn:
            pass
        class C_DOTA_Item_BootsOfElven:
            pass
        class C_DOTA_Ability_Muerta_OfrendaDestroy:
            pass
        class CDOTA_Ability_Snapfire_SpitCreep:
            pass
        class C_DOTA_Ability_Techies_Squees_Scope:
            pass
        class CDOTA_Ability_Centaur_Mount:
            pass
        class C_DOTA_Ability_Meepo_Innate_ToughChoices:
            pass
        class C_DOTA_Ability_Tidehunter_KrakenShell:
            pass
        class C_DOTA_Ability_Crystal_Maiden_GlacialGuard:
            pass
        class CDOTA_Modifier_BigThunderLizard_Slam:
            pass
        class CInfoTarget:
            pass
        class C_DOTA_Ability_Special_Bonus_Cleave_140:
            pass
        class CDOTA_Modifier_EldwurmsEdda:
            pass
        class CDOTA_Modifer_Item_DandelionAmulet:
            pass
        class CDOTA_Modifier_Item_GhostScepter:
            pass
        class CDOTA_Modifier_Item_TranquilBoots2:
            pass
        class CDOTA_Modifier_Muerta_Gunslinger_Increased_Proc_Chance:
            pass
        class CDOTA_Modifier_Pangolier_Swashbuckle_Damage_Penalty:
            pass
        class CDOTA_Modifier_Oracle_DivinersDeck_Death:
            pass
        class CDOTA_Modifier_Brewmaster_BrewUp:
            pass
        class CDOTA_Modifier_Batrider_Flamebreak_Damage:
            pass
        class CDOTA_Modifier_ShadowShaman_Voodoo:
            pass
        class CDOTA_Modifier_CrystalMaiden_FreezingField_Tracker:
            pass
        class CDOTA_Modifier_NPXBuff:
            pass
        class CPlayer_CameraServices:
            pass
        class CPulseCell_Timeline:
            pass
        class CPulseCell_Inflow_EntOutputHandler:
            pass
        class CDOTA_Unit_Hero_Elder_Titan:
            pass
        class C_DOTA_Unit_Hero_Obsidian_Destroyer:
            pass
        class C_DOTA_Item_Ceremonial_Robe:
            pass
        class C_DOTA_Ability_Ringmaster_Impalement:
            pass
        class C_DOTA_Ability_Muerta_Supernatural:
            pass
        class CDOTA_Ability_Mars_GodsRebuke:
            pass
        class C_DOTA_Ability_Elder_Titan_NaturalOrder:
            pass
        class C_DOTA_Ability_Alchemist_Corrosive_Weaponry:
            pass
        class C_DOTA_Ability_Clinkz_Empty1:
            pass
        class C_DOTA_Ability_Animation_RightClawSwipe:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Lina_6:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Antimage_4:
            pass
        class CDOTA_Modifier_Special_Bonus_Vision:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_50:
            pass
        class C_DOTA_Ability_Special_Bonus_Gold_lvl15_r:
            pass
        class C_DOTA_TrackingProjectileInfoParticleAPI:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Modifier_Roshans_Banner_Aura:
            pass
        class CDOTA_Modifier_Mars_ArenaOfBlood_Kill_Buff:
            pass
        class CDOTA_Modifier_Winter_Wyvern_Arctic_Burn_Slow:
            pass
        class CDOTA_Modifier_Meepo_Petrify:
            pass
        class CDOTA_Modifier_Lina_SuperCharged:
            pass
        class CDOTA_Modifier_Razor_Arcana_EmpoweredState:
            pass
        class CDOTA_Modifier_Juggernaut_BladeFury:
            pass
        class CDOTA_Modifier_ScriptedMotionController:
            pass
        class CHeroesPerPlayer:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CScenePayloadVData:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class C_DOTA_Unit_Hero_Magnataur:
            pass
        class C_DOTA_Ability_Pangolier_HeartPiercer:
            pass
        class CDOTA_Ability_Broodmother_SpinWeb:
            pass
        class C_DOTA_Ability_Lion_FingerOfDeath:
            pass
        class C_DOTA_Ability_Nevermore_Shadowraze2:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Keeper_of_the_Light_7:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_900:
            pass
        class PlayerResourcePlayerPeriodicResourceData_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Modifier_Tome_of_Aghanim:
            pass
        class CDOTA_Modifier_Ringmaster_TameTheBeasts:
            pass
        class CDOTA_Modifier_Dawnbreaker_Fire_Wreath_Magic_Immunity_Tooltip:
            pass
        class CDOTA_Modifier_Techies_StickyBomb_Slow:
            pass
        class CDOTA_Modifier_Slark_Armor_Reduction_Debuff:
            pass
        class CDOTA_Modifier_Meepo_Flung:
            pass
        class CDOTA_Modifier_NightStalker_Void_Zone:
            pass
        class CDOTA_Modifier_Luna_LucentBeam_Damage_Buff:
            pass
        class CDOTA_Modifier_DeathProphet_Scepter:
            pass
        class CDOTA_Modifier_Courier_TakeStashItems:
            pass
        class CDOTA_Modifier_Sven_Warcry_Passive:
            pass
        class CFilterAttributeInt:
            pass
        class C_DOTA_Item_DaggerOfRistul:
            pass
        class C_DOTA_Item_Fusion_rune:
            pass
        class C_DOTA_Ability_Invoker_SunStrike:
            pass
        class C_DOTA_Ability_PhantomLancer_PhantomEdge:
            pass
        class CDOTA_Ability_Axe_BattleHunger:
            pass
        class CDOTA_Ability_VengefulSpirit_Revenge:
            pass
        class C_DOTA_Ability_Sven_GreatCleave:
            pass
        class CPointTemplate:
            pass
        class C_DOTA_Ability_PudgeMiniboss_ArmorCorruption:
            pass
        class CDOTA_Modifier_Seasonal_Summon_Common_Thinker:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Weaver_1:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Slardar_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Range_400:
            pass
        class C_DOTA_Ability_Special_Bonus_Attributes:
            pass
        class CDOTA_Modifier_Ringmaster_UnicycleMovement:
            pass
        class CDOTA_Modifier_Phoenix_FireSpiritBurn:
            pass
        class CDOTA_Modifier_Abyssal_Underling_Warrior_ManaBurn:
            pass
        class CDOTA_Modifier_Spirit_Bear_Attack_Damage:
            pass
        class CDOTA_Modifier_Clinkz_WindWalk:
            pass
        class CDOTA_Modifier_Lion_Innate_ToHellAndBack_Buff:
            pass
        class CDOTA_Modifier_Sniper_Shrapnel_Thinker:
            pass
        class CDOTA_Modifier_Kunkka_GhostShip_DamageDelay:
            pass
        class CDOTA_Modifier_Bloodseeker_BloodMist_Slow:
            pass
        class CDOTA_Modifier_Stunned:
            pass
        class CPlayer_FlashlightServices:
            pass
        class C_DOTA_Unit_Hero_Beastmaster_Hawk:
            pass
        class C_DOTA_Item_GaleGuard:
            pass
        class C_DOTA_Ability_Largo_AmphibianRhapsody:
            pass
        class C_DOTA_Ability_Kez_Shodo_Sai:
            pass
        class C_DOTA_Ability_Invoker_Tornado_AD:
            pass
        class CDOTA_Ability_DarkSeer_Vacuum:
            pass
        class C_DOTA_Ability_Luna_Eclipse:
            pass
        class C_DOTA_Ability_WitchDoctor_DeathWard:
            pass
        class CDOTA_Modifier_Greevil_Miniboss_Blue_IceVortexThinker:
            pass
        class CDOTA_Modifier_Mutation_CreateTombstone:
            pass
        class CDOTA_Modifier_AghsFort_Waveblaster_Leap:
            pass
        class CDOTA_Ability_Creature_IceSlam:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Night_Stalker:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_175:
            pass
        class PlayerResourcePlayerData_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Modifier_Item_Nether_Shawl:
            pass
        class CDOTA_Modifier_Item_WraithBand:
            pass
        class CDOTA_Modifier_Hoodwink_Bushwhack_Trap:
            pass
        class CDOTA_Modifier_Magnataur_Horn_Toss_Slow:
            pass
        class CDOTA_Modifier_Gyrocopter_Side_Gunner:
            pass
        class CDOTA_Modifier_Life_Stealer_Infest_Effect:
            pass
        class CDOTA_Modifier_FacelessVoid_ArcanaDamageRouter:
            pass
        class CDOTA_Modifier_Razor_Arcana_Kill_Effect:
            pass
        class CDOTA_Unit_Miniboss_Minion:
            pass
        class CDOTA_Item_MetamorphicMandible:
            pass
        class C_DOTA_Item_DandelionAmulet:
            pass
        class C_DOTA_Item_MeteorHammer:
            pass
        class C_DOTA_Item_Hood_Of_Defiance:
            pass
        class C_DOTA_Ability_Hoodwink_HeavyQuiver:
            pass
        class C_DOTA_Ability_Bristleback_Prickly:
            pass
        class CDOTA_Ability_Night_Stalker_Night_Reign:
            pass
        class C_DOTA_Ability_Sand_King_Scorpion_Strike:
            pass
        class C_DOTA_Ability_Juggernaut_Vaulted_Strike:
            pass
        class C_DOTA_Ability_Juggernaut_BladeFury:
            pass
        class CDOTA_Modifier_AghsFort_TrapRoom_MeatHook_PathingFix:
            pass
        class CDOTA_Modifier_AghsFort_Spectre_ActiveDispersion:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Wisp_4:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Clockwerk_4:
            pass
        class CDOTA_Modifier_Item_Witch_Blade:
            pass
        class CDOTA_Modifier_Item_Pavise:
            pass
        class CDOTA_Modifier_MaskOfMadness_Berserk:
            pass
        class CDOTA_Modifier_MonkeyKing_FurArmy_SoldierInactive:
            pass
        class CDOTA_Modifier_AbyssalUnderlord_AtrophyAura_Effect:
            pass
        class CDOTA_Modifier_Nyx_Assassin_Vendetta_Armor_Reduction:
            pass
        class CDOTA_Modifier_Meepo_Divided_We_Stand:
            pass
        class CDOTA_Modifier_SpiritBreaker_GreaterBash_Knockback:
            pass
        class CDOTA_Modifier_Jakiro_Macropyre_Ice_Edge_Slow:
            pass
        class CDOTA_Modifier_Roshan_Bash:
            pass
        class CDOTA_Modifier_Razor_StaticLink:
            pass
        class CDOTA_Modifier_SandKing_Epicenter:
            pass
        class CDOTA_Modifier_HP_Regen:
            pass
        class CPulseCell_Outflow_CycleOrdered__InstanceState_t:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class C_DOTA_Item_Recipe_Angels_Demise:
            pass
        class C_DOTA_Item_Recipe_Cornucopia:
            pass
        class C_DOTA_Item_Spirit_Vessel:
            pass
        class C_DOTA_Item_Aegis:
            pass
        class C_DOTA_Item_Clarity:
            pass
        class CDOTA_Ability_Dark_Willow_Pixie_Dust:
            pass
        class C_DOTA_Ability_DarkWillow_CursedCrown:
            pass
        class C_DOTA_Ability_Phoenix_Supernova:
            pass
        class CDOTA_Ability_Treant_LifeBomb_Explode:
            pass
        class CDOTA_Ability_Lycan_SummonWolves_GeistForm:
            pass
        class C_DOTA_Ability_Shadow_Shaman_Fowl_Play:
            pass
        class CDOTA_Ability_Frogmen_WaterBubble_Large:
            pass
        class CDOTA_Modifier_Greevil_Miniboss_White_Degen_Aura:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Rubick_5:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Huskar_3:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lifestealer_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_8:
            pass
        class C_DOTA_Ability_Special_Bonus_All_Stats_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Speed_70:
            pass
        class CDOTA_Modifier_Item_SuperSwift_Blink:
            pass
        class CDOTA_Modifier_Item_Voodoo_Mask:
            pass
        class CDOTA_Modifier_Item_CrellasCrozier_Movespeed_Debuff:
            pass
        class CDOTA_Modifier_Hoodwink_Boomerang_Invulnerable:
            pass
        class CDOTA_Modifier_Snapfire_GobbleUp_Creep:
            pass
        class CDOTA_Modifier_Visage_GraveChill_Debuff:
            pass
        class CDOTA_Modifier_Undying_Decay_BuffCounter:
            pass
        class CDOTA_Modifier_Undying_Decay_Debuff:
            pass
        class CDOTA_Modifier_Shadow_Demon_Shadow_Poison:
            pass
        class CDOTA_Modifier_Silencer_BlabberMouth:
            pass
        class CDOTA_Modifier_Chen_Penitence:
            pass
        class CDOTA_Modifier_Batrider_StickyNapalm_Application:
            pass
        class CDOTA_Modifier_PhantomAssassin_PhantomStrike:
            pass
        class CDOTA_Modifier_FacelessVoid_Timelock_Basic_Proc_Marker:
            pass
        class CDOTA_Modifier_Beastmaster_Rugged:
            pass
        class CDOTA_Modifier_Shadowraze_Debuff:
            pass
        class CDOTA_Ability_AntiMage_BlinkParticleAPI:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Modifier_Item_Imbue_Choice:
            pass
        class C_DOTA_Unit_Hero_Tusk:
            pass
        class C_DOTA_BaseNPC:
            pass
        class CDOTA_Item_Recipe_Diffusal_Blade2:
            pass
        class C_DOTA_Item_Sphere:
            pass
        class C_DOTA_Item_Dagon_Upgraded4:
            pass
        class CDOTA_Ability_AbyssalUnderlord_PitOfMalice:
            pass
        class CDOTA_Ability_Abaddon_AphoticShield:
            pass
        class C_DOTA_Ability_Viper_Predator:
            pass
        class C_DOTA_Ability_Nian_Whirlpool:
            pass
        class C_DOTA_Ability_Riki_Poison_Dart:
            pass
        class C_DOTA_Ability_Frogmen_TendrilsOfTheDeep:
            pass
        class CDOTA_Modifier_OgreMagi_FrostArmor_Slow:
            pass
        class C_SoundEventAABBEntity:
            pass
        class C_DOTA_Ability_AghsFort_Creature_Phoenix_FireSpirits:
            pass
        class C_DOTA_Ability_Special_Bonus_Cast_Range_100:
            pass
        class PlayerResourcePlayerTeamData_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class PlayerResourcePlayerEventData_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Modifier_Item_Misericorde:
            pass
        class CDOTA_Modifier_Item_Clumsy_Net:
            pass
        class CDOTA_Modifier_Hoodwink_Decoy_Illusion:
            pass
        class CDOTA_Modifier_Mars_ArenaOfBlood_AnimationAura:
            pass
        class CDOTA_Modifier_DarkWillow_ShadowRealm_Aura_Debuff:
            pass
        class CDOTA_Modifier_AbyssalUnderlord_AtrophyAura_DmgBuffCounter:
            pass
        class CDOTA_Modifier_AbyssalUnderlord_PitOfMalice_Buff_Placer:
            pass
        class CDOTA_Modifier_Rubick_NullField_Effect:
            pass
        class CDOTA_Modifier_Brewmaster_StormBrewling_StormStance:
            pass
        class CDOTA_Modifier_Alchemist_AcidSpray:
            pass
        class CDOTA_Modifier_Clinkz_Infernal_Shred:
            pass
        class CDOTA_Modifier_Rattletrap_Cog_Pinball:
            pass
        class CDOTA_Modifier_Dazzle_Innate_Weave_Armor_Counter:
            pass
        class CDOTA_Modifier_TemplarAssassin_Meditation:
            pass
        class CDOTA_Modifier_Shadow_Shaman_Serpent_Ward_Shackle_Tracker:
            pass
        class CDOTA_Modifier_Holdout_StaticRemnantThinker:
            pass
        class CDOTA_Modifier_Mirana_Leap_Root:
            pass
        class CDOTA_Modifier_Rune_ExtraDamage:
            pass
        class CDOTA_Modifier_RootedUndispellable:
            pass
        class CDOTA_Modifier_Invulnerable:
            pass
        class C_TintController:
            pass
        class C_DOTA_Item_Spy_Gadget:
            pass
        class C_DOTA_Item_Dagon_Upgraded:
            pass
        class C_DOTA_Item_RingOfRegeneration:
            pass
        class C_DOTA_Item_GlovesOfHaste:
            pass
        class C_DOTA_Ability_EmberSpirit_Immolation:
            pass
        class C_DOTA_Ability_Skywrath_Mage_Staff_Of_The_Scion:
            pass
        class CDOTA_Ability_Broodmother_PoisonSting:
            pass
        class C_DOTA_Ability_Mirana_Starfall:
            pass
        class CDOTA_Ability_Lamp_Use:
            pass
        class CDOTA_Modifier_AghsFort_SwampSickness:
            pass
        class CDOTA_Modifier_Healing_Campfire_Aura:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Tiny_7:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Base_Damage_15:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_300:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_100:
            pass
        class CDOTA_Modifier_Item_Mysterious_Hat:
            pass
        class CDOTA_Modifier_Dawnbreaker_Solar_Guardian_After_Land:
            pass
        class CDOTA_Modifier_Pangolier_Fortune_Favors_The_Bold:
            pass
        class CDOTA_Modifier_Techies_Minefield_Sign_Scepter:
            pass
        class CDOTA_Modifier_Abaddon_TheQuickening:
            pass
        class CDOTA_Modifier_Rattletrap_BatteryAssault:
            pass
        class CDOTA_Modifier_Luna_LunarBlessing:
            pass
        class CDOTA_Modifier_Slardar_Amplify_Damage_SelfBuff:
            pass
        class CDOTA_Modifier_Bloodseeker_BloodMist_Passive:
            pass
        class CDOTA_NPC_Observer_Ward_TrueSight:
            pass
        class C_DOTA_Unit_Hero_Silencer:
            pass
        class C_DOTA_BaseNPC_HoldoutTower_ReduceSpeed:
            pass
        class C_DOTA_Item_Giants_Ring:
            pass
        class C_DOTA_Item_Bloodstone:
            pass
        class C_DOTA_Item_Assault_Cuirass:
            pass
        class C_DOTA_Item_Hyperstone:
            pass
        class CDOTA_Ability_GraniteGolem_Bash:
            pass
        class C_DOTA_Ability_Neutral_SpellImmunity:
            pass
        class CDOTA_Modifier_Mutation_CritChance_Team_Aura:
            pass
        class C_FuncBrush:
            pass
        class C_DOTA_Ability_Special_Bonus_Strength_40:
            pass
        class C_DOTA_Ability_Special_Bonus_Movement_Speed_30:
            pass
        class CDOTA_Modifier_Jidi_Pollen_Bag:
            pass
        class CDOTA_Modifier_Seer_Stone:
            pass
        class CDOTA_Modifier_Hoodwink_Tomokan_Tracker_Aura:
            pass
        class CDOTA_Modifier_Oracle_RainOfDestiny:
            pass
        class CDOTA_Modifier_Oracle_PurifyingFlames:
            pass
        class CDOTA_Modifier_KeeperOfTheLight_SpiritForm:
            pass
        class CDOTA_Modifier_NightStalker_Void:
            pass
        class CDOTA_Modifier_Rattletrap_Cogs_Movement:
            pass
        class CDOTA_Modifier_FacelessVoid_TimeZone:
            pass
        class CDOTA_Modifier_Tiny_CraggyExterior:
            pass
        class CDOTA_Modifier_BackdoorProtectionActive:
            pass
        class CIngameEvent_SeasonFall2025:
            pass
        class C_DOTA_Ability_Muerta_DeadShot:
            pass
        class CDOTA_Ability_Slark_DarkPact:
            pass
        class C_DOTA_Ability_Spectre_Haunt:
            pass
        class CDOTA_Modifier_Aghsfort_Reward_CritAura:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Rubick_6:
            pass
        class CDOTA_Modifier_Special_Bonus_Night_Vision:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_40:
            pass
        class C_DOTA_Ability_Special_Bonus_Agility_7:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_Regen_6:
            pass
        class CDOTA_Modifier_IdolOfScreeauk:
            pass
        class CDOTA_Modifier_Item_Terror_Mask_Fear:
            pass
        class CDOTA_Modifier_Jacket_Blind:
            pass
        class CDOTA_Modifier_Marci_Special_Delivery_Teleport_Effect:
            pass
        class CDOTA_Modifier_Mars_Scepter_Damage_Tracker:
            pass
        class CDOTA_Modifier_ArcWarden_Flux_Alone:
            pass
        class CDOTA_Modifier_Undying_Decay_Buff_Shared:
            pass
        class CDOTA_Modifier_Spectre_Dispersion_Memory:
            pass
        class CDOTA_Modifier_Leshrac_Decrepify:
            pass
        class CDOTA_Modifier_SandKing_SandStorm:
            pass
        class C_DOTATurboGameMode:
            pass
        class C_DOTA_Item_Ancient_Janggo:
            pass
        class C_DOTA_Item_Recipe_WraithBand:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Disruptor_9:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Amplify_9:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_Regen_12:
            pass
        class CDOTA_Modifier_ForgargersHealth_Buff:
            pass
        class CDOTA_Modifier_Item_Essence_Distiller_Thinker_VFX:
            pass
        class CDOTA_Modifier_BootsOfTravel_Incoming:
            pass
        class CDOTA_Modifier_Snapfire_FiresnapCookie_LandingGesture:
            pass
        class CDOTA_Modifier_Terrorblade_Metamorphosis:
            pass
        class CDOTA_Modifier_TrollWarlord_WhirlingAxes_Blind:
            pass
        class CDOTA_Modifier_Ogre_Magi_Smash_Buff:
            pass
        class CDOTA_Modifier_Lycan_Shapeshift:
            pass
        class CDOTA_Modifier_Batrider_StickyNapalm:
            pass
        class CDOTA_Modifier_Omniknight_Angelic_Flight:
            pass
        class CDOTA_Modifier_DeathProphet_Exorcism:
            pass
        class CDOTA_Modifier_Beastmaster_Hawk_Dive:
            pass
        class C_EconEntity__AttachedParticleInfo_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class PhysicsRagdollPose_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CPropDataComponent:
            pass
        class C_DOTA_Unit_Hero_Ursa:
            pass
        class CDOTA_Item_Guardian_Shell:
            pass
        class CDOTA_Item_Trident:
            pass
        class C_DOTA_Item_Philosophers_Stone:
            pass
        class C_DOTA_Item_CrellasCrozier:
            pass
        class C_DOTA_Ability_Legion_Commander_Outfight_Them:
            pass
        class C_DOTA_Ability_Courier_TransferItems_ToOtherPlayer:
            pass
        class C_DOTA_Ability_Enigma_Splitting_Image:
            pass
        class C_DOTA_Ability_Razor_Dynamo:
            pass
        class C_DOTA_Ability_BackdoorProtection:
            pass
        class CDOTA_Modifier_Passive_Lotus_Pool:
            pass
        class CDOTA_Modifier_Seasonal_Summon_TI11_Balloon_Thinker:
            pass
        class C_DOTA_Ability_AghsFort_TrapRoom_Hookshot:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Centaur_2:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Axe_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Ember_Spirit_6:
            pass
        class C_DOTA_LinearProjectileInfoParticleAPI:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Modifier_Brewmaster_PermanentImmolation:
            pass
        class CDOTA_Modifier_Broodmother_StickySnare:
            pass
        class CDOTA_Modifier_Phantom_Assassin_Coup_Counter:
            pass
        class CDOTA_Modifier_Nian_Big_Flinch:
            pass
        class CDOTA_Modifier_Roshan_InherentBuffs:
            pass
        class CDOTA_Modifier_Bloodseeker_Thirst_Speed:
            pass
        class CDOTA_Modifier_CrystalMaiden_Let_It_Go_Thinker:
            pass
        class CDOTA_AbilityDraftAbilityState:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CPulseCell_LimitCount__InstanceState_t:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # X��@�
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class C_DOTA_BaseNPC_RotatableBuilding:
            pass
        class C_DOTA_Item_Recipe_Trickster_Cloak:
            pass
        class C_DOTA_Item_Recipe_Witch_Blade:
            pass
        class CDOTA_Ability_Shadow_Demon_Soul_Catcher:
            pass
        class C_DOTA_Ability_Rattletrap_RocketFlare:
            pass
        class C_DOTA_Ability_Holdout_ScourgeWard:
            pass
        class CDOTA_Ability_Bloodseeker_Sanguivore:
            pass
        class C_DOTA_Ability_VengefulSpirit_Nether_Swap:
            pass
        class CDOTA_Modifier_HarpyStorm_ChainLightning:
            pass
        class CDOTA_Item_BookIntelligence:
            pass
        class CDOTA_Ability_AghsFort_Creature_SpikedCarapace:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lifestealer_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Ancient_Apparition_1:
            pass
        class C_DOTA_Ability_Special_Bonus_Cast_Range_225:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Range_50:
            pass
        class C_DOTA_Ability_Special_Bonus_Intelligence_35:
            pass
        class C_DOTA_Ability_Special_Bonus_All_Stats_12:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Speed_200:
            pass
        class CDOTA_Modifier_Item_Naginata:
            pass
        class CDOTA_Modifier_Item_Trident:
            pass
        class CDOTA_Modifier_Item_ChainMail:
            pass
        class CDOTA_Modifier_CallOfTheWild_Boar_PoisonEffect:
            pass
        class CDOTA_Modifier_Beastmaster_Axe_Stack_Counter:
            pass
        class CDOTA_Modifier_Tidehunter_KrakenShell_Boost:
            pass
        class CDOTA_Modifier_Juggernaut_BladeDance:
            pass
        class C_DOTA_Item_Recipe_RingOfAquila:
            pass
        class C_DOTA_Ability_KeeperOfTheLight_Will_O_Wisp:
            pass
        class C_DOTA_Ability_PhantomAssassin_Blur:
            pass
        class C_DOTA_Ability_AntiMage_Blink:
            pass
        class C_DOTA_Ability_Greevil_Miniboss_Green_Overgrowth:
            pass
        class CDOTA_Modifier_Fountain_Glyph:
            pass
        class C_DynamicLight:
            pass
        class C_DOTA_Ability_Special_Bonus_30_Crit_2:
            pass
        class C_DOTA_Ability_Special_Bonus_Reincarnation_300:
            pass
        class TrackedStatNetworkData_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Modifier_Item_Enhancement_Vast:
            pass
        class CDOTA_Modifier_Snapfire_FiresnapCookie_PreHop:
            pass
        class CDOTA_Modifier_Phoenix_Sun_Ray_Blind:
            pass
        class CDOTA_Modifier_Nyx_Assassin_Jolt_Debuff:
            pass
        class CDOTA_Modifier_ChaosKnight_Chaos_Strike:
            pass
        class CDOTA_Modifier_Alchemist_ChemicalRageTransform:
            pass
        class CDOTA_Modifier_Rattletrap_Hookshot_Barrier:
            pass
        class CDOTA_Modifier_Roshan_RevengeRoar_Windup:
            pass
        class CDOTA_Modifier_Ursa_Innate_Maul:
            pass
        class CDOTA_Modifier_CrystalMaiden_FrozenExpanse:
            pass
        class CDOTA_Modifier_DebuffImmune:
            pass
        class EngineCountdownTimer:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Item_WindLace:
            pass
        class C_DOTA_Item_Pipe:
            pass
        class C_DOTA_Item_ChainMail:
            pass
        class C_DOTA_Ability_Dawnbreaker_Luminosity:
            pass
        class C_DOTA_Ability_Elder_Titan_NaturalOrder_Spirit:
            pass
        class CDOTA_Ability_Medusa_GorgonGrasp:
            pass
        class C_CDOTA_Ability_Treant_LeechSeed:
            pass
        class CDOTA_Ability_Invoker_InvokedBase:
            pass
        class C_DOTA_Ability_Jakiro_IcePath_Detonate:
            pass
        class C_DOTA_Ability_Slardar_Slithereen_Crush:
            pass
        class C_DOTA_Ability_Sniper_Concussive_Grenade:
            pass
        class C_DOTA_Ability_AncientApparition_BoneChill:
            pass
        class C_SoundEventSphereEntity:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Rubick_8:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Enigma:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_800:
            pass
        class CDOTA_Modifier_Item_Enhancement_Titanic:
            pass
        class CDOTA_Modifier_Item_ConsecratedWraps_Effect:
            pass
        class CDOTA_Modifier_Orb_Of_Destruction_Debuff:
            pass
        class CDOTA_Modifier_Item_Moonshard:
            pass
        class CDOTA_Modifier_ArcWarden_MagneticField_Attack_Speed:
            pass
        class CDOTA_Modifier_Oracle_DivinersDeck_TheWorld_Next:
            pass
        class CDOTA_Modifier_Visage_Lurker_Linger:
            pass
        class CDOTA_Modifier_Weaver_Swarm_Debuff:
            pass
        class CDOTA_Modifier_Clinkz_Strafe:
            pass
        class CDOTA_Modifier_Rattletrap_Cog_Self_Bonuses:
            pass
        class CDOTA_Modifier_Puck_Coiled:
            pass
        class CDOTA_Modifier_Kunkka_Ghost_Ship_Fleet:
            pass
        class CDOTA_Modifier_Jugg_Caster:
            pass
        class CDOTA_Modifier_MoveSpeed_Percentage:
            pass
        class CIngameEvent_TI2020:
            pass
        class C_DOTA_BaseNPC_Promo:
            pass
        class C_DOTA_Item_Angels_Demise:
            pass
        class CDOTA_Item_Faded_Broach:
            pass
        class CDOTA_Ability_Snapfire_Scatterblast:
            pass
        class C_DOTA_Ability_Keeper_of_the_Light_SpecialReserve:
            pass
        class C_DOTA_Ability_Batrider_Firefly:
            pass
        class C_DOTA_Ability_Venomancer_VenomousGale:
            pass
        class C_DOTA_Ability_Slardar_Sprint:
            pass
        class C_DOTA_Ability_ForestTrollHighPriest_ManaAura:
            pass
        class CDOTA_Ability_AghsFort_Ascension_MagneticField:
            pass
        class CDOTA_Modifier_Seasonal_Summon_TI9_Balloon_Thinker:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Luna_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Armor_6:
            pass
        class CDOTA_Modifier_Item_Revenants_Brooch:
            pass
        class CDOTA_Modifier_Item_Dagon:
            pass
        class CDOTA_Modifier_Primal_Beast_Colossal:
            pass
        class CDOTA_Modifier_PrimalBeast_Uproar_Slow:
            pass
        class CDOTA_Modifier_Grimstroke_DarkArtistry_Slow:
            pass
        class CDOTA_Modifier_Grimstroke_InkCreature_Debuff:
            pass
        class CDOTA_Modifier_Magnetic_Horn_Items:
            pass
        class CDOTA_Modifier_DoomBringer_Doom:
            pass
        class CDOTA_Modifier_Batrider_Displacement_Buff:
            pass
        class CDOTA_Modifier_DeathProphet_Silence:
            pass
        class CDOTA_Modifier_WitchDoctor_Voodoo_Switcheroo:
            pass
        class CDOTA_Modifier_Windrunner_GaleForce:
            pass
        class CDOTA_Modifier_DrowRanger_FrostArrows_Shard_Slow:
            pass
        class CDOTA_Modifier_Pudge_Dismember_Pull:
            pass
        class CDOTA_Modifier_Cyclone:
            pass
        class C_DOTAGamerules:
            pass
        class CDOTA_BaseNPC_Phantom_Assassin_GroundDagger:
            pass
        class C_DOTA_Item_Tier2Token:
            pass
        class C_TonemapController2Alias_env_tonemap_controller2:
            pass
        class C_DOTA_Item_Jidi_Pollen_Bag:
            pass
        class C_DOTA_Ability_Winter_Wyvern_Eldwurm_Scholar:
            pass
        class C_DOTA_Ability_Tusk_IceShards:
            pass
        class CDOTA_Ability_Centaur_Horsepower:
            pass
        class C_DOTA_Ability_DeathProphet_SpiritSiphon:
            pass
        class C_DOTA_Ability_Windrunner_FocusFire:
            pass
        class C_DOTA_Ability_Zuus_Cloud:
            pass
        class C_DOTA_Item_BotChallenge_MeteorStaff:
            pass
        class CDOTA_Ability_MudGolem_RockDestroy:
            pass
        class C_DOTA_Ability_WarpineRaider_SeedShot:
            pass
        class CFilterLOS:
            pass
        class CPointOrient:
            pass
        class CDOTA_Modifier_Seasonal_TI11_DuelAccepted:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Antimage_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_10:
            pass
        class CDeferredLightBase:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_GlobalLight:
            pass
        class C_EnvWindClientside:
            pass
        class CDOTA_Modifier_HeavensHalberd_Debuff:
            pass
        class CDOTA_Modifier_Item_MonkeyKingBar:
            pass
        class CDOTA_Modifier_Item_MagicWand:
            pass
        class CDOTA_Modifier_Item_PowerTreads:
            pass
        class CDOTA_Modifier_Dawnbreaker_Converge_Thinker:
            pass
        class CDOTA_Modifier_Elder_Titan_NaturalOrder_Aura_MagicResistance:
            pass
        class CDOTA_Modifier_Nian_EruptionThinker:
            pass
        class CDOTA_Modifier_Riki_TricksOfTheTrade_Move:
            pass
        class CDOTA_Modifier_Zuus_Cloud:
            pass
        class CDOTA_Modifier_PhantomLancer_PhantomEdge_Agility:
            pass
        class sky3dparams_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_FightingGame_EffectsEntity:
            pass
        class C_DOTA_Item_Enhancement_Vampiric:
            pass
        class C_DOTA_Item_Swift_Blink:
            pass
        class C_DOTA_Item_Craggy_Coat:
            pass
        class C_DOTA_Item_Recipe_Heart:
            pass
        class C_DOTA_Ability_Abaddon_Withering_Mist:
            pass
        class C_DOTA_Ability_Shredder_ChakramAlias_shredder_chakram_2:
            pass
        class C_DOTA_Ability_Visage_Stone_Form_Self_Cast:
            pass
        class CDOTA_Ability_NagaSiren_RipTide:
            pass
        class C_DOTA_Ability_Meepo_MegaMeepo:
            pass
        class C_DOTA_Ability_Pugna_Oblivion_Savant:
            pass
        class C_DOTA_Ability_Venomancer_Latent_Poison:
            pass
        class C_DOTA_Ability_Animation_TailSpin:
            pass
        class C_DOTA_Ability_Razor_EyeOfTheStorm:
            pass
        class C_DOTA_Ability_Greevil_Miniboss_Blue_IceVortex:
            pass
        class C_SoundEventConeEntity:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Doom_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Evasion_25:
            pass
        class CDOTA_Modifier_Item_Black_Grimoire:
            pass
        class CDOTA_Modifier_Item_Bullwhip_Buff_Thinker:
            pass
        class CDOTA_Modifier_Item_CraniumBasher:
            pass
        class CDOTA_Modifier_Muerta_PierceTheVeilTransform:
            pass
        class CDOTA_Modifier_Techies_ReactiveTazer:
            pass
        class CDOTA_Modifier_Disruptor_KineticWall_Thinker:
            pass
        class CDOTA_Modifier_Rubick_Arcane_Supremacy:
            pass
        class CDOTA_Modifier_Treant_LeechSeed_Slow:
            pass
        class CDOTA_Modifier_LoneDruid_Entangle_Counter:
            pass
        class CDOTA_Modifier_Brewmaster_Fire_Phase:
            pass
        class CDOTA_Modifier_Jakiro_IcePath_Thinker:
            pass
        class CDOTA_Modifier_Leshrac_Defilement:
            pass
        class CDOTA_Modifier_TemplarAssassin_InnerPeace_Passive:
            pass
        class CDOTA_Modifier_Miniboss_Team_Unreliable_Gold_Tracker_Thinker:
            pass
        class CDOTA_Modifier_Kunkka_Torrent_Storm:
            pass
        class CDOTA_Modifier_Lina_LagunaBlade_Barrier:
            pass
        class CDOTA_Modifier_StormSpirit_Electric_Rave:
            pass
        class CDOTA_Modifier_Mirana_Leap_Buff:
            pass
        class CDOTA_Modifier_SandKing_Impale:
            pass
        class CDestructiblePartsComponent:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class C_DOTA_NPC_TechiesMines:
            pass
        class C_DOTA_Unit_Hero_Beastmaster_Boar:
            pass
        class C_DOTA_PortraitBaseModel:
            pass
        class C_EnvDeferredSpotLightClientOnly:
            pass
        class C_DOTA_Item_Tree_Processor:
            pass
        class C_DOTA_Item_Recipe_SheepStick:
            pass
        class C_DOTA_Item_PoorMansShield:
            pass
        class C_DOTA_Ability_Hoodwink_AcornShot:
            pass
        class C_DOTA_Ability_EmberSpirit_FlameGuard:
            pass
        class C_DOTA_Ability_Alchemist_GoblinsGreed:
            pass
        class C_DOTA_Ability_BountyHunter_WindWalk:
            pass
        class C_DOTA_Ability_Clinkz_Strafe:
            pass
        class CDOTA_Ability_Life_Stealer_AssimilateEject:
            pass
        class CDOTA_Ability_Miniboss_Reflect:
            pass
        class C_DOTA_Ability_Greevil_Miniboss_Yellow_IonShell:
            pass
        class C_DOTA_Ability_Greevil_Miniboss_Red_Overpower:
            pass
        class C_EnvWind:
            pass
        class CDOTA_Modifier_Seasonal_TI11_RockPaperScissors_Playing:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Hoodwink_BushwhackCooldown:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Ursa_3:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Morphling_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_15:
            pass
        class C_DOTA_Ability_Special_Bonus_Movement_Speed_Percentage_16:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_Regen_80:
            pass
        class C_DOTA_Ability_AttributeBonus:
            pass
        class CDOTA_Modifier_MaskOfDispair_Berserk:
            pass
        class CDOTA_Modifier_Item_Spider_Legs_Active:
            pass
        class CDOTA_Modifier_Marci_Grapple_VictimMotion:
            pass
        class CDOTA_Modifier_Bristleback_QuillSpray_AutoCast:
            pass
        class CDOTA_Modifier_Shredder_TwistedChakram_Invulnerable:
            pass
        class CDOTA_Modifier_Wisp_Relocate_Thinker:
            pass
        class CDOTA_Modifier_Silencer_CurseOfTheSilent:
            pass
        class CDOTA_Modifier_Broodmother_Silken_Bola:
            pass
        class CDOTA_Modifier_Leshrac_Lightning_Storm:
            pass
        class CDOTA_Modifier_ShadowShaman_Self_Hex:
            pass
        class CDOTA_Modifier_VengefulSpirit_Nether_Swap_DamageReduction:
            pass
        class CDOTA_Modifier_Item_Mask_Spin_Crit:
            pass
        class CDOTA_Modifier_Bane_Nightmare_AttackSpeed_Bonus:
            pass
        class CPulseCell_Step_DebugLog:
            pass
        class C_DOTA_Unit_Hero_Slardar:
            pass
        class C_DOTA_Item_Bottomless_Chalice:
            pass
        class C_DOTA_Item_Faerie_Fire:
            pass
        class C_DOTA_Ability_Weaver_Threads_Of_Fate:
            pass
        class C_DOTA_Ability_AntiMage_ManaThirst:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_500:
            pass
        class CDOTA_Modifier_Item_VindicatorsAxe:
            pass
        class CDOTA_Modifier_Item_Fallen_Sky:
            pass
        class CDOTA_Modifier_Eul_Cyclone:
            pass
        class CDOTA_Modifier_Item_CrellasCrozier_Active:
            pass
        class CDOTA_Modifier_Dawnbreaker_Celestial_Hammer_Caster:
            pass
        class CDOTA_Modifier_Legion_Commander_MomentOfCourage:
            pass
        class CDOTA_Modifier_TrollWarlord_Fervor:
            pass
        class CDOTA_Modifier_Medusa_FixedMovespeed:
            pass
        class CDOTA_Modifier_Wisp_Spirits:
            pass
        class CDOTA_Modifier_KeeperOfTheLight_Radiant_Bind:
            pass
        class CDOTA_Modifier_KeeperOfTheLight_ManaLeak:
            pass
        class CDOTA_Modifier_Meepo_Innate_PackRat:
            pass
        class CDOTA_Modifier_Beastmaster_WildAxes_Intrinsic:
            pass
        class CDOTA_Modifier_Warlock_Rain_of_Chaos_Leash:
            pass
        class CDOTA_Modifier_Courier_Morph_Effect:
            pass
        class C_DOTA_Item_Roshans_Banner:
            pass
        class C_DOTA_Item_LanceOfPursuit:
            pass
        class C_DOTA_Item_Naginata:
            pass
        class CDOTA_Ability_Abyssal_Underling_Warrior_Sight:
            pass
        class CDOTA_Ability_Slark_Barracuda:
            pass
        class C_DOTA_Ability_Rubick_Hidden2:
            pass
        class CDOTA_Ability_Furion_CurseOfTheForest:
            pass
        class CDOTA_Ability_Bloodseeker_Bloodrage:
            pass
        class C_DOTA_Ability_Creature_Fire_Breath:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Abaddon_4:
            pass
        class CDOTA_Modifier_Item_Enhancement_Greedy:
            pass
        class CDOTA_Modifier_Item_Eternal_Shroud_Bonus_Magic_Resist:
            pass
        class CDOTA_Modifier_StaffOfWizardry:
            pass
        class CDOTA_Modifier_Largo_Frogstomp_TreeShake:
            pass
        class CDOTA_Modifier_VoidSpirit_ResonantPulse_PhysicalBuff:
            pass
        class CDOTA_Modifier_Elder_Titan_AncestralSpirit_Hidden:
            pass
        class CDOTA_Modifier_Treant_Lifebomb_Explode:
            pass
        class CDOTA_Modifier_DoomBringer_InfernalBlade_Burn:
            pass
        class CDOTA_Modifier_Enchantress_Bunny_Hop:
            pass
        class CDOTA_Modifier_Venomancer_Latent_Poison:
            pass
        class CDOTA_Modifier_Death_Seeker_OutOfWorld:
            pass
        class CDOTA_Modifier_Necrolyte_Heartstopper_Aura_Effect:
            pass
        class CBodyComponentBaseAnimGraph:
            pass
        class CDOTA_Survivors_EffectsEntity:
            pass
        class C_DOTA_Item_Possessed_Mask:
            pass
        class C_DOTA_Item_Mirror_Shield:
            pass
        class C_DOTA_Item_Holy_Locket:
            pass
        class C_DOTA_Item_Recipe_Headdress:
            pass
        class C_DOTA_Ability_AbyssalUnderlord_Portal_Warp:
            pass
        class C_DOTA_Ability_Undying_CeaselessDirge:
            pass
        class C_DOTA_Ability_Ogre_Magi_Unrefined_Fireblast:
            pass
        class CDOTA_Ability_Life_Stealer_UnfetteredFury:
            pass
        class C_DOTA_Ability_Pugna_NetherWard:
            pass
        class C_DOTA_Ability_Beastmaster_Rugged:
            pass
        class C_DOTA_Ability_Beastmaster_DrumsOfSlom:
            pass
        class C_DOTA_Ability_Lich_FrostShield:
            pass
        class CDOTA_Modifier_AghsFort_Ascension_MagneticField_Evasion:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Riki_5:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lion_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Enigma_9:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Arc_Warden:
            pass
        class C_DOTA_Ability_Special_Bonus_Evasion_50:
            pass
        class C_DOTA_Ability_Special_Bonus_Cast_Range_350:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_90:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_Regen_20:
            pass
        class CDOTA_Modifier_Item_Enhancement_Mystical:
            pass
        class CDOTA_Modifier_Item_Diadem:
            pass
        class CDOTA_Modifier_Snapfire_Boomstick:
            pass
        class CDOTA_Modifier_Obsidian_Destroyer_Equilibrium_Debuff_Counter:
            pass
        class CDOTA_Modifier_SpiritBreaker_GreaterBash_Speed:
            pass
        class CDOTA_Modifier_Huskar_Berserkers_Blood:
            pass
        class CDOTA_Modifier_Life_Stealer_Assimilate:
            pass
        class CDOTA_Modifier_Tinker_MarchOfTheMachinesThinker:
            pass
        class CDOTA_Modifier_Tidehunter_AnchorSmash_Caster:
            pass
        class CDOTA_Modifier_Lich_Sinister_Gaze_Self:
            pass
        class CDOTA_Modifier_Kunkka_Torrent_Thinker:
            pass
        class CDOTA_Modifier_CrystalMaiden_IceRink_Thinker:
            pass
        class CDOTA_Modifier_AncientApparition_BoneChill:
            pass
        class CPulseCell_BaseYieldingInflow:
            pass
        class PulseNodeDynamicOutflows_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CDOTA_Unit_Hero_Mars:
            pass
        class C_DOTA_Item_Harpoon:
            pass
        class C_DOTA_Item_Witless_shako:
            pass
        class C_DOTA_Item_Flying_Courier:
            pass
        class C_DOTA_Ability_Bristleback_Warpath:
            pass
        class C_DOTA_Ability_Disruptor_StaticStorm:
            pass
        class C_DOTA_Ability_Rubick_Hidden4:
            pass
        class C_DOTA_Ability_BountyHunter_Lookout:
            pass
        class C_DOTA_Ability_Life_Stealer_Open_Wounds:
            pass
        class C_DOTA_Ability_AntiMage_Counterspell:
            pass
        class CDOTA_Modifier_SatyrHellcaller_UnholyAura:
            pass
        class CDOTA_Ability_FrostbittenGolem_TimeWarpAura:
            pass
        class C_DOTA_Ability_Ghost_FrostAttack:
            pass
        class C_DOTA_Ability_Aghanim_UrnUpheaval:
            pass
        class C_DOTA_Ability_PineCone_AcornShot:
            pass
        class C_DOTAFogOfWarTempViewers:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Kunkka_3:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Spectre:
            pass
        class C_DOTA_Ability_Special_Bonus_Agility_80:
            pass
        class CDOTA_Modifier_Item_Assault_Cuirass:
            pass
        class CDOTA_Modifier_Eul_Cyclone_Thinker:
            pass
        class CDOTA_Modifier_Grimstroke_Scepter_Buff:
            pass
        class CDOTA_Modifier_Winter_Wyvern_Arctic_Burn_Flight:
            pass
        class CDOTA_Modifier_Skywrath_Mage_Arcane_Bolt_Lifesteal:
            pass
        class CDOTA_Modifier_DoomBringer_Lvl_Pain:
            pass
        class CDOTA_Modifier_Rattletrap_Overclocking:
            pass
        class CPlayer_MovementServices_Humanoid:
            pass
        class C_DOTA_Unit_Hero_Necrolyte:
            pass
        class CDOTA_Item_Recipe_Arcane_Blink:
            pass
        class CDOTA_Item_Moonshard:
            pass
        class C_DOTA_Item_Soul_Ring:
            pass
        class C_DOTA_Item_Recipe_OblivionStaff:
            pass
        class C_DOTA_Ability_Tusk_Bitter_Chill:
            pass
        class C_DOTA_Ability_Centaur_Innate_Rawhide:
            pass
        class C_DOTA_Ability_SpiritBreaker_KnockbackAmplficiation:
            pass
        class C_DOTA_Ability_Ursa_Earthshock:
            pass
        class C_DOTA_Ability_Morphling_Accumulation:
            pass
        class C_DotaSubquestPlayerStat:
            pass
        class CDOTA_Modifier_AghsFort_BossWinterWyvern_Cold_Embrace_Debuff:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Pudge_6:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Amplify_16:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_650:
            pass
        class CDOTA_Modifier_Wind_Waker:
            pass
        class CDOTA_Modifier_Item_Wind_Waker:
            pass
        class CDOTA_Modifier_Item_Nullifier_Slow:
            pass
        class CDOTA_Modifier_Item_Infused_Raindrop:
            pass
        class CDOTA_Modifier_Largo_AmphibianRhapsody_Self_Tracker:
            pass
        class CDOTA_Modifier_MonkeyKing_Transform:
            pass
        class CDOTA_Modifier_Phoenix_IcarusYoink:
            pass
        class CDOTA_Modifier_Terrorblade_Demon_Zeal_Aura:
            pass
        class CDOTA_Modifier_Medusa_StoneGaze:
            pass
        class CDOTA_Modifier_Medusa_Mystic_Snake_AttackBuff:
            pass
        class CDOTA_Modifier_Keeper_of_the_Light_SpecialReserve:
            pass
        class CDOTA_Modifier_NightStalker_CripplingFear:
            pass
        class CDOTA_Modifier_Dazzle_Poison_Touch_Split:
            pass
        class CDOTA_Modifier_Nian_Knockdown:
            pass
        class CDOTA_Modifier_Lion_FingerPunch:
            pass
        class CDOTA_Modifier_Shadow_Shaman_Fowl_Play:
            pass
        class CDOTA_Modifier_InvisibleTrueSightImmune:
            pass
        class C_DOTA_DisplacementVisibility:
            pass
        class CPulseCell_IsRequirementValid__Criteria_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Unit_Hero_Chen:
            pass
        class C_DOTA_Unit_Hero_Kunkka:
            pass
        class C_DOTA_BaseNPC_Effigy_Statue:
            pass
        class C_PhysPropClientside:
            pass
        class CDOTA_Item_Forage_Health:
            pass
        class CDOTA_Item_Spark_Of_Courage:
            pass
        class CDOTA_Item_Helm_Of_The_Undying:
            pass
        class C_DOTA_Item_Recipe_Arcane_Boots:
            pass
        class C_DOTA_Item_Recipe_RefresherOrb:
            pass
        class CDOTA_Ability_Broodmother_SpinWeb_Destroy:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Lina_1:
            pass
        class C_DOTA_Ability_SandKing_CausticFinale:
            pass
        class C_BaseAnimatingOverlay:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Oracle_7:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Spectre_4:
            pass
        class C_DOTA_Ability_Special_Bonus_20_Bash_2:
            pass
        class C_DOTA_Ability_Special_Bonus_Evasion_75:
            pass
        class C_DOTA_Ability_Special_Bonus_All_Stats_20:
            pass
        class CDOTA_Modifier_Item_Enhancement_Boundless:
            pass
        class CDOTA_Modifier_Item_Terror_Mask_Fear_Aura:
            pass
        class CDOTA_Modifier_Nemesis_Curse:
            pass
        class CDOTA_Modifier_Item_Sorcerers_Staff:
            pass
        class CDOTA_Modifier_Item_Grove_Bow:
            pass
        class CDOTA_Modifier_Primal_Beast_Innate_Status_Resistance_Per_Time:
            pass
        class CDOTA_Modifier_Mars_Bulwark:
            pass
        class CDOTA_Modifier_Pangolier_HeartPiercer_Delay:
            pass
        class CDOTA_Modifier_Techies_LandMine:
            pass
        class CDOTA_Modifier_LegionCommander_IntimidateSlow:
            pass
        class CDOTA_Modifier_Silencer_LastWord_Disarm:
            pass
        class CDOTA_Modifier_Shukuchi_Slow:
            pass
        class CDOTA_Modifier_Furion_Sprout_Entangle:
            pass
        class CDOTA_Modifier_QueenOfPain_Innate_Seduction:
            pass
        class CDOTA_Modifier_Sniper_TakeAim_Bonus:
            pass
        class CDOTA_Modifier_Earthshaker_Shard:
            pass
        class CDOTA_Modifier_Juggernaut_Vaulted_Strike_Slashes:
            pass
        class C_BaseDoor:
            pass
        class CDOTA_Ability_BotChallenge_SkeletonKing_BoneGuard_DamageTracker:
            pass
        class CDOTA_Item_Recipe_Orb_Of_Destruction:
            pass
        class C_DOTA_Item_Horizon:
            pass
        class C_DOTA_Item_Recipe_InvisibilityEdge:
            pass
        class C_DOTA_Ability_Rubick_Hidden1:
            pass
        class C_DOTA_Ability_Tiny_Tree_Grab:
            pass
        class C_DOTA_Ability_Lina_Slow_Burn:
            pass
        class C_DOTA_Ability_CentaurKhan_WarStomp:
            pass
        class CDOTA_Ability_Seasonal_TI11_Rock:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Spirit_Breaker_2:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_80:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Speed_175:
            pass
        class C_DOTA_PlayerResource:
            pass
        class CDOTA_Modifier_Item_Enhancement_Nimble:
            pass
        class CDOTA_Modifier_Item_Silver_Edge:
            pass
        class CDOTA_Modifier_Largo_CroakOfGenius_Debuff:
            pass
        class CDOTA_Modifier_Snapfire_FiresnapCookie_ShortHop:
            pass
        class CDOTA_Modifier_TrollWarlord_Melee:
            pass
        class CDOTA_Modifier_Lycan_Shapeshift_Speed:
            pass
        class CDOTA_Modifier_Earthshaker_EnchantTotem_Animation:
            pass
        class CDOTA_Modifier_SkeletonKing_MortalStrikeBonus:
            pass
        class CDOTA_Modifier_Mirana_Starfall_Starstruck:
            pass
        class C_DOTABaseGameMode:
            pass
        class EntityRenderAttribute_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CPulseCell_Inflow_ObservableVariableListener:
            pass
        class CDOTA_BaseNPC_Largo_Frogling:
            pass
        class C_DOTA_Item_Eternal_Shroud:
            pass
        class CDOTA_Item_Recipe_Force_Boots:
            pass
        class C_DOTA_Item_Recipe_Blade_Mail:
            pass
        class CDOTA_Ability_VoidSpirit_AetherRemnant:
            pass
        class C_DOTA_Ability_Rubick_TelekinesisLand_Self:
            pass
        class C_DOTA_Ability_Invoker_Empty1:
            pass
        class CDOTA_Ability_Broodmother_SpawnSpiderlings:
            pass
        class CDOTA_Ability_Night_Stalker_Heart_Of_Darkness:
            pass
        class C_DOTA_Ability_Courier_GoToEnemySecretShop:
            pass
        class CDOTA_Ability_Pudge_Dismember:
            pass
        class CDOTA_Modifier_Spawnlord_Master_Stomp:
            pass
        class CDOTA_Modifier_Mutation_Treecutter_Aura:
            pass
        class C_DOTA_Item_BagOfGold:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Nyx_6:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Drow_Ranger_8:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_25:
            pass
        class CDOTA_Modifier_Item_Ethereal_Blade_Slow:
            pass
        class CDOTA_Modifier_VoidSpirit_AetherRemnantCreepDamage:
            pass
        class CDOTA_Modifier_Mars_ArenaOfBlood_Spear:
            pass
        class CDOTA_Modifier_Bristleback_Warpath_Stack:
            pass
        class CDOTA_Modifier_Disruptor_Thunder_Strike_Slow:
            pass
        class CDOTA_Modifier_Ogre_Magi_Ignite:
            pass
        class CDOTA_Modifier_Shadow_Demon_Soul_Catcher_SpellAmp:
            pass
        class CDOTA_Modifier_FacelessVoid_TimeWalk_ShardBuff:
            pass
        class CDOTA_Modifier_Miniboss_InherentBuffs:
            pass
        class CDOTA_Modifier_Kunkka_Tidebringer_Slow:
            pass
        class CDOTA_Modifier_Zuus_LightningBoltThinker:
            pass
        class CDOTA_Modifier_Morphling_Replicate_Illusion:
            pass
        class CDOTA_Modifier_Razor_Dynamo:
            pass
        class CDOTA_Modifier_CrystalMaiden_FreezingField_Slow:
            pass
        class CDOTA_Modifier_VengefulSpirit_Nether_Swap_ChargeCounter:
            pass
        class CDOTA_Modifier_Drow_Ranger_Multishot_Damage:
            pass
        class CDOTA_Modifier_Nevermore_Frenzy:
            pass
        class CDOTA_Modifier_Rune_SuperRegen:
            pass
        class C_DOTA_Unit_Twin_Gate:
            pass
        class C_DOTA_Item_Sange:
            pass
        class C_DOTA_Ability_Oracle_Clairvoyant_Curse:
            pass
        class C_DOTA_Ability_LoneDruid_SpiritBear_SpiritLink:
            pass
        class C_DOTA_Ability_Invoker_AttributeBonus:
            pass
        class C_DOTA_Ability_DoomBringer_Doom:
            pass
        class C_DOTA_Ability_Weaver_Rewoven:
            pass
        class CDOTA_Ability_Rattletrap_Armor_Power:
            pass
        class C_DOTA_Ability_Puck_DreamCoil:
            pass
        class C_DOTA_Ability_Special_Bonus_Cast_Range_175:
            pass
        class CDOTA_Modifier_Item_Enhancement_Dominant:
            pass
        class CDOTA_Modifier_Item_Guardian_Shell:
            pass
        class CDOTA_Modifier_Item_Force_Field_Bonus:
            pass
        class CDOTA_Modifier_Item_GlimmerCape:
            pass
        class CDOTA_Modifier_Item_Medallion_Of_Courage_Armor_Addition:
            pass
        class CDOTA_Modifier_Item_PhaseBoots:
            pass
        class CDOTA_Modifier_ArcWarden_MagneticField_Thinker_Evasion:
            pass
        class CDOTA_Modifier_TrollWarlord_BattleTrance_Vision:
            pass
        class CDOTA_Modifier_Shadow_Demon_Soul_Catcher:
            pass
        class CDOTA_Modifier_Lycan_Shapeshift_Transform:
            pass
        class CDOTA_Modifier_Lifestealer_CorpseEater:
            pass
        class CDOTA_Modifier_Pugna_NetherWard_Aura:
            pass
        class CDOTA_Modifier_Roshan_Moving:
            pass
        class CDOTA_Modifier_Riki_TricksOfTheTrade:
            pass
        class CDOTA_Modifier_Axe_BerserkersCall:
            pass
        class CIngameEvent_SeasonalRewardLine:
            pass
        class CFilterMultipleAPI:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Unit_Hero_Dazzle:
            pass
        class C_DOTA_Item_Recipe_Vanguard:
            pass
        class C_DOTA_Ability_ArcWarden_Flux:
            pass
        class C_DOTA_Ability_Terrorblade_Reflection:
            pass
        class CDOTA_Ability_Nevermore_Frenzy:
            pass
        class C_DOTA_Ability_LotusPool:
            pass
        class CDOTA_Modifier_Aghsfort_Aziyog_Underlord_Portal_Warp_Channel:
            pass
        class CDOTA_Ability_Aghsfort_Aziyog_Underlord_Firestorm:
            pass
        class CDOTA_Ability_Aghanim_Spear:
            pass
        class C_DOTA_Ability_AghsFort_Gyrocopter_Multi_Homing_Missile:
            pass
        class CDOTA_Item_AghsFort_RefresherOrb_Shard:
            pass
        class C_DOTA_Ability_Corspselord_Revive:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Techies_2:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Puck_2:
            pass
        class C_DOTA_Ability_Special_Bonus_Lifesteal_20:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Range_75:
            pass
        class C_DOTA_Ability_Special_Bonus_Agility_9:
            pass
        class CDOTA_Modifier_Item_Guardian_Greaves_Aura:
            pass
        class CDOTA_Modifier_Item_NullTalisman:
            pass
        class CDOTA_Modifier_VoidSpirit_AetherRemnantThinker:
            pass
        class CDOTA_Modifier_MonkeyKing_RightClickJump_Activity:
            pass
        class CDOTA_Modifier_Techies_StasisTrap:
            pass
        class CDOTA_Modifier_Brewmaster_EarthBrewling_EarthStance:
            pass
        class CDOTA_Modifier_Brewmaster_PermanentImmolation_Aura:
            pass
        class CDOTA_Modifier_Rattletrap_Cog_Marker:
            pass
        class CDOTA_Modifier_FacelessVoid_TimeWalk_Tracker:
            pass
        class CDOTA_Modifier_Sand_King_Scorpion_Strike_Slow:
            pass
        class TransitioningLayer_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Unit_Hero_Shredder:
            pass
        class C_DOTA_Unit_Hero_Rubick:
            pass
        class C_DOTA_Item_Kaya:
            pass
        class C_DOTA_Item_PowerTreads:
            pass
        class C_DOTA_Ability_Venomancer_Sepsis:
            pass
        class C_DOTA_Ability_Warlock_Fatal_Bonds:
            pass
        class C_DOTA_Ability_Courier_Shield:
            pass
        class TempViewerInfo_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Wraith_King_10:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Juggernaut_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_65:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Speed_110:
            pass
        class CDOTA_Modifier_Item_Enhancement_Manic:
            pass
        class CDOTA_Modifier_Divine_Regalia:
            pass
        class CDOTA_Modifier_Seer_Stone_Truesight:
            pass
        class CDOTA_Modifier_Item_Ancient_Janggo_Aura:
            pass
        class CDOTA_Modifier_Item_DustofAppearance_Thinker:
            pass
        class CDOTA_Modifier_Item_QuellingBlade:
            pass
        class CDOTA_Modifier_Dawnbreaker_Luminosity:
            pass
        class CDOTA_Modifier_Hoodwink_Sharpshooter_VisionThinker:
            pass
        class CDOTA_Modifier_AbyssalUnderlord_PitOfMalice_Thinker:
            pass
        class CDOTA_Modifier_Techies_StickyBombThrow:
            pass
        class CDOTA_Modifier_Slark_EssenceShift_DebuffCounter:
            pass
        class CDOTA_Modifier_Broodmother_SpinWeb_ChargeCounter:
            pass
        class CDOTA_Modifier_Nian_Dive:
            pass
        class CDOTA_Modifier_Lion_FingerOfDeath_Delay:
            pass
        class CDOTA_Modifier_Sniper_Assassinate_Trigger:
            pass
        class CDOTA_Modifier_Morphling_ScepterStatsDrain_Strength_Buff:
            pass
        class C_IngameEvent_TI6:
            pass
        class C_DOTA_Unit_Hero_DoomBringer:
            pass
        class C_DOTA_Item_RuneSpawner_Powerup:
            pass
        class C_DOTA_Item_Enhancement_Vital:
            pass
        class C_DOTA_Item_Unwavering_Condition:
            pass
        class CDOTA_Item_Orb_Of_Destruction:
            pass
        class CDOTA_Item_Recipe_Fallen_Sky:
            pass
        class C_DOTA_Item_Recipe_Phylactery:
            pass
        class C_DOTA_Item_RingOfProtection:
            pass
        class C_DOTA_Ability_Legion_Commander_Duel:
            pass
        class C_DOTA_Ability_Rubick_Hidden5:
            pass
        class C_DOTA_Ability_Undying_Decay:
            pass
        class C_DOTA_Ability_Leshrac_Lightning_Storm:
            pass
        class C_DOTA_Ability_Venomancer_PlagueWard:
            pass
        class CDOTA_Modifier_Tutorial_Sleep:
            pass
        class CDOTA_Modifier_Mutation_StationaryDamageReduction:
            pass
        class CBaseAnimatingActivity:
            pass
        class CDOTA_Modifier_AghsFort_AssaultCaptain_SunRay:
            pass
        class C_DOTA_Ability_Special_Bonus_Gold_Income_300:
            pass
        class C_DOTA_Ability_Special_Bonus_Strength_8:
            pass
        class CDOTA_Modifier_Item_Enchanters_Bauble:
            pass
        class CDOTA_Modifier_Item_Paintball_Debuff:
            pass
        class CDOTA_Modifier_Grimstroke_SoulChain_ChannelCheck:
            pass
        class CDOTA_Modifier_Winter_Wyvern_Cold_Embrace:
            pass
        class CDOTA_Modifier_AbyssalUnderlord_AtrophyAura_HeroDmgBuff:
            pass
        class CDOTA_Modifier_TrollWarlord_BattleTrance_Ally:
            pass
        class CDOTA_Modifier_Medusa_ManaShield_AttackBuff:
            pass
        class CDOTA_Modifier_Nyx_Assassin_Vendetta_Fast:
            pass
        class CDOTA_Modifier_Obsidian_Destroyer_Equilibrium_Barrier:
            pass
        class CDOTA_Modifier_Invoker_IceWall_VectorTarget_Thinker:
            pass
        class CDOTA_Modifier_Lina_LightStrikeArray_Thinker:
            pass
        class CDOTA_Modifier_SkeletonKing_BoneGuard_Summon:
            pass
        class CDOTA_Modifier_HeroStatuePedestal:
            pass
        class CIngameEvent_TI2025:
            pass
        class CModelState:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CPulseCell_LerpCameraSettings__CursorState_t:
            pass
        class CPulseCell_Outflow_CycleOrdered:
            pass
        class CDOTA_BaseNPC_Tinker_Turret:
            pass
        class C_DOTA_Item_Diadem:
            pass
        class CDOTA_Item_Kaya_And_Sange:
            pass
        class C_DOTA_Item_Recipe_Bloodthorn:
            pass
        class CDOTA_Item_Recipe_DragonLance:
            pass
        class C_DOTA_Item_IronwoodBranch:
            pass
        class CDOTA_Ability_Muerta_SpectralSlug:
            pass
        class C_DOTA_Ability_ArcWarden_TempestDouble:
            pass
        class CDOTA_Ability_AbyssalUnderlord_Firestorm:
            pass
        class C_DOTA_Ability_Elder_Titan_Tip_The_Scales:
            pass
        class C_DOTA_Ability_NagaSiren_MirrorImage:
            pass
        class C_DOTA_Ability_Silencer_Oppressive_Silence:
            pass
        class C_DOTA_Ability_Invoker_IceWall:
            pass
        class C_DOTA_Ability_FacelessVoid_TimeWalk:
            pass
        class C_DOTA_Ability_Beastmaster_Hawk_Dive:
            pass
        class CDOTA_Ability_FillerAbility:
            pass
        class CDOTA_Modifier_FelBeast_Haunt_OnDeath:
            pass
        class CDOTA_Modifier_Spawnlord_Aura:
            pass
        class CDOTA_Modifier_AncientRockGolem_Weakening:
            pass
        class CDOTA_Ability_Seasonal_TI11_Duel:
            pass
        class CDOTA_Ability_Seasonal_TI11_Paper:
            pass
        class CDOTA_Ability_Seasonal_TI11_RockPaperScissors_Base:
            pass
        class CDOTA_Modifier_Aghsfort_Reward_ArmorAura_Bonus:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Tidehunter_8:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Pudge_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_45:
            pass
        class C_DOTA_Ability_Special_Bonus_Agility_6:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Speed_225:
            pass
        class CDOTA_Modifier_Item_Gungir:
            pass
        class CDOTA_Modifier_Item_EchoSabre:
            pass
        class CDOTA_Modifier_Necronomicon_Warrior_Sight:
            pass
        class CDOTA_Modifier_NagaSiren_SongOfTheSiren_IgnoreMe:
            pass
        class CDOTA_Modifier_SpiritBreaker_GreaterBash_Guaranteed:
            pass
        class CDOTA_Modifier_SpiritBreaker_GreaterBash_CascadingBashes:
            pass
        class CDOTA_Modifier_Miniboss_Minion_Summoner_Buff:
            pass
        class CDOTA_Modifier_Lina_CritDebuff:
            pass
        class CIngameEvent_TI2023:
            pass
        class CCollisionProperty:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_BaseNPC_Hero:
            pass
        class C_DOTA_Item_Tier1Token:
            pass
        class CFilterMassGreater:
            pass
        class C_DOTA_Item_ManaDraught:
            pass
        class CDOTA_Item_Recipe_Broom_Handle:
            pass
        class C_DOTA_Item_Recipe_HandOfMidas:
            pass
        class C_DOTA_Ability_Skywrath_Mage_Ancient_Seal:
            pass
        class CDOTA_Ability_Nyx_Assassin_Burrow:
            pass
        class C_EntityDissolve:
            pass
        class C_DOTA_Ability_Special_Bonus_Lifesteal_18:
            pass
        class C_DOTA_Ability_Special_Bonus_Respawn_Reduction_60:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Amplify_15:
            pass
        class C_DOTA_Ability_Special_Bonus_Cast_Range_75:
            pass
        class CDOTA_Modifier_OgreSealTotem_Slow:
            pass
        class CDOTA_Modifier_Item_Elven_Tunic:
            pass
        class CDOTA_Modifier_Largo_Song_Attack_Burst:
            pass
        class CDOTA_Modifier_EarthSpirit_BoulderSmash:
            pass
        class CDOTA_Modifier_Slark_ShadowDance_Passive:
            pass
        class CDOTA_Modifier_Wisp_Tether_Cooldown:
            pass
        class CDOTA_Modifier_Lycan_SummonWolves_Hamstring:
            pass
        class CDOTA_Modifier_DarkSeer_IonShell_IllusionInvulnerability:
            pass
        class CDOTA_Modifier_Bloodseeker_Sanguivore:
            pass
        class QuickBuySlot_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_SoundOpvarSetOBBEntity:
            pass
        class C_DOTA_Item_EyeOfTheVizier:
            pass
        class C_DOTA_Item_Voodoo_Mask:
            pass
        class CDOTA_Item_Enchanted_Mango:
            pass
        class C_DOTA_Item_Recipe_Radiance:
            pass
        class C_DOTA_Ability_DarkWillow_Bedlam:
            pass
        class C_DOTA_Ability_Medusa_StoneGaze:
            pass
        class CDOTA_Ability_Neutral_Upgrade:
            pass
        class C_DOTA_Ability_HarpyStorm_ChainLightning:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lycan_1:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Phantom_Assassin:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Enigma_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Ember_Spirit_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Strength_6:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Speed_50:
            pass
        class CPulseCell_WaitForPanelClass__CursorState_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CDOTA_Modifier_Item_PigletPole:
            pass
        class CDOTA_Modifier_Item_Angels_Demise_Break:
            pass
        class CDOTA_Modifier_Item_Quarterstaff:
            pass
        class CDOTA_Modifier_Elder_Titan_Tip_The_Scales_Effect:
            pass
        class CDOTA_Modifier_Rubick_FadeBoltBuff:
            pass
        class CDOTA_Modifier_Undying_Tombstone_Zombie_Deathstrike_Slow:
            pass
        class CDOTA_Modifier_Treant_LeechSeedPassive:
            pass
        class CDOTA_Modifier_Lycan_SummonWolves_Maim:
            pass
        class CDOTA_Modifier_Truesight_Aura:
            pass
        class C_DOTA_Item_Recipe_Caster_Rapier:
            pass
        class C_DOTA_Item_Ironwood_tree:
            pass
        class CDOTA_Item_Recipe_MeteorHammer:
            pass
        class C_DOTA_Item_Skadi:
            pass
        class C_DOTA_Item_VitalityBooster:
            pass
        class C_DOTA_Ability_DarkWillow_Terrorize:
            pass
        class C_DOTA_Ability_Lina_DragonSlave:
            pass
        class C_DOTA_Ability_CrystalMaiden_FreezingFieldStop:
            pass
        class CDOTA_Item_BookStrength:
            pass
        class CDOTA_Modifier_UpheavalUrn_Reincarnation:
            pass
        class CDOTA_Modifier_PudgeMiniboss_HatefulStrike:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Chen_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Centaur_1:
            pass
        class CDOTA_Modifier_Special_Bonus_Crit:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_Regen_1:
            pass
        class CDOTA_Modifier_Item_Enhancement_Wise:
            pass
        class CDOTA_Modifier_Item_The_Leveller:
            pass
        class CDOTA_Modifier_Tome_Of_Knowledge:
            pass
        class CDOTA_Modifier_Undying_Tombstone_Bunker_Cooldown:
            pass
        class CDOTA_Modifier_Chen_HandOfGod_Invuln:
            pass
        class CDOTA_Modifier_Lich_Sinister_Gaze_Cryophobia_Debuff:
            pass
        class CDOTA_Modifier_SkeletonKing_BoneGuard:
            pass
        class CDOTA_Modifier_Pudge_RottenCore:
            pass
        class CDOTA_Modifier_Teleporting_Root_Logic:
            pass
        class CTormentorSpawnPhase:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_SimpleObstruction:
            pass
        class CAttributeContainer:
            pass
        class C_DOTA_Unit_Hero_Bristleback:
            pass
        class CDOTA_Unit_Announcer:
            pass
        class C_ClientRagdoll:
            pass
        class CDOTA_Item_Enchanted_Quiver:
            pass
        class C_DOTA_Ability_Meepo_Ransack:
            pass
        class C_DOTA_Ability_Sniper_Shrapnel:
            pass
        class C_DOTA_Ability_Lich_DeathCharge:
            pass
        class CDOTA_Modifier_Passive_Lotus_Pool_Building:
            pass
        class CDOTA_Modifier_AghsFort_Blessings_Debuff_Duration_Increase:
            pass
        class CDOTA_Ability_AghsFort_Ravage_Potion:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Grimstroke_3:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lycan_5:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lich_2:
            pass
        class CDOTA_Modifier_BotChallenge_SkeletonKing_BoneGuard:
            pass
        class CDOTA_Modifier_Item_AeonDisk:
            pass
        class CDOTA_Modifier_Item_Javelin:
            pass
        class CDOTA_Modifier_Primal_Beast_Trample_HasteAura:
            pass
        class CDOTA_Modifier_Mars_ArenaOfBlood_SelfLeash:
            pass
        class CDOTA_Modifier_MonkeyKing_QuadrupleTap_Counter:
            pass
        class CDOTA_Modifier_Shredder_Flamethrower_TreeFire_Thinker:
            pass
        class CDOTA_Modifier_Obsidian_Destroyer_Equilibrium_Debuff:
            pass
        class CDOTA_Modifier_SpiritBreaker_PlanarPocket:
            pass
        class CDOTA_Modifier_DarkSeer_Vacuum:
            pass
        class CDOTA_Modifier_Kunkka_XMarksTheSpot:
            pass
        class CDOTA_Modifier_Sand_King_BurrowStrike:
            pass
        class CDOTA_Modifier_AncientApparition_ColdFeet:
            pass
        class PulseSelectorOutflowList_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class C_DOTA_Unit_Hero_Legion_Commander:
            pass
        class C_DOTA_Unit_Hero_Warlock:
            pass
        class C_DOTA_Item_Gungir:
            pass
        class C_DOTA_Item_Recipe_Disperser:
            pass
        class C_DOTA_Ability__Primal_Beast_Colossal:
            pass
        class C_DOTA_Ability_Skywrath_Mage_Concussive_Shot:
            pass
        class C_DOTA_Ability_Treant_Overgrowth:
            pass
        class C_DOTA_Ability_Invoker_Exort:
            pass
        class C_DOTA_Ability_DrowRanger_CreepRally:
            pass
        class C_DOTA_Ability_SatyrSoulstealer_ManaBurn:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Oracle_2:
            pass
        class C_DOTA_Ability_Special_Bonus_Agility_30:
            pass
        class C_DOTA_Ability_Special_Bonus_Agility_25:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_Regen_150:
            pass
        class CShmupPanoramaFuncs:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class ClientQuickBuyItemState:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Modifier_Item_Gossamer_Cape:
            pass
        class CDOTA_Modifier_Item_Buckler_Effect:
            pass
        class CDOTA_Modifier_DarkWillow_Creature_Invulnerable:
            pass
        class CDOTA_Modifier_Techies_RemoteMine:
            pass
        class CDOTA_Modifier_Invoker_ColdSnap:
            pass
        class CDOTA_Modifier_Huskar_Life_Break_Charge:
            pass
        class CDOTA_Modifier_Enchantress_RabbleRouser:
            pass
        class CDOTA_Modifier_Omniknight_Innate_HealingHammer:
            pass
        class CDOTA_Modifier_Leshrac_ChronopticNourishment:
            pass
        class CDOTA_Modifier_ShadowShaman_Shackles:
            pass
        class CDOTA_Modifier_Morphling_Max_Attribute_Shift_Tracker:
            pass
        class CDOTA_Modifier_Disable_Healing:
            pass
        class CDOTA_Modifier_Filler_Heal_Aura:
            pass
        class CPulseCell_PlaySequence__CursorState_t:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class CBodyComponentSkeletonInstance:
            pass
        class C_DOTA_Unit_Hero_Razor:
            pass
        class C_DOTA_Item_Recipe_Soul_Booster:
            pass
        class C_DOTA_Item_Recipe_RodOfAtos:
            pass
        class C_DOTA_Ability_Terrorblade_Demon_Zeal:
            pass
        class C_DOTA_Ability_Skywrath_Mage_Shield_Of_The_Scion:
            pass
        class CDOTA_Ability_Keeper_of_the_Light_BrightSpeed:
            pass
        class C_DOTA_Ability_Weaver_TimeLapse:
            pass
        class C_DOTA_Ability_Leshrac_Defilement:
            pass
        class C_DOTA_Ability_DragonKnight_DragonBlood:
            pass
        class C_DOTA_Ability_Nian_Waterball:
            pass
        class CDOTA_Ability_Roshan_Devotion:
            pass
        class CDOTA_Ability_KoboldTunneler_Prospecting:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lycan_2:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Crystal_Maiden_1:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Centaur_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Amplify_14:
            pass
        class CDOTA_Ability_Special_Bonus_Spell_AoE_100:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Speed_10:
            pass
        class CDOTA_Modifier_BotChallenge_SkeletonKing_BoneGuard_Summon_Thinker:
            pass
        class CDOTA_Modifier_Item_MaskOfMadness:
            pass
        class CDOTA_Modifier_Ringmaster_Wheel_Mesmerize_Pull:
            pass
        class CDOTA_Modifier_Muerta_Supernatural:
            pass
        class CDOTA_Modifier_Centaur_Return:
            pass
        class CDOTA_Modifier_Furion_Money_Tree:
            pass
        class CDOTA_Modifier_Special_Bonus_Unique_Beastmaster_6_Aura:
            pass
        class CDOTA_Modifier_PhantomLancer_Doppelwalk_Illusion:
            pass
        class CDOTA_Modifier_Pudge_GraftFlesh:
            pass
        class CDOTA_Modifier_AncientApparition_DeathRime:
            pass
        class CDOTA_Modifier_Muted:
            pass
        class CDOTA_Modifier_ScoutVisible:
            pass
        class CDOTA_Item_Recipe_Havoc_Hammer:
            pass
        class C_DOTA_Item_Iron_Talon:
            pass
        class CDOTA_Ability_Elder_Titan_Momentum:
            pass
        class CDOTA_Ability_Broodmother_Silken_Bola:
            pass
        class C_DOTA_Ability_Firework_Mine:
            pass
        class C_DOTA_Ability_Ursa_Overpower:
            pass
        class CDOTA_Ability_Shadow_Shaman_Urnaconda:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Lina_3:
            pass
        class C_DOTA_Ability_Juggernaut_BladeDance:
            pass
        class CDOTA_Modifier_Greevil_Miniboss_Red_Overpower:
            pass
        class CDOTA_Modifier_DoNotCastEnsnare:
            pass
        class C_DOTA_Ability_AghsFort_Wave_Blast:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Underlord_8:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_400:
            pass
        class CDOTA_Modifier_Item_Giants_Ring_Visual:
            pass
        class CDOTA_Modifier_Havoc_Hammer_Slow:
            pass
        class CDOTA_Modifier_Item_GlimmerCape_Building_Limit:
            pass
        class CDOTA_Modifier_Item_Diffusal_Blade_Slow:
            pass
        class CDOTA_Modifier_Marci_CompanionRun_Intrinsic:
            pass
        class CDOTA_Modifier_ArcWarden_TempestDouble_Debuff:
            pass
        class CDOTA_Modifier_LegionCommander_OverwhelmingOdds_Shield:
            pass
        class CDOTA_Modifier_Ogre_Magi_Ignite_Multicast:
            pass
        class CDOTA_Modifier_Brewmaster_BottomsUp_Passive:
            pass
        class CDOTA_Modifier_Alchemist_GoblinsGreed:
            pass
        class CDOTA_Modifier_Tinker_Turret_Drop_Thinker:
            pass
        class CDOTA_Modifier_ShadowShaman_SerpentWardInvuln:
            pass
        class CDOTA_Modifier_Windrunner_Tailwind:
            pass
        class CDOTA_Modifier_Bloodseeker_BloodMist_Barrier:
            pass
        class CDOTA_Modifier_Earthshaker_EnchantTotem:
            pass
        class CDOTA_Modifier_DrowRanger_WaveOfSilence_Buff:
            pass
        class CDOTA_Modifier_Bane_Enfeeble_Effect:
            pass
        class C_DOTA_Unit_Undying_Tombstone:
            pass
        class CDOTA_Item_The_Leveller:
            pass
        class CDOTA_Item_Demonicon:
            pass
        class C_DOTA_Item_Flicker:
            pass
        class C_DOTA_Item_Recipe_Yasha:
            pass
        class CDOTA_Ability_Abyssal_Underling_Warrior_LastWill:
            pass
        class C_DOTA_Ability_Medusa_SplitShot:
            pass
        class CDOTA_Ability_Shadow_Demon_Shadow_Servant:
            pass
        class C_DOTA_Ability_FacelessVoid_Chronosphere:
            pass
        class C_DOTA_Ability_Nian_Frenzy:
            pass
        class C_DOTA_Item_Tombstone:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Amplify_11:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_16:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_Regen_4:
            pass
        class CDOTA_Modifier_Item_Enhancement_Fleetfooted:
            pass
        class CDOTA_Modifier_Item_Pipe:
            pass
        class CDOTA_Modifier_Item_Sphere_Upgrade_Absorb:
            pass
        class CDOTA_Modifier_Kez_Katana_Shard_Active:
            pass
        class CDOTA_Modifier_Pangolier_Rollup:
            pass
        class CDOTA_Modifier_AbyssalUnderlord_DarkRift:
            pass
        class CDOTA_Modifier_Centaur_Mounted:
            pass
        class CDOTA_Modifier_Jakiro_LiquidIce:
            pass
        class CDOTA_Modifier_BountyHunter_Track_Bear_Trap:
            pass
        class CDOTA_Modifier_Enchantress_NaturesAttendants_DamageTracker:
            pass
        class CDOTA_Modifier_Rattletrap_Cog:
            pass
        class CDOTA_Modifier_Courier_Flying:
            pass
        class CDOTA_Modifier_Necrolyte_GhostShroud_Aura_Effect:
            pass
        class CScriptComponent:
            pass
        class C_DOTA_BaseNPC_Trap_Ward:
            pass
        class C_DOTA_Ability_Pangolier_ShieldCrash:
            pass
        class C_DOTA_Ability_MonkeyKing_Transform:
            pass
        class C_DOTA_Ability_Bristleback_Bristleback:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Disruptor_2:
            pass
        class C_DOTA_Ability_Invoker_ForgeSpirit_AD:
            pass
        class CDOTA_Ability_DarkSeer_WallOfReplica:
            pass
        class C_DOTA_Ability_Tidehunter_DeadInTheWater:
            pass
        class C_DOTA_Ability_Sven_GodsStrength:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Tusk_6:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Invoker_9:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Luna_6:
            pass
        class C_DOTA_Ability_Special_Bonus_Strength_25:
            pass
        class CDOTA_Modifier_Item_Orb_Of_Frost:
            pass
        class CDOTA_Modifier_Blight_Stone_Corruption:
            pass
        class CDOTA_Modifier_PrimalBeast_Uproar:
            pass
        class CDOTA_Modifier_Snapfire_FiresnapCookie_AllyFlailAnim:
            pass
        class CDOTA_Modifier_MonkeyKing_TreeDance_Activity:
            pass
        class CDOTA_Modifier_AbyssalUnderlord_PitOfMalice_Ensnare:
            pass
        class CDOTA_Modifier_LegionCommander_DuelAura_Effect:
            pass
        class CDOTA_Modifier_Abaddon_Mist_Coil_Damage_Penalty:
            pass
        class CDOTA_Modifier_NagaSiren_Eelskin:
            pass
        class CDOTA_Modifier_Nyx_Assassin_Nyxth_Sense:
            pass
        class CDOTA_Modifier_Huskar_Life_Break:
            pass
        class CDOTA_Modifier_DragonKnight_Fireball_Thinker:
            pass
        class CDOTA_Modifier_Nevermore_Necromastery:
            pass
        class CDOTA_Modifier_Juggernaut_Healing_Ward_Tracker:
            pass
        class CDOTA_Modifier_Tutorial_Disable_Healing:
            pass
        class C_PortraitWorldCallbackHandler:
            pass
        class C_DynamicProp:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Batrider_1:
            pass
        class C_DOTA_Ability_Omniknight_Hammer_Of_Purity:
            pass
        class CDOTA_Ability_Puck_WaningRift:
            pass
        class C_DOTA_Ability_Greevil_Miniboss_Yellow_Surge:
            pass
        class C_DOTA_Ability_DarkTrollWarlord_RaiseDead:
            pass
        class CDOTA_Modifier_Aghsfort_Enrage:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Silencer_7:
            pass
        class C_DOTA_Ability_Special_Bonus_Evasion_15:
            pass
        class C_DOTA_Ability_Special_Bonus_Magic_Resistance_35:
            pass
        class C_DOTA_Ability_Special_Bonus_Movement_Speed_15:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_300:
            pass
        class CDOTA_Modifier_Roshans_Banner_Effect:
            pass
        class CDOTA_Modifier_Item_Book_Of_Shadows_Buff:
            pass
        class CDOTA_Modifier_Item_Mechanical_Arm:
            pass
        class CDOTA_Modifier_Item_EagleEye:
            pass
        class CDOTA_Modifier_Techies_Minefield_Sign_Thinker:
            pass
        class CDOTA_Modifier_Techies_StasisTrap_Stunned:
            pass
        class CDOTA_Modifier_Obsidian_Destroyer_Mana_Allergy:
            pass
        class CDOTA_Modifier_DragonKnight_WyrmsWrath:
            pass
        class CDOTA_Modifier_WitchDoctor_DeathWard_Voodoo_Switcheroo_AttackSpeedReduction:
            pass
        class CDOTA_Modifier_Bloodseeker_BloodMist:
            pass
        class CDOTA_Modifier_Axe_CullingBlade_JungleWeaponGesture:
            pass
        class C_DOTA_Unit_Hero_Jakiro:
            pass
        class C_EnvDeferredLight:
            pass
        class CDOTA_Item_Searing_Signet:
            pass
        class C_DOTA_Item_BootsOfSpeed:
            pass
        class C_DOTA_Ability_Lich_Sinister_Gaze:
            pass
        class CDOTA_Ability_Frogmen_WaterBubble_Base:
            pass
        class CDOTA_Modifier_EnragedWildkin_ToughnessAura_Bonus:
            pass
        class CDOTA_Modifier_Ascension_AcidBlood:
            pass
        class CDOTA_Modifier_Special_Bonus_Exp_Boost:
            pass
        class CDOTA_Modifier_Item_WizardHat:
            pass
        class CDOTA_Modifier_Cloak_Of_Flames_Debuff:
            pass
        class CDOTA_Modifier_Item_Iron_Talon:
            pass
        class CDOTA_Modifier_Muerta_TheCalling_Invulnerable:
            pass
        class CDOTA_Modifier_Skywrath_Mage_Ruin_And_Restoration:
            pass
        class CDOTA_Modifier_Tusk_WalrusPunch_AirTime:
            pass
        class CDOTA_Modifier_Gyrocopter_Rocket_Barrage_Movespeed:
            pass
        class CDOTA_Modifier_Weaver_GeminateAttack:
            pass
        class CDOTA_Modifier_Broodmother_SpawnSpiderlings:
            pass
        class CDOTA_Modifier_PhantomAssassin_StiflingDagger:
            pass
        class CDOTA_Modifier_Morphling_Adaptive_Strike:
            pass
        class CDOTA_Modifier_Bloodseeker_Rupture:
            pass
        class CDOTA_Modifier_Earthshaker_Spirit_Cairn:
            pass
        class CDOTA_Modifier_Sven_Wrath_Of_God:
            pass
        class CDOTA_Modifier_Hidden_NoDamage:
            pass
        class CIngameEvent_NewBloom2019:
            pass
        class C_DOTA_Unit_Miniboss:
            pass
        class CDOTA_Unit_CustomGameAnnouncerAghanim:
            pass
        class CDOTA_Item_Falconers_Glove:
            pass
        class CDOTA_Item_Recipe_SpecialistsArray:
            pass
        class CDOTA_Item_Recipe_Panic_Button:
            pass
        class CDOTA_Item_Broom_Handle:
            pass
        class CDOTA_Item_ForceStaff:
            pass
        class CDOTA_Ability_Grimstroke_SpiritWalk:
            pass
        class C_DOTA_Ability_Legion_Commander_MomentOfCourage:
            pass
        class C_DOTA_Ability_Batrider_Flamebreak:
            pass
        class C_DOTA_Ability_Huskar_Inner_Fire:
            pass
        class C_DOTA_Ability_Witch_Doctor_Innate_Voodoo_Jinx:
            pass
        class C_DOTA_Ability_Tinker_HeatSeekingMissile:
            pass
        class C_DOTA_Ability_Tornado_Tempest:
            pass
        class CDOTA_Ability_Consumable_Hidden:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Brewmaster_4:
            pass
        class C_DOTA_Ability_Special_Bonus_All_Stats_4:
            pass
        class CDOTA_Modifier_Item_Psychic_Headband:
            pass
        class CDOTA_Modifier_Item_Necronomicon_Mana_Aura:
            pass
        class CDOTA_Modifier_Item_AbyssalBlade:
            pass
        class CDOTA_Modifier_ArcWarden_TempestDouble:
            pass
        class CDOTA_Modifier_Oracle_DivinersDeck_TheFool_Next:
            pass
        class CDOTA_Modifier_Oracle_FalsePromiseTimer:
            pass
        class CDOTA_Modifier_Medusa_StoneGaze_Slow:
            pass
        class CDOTA_Modifier_Undying_Tombstone_Death_Trigger:
            pass
        class CDOTA_Modifier_LoneDruid_SpiritBear_VFX:
            pass
        class CDOTA_Modifier_Invoker_IceWall_SlowDebuff:
            pass
        class CDOTA_Modifier_Nian_Attachment_Regrow:
            pass
        class CDOTA_Modifier_Lich_Chain_Frost_Thinker:
            pass
        class C_IngameEvent_WM2016:
            pass
        class C_TextureBasedAnimatable:
            pass
        class C_LightEnvironmentEntity:
            pass
        class C_DOTA_NeutralSpawner:
            pass
        class C_DOTA_Item_Terror_Mask:
            pass
        class C_DOTA_Ability_Hoodwink_Sharpshooter:
            pass
        class C_DOTA_Ability_Terrorblade_Metamorphosis:
            pass
        class CDOTA_Ability_Gyrocopter_Side_Gunner:
            pass
        class C_DOTA_Ability_Spectre_Dispersion:
            pass
        class C_DOTA_Ability_Tiny_Avalanche:
            pass
        class C_DOTA_Ability_SkeletonKing_HellfireBlast:
            pass
        class C_DOTA_Ability_GnollAssassin_EnvenomedWeapon:
            pass
        class CDOTA_Modifier_AghsFort_PlayerTransform:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Skywrath:
            pass
        class C_DOTA_Ability_Special_Bonus_Status_Resistance_15:
            pass
        class C_DOTA_Ability_Special_Bonus_Gold_lvl25_r:
            pass
        class CDOTA_Modifier_Item_Dormant_Curio:
            pass
        class CDOTA_Modifier_Spell_Prism:
            pass
        class CDOTA_Modifier_Item_Pupils_gift:
            pass
        class CDOTA_Modifier_Item_WraithPact_Thinker:
            pass
        class CDOTA_Modifier_Item_RodOfAtos:
            pass
        class CDOTA_Modifier_Kez_RavensVeil_Thinker:
            pass
        class CDOTA_Modifier_MonkeyKing_ArcToGround:
            pass
        class CDOTA_Modifier_DoomBringer_InfernalBlade:
            pass
        class CDOTA_Modifier_Viper_Nose_Dive:
            pass
        class CDOTA_Modifier_Rune_Regen:
            pass
        class CDOTA_Modifier_Bonus_Armor:
            pass
        class CDOTA_Survivors_UnitEntity:
            pass
        class C_DOTA_Item_Cloak_Of_Flames:
            pass
        class C_DOTA_Item_Recipe_Timeless_Relic:
            pass
        class C_DOTA_Item_Boots_Of_Bearing:
            pass
        class C_DOTA_Item_Recipe_Vladmir:
            pass
        class C_DOTA_Item_Recipe_Bracer:
            pass
        class C_DOTA_Ability_Marci_Special_Delivery:
            pass
        class C_DOTA_Ability_TrollWarlord_SwitchStance:
            pass
        class CDOTA_Ability_Viper_Nethertoxin:
            pass
        class C_DOTA_Item_Lua:
            pass
        class C_DOTA_Ability_PolarFurbolgUrsaWarrior_ThunderClap:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Enigma_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Evasion_30:
            pass
        class C_DOTA_Ability_Special_Bonus_Strength_13:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_Regen_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Block_15:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_150:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_1000:
            pass
        class CMatchTrackedStatsEntity:
            pass
        class C_DOTAReflectionSkybox:
            pass
        class CDOTA_Modifier_Item_Bloodstone_Aura:
            pass
        class CDOTA_Modifier_Largo_AmphibianRhapsody_Ambient_Buff:
            pass
        class CDOTA_Modifier_Shadow_Demon_Shadow_Servant:
            pass
        class CDOTA_Modifier_FacelessVoid_TimeDilation_Distortion_Aura_Applicator:
            pass
        class CDOTA_Modifier_Lion_ManaDrain:
            pass
        class CDOTA_Modifier_Tower_Truesight_Aura:
            pass
        class DestructiblePartDamageRequestAPI:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Unit_Hero_Grimstroke:
            pass
        class C_DOTA_BaseNPC_Creep_Siege:
            pass
        class C_DOTA_Item_Outworld_Staff:
            pass
        class C_DOTA_Item_HelmOfTheOverlord:
            pass
        class C_DOTA_Item_Recipe_Shivas_Guard:
            pass
        class C_DOTA_Ability_Kez_GrapplingClaw:
            pass
        class C_DOTA_Ability_Grimstroke_Ink_Over:
            pass
        class C_DOTA_Ability_Gyrocopter_Rocket_Barrage:
            pass
        class C_DOTA_Ability_Dazzle_Shallow_Grave:
            pass
        class C_DOTA_Ability_TemplarAssassin_Trap:
            pass
        class C_DOTA_Ability_AlphaWolf_CommandAura:
            pass
        class CDOTA_Ability_MudGolem_HurlBoulder:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Necrophos:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Phantom_Lancer_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_100:
            pass
        class C_DOTA_Ability_Special_Bonus_Intelligence_75:
            pass
        class C_FoWBlockerEntity:
            pass
        class CDOTA_Modifier_Foragers_Kit_Thinker:
            pass
        class CDOTA_Modifier_Item_Helm_Of_The_Undying:
            pass
        class CDOTA_Modifier_Muerta_Ofrenda_Invulnerable:
            pass
        class CDOTA_Modifier_VoidSpirit_AetherRemnantUnit_Truesight:
            pass
        class CDOTA_Modifier_NagaSiren_RipTideCounter:
            pass
        class CDOTA_Modifier_QueenOfPain_SonicWave_Delay:
            pass
        class CDOTA_Modifier_ShadowShaman_SerpentWard:
            pass
        class CDOTA_Modifier_Mirana_Nightveil:
            pass
        class CLogicRelayAPI:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_BaseNPC_LotusPool:
            pass
        class C_TriggerPhysics:
            pass
        class C_PropDoorRotating:
            pass
        class CDOTA_Item_Recipe_Eagle_Eye:
            pass
        class C_DOTA_Item_BlinkDagger:
            pass
        class C_DOTA_Ability_Phoenix_Dying_Light:
            pass
        class C_DOTA_Ability_Shredder_ReturnChakramAlias_shredder_return_chakram_2:
            pass
        class C_DOTA_Ability_TrollWarlord_Whirling_Axes_Melee:
            pass
        class C_DOTA_Ability_Slark_Pounce:
            pass
        class CDOTA_Ability_TemplarAssassin_InnerPeace:
            pass
        class C_DOTA_Ability_StormSpirit_Overload:
            pass
        class CDOTA_Modifier_Creep_Irresolute:
            pass
        class CDOTA_Modifier_Launchpad:
            pass
        class CInfoWorldLayer:
            pass
        class CDOTA_Modifier_AghsFort_TrapRoom_MeatHook:
            pass
        class CDOTA_Modifier_AghsFort_TorrentEffectPotion_Torrent:
            pass
        class C_DOTA_Item_UpgradedBarricade:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Warlock_10:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Axe:
            pass
        class C_DOTA_Ability_Special_Bonus_Movement_Speed_75:
            pass
        class CDOTA_Modifier_Item_Dynamite_Jacket:
            pass
        class CDOTA_Modifier_Item_Crown:
            pass
        class CDOTA_Modifier_Item_Ancient_Janggo:
            pass
        class CDOTA_Modifier_Item_Pipe_Debuff:
            pass
        class CDOTA_Modifier_Item_Radiance_Debuff:
            pass
        class CDOTA_Modifier_Item_MaskOfDeath:
            pass
        class CDOTA_Modifier_Disruptor_KineticField_TouchSlow:
            pass
        class CDOTA_Modifier_Lycan_SummonWolves_CriticalStrike:
            pass
        class CDOTA_Modifier_Dazzle_Bad_Juju_Manacost:
            pass
        class CDOTA_Modifier_Pugna_DrainSouls:
            pass
        class CDOTA_Modifier_Lion_Arcana_Kill_Effect:
            pass
        class CDOTA_Modifier_Morphling_Replicate:
            pass
        class CDOTA_Modifier_Lina_LagunaBlade_Line:
            pass
        class CDOTA_Modifier_Razor_StormSurge_Slow:
            pass
        class CDOTA_Modifier_Pudge_Rot:
            pass
        class CBodyComponentBaseModelEntity:
            pass
        class C_DOTA_Ability_Kez_RavensVeil:
            pass
        class CDOTA_Ability_Marci_Companion_Run:
            pass
        class C_DOTA_Ability_Dawnbreaker_Unbreakable:
            pass
        class C_DOTA_Ability_Gyrocopter_Afterburner:
            pass
        class CDOTA_Ability_Night_Stalker_MidnightFeast:
            pass
        class CDOTA_Ability_Tinker_DeployTurrets:
            pass
        class C_DOTA_Ability_Frogmen_Riverborn_Aura:
            pass
        class CDOTA_Ability_AghsFort_Morphling_Waveform:
            pass
        class CDOTA_Ability_Seasonal_TI9_Monkey:
            pass
        class CDOTA_Modifier_Special_Bonus_Attack_Damage:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Ember_Spirit_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_6:
            pass
        class C_DOTA_Ability_Special_Bonus_Cast_Range_300:
            pass
        class CDOTA_Modifier_Item_SpecialistsArray_ProcDamage:
            pass
        class CDOTA_Modifier_Item_Penta_Edged_Sword:
            pass
        class CDOTA_Modifier_Ringmaster_FunhouseMirror_Phase:
            pass
        class CDOTA_Modifier_Primal_Beast_Innate_Slow_Resistance_Per_Time:
            pass
        class CDOTA_Modifier_Visage_Amor_Reduction:
            pass
        class CDOTA_Modifier_KeeperOfTheLight_BlindingLight_Thinker:
            pass
        class CDOTA_Modifier_Lycan_Apex_Predator:
            pass
        class CDOTA_Modifier_DoomBringer_ScorchedEarthEffect:
            pass
        class CDOTA_Modifier_Death_Prophet_Attack_Scepter:
            pass
        class CDOTA_Modifier_Tinker_Repair_Bots:
            pass
        class CDOTA_Modifier_StormSpirit_Overload_Attack_Speed_Buff:
            pass
        class CDOTA_Modifier_Sven_GodsStrength:
            pass
        class CDOTA_Modifier_Bane_FiendsGrip_Self:
            pass
        class CDOTA_Modifier_No_Invisibility:
            pass
        class C_DOTA_Unit_Hero_Ogre_Magi:
            pass
        class C_BaseTrigger:
            pass
        class C_DOTA_Ability_Techies_Suicide:
            pass
        class C_DOTA_Ability_Nian_Sigils:
            pass
        class C_DOTA_Ability_Lina_Flame_Cloak:
            pass
        class C_DOTA_Ability_Mirana_SolarFlare:
            pass
        class C_DOTA_Ability_SandKing_SandStorm:
            pass
        class CDOTA_Modifier_Frogmen_ArmOfTheDeep_Stun:
            pass
        class CDOTA_Ability_AghsFort_Arcanist_Potion:
            pass
        class CDOTA_Modifier_Special_Bonus_Cast_Range:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_Regen_30:
            pass
        class CPulsePanoramaFuncs:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Modifier_Item_Disperser:
            pass
        class CDOTA_Modifier_Item_MeteorHammer:
            pass
        class CDOTA_Modifier_Dawnbreaker_BreakOfDawn:
            pass
        class CDOTA_Modifier_MonkeyKing_UnperchedStunned:
            pass
        class CDOTA_Modifier_Techies_Squees_Scope:
            pass
        class CDOTA_Modifier_Techies_StickyBomb_Countdown:
            pass
        class CDOTA_Modifier_Spectre_SpectralDaggerPathPhased:
            pass
        class CDOTA_Modifier_BountyHunter_TrackGold:
            pass
        class CDOTA_Modifier_Broodmother_SpinWeb_NoTreeWalking:
            pass
        class CDOTA_Modifier_Life_Stealer_Ghoul_Frenzy_Slow:
            pass
        class CDOTA_Modifier_Death_Prophet_MourningRitual:
            pass
        class C_DOTA_Unit_Hero_PhantomAssassin:
            pass
        class CDOTA_BaseNPC_Seasonal_Penguin:
            pass
        class C_DynamicPropClientFadeOut:
            pass
        class FilterDamageType:
            pass
        class C_DOTA_Ability_LoneDruid_SavageRoar:
            pass
        class C_DOTA_Ability_Sniper_TakeAim:
            pass
        class CDOTA_Ability_Bloodseeker_Rupture:
            pass
        class C_DOTA_Ability_Sven_Vanquisher:
            pass
        class C_DOTA_Ability_DeathProphet_Exorcism:
            pass
        class CDOTA_Ability_AghsFort_Creature_Impale:
            pass
        class CDOTA_Modifier_Special_Bonus_Movement_Speed:
            pass
        class C_DOTA_Ability_Special_Bonus_Lifesteal_40:
            pass
        class C_DOTA_Ability_Special_Bonus_Agility_40:
            pass
        class CDOTA_Modifier_Item_Force_Field_Bonus_Aura:
            pass
        class CDOTA_Modifier_Item_AeonDisk_Buff:
            pass
        class CDOTA_Modifier_Item_Sphere:
            pass
        class CDOTA_Modifier_Snapfire_LilShredder_Debuff:
            pass
        class CDOTA_Modifier_Mars_Scepter_Damage:
            pass
        class CDOTA_Modifier_Abaddon_Frostmourne_Buff:
            pass
        class CDOTA_Modifier_Disruptor_Thunder_Strike:
            pass
        class CDOTA_Modifier_Invoker_IceWall_AllyBuffAura:
            pass
        class CDOTA_Modifier_PhantomLancer_PhantomEdge_Debuff:
            pass
        class CDOTA_Modifier_Mirana_SelemenesFaithful:
            pass
        class CAttributeList:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CPulseCell_Inflow_Wait:
            pass
        class C_DOTA_Unit_Scout:
            pass
        class CDOTA_Item_Hurricane_Pike:
            pass
        class CDOTA_Ability_Techies_ReactiveTazer:
            pass
        class C_DOTA_Ability_Furion_WrathOfNature:
            pass
        class C_DOTA_Ability_Nian_Eruption:
            pass
        class C_DOTA_Item_Miniboss_Minion_Summoner:
            pass
        class C_DOTA_Ability_Windrunner_Powershot:
            pass
        class CDOTA_Modifier_BigThunderLizard_Frenzy:
            pass
        class C_DOTA_Ability_SatyrTrickster_Purge:
            pass
        class CFilterProximity:
            pass
        class CDOTA_Ability_Seasonal_TI11_BubbleGun:
            pass
        class CDOTA_Modifier_Special_Bonus_Movement_Speed_Percentage:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Ancient_Apparition_2:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Range_175:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Base_Damage_25:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_Regen_125:
            pass
        class CTeamTrackedStatsEntity:
            pass
        class CDOTA_Modifier_Hoodwink_AcornShot_ArmorCorruption:
            pass
        class CDOTA_Modifier_Treant_NaturesGrasp_Creation_Thinker:
            pass
        class CDOTA_Modifier_Meepo_Ransack:
            pass
        class CDOTA_Modifier_LoneDruid_SpiritBear_Fetch_Self:
            pass
        class CDOTA_Modifier_Roshan_SpellBlock:
            pass
        class CDOTA_Modifier_FountainPassive:
            pass
        class CDOTA_Modifier_Enigma_Event_Horizon_Aura:
            pass
        class CEffectData:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Unit_Hero_EarthSpirit:
            pass
        class C_ParticleSystem:
            pass
        class CDOTA_Ability_Kez_Echo_Slash:
            pass
        class C_DOTA_Ability_Primal_Companion:
            pass
        class CDOTA_Ability_Beastmaster_PrimalRoar:
            pass
        class CDOTA_Ability_Seasonal_TI11_Scissors:
            pass
        class C_DOTA_Ability_Morty_Hop_Launch:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Windranger_3:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Storm_Spirit_7:
            pass
        class C_DOTA_Ability_Special_Bonus_Movement_Speed_40:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_Regen_175:
            pass
        class C_DOTA_Ability_Special_Bonus_TrueStrike:
            pass
        class CDOTA_Modifier_Item_SuperOverwhelming_Blink:
            pass
        class CDOTA_Modifier_Force_Boots:
            pass
        class CDOTA_Modifier_Marci_Lunge_Debuff:
            pass
        class CDOTA_Modifier_Pangolier_Gyroshell_Ricochet:
            pass
        class CDOTA_Modifier_Huskar_Berserkers_Blood_Aura:
            pass
        class CDOTA_Modifier_Enchantress_Impetus:
            pass
        class CDOTA_Modifier_Enchantress_NaturesAttendants:
            pass
        class CDOTA_Modifier_Omniknight_GuardianAngel_Aura:
            pass
        class CDOTA_Modifier_Clinkz_BurningBarrage:
            pass
        class CDOTA_Modifier_Dark_Seer_Quick_Wit:
            pass
        class CDOTA_Modifier_Furion_Natures_Profit:
            pass
        class CDOTA_Modifier_Viper_CorrosiveSkin_Slow:
            pass
        class CDOTA_Modifier_FacelessVoid_TimeLock_ForceProc:
            pass
        class CDOTA_Modifier_QueenOfPain_ScreamOfPain_Fear:
            pass
        class CDOTA_Modifier_Nian_Invulnerable:
            pass
        class CDOTA_Modifier_Miniboss_Radiance_Debuff:
            pass
        class CDOTA_Modifier_Enigma_Innate_EventHorizonAura:
            pass
        class CDOTA_Modifier_Tiny_TossTree_Slow:
            pass
        class CPulseCell_Outflow_CycleShuffled:
            pass
        class CDOTA_Unit_Hero_ArcWarden:
            pass
        class C_DOTA_Unit_Hero_Juggernaut:
            pass
        class C_DOTA_Item_Cornucopia:
            pass
        class CDOTA_Item_RiverPainter7:
            pass
        class C_DOTA_Item_HelmOfTheDominator_2:
            pass
        class C_DOTA_Item_OblivionStaff:
            pass
        class C_DOTA_Item_MysticStaff:
            pass
        class C_DOTA_Ability_Ogre_Magi_Multicast:
            pass
        class C_DOTA_Ability_SpiritBreaker_HerdMentality:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Phantom_Lancer_2:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Pangolier_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Amplify_18:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_600:
            pass
        class CDOTA_Modifier_Terrorblade_Reflection_Invulnerability:
            pass
        class CDOTA_Modifier_Slark_ShadowDance_PassiveRegen:
            pass
        class CDOTA_Modifier_Disruptor_KineticFieldThinker:
            pass
        class CDOTA_Modifier_Undying_SoulRip_Share_Strength:
            pass
        class CDOTA_Modifier_Alchemist_ChemicalRage:
            pass
        class CDOTA_Modifier_Rattletrap_Jetpack_Tracker:
            pass
        class CDOTA_Modifier_Warlock_Imp_AutoAttack:
            pass
        class CDOTA_Modifier_Earthshaker_Arcana_Kill_Effect:
            pass
        class CDOTA_Modifier_StormSpirit_Overload:
            pass
        class CDOTA_Modifier_Item_Mask_Crit_Lifesteal_Consumed:
            pass
        class CDOTA_Modifier_TempAttackSpeedBoosted:
            pass
        class C_DOTA_Unit_Hero_CrystalMaiden:
            pass
        class C_FuncMover:
            pass
        class C_DOTA_Item_RuneSpawner_Bounty:
            pass
        class C_DOTA_Item_Enhancement_Timelss:
            pass
        class C_DOTA_Item_Recipe_SangeAndYasha:
            pass
        class C_DOTA_Ability_Oracle_Clairvoyant_Cure:
            pass
        class CDOTA_Ability_Shadow_Demon_Menace:
            pass
        class C_DOTA_Ability_Clinkz_DeathPact:
            pass
        class C_DOTA_Ability_Shadow_Shaman_Voodoo_Hands:
            pass
        class C_DOTA_Ability_Puck_PhaseShift:
            pass
        class C_DOTA_Ability_Morphling_Waveform:
            pass
        class CDOTA_Modifier_Legion_Commander_Duel:
            pass
        class CDOTA_Ability_Seasonal_Summon_TI9_Balloon:
            pass
        class CDOTA_Modifier_Special_Bonus_Gold:
            pass
        class C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_14:
            pass
        class C_DOTA_Ability_Special_Bonus_Armor_12:
            pass
        class C_DOTA_Ability_Special_Bonus_Mana_Reduction_9:
            pass
        class CDOTA_Modifier_DarkWillow_Terrorize_Thinker:
            pass
        class CDOTA_Modifier_NagaSiren_SongOfTheSiren:
            pass
        class CDOTA_Modifier_Brewmaster_Pulverize:
            pass
        class CDOTA_Modifier_SpiritBreaker_EmpoweringHaste:
            pass
        class CDOTA_Modifier_Clinkz_Tar_Bomb_SearingArrows:
            pass
        class CLightComponent:
            pass
        class CDOTA_Ability_Grimstroke_DarkArtistry:
            pass
        class C_DOTA_Ability_Oracle_DivinersDeck:
            pass
        class C_DOTA_Ability_Oracle_RainOfDestiny:
            pass
        class C_DOTA_Ability_Spectre_SpectralDagger:
            pass
        class C_DOTA_Ability_Ursa_Enrage:
            pass
        class C_DOTA_Ability_PhantomLancer_Doppelwalk:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Snapfire_8:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Snapfire_6:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_70:
            pass
        class C_DOTA_Ability_Special_Bonus_Cast_Range_250:
            pass
        class CDOTAPlayer_CameraServices:
            pass
        class CDOTA_Modifier_DarkWillow_BrambleMaze_Thinker:
            pass
        class CDOTA_Modifier_Tusk_FrozenSigil:
            pass
        class CDOTA_Modifier_NagaSiren_RipTide:
            pass
        class CDOTA_Modifier_Shadow_Demon_Soul_Catcher_Illusion:
            pass
        class CDOTA_Modifier_Earthshaker_Fissure_Stun:
            pass
        class CDOTA_Modifier_Nevermore_FeastOfSouls_Collection:
            pass
        class CDOTA_Modifier_Item_Mask_Crit_Lifesteal:
            pass
        class CDOTA_Modifier_PersistentInvisibility:
            pass
        class C_IngameEvent_DotaPrime:
            pass
        class C_DOTA_Item_WizardHat:
            pass
        class C_DOTA_Ability_Dawnbreaker_Land:
            pass
        class C_DOTA_Ability_NagaSiren_SongOfTheSiren_Cancel:
            pass
        class C_DOTA_Ability_Ogre_Magi_School:
            pass
        class C_DOTA_Ability_Faceless_Void_Innate_Time_Walk_Backtrack_Duration:
            pass
        class CDOTA_Modifier_ContextualTips:
            pass
        class C_EnvCubemap:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Witch_Doctor_2:
            pass
        class CDOTA_Modifier_Item_FlayersBota:
            pass
        class CDOTA_Modifier_Item_Rattlecage_Slow:
            pass
        class CDOTA_Modifier_Psychic_Headband:
            pass
        class CDOTA_Modifier_Item_ConsecratedWraps_Speed_Buff:
            pass
        class CDOTA_Modifier_Wisp_Spirits_Slow:
            pass
        class CDOTA_Modifier_Chen_HolyPersuasion_Intrinsic:
            pass
        class CDOTA_Modifier_Life_Stealer_Open_Wounds_Attack_Speed_Bonus:
            pass
        class CDOTA_Modifier_Enigma_Black_Hole_Pull_Scepter:
            pass
        class CDOTA_Modifier_Antimage_DampenMagic_Aura:
            pass
        class CDOTA_Modifier_BackdoorProtectionInBase:
            pass
        class CBodyComponent:
            pass
        class CPulseCell_Inflow_Method:
            pass
        class C_DOTA_Wisp_Spirit:
            pass
        class C_DOTA_Unit_Hero_Nyx_Assassin:
            pass
        class C_DOTA_MinibossSpawner:
            pass
        class C_DOTA_Item_PocketTower:
            pass
        class C_DOTA_Item_MonkeyKingBar:
            pass
        class C_DOTA_Ability_Phoenix_FireSpirits:
            pass
        class C_DOTA_Ability_EmberSpirit_Activate_FireRemnant:
            pass
        class CDOTA_Ability_KeeperOfTheLight_SpiritForm:
            pass
        class CDOTA_Ability_DarkSeer_IonShell:
            pass
        class C_DOTA_Ability_Earthshaker_EchoSlam:
            pass
        class CDOTA_Modifier_HillTroll_RallyAura:
            pass
        class CDOTA_Modifier_Aghsfort_Pugna_Grandmaster_NetherWard:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Omniknight_7:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Ancient_Apparition_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Cast_Range_60:
            pass
        class C_DOTA_Ability_Special_Bonus_Agility_And_Intelligence_6:
            pass
        class CDOTA_Modifier_Item_Essence_Ring_Active:
            pass
        class CDOTA_Modifier_Item_Princes_Knife:
            pass
        class CDOTA_Modifier_MonkeyKing_BoundlessStrike_ShardMovement:
            pass
        class CDOTA_Modifier_Terrorblade_Metamorphosis_Transform_Aura:
            pass
        class CDOTA_Modifier_AbyssalUnderlord_PitOfMalice_Slow:
            pass
        class CDOTA_Modifier_Tusk_Drinking_Buddies_Pull:
            pass
        class CDOTA_Modifier_Slark_Pounce_ChargeCounter:
            pass
        class CDOTA_Modifier_Undying_FleshGolem_Intrinsic:
            pass
        class CDOTA_Modifier_Meepo_MegaMeepo:
            pass
        class CDOTA_Modifier_Shadow_Demon_Disseminate:
            pass
        class CDOTA_Modifier_Invoker_EMP_Pull:
            pass
        class CDOTA_Modifier_Viper_PoisonAttack:
            pass
        class CDOTA_Modifier_Miniboss_Minion_Deflecting_Shield_Aura:
            pass
        class CDOTA_Modifier_Kunkka_Tidebringer:
            pass
        class CDOTA_Modifier_Item_BotChallenge_MeteorStaff_Land:
            pass
        class CDOTA_Modifier_MonkeyKing_BouncePerch:
            pass
        class C_DOTA_UnitInventory:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_BaseCombatCharacter:
            pass
        class C_DOTATeam:
            pass
        class C_DOTA_Item_SplintMail:
            pass
        class C_DOTA_Ability_Rubick_FadeBolt:
            pass
        class C_DOTA_Ability_Omniknight_Repel:
            pass
        class C_DOTA_Ability_Morphling_Flow:
            pass
        class CDOTA_Modifer_Furbolg_Enrage_AttackSpeed:
            pass
        class CDOTA_Modifier_PineCone_ShieldBash_Crit:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Undying_5:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Bloodseeker_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Sniper_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Evasion_10:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Speed_15:
            pass
        class CDOTA_Modifier_Item_Ethereal_Blade:
            pass
        class CDOTA_Modifier_EarthSpirit_Petrify:
            pass
        class CDOTA_Modifier_EmberSpirit_FireRemnant_ChargeCounter:
            pass
        class CDOTA_Modifier_Tusk_FrozenSigil_Aura:
            pass
        class CDOTA_Modifier_Bristleback_Active_ConicalQuillSpray:
            pass
        class CDOTA_Modifier_Shredder_Flamethrower_Damage:
            pass
        class CDOTA_Modifier_Invoker_ChaosMeteor_Land:
            pass
        class CDOTA_Modifier_Invoker_Instance:
            pass
        class CDOTA_Modifier_Broodmother_InsatiableHunger_Aura:
            pass
        class CDOTA_Modifier_Broodmother_SpawnSpiderlingsShard:
            pass
        class CDOTA_Modifier_Omniknight_Hammer_Of_Purity_Bomb:
            pass
        class CDOTA_Modifier_Dazzle_Poison_Touch_Self:
            pass
        class CDOTA_Modifier_JumpBootsIntrinsic:
            pass
        class CDOTA_Modifier_Mirana_Starfall_Thinker:
            pass
        class CDOTA_Modifier_Nevermore_FeastOfSouls_Linger:
            pass
        class CDOTA_Modifier_HallOfFame_Glow:
            pass
        class CRoshanPhaseInfo:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CGlowProperty:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Item_Enhancement_Dominant:
            pass
        class C_DOTA_Item_Recipe_Wind_Waker:
            pass
        class C_DOTA_Item_Orb_of_Venom:
            pass
        class C_DOTA_Item_Recipe_Dagon4:
            pass
        class C_DOTA_Ability_BountyHunter_Jinada:
            pass
        class C_DOTA_Ability_Necrolyte_GhostShroud:
            pass
        class C_DOTA_Ability_SkeletonKing_Reincarnation:
            pass
        class CDOTA_Modifier_Mutation_CreateTombstone_Team_Aura:
            pass
        class CDOTA_Ability_Capture:
            pass
        class C_PointClientUIDialog:
            pass
        class C_DotaQuest:
            pass
        class C_DOTA_Ability_AghsFort_ShadowShaman_Shackles:
            pass
        class CDOTA_Ability_Ascension_Bulwark:
            pass
        class CDOTA_Modifier_Muerta_SpectralSlugEthereal:
            pass
        class CDOTA_Modifier_Snapfire_Magma_Thinker:
            pass
        class CDOTA_Modifier_Mars_ArenaOfBlood_VisionObstruction:
            pass
        class CDOTA_Modifier_Terrorblade_Fear:
            pass
        class CDOTA_Modifier_Terrorblade_ConjureImage:
            pass
        class CDOTA_Modifier_Undying_FleshGolem_PlagueAura:
            pass
        class CDOTA_Modifier_Furion_WrathOfNature_Buff:
            pass
        class CPulseCell_BaseValue:
            pass
        class C_DOTA_Item_Seer_Stone:
            pass
        class CDOTA_Item_RiverPainter4:
            pass
        class C_DOTA_Item_RefresherOrb:
            pass
        class C_DOTA_Item_Broadsword:
            pass
        class C_DOTA_Ability_Disruptor_Glimpse:
            pass
        class C_DOTA_Ability_Meepo_Petrify:
            pass
        class C_DOTA_Ability_Brewmaster_CinderBrew:
            pass
        class CDOTA_Modifier_Creep_Siege:
            pass
        class C_DOTA_Ability_OgreBruiser_OgreSmash:
            pass
        class CCitadelSoundOpvarSetOBB:
            pass
        class CDOTA_Modifier_AghsFort_TreantMiniboss_NaturesGuise:
            pass
        class CDOTA_Ability_AghsFort_Tower_BlastWave:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Underlord_5:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lycan_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Bristleback:
            pass
        class C_DOTA_Ability_Special_Bonus_Evasion_20:
            pass
        class C_DOTA_Ability_Special_Bonus_Respawn_Reduction_20:
            pass
        class CDOTA_Modifier_Item_Doubloon:
            pass
        class CDOTA_Modifier_Item_Terror_Mask:
            pass
        class CDOTA_Modifier_Mars_ArenaOfBlood_Thinker:
            pass
        class CDOTA_Modifier_Slark_Fish_Bait_Post:
            pass
        class CDOTA_Modifier_Alchemist_Berserk_Potion:
            pass
        class CDOTA_Modifier_Spectre_Reality:
            pass
        class CDOTA_Modifier_DeathProphet_CryptSwarm_Slow:
            pass
        class CDOTA_Modifier_StormSpirit_Galvanized:
            pass
        class CDOTA_Modifier_Rune_Illusion:
            pass
        class CPlayer_WaterServices:
            pass
        class CPulseCell_BooleanSwitchState:
            pass
        class C_DOTA_Unit_Hero_Sniper:
            pass
        class C_DOTA_Item_Enhancement_Vast:
            pass
        class C_DOTA_Item_Consecrated_Wraps:
            pass
        class C_DOTA_Item_OrchidMalevolence:
            pass
        class C_DOTA_Item_Shawl:
            pass
        class C_DOTA_Ability_EarthSpirit_Magnetize:
            pass
        class C_DOTA_Ability_Rubick_Arcane_Supremacy:
            pass
        class C_DOTA_Ability_Huskar_Inner_Vitality:
            pass
        class C_DOTA_Ability_PhantomAssassin_Stifling_Dagger:
            pass
        class CDOTA_Ability_Miniboss_Minion_Deflecting_Shield:
            pass
        class C_DOTA_Ability_DataDriven:
            pass
        class CDOTA_Modifier_Furbolg_Enrage_AttackSpeed_OnDeath:
            pass
        class C_DOTA_Ability_AghsFort_Shadow_Demon_Shadow_Poison:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Dark_Seer_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Evasion_16:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_15:
            pass
        class CDOTA_Modifier_Orb_Of_Revelations_Reveal:
            pass
        class CDOTA_Modifier_Necronomicon_Warrior_LastWill:
            pass
        class CDOTA_Modifier_Item_Necronomicon_3:
            pass
        class CDOTA_Modifier_Item_Shivas_Guard:
            pass
        class CDOTA_Modifier_AbyssalUnderlord_Underling_Autoattack:
            pass
        class CDOTA_Modifier_Shredder_ReactiveArmor_Bomb:
            pass
        class CDOTA_Modifier_TrollWarlord_BerserkersRage_Ensnare:
            pass
        class CDOTA_Modifier_Invoker_IceWall_Thinker:
            pass
        class CDOTA_Modifier_Broodmother_PoisonSting:
            pass
        class CDOTA_Modifier_Beastmaster_SummonRaptor_Summoning:
            pass
        class CDOTA_Modifier_Ursa_Earthshock:
            pass
        class CDOTA_Modifier_Tinker_Defensive_Matrix:
            pass
        class CDOTA_Modifier_Mirana_MoonlightShadow:
            pass
        class C_IngameEvent_DotaPlus:
            pass
        class CDOTA_PlayerChallengeInfo:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class VPhysicsCollisionAttribute_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Unit_Hero_DarkWillow:
            pass
        class C_DOTA_Unit_Hero_Pangolier:
            pass
        class C_DOTA_BaseNPC_Tusk_Sigil:
            pass
        class C_DOTA_Unit_Hero_DarkSeer:
            pass
        class C_DOTA_Unit_Hero_Pugna:
            pass
        class C_DOTA_NPC_Lich_Ice_Spire:
            pass
        class C_PlayerCosmeticPropClientside:
            pass
        class C_DynamicPropAlias_dynamic_prop:
            pass
        class CDOTA_Item_Rabbits_Foot:
            pass
        class C_DOTA_Item_Mechanical_Arm:
            pass
        class CDOTA_Item_Recipe_Crimson_Guard:
            pass
        class C_DOTA_Ability_Brewmaster_SpellImmunity:
            pass
        class C_DOTA_Ability_Tiny_Insurmountable:
            pass
        class C_DOTA_Ability_Juggernaut_Duelist:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_DarkWillow_1:
            pass
        class CDOTA_Modifier_Special_Bonus_Evasion:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_800:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_225:
            pass
        class C_DOTACheers:
            pass
        class CDOTA_Modifier_Dragon_Scale_Burn:
            pass
        class CDOTA_Modifier_Item_Desolator_2:
            pass
        class CDOTA_Modifier_Tome_Of_Knowledge_Consumed:
            pass
        class CDOTA_Modifier_GlimmerCape_Fade:
            pass
        class CDOTA_Modifier_Tenderizer_Weaken:
            pass
        class CDOTA_Modifier_Item_ObserverWard:
            pass
        class CDOTA_Modifier_Snapfire_GobbleUp_BellyHasUnit:
            pass
        class CDOTA_Modifier_Mars_Dauntless_Effect:
            pass
        class CDOTA_Modifier_Tusk_WalrusKick_AirTime:
            pass
        class CDOTA_Modifier_Visage_Silent_As_The_Grave_Bonus:
            pass
        class CDOTA_Modifier_LoneDruid_SavageRoar:
            pass
        class CDOTA_Modifier_Gyrocopter_Flak_Cannon:
            pass
        class CDOTA_Modifier_Nian_WhirlpoolThinker:
            pass
        class CDOTA_Modifier_Lich_FrostShield:
            pass
        class CDOTA_Modifier_Morphling_Morph_Str:
            pass
        class CDOTA_Modifier_PhantomLancer_PhantomEdge:
            pass
        class CDOTA_Modifier_VengefulSpirit_Command_Negative_Aura_Effect:
            pass
        class CDOTA_Modifier_DrowRanger_FrostArrows_Slow:
            pass
        class CDOTA_Modifier_Sand_King_Scorpion_Strike:
            pass
        class CDOTA_Modifier_TrueSightFoW:
            pass
        class CDOTA_Unit_Hero_Gyrocopter:
            pass
        class CEnvSoundscapeProxyAlias_snd_soundscape_proxy:
            pass
        class CDOTA_Item_Havoc_Hammer:
            pass
        class C_DOTA_Item_Necronomicon_Level2:
            pass
        class C_DOTA_Item_Recipe_Ring_Of_Basilius:
            pass
        class C_DOTA_Ability_MonkeyKing_QuadrupleTap:
            pass
        class C_DOTA_Ability_Meepo_Megameepo_Fling:
            pass
        class CDOTA_Modifier_BlueDragonspawnSorcerer_Evasion:
            pass
        class C_SceneEntity:
            pass
        class CDOTA_Modifier_PudgeMiniboss_ArmorCorruption:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Drow_Ranger_7:
            pass
        class C_SpeechBubbleManager:
            pass
        class CDOTA_Modifier_Item_SistersShroud:
            pass
        class CDOTA_Modifier_Item_Mirror_Shield:
            pass
        class CDOTA_Modifier_Royale_With_Cheese:
            pass
        class CDOTA_Modifier_Item_Necronomicon_Mana_Aura_3:
            pass
        class CDOTA_Modifier_Item_Assault_Cuirass_Negative_Armor_Aura:
            pass
        class CDOTA_Modifier_Bristleback_BrawlersGrit:
            pass
        class CDOTA_Modifier_Invoker_ChaosMeteor_Burn:
            pass
        class CDOTA_Modifier_Huskar_Inner_Fire_Knockback:
            pass
        class CDOTA_Modifier_Miniboss_Reflect:
            pass
        class CDOTA_Modifier_Windrunner_Windrun_Invis:
            pass
        class CDOTA_Modifier_DrowRanger_Marksmanship:
            pass
        class CDOTA_Modifier_SkeletonKing_BoneGuard_Summon_Thinker:
            pass
        class CIngameEvent_MonsterHunter:
            pass
        class CPulseCell_Inflow_Yield:
            pass
        class CPulseMathlib:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Unit_Hero_Enchantress:
            pass
        class C_DOTA_Unit_Hero_Puck:
            pass
        class CDOTA_BaseNPC_Seasonal_CNY_Balloon:
            pass
        class C_DOTA_BaseNPC_NeutralItemStash:
            pass
        class CDOTA_Artillery_EffectsEntity:
            pass
        class CDOTA_Item_Recipe_Clumsy_Net:
            pass
        class C_DOTA_Item_Recipe_WraithPact:
            pass
        class C_DOTA_Item_Ring_Of_Basilius:
            pass
        class C_DOTA_Item_Recipe_Black_King_Bar:
            pass
        class C_DOTA_Ability_Lycan_SummonWolves_PermanentInvisibility:
            pass
        class C_DOTA_Ability_Beastmaster_InnerBeast:
            pass
        class C_DOTA_Ability_Razor_StaticLink:
            pass
        class C_DOTA_Ability_Greevil_Miniboss_Orange_LightStrikeArray:
            pass
        class CDOTA_Ability_Kobold_Disarm:
            pass
        class CDOTA_Modifier_Aghsfort_Reward_HPAura:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Bounty_Hunter_8:
            pass
        class C_DOTA_Ability_Special_Bonus_Night_Vision_800:
            pass
        class C_DOTA_Ability_Special_Bonus_Strength_4:
            pass
        class CDOTA_Modifier_Item_Enhancement_Vital:
            pass
        class CDOTA_Modifier_Item_Spirit_Vessel:
            pass
        class CDOTA_Modifier_Item_Solar_Crest_Armor_Addition:
            pass
        class CDOTA_Modifier_Item_Maelstrom:
            pass
        class CDOTA_Modifier_Item_RobeOfMagi:
            pass
        class CDOTA_Modifier_Ringmaster_TameTheBeasts_Fear:
            pass
        class CDOTA_Modifier_Marci_Special_Delivery:
            pass
        class CDOTA_Modifier_Mars_Spear_Impale:
            pass
        class CDOTA_Modifier_Skywrath_Mage_Concussive_Shot:
            pass
        class CDOTA_Modifier_Visage_SoulAssumption_Delay:
            pass
        class CDOTA_Modifier_LoneDruid_SpiritLink_BearFear:
            pass
        class CDOTA_Modifier_Spirit_Bear_Necessities:
            pass
        class CDOTA_Modifier_Lycan_Shard:
            pass
        class CDOTA_Modifier_DoomBringer_Doom_Intrinsic:
            pass
        class CDOTA_Modifier_Lion_Innate_ToHellAndBack_RespawnBuff:
            pass
        class CDOTA_Modifier_Pudge_FleshHeap_Block:
            pass
        class CDOTA_Modifier_Tutorial_LowAttackPriority:
            pass
        class CIngameEvent_PermanentGrants:
            pass
        class CDOTA_Unit_Templar_Gate:
            pass
        class C_EconEntity:
            pass
        class C_DOTA_Item_RuneSpawner:
            pass
        class C_DOTA_Item_Recipe_Devastator:
            pass
        class C_DOTA_Item_Elven_Tunic:
            pass
        class CDOTA_Ability_Mars_Spear:
            pass
        class C_DOTA_Ability_Shredder_TimberChain:
            pass
        class CDOTA_Modifier_Greevil_Miniboss_Purple_VenomousGale:
            pass
        class CDOTA_Modifier_Greevil_Miniboss_Red_Earthshock:
            pass
        class CDOTA_Modifier_CentaurKhan_EnduranceAura:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Sven_3:
            pass
        class CDOTA_Modifier_Item_Orb_Of_Corrosion:
            pass
        class CDOTA_Modifier_Ringmaster_Decoy_Wheel:
            pass
        class CDOTA_Modifier_Dawnbreaker_Fire_Wreath_Slow:
            pass
        class CDOTA_Modifier_Keeper_of_the_Light_ManaMagnifier_Boost:
            pass
        class CDOTA_Modifier_Brewmaster_FireBrewling_FireStance:
            pass
        class CDOTA_Modifier_Obsidian_Destroyer_Sanity_Eclipse_Counter:
            pass
        class CDOTA_Modifier_Clockwerk_RocketFlare_Thinker:
            pass
        class CDOTA_Modifier_Rattletrap_Cog_Thinker_Self_Bonuses:
            pass
        class CDOTA_Modifier_QueenOfPain_Arcana:
            pass
        class C_BaseAnimatingOverlayController:
            pass
        class CPlayer_UseServices:
            pass
        class C_DOTA_Unit_Hero_Weaver:
            pass
        class C_DOTA_Item_Magnifying_Monocle:
            pass
        class C_DOTA_Item_Recipe_Titan_Sliver:
            pass
        class CDOTA_Item_Recipe_GreaterFamango:
            pass
        class C_DOTA_Item_Recipe_Necronomicon_3:
            pass
        class C_DOTA_Item_Recipe_Sange:
            pass
        class C_DOTA_Ability_Obsidian_Destroyer_SanityEclipse:
            pass
        class C_DOTA_Ability_Slardar_Bash:
            pass
        class C_PointValueRemapper:
            pass
        class C_DotaSubquestTutorialEvent:
            pass
        class CDOTA_Ability_AghsFort_TrapRoom_MeatHook:
            pass
        class CDOTA_Modifier_Aghsfort_Bonus_Pudge_Meat_Hook:
            pass
        class C_DOTA_Ability_Special_Bonus_Agility_100:
            pass
        class CDOTA_Modifier_PrimalBeast_Pulverize_AoeBuff:
            pass
        class CDOTA_Modifier_Legion_Commander_Duel_DamageBoost:
            pass
        class CDOTA_Modifier_Elder_Titan_Tip_The_Scales_Aura:
            pass
        class CDOTA_Modifier_Visage_SummonFamiliars_Recall:
            pass
        class CDOTA_Modifier_Brewmaster_Fear:
            pass
        class CDOTA_Modifier_Clinkz_Infernal_Shred_ArmorPiercing:
            pass
        class CDOTA_Modifier_Lion_ManaDrain_Immunity:
            pass
        class CDOTA_Modifier_Kunkka_Torrent:
            pass
        class CGameSceneNodeHandle:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Unit_Hero_NightStalker:
            pass
        class C_DOTA_NPC_BaseBlocker:
            pass
        class CDOTA_Match3_PieceEntity:
            pass
        class C_DOTA_Item_Aghanims_Shard_Roshan:
            pass
        class C_DOTA_Ability_Kez_BaseAbility:
            pass
        class C_DOTA_Ability_AbyssalUnderlord_DarkRift:
            pass
        class C_DOTA_Ability_SandKing_Epicenter:
            pass
        class CDOTA_Ability_AncientApparition_IceBlast:
            pass
        class CDOTA_Modifier_PineCone_AcornShot_DelayThinker:
            pass
        class CDOTA_Modifier_Special_Bonus_MP_Regen_Amp:
            pass
        class C_DOTA_Ability_Special_Bonus_Movement_Speed_60:
            pass
        class CDOTA_Modifier_Ringmaster_Wheel_Debuff:
            pass
        class CDOTA_Modifier_Tusk_Snowball_Movement:
            pass
        class CDOTA_Modifier_Undying_Tombstone_HP:
            pass
        class CDOTA_Modifier_Brewmaster_DrunkenBrawler_BrewedUp_Hangover:
            pass
        class CDOTA_Modifier_Brewmaster_DrunkenBrawler_Passive:
            pass
        class CDOTA_Modifier_Obsidian_Destroyer_Equilibrium_BonusMana:
            pass
        class CDOTA_Modifier_Broodmother_WebWalk_Thinker:
            pass
        class CDOTA_Modifier_DarkSeer_Normal_Punch_Illusion:
            pass
        class CDOTA_Modifier_ShadowShaman_EtherShock:
            pass
        class CDOTA_Modifier_Lina_Slow_Burn:
            pass
        class CDOTA_Modifier_Earthshaker_EchoSlam_Delay:
            pass
        class CDOTA_Modifier_Mirana_Leap:
            pass
        class CDOTA_Modifier_Nevermore_Presence:
            pass
        class C_DOTACustomGameHeroPickRules:
            pass
        class CPulseCell_Unknown:
            pass
        class C_DOTA_Item_Aghanims_Shard:
            pass
        class CDOTA_Ability_Dawnbreaker_Celestial_Hammer:
            pass
        class CDOTA_Ability_Skywrath_Mage_Ruin_And_Restoration:
            pass
        class C_DOTA_Ability_Shredder_Reactive_Armor:
            pass
        class C_DOTA_Ability_TrollWarlord_BattleTrance:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Disruptor:
            pass
        class C_DOTA_Ability_Omniknight_Martyr:
            pass
        class CDOTA_Ability_Life_Stealer_Empty3:
            pass
        class C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_25:
            pass
        class C_DOTA_Ability_Special_Bonus_Strength_3:
            pass
        class C_DOTA_DataCustomTeam:
            pass
        class CDOTA_Modifier_Item_TranquilBoots:
            pass
        class CDOTA_Modifier_Bristleback_Warpath_Active:
            pass
        class CDOTA_Modifier_Magnataur_ReversePolarity_Stun:
            pass
        class CDOTA_Modifier_Tiny_Toss:
            pass
        class CDOTA_Modifier_Morphling_ScepterStatsDrain_Agility_Buff:
            pass
        class CDOTA_Modifier_Morphling_ScepterStatsDrain_Strength_Debuff:
            pass
        class CDOTA_Modifier_Bloodseeker_Thirst_Active:
            pass
        class C_IngameEvent_FV2018:
            pass
        class C_DOTA_Unit_Brewmaster_PrimalFire:
            pass
        class CDOTA_Item_Conjurers_Catalyst:
            pass
        class CDOTA_Ability_Hoodwink_MistwoodsWayfarer:
            pass
        class C_DOTA_Ability_Snapfire_Buckshot:
            pass
        class CDOTA_Ability_Nyx_Assassin_ManaBurn:
            pass
        class C_DOTA_Ability_Nevermore_Requiem:
            pass
        class CDOTA_Modifier_AghsFort_TreantMiniboss_NaturesGuise_Root:
            pass
        class CDOTA_Modifier_PineCone_AcornShot_Slow:
            pass
        class CDOTA_Ability_Aghsfort_Reward_ArmorAura:
            pass
        class CDOTA_Ability_Seasonal_Summon_Penguin:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Broodmother_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_20:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_Regen_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Gold_Ability_Draft:
            pass
        class CDOTA_Modifier_Item_MartyrsPlate_Effect:
            pass
        class CDOTA_Modifier_Item_Stormcrafter:
            pass
        class CDOTA_Modifier_Item_Spirit_Vessel_Heal:
            pass
        class CDOTA_Modifier_Item_Cyclone:
            pass
        class CDOTA_Modifier_Item_BeltOfStrength:
            pass
        class CDOTA_Modifier_Muerta_PierceTheVeil_SpellAmpBoost:
            pass
        class CDOTA_Modifier_DarkWillow_Bedlam:
            pass
        class CDOTA_Modifier_MonkeyKing_SpringSlow:
            pass
        class CDOTA_Modifier_Phoenix_IcarusDiveBurn:
            pass
        class CDOTA_Modifier_Centaur_Return_Bonus_Damage:
            pass
        class CDOTA_Modifier_SpiritBreaker_BullRush:
            pass
        class CDOTA_Modifier_Rattletrap_JetPack:
            pass
        class CDOTA_Modifier_Special_Bonus_Unique_Beastmaster_5:
            pass
        class CDOTA_Modifier_Axe_CounterHelix_DamageReduction:
            pass
        class CDOTA_Modifier_BookOfIntelligence:
            pass
        class CDOTA_Unit_Grimstroke_InkCreature:
            pass
        class C_DOTA_Unit_Courier:
            pass
        class CDOTA_Item_GunpowderGauntlets:
            pass
        class C_DOTA_Item_Recipe_HelmOfTheDominator:
            pass
        class C_DOTA_Ability_Hoodwink_Bushwhack:
            pass
        class C_DOTA_Ability_Invoker_Alacrity_AD:
            pass
        class C_DOTA_Ability_Enchantress_RabbleRouser:
            pass
        class CDOTA_Ability_Leshrac_ChronopticNourishment:
            pass
        class C_DOTA_Ability_Roshan_RevengeRoar:
            pass
        class CDOTA_Modifier_Watch_Tower_Invulnerable_Temporary:
            pass
        class CDOTA_Modifier_UpgradedBarricade:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Snapfire_3:
            pass
        class CDOTA_Modifier_Foragers_Kit:
            pass
        class CDOTA_Modifier_Searing_Signet_Burn:
            pass
        class CDOTA_Modifier_DarkWillow_BrambleMaze_Creation_Thinker:
            pass
        class CDOTA_Modifier_Silencer_GlobalSilence:
            pass
        class CDOTA_Modifier_Invoker_GhostWalk_Self:
            pass
        class CDOTA_Modifier_Spectre_ShadowStep_Illusion:
            pass
        class CDOTA_Modifier_Broodmother_WebWalk:
            pass
        class CDOTA_Modifier_Tiny_Insurmountable:
            pass
        class CDOTA_Modifier_Bane_FiendsGrip_Illusion:
            pass
        class CDOTA_Modifier_Antimage_DampenMagic_Bonus:
            pass
        class C_DOTA_Unit_Hero_WitchDoctor:
            pass
        class CDOTA_Item_Recipe_Faded_Broach:
            pass
        class C_DOTA_Ability_Gyrocopter_Chop_Shop:
            pass
        class C_DOTA_Ability_Enchantress_Enchant:
            pass
        class C_DOTA_Ability_DeathProphet_Silence:
            pass
        class CDOTA_Ability_FacelessVoid_TimeZone:
            pass
        class C_DOTA_Item_Nian_Flag_Trap:
            pass
        class C_DOTA_Ability_Nian_GreaterBash:
            pass
        class CDOTA_Modifier_HillTroll_Rally_Stack:
            pass
        class C_DOTA_Ability_EnragedWildkin_ToughnessAura:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Storm_Spirit:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Mirana_1:
            pass
        class C_DOTA_Ability_Special_Bonus_Gold_Income_240:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Base_Damage_40:
            pass
        class C_DOTA_Ability_Special_Bonus_Intelligence_13:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Speed_35:
            pass
        class CDOTA_Modifier_Item_Occult_Bracelet:
            pass
        class CDOTA_Modifier_Third_Eye:
            pass
        class CDOTA_Modifier_OrchidMalevolence_Debuff:
            pass
        class CDOTA_Modifier_Hoodwink_AcornShot_Dummy:
            pass
        class CDOTA_Modifier_Pangolier_Gyroshell:
            pass
        class CDOTA_Modifier_Oracle_DivinersDeck_TheLovers_Next:
            pass
        class CDOTA_Modifier_Magnataur_Skewer_Impact:
            pass
        class CDOTA_Modifier_Shadow_Demon_Menace:
            pass
        class CDOTA_Modifier_Lycan_Wolf_Bite_Attack_Range:
            pass
        class CDOTA_Modifier_Clinkz_BurningBarrage_DamageReduction:
            pass
        class CDOTA_Modifier_Leshrac_Lightning_Storm_Slow:
            pass
        class CDOTA_Modifier_SandKing_SandStorm_Blind:
            pass
        class CPulseCell_Outflow_CycleRandom:
            pass
        class CPulseCell_Step_PublicOutput:
            pass
        class C_DOTA_BaseNPC_Additive:
            pass
        class CDOTA_Match3_EffectsEntity:
            pass
        class C_DOTA_Item_Enhancement_Thick:
            pass
        class C_DOTA_Item_UltimateScepter_2:
            pass
        class CDOTA_Ability_Tusk_WalrusPunch:
            pass
        class C_DOTA_Ability_Wisp_Relocate:
            pass
        class C_DOTA_Ability_Rattletrap_BatteryAssault:
            pass
        class C_DOTA_Ability_Leshrac_Diabolic_Edict:
            pass
        class C_DOTA_Ability_TemplarAssassin_Trap_Teleport:
            pass
        class C_DOTA_Ability_Sniper_Headshot:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_30:
            pass
        class C_DOTA_Ability_Special_Bonus_Gold_lvl15_l:
            pass
        class CDOTA_Modifier_Spellslinger_Restore:
            pass
        class CDOTA_Modifier_Item_Hood_Of_Defiance:
            pass
        class CDOTA_Modifier_Terrorblade_Metamorphosis_Transform:
            pass
        class CDOTA_Modifier_DoomBringer_Devils_Bargain:
            pass
        class CDOTA_Modifier_DoomBringer_Doom_Break:
            pass
        class CDOTA_Modifier_Luna_Eclipse:
            pass
        class CDOTA_Modifier_TemplarAssassin_Meld:
            pass
        class CDOTA_Modifier_Venomancer_PoisonStingBase:
            pass
        class CDOTA_Modifier_Beastmaster_InnerBeast_Aura:
            pass
        class CDOTA_Modifier_Slardar_Bash_Active:
            pass
        class CDOTA_Modifier_Necrolyte_Heartstopper_Aura:
            pass
        class C_DOTA_Unit_Hero_Leshrac:
            pass
        class C_DOTA_Item_Overwhelming_Blink:
            pass
        class CDOTA_Item_Force_Boots:
            pass
        class CDOTA_Ability_Magnataur_Shockwave:
            pass
        class C_DOTA_Ability_Chen_Penitence:
            pass
        class CDOTA_Ability_Furion_SpiritOfTheForest:
            pass
        class C_DOTA_Ability_Luna_LucentBeam:
            pass
        class C_DOTA_Ability_Sven_Shieldbreaker:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Terrorblade:
            pass
        class C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_6:
            pass
        class CDOTA_Modifier_Item_Arcane_Blink_Buff:
            pass
        class CDOTA_Modifier_Techies_DeployRemoteMine:
            pass
        class CDOTA_Modifier_Keeper_of_the_Light_ManaMagnifier:
            pass
        class CDOTA_Modifier_DarkSeer_Surge_Trail:
            pass
        class CDOTA_Modifier_Rattletrap_Cog_Thinker:
            pass
        class CDOTA_Modifier_Leshrac_Diabolic_Edict:
            pass
        class CDOTA_Modifier_Viper_Nose_Dive_Effect:
            pass
        class CDOTA_Modifier_PhantomAssassin_Gravestone_Thinker:
            pass
        class CDOTA_Modifier_Enigma_Black_Hole_Thinker_Scepter:
            pass
        class CDOTA_Modifier_Lich_ChainFrost_Slow:
            pass
        class CDOTA_Modifier_Mirana_Starfall_Scepter_Thinker:
            pass
        class CDOTA_Modifier_AncientApparition_BoneChill_Debuff:
            pass
        class C_DOTA_Unit_Hero_Terrorblade:
            pass
        class C_DOTA_Item_Recipe_Desolator_2:
            pass
        class C_DOTA_Ability_ChaosKnight_Chaos_Strike:
            pass
        class C_DOTA_Ability_Lycan_SummonWolves_CriticalStrike:
            pass
        class CDOTA_Ability_DarkSeer_Normal_Punch:
            pass
        class C_DOTA_Ability_WitchDoctor_Voodoo_Switcheroo:
            pass
        class CDOTA_Ability_Tinker_Shrink_Ray:
            pass
        class CDOTA_Item_Tombstone_Mutation:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Invoker_11:
            pass
        class CDOTA_Modifier_Safety_Bubble:
            pass
        class CDOTA_Modifier_Item_Spy_Gadget:
            pass
        class CDOTA_Modifier_NagaSiren_SlithereenCutlass_MissChance:
            pass
        class CDOTA_Modifier_Meepo_Poof_Slow:
            pass
        class CDOTA_Modifier_Brewmaster_PrimalSplitDuration:
            pass
        class CDOTA_Modifier_SpiritBreaker_KnockbackAmp:
            pass
        class CDOTA_Modifier_SkeletonKing_Reincarnation:
            pass
        class CDOTA_Modifier_Aura:
            pass
        class C_DOTA_Unit_Hero_Batrider:
            pass
        class C_DOTA_BaseNPC_Creep_Talking:
            pass
        class C_DOTA_Item_Recipe_OrchidMalevolence:
            pass
        class CDOTA_Ability_Slark_SaltwaterShiv:
            pass
        class C_DOTA_Ability_Furion_Sprout:
            pass
        class C_DOTA_Ability_Furion_Greater_Sprout:
            pass
        class CDOTA_Ability_Death_Prophet_MourningRitual:
            pass
        class C_DOTA_Ability_Courier_GoToSecretShop:
            pass
        class C_DOTA_Ability_Courier_TakeStashItems:
            pass
        class C_DOTA_Ability_Enigma_BlackHole:
            pass
        class C_DOTA_Ability_Tiny_Tree_Channel:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Furion_2:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Mirana_7:
            pass
        class CDOTA_Ability_Special_Bonus_Spell_AoE_75:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Block_20:
            pass
        class CDOTA_Modifier_BotChallenge_SkeletonKing_BoneGuard_Summon:
            pass
        class CDOTA_Modifier_Item_TiaraOfSelemene:
            pass
        class CDOTA_Modifier_Item_Urn_Of_Shadows:
            pass
        class CDOTA_Modifier_Item_Bloodstone:
            pass
        class CDOTA_Modifier_Item_Aegis:
            pass
        class CDOTA_Modifier_MonkeyKing_FurArmyThinker:
            pass
        class CDOTA_Modifier_Shredder_Timberchain_Splinter_Slow:
            pass
        class CDOTA_Modifier_Spectre_Haunt:
            pass
        class CDOTA_Modifier_Pugna_NetherBlast_Thinker:
            pass
        class CDOTA_Modifier_Roshan_DevotionAura:
            pass
        class CDOTA_Modifier_Enigma_MidnightPulseThinker:
            pass
        class CDOTA_Modifier_VengefulSpirit_Command_Aura_Effect:
            pass
        class CDOTA_Modifier_AntiMage_ManaVoid:
            pass
        class CDOTA_Modifier_Filler_Heal:
            pass
        class C_IngameEvent_TI7:
            pass
        class CPulse_BlackboardReference:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class C_DOTA_Unit_VisageFamiliar:
            pass
        class CDOTA_Item_Enchanters_Bauble:
            pass
        class CDOTA_Item_Octarine_Core:
            pass
        class C_DOTA_Item_Recipe_Ethereal_Blade:
            pass
        class C_DOTA_Item_Recipe_Maelstrom:
            pass
        class C_DOTA_Item_RingOfHealth:
            pass
        class C_DOTA_Ability_Brewmaster_AstralPulse:
            pass
        class C_DOTA_Ability_Leshrac_Greater_Lightning_Storm:
            pass
        class C_DOTA_Ability_Twin_Gate_Portal_Warp:
            pass
        class CDOTA_Modifier_Seasonal_TI11_RockPaperScissors:
            pass
        class CDOTA_Ability_UpheavalUrn_Reincarnation:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Pudge_7:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Earthshaker_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Ancient_Apparition_7:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Arc_Warden_5:
            pass
        class CDOTA_Modifier_Item_Moonshard_Consumed:
            pass
        class CDOTA_Modifier_Kez_GrapplingClaw_Slow:
            pass
        class CDOTA_Modifier_Phoenix_Sun_Ray_Slow:
            pass
        class CDOTA_Modifier_Techies_Suicide_Leap_Animation:
            pass
        class CDOTA_Modifier_Slark_EssenceShift:
            pass
        class CDOTA_Modifier_Invoker_Exort_Intrinsic:
            pass
        class CDOTA_Modifier_Venomancer_Plague_Carrier:
            pass
        class CDOTA_Modifier_Enigma_DemonicConversion:
            pass
        class CDOTA_Modifier_PhantomLancer_JuxtaposeIllusionUncontrollable:
            pass
        class C_DOTA_Item_AncientGuardian:
            pass
        class C_DOTA_Item_Disperser:
            pass
        class C_DOTA_Ability_Rubick_Curiosity:
            pass
        class C_DOTA_Ability_Lycan_FeralImpulse:
            pass
        class CDOTA_Ability_Obsidian_Destroyer_Objurgation:
            pass
        class CDOTA_Ability_Ursa_Bear_Down:
            pass
        class CDOTA_Ability_AghsFort_Creature_Phoenix_LaunchFireSpirit:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Broodmother_4:
            pass
        class CDOTA_Modifier_Special_Bonus_All_Stats:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_Regen_5:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_Regen_50:
            pass
        class C_DOTA_Ability_Special_Bonus_Cleave_150:
            pass
        class CDOTA_Modifier_Muerta_Revenant_Thinker:
            pass
        class CDOTA_Modifier_Marci_Lunge_LandingAnim:
            pass
        class CDOTA_Modifier_Winter_Wyvern_Dragon_Sight:
            pass
        class CDOTA_Modifier_Centaur_Return_Aura:
            pass
        class CDOTA_Modifier_Chaos_Knight_FundamentalForging:
            pass
        class CDOTA_Modifier_Enchantress_Little_Friends_Kill_Credit:
            pass
        class CDOTA_Modifier_QueenOfPain_SonicWave_Knockback:
            pass
        class CDOTA_Modifier_Warlock_Golem_Permanent_Immolation_Debuff:
            pass
        class CDOTA_Modifier_Morphling_Become_Agility:
            pass
        class CODTA_Unit_Warlock_Imp:
            pass
        class CDOTA_Item_Recipe_Demonicon:
            pass
        class C_DOTA_Item_Third_eye:
            pass
        class C_DOTA_Ability_Necronomicon_Warrior_ManaBurn:
            pass
        class C_DOTA_Ability_ArcWarden_MagneticField:
            pass
        class C_DOTA_Ability_Tusk_Snowball:
            pass
        class C_DOTA_Ability_Meepo_FlingRelease:
            pass
        class C_DOTA_Ability_FacelessVoid_TimeWalk_Reverse:
            pass
        class CDOTA_Ability_Tinker_WarpGrenade:
            pass
        class C_DOTA_Ability_Zuus_StaticField:
            pass
        class C_DOTA_Ability_Morphling_Syntropy:
            pass
        class CDOTA_Modifier_ForestTrollHighPriest_HealAmp:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Amplify_25:
            pass
        class CDOTA_Modifier_Item_MartyrsPlate:
            pass
        class CDOTA_Modifier_Item_Vladmir_Aura:
            pass
        class CDOTA_Modifier_Item_SobiMask:
            pass
        class CDOTA_Modifier_Kez_FalconRush_Intrinsic:
            pass
        class CDOTA_Modifier_Elder_Titan_EchoStomp_Magic_Immune:
            pass
        class CDOTA_Modifier_Centaur_Cart:
            pass
        class CDOTA_Modifier_NagaSiren_RipTide_Slow:
            pass
        class CDOTA_Modifier_Ogre_Magi_Fireblast_Multicast:
            pass
        class CDOTA_Modifier_Treant_NaturesGuise:
            pass
        class CDOTA_Modifier_Treant_EyesInTheForest_Thinker:
            pass
        class CDOTA_Modifier_SpiritBreaker_ChargeOfDarknessTarget:
            pass
        class CDOTA_Modifier_DragonKnight_FrostBreath_Slow:
            pass
        class CDOTA_Modifier_Phantom_Assassin_Immaterial:
            pass
        class CDOTA_Modifier_Miniboss_Radiance:
            pass
        class CDOTA_Modifier_Courier_TransferItems:
            pass
        class CDOTA_Modifier_Sniper_KeenScope:
            pass
        class CDOTA_Modifier_CrystalMaiden_Let_It_Go_Bonus:
            pass
        class CDOTA_Modifier_AntiMage_Mana_Thirst_Vision:
            pass
        class C_DOTAMutationGameMode:
            pass
        class CAnimGraphNetworkedVariables:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Unit_AghsFort_Creature_DungeonBat:
            pass
        class C_DOTA_Item_Recipe_Skadi:
            pass
        class C_DOTA_Ability_Shredder_Chakram:
            pass
        class CDOTA_Ability_Centaur_DoubleEdge:
            pass
        class C_DOTA_Ability_Viper_Nose_Dive:
            pass
        class CDOTA_Ability_Spawnlord_Master_Stomp:
            pass
        class C_DOTA_Ability_Special_Bonus_Strength_9:
            pass
        class C_DOTA_Ability_Special_Bonus_Intelligence_12:
            pass
        class CDOTA_Modifier_Item_Quicksilver_Amulet:
            pass
        class CDOTA_Modifier_RodOfAtos_Debuff:
            pass
        class CDOTA_Modifier_Item_BladesOfAttack:
            pass
        class CDOTA_Modifier_Item_Mantle:
            pass
        class CDOTA_Modifier_Hoodwink_AcornShot_TreeThinker:
            pass
        class CDOTA_Modifier_Meepo_Geomancy:
            pass
        class CDOTA_Modifier_LoneDruid_DruidForm_Transform:
            pass
        class CDOTA_Modifier_Silencer_LastWord:
            pass
        class CDOTA_Modifier_Windrunner_Arcana:
            pass
        class CDOTA_Modifier_Kunkka_XMarksTheSpotMarker:
            pass
        class CDOTA_Modifier_Mirana_SelemenesFaithful_Aura:
            pass
        class CDOTA_Modifier_Sand_King_BurrowStrike_Tracker:
            pass
        class CDOTA_Modifier_Nevermore_Requiem_Thinker:
            pass
        class CDOTA_ArcanaDataEntity_FacelessVoid:
            pass
        class CFilterModel:
            pass
        class C_DOTA_Item_MartyrsPlate:
            pass
        class CDOTA_Item_Spider_Legs:
            pass
        class C_DOTA_Ability_Chen_Martyrdom:
            pass
        class CDOTA_Ability_Skeleton_King_Innate_VampiricSpirit:
            pass
        class C_DOTA_Ability_BlueDragonspawnOverseer_Evasion:
            pass
        class C_DOTA_Ability_AlphaWolf_CriticalStrike:
            pass
        class C_SoundAreaEntityOrientedBox:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Leshrac_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Axe_2:
            pass
        class C_DOTA_Ability_Special_Bonus_Cleave_15:
            pass
        class CDOTA_Modifier_Item_Angels_Demise_InternalCD:
            pass
        class CDOTA_Modifier_Item_HydrasBreath:
            pass
        class CDOTA_Modifier_Item_HelmOfTheDominator_2:
            pass
        class CDOTA_Modifier_Item_SangeAndYasha:
            pass
        class CDOTA_Modifier_Marci_Unleash_FlurryCooldown:
            pass
        class CDOTA_Modifier_Abaddon_Frostmourne_Debuff:
            pass
        class CDOTA_Modifier_Skywrath_Mage_Mystic_Flare:
            pass
        class CDOTA_Modifier_Slark_Pounce_Leash:
            pass
        class CDOTA_Modifier_Disruptor_KineticField:
            pass
        class CDOTA_Modifier_Spectre_Dispersion:
            pass
        class CDOTA_Modifier_Jakiro_DoubleTrouble:
            pass
        class CDOTA_Modifier_Tidehunter_Anchor_Unit:
            pass
        class CDOTA_Modifier_Windrunner_ShackleShot:
            pass
        class CDOTA_Modifier_ProjectileVisionOnMinimap:
            pass
        class DOTA_AssassinMinigameNetworkState:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_SoundOpvarSetPointEntity:
            pass
        class C_DOTA_Unit_Hero_Naga_Siren:
            pass
        class C_DOTA_Item_Dormant_Curio:
            pass
        class C_DOTA_Item_Rattlecage:
            pass
        class C_DOTA_Ability_Oracle_FatesEdict:
            pass
        class CDOTA_Ability_Techies_Minefield_Sign:
            pass
        class C_DOTA_Ability_Chen_HandOfGod:
            pass
        class CDOTA_Ability_Miniboss_Alleviation:
            pass
        class C_DOTA_Ability_Earthshaker_Fissure:
            pass
        class CPulseGameBlackboard:
            pass
        class C_DOTA_Ability_Special_Bonus_Intelligence_10:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_100:
            pass
        class TreeModelReplacement_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Modifier_Item_Enhancement_Crude:
            pass
        class CDOTA_Modifier_Item_Illusionists_Cape_Aura:
            pass
        class CDOTA_Modifier_Item_Force_Boots:
            pass
        class CDOTA_Modifier_Muerta_PartingShot_SoulDebuff:
            pass
        class CDOTA_Modifier_Earth_Spirit_Magnetize_Hero_Self_Buff:
            pass
        class CDOTA_Modifier_EmberSpirit_FireRemnantThinker:
            pass
        class CDOTA_Modifier_Shadow_Demon_DemonicPurge_ChargeCounter:
            pass
        class CDOTA_Modifier_Invoker_Tornado_Twister:
            pass
        class CDOTA_Modifier_Chen_Penitence_Self_Attack_Range:
            pass
        class CDOTA_Modifier_Batrider_Smoldering_Resin:
            pass
        class CDOTA_Modifier_Dark_Seer_Mental_Fortitude:
            pass
        class CDOTA_Modifier_Lina_Overheat:
            pass
        class CDOTA_Modifier_CrystalMaiden_IceRink_Movement:
            pass
        class C_IngameEvent_WM2017:
            pass
        class CChoreoComponent:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CPulseCell_Value_RandomInt:
            pass
        class C_DOTA_Item_Recipe_Perseverance:
            pass
        class C_DOTA_Ability_Elder_Titan_EchoStomp:
            pass
        class C_DOTA_Ability_Shredder_Exposure_Therapy:
            pass
        class C_DOTA_Ability_BountyHunter_ShurikenToss:
            pass
        class C_DOTA_Ability_Dazzle_NothlBoon:
            pass
        class C_DOTA_Ability_Lion_Innate_ToHellAndBack:
            pass
        class C_DOTA_Ability_Earthshaker_Spirit_Cairn:
            pass
        class CDOTA_Modifier_AghsFort_DragonKnight_BreatheFire_Debuff:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Crystal_Maiden_2:
            pass
        class CDOTA_Modifier_Gungnir_Debuff_MagicResistance:
            pass
        class CDOTA_Modifier_Item_Yasha_And_Kaya:
            pass
        class CDOTA_Modifier_Item_ForceStaff:
            pass
        class CDOTA_Modifier_Ringmaster_TheBox_Transform:
            pass
        class CDOTA_Modifier_PrimalBeast_Pulverize_Self:
            pass
        class CDOTA_Modifier_Marci_Lunge_TrackingMotion:
            pass
        class CDOTA_Modifier_EmberSpirit_SleightOfFist_Caster_Invulnerability:
            pass
        class CDOTA_Modifier_Centaur_DoubleEdge_Slow:
            pass
        class CDOTA_Modifier_NagaSiren_SlithereenCutlass:
            pass
        class CDOTA_Modifier_FacelessVoid_TimeDilation_Distortion:
            pass
        class CDOTA_Modifier_Venomancer_PoisonStingWard:
            pass
        class CDOTA_Modifier_Nian_Attachment:
            pass
        class CDOTA_Modifier_Glyph_Reset:
            pass
        class CDOTA_Modifier_Stacking_Base:
            pass
        class C_IngameEvent_FV2019:
            pass
        class C_RagdollPropAttached:
            pass
        class CDOTA_Item_Recipe_Overwhelming_Blink:
            pass
        class C_DOTA_Item_Dagon:
            pass
        class C_DOTA_Item_Recipe_Dagon3:
            pass
        class C_DOTA_Ability_Slark_Depth_Shroud:
            pass
        class CDOTA_Ability_NagaSiren_Deluge:
            pass
        class C_DOTA_Ability_Batrider_StickyNapalm:
            pass
        class CDOTA_Ability_Clinkz_SearingArrows:
            pass
        class C_ModelPointEntity:
            pass
        class CDOTA_Modifier_Seasonal_Summon_CNY_Balloon_Thinker:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Timbersaw:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Speed_45:
            pass
        class CDOTA_Modifier_RippersLash_Debuff:
            pass
        class CDOTA_Modifier_Item_Heavy_Blade:
            pass
        class CDOTA_Modifier_Lotus_Orb_Delay:
            pass
        class CDOTA_Modifier_Item_UltimateScepter_Consumed:
            pass
        class CDOTA_Modifier_Ringmaster_DarkCarnivalSouvenirs:
            pass
        class CDOTA_Modifier_Pangolier_Swashbuckle_Stunned:
            pass
        class CDOTA_Modifier_Phoenix_SunRayVision:
            pass
        class CDOTA_Modifier_Tusk_Tag_Team_Attack_Slow:
            pass
        class CDOTA_Modifier_Wisp_Spirit_Invulnerable:
            pass
        class CDOTA_Modifier_Omniknight_HammerOfPurity:
            pass
        class CDOTA_Modifier_Rattletrap_Junk_Mail:
            pass
        class CDOTA_Modifier_Rattletrap_Cog_Immune:
            pass
        class CDOTA_Modifier_Ursa_Earthshock_Move:
            pass
        class CDOTA_Modifier_Lina_Slow_BurnIntrinsic:
            pass
        class C_DOTA_Unit_Hero_Tiny:
            pass
        class CDOTA_Item_Ex_Machina:
            pass
        class C_DOTA_Item_Tome_of_aghanim:
            pass
        class CDOTA_Item_RiverPainter6:
            pass
        class C_DOTA_Item_MithrilHammer:
            pass
        class C_DOTA_Ability_Ringmaster_Wheel:
            pass
        class C_DOTA_Ability_Oracle_Prognosticate:
            pass
        class C_DOTA_Ability_NyxAssassin_Innate_Mana_Burn:
            pass
        class C_DOTA_Ability_Templar_Assassin_Hidden_Gates:
            pass
        class CDOTA_Ability_Miniboss_Minion_FollowingMovement:
            pass
        class C_DOTA_Ability_WitchDoctor_ParalyzingCask:
            pass
        class C_DOTA_Ability_Morphling_Morph_Agi:
            pass
        class C_DOTA_Ability_AncientRockGolem_Weakening_Aura:
            pass
        class CDOTA_Modifier_AghsFort_Tower_BlastWave_Thinker:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Ursa_2:
            pass
        class C_DOTA_Ability_Special_Bonus_Status_Resistance_25:
            pass
        class C_DOTA_Ability_Special_Bonus_Corruption_5:
            pass
        class CDOTA_Modifier_Ceremonial_Robe_Aura:
            pass
        class CDOTA_Modifier_Item_Orb_of_Venom:
            pass
        class CDOTA_Modifier_Visage_SummonFamiliars_StoneForm_Buff:
            pass
        class CDOTA_Modifier_Ogre_Magi_School:
            pass
        class CDOTA_Modifier_Life_Stealer_Rage:
            pass
        class CDOTA_Modifier_Viper_NethertoxinMute:
            pass
        class CDOTA_Modifier_Necrolyte_GhostShroud_Active:
            pass
        class CDOTA_Modifier_Lina_LagunaBlade_Superheated:
            pass
        class CDOTA_Modifier_PhantomLancer_Juxtapose_Invisibility:
            pass
        class CDOTA_Modifier_Sven_Warcry_Barrier:
            pass
        class C_DOTA_CombatLogQueryProgress:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CAnimationLayer:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Unit_Hero_StormSpirit:
            pass
        class C_DOTA_BaseNPC_Effigy_BattleCup:
            pass
        class C_DOTA_Item_Recipe_Orb_Of_Revelations:
            pass
        class C_DOTA_Item_Recipe_Chipped_Vest:
            pass
        class CDOTA_Item_Mango_Tree:
            pass
        class C_DOTA_Item_QuellingBlade:
            pass
        class C_DOTA_Ability_Lycan_Shapeshift:
            pass
        class C_DOTA_Ability_Brewmaster_HurlBoulder:
            pass
        class C_DOTA_Ability_Alchemist_UnstableConcoction:
            pass
        class C_DOTA_Ability_Drow_Ranger_Glacier:
            pass
        class CDOTA_Modifier_HarpyScout_TakeOff:
            pass
        class C_DOTA_Ability_SatyrHellcaller_UnholyAura:
            pass
        class C_DOTA_Ability_SatyrHellcaller_Shockwave:
            pass
        class CDOTA_Modifier_Plus_HighFiveSuccess:
            pass
        class CDOTA_Modifier_Special_Bonus_Base_Attack_Rate:
            pass
        class C_DOTA_Ability_Special_Bonus_Magic_Resistance_30:
            pass
        class CDOTA_Modifier_Item_PigletPole_Critter:
            pass
        class CDOTA_Modifier_Item_Paintball:
            pass
        class CDOTA_Modifier_Item_PhaseBoots_Active:
            pass
        class CDOTA_Modifier_Pangolier_Swashbuckle:
            pass
        class CDOTA_Modifier_Bristleback_BrawlersGrit_Buff:
            pass
        class CDOTA_Modifier_SpiritBreaker_NetherStrike_Vision:
            pass
        class CDOTA_Modifier_Dazzle_NothlBoon:
            pass
        class CDOTA_Modifier_Courier_Burst:
            pass
        class CDOTA_Modifier_WitchDoctor_Voodoo_Restoration_Heal:
            pass
        class CDOTA_Modifier_Morphling_Waveform:
            pass
        class C_DOTA_Unit_Hero_Meepo:
            pass
        class CDOTA_Item_Prophets_Pendulum:
            pass
        class C_DOTA_Item_Enhancement_Brawny:
            pass
        class C_DOTA_Item_Fallen_Sky:
            pass
        class CDOTA_Item_Recipe_Ocean_Heart:
            pass
        class C_DOTA_Ability_AbyssalUnderlord_Abyssal_Horde:
            pass
        class C_DOTA_Ability_Skywrath_Mage_Arcane_Bolt:
            pass
        class CDOTA_Ability_Treant_LifeBomb:
            pass
        class CDOTA_Ability_Lycan_SummonWolves_Hamstring:
            pass
        class C_DOTA_Ability_Omniknight_Pacify:
            pass
        class CDOTA_Modifier_MudGolem_Rock_Shard:
            pass
        class CPathSimple:
            pass
        class CDOTA_Modifier_AghsFort_ShadowWaveEffectPotion:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Riki_9:
            pass
        class C_DOTA_Ability_Special_Bonus_Respawn_Reduction_35:
            pass
        class C_DOTA_Ability_Special_Bonus_All_Stats_14:
            pass
        class CDOTA_Modifier_Marci_CompanionRun_AllyMovespeed:
            pass
        class CDOTA_Modifier_Hoodwink_MistwoodsWayfarer_Passive:
            pass
        class CDOTA_Modifier_Pudge_Dismember_Gluttony_Strength_Stack:
            pass
        class C_DOTA_BaseNPC_HoldoutTower:
            pass
        class C_DOTA_Item_Recipe_Dagon5:
            pass
        class C_DOTA_Item_WraithBand:
            pass
        class C_DOTA_Ability_PrimalBeast_Onslaught_Release:
            pass
        class CDOTA_Ability_Special_Bonus_Unique_Marci_Grapple_StunDuration:
            pass
        class C_DOTA_Ability_Techies_FocusedDetonate:
            pass
        class C_DOTA_Ability_QueenOfPain_Blink:
            pass
        class C_DOTA_Ability_Necrolyte_Sadist_Stop:
            pass
        class CDOTA_Ability_Bloodseeker_Thirst:
            pass
        class CDOTA_Ability_Mirana_CelestialQuiver:
            pass
        class C_DOTA_Ability_Nevermore_Shadowraze:
            pass
        class C_DOTA_Ability_Bane_FiendsGrip:
            pass
        class C_FuncTrackTrain:
            pass
        class CDOTA_Modifier_Aghsfort_Aziyog_Underlord_Portal_FX:
            pass
        class C_DOTA_Ability_Aghsfort_Wildwing_Tornado_Blast:
            pass
        class CDOTA_Modifier_Ascension_AcidBlood_Thinker:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Tusk_5:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Ursa_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Cast_Range_200:
            pass
        class C_DOTA_Ability_Special_Bonus_Night_Vision_500:
            pass
        class C_DOTA_Ability_Special_Bonus_Movement_Speed_Percentage_14:
            pass
        class C_DOTA_Ability_Special_Bonus_Mana_Break_15:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Speed_100:
            pass
        class CDOTAPulsePanoramaFuncs:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Modifier_Item_Mage_Slayer:
            pass
        class CDOTA_Modifier_Item_Phylactery_Slow:
            pass
        class CDOTA_Modifier_Item_Guardian_Greaves:
            pass
        class CDOTA_Modifier_Elder_Titan_EarthSplitter_Thinker:
            pass
        class CDOTA_Modifier_Nyx_Assassin_Impale:
            pass
        class CDOTA_Modifier_TargetDummy_Unkillable:
            pass
        class CDOTA_Modifier_FacelessVoid_Arcana:
            pass
        class CDOTA_Modifier_Sniper_TakeAim:
            pass
        class CDOTA_Modifier_Riki_Permanent_Invisibility:
            pass
        class CDOTA_Modifier_Razor_StormSurge:
            pass
        class CDOTA_Modifier_Earthshaker_EnchantTotem_BodyProjectile:
            pass
        class CDOTA_Modifier_SkeletonKing_MortalStrike:
            pass
        class CDOTA_Modifier_Bane_FiendsGrip:
            pass
        class CDOTA_Modifier_Hide_On_Minimap:
            pass
        class C_DOTATurboHeroPickRules:
            pass
        class C_EconWearable:
            pass
        class CDOTA_Item_AshLegionShield:
            pass
        class C_DOTA_Ability_Winter_Wyvern_Bookwyrm:
            pass
        class C_DOTA_Ability_Rubick_Empty1:
            pass
        class C_DOTA_Ability_Dazzle_Bad_Juju:
            pass
        class CDOTA_Ability_Beastmaster_Summon_Raptor:
            pass
        class C_DOTA_Ability_Brewmaster_PrimalSplit:
            pass
        class C_EnvDecal:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Skywrath_6:
            pass
        class C_DOTA_Ability_Special_Bonus_Mana_Reduction_10:
            pass
        class PlayerResourceBroadcasterData_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Modifier_Item_Necronomicon_2:
            pass
        class CDOTA_Modifier_Event_Frogling:
            pass
        class CDOTA_Modifier_VoidSpirit_AetherRemnant_Unit:
            pass
        class CDOTA_Modifier_Pangolier_Swashbuckle_Slow:
            pass
        class CDOTA_Modifier_Legion_Commander_PressTheAttack:
            pass
        class CDOTA_Modifier_Legion_Commander_OverwhelmingOdds_Armor:
            pass
        class CDOTA_Modifier_Abaddon_Frostmourne_Debuff_Bonus:
            pass
        class CDOTA_Modifier_Treant_LeechSeed:
            pass
        class CDOTA_Modifier_Dark_Seer_Aggrandize:
            pass
        class CDOTA_Modifier_Tidehunter_Gush:
            pass
        class fogparams_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Unit_Hero_Undying:
            pass
        class C_EnvDeferredLightClientOnly:
            pass
        class C_DOTA_Item_Enhancement_Hulking:
            pass
        class CDOTA_Ability_Life_Stealer_Empty1:
            pass
        class C_DOTA_Ability_Witch_Doctor_Innate_Maledict_Heal_Reduction:
            pass
        class CDOTA_Modifier_KoboldTunneler_ProspectingAura:
            pass
        class C_DOTA_Ability_Special_Bonus_20_Crit_2:
            pass
        class C_DOTA_Ability_Special_Bonus_Respawn_Reduction_15:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_350:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_275:
            pass
        class CDOTA_Modifier_Item_Enhancement_Fierce:
            pass
        class CDOTA_Modifier_Item_Holy_Locket:
            pass
        class CDOTA_Modifier_Item_Lotus_Orb:
            pass
        class CDOTA_Modifier_Item_CrellasCrozier:
            pass
        class CDOTA_Modifier_Largo_CatchyLick_Knockback:
            pass
        class CDOTA_Modifier_Dawnbreaker_Solar_Guardian_AirTime:
            pass
        class CDOTA_Modifier_Phoenix_Dying_Light:
            pass
        class CDOTA_Modifier_Brewmaster_PrimalSplit:
            pass
        class CDOTA_Modifier_TemplarAssassin_RefractionAbsorb:
            pass
        class CDOTA_Modifier_Beastmaster_InnerBeast_Berserk:
            pass
        class CDOTA_Modifier_VengefulSpirit_Command_Aura_Effect_Debuff:
            pass
        class CDOTA_Modifier_SandKing_SandStorm_Invisibility:
            pass
        class CDOTA_Modifier_SecondaryUnit_Taunt:
            pass
        class C_DOTA_Unit_Hero_Huskar:
            pass
        class C_DOTA_Item_BlackGrimoire:
            pass
        class C_DOTA_Item_Wind_Waker:
            pass
        class C_DOTA_Item_Recipe_Pupils_gift:
            pass
        class C_DOTA_Item_HandOfMidas:
            pass
        class C_DOTA_Item_Mantle:
            pass
        class CDOTA_Ability_Special_Bonus_Unique_Marci_Guardian_Lifesteal:
            pass
        class CDOTA_Ability_Abaddon_DeathCoil:
            pass
        class C_DOTA_Ability_Dazzle_ShadowWave:
            pass
        class C_DOTA_Ability_Nian_Dive:
            pass
        class C_DOTA_Ability_Zuus_ThundergodsWrath:
            pass
        class CDOTA_Modifier_MudGolem_CloakAura_Bonus:
            pass
        class C_Beam:
            pass
        class C_EnvLightProbeVolume:
            pass
        class C_DOTA_Ability_AghsFort_AssaultCaptain_SunRay:
            pass
        class CDOTA_AghsFort_Ability_Creature_Venomancer_PoisonSting:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Silencer_2:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Dawnbreaker_FireWreath_Swipe:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Mirana_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Earthshaker_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Exp_Boost_5:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_Regen_4:
            pass
        class CDOTA_Modifier_Item_HydrasBreath_Thinker:
            pass
        class CDOTA_Modifier_Item_Flicker_Damaged:
            pass
        class CDOTA_Modifier_Centaur_Double_Edge_Damage_Tracking:
            pass
        class CDOTA_Modifier_Nyx_Assassin_Jolt_Damage_Tracker:
            pass
        class CDOTA_Modifier_SpiritBreaker_ChargeOfDarkness_Linger:
            pass
        class CDOTA_Modifier_Omniknight_Degen_Aura_Effect:
            pass
        class CDOTA_Modifier_Venomancer_VenomousGale:
            pass
        class CDOTA_Modifier_Windrunner_Tailwind_Counter:
            pass
        class CDOTA_Modifier_Bloodseeker_Bloodrage:
            pass
        class CDOTA_Modifier_AttackImmune:
            pass
        class CExplosionTypeData:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CDOTA_Item_Partisans_Brand:
            pass
        class C_DOTA_Item_Recipe_Arcane_Ring:
            pass
        class CDOTA_Item_Recipe_Grove_Bow:
            pass
        class C_DOTA_Item_Recipe_Mekansm:
            pass
        class C_DOTA_Item_MaskOfMadness:
            pass
        class C_DOTA_Item_StoutShield:
            pass
        class C_DOTA_Ability_DoomBringer_ScorchedEarth:
            pass
        class C_DOTA_Ability_Enchantress_Untouchable:
            pass
        class C_DOTA_Ability_Tiny_Grow:
            pass
        class C_DOTA_Ability_Morphling_Morph_Str:
            pass
        class C_DOTA_Ability_AntiMage_SpellShield:
            pass
        class C_DOTA_Ability_DarkTrollWarlord_Ensnare:
            pass
        class CDOTA_Modifier_Watch_Tower_Marker:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lion_8:
            pass
        class C_DOTA_Ability_Special_Bonus_Exp_Boost_10:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_600:
            pass
        class CDOTA_Modifier_Oracle_DivinersDeck_TheFool:
            pass
        class CDOTA_Modifier_Legion_Commander_MomentOfCourage_Lifesteal:
            pass
        class CDOTA_Modifier_Shredder_Chakram_Thinker:
            pass
        class CDOTA_Modifier_Meepo_Fling_Held:
            pass
        class CDOTA_Modifier_TemplarAssassin_Meld_Animation:
            pass
        class CDOTA_Modifier_Pugna_LifeDrain:
            pass
        class CDOTA_Modifier_CrystalMaiden_Arcane_Overflow_Active:
            pass
        class CDOTA_Modifier_DrowRanger_FrostArrows_Hypothermia_Active:
            pass
        class CDOTA_Modifier_Juggernaut_Trinity_Movement:
            pass
        class C_DOTA_Unit_Broodmother_Web:
            pass
        class C_DOTA_Item_Sisters_Shroud:
            pass
        class CDOTA_Item_Ward_Dispenser:
            pass
        class C_DOTA_Ability_Hoodwink_Scurry:
            pass
        class C_DOTA_Ability_Terrorblade_Dark_Unity:
            pass
        class CDOTA_Ability_Dazzle_NothlProjectionEnd:
            pass
        class C_DOTA_Ability_Riki_BlinkStrike:
            pass
        class C_DOTA_Ability_SkeletonKing_MortalStrike:
            pass
        class CDOTA_Modifier_AghsFort_Venomancer_PoisonSting:
            pass
        class CDOTA_Modifier_AghsFort_SkeletonKing_VampiricAura:
            pass
        class CDOTA_Ability_Aghsfort_TempBuff_CorpseExplosion:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lich_1:
            pass
        class CDOTA_Modifier_Item_Solar_Crest:
            pass
        class CDOTA_Modifier_Item_HeavensHalberd:
            pass
        class CDOTA_Modifier_Item_Veil_Of_Discord_Debuff:
            pass
        class CDOTA_Modifier_Ringmaster_CrystalBall_Truesight:
            pass
        class CDOTA_Modifier_Marci_Lunge_Buff:
            pass
        class CDOTA_Modifier_Invoker_AttackVisuals:
            pass
        class CDOTA_Modifier_Rattletrap_Armor_Power:
            pass
        class CDOTA_Modifier_DragonKnight_BlackDragon_Tooltip:
            pass
        class CDOTA_Modifier_Slardar_Slithereen_Crush:
            pass
        class CDOTA_Modifier_Axe_Culling_Blade_NoMinHealth:
            pass
        class CDOTA_Modifier_Taunt:
            pass
        class C_DOTA_MapTree:
            pass
        class CDOTA_Ability_Abyssal_Underling_Archer_AoE:
            pass
        class C_DOTA_Ability_FacelessVoid_Innate_DistortionField:
            pass
        class CDOTA_Modifier_GiantWolf_CriticalStrike:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Underlord_6:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_500:
            pass
        class CDOTA_Modifier_Ringmaster_StrongmanTonic_Buff:
            pass
        class CDOTA_Modifier_Dawnbreaker_Fire_Wreath_Caster:
            pass
        class CDOTA_Modifier_Techies_Suicide_RespawnTime:
            pass
        class CDOTA_Modifier_Centaur_Horsepower:
            pass
        class CDOTA_Modifier_LoneDruid_Rabid:
            pass
        class CDOTA_Modifier_Fissure_Rooted:
            pass
        class CDOTA_Modifier_Pudge_Swallow_Hide:
            pass
        class CDOTA_Modifier_ScoutBonuses:
            pass
        class CIngameEvent_TI2024:
            pass
        class C_DOTA_Unit_Roshans_Banner:
            pass
        class C_DOTA_BaseNPC_HoldoutTower_LightFast:
            pass
        class CEnvSoundscapeTriggerableAlias_snd_soundscape_triggerable:
            pass
        class CDOTA_Item_Spellslinger:
            pass
        class C_DOTA_Item_Book_Of_Shadows:
            pass
        class C_DOTA_Item_Veil_Of_Discord:
            pass
        class CDOTA_Ability_Techies_StasisTrap:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Abaddon_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Disruptor_4:
            pass
        class C_DOTA_Ability_Lycan_Apex_Predator:
            pass
        class C_DOTA_Ability_Invoker_IceWall_AD:
            pass
        class CDOTA_Ability_Alchemist_ChemicalRage:
            pass
        class C_DOTA_Ability_Life_Stealer_Ghoul_Frenzy:
            pass
        class CDOTA_Ability_Venomancer_PoisonSting:
            pass
        class C_DOTA_Ability_Juggernaut_HealingWard:
            pass
        class C_DOTA_Ability_Bane_Enfeeble:
            pass
        class C_Breakable:
            pass
        class CDOTA_Modifier_AghsFort_Potion_SpendCharge:
            pass
        class CDOTA_Modifier_Aghsfort_TempBuff_CorpseExplosion_Debuff:
            pass
        class C_DOTA_Ability_Healing_Campfire:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Oracle_8:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Templar_Assassin_8:
            pass
        class CDOTA_Modifier_Item_AshLegionShield:
            pass
        class CDOTA_Modifier_Devastator_Debuff:
            pass
        class CDOTA_Modifier_Light_Collector_Blind:
            pass
        class CDOTA_Modifier_Item_Tree_Processor_Speed:
            pass
        class CDOTA_Modifier_Item_Trusty_Shovel:
            pass
        class CDOTA_Modifier_Item_Essence_Distiller_Thinker:
            pass
        class CDOTA_Modifier_Item_Mekansm:
            pass
        class CDOTA_Modifier_Mars_Dauntless:
            pass
        class CDOTA_Modifier_Shredder_Reactive_Armor:
            pass
        class CDOTA_Modifier_Rubick_NullField:
            pass
        class CDOTA_Modifier_ChaosKnight_Phantasm_Illusion_Shard:
            pass
        class CDOTA_Modifier_Lycan_SummonWolves_BonusDamage:
            pass
        class CDOTA_Modifier_Treant_Large_Bonus:
            pass
        class CDOTA_Modifier_Treant_Bonus:
            pass
        class CDOTA_Modifier_Windrunner_Windrun_ChargeCounter:
            pass
        class C_DOTA_PortraitEntity:
            pass
        class C_DOTA_Unit_Earth_Spirit_Stone:
            pass
        class C_DOTA_BaseNPC_Creep_Neutral:
            pass
        class CDOTA_Item_FlayersBota:
            pass
        class CDOTA_Item_Illusionsts_Cape:
            pass
        class C_DOTA_Ability_Wisp_Spirits_In:
            pass
        class CDOTA_Modifier_PathMoveSpeed:
            pass
        class C_DOTA_Ability_Special_Bonus_Day_Vision_400:
            pass
        class CDOTA_Modifier_Avianas_Feather:
            pass
        class CDOTA_Modifier_Item_Bullwhip_Buff:
            pass
        class CDOTA_Modifier_Item_IronwoodBranch:
            pass
        class CDOTA_Modifier_Furion_CurseOfTheForest:
            pass
        class CDOTA_Modifier_Furion_Sprout_Marker:
            pass
        class CDOTA_Modifier_Furion_Sprout_Heal:
            pass
        class CDOTA_Modifier_Provide_Vision:
            pass
        class CDOTA_Modifier_Tower_Aura_Bonus:
            pass
        class CIngameEvent_Crownfall:
            pass
        class CDOTA_ArcanaDataEntity_Razor:
            pass
        class CFilterName:
            pass
        class C_DOTA_Ability_Grimstroke_Return:
            pass
        class C_DOTA_Ability_Oracle_PurifyingFlames:
            pass
        class C_DOTA_Ability_Lich_FrostNova:
            pass
        class CDOTA_Modifier_Greevil_Miniboss_Blue_ColdFeet_Freeze:
            pass
        class CDOTA_Modifier_Greevil_Miniboss_Black_Nightmare:
            pass
        class CDOTA_Modifier_JungleVarmint_Dive:
            pass
        class C_DOTA_Ability_PudgeMiniboss_HatefulStrike:
            pass
        class C_DOTA_Ability_Morty_Hop:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Bane_2:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Antimage_6:
            pass
        class CDOTA_Modifier_Special_Bonus_Attack_Range:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Range_125:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Speed_60:
            pass
        class CDOTA_Modifier_Item_HelmOfTheDominator_BonusHealth:
            pass
        class CDOTA_Modifier_Ringmaster_Wheel_Mesmerize:
            pass
        class CDOTA_Modifier_Snapfire_SpitCreep_ArcingUnit:
            pass
        class CDOTA_Modifier_Oracle_DivinersDeck_TheTower:
            pass
        class CDOTA_Modifier_Nyx_Assassin_Nyxth_Sense_Effect:
            pass
        class CDOTA_Modifier_Lycan_FeralImpulse_Aura:
            pass
        class CDOTA_Modifier_Spectre_SpectralDagger:
            pass
        class CDOTA_Modifier_NightStalker_CripplingFear_Aura:
            pass
        class CDOTA_Modifier_DarkSeer_Normal_Punch:
            pass
        class CDOTA_Modifier_Dazzle_NothlProjection_SoulDebuff:
            pass
        class CDOTA_Modifier_Nian_Apocalypse:
            pass
        class CDOTA_Modifier_Zuus_Heavenly_Jump:
            pass
        class CDOTA_Modifier_SkeletonKing_Reincarnation_Scepter_RespawnTime:
            pass
        class CDOTA_Modifier_SetScaleset:
            pass
        class C_DOTA_Unit_Hero_Slark:
            pass
        class C_DOTA_Unit_Hero_SpiritBreaker:
            pass
        class C_DOTA_Item_Tier5Token:
            pass
        class C_RagdollProp:
            pass
        class C_DOTA_Item_AbyssalBlade:
            pass
        class C_DOTA_Ability_Largo_CroakOfGenius:
            pass
        class C_DOTA_Ability_Largo_AmphibianRhapsody_FightSong:
            pass
        class CDOTA_Ability_Snapfire_GobbleUp:
            pass
        class CDOTA_Ability_Snapfire_LilShredder:
            pass
        class C_DOTA_Ability_Invoker_Innate_Mastermind:
            pass
        class C_DOTA_Ability_Enigma_MidnightPulse:
            pass
        class C_DOTA_Ability_Tiny_TossTree:
            pass
        class CDOTA_Modifier_Item_Helm_Of_The_Undying_Active:
            pass
        class C_DotaSubquestBuyItems:
            pass
        class CDOTA_Modifier_Seasonal_TI11_CongaLine:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Shadow_Demon_3:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Warlock_4:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Phantom_Assassin_7:
            pass
        class C_DOTA_Ability_Special_Bonus_Respawn_Reduction_40:
            pass
        class C_DOTA_Ability_Special_Bonus_Intelligence_16:
            pass
        class C_DOTA_Ability_Special_Bonus_Mana_Break_20:
            pass
        class C_DOTA_Ability_Special_Bonus_Gold_lvl20_r:
            pass
        class CDOTA_Modifier_Item_AncientGuardian:
            pass
        class CDOTA_Modifier_Repair_Kit:
            pass
        class CDOTA_Modifier_Smoke_Of_Deceit_Secondary_Application_Cooldown:
            pass
        class CDOTA_Modifier_Marci_Dispose_Debuff:
            pass
        class CDOTA_Modifier_Pangolier_HeartPiercer:
            pass
        class CDOTA_Modifier_Oracle_DivinersDeck_TheTower_Next:
            pass
        class CDOTA_Modifier_DoomBringer_Lvl_Pain_Debuff:
            pass
        class CDOTA_Modifier_PhantomAssassin_MarkOfDeath:
            pass
        class CDOTA_Modifier_Nian_Waterball:
            pass
        class CDOTA_Modifier_Lich_FrostNova_Slow:
            pass
        class C_DOTA_Item_Misericorde:
            pass
        class C_DOTA_Item_Imp_Claw:
            pass
        class C_DOTA_Item_Desolator_2:
            pass
        class C_DOTA_Item_Recipe_Soul_Ring:
            pass
        class C_DOTA_Item_TranquilBoots:
            pass
        class C_DOTA_Item_Flask:
            pass
        class CDOTA_Ability_Pudge_MeatHook:
            pass
        class CDOTA_Modifier_DoNotCastSmash:
            pass
        class CDOTA_Modifier_KoboldTaskmaster_SpeedAura_Bonus:
            pass
        class C_DOTA_Ability_AghsFort_AssaultCaptain_SearingChains:
            pass
        class C_DOTA_Ability_AghsFort_Shadow_Demon_Shadow_Poison_Release:
            pass
        class CDOTA_Ability_Seasonal_Summon_CNY_Tree:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Mirana_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Armor_9:
            pass
        class CDOTA_Modifier_Item_Prophets_Pendulum:
            pass
        class CDOTA_Modifier_Item_Quickening_Charm:
            pass
        class CDOTA_Modifier_Item_Sange:
            pass
        class CDOTA_Modifier_Item_BladeOfAlacrity:
            pass
        class CDOTA_Modifier_Largo_AmphibianRhapsody_Self:
            pass
        class CDOTA_Modifier_PrimalBeast_Onslaught_Knockback:
            pass
        class CDOTA_Modifier_Marci_Special_Delivery_Aura:
            pass
        class CDOTA_Modifier_Elder_Titan_Momentum:
            pass
        class CDOTA_Modifier_Treant_NaturesGuise_Invis:
            pass
        class CDOTA_Modifier_Chaos_Knight_Illusions_Damage_Reduction:
            pass
        class CDOTA_Modifier_Animation_RightClawSwipe:
            pass
        class CDOTA_Modifier_Sniper_Concussive_Grenade:
            pass
        class CDOTA_Modifier_Juggernaut_Healing_Ward_Heal:
            pass
        class CDOTA_Modifier_Pudge_Dismember_Gluttony_Strength_BuffCounter:
            pass
        class CDotaMoveSpeedModifierPath:
            pass
        class C_DOTA_Item_Enhancement_Restorative:
            pass
        class CDOTA_Item_Recipe_Ward_Dispenser:
            pass
        class C_DOTA_Item_MaskOfDeath:
            pass
        class C_DOTA_Ability_Medusa_FixedMovespeed:
            pass
        class C_DOTA_Ability_Disruptor_Thunder_Strike:
            pass
        class C_DOTA_Ability_Silencer_CurseOfTheSilent:
            pass
        class C_DOTA_Ability_ForgedSpirit_MeltingStrike:
            pass
        class CDOTA_Ability_Broodmother_InsatiableHunger:
            pass
        class C_DOTA_Ability_Tidehunter_ArmOfTheDeep:
            pass
        class CDOTA_Modifier_Spawnlord_Master_Freeze:
            pass
        class CDOTA_Modifier_Mutation_CreateTombstone_Aura:
            pass
        class CDOTA_Ability_Aghsfort_Minor_Stats_Upgrade:
            pass
        class C_DOTA_Ability_Special_Bonus_Armor_8:
            pass
        class C_DOTA_Ability_Special_Bonus_Strength_16:
            pass
        class C_DOTA_Ability_Special_Bonus_Intelligence_20:
            pass
        class C_DOTA_Ability_Special_Bonus_Base:
            pass
        class CDOTA_Modifier_Kobold_Cup:
            pass
        class CDOTA_Modifier_Item_Devastator:
            pass
        class CDOTA_Modifier_Item_Disperser_Evasion_Buff:
            pass
        class CDOTA_Modifier_Snapfire_Scatterblast_Disarm:
            pass
        class CDOTA_Modifier_MonkeyKing_FurArmy_SoldierInPosition:
            pass
        class CDOTA_Modifier_NagaSiren_Deluge_Status_Resistance:
            pass
        class CDOTA_Modifier_ChaosKnight_Phantasm_Illusion:
            pass
        class CDOTA_Modifier_LoneDruid_SpiritLink_Active:
            pass
        class CDOTA_Modifier_Invoker_SunStrike_Cataclysm_Thinker:
            pass
        class CDOTA_Modifier_NightStalker_Void_Intrinsic:
            pass
        class CDOTA_Modifier_Leshrac_Pulse_Nova:
            pass
        class CDOTA_Modifier_PoisonNova_Thinker:
            pass
        class C_DOTA_Unit_Hero_Rattletrap:
            pass
        class CDOTA_Item_Recipe_Woodland_Striders:
            pass
        class C_DOTA_Ability_KeeperOfTheLight_Radiant_Bind:
            pass
        class C_DOTA_Ability_Invoker_Wex:
            pass
        class CDOTA_Ability_Alchemist_AcidSpray:
            pass
        class CDOTA_Ability_Dark_Seer_Mental_Fortitude:
            pass
        class CDOTA_Modifier_Tornado_Tempest_Debuff:
            pass
        class CDOTA_Modifier_AghsFort_TrapRoom_Hookshot:
            pass
        class CDOTA_Modifier_Wave_Blast_Disarm:
            pass
        class C_DOTA_Ability_Special_Bonus_Strength_15:
            pass
        class CDOTA_Modifier_Mars_ArenaOfBlood:
            pass
        class CDOTA_Modifier_Techies_Minefield_Sign_Scepter_Aura:
            pass
        class CDOTA_Modifier_CallOfTheWild_Boar_Poison:
            pass
        class CDOTA_Modifier_Tiny_Avalanche_Stun:
            pass
        class CDOTA_Modifier_Zuus_ThundergodsWrathVisionThinker:
            pass
        class CDOTA_Modifier_Tutorial_HideNPC:
            pass
        class C_DOTABaseCustomHeroPickRules:
            pass
        class CPulse_CallInfo:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class C_DOTA_Unit_Hero_BountyHunter:
            pass
        class C_DOTA_Unit_Hero_Beastmaster:
            pass
        class CDOTA_Unit_CustomGameAnnouncer:
            pass
        class C_DOTA_BaseNPC_Creature:
            pass
        class CBaseAnimGraph:
            pass
        class C_DOTA_Item_RippersLash:
            pass
        class C_DOTA_Item_Occult_Bracelet:
            pass
        class C_DOTA_Ability_Necronomicon_Archer_AoE:
            pass
        class CDOTA_Item_Recipe_Battlefury:
            pass
        class CDOTA_Ability_Marci_Guardian:
            pass
        class C_DOTA_Ability_Undying_TombstoneUnitGrab:
            pass
        class CDOTA_Ability_Broodmother_IncapacitatingBite:
            pass
        class CDOTA_Ability_Miniboss_InherentBuffs:
            pass
        class C_DOTA_Ability_Witch_Doctor_GrisGris:
            pass
        class CDOTA_Modifier_BlackDragon_DragonhideAura_Bonus:
            pass
        class CDOTAEmptyAbility:
            pass
        class C_DOTA_Ability_Wisp_Spirits:
            pass
        class CDOTA_Modifier_Aghsfort_Aziyog_Underlord_Firestorm_Burn:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lycan_8:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lich_7:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Zeus_4:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Ogre_Magi_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_20:
            pass
        class C_DOTA_Ability_Special_Bonus_Intelligence_7:
            pass
        class CDOTA_Modifier_Item_DuelistGloves:
            pass
        class CDOTA_Modifier_Item_Headdress_Aura:
            pass
        class CDOTA_Modifier_Item_Yasha:
            pass
        class CDOTA_Modifier_Item_Broadsword:
            pass
        class CDOTA_Modifier_Largo_Frogstomp_Thinker:
            pass
        class CDOTA_Modifier_Ringmaster_Impalement_Bleed:
            pass
        class CDOTA_Modifier_DarkWillow_CursedCrown:
            pass
        class CDOTA_Modifier_Abaddon_BorrowedTime:
            pass
        class CDOTA_Modifier_Meepo_Earthbind_ChainDuration:
            pass
        class CDOTA_Modifier_ChaosKnight_Phantasm:
            pass
        class CDOTA_Modifier_Shadow_Demon_InnateSoulCatcher_HPLoss:
            pass
        class CDOTA_Modifier_Alchemist_Corrosive_Weaponry_Debuff:
            pass
        class CDOTA_Modifier_BountyHunter_Big_Game_Hunter:
            pass
        class CDOTA_Modifier_WitchDoctor_MaledictDoT:
            pass
        class CDOTA_Modifier_Pudge_Meat_Hook_Reveal:
            pass
        class CDOTA_Modifier_AntiMage_Counterspell_Passive:
            pass
        class C_IngameEvent_Base:
            pass
        class CPulseCell_InlineNodeSkipSelector:
            pass
        class CDOTA_BaseNPC_Seasonal_TI9_Drums:
            pass
        class C_LightEntity:
            pass
        class C_DOTA_Item_Titan_Sliver:
            pass
        class CDOTA_Item_Recipe_Lotus_Orb:
            pass
        class C_DOTA_Item_Recipe_Diffusal_Blade:
            pass
        class C_DOTA_Item_RodOfAtos:
            pass
        class CDOTA_Ability_Gyrocopter_Call_Down:
            pass
        class C_DOTA_Ability_Tiny_Toss:
            pass
        class C_DOTA_Ability_AntiMage_Mana_Overload:
            pass
        class C_DotaSubquestAbilityCastCount:
            pass
        class CDOTA_Modifier_Special_Bonus_Haste:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Earthshaker:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Faceless_Void_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_160:
            pass
        class C_DOTA_Ability_Special_Bonus_Movement_Speed_50:
            pass
        class CPulseGraphInstance_PanoramaPanel:
            pass
        class CDOTA_Modifier_Ethereal_Blade_Ethereal:
            pass
        class CDOTA_Modifier_Largo_Frogling:
            pass
        class CDOTA_Modifier_MonkeyKing_FurArmy_BonusDamage:
            pass
        class CDOTA_Modifier_Abyssal_Underling_Archer_AoE:
            pass
        class CDOTA_Modifier_Techies_LandMines_ChargeCounter:
            pass
        class CDOTA_Modifier_Shredder_Chakram_Debuff:
            pass
        class CDOTA_Modifier_Omniknight_Pacify:
            pass
        class CDOTA_Modifier_Clinkz_DeathPact:
            pass
        class CDOTA_Modifier_Tidehunter_KrakenShell:
            pass
        class CDOTA_Modifier_Lina_Combustion:
            pass
        class CDOTA_Modifier_CrystalMaiden_Frostbite:
            pass
        class CDOTA_Modifier_Pudge_Meat_Hook:
            pass
        class CDOTA_Modifier_Followthrough:
            pass
        class CDOTA_Modifier_Tower_Aura:
            pass
        class CDOTA_AbilityDraftHeroState:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Unit_Hero_Abaddon:
            pass
        class C_DOTA_Unit_TargetDummy:
            pass
        class C_LocalTempEntity:
            pass
        class CDOTA_Item_Panic_Button:
            pass
        class C_DOTA_Item_Courier:
            pass
        class C_DOTA_Item_Necronomicon:
            pass
        class CDOTA_Ability_Elder_Titan_FundamentalFury_Spirit:
            pass
        class C_DOTA_Ability_KeeperOfTheLight_IlluminateEnd:
            pass
        class C_DOTA_Ability_Weaver_GeminateAttack:
            pass
        class CDOTA_Modifier_AghsFort_Ascension_MagneticField_Thinker_Evasion:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_VoidSpirit_8:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Skywrath_4:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Slardar_4:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Windranger_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Pangolier:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Amplify_12:
            pass
        class C_DOTA_LightInfo:
            pass
        class CDOTA_Modifier_Item_Trickster_Cloak:
            pass
        class CDOTA_Modifier_Demonicon_Bonus:
            pass
        class CDOTA_Modifier_Item_Tenderizer:
            pass
        class CDOTA_Modifier_Muerta_Gunslinger:
            pass
        class CDOTA_Modifier_Silencer_Oppressive_Silence:
            pass
        class CDOTA_Modifier_Warlock_Imp_ExplodeThinker:
            pass
        class CDOTA_Modifier_Item_GrisGris:
            pass
        class CDOTA_Modifier_Zuus_Static_Field_Slow:
            pass
        class C_DOTA_Unit_Roshan:
            pass
        class C_DOTA_Item_Heavy_Blade:
            pass
        class CDOTA_Item_Recipe_The_Leveller:
            pass
        class CDOTA_Item_Medallion_Of_Courage:
            pass
        class C_DOTA_Item_Recipe_Revenants_Brooch:
            pass
        class C_DOTA_Ability_DarkWillow_ShadowRealm:
            pass
        class C_DOTA_Ability_Lina_FierySoul:
            pass
        class C_DOTA_Ability_CrystalMaiden_Let_It_Go:
            pass
        class CDOTA_Modifier_Frogmen_WaterBubble:
            pass
        class CDOTA_Modifier_JungleVarmint_Creator:
            pass
        class C_PointEntity:
            pass
        class C_DOTA_Ability_Special_Bonus_Cast_Range_125:
            pass
        class C_DOTA_Ability_Special_Bonus_Magic_Resistance_14:
            pass
        class C_DOTA_Ability_Special_Bonus_Agility_5:
            pass
        class CDOTA_Modifier_Item_Enhancement_Hulking:
            pass
        class CDOTA_Modifier_Item_SistersShroud_Buff:
            pass
        class CDOTA_Modifier_Item_Polliwog_Charm_Buff:
            pass
        class CDOTA_Modifier_Kez_FalconRush_Echo_Damage:
            pass
        class CDOTA_Modifier_Kez_Katana_Bleed:
            pass
        class CDOTA_Modifier_Primal_Beast_Uproar_Projectile_Thinker:
            pass
        class CDOTA_Modifier_Dawnbreaker_Hearthfire_Thinker:
            pass
        class CDOTA_Modifier_Mars_Gods_Rebuke:
            pass
        class CDOTA_Modifier_Grimstroke_Ink_Trail_Debuff:
            pass
        class CDOTA_Modifier_Invoke_Bonuses:
            pass
        class CDOTA_Modifier_Broodmother_SpinWeb_Degen:
            pass
        class CDOTA_Modifier_FacelessVoid_Chronosphere:
            pass
        class CDOTA_Modifier_Courier_Shield:
            pass
        class CDOTA_Modifier_Tinker_Turret:
            pass
        class CDOTA_Modifier_VengefulSpirit_Revenge:
            pass
        class CDOTA_Modifier_Rune_Haste:
            pass
        class C_SingleplayRules:
            pass
        class CLogicalEntity:
            pass
        class C_DOTA_Item_Diffusal_Blade_Level2:
            pass
        class C_DOTA_Item_CraniumBasher:
            pass
        class C_DOTA_Item_JumpBoots:
            pass
        class C_DOTA_Ability_Courier_QueuePickupFromStash:
            pass
        class C_DOTA_Ability_Necrolyte_Heartstopper_Aura:
            pass
        class C_DOTA_Ability_WitchDoctor_Maledict:
            pass
        class CDOTA_Modifier_XP_Fountain:
            pass
        class CDOTA_Modifier_Mutation_NoHealthBars_Aura:
            pass
        class C_DOTA_Ability_Zombie_Berserk:
            pass
        class CDOTA_Ability_Seasonal_PartyHat:
            pass
        class C_DOTA_Ability_Seasonal_Decorate_Tree:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Slardar_3:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Antimage_7:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Dragon_Knight_3:
            pass
        class C_DOTA_Ability_Special_Bonus_50_Crit_40:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_400:
            pass
        class CDOTA_Modifier_Item_Craggy_Coat_Tanky:
            pass
        class CDOTA_Modifier_Item_Mjollnir_Static:
            pass
        class CDOTA_Modifier_Item_Shivas_Guard_Aura:
            pass
        class CDOTA_Modifier_Hoodwink_TomokanTracker_Tracks:
            pass
        class CDOTA_Modifier_Mars_Bulwark_Soldier_Bonus:
            pass
        class CDOTA_Modifier_Techies_Arcana_DamageListener:
            pass
        class CDOTA_Modifier_Slark_EssenceShift_Debuff:
            pass
        class CDOTA_Modifier_Chen_DivineFavor:
            pass
        class CDOTA_Modifier_Leshrac_Greater_Lightning_Storm:
            pass
        class CDOTA_Modifier_Venomancer_NoxiousPlaguePrimary:
            pass
        class CDOTA_Modifier_Courier_ReturnStashItems:
            pass
        class CDOTA_Modifier_Axe_BerserkersCallArmor:
            pass
        class CDOTA_Modifier_VengefulSpirit_Nether_Swap_PathingFix:
            pass
        class CDOTA_Modifier_VengefulSpirit_Command_Negative_Aura:
            pass
        class CDOTA_Muerta_Revenant:
            pass
        class C_DOTA_Unit_Hero_ChaosKnight:
            pass
        class C_DOTA_BaseNPC_Seasonal_Snowman:
            pass
        class C_DOTA_Item_Cyclone:
            pass
        class CDOTA_Ability_Tusk_IceShards_Stop:
            pass
        class CDOTA_Ability_DoomBringer_Devils_Bargain:
            pass
        class CDOTA_Ability_Lina_Combustion:
            pass
        class CDOTA_Modifier_EnragedWildkin_ToughnessAura:
            pass
        class CDOTA_Modifier_Twin_Gate_Warp_Channel:
            pass
        class C_PrecipitationBlocker:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Furion_5:
            pass
        class CDOTA_Modifier_Special_Bonus_Lifesteal:
            pass
        class CPulseCell_WaitForPanelClass:
            pass
        class CDOTA_Modifier_Item_OgreSealTotem:
            pass
        class CDOTA_Modifier_Item_Overwhelming_Blink:
            pass
        class CDOTA_Modifier_Terrorblade_Demon_Zeal:
            pass
        class CDOTA_Modifier_AbyssalUnderlord_Firestorm_Thinker:
            pass
        class CDOTA_Modifier_Abaddon_Borrowed_Time_Damage_Redirect:
            pass
        class CDOTA_Modifier_Gyrocopter_HomingMissile_ChargeCounter:
            pass
        class CDOTA_Modifier_FacelessVoid_TimeWalk_Slow:
            pass
        class CDOTA_Modifier_Tiny_Grow:
            pass
        class CDOTA_Modifier_StormSpirit_StaticRemnantThinker:
            pass
        class CDOTA_Buff_Item:
            pass
        class C_SoundOpvarSetPathCornerEntity:
            pass
        class CPlayer_WeaponServices:
            pass
        class CDOTA_Unit_Hero_Kez:
            pass
        class C_DOTA_Unit_Hero_Techies:
            pass
        class C_DOTA_Unit_Hero_Spectre:
            pass
        class C_DOTA_Unit_Hero_DragonKnight:
            pass
        class C_DOTA_Item_Orb_Of_Revelations:
            pass
        class CDOTA_Item_Recipe_Keen_Optic:
            pass
        class C_DOTA_Ability_Muerta_Gunslinger:
            pass
        class C_DOTA_Ability_Pangolier_Rollup:
            pass
        class CDOTA_Ability_Centaur_Return:
            pass
        class CDOTA_Ability_Ursa_Innate_Maul:
            pass
        class C_DOTA_Item_DataDriven:
            pass
        class C_DOTA_Ability_Frogmen_ArmOfTheDeep:
            pass
        class C_DOTA_Ability_Greevil_Miniboss_Blue_ColdFeet:
            pass
        class C_DOTA_Ability_Special_Bonus_Armor_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Strength_30:
            pass
        class C_DOTA_Ability_Special_Bonus_Gold_lvl25_l:
            pass
        class CDOTA_Modifier_Item_Satanic:
            pass
        class CDOTA_Modifier_Item_Hyperstone:
            pass
        class CDOTA_Modifier_Hoodwink_ArcingBoomerang:
            pass
        class CDOTA_Modifier_MonkeyKing_Spring_Thinker:
            pass
        class CDOTA_Modifier_LegionCommander_Intimidate_DamageReduction:
            pass
        class CDOTA_Modifier_Legion_Commander_PressTheAttack_Immunity:
            pass
        class CDOTA_Modifier_Skywrath_Mystic_Flare_Aura_Effect:
            pass
        class CDOTA_Modifier_Wisp_Tether:
            pass
        class CDOTA_Modifier_Treant_Overgrowth:
            pass
        class CDOTA_Modifier_Silencer_BrainDrain:
            pass
        class CDOTA_Modifier_Beastmaster_PrimalRoar_Speed:
            pass
        class CDOTA_Modifier_Earthshaker_Slugger_Intrinsic:
            pass
        class C_DOTA_Unit_Hero_VengefulSpirit:
            pass
        class CDOTA_Item_Foragers_Mana:
            pass
        class C_DOTA_Item_Enhancement_Boundless:
            pass
        class C_DOTA_Item_RiverPainter2:
            pass
        class C_DOTA_Item_Javelin:
            pass
        class C_DOTA_Ability_Pangolier_Gyroshell:
            pass
        class C_DOTA_Ability_Bristleback_BrawlersGrit:
            pass
        class C_DOTA_Ability_Enchantress_Bunny_Hop:
            pass
        class CDOTA_Ability_Furion_Natures_Profit:
            pass
        class C_DOTA_Ability_AntiMage_ManaBreak:
            pass
        class CDOTA_Modifier_Mutation_Crit_Chance:
            pass
        class C_TriggerVolume:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Windranger_2:
            pass
        class C_DOTA_Ability_Special_Bonus_Gold_Income_30:
            pass
        class CDOTA_Modifier_Item_Enhancement_Thick:
            pass
        class CDOTA_Modifier_Item_UnrelentingEye:
            pass
        class CDOTA_Modifier_Item_Orb_Of_Destruction:
            pass
        class CDOTA_Modifier_Item_Spirit_Vessel_Damage:
            pass
        class CDOTA_Modifier_Item_Armlet:
            pass
        class CDOTA_Modifier_Item_DemonEdge:
            pass
        class CDOTA_Modifier_Hoodwink_HeavyQuiver:
            pass
        class CDOTA_Modifier_Pangolier_LuckyShot:
            pass
        class CDOTA_Modifier_ArcWarden_Runic_Infusion:
            pass
        class CDOTA_Modifier_Visage_GravekeepersCloak_Stack:
            pass
        class CDOTA_Modifier_LoneDruid_TrueForm_Transform:
            pass
        class CDOTA_Modifier_Brewmaster_Void_AstralPulse:
            pass
        class CDOTA_Modifier_Clinkz_Tar_Bomb_Slow:
            pass
        class CDOTA_Modifier_Pugna_NetherWard:
            pass
        class CDOTA_Modifier_Beastmaster_Hawk_Perch_Flight:
            pass
        class CDOTA_Modifier_Firecracker_Debuff:
            pass
        class CDOTA_Modifier_Zuus_ThunderTrail_Debuff:
            pass
        class C_DOTA_Unit_Hero_Lycan:
            pass
        class C_DOTA_Unit_Hero_Morphling:
            pass
        class C_DOTA_Item_Enhancement_Wise:
            pass
        class CDOTA_Item_Recipe_Pavise:
            pass
        class C_DOTA_Item_Crimson_Guard:
            pass
        class C_DOTA_Ability_NyxAssassin_NeuroSting:
            pass
        class C_DOTA_Ability_Tinker_MarchOfTheMachines:
            pass
        class C_DOTA_Ability_Creep_Irresolute:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_Regen_6:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Speed_20:
            pass
        class CDOTA_Modifier_Magnifying_Monocle:
            pass
        class CDOTA_Modifier_Item_Medallion_Of_Courage_Armor_Reduction:
            pass
        class CDOTA_Modifier_Broodmother_Spiders_Milk:
            pass
        class CDOTA_Modifier_Phantom_Assassin_Blur_Manacost_Reduction:
            pass
        class CDOTA_Modifier_Turbo_Courier_Invulnerable:
            pass
        class CDOTA_Modifier_WitchDoctor_DeathWard_SecondaryAttack:
            pass
        class CPulseCell_LimitCount:
            pass
        class CPulseCell_Step_CallExternalMethod:
            pass
        class C_DOTA_Unit_Hero_FacelessVoid:
            pass
        class C_DynamicPropAlias_prop_dynamic_override:
            pass
        class CDOTA_Item_Ward_Maker:
            pass
        class C_DOTA_Item_SheepStick:
            pass
        class C_DOTA_Ability_Largo_AmphibianRhapsody_Song:
            pass
        class C_DOTA_Ability_Phoenix_SunRay:
            pass
        class C_DOTA_Ability_SpiritBreaker_ChargeOfDarkness:
            pass
        class C_DOTA_Ability_Tinker_Innate_Keen_Teleport_XP_On_Death:
            pass
        class CEnvSoundscapeTriggerable:
            pass
        class CDOTA_Ability_AghsFort_Creature_Phoenix_Supernova:
            pass
        class CDOTA_Modifier_AghsFort_Creature_Phoenix_FireSpiritCount:
            pass
        class CDOTA_Modifier_AghsFort_StonehallGeneral_OverwhelmingOdds_Thinker:
            pass
        class CDOTA_Ability_Seasonal_TI9_Shovel:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Nyx_5:
            pass
        class CDOTA_Modifier_Special_Bonus_Agility_And_Intelligence:
            pass
        class C_DOTA_Ability_Special_Bonus_Magic_Resistance_25:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_Regen_3:
            pass
        class CDOTA_Modifier_Item_Force_Field_Effect:
            pass
        class CDOTA_Modifier_Dagon:
            pass
        class CDOTA_Modifier_Magnataur_Skewer_Movement:
            pass
        class CDOTA_Modifier_TrollWarlord_WhirlingAxes_Slow:
            pass
        class CDOTA_Modifier_Gyrocopter_Afterburner:
            pass
        class CDOTA_Modifier_Weaver_Threads_Of_Fate_Linger:
            pass
        class CDOTA_Modifier_Nian_Hurricane_Whirlpool:
            pass
        class CDOTA_Modifier_Crystal_Maiden_Crystal_Clone:
            pass
        class CDOTA_Modifier_Juggernaut_BladeFury_Pull:
            pass
        class C_DOTA_BaseNPC_Venomancer_PlagueWard:
            pass
        class C_DOTA_GuildBannerProp:
            pass
        class C_DOTA_Item_Dezun_Bloodrite:
            pass
        class C_DOTA_Item_Safety_Bubble:
            pass
        class C_DOTA_Item_Stormcrafter:
            pass
        class C_DOTA_Item_Desolator:
            pass
        class C_DOTA_Item_Blade_Mail:
            pass
        class C_DOTA_Ability_MonkeyKing_Spring:
            pass
        class C_DOTA_Ability_EarthSpirit_StoneCaller:
            pass
        class C_DOTA_Ability_Medusa_ManaShield:
            pass
        class C_DOTA_Ability_Lycan_Howl:
            pass
        class C_DOTA_Ability_BountyHunter_WindWalk_Ally:
            pass
        class C_DOTA_Ability_Clinkz_Burning_Army:
            pass
        class C_DOTA_Ability_Leshrac_Pulse_Nova:
            pass
        class CDOTA_Ability_CrystalMaiden_IceRink:
            pass
        class C_DOTA_Ability_Juggernaut_Omnislash:
            pass
        class CDOTA_Modifier_AlphaWolf_CriticalStrike:
            pass
        class CDOTA_Ability_Seasonal_Firecrackers:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Pugna_1:
            pass
        class CDOTA_Modifier_Special_Bonus_Intelligence:
            pass
        class C_DOTA_Ability_Special_Bonus_Lifesteal_25:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_12:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Speed_55:
            pass
        class CDOTA_Modifier_Item_Mekansm_Noheal:
            pass
        class CDOTA_Modifier_SheepStick_Debuff:
            pass
        class CDOTA_Modifier_Pangolier_Gyroshell_Stun:
            pass
        class CDOTA_Modifier_Winter_Wyvern_Accelerated_Learning:
            pass
        class CDOTA_Modifier_EmberSpirit_SleightOfFist_InProgress:
            pass
        class CDOTA_Modifier_Obsidian_Destroyer_ManaCost:
            pass
        class CDOTA_Modifier_DoomBringer_ScorchedEarthEffectAura:
            pass
        class CDOTA_Modifier_Enchantress_Little_Friends:
            pass
        class CDOTA_Modifier_Life_Stealer_Open_Wounds:
            pass
        class CDOTA_Modifier_Viper_Predator:
            pass
        class CDOTA_Modifier_DeathProphet_SpiritSiphon_Fear:
            pass
        class CDOTA_Modifier_Roshan_RevengeRoar:
            pass
        class CDOTA_Modifier_CDOTA_Ability_Axe_One_Man_Army:
            pass
        class CDOTA_Modifier_VengefulSpirit_XP_Earn:
            pass
        class CDOTA_Modifier_Silence:
            pass
        class CDOTA_Modifier_ModelScaleAnimateTime:
            pass
        class CDOTA_Modifier_StackedNeutral:
            pass
        class C_DOTA_Unit_Hero_Omniknight:
            pass
        class CDOTA_Item_Essence_Ring:
            pass
        class C_DOTA_Ability_Meepo_Geomancy:
            pass
        class C_DOTA_Ability_QueenOfPain_Innate_Seduction:
            pass
        class CDOTA_Ability_Pudge_Eject:
            pass
        class CDOTA_Modifier_GiantWolf_Intimidate:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Centaur_5:
            pass
        class CDOTA_Modifier_Special_Bonus_Corruption_Debuff:
            pass
        class CDOTA_Modifier_Falconers_Glove:
            pass
        class CDOTA_Modifier_Orb_Of_Corrosion_Debuff:
            pass
        class CDOTA_Modifier_Item_GemOfTrueSight:
            pass
        class CDOTA_Modifer_Item_GlovesOfHaste:
            pass
        class CDOTA_Modifier_Pangolier_HeartPiercer_Debuff:
            pass
        class CDOTA_Modifier_Razor_EyeOfTheStorm_Passive:
            pass
        class CDOTA_Modifier_Razor_StaticLink_Debuff:
            pass
        class CHeroStatueLiked:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CSkyboxReference:
            pass
        class C_PortraitWorldPet:
            pass
        class C_DOTA_Ability_Terrorblade_Sunder:
            pass
        class C_DOTA_Ability_AbyssalUnderlord_Dark_Portal:
            pass
        class CDOTA_Ability_Roshan_Bash:
            pass
        class C_DOTA_Ability_Kunkka_Torrent_Storm:
            pass
        class C_DOTA_Ability_BlackDragon_Fireball:
            pass
        class C_DOTA_Ability_MonkeyKing_FurArmy:
            pass
        class CDOTA_Modifier_AghsFort_Ascension_Silence_Charge:
            pass
        class C_DOTA_Ability_Creature_Ice_Breath:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Medusa_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Amplify_20:
            pass
        class C_DOTA_Ability_Special_Bonus_Cleave_175:
            pass
        class C_DOTA_Ability_Special_Bonus_Corruption_4:
            pass
        class CDOTA_Modifier_Item_Harpoon_Slow:
            pass
        class CDOTA_Modifier_Item_Dragon_Scale:
            pass
        class CDOTA_Modifier_Item_Soul_Ring:
            pass
        class CDOTA_Modifier_Dawnbreaker_Luminosity_Attack_Buff:
            pass
        class CDOTA_Modifier_Wisp_Tether_Scepter:
            pass
        class CDOTA_Modifier_Invoker_Magic_Amp_Debuff:
            pass
        class CDOTA_Modifier_Omniknight_GuardianAngel_Self_Buff:
            pass
        class CDOTA_Modifier_TemplarAssassin_Trap_Slow:
            pass
        class CDOTA_Modifier_FacelessVoid_TimeDilation_Slow:
            pass
        class CDOTA_Modifier_Nian_Frenzy:
            pass
        class CDOTA_Modifier_Morphling_ScepterStatsDrain_All_Buff:
            pass
        class CDOTA_Modifier_Tutorial_ForceAnimation:
            pass
        class CDOTA_Item_Recipe_Moonshard:
            pass
        class C_DOTA_Item_SangeAndYasha:
            pass
        class C_DOTA_Item_Recipe_CraniumBasher:
            pass
        class C_DOTA_Ability_Wisp_Spirits_Out:
            pass
        class C_DOTA_Ability_Ogre_Magi_DumbLuck:
            pass
        class C_DOTA_Ability_Treant_NaturesGrasp:
            pass
        class C_DOTA_Ability_Gyrocopter_Homing_Missile:
            pass
        class C_DOTA_Ability_Huskar_Life_Break:
            pass
        class C_DOTA_Ability_DeathProphet_SpiritCollector:
            pass
        class C_DOTA_Ability_Puck_EtherealJaunt:
            pass
        class CDOTA_Ability_Axe_BerserkersCall:
            pass
        class CFilterClass:
            pass
        class C_PointCameraVFOV:
            pass
        class C_PointCamera:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Chen_2:
            pass
        class CDOTA_Modifier_Special_Bonus_Spell_Immunity:
            pass
        class C_DOTA_Ability_Special_Bonus_Mana_Break_25:
            pass
        class CDOTA_Modifier_Light_Collector:
            pass
        class CDOTA_Modifier_Necronomicon_Archer_AoE:
            pass
        class CDOTA_Modifier_Item_UltimateScepter:
            pass
        class CDOTA_Modifier_Item_MithrilHammer:
            pass
        class CDOTA_Modifier_Dawnbreaker_Solar_Guardian_Thinker:
            pass
        class CDOTA_Modifier_Bristleback_QuillSpray:
            pass
        class CDOTA_Modifier_Centaur_Hitched_Into_Cart:
            pass
        class CDOTA_Modifier_Lycan_SummonWolves_Bash:
            pass
        class CDOTA_Modifier_Invoker_Wex_Intrinsic:
            pass
        class CDOTA_Modifier_Clinkz_Burning_Army:
            pass
        class CDOTA_Modifier_DeathProphet_SpiritSiphon_Buff:
            pass
        class CDOTA_Modifier_Venomancer_WardCounter:
            pass
        class CDOTA_Modifier_Nian_Whirlpool_Pull:
            pass
        class CDOTA_Modifier_Lich_ChainFrost_OnDeath:
            pass
        class CDOTA_Modifier_VengeulSpirit_SoulStrike:
            pass
        class C_DOTA_PhantomAssassin_Gravestone:
            pass
        class C_DOTA_Item_Recipe_Naginata:
            pass
        class C_DOTA_Item_Mekansm:
            pass
        class C_DOTA_Ability_EarthSpirit_GeomagneticGrip:
            pass
        class C_DOTA_Ability_Meepo_Innate_PackRat:
            pass
        class CDOTA_Modifier_Greevil_Miniboss_Sight:
            pass
        class C_DOTA_Ability_BigThunderLizard_Frenzy:
            pass
        class C_DOTA_Ability_CentaurKhan_EnduranceAura:
            pass
        class CPathWithDynamicNodes:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Muerta_TrickShotCharges:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Death_Prophet_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Cast_Range_325:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Speed_160:
            pass
        class CPanel2DAPI:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Modifer_Item_Blitz_Knuckles:
            pass
        class CDOTA_Modifier_EarthSpirit_StoneCaller_Innate:
            pass
        class CDOTA_Modifier_Techies_SnareTrap_Slowed:
            pass
        class CDOTA_Modifier_Magnataur_Shockwave:
            pass
        class CDOTA_Modifier_Magnataur_Shockwave_Erupt:
            pass
        class CDOTA_Modifier_NagaSiren_MirrorImage:
            pass
        class CDOTA_Modifier_Undying_Tombstone_Zombie_Modifier:
            pass
        class CDOTA_Modifier_ChaosKnight_Reality_Rift_Debuff:
            pass
        class CDOTA_Modifier_Silencer_GlaivesOfWisdom_Debuff_Bonus:
            pass
        class CDOTA_Modifier_SpiritBreaker_KnockbackAmpAura:
            pass
        class CDOTA_Modifier_BountyHunter_TrackEffect:
            pass
        class CDOTA_Modifier_CrystalMaiden_CrystalNova_Barrier:
            pass
        class C_DOTA_Item_Recipe_Vambrace:
            pass
        class C_DOTA_Ability_Primal_Beast_Innate_Status_Resistance_Per_Time:
            pass
        class C_DOTA_Ability_Invoker_ChaosMeteor_AD:
            pass
        class CBaseFilter:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_VoidSpirit_4:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Vengeful_Spirit_9:
            pass
        class C_DOTA_Ability_Special_Bonus_Gold_Income_210:
            pass
        class C_DOTA_Ability_Special_Bonus_Magic_Resistance_50:
            pass
        class C_DOTAAppCheers:
            pass
        class CDOTA_Modifier_Item_Angels_Demise_Slow:
            pass
        class CDOTA_Modifier_Item_Havoc_Hammer:
            pass
        class CDOTA_Modifier_Item_Butterfly_Extra:
            pass
        class CDOTA_Modifier_Visage_SummonFamiliars_StoneForm_Thinker:
            pass
        class CDOTA_Modifier_DarkSeer_WallOfReplica_Slow:
            pass
        class CDOTA_Modifier_NianChargePinned:
            pass
        class CDOTA_Modifier_SkeletonKing_SpectralBlade_Curse:
            pass
        class PulseObservableBoolExpression_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class C_DOTA_Unit_Hero_Phoenix:
            pass
        class C_DOTA_Unit_Hero_Bloodseeker:
            pass
        class C_PortraitWorldLight:
            pass
        class C_PointEntityAlias_info_target_portrait_root:
            pass
        class CDOTA_Item_Recipe_Trident:
            pass
        class CDOTA_Ability_Marci_Bodyguard:
            pass
        class CDOTA_Ability_Centaur_Stampede:
            pass
        class C_DOTA_Ability_Keeper_of_the_Light_ManaMagnifier:
            pass
        class C_DOTA_Ability_Undying_FleshGolem:
            pass
        class C_DOTA_Ability_Windrunner_EasyBreezy:
            pass
        class C_DOTA_Ability_ForestTrollHighPriest_Heal:
            pass
        class CDOTA_Modifier_Watcher_State:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Venomancer_5:
            pass
        class CDOTA_Modifier_Special_Bonus_Armor:
            pass
        class C_DOTA_Ability_Special_Bonus_Strength_5:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_475:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_375:
            pass
        class CDOTA_Modifier_Item_Falcon_Blade_Mana_Stack:
            pass
        class CDOTA_Modifier_Item_Essence_Distiller:
            pass
        class CDOTA_Modifier_AghanimsScepter_WaitForUpgradeSelected:
            pass
        class CDOTA_Modifier_Ringmaster_Wheel_Knockback:
            pass
        class CDOTA_Modifier_NagaSiren_Crit_Passive:
            pass
        class CDOTA_Modifier_Enchantress_RabbleRouser_Aura:
            pass
        class CDOTA_Modifier_Miniboss_Alleviation_Active:
            pass
        class CDOTA_Modifier_Lina_Fiery_Cloak:
            pass
        class CDOTA_Modifier_DrowRanger_FrostArrows:
            pass
        class C_DOTA_Unit_Broodmother_Spiderling:
            pass
        class C_DOTA_BaseNPC_HallofFame:
            pass
        class CDOTA_Minesweeper_EffectsEntity:
            pass
        class C_DOTA_Item_SpecialistsArray:
            pass
        class C_DOTA_Item_Recipe_Psychic_Headband:
            pass
        class C_DOTA_Ability_Necronomicon_Archer_ManaBurn:
            pass
        class C_DOTA_Ability_Invoker_GhostWalk_AD:
            pass
        class C_DOTA_Ability_DragonKnight_BreatheFire:
            pass
        class C_DOTA_Ability_Luna_LunarOrbit:
            pass
        class C_DOTA_Ability_Enigma_Malefice:
            pass
        class C_DOTA_Ability_Bane_Nightmare:
            pass
        class C_DOTA_Ability_Greevil_Miniboss_Black_Nightmare:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Luna_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_70:
            pass
        class C_DOTA_Ability_Special_Bonus_Intelligence_25:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_Regen_250:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_225:
            pass
        class CDOTA_Modifier_Item_ManaDraught:
            pass
        class CDOTA_Modifier_Item_DragonLance:
            pass
        class CDOTA_Modifier_Item_Mjollnir:
            pass
        class CDOTA_Modifier_Kez_TalonToss_Silence:
            pass
        class CDOTA_Modifier_Marci_Lunge_Arc:
            pass
        class CDOTA_Modifier_Phoenix_SupernovaHiding:
            pass
        class CDOTA_Modifier_Abaddon_BorrowedTime_ImmolationDamage:
            pass
        class CDOTA_Modifier_Tusk_Snowball_Target:
            pass
        class CDOTA_Modifier_Visage_GravekeepersCloak:
            pass
        class CDOTA_Modifier_Silencer_GlaivesOfWisdom_Debuff:
            pass
        class CDOTA_Modifier_Chen_HandOfGod_Hot:
            pass
        class CDOTA_Modifier_Dazzle_NothlProjection_Knockback:
            pass
        class CDOTA_Modifier_AntiMage_Blink_Illusion:
            pass
        class C_DOTA_Ability_Clinkz_BurningBarrage:
            pass
        class C_DOTA_Ability_Templar_Assassin_Third_Eye:
            pass
        class C_DOTA_Ability_Courier_GoToSideShop:
            pass
        class CDOTA_Modifier_Creep_Bonus_XP:
            pass
        class CDOTA_Modifier_AghsFort_TorrentEffectPotion_Thinker:
            pass
        class CDOTA_Modifier_Morty_Hop_Controller:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Snapfire_5:
            pass
        class CDOTA_Modifier_Item_MartyrsPlate_Aura:
            pass
        class CDOTA_Modifier_Item_Sphere_Target_Enemy:
            pass
        class CDOTA_Modifier_Kez_GrapplingClaw_LandingAnim:
            pass
        class CDOTA_Modifier_Pangolier_Swashbuckle_ChargeCounter:
            pass
        class CDOTA_Modifier_AbyssalUnderlord_AtrophyAura_Scepter:
            pass
        class CDOTA_Modifier_Troll_Warlord_Rampage:
            pass
        class CDOTA_Modifier_PhantomAssassin_Gravestone:
            pass
        class CDOTA_Modifier_DeathProphet_SpiritCollector:
            pass
        class CDOTA_Modifier_Morphling_Syntropy:
            pass
        class CEntityIdentity:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CPulseCell_LimitCount__Criteria_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Unit_Underlord_Portal:
            pass
        class C_DOTA_Item_WraithPact:
            pass
        class C_DOTA_Ability_MonkeyKing_TreeDance:
            pass
        class C_DOTA_Ability_Skywrath_Mage_Mystic_Flare:
            pass
        class C_DOTA_Ability_Ogre_Magi_Bloodlust:
            pass
        class C_DOTA_Ability_Ogre_Magi_Ignite:
            pass
        class C_DOTA_Ability_Bear_Empty2:
            pass
        class C_DOTA_Ability_SpiritBreaker_PlanarPocket:
            pass
        class C_DOTA_Ability_Venomancer_NoxiousPlague:
            pass
        class C_DOTA_Ability_Windrunner_GaleForce:
            pass
        class C_DOTA_Ability_Zuus_Lightning_Hands:
            pass
        class CDOTA_Modifier_Mutation_Vampire_Aura:
            pass
        class CDOTA_Modifier_Seasonal_TI11_BubbleGun:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Base_Damage_30:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Speed_80:
            pass
        class C_DOTA_Ability_Special_Bonus_MP_1000:
            pass
        class CDOTA_Modifier_Item_Slippers:
            pass
        class CDOTA_Modifier_Ringmaster_TameTheBeasts_Thinker:
            pass
        class CDOTA_Modifier_PrimalBeast_Pulverize:
            pass
        class CDOTA_Modifier_Dawnbreaker_Solar_Guardian_LandingStun:
            pass
        class CDOTA_Modifier_Snapfire_LilShredder_Attack:
            pass
        class CDOTA_Modifier_Grimstroke_InkCreature:
            pass
        class CDOTA_Modifier_Pangolier_GyroshellTimeout:
            pass
        class CDOTA_Modifier_Winter_Wyvern_Winters_Curse_Kill_Credit:
            pass
        class CDOTA_Modifier_BlindingLight_Knockback:
            pass
        class CDOTA_Modifier_PhantomAssassin_ScreenSplatter:
            pass
        class CDOTA_Modifier_Enigma_BlackHole_Pull:
            pass
        class CDOTA_Modifier_Morphling_ScepterStatsDrain_Intelligence_Debuff:
            pass
        class CDOTA_Modifier_Bane_FiendsGrip_Illusion_Can_Only_Channel:
            pass
        class CDOTA_Modifier_DataDriven:
            pass
        class C_DOTA_Unit_Hero_Broodmother:
            pass
        class C_DOTA_Item_EchoSabre:
            pass
        class C_DOTA_Item_TierToken:
            pass
        class C_DOTA_Ability_Largo_AmphibianRhapsody_DoubleTime:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Hoodwink_Bushwhack_Duration:
            pass
        class CDOTA_Ability_Winter_Wyvern_Arctic_Burn:
            pass
        class CDOTA_Ability_Nyx_Assassin_Jolt:
            pass
        class C_DOTA_Ability_Undying_Tombstone:
            pass
        class C_DOTA_Ability_PhantomAssassin_PhantomStrike:
            pass
        class CDOTA_Ability_Puck_Puckish:
            pass
        class C_DOTA_Ability_PhantomLancer_SpiritLance:
            pass
        class CDOTA_Modifier_FelBeast_Haunt:
            pass
        class CDOTA_Modifier_Neutral_SpellImmunity_Visible:
            pass
        class C_DOTA_PushWave:
            pass
        class CDOTA_Modifier_Mutation_Treecutter:
            pass
        class CBasePlayerVData:
            pass
        class CDOTA_Modifier_Special_Bonus_Gold_Income:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Visage_5:
            pass
        class CDOTA_Modifier_Item_Enhancement_Audacious:
            pass
        class CDOTA_Modifier_Item_Paladin_Sword:
            pass
        class CDOTA_Modifier_Ringmaster_StrongmanTonic_ScaleSpeed:
            pass
        class CDOTA_Modifier_Muerta_Ofrenda_DamageAmp:
            pass
        class CDOTA_Modifier_Snapfire_Buckshot:
            pass
        class CDOTA_Modifier_Terrorblade_Metamorphosis_Fear_Thinker:
            pass
        class CDOTA_Modifier_LegionCommander_Intimidate:
            pass
        class CDOTA_Modifier_Magnataur_ReversePolarity:
            pass
        class CDOTA_Modifier_Omniknight_Repel:
            pass
        class CDOTA_Modifier_Item_Vermillion_Robe_Flames:
            pass
        class CDOTA_Modifier_Riki_SmokeScreen:
            pass
        class CDOTA_BaseNPC_Seasonal_Dragon:
            pass
        class C_LightSpotEntity:
            pass
        class C_DOTA_Item_HeavensHalberd:
            pass
        class C_DOTA_Item_Cheese:
            pass
        class C_DOTA_Ability_Muerta_TheCalling:
            pass
        class C_DOTA_Ability_LoneDruid_SpiritBear_Entangle:
            pass
        class C_DOTA_Ability_TemplarAssassin_PsiBlades:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Slardar_2:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Mirana_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Magic_Resistance_10:
            pass
        class CDOTA_Modifier_Item_Eternal_Shroud:
            pass
        class CDOTA_Modifier_Magnataur_Shockwave_Eruption_Slow:
            pass
        class CDOTA_Modifier_DeathProphet_Slow:
            pass
        class CDOTA_Modifier_Enigma_DemonicConversion_ModelScale:
            pass
        class CPulseCell_CursorQueue:
            pass
        class CPulseCell_Value_RandomFloat:
            pass
        class CPulseExecCursor:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_DOTA_Unit_Brewmaster_PrimalEarth:
            pass
        class C_DOTA_Unit_LoopingSound:
            pass
        class CDOTA_Item_Recipe_Ninja_Gear:
            pass
        class C_DOTA_Ability_Dawnbreaker_Solar_Guardian:
            pass
        class C_DOTA_Ability_Lich_FrostArmor:
            pass
        class C_Sprite:
            pass
        class C_DotaSubquestBase:
            pass
        class CDOTA_Modifier_AghsFort_Watch_Tower_Capturing:
            pass
        class CDOTA_Ability_AghsFort_ShadowWaveEffectPotion:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Riki_7:
            pass
        class C_DOTA_Ability_Special_Bonus_Movement_Speed_20:
            pass
        class CDOTA_Modifier_Item_Princes_Knife_Hex:
            pass
        class CDOTA_Modifier_Skywrath_Mage_Arcana_Kill_Effect:
            pass
        class CDOTA_Modifier_Weaver_Shukuchi_GeminateAttackMark:
            pass
        class C_DOTA_Unit_Hero_LoneDruid:
            pass
        class C_DOTA_Item_Recipe_Giants_Ring:
            pass
        class C_DOTA_Item_Infused_Raindrop:
            pass
        class C_DOTA_Ability_Necronomicon_Warrior_Sight:
            pass
        class C_DOTA_Item_Dagon_Upgraded3:
            pass
        class C_DOTA_Item_Reaver:
            pass
        class C_DOTA_Item_Recipe_NullTalisman:
            pass
        class C_DOTA_Ability_Invoker_EMP_AD:
            pass
        class CDOTA_Ability_CallOfTheWild_Boar_Poison:
            pass
        class C_DOTA_Ability_DrowRanger_FrostArrows:
            pass
        class CDOTA_Ability_Juggernaut_Swift_Slash:
            pass
        class C_DOTA_Item_DeathGoldDropped:
            pass
        class CDOTA_Ability_Generic_Hidden:
            pass
        class CDOTA_Modifier_AghsFort_TreantMiniboss_NaturesGuise_Tree_Walking:
            pass
        class CDOTA_Ability_AghsFort_DragonKnight_BreatheFire:
            pass
        class CDOTA_Modifier_AghsFort_ExplosiveBarrel:
            pass
        class CDOTA_Ability_Aghsfort_Reward_CritAura:
            pass
        class CDOTA_Modifier_Seasonal_Summon_Snowman_Thinker:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Oracle_5:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Doom_9:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Amplify_4:
            pass
        class CDOTA_Modifier_Magnataur_Empower_StackCounter:
            pass
        class CDOTA_Modifier_Magnataur_Empower:
            pass
        class CDOTA_Modifier_Centaur_Stampede:
            pass
        class CDOTA_Modifier_Broodmother_InsatiableHunger:
            pass
        class CDOTA_Modifier_Phantom_Assassin_GroundDagger:
            pass
        class CDOTA_Modifier_Lich_Ice_Spire:
            pass
        class CDOTA_Modifier_Puck_DreamCoil_Thinker:
            pass
        class CDOTA_Modifier_VengefulSpirit_Arcana:
            pass
        class CDOTA_Modifier_TrueSight:
            pass
        class CDOTA_Modifier_Bonus_Damage:
            pass
        class C_PhysicsProp:
            pass
        class CFilterTeam:
            pass
        class CDOTA_Item_Quickening_Charm:
            pass
        class C_DOTA_Item_Recipe_Mjollnir:
            pass
        class C_DOTA_Ability_Silencer_GlaivesOfWisdom:
            pass
        class C_DOTA_Ability_Tiny_CraggyExterior:
            pass
        class C_DOTA_Ability_OgreMagi_FrostArmor:
            pass
        class CDOTA_Aghsfort_Modifier_Magnus_Push_Skewer_Movement:
            pass
        class CDOTA_Modifier_Seasonal_TI9_Drums_Thinker:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Damage_60:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Base_Damage_100:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_Regen_40:
            pass
        class CDOTA_Modifier_Dezun_Bloodrite:
            pass
        class CDOTA_Modifier_Item_Woodland_Striders:
            pass
        class CDOTA_Modifier_Item_Boots_Of_Bearing:
            pass
        class CDOTA_Modifier_Mars_GodsRebuke_Crit:
            pass
        class CDOTA_Modifier_Mars_ArenaOfBlood_VisionObstructionAura:
            pass
        class CDOTA_Modifier_DarkWillow_ShadowRealm_Buff:
            pass
        class CDOTA_Modifier_Winter_Wyvern_Winterproof:
            pass
        class CDOTA_Modifier_Phoenix_FireSpiritCount:
            pass
        class CDOTA_Modifier_Brewmaster_PrimalSplitDelay:
            pass
        class CDOTA_Modifier_TemplarAssassin_Meditation_Illusion:
            pass
        class CDOTA_Modifier_Nian_Damage_Reflection:
            pass
        class CDOTA_Modifier_SandKing_CausticFinale:
            pass
        class CDOTA_Unit_Hero_Ringmaster:
            pass
        class C_InfoPlayerStartDota:
            pass
        class C_DOTA_Item_Royale_With_Cheese:
            pass
        class CDOTA_Item_Mask_Crit_Lifesteal:
            pass
        class CInfoInteraction:
            pass
        class CDOTA_Modifier_Creature_HybridFlyer:
            pass
        class C_DOTA_Ability_Slithereen_Riptide:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Ursa_8:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Drow_Ranger_2:
            pass
        class CDOTA_Modifier_Special_Bonus_Strength:
            pass
        class C_DOTA_Ability_Special_Bonus_Lifesteal_8:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Speed_30:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_200:
            pass
        class CDOTA_Modifier_Item_Ceremonial_Robe:
            pass
        class CDOTA_Modifier_Item_Hurricane_Pike:
            pass
        class CDOTA_Modifier_Item_Solar_Crest_Armor_Reduction:
            pass
        class CDOTA_Modifier_Smoke_Of_Deceit:
            pass
        class CDOTA_Modifier_Item_RingOfHealth:
            pass
        class CDOTA_Modifier_Mars_Bulwark_Active:
            pass
        class CDOTA_Modifier_Visage_SummonFamiliars_DamageCharge:
            pass
        class CDOTA_Modifier_Obsidian_Destroyer_ObjurgationMana:
            pass
        class CDOTA_Modifier_Jakiro_IcePath_Stun:
            pass
        class CDOTA_Modifier_Enchantress_Enchant_Controlled:
            pass
        class CDOTA_Modifier_QueenOfPain_ShadowStrike_Buff:
            pass
        class CDOTA_Modifier_Firework_Mine:
            pass
        class CDOTA_Modifier_Animation_TailSpin:
            pass
        class CDOTA_Modifier_StormSpirit_BallLightning:
            pass
        class CBasePlayerWeaponVData:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class C_DOTA_Item_Tier3Token:
            pass
        class C_DOTA_Item_Recipe_Hood_Of_Defiance:
            pass
        class C_DOTA_Ability_Hoodwink_HuntersQuiver:
            pass
        class CDOTA_Ability_Tusk_Drinking_Buddies:
            pass
        class CDOTA_Ability_Brewmaster_LiquidCourage:
            pass
        class C_DOTA_Ability_Weaver_Shukuchi:
            pass
        class CDOTA_Ability_MudGolem_CloakAura:
            pass
        class CDOTA_Modifier_AlphaWolf_CommandAura:
            pass
        class CDOTA_Modifier_AghsFort_Lifestealer_Enraged_Pulse:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Keeper_of_the_Light_2:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Antimage_5:
            pass
        class CDOTA_Modifier_Special_Bonus_Respawn_Reduction:
            pass
        class C_DOTA_Ability_Special_Bonus_Attack_Speed_25:
            pass
        class C_DOTA_DataSpectator:
            pass
        class CDOTA_Modifier_Phylactery_KillCredit:
            pass
        class CDOTA_Modifier_Voidwalker_Phased:
            pass
        class CDOTA_Modifier_Item_Butterfly:
            pass
        class CDOTA_Modifier_Tusk_Snowball_Movement_Friendly:
            pass
        class CDOTA_Modifier_Gyrocopter_Homing_Rocket_Barrage:
            pass
        class CDOTA_Modifier_IonShell:
            pass
        class CDOTA_Modifier_DragonKnight_CorrosiveBreathDoT:
            pass
        class CDOTA_Modifier_CallOfTheWild_Hawk_Reveal:
            pass
        class CDOTA_Modifier_JumpBoots:
            pass
        class CDOTA_Modifier_Tiny_Rocksteady:
            pass
        class CDOTA_Modifier_CrystalMaiden_BrillianceAura:
            pass
        class CDOTA_Modifier_VengefulSpirit_Hybrid_Special:
            pass
        class CDOTA_Modifier_Juggernaut_BladeFury_PullAura:
            pass
        class CDOTA_Modifier_Lua:
            pass
        class CDOTA_Modifier_Rune_Invisibility:
            pass
        class CDOTA_Modifier_ProjectileVision:
            pass
        class C_DOTAGamerulesProxy:
            pass
        class C_DOTAGameManagerProxy:
            pass
        class C_DOTA_Unit_Hero_Lina:
            pass
        class C_DOTA_Item_Recipe_Satanic:
            pass
        class CDOTA_Ability_Elder_Titan_EarthSplitter:
            pass
        class C_DOTA_Ability_Wisp_Empty1:
            pass
        class C_DOTA_Ability_Life_Stealer_Feast:
            pass
        class C_DOTA_Ability_Pugna_NetherBlast:
            pass
        class C_DOTA_Ability_Slardar_Slardar_SeabornSentinel:
            pass
        class C_DOTA_Ability_Windrunner_Shackleshot:
            pass
        class CDOTA_Ability_Zuus_Thunder_Trail:
            pass
        class CInfoParticleTarget:
            pass
        class C_DOTA_Ability_Aghsfort_Pugna_Grandmaster_NetherWard:
            pass
        class CDOTA_Ability_AghsFort_TreantMiniboss_NaturesGuise:
            pass
        class CDOTA_Ability_Aghsfort_Reward_MagicResistAura:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Arc_Warden_4:
            pass
        class C_DOTA_Ability_Special_Bonus_Unique_Lone_Druid_11:
            pass
        class C_DOTA_Ability_Special_Bonus_Cast_Range_50:
            pass
        class C_DOTA_Ability_Special_Bonus_HP_Regen_15:
            pass
        class C_DOTA_Ability_Special_Bonus_Cleave_130:
            pass
        class CDOTA_Modifier_CripplingCrossbow_Debuff:
            pass
        class CDOTA_Modifier_Fusion_runeHealing:
            pass
        class CDOTA_Modifier_Marci_Bodyguard_Self:
            pass
        class CDOTA_Modifier_Dawnbreaker_Fire_Wreath_Attack_Bonus:
            pass
        class CDOTA_Modifier_Grimstroke_SpiritWalk_Buff:
            pass
        class CDOTA_Modifier_Centaur_HoofStomp_Windup:
            pass
        class CDOTA_Modifier_KeeperOfTheLight_Will_O_Wisp_Aura:
            pass
        class CDOTA_Modifier_Treant_NaturesGrasp_Damage:
            pass
        class CDOTA_Modifier_Silencer_GlaivesOfWisdom_AttackCounter:
            pass
        class CDOTA_Modifier_Alchemist_AcidSpray_Thinker:
            pass
        class CDOTA_Modifier_Crystal_Maiden_GlacialGuard_Passive:
            pass
        class CDOTA_Modifier_Drow_Ranger_Glacier_Hilltop_Removal:
            pass
        class CDOTA_Modifier_Drow_Ranger_Glacier_Hilltop:
            pass
        class C_DOTA_Unit_Hero_QueenOfPain:
            pass
        class C_DOTA_Item_Divine_Regalia:
            pass
        class C_DOTA_Item_UnrelentingEye:
            pass
        class C_DOTA_Item_UnstableWand:
            pass
        class C_DOTA_Item_Vambrace:
            pass
        class CDOTA_Item_Recipe_GreatFamango:
            pass
        class C_DOTA_Ability_NianCharge:
            pass
        class C_DOTA_Ability_Riki_Backstab:
            pass
        class CDOTA_Modifier_BlackDragon_DragonhideAura:
            pass
        class C_PointClientUIWorldPanel:
            pass
        class CDOTA_Ability_Seasonal_Summon_Dragon_Thinker:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lycan_3:
            pass
        class PlayerNeutralChoices_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTA_Modifier_Item_PogoStick:
            pass
        class CDOTA_Modifier_Item_Necronomicon_Archer_Aura:
            pass
        class CDOTA_Modifier_Item_UltimateScepter_Consumed_Alchemist:
            pass
        class CDOTA_Modifier_Tusk_Tag_Team_Aura:
            pass
        class CDOTA_Modifier_Medusa_VenomedVolley_Slow:
            pass
        class CDOTA_Modifier_Rubick_SpellSteal:
            pass
        class CDOTA_Modifier_SpiritBreaker_HerdMentality:
            pass
        class CDOTA_Modifier_Huskar_Inner_Vitality:
            pass
        class CDOTA_Modifier_TemplarAssassin_PsiBlades:
            pass
        class CDOTA_Modifier_Roshan_Slam:
            pass
        class CDOTA_Modifier_Lion_Voodoo:
            pass
        class CDOTA_Modifier_Zuus_Lightning_Hands:
            pass
        class CDOTA_Modifier_SkeletonKing_SpectralBlade_CurseCooldown:
            pass
        class C_EntityFlame:
            pass
        class C_DOTA_Item_Recipe_AeonDisk:
            pass
        class C_DOTA_Ability_Hoodwink_TomokanTracker:
            pass
        class C_DOTA_Ability_Tusk_Launch_Snowball:
            pass
        class C_DOTA_Ability_BountyHunter_Big_Game_Hunter:
            pass
        class CDOTA_Modifier_AghsFort_TreantMiniboss_NaturesGuise_Invis:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Hoodwink_AcornShotCharges:
            pass
        class C_DOTA_Ability_Special_Bonus_Respawn_Reduction_30:
            pass
        class C_DOTA_Ability_Special_Bonus_Spell_Amplify_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Movement_Speed_Percentage_10:
            pass
        class CDOTA_Modifier_Giant_Maul_Debuff:
            pass
        class CDOTA_Modifier_Spell_Prism_Active:
            pass
        class CDOTA_Modifier_Item_Assault_Cuirass_Negative_Armor:
            pass
        class CDOTA_Modifier_Largo_FroglingBand_Self:
            pass
        class CDOTA_Modifier_Kez_RavensVeil_Blind:
            pass
        class CDOTA_Modifier_Bristleback_ViscousNasalGoo:
            pass
        class CDOTA_Modifier_Lycan_SummonWolves_Hamstring_DamageAmp:
            pass
        class CDOTA_Modifier_Lycan_SummonWolves_Health:
            pass
        class CDOTA_Modifier_Obsidian_Destroyer_EssenceAura_Effect:
            pass
        class CDOTA_Modifier_Clinkz_Tar_Bomb_Thinker:
            pass
        class CDOTA_Modifier_Furion_Sprout_Blind_Aura:
            pass
        class CDOTA_Modifier_Phased:
            pass
        class CDOTA_Modifier_Rune_Shield:
            pass
        class C_DOTA_Unit_Hero_Nevermore:
            pass
        class CDOTA_BaseNPC_Seasonal_TI9_Monkey:
            pass
        class C_DOTAPlayerController:
            pass
        class C_DOTA_Item_Recipe_Stormcrafter:
            pass
        class C_DOTA_Item_Greater_Faerie_Fire:
            pass
        class C_DOTA_Item_Recipe_Blood_Grenade:
            pass
        class C_DOTA_Item_Smoke_Of_Deceit:
            pass
        class C_DOTA_Item_Perseverance:
            pass
        class C_DOTA_Ability_PrimalBeast_Trample:
            pass
        class C_DOTA_Ability_LoneDruid_Rabid:
            pass
        class C_DOTA_Ability_Spectre_Desolate:
            pass
        class C_DOTA_Ability_BountyHunter_Track:
            pass
        class C_DOTA_DeathProphet_Exorcism_Spirit:
            pass
        class CDOTA_Ability_GraniteGolem_HPAura:
            pass
        class CDOTA_Modifier_731_Teaser_Thinker:
            pass
        class CDOTA_Modifier_Mutation_DeathExplosionDelayed:
            pass
        class CBasePlayerController:
            pass
        class CDOTA_Modifier_AghsFort_Shadow_Demon_Shadow_Poison:
            pass
        class C_DOTA_Ability_AghsFort_RockGolem_Smash:
            pass
        class CDOTA_Modifier_Aghsfort_Reward_MagicResistAura:
            pass
        class C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Razor_3:
            pass
        class C_DOTA_Ability_Special_Bonus_Status_Resistance_20:
            pass
        class C_DOTA_Ability_Special_Bonus_Armor_4:
            pass
        class CDOTA_Modifier_Minotaur_Horn_Immune:
            pass
        class CDOTA_Modifier_Royal_Jelly_Regen:
            pass
        class CDOTA_Modifier_Item_Crimson_Guard_NoStack:
            pass
        class CDOTA_Modifier_Skywrath_Mage_Shard_Bonus_Counter:
            pass
        class CDOTA_Modifier_Rubick_Might_And_Magus:
            pass
        class CDOTA_Modifier_LoneDruid_GiftBearer:
            pass
        class CDOTA_Modifier_Obsidian_Destroyer_AstralImprisonment_Prison:
            pass
        class CDOTA_Modifier_Jakiro_DualBreath_Slow:
            pass
        class CDOTA_Modifier_Tinker_Innate_Keen_Teleport_XP_On_Death:
            pass
        class CDOTA_Modifier_Morphling_Become_Strength:
            pass
        class CDOTA_Modifier_Earthshaker_Fissure_Line_Thinker:
            pass
        class ArtyProgressBarDef_t:
            pass
        class CDOTACrownfallCreditsMapSceneAnimateableDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class ArtyWeaponID_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTAFightingGameActionDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsEnemyDefinition__PickupChance:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class sControlGroupElem:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CSurvivorsEnemyDefinition_Pillar:
            pass
        class CDOTAOverworldCharacterConditional:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class Match3GameModeID_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CShmupPlayerDefinition:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class CSurvivorsSpawnerTowerDefinition:
            pass
        class ArtyCannonDef_t:
            pass
        class CandyShopCandyType_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsSpawnerGolem:
            pass
        class ArtyGameModeInfo_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CandyShopRewardSlot_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CAnnouncerDescriptor:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class C_VerticalMotionController:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CSurvivorsPowerUpDefinition_AreaAttack:
            pass
        class CSurvivorsPowerUpDefinition_AreaAttack_CircleConstant:
            pass
        class CDOTAOverworldNode:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CDOTACrownfallCreditsDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsPowerUp_AreaAttack_Circle:
            pass
        class CDOTAFishingGameFish:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CShmupTrackDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsLevelDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CBasePortraitData:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class ShmupPathID_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class SettingsSectionIndex_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTAOverworldClickable:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class PlayerSeatAssignment_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CSurvivorsGameModeDefinition__SeparationLayerData:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class CSurvivorsSpawnerDestructibles:
            pass
        class CSurvivorsGameSnapshot:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class Match3AbilityID_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CMatch3OpponentActionInstanceDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class screenshake_t:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class CSurvivorsPowerUpDefinition_ArcaneBolt:
            pass
        class C_DotaTree:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CSurvivorsLootTable__CLootEntry:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class CSurvivorsPowerUp_Frostbite:
            pass
        class CDOTAFightingGameHeroStyleDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CDOTARoadToTIChallengeDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class ArtyWeaponInfo_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsSpawnerEliteTurretDefinition:
            pass
        class CSurvivorsPowerUp_LandMine:
            pass
        class SettingsSubSectionIndex_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTAFlappySkywrathInputAction:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CDOTAOverworldTarotCard:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsAttributeDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsEnemyPillar:
            pass
        class CSurvivorsPowerUpDefinition_Spirits:
            pass
        class CrownfallSurvivorsLightingEnvironment_t:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class CDOTAFlappySkywrathCharacter:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class CSurvivorsPowerUp_LagunaBlade:
            pass
        class CSurvivorsDifficultyDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsPowerUp_AreaAttack_Line:
            pass
        class CSurvivorsSpawnerDestructiblesDefinition:
            pass
        class CSurvivorsEnemyDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CDOTAOverworldRoom:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CDOTAFishingGameDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsSpawnerGolemDefinition:
            pass
        class CSurvivorsEnemyEventDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class SZooSetAnnotationsConfig_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class ArtyGraphicID_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CSurvivorsUnit_Snotty:
            pass
        class ArtyConstants_t:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class CSurvivorsAttributeDefinition__MetaProgressionTier_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsPowerUp_InstantAttack:
            pass
        class Match3OpponentActionID_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTAOverworldPath:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsPowerUp_Stampede:
            pass
        class CSurvivorsPowerUpDefinition_InstantAttack:
            pass
        class CSurvivorsPowerUpDefinition_Frostbite:
            pass
        class CDOTAOverworldCharacter:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class SZooSetAnnotations_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsPowerUpDefinition_MagicMissile:
            pass
        class CSurvivorsPickupDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CMatch3OpponentActionDefinition:
            pass
        class CDOTAOverworldHero:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsAttackParticleInfo:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class PointCameraSettings_t:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class CSurvivorsAttributeValue:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class CrownfallCreditsAABB_t:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class CSurvivorsPowerUp:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CMatch3AbilityParamDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CInterpolatedValue:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class ArtySpawnerDef_t:
            pass
        class CSurvivorsPowerUpDefinition_MortimerKisses:
            pass
        class CSurvivorsPowerUpDefinition_Track:
            pass
        class ArtyLevelObjectInstance_t:
            pass
        class SurvivorsPickupID_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CShmupBossDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class NewSettingsID_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class ArtyLevelID_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CSurvivorsEnemyDefinition_ImperiaBoss:
            pass
        class CShmupBulletInfo:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # �
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class SZooSetAnnotation_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class SurvivorsPhysicsBodyID_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTACrownfallCreditsBlockDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsPowerUp_AreaAttack:
            pass
        class SurvivorsHeroID_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTAOverworldCharacterBase:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsPowerUpDefinition_EchoStrike:
            pass
        class CDOTAFlappySkywrathDifficulty:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class TimedEvent:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CFlashlightEffect:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CSurvivorsEnemyDefinition__Attack:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CShmupGameDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsHeroDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsEnemyDefinition_Snotty:
            pass
        class CSurvivorsPowerUp_MortimerKisses:
            pass
        class CDOTAFlappySkywrathDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsEnemyGolem:
            pass
        class CShmupPathDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CShmupEventTime:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CMatch3GameModeDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CDOTAOverworldDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class SNewSettingsDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsPowerUp_Spirits:
            pass
        class CShmupEnemyDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsEnemy:
            pass
        class SurvivorsUpgradeID_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class ArtyUnitDef_t:
            pass
        class CSurvivorsLootTable__CLootEntryCollection:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class ArtyGameObjectDef_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CShmupEventEnemySpawn:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class SurvivorsLevelID_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CSurvivorsPowerUp_EchoStrike:
            pass
        class CSurvivorsPowerUp_Track:
            pass
        class RoadToTIQuestDefinition_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class Match3OpponentID_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CSurvivorsEnemyAbsorber:
            pass
        class CDOTAMinesweeperStageDefinition:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class C_CommandContext:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CSurvivorsUnit:
            pass
        class CPortraitData:
            pass
        class CSurvivorsPowerUpDefinition_Stampede:
            pass
        class ArtyGameModeLevelInfo_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class OverworldSplineInfo_t:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class CSurvivorsEnemyDefinition_Golem:
            pass
        class CSurvivorsEnemyResurrector:
            pass
        class CSurvivorsPowerUp_ProjectileAttack:
            pass
        class ShmupEnemyID_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CSurvivorsEntity:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CandyShopDefinitionGC_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CDOTACandyShopDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class SurvivorsParticleID_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class ArtyGameObjectInstance_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class SettingsSearchDataIndex_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class SurvivorsDifficultyID_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CSurvivorsPowerUpDefinition_Snotty:
            pass
        class CSurvivorsLootTable:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CMatch3HeroDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsEnemySnapshot:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class CSurvivorsPowerUp_KnifeThrow:
            pass
        class CDOTAFightingGameCancelOptionDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CDOTAOverworldTheme:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsPowerUp_CounterHelix:
            pass
        class CSurvivorsPowerUp_AreaAttack_CircleConstant:
            pass
        class CSurvivorsEnemyDefinition_Resurrector:
            pass
        class CShmupBossPhase:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsPowerUpSnapshot:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CShmupEventDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsPowerUpDefinition_KnifeThrow:
            pass
        class CSurvivorsAttributeDefinition__MetaProgressionTierCost_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class ShmupEventID_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTAMinesweeperPlayerDefinition:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class ArtyLevelWeaponInstance_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsPowerUp_MagicMissile:
            pass
        class SurvivorsGameModeID_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CSurvivorsPlayerSnapshot:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CandyShopRewardOption_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsPowerUp_ArcaneBolt:
            pass
        class CShmupPathEvent:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class SurvivorsPowerUpID_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTAOverworldEncounterReward:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CDOTAOverworldRoomGroup:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsSpawner:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CSurvivorsPowerUpDamageTickInfo:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CDOTAMinesweeperGameDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CrownfallSurvivorsLightingOverride_t:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class CClientAlphaProperty:
            pass
        class CSurvivorsPowerUpDefinition_CounterHelix:
            pass
        class CDOTAOverworldHeroReward:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class ArtyLevelInfo_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsPowerUpDefinition_AreaAttack_Line:
            pass
        class CSurvivorsPowerUpDefinition_LagunaBlade:
            pass
        class C_HorizontalMotionController:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class screenfade_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class ArtyEnemyOrder_t:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class CMatch3Level:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class ArtyGraphicInfo_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CMatch3AbilityBaseDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CGlobalLightBase:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class SurvivorsEnemyID_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CMatch3AbilityDefinition:
            pass
        class CDOTACrownfallCreditsMapSceneDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsSpawnerEliteTurret:
            pass
        class CandyShopRewardOptionGC_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsPowerUp_Snotty:
            pass
        class CSurvivorsSpawnerTower:
            pass
        class CDOTACrownfallCreditsCharacterDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class ArtyGameObjectID_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CSurvivorsPowerUpDefinition_LandMine:
            pass
        class CMatch3OpponentHeroItemDefinition:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class IClientAlphaProperty:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class ArtyEnemyDef_t:
            pass
        class CDOTAEventActionTrigger:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsUpgradeDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsImperiaBoss:
            pass
        class CDOTAEventActionGrantAndClaimPair:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class SurvivorsUnitID_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CSurvivorsPowerUpDefinition_ProjectileAttack:
            pass
        class CDOTAFightingGameHeroDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CDOTAEventActionGrantAndClaimPairTrigger:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CLightInfoBase:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CSurvivorsPowerUpDefinition_Swashbuckle:
            pass
        class CDOTAOverworldEncounter:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsSpawnerDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class DOTAOverworldCharacterOverrideConditional_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class ArtyPlayerDef_t:
            pass
        class CDOTAOverworldPathColorRule:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class CSurvivorsPowerUp_Swashbuckle:
            pass
        class CDOTAOverworldToken:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class NeutralCampStackPullAlarm_t:
             = 0x10110FF # 
             = 0x0 # 
             = 0x45 # 
             = 0x0 # 
             = 0x0 # 
             = 0x400 # 
             = 0x0 # 
             = 0x1032000 # 
             = 0x0 # 
             = 0x1015000 # 
             = 0x0 # 
             = 0x61007400 # 
             = 0x6F006400 # 
             = 0x74006100 # 
             = 0x2E003000 # 
             = 0x3A004300 # 
             = 0x63006900 # 
             = 0x44002000 # 
             = 0x6D007200 # 
             = 0x5C003000 # 
             = 0x66006E00 # 
             = 0x61004400 # 
             = 0x6E006900 # 
             = 0x6C005000 # 
             = 0x32003000 # 
             = 0x6E006500 # 
             = 0x7FF98B # 
             = 0x0 # 
             = 0x1 # 
             = 0x69006400 # 
             = 0x1015000 # 
             = 0x44005C00 # 
             = 0x6E006900 # 
             = 0x0 # 
             = 0x0 # 
             = 0x7700 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D004D00 # 
             = 0x600 # 
             = 0x0 # 
             = 0x0 # 
             = 0x100F003 # 
             = 0x1100000 # 
        class CSurvivorsEnemyDefinition_Absorber:
            pass
        class CSurvivorsPowerUpDefinition_AreaAttack_Circle:
            pass
        class CSurvivorsPickupSnapshot:
             = 0x1 # 
             = 0x0 # 
             = 0x1 # 
             = 0x0 # 
             = 0xFF00 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x50005C # 
             = 0x6F0072 # 
             = 0x660065 # 
             = 0x34005C # 
             = 0x70004D # 
             = 0x670069 # 
             = 0x610074 # 
             = 0x6F0064 # 
             = 0x740061 # 
             = 0x2E0030 # 
             = 0x0 # 
             = 0x0 # 
             = 0xE02FB640 # 
             = 0x700073 # 
             = 0x0 # 
             = 0x730069 # 
             = 0x720065 # 
             = 0xEE # 
             = 0x80 # 
             = 0x20004D # 
             = 0x520002 # 
             = 0x0 # 
             = 0x15 # 
             = 0x0 # 
             = 0x0 # 
             = 0x0 # 
        class CDOTAMinesweeperStageProgressionChoice:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsPowerUpDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CSurvivorsGameModeDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CMatch3OpponentDefinition:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class PortraitWorldLightConfig_t:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
        class CShmupBossBodyPart:
             = 0x10120 # 
             = 0x10 # 
             = 0x0 # 
            ` = 0x0 # 
             = 0xEEFFEEFF # 
             = 0x1000 # 
             = 0x0 # 
             = 0x10FD0 # �
             = 0x10110 # 
             = 0x10750 # P
             = 0x1FE000 # 
             = 0x1 # 
            P = 0x10330 # 
             = 0x10150 # P
             = 0xE02FA810 # 
             = 0x630069 # 
             = 0x440020 # 
             = 0x6D0072 # 
             = 0x5C0030 # 
             = 0x6F0072 # 
             = 0x6F0073 # 
             = 0x6E0065 # 
             = 0x31002E # 
             = 0x61004F # 
             = 0x6C0000 # 
             = 0x4D005C # 
             = 0x730077 # 
             = 0x6F0066 # 
             = 0x2D0036 # 
             = 0x0 # 
             = 0x0 # 
             = 0x4B # 
             = 0x0 # 
             = 0x5D00005D # 
            P = 0x10150 # P
             = 0x6C0070 # 
             = 0x5C # 
             = 0x0 # 
             = 0x0 # 
             = 0xEE # 
             = 0x3 # 
             = 0x790079 # 
             = 0x70 # 
             = 0x0 # 
             = 0x99A46D76 # 
             = 0x1D00001D # 
             = 0x100F0 # �
            � = 0xF000 # 
