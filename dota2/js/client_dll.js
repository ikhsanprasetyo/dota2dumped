// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-03-30 13:31:39.632920200 +07:00

export const Schemas = {
    client_dll: {
        C_BaseCombatCharacter__WaterWakeMode_t: {
            WATER_WAKE_NONE: 0x0,
            WATER_WAKE_IDLE: 0x1,
            WATER_WAKE_WALKING: 0x2,
            WATER_WAKE_RUNNING: 0x3,
            WATER_WAKE_WATER_OVERHEAD: 0x4,
        },
        PulseBestOutflowRules_t: {
            SORT_BY_NUMBER_OF_VALID_CRITERIA: 0x0,
            SORT_BY_OUTFLOW_INDEX: 0x1,
        },
        PulseCursorCancelPriority_t: {
            None: 0x0,
            CancelOnSucceeded: 0x1,
            SoftCancel: 0x2,
            HardCancel: 0x3,
        },
        CDOTA_BaseNPC_AghsFort_Watch_Tower__ExitDirection_t: {
            EXIT_DIRECTION_LEFT: 0x0,
            EXIT_DIRECTION_TOP: 0x1,
            EXIT_DIRECTION_RIGHT: 0x2,
        },
        PulseMethodCallMode_t: {
            SYNC_WAIT_FOR_COMPLETION: 0x0,
            ASYNC_FIRE_AND_FORGET: 0x1,
        },
        VisualNovelDialogueLineType_t: {
            k_eVisualNovelDialogueLineType_Invalid: 0x0,
            k_eVisualNovelDialogueLineType_Text: 0x1,
            k_eVisualNovelDialogueLineType_PopupText: 0x2,
            k_eVisualNovelDialogueLineType_Comic: 0x3,
            k_eVisualNovelDialogueLineType_ExternalComic: 0x4,
            k_eVisualNovelDialogueLineType_Video: 0x5,
            k_eVisualNovelDialogueLineType_ConditionalIf: 0x6,
            k_eVisualNovelDialogueLineType_ConditionalElse: 0x7,
        },
        EOverworldEncounterRewardStyle: {
            k_eOverworldEncounterRewardStyle_Invalid: 0x0,
            k_eOverworldEncounterRewardStyle_Choice: 0x1,
            k_eOverworldEncounterRewardStyle_Random: 0x2,
            k_eOverworldEncounterRewardStyle_HighScore: 0x3,
            k_eOverworldEncounterRewardStyle_ClaimCredits: 0x4,
            k_eOverworldEncounterRewardStyle_Quest: 0x5,
            k_eOverworldEncounterRewardStyle_ClaimCreditsRange: 0x6,
            k_eOverworldEncounterRewardStyle_Custom: 0xA,
            k_eOverworldEncounterRewardStyle_RepeatableChoice: 0xB,
        },
        ESurvivorsEnemySpawnBehavior: {
            INVALID_SPAWN_BEHAVIOR: 0xFFFFFFFFFFFFFFFF,
            FIXED_DIRECTION: 0x0,
            FIXED_DIRECTION_PERPENDICULAR_WALL: 0x1,
            OVAL_AROUND_PLAYER: 0x2,
            STATIC_IN_MAP: 0x3,
            STATIC_IN_MAP_IGNORE_PLAYER_RADIUS: 0x4,
            RANDOM_DIRECTION: 0x5,
            SPAWNER_ORIGIN_RADIUS: 0x6,
        },
        EOverworldTokenType: {
            k_eOverworldTokenType_Invalid: 0x0,
            k_eOverworldTokenType_Generic: 0x1,
            k_eOverworldTokenType_Hidden: 0x2,
            k_eOverworldTokenType_Premium: 0x3,
            k_eOverworldTokenType_Scrap: 0x4,
        },
        CMsgDOTARequestMatches_SkillLevel: {
            CMsgDOTARequestMatches_SkillLevel_Any: 0x0,
            CMsgDOTARequestMatches_SkillLevel_Normal: 0x1,
            CMsgDOTARequestMatches_SkillLevel_High: 0x2,
            CMsgDOTARequestMatches_SkillLevel_VeryHigh: 0x3,
        },
        StartupBehavior_t: {
            UNIT_STARTUP_BEHAVIOR_DEFAULT: 0x0,
            UNIT_STARTUP_BEHAVIOR_TAUNT: 0x1,
        },
        ETeamFanContentAssetType: {
            k_eFanContentAssetType_LogoPNG: 0x1,
            k_eFanContentAssetType_LogoSVG: 0x2,
            k_eFanContentAssetType_Logo3D: 0x3,
            k_eFanContentAssetType_Players: 0x4,
            k_eFanContentAssetType_Sprays: 0x5,
            k_eFanContentAssetType_Wallpapers: 0x6,
            k_eFanContentAssetType_Emoticons: 0x7,
            k_eFanContentAssetType_VoiceLines: 0x8,
            k_eFanContentAssetType_Localization: 0x9,
            k_eFanContentAssetType_Banner: 0xA,
            k_eFanContentAssetType_BaseLogo: 0xB,
        },
        SurvivorsAttributeType_t: {
            k_eSurvivorsAttribute_Invalid: 0x0,
            k_eSurvivorsAttribute_MaxHP: 0x1,
            k_eSurvivorsAttribute_HPRegen: 0x2,
            k_eSurvivorsAttribute_LifeSteal: 0x3,
            k_eSurvivorsAttribute_Damage: 0x4,
            k_eSurvivorsAttribute_DamageMultiplier: 0x5,
            k_eSurvivorsAttribute_Damage_Physical: 0x6,
            k_eSurvivorsAttribute_DamageMultiplier_Physical: 0x7,
            k_eSurvivorsAttribute_Damage_Magical: 0x8,
            k_eSurvivorsAttribute_DamageMultiplier_Magical: 0x9,
            k_eSurvivorsAttribute_CriticalStrike_Physical_Chance: 0xA,
            k_eSurvivorsAttribute_CriticalStrike_Physical_DamageMultiplier: 0xB,
            k_eSurvivorsAttribute_MovementSpeed: 0xC,
            k_eSurvivorsAttribute_Cooldown: 0xD,
            k_eSurvivorsAttribute_CooldownReductionMultiplier: 0xE,
            k_eSurvivorsAttribute_Range: 0xF,
            k_eSurvivorsAttribute_Armor: 0x10,
            k_eSurvivorsAttribute_DamageReflection: 0x11,
            k_eSurvivorsAttribute_Dodge: 0x12,
            k_eSurvivorsAttribute_ExpMultiplier: 0x13,
            k_eSurvivorsAttribute_PickupRadius: 0x14,
            k_eSurvivorsAttribute_Luck: 0x15,
            k_eSurvivorsAttribute_Lives: 0x16,
            k_eSurvivorsAttribute_LimitBreak: 0x17,
            k_eSurvivorsAttribute_DashSpeed: 0x18,
            k_eSurvivorsAttribute_DashDuration: 0x19,
            k_eSurvivorsAttribute_DashCooldown: 0x1A,
            k_eSurvivorsAttribute_NumDashes: 0x1B,
            k_eSurvivorsAttribute_TargetCount: 0x1C,
            k_eSurvivorsAttribute_StunDuration: 0x1D,
            k_eSurvivorsAttribute_ProjectileRadius: 0x1E,
            k_eSurvivorsAttribute_ProjectileBounces: 0x1F,
            k_eSurvivorsAttribute_AoEIncrease: 0x20,
            k_eSurvivorsAttribute_KnockbackDistance: 0x21,
            k_eSurvivorsAttribute_FreezeDuration: 0x22,
            k_eSurvivorsAttribute_VulnerabilityDuration: 0x23,
            k_eSurvivorsAttribute_VulnerabilityDamagePercent: 0x24,
            k_eSurvivorsAttribute_VulnerabilityStunOnHitDuration: 0x25,
            k_eSurvivorsAttribute_ModifierDamagePerTick: 0x26,
            k_eSurvivorsAttribute_ProjectileSpeed: 0x27,
            k_eSurvivorsAttribute_ProjectileAttackInterval: 0x28,
            k_eSurvivorsAttribute_Width: 0x29,
            k_eSurvivorsAttribute_Length: 0x2A,
            k_eSurvivorsAttribute_Radius: 0x2B,
            k_eSurvivorsAttribute_InitialTickDelay: 0x2C,
            k_eSurvivorsAttribute_DamageTicks: 0x2D,
            k_eSurvivorsAttribute_TimeBetweenTicks: 0x2E,
            k_eSurvivorsAttribute_LifeTime: 0x2F,
            k_eSurvivorsAttribute_ArmingTime: 0x30,
            k_eSurvivorsAttribute_TriggerTime: 0x31,
            k_eSurvivorsAttribute_MaxRerolls: 0x32,
            k_eSurvivorsAttribute_ProjectileHitCount: 0x33,
            k_eSurvivorsAttribute_ProjectileHorizontalOffset: 0x34,
            k_eSurvivorsAttribute_SplashDamagePercentage: 0x35,
            k_eSurvivorsAttribute_Directions: 0x36,
            k_eSurvivorsAttribute_Duration: 0x37,
            k_eSurvivorsAttribute_DurationExtension: 0x38,
            k_eSurvivorsAttribute_StampedeMovementSpeed: 0x39,
            k_eSurvivorsAttribute_ActiveAbilitySlots: 0x3A,
            k_eSurvivorsAttribute_KnockbackDistanceMultiplier: 0x3B,
            k_eSurvivorsAttribute_FreezeSplashRadius: 0x3C,
            k_eSurvivorsAttribute_CriticalStrike_Projectile_Chance: 0x3D,
            k_eSurvivorsAttribute_CriticalStrike_Projectile_DamageMultiplier: 0x3E,
            k_eSurvivorsAttribute_Projectile_BonusMagicDamage: 0x3F,
            k_eSurvivorsAttribute_MagicDamageWeakensPhysicalDamageResistance_Duration: 0x40,
            k_eSurvivorsAttribute_MagicDamageWeakensPhysicalDamageResistance_Percentage: 0x41,
            k_eSurvivorsAttribute_BonusPhysicalDamage: 0x42,
            k_eSurvivorsAttribute_IncomingDamageAmplification: 0x43,
            k_eSurvivorsAttribute_IncomingDamageAmplificationDuration: 0x44,
            k_eSurvivorsAttribute_FreezeStrength: 0x45,
            k_eSurvivorsAttribute_ProjectileDamagePercent: 0x46,
            k_eSurvivorsAttribute_ProjectileAttack: 0x47,
            k_eSurvivorsAttribute_DurationEffect: 0x48,
            k_eSurvivorsAttribute_SlowStrength: 0x49,
            k_eSurvivorsAttribute_SlowDuration: 0x4A,
            k_eSurvivorsAttribute_Count: 0x4B,
        },
        ESurvivorsEnemyMovementCapability: {
            ENEMY_MOVEMENT_CAPABILITY_INVALID: 0xFFFFFFFFFFFFFFFF,
            ENEMY_MOVEMENT_CAPABILITY_GROUND: 0x0,
            ENEMY_MOVEMENT_CAPABILITY_FLYING: 0x1,
        },
        EOverworldIntroProgressState: {
            k_eOverworldIntroProgressState_StoryRecap: 0x0,
            k_eOverworldIntroProgressState_GetStarted: 0x1,
            k_eOverworldIntroProgressState_HowToPlay: 0x2,
            k_eOverworldIntroProgressState_FreeToken: 0x3,
            k_eOverworldIntroProgressState_RevealInventory: 0x4,
            k_eOverworldIntroProgressState_UnlockNode: 0x5,
            k_eOverworldIntroProgressState_ShowLearnMore: 0x6,
            k_eOverworldIntroProgressState_Complete: 0x7,
        },
        ETalentContentAssetType: {
            k_eTalentContentAssetType_Photo: 0x1,
            k_eTalentContentAssetType_Autograph: 0x2,
            k_eTalentContentAssetType_Voicelines: 0x3,
        },
        EShowcaseType: {
            k_eShowcaseType_Invalid: 0x0,
            k_eShowcaseType_Profile: 0x1,
            k_eShowcaseType_MiniProfile: 0x2,
            k_eShowcaseType_DefaultProfile: 0x3,
            k_eShowcaseType_DefaultMiniProfile: 0x4,
        },
        DOTA_WatchReplayType: {
            DOTA_WATCH_REPLAY_NORMAL: 0x0,
            DOTA_WATCH_REPLAY_HIGHLIGHTS: 0x1,
        },
        ESurvivorsBossAbility: {
            INVALID: 0xFFFFFFFFFFFFFFFF,
            BURNING_GROUND: 0x0,
            RADIATE_RAYS: 0x1,
            DEMON_PORTALS: 0x2,
            MAGIC_MISSILE: 0x3,
        },
        ETournamentEvent: {
            k_ETournamentEvent_None: 0x0,
            k_ETournamentEvent_TournamentCreated: 0x1,
            k_ETournamentEvent_TournamentsMerged: 0x2,
            k_ETournamentEvent_GameOutcome: 0x3,
            k_ETournamentEvent_TeamGivenBye: 0x4,
            k_ETournamentEvent_TournamentCanceledByAdmin: 0x5,
            k_ETournamentEvent_TeamAbandoned: 0x6,
            k_ETournamentEvent_ScheduledGameStarted: 0x7,
            k_ETournamentEvent_Canceled: 0x8,
            k_ETournamentEvent_TeamParticipationTimedOut_EntryFeeRefund: 0x9,
            k_ETournamentEvent_TeamParticipationTimedOut_EntryFeeForfeit: 0xA,
            k_ETournamentEvent_TeamParticipationTimedOut_GrantedVictory: 0xB,
        },
        PortraitScale_t: {
            PORTRAIT_SCALE_INVALID: 0xFFFFFFFFFFFFFFFF,
            PORTRAIT_SCALE_LOADOUT: 0x0,
            PORTRAIT_SCALE_ALTERNATE_LOADOUT: 0x1,
            PORTRAIT_SCALE_WORLD: 0x2,
            PORTRAIT_SCALE_SPECTATOR_LOADOUT: 0x3,
            PORTRAIT_SCALE_VERSUS_LOADOUT: 0x4,
        },
        ENewSettingsBadge: {
            eNewSettingsBadge_New: 0x0,
            eNewSettingsBadge_Updated: 0x1,
        },
        EDOTAGroupMergeResult: {
            k_EDOTAGroupMergeResult_OK: 0x0,
            k_EDOTAGroupMergeResult_FAILED_GENERIC: 0x1,
            k_EDOTAGroupMergeResult_NOT_LEADER: 0x2,
            k_EDOTAGroupMergeResult_TOO_MANY_PLAYERS: 0x3,
            k_EDOTAGroupMergeResult_TOO_MANY_COACHES: 0x4,
            k_EDOTAGroupMergeResult_ENGINE_MISMATCH: 0x5,
            k_EDOTAGroupMergeResult_NO_SUCH_GROUP: 0x6,
            k_EDOTAGroupMergeResult_OTHER_GROUP_NOT_OPEN: 0x7,
            k_EDOTAGroupMergeResult_ALREADY_INVITED: 0x8,
            k_EDOTAGroupMergeResult_NOT_INVITED: 0x9,
        },
        EPartyBeaconType: {
            k_EPartyBeaconType_Available: 0x0,
            k_EPartyBeaconType_Joinable: 0x1,
        },
        EOverworldFortuneReward: {
            k_eOverworldFortuneReward_Invalid: 0x0,
            k_eOverworldFortuneReward_RandomToken: 0x1,
            k_eOverworldFortuneReward_ExtraToken: 0x2,
            k_eOverworldFortuneReward_CandyBag: 0x3,
            k_eOverworldFortuneReward_DiscountCoin: 0x4,
            k_eOverworldFortuneReward_Treasure: 0x5,
        },
        EDOTAFightingGameAILevel: {
            FG_AI_LEVEL_NONE: 0xFFFFFFFFFFFFFFFF,
            FG_AI_LEVEL_EASY: 0x0,
            FG_AI_LEVEL_MEDIUM: 0x1,
            FG_AI_LEVEL_HARD: 0x2,
        },
        EShowcaseAuditAction: {
            k_eShowcaseAuditAction_Invalid: 0x0,
            k_eShowcaseAuditAction_ShowcaseChanged: 0x1,
            k_eShowcaseAuditAction_AdminShowcaseReset: 0x2,
            k_eShowcaseAuditAction_AdminShowcaseAccountLocked: 0x3,
            k_eShowcaseAuditAction_AdminShowcaseExonerated: 0x4,
            k_eShowcaseAuditAction_AdminShowcaseConvicted: 0x5,
            k_eShowcaseAuditAction_AdminModerationApproved: 0x6,
            k_eShowcaseAuditAction_AdminModerationRejected: 0x7,
        },
        EItemEditorReservationResult: {
            k_EItemEditorReservationResult_OK: 0x1,
            k_EItemEditorReservationResult_AlreadyExists: 0x2,
            k_EItemEditorReservationResult_Reserved: 0x3,
            k_EItemEditorReservationResult_TimedOut: 0x4,
        },
        CMsgClientToGCUpdateComicBookStat_Type: {
            CMsgClientToGCUpdateComicBookStat_Type_HighestPageRead: 0x1,
            CMsgClientToGCUpdateComicBookStat_Type_SecondsSpentReading: 0x2,
            CMsgClientToGCUpdateComicBookStat_Type_HighestPercentRead: 0x3,
        },
        EHeroSelectionText: {
            k_EHeroSelectionText_Invalid: 0xFFFFFFFFFFFFFFFF,
            k_EHeroSelectionText_None: 0x0,
            k_EHeroSelectionText_ChooseHero: 0x1,
            k_EHeroSelectionText_AllDraft_Planning_YouFirst: 0x2,
            k_EHeroSelectionText_AllDraft_Planning_TheyFirst: 0x3,
            k_EHeroSelectionText_AllDraft_Banning: 0x4,
            k_EHeroSelectionText_AllDraft_Ban_Waiting: 0x5,
            k_EHeroSelectionText_AllDraft_PickTwo: 0x6,
            k_EHeroSelectionText_AllDraft_PickOneMore: 0x7,
            k_EHeroSelectionText_AllDraft_PickOne: 0x8,
            k_EHeroSelectionText_AllDraft_WaitingRadiant: 0x9,
            k_EHeroSelectionText_AllDraft_WaitingDire: 0xA,
            k_EHeroSelectionText_AllDraft_TeammateRandomed: 0xB,
            k_EHeroSelectionText_AllDraft_YouPicking_LosingGold: 0xC,
            k_EHeroSelectionText_AllDraft_TheyPicking_LosingGold: 0xD,
            k_EHeroSelectionText_CaptainsMode_ChooseCaptain: 0xE,
            k_EHeroSelectionText_CaptainsMode_WaitingForChooseCaptain: 0xF,
            k_EHeroSelectionText_CaptainsMode_YouSelect: 0x10,
            k_EHeroSelectionText_CaptainsMode_TheySelect: 0x11,
            k_EHeroSelectionText_CaptainsMode_YouBan: 0x12,
            k_EHeroSelectionText_CaptainsMode_TheyBan: 0x13,
            k_EHeroSelectionText_RandomDraft_HeroReview: 0x14,
            k_EHeroSelectionText_RandomDraft_RoundDisplay: 0x15,
            k_EHeroSelectionText_RandomDraft_Waiting: 0x16,
            k_EHeroSelectionText_EventGame_BanPhase: 0x17,
        },
        EOverworldProgressionType: {
            k_eOverworldProgressionType_NodesAndPaths: 0x0,
            k_eOverworldProgressionType_NodesPathsAndRooms: 0x1,
        },
        EBingoAuditAction: {
            k_eBingoAuditAction_Invalid: 0x0,
            k_eBingoAuditAction_DevModifyTokens: 0x1,
            k_eBingoAuditAction_DevClearInventory: 0x2,
            k_eBingoAuditAction_DevRerollCard: 0x3,
            k_eBingoAuditAction_ShuffleCard: 0x4,
            k_eBingoAuditAction_RerollSquare: 0x5,
            k_eBingoAuditAction_UpgradeSquare: 0x6,
            k_eBingoAuditAction_ClaimRow: 0x7,
            k_eBingoAuditAction_EventActionTokenGrant: 0x8,
            k_eBingoAuditAction_SupportGrantTokens: 0x9,
            k_eBingoAuditAction_SupportStatThresholdFixup: 0xA,
        },
        DOTAVisualNovelDialogueEffect_t: {
            VN_DIALOGUE_EFFECT_NONE: 0x0,
            VN_DIALOGUE_EFFECT_ENTRANCE_SLIDE_SLOW: 0x1,
            VN_DIALOGUE_EFFECT_ENTRANCE_SLIDE_FAST: 0x2,
            VN_DIALOGUE_EFFECT_ENTRANCE_RISING: 0x4,
            VN_DIALOGUE_EFFECT_ENTRANCE_FALLING: 0x8,
            VN_DIALOGUE_EFFECT_ENTRANCE_LEFT: 0x10,
            VN_DIALOGUE_EFFECT_ENTRANCE_RIGHT: 0x20,
            VN_DIALOGUE_EFFECT_EXIT_INSTANT: 0x40,
            VN_DIALOGUE_EFFECT_EXIT_SLIDE_SLOW: 0x80,
            VN_DIALOGUE_EFFECT_EXIT_SLIDE_FAST: 0x100,
            VN_DIALOGUE_EFFECT_EXIT_FALLING: 0x200,
            VN_DIALOGUE_EFFECT_EXIT_RISING: 0x400,
            VN_DIALOGUE_EFFECT_WINDOW_SHAKE: 0x800,
            VN_DIALOGUE_EFFECT_PORTRAIT_SHAKE: 0x1000,
            VN_DIALOGUE_EFFECT_PORTRAIT_WEAVE: 0x2000,
            VN_DIALOGUE_EFFECT_PORTRAIT_FLIP: 0x4000,
            VN_DIALOGUE_EFFECT_TEXT_INSTANT: 0x8000,
            VN_DIALOGUE_EFFECT_TEXT_FAST: 0x10000,
            VN_DIALOGUE_EFFECT_TEXT_BOLD: 0x20000,
            VN_DIALOGUE_EFFECT_TEXT_SMALL: 0x40000,
            VN_DIALOGUE_EFFECT_NO_WAIT: 0x80000,
            VN_DIALOGUE_EFFECT_ACTOR_ANGER: 0x100000,
            VN_DIALOGUE_EFFECT_ACTOR_ANXIETY: 0x200000,
            VN_DIALOGUE_EFFECT_ACTOR_SWEATDROP: 0x400000,
            VN_DIALOGUE_EFFECT_POPUP_WET: 0x800000,
            VN_DIALOGUE_EFFECT_POPUP_FOCUS_BOTTOM: 0x1000000,
            VN_DIALOGUE_EFFECT_POPUP_FOCUS_TOP: 0x2000000,
            VN_DIALOGUE_EFFECT_PORTRAIT_BIG: 0x4000000,
            VN_DIALOGUE_EFFECT_CLEAR_STAGE: 0x8000000,
            VN_DIALOGUE_EFFECT_PORTRAIT_JITTER: 0x10000000,
            VN_DIALOGUE_EFFECT_PORTRAIT_BORDER_FROZEN: 0x20000000,
            VN_DIALOGUE_EFFECT_PORTRAIT_SMALL: 0x40000000,
            VN_DIALOGUE_EFFECT_PORTRAIT_NO_BLUR_OTHER: 0x80000000,
        },
        EArtyHitboxType: {
            k_eAABB: 0x0,
            k_eCircle: 0x1,
            k_eLine: 0x2,
            k_eRay: 0x3,
        },
        EShowcaseItemState: {
            k_eShowcaseItemState_Ok: 0x0,
            k_eShowcaseItemState_MinorModifications: 0x1,
            k_eShowcaseItemState_ValidityUnknown: 0x2,
            k_eShowcaseItemState_PartiallyInvalid: 0x3,
            k_eShowcaseItemState_Invalid: 0x4,
            k_eShowcaseItemState_Failure: 0x5,
        },
        ESupportEventRequestResult: {
            k_ESupportEventRequestResult_Success: 0x0,
            k_ESupportEventRequestResult_Timeout: 0x1,
            k_ESupportEventRequestResult_CantLockSOCache: 0x2,
            k_ESupportEventRequestResult_ItemNotInInventory: 0x3,
            k_ESupportEventRequestResult_InvalidItemDef: 0x4,
            k_ESupportEventRequestResult_InvalidEvent: 0x5,
            k_ESupportEventRequestResult_EventExpired: 0x6,
            k_ESupportEventRequestResult_InvalidSupportAccount: 0x7,
            k_ESupportEventRequestResult_InvalidSupportMessage: 0x8,
            k_ESupportEventRequestResult_InvalidEventPoints: 0x9,
            k_ESupportEventRequestResult_InvalidPremiumPoints: 0xA,
            k_ESupportEventRequestResult_InvalidActionID: 0xB,
            k_ESupportEventRequestResult_InvalidActionScore: 0xC,
            k_ESupportEventRequestResult_TransactionFailed: 0xD,
        },
        ETournamentTemplate: {
            k_ETournamentTemplate_None: 0x0,
            k_ETournamentTemplate_AutomatedWin3: 0x1,
        },
        EDevEventRequestResult: {
            k_EDevEventRequestResult_Success: 0x0,
            k_EDevEventRequestResult_NotAllowed: 0x1,
            k_EDevEventRequestResult_InvalidEvent: 0x2,
            k_EDevEventRequestResult_SqlFailure: 0x3,
            k_EDevEventRequestResult_Timeout: 0x4,
            k_EDevEventRequestResult_LockFailure: 0x5,
            k_EDevEventRequestResult_SDOLoadFailure: 0x6,
        },
        EGuildChatType: {
            k_EGuildChatType_Unspecified: 0x0,
            k_EGuildChatType_SteamChatGroup: 0x1,
            k_EGuildChatType_GC: 0x2,
        },
        EArtyOrderFlag: {
            k_EArtyOrderFlag_None: 0x0,
            k_EArtyOrderFlag_MoveLeft: 0x1,
            k_EArtyOrderFlag_MoveRight: 0x2,
            k_EArtyOrderFlag_AimUp: 0x4,
            k_EArtyOrderFlag_AimDown: 0x8,
            k_EArtyOrderFlag_NextWeapon: 0x10,
            k_EArtyOrderFlag_PrevWeapon: 0x20,
            k_EArtyOrderFlag_FireStart: 0x40,
            k_EArtyOrderFlag_FireStop: 0x80,
            k_EArtyOrderFlag_FineControl: 0x100,
            k_EArtyOrderFlag_PowerUp: 0x200,
            k_EArtyOrderFlag_PowerDown: 0x400,
            k_EArtyOrderFlag_MoveUp: 0x800,
            k_EArtyOrderFlag_MoveDown: 0x1000,
        },
        ECandyShopRewardOptionType: {
            k_eCandyShopRewardOptionType_Invalid: 0xFFFFFFFFFFFFFFFF,
            k_eCandyShopRewardOptionType_SingleItem: 0x0,
            k_eCandyShopRewardOptionType_LootList: 0x1,
            k_eCandyShopRewardOptionType_SingleEventAction: 0x2,
            k_eCandyShopRewardOptionType_EventPoints: 0x3,
        },
        ETournamentTeamState: {
            k_ETournamentTeamState_Unknown: 0x0,
            k_ETournamentTeamState_Node1: 0x1,
            k_ETournamentTeamState_NodeMax: 0x400,
            k_ETournamentTeamState_Eliminated: 0x36B3,
            k_ETournamentTeamState_Forfeited: 0x36B4,
            k_ETournamentTeamState_Finished1st: 0x3A99,
            k_ETournamentTeamState_Finished2nd: 0x3A9A,
            k_ETournamentTeamState_Finished3rd: 0x3A9B,
            k_ETournamentTeamState_Finished4th: 0x3A9C,
            k_ETournamentTeamState_Finished5th: 0x3A9D,
            k_ETournamentTeamState_Finished6th: 0x3A9E,
            k_ETournamentTeamState_Finished7th: 0x3A9F,
            k_ETournamentTeamState_Finished8th: 0x3AA0,
            k_ETournamentTeamState_Finished9th: 0x3AA1,
            k_ETournamentTeamState_Finished10th: 0x3AA2,
            k_ETournamentTeamState_Finished11th: 0x3AA3,
            k_ETournamentTeamState_Finished12th: 0x3AA4,
            k_ETournamentTeamState_Finished13th: 0x3AA5,
            k_ETournamentTeamState_Finished14th: 0x3AA6,
            k_ETournamentTeamState_Finished15th: 0x3AA7,
            k_ETournamentTeamState_Finished16th: 0x3AA8,
        },
        EFightingGameInvulnerabilityFlags: {
            FG_INVULNERABILITY_NONE: 0x0,
            FG_INVULNERABILITY_STRIKE: 0x1,
            FG_INVULNERABILITY_PROJECTILE: 0x2,
        },
        EArtyLayer: {
            k_eDefault: 0x1,
            k_eTerrain: 0x2,
            k_eShots: 0x4,
            k_eRays: 0x8,
            k_eFX: 0x10,
            k_ePhysical: 0x7,
            k_eFullObjects: 0x3,
            k_eAllLayers: 0xFFFFFFFF,
        },
        SteamUGCQuery: {
            RankedByVote: 0x0,
            RankedByPublicationDate: 0x1,
            AcceptedForGameRankedByAcceptanceDate: 0x2,
            RankedByTrend: 0x3,
            FavoritedByFriendsRankedByPublicationDate: 0x4,
            CreatedByFriendsRankedByPublicationDate: 0x5,
            RankedByNumTimesReported: 0x6,
            CreatedByFollowedUsersRankedByPublicationDate: 0x7,
            NotYetRated: 0x8,
            RankedByTotalVotesAsc: 0x9,
            RankedByVotesUp: 0xA,
            RankedByTextSearch: 0xB,
            RankedByTotalUniqueSubscriptions: 0xC,
            RankedByPlaytimeTrend: 0xD,
            RankedByTotalPlaytime: 0xE,
            RankedByAveragePlaytimeTrend: 0xF,
            RankedByLifetimeAveragePlaytime: 0x10,
            RankedByPlaytimeSessionsTrend: 0x11,
            RankedByLifetimePlaytimeSessions: 0x12,
        },
        EOverworldNodeAlertState: {
            k_eOverworldNodeAlertState_Unset: 0x0,
            k_eOverworldNodeAlertState_Show: 0x1,
            k_eOverworldNodeAlertState_Hide: 0x2,
        },
        DOTAKeybindCommand_t: {
            DOTA_KEYBIND_NONE: 0x0,
            DOTA_KEYBIND_FIRST: 0x1,
            DOTA_KEYBIND_CAMERA_DOWN: 0x2,
            DOTA_KEYBIND_CAMERA_LEFT: 0x3,
            DOTA_KEYBIND_CAMERA_RIGHT: 0x4,
            DOTA_KEYBIND_CAMERA_GRIP: 0x5,
            DOTA_KEYBIND_CAMERA_YAW_GRIP: 0x6,
            DOTA_KEYBIND_CAMERA_SAVED_POSITION_1: 0x7,
            DOTA_KEYBIND_CAMERA_SAVED_POSITION_2: 0x8,
            DOTA_KEYBIND_CAMERA_SAVED_POSITION_3: 0x9,
            DOTA_KEYBIND_CAMERA_SAVED_POSITION_4: 0xA,
            DOTA_KEYBIND_CAMERA_SAVED_POSITION_5: 0xB,
            DOTA_KEYBIND_CAMERA_SAVED_POSITION_6: 0xC,
            DOTA_KEYBIND_CAMERA_SAVED_POSITION_7: 0xD,
            DOTA_KEYBIND_CAMERA_SAVED_POSITION_8: 0xE,
            DOTA_KEYBIND_CAMERA_SAVED_POSITION_9: 0xF,
            DOTA_KEYBIND_CAMERA_SAVED_POSITION_10: 0x10,
            DOTA_KEYBIND_HERO_ATTACK: 0x11,
            DOTA_KEYBIND_HERO_MOVE: 0x12,
            DOTA_KEYBIND_HERO_MOVE_DIRECTION: 0x13,
            DOTA_KEYBIND_PATROL: 0x14,
            DOTA_KEYBIND_HERO_STOP: 0x15,
            DOTA_KEYBIND_HERO_HOLD: 0x16,
            DOTA_KEYBIND_HERO_SELECT: 0x17,
            DOTA_KEYBIND_COURIER_SELECT: 0x18,
            DOTA_KEYBIND_COURIER_DELIVER: 0x19,
            DOTA_KEYBIND_COURIER_BURST: 0x1A,
            DOTA_KEYBIND_COURIER_SHIELD: 0x1B,
            DOTA_KEYBIND_PAUSE: 0x1C,
            DOTA_SELECT_ALL: 0x1D,
            DOTA_SELECT_ALL_OTHERS: 0x1E,
            DOTA_RECENT_EVENT: 0x1F,
            DOTA_KEYBIND_CHAT_TEAM: 0x20,
            DOTA_KEYBIND_CHAT_GLOBAL: 0x21,
            DOTA_KEYBIND_CHAT_TEAM2: 0x22,
            DOTA_KEYBIND_CHAT_GLOBAL2: 0x23,
            DOTA_KEYBIND_CHAT_VOICE_PARTY: 0x24,
            DOTA_KEYBIND_CHAT_VOICE_TEAM: 0x25,
            DOTA_KEYBIND_CHAT_WHEEL: 0x26,
            DOTA_KEYBIND_CHAT_WHEEL2: 0x27,
            DOTA_KEYBIND_CHAT_WHEEL_CARE: 0x28,
            DOTA_KEYBIND_CHAT_WHEEL_BACK: 0x29,
            DOTA_KEYBIND_CHAT_WHEEL_NEED_WARDS: 0x2A,
            DOTA_KEYBIND_CHAT_WHEEL_STUN: 0x2B,
            DOTA_KEYBIND_CHAT_WHEEL_HELP: 0x2C,
            DOTA_KEYBIND_CHAT_WHEEL_GET_PUSH: 0x2D,
            DOTA_KEYBIND_CHAT_WHEEL_GOOD_JOB: 0x2E,
            DOTA_KEYBIND_CHAT_WHEEL_MISSING: 0x2F,
            DOTA_KEYBIND_CHAT_WHEEL_MISSING_TOP: 0x30,
            DOTA_KEYBIND_CHAT_WHEEL_MISSING_MIDDLE: 0x31,
            DOTA_KEYBIND_CHAT_WHEEL_MISSING_BOTTOM: 0x32,
            DOTA_KEYBIND_HERO_CHAT_WHEEL: 0x33,
            DOTA_KEYBIND_SPRAY_WHEEL: 0x34,
            DOTA_KEYBIND_ABILITY_PRIMARY1: 0x35,
            DOTA_KEYBIND_ABILITY_PRIMARY2: 0x36,
            DOTA_KEYBIND_ABILITY_PRIMARY3: 0x37,
            DOTA_KEYBIND_ABILITY_SECONDARY1: 0x38,
            DOTA_KEYBIND_ABILITY_SECONDARY2: 0x39,
            DOTA_KEYBIND_ABILITY_ULTIMATE: 0x3A,
            DOTA_KEYBIND_TALENT_UPGRADE_LEFT: 0x3B,
            DOTA_KEYBIND_TALENT_UPGRADE_RIGHT: 0x3C,
            DOTA_KEYBIND_TALENT_UPGRADE_ATTRIBUTE: 0x3D,
            DOTA_KEYBIND_NEUTRAL_ITEM_SELECT1: 0x3E,
            DOTA_KEYBIND_NEUTRAL_ITEM_SELECT2: 0x3F,
            DOTA_KEYBIND_NEUTRAL_ITEM_SELECT3: 0x40,
            DOTA_KEYBIND_NEUTRAL_ITEM_SELECT4: 0x41,
            DOTA_KEYBIND_NEUTRAL_ITEM_SELECT5: 0x42,
            DOTA_KEYBIND_ABILITY_PRIMARY1_QUICKCAST: 0x43,
            DOTA_KEYBIND_ABILITY_PRIMARY2_QUICKCAST: 0x44,
            DOTA_KEYBIND_ABILITY_PRIMARY3_QUICKCAST: 0x45,
            DOTA_KEYBIND_ABILITY_SECONDARY1_QUICKCAST: 0x46,
            DOTA_KEYBIND_ABILITY_SECONDARY2_QUICKCAST: 0x47,
            DOTA_KEYBIND_ABILITY_ULTIMATE_QUICKCAST: 0x48,
            DOTA_KEYBIND_ABILITY_PRIMARY1_EXPLICIT_AUTOCAST: 0x49,
            DOTA_KEYBIND_ABILITY_PRIMARY2_EXPLICIT_AUTOCAST: 0x4A,
            DOTA_KEYBIND_ABILITY_PRIMARY3_EXPLICIT_AUTOCAST: 0x4B,
            DOTA_KEYBIND_ABILITY_SECONDARY1_EXPLICIT_AUTOCAST: 0x4C,
            DOTA_KEYBIND_ABILITY_SECONDARY2_EXPLICIT_AUTOCAST: 0x4D,
            DOTA_KEYBIND_ABILITY_ULTIMATE_EXPLICIT_AUTOCAST: 0x4E,
            DOTA_KEYBIND_ABILITY_PRIMARY1_QUICKCAST_AUTOCAST: 0x4F,
            DOTA_KEYBIND_ABILITY_PRIMARY2_QUICKCAST_AUTOCAST: 0x50,
            DOTA_KEYBIND_ABILITY_PRIMARY3_QUICKCAST_AUTOCAST: 0x51,
            DOTA_KEYBIND_ABILITY_SECONDARY1_QUICKCAST_AUTOCAST: 0x52,
            DOTA_KEYBIND_ABILITY_SECONDARY2_QUICKCAST_AUTOCAST: 0x53,
            DOTA_KEYBIND_ABILITY_ULTIMATE_QUICKCAST_AUTOCAST: 0x54,
            DOTA_KEYBIND_ABILITY_PRIMARY1_AUTOMATIC_AUTOCAST: 0x55,
            DOTA_KEYBIND_ABILITY_PRIMARY2_AUTOMATIC_AUTOCAST: 0x56,
            DOTA_KEYBIND_ABILITY_PRIMARY3_AUTOMATIC_AUTOCAST: 0x57,
            DOTA_KEYBIND_ABILITY_SECONDARY1_AUTOMATIC_AUTOCAST: 0x58,
            DOTA_KEYBIND_ABILITY_SECONDARY2_AUTOMATIC_AUTOCAST: 0x59,
            DOTA_KEYBIND_ABILITY_ULTIMATE_AUTOMATIC_AUTOCAST: 0x5A,
            DOTA_KEYBIND_INVENTORY1: 0x5B,
            DOTA_KEYBIND_INVENTORY2: 0x5C,
            DOTA_KEYBIND_INVENTORY3: 0x5D,
            DOTA_KEYBIND_INVENTORY4: 0x5E,
            DOTA_KEYBIND_INVENTORY5: 0x5F,
            DOTA_KEYBIND_INVENTORY6: 0x60,
            DOTA_KEYBIND_INVENTORYTP: 0x61,
            DOTA_KEYBIND_INVENTORYNEUTRAL: 0x62,
            DOTA_KEYBIND_INVENTORY1_QUICKCAST: 0x63,
            DOTA_KEYBIND_INVENTORY2_QUICKCAST: 0x64,
            DOTA_KEYBIND_INVENTORY3_QUICKCAST: 0x65,
            DOTA_KEYBIND_INVENTORY4_QUICKCAST: 0x66,
            DOTA_KEYBIND_INVENTORY5_QUICKCAST: 0x67,
            DOTA_KEYBIND_INVENTORY6_QUICKCAST: 0x68,
            DOTA_KEYBIND_INVENTORYTP_QUICKCAST: 0x69,
            DOTA_KEYBIND_INVENTORYNEUTRAL_QUICKCAST: 0x6A,
            DOTA_KEYBIND_INVENTORY1_AUTOCAST: 0x6B,
            DOTA_KEYBIND_INVENTORY2_AUTOCAST: 0x6C,
            DOTA_KEYBIND_INVENTORY3_AUTOCAST: 0x6D,
            DOTA_KEYBIND_INVENTORY4_AUTOCAST: 0x6E,
            DOTA_KEYBIND_INVENTORY5_AUTOCAST: 0x6F,
            DOTA_KEYBIND_INVENTORY6_AUTOCAST: 0x70,
            DOTA_KEYBIND_INVENTORYTP_AUTOCAST: 0x71,
            DOTA_KEYBIND_INVENTORYNEUTRAL_AUTOCAST: 0x72,
            DOTA_KEYBIND_INVENTORY1_QUICKAUTOCAST: 0x73,
            DOTA_KEYBIND_INVENTORY2_QUICKAUTOCAST: 0x74,
            DOTA_KEYBIND_INVENTORY3_QUICKAUTOCAST: 0x75,
            DOTA_KEYBIND_INVENTORY4_QUICKAUTOCAST: 0x76,
            DOTA_KEYBIND_INVENTORY5_QUICKAUTOCAST: 0x77,
            DOTA_KEYBIND_INVENTORY6_QUICKAUTOCAST: 0x78,
            DOTA_KEYBIND_INVENTORYTP_QUICKAUTOCAST: 0x79,
            DOTA_KEYBIND_INVENTORYNEUTRAL_QUICKAUTOCAST: 0x7A,
            DOTA_KEYBIND_CONTROL_GROUP1: 0x7B,
            DOTA_KEYBIND_CONTROL_GROUP2: 0x7C,
            DOTA_KEYBIND_CONTROL_GROUP3: 0x7D,
            DOTA_KEYBIND_CONTROL_GROUP4: 0x7E,
            DOTA_KEYBIND_CONTROL_GROUP5: 0x7F,
            DOTA_KEYBIND_CONTROL_GROUP6: 0x80,
            DOTA_KEYBIND_CONTROL_GROUP7: 0x81,
            DOTA_KEYBIND_CONTROL_GROUP8: 0x82,
            DOTA_KEYBIND_CONTROL_GROUP9: 0x83,
            DOTA_KEYBIND_CONTROL_GROUP10: 0x84,
            DOTA_KEYBIND_CONTROL_GROUPCYCLE: 0x85,
            DOTA_KEYBIND_SELECT_ALLY1: 0x86,
            DOTA_KEYBIND_SELECT_ALLY2: 0x87,
            DOTA_KEYBIND_SELECT_ALLY3: 0x88,
            DOTA_KEYBIND_SELECT_ALLY4: 0x89,
            DOTA_KEYBIND_SELECT_ALLY5: 0x8A,
            DOTA_KEYBIND_SHOP_TOGGLE: 0x8B,
            DOTA_KEYBIND_SCOREBOARD_TOGGLE: 0x8C,
            DOTA_KEYBIND_COMBATLOG_TOGGLE: 0x8D,
            DOTA_KEYBIND_SCREENSHOT: 0x8E,
            DOTA_KEYBIND_ESCAPE: 0x8F,
            DOTA_KEYBIND_CONSOLE: 0x90,
            DOTA_KEYBIND_DEATH_SUMMARY: 0x91,
            DOTA_KEYBIND_LEARN_ABILITIES: 0x92,
            DOTA_KEYBIND_LEARN_STATS: 0x93,
            DOTA_KEYBIND_ACTIVATE_GLYPH: 0x94,
            DOTA_KEYBIND_ACTIVATE_RADAR: 0x95,
            DOTA_KEYBIND_PURCHASE_QUICKBUY: 0x96,
            DOTA_KEYBIND_PURCHASE_STICKY: 0x97,
            DOTA_KEYBIND_TOGGLE_BUYBACK_PROTECTION: 0x98,
            DOTA_KEYBIND_GRAB_STASH_ITEMS: 0x99,
            DOTA_KEYBIND_TOGGLE_AUTOATTACK: 0x9A,
            DOTA_KEYBIND_TOGGLE_OVERLAYMAP: 0x9B,
            DOTA_KEYBIND_OVERLAYMAP_INPUTKEY: 0x9C,
            DOTA_KEYBIND_FILTER_ENEMY: 0x9D,
            DOTA_KEYBIND_FILTER_ALLY: 0x9E,
            DOTA_KEYBIND_FILTER_HERO: 0x9F,
            DOTA_KEYBIND_FILTER_NONHERO: 0xA0,
            DOTA_KEYBIND_TAUNT: 0xA1,
            DOTA_KEYBIND_SHOP_CONSUMABLES: 0xA2,
            DOTA_KEYBIND_SHOP_ATTRIBUTES: 0xA3,
            DOTA_KEYBIND_SHOP_ARMAMENTS: 0xA4,
            DOTA_KEYBIND_SHOP_ARCANE: 0xA5,
            DOTA_KEYBIND_SHOP_BASICS: 0xA6,
            DOTA_KEYBIND_SHOP_SUPPORT: 0xA7,
            DOTA_KEYBIND_SHOP_CASTER: 0xA8,
            DOTA_KEYBIND_SHOP_WEAPONS: 0xA9,
            DOTA_KEYBIND_SHOP_ARMOR: 0xAA,
            DOTA_KEYBIND_SHOP_ARTIFACTS: 0xAB,
            DOTA_KEYBIND_SHOP_SIDE_PAGE_1: 0xAC,
            DOTA_KEYBIND_SHOP_SIDE_PAGE_2: 0xAD,
            DOTA_KEYBIND_SHOP_SECRET: 0xAE,
            DOTA_KEYBIND_SHOP_SEARCHBOX: 0xAF,
            DOTA_KEYBIND_SHOP_SLOT_1: 0xB0,
            DOTA_KEYBIND_SHOP_SLOT_2: 0xB1,
            DOTA_KEYBIND_SHOP_SLOT_3: 0xB2,
            DOTA_KEYBIND_SHOP_SLOT_4: 0xB3,
            DOTA_KEYBIND_SHOP_SLOT_5: 0xB4,
            DOTA_KEYBIND_SHOP_SLOT_6: 0xB5,
            DOTA_KEYBIND_SHOP_SLOT_7: 0xB6,
            DOTA_KEYBIND_SHOP_SLOT_8: 0xB7,
            DOTA_KEYBIND_SHOP_SLOT_9: 0xB8,
            DOTA_KEYBIND_SHOP_SLOT_10: 0xB9,
            DOTA_KEYBIND_SHOP_SLOT_11: 0xBA,
            DOTA_KEYBIND_SHOP_SLOT_12: 0xBB,
            DOTA_KEYBIND_SHOP_SLOT_13: 0xBC,
            DOTA_KEYBIND_SHOP_SLOT_14: 0xBD,
            DOTA_KEYBIND_SPEC_CAMERA_UP: 0xBE,
            DOTA_KEYBIND_SPEC_CAMERA_DOWN: 0xBF,
            DOTA_KEYBIND_SPEC_CAMERA_LEFT: 0xC0,
            DOTA_KEYBIND_SPEC_CAMERA_RIGHT: 0xC1,
            DOTA_KEYBIND_SPEC_CAMERA_GRIP: 0xC2,
            DOTA_KEYBIND_SPEC_CAMERA_SAVED_POSITION_1: 0xC3,
            DOTA_KEYBIND_SPEC_CAMERA_SAVED_POSITION_2: 0xC4,
            DOTA_KEYBIND_SPEC_CAMERA_SAVED_POSITION_3: 0xC5,
            DOTA_KEYBIND_SPEC_CAMERA_SAVED_POSITION_4: 0xC6,
            DOTA_KEYBIND_SPEC_CAMERA_SAVED_POSITION_5: 0xC7,
            DOTA_KEYBIND_SPEC_CAMERA_SAVED_POSITION_6: 0xC8,
            DOTA_KEYBIND_SPEC_CAMERA_SAVED_POSITION_7: 0xC9,
            DOTA_KEYBIND_SPEC_CAMERA_SAVED_POSITION_8: 0xCA,
            DOTA_KEYBIND_SPEC_CAMERA_SAVED_POSITION_9: 0xCB,
            DOTA_KEYBIND_SPEC_CAMERA_SAVED_POSITION_10: 0xCC,
            DOTA_KEYBIND_SPEC_UNIT_SELECT: 0xCD,
            DOTA_KEYBIND_SPEC_HERO_SELECT: 0xCE,
            DOTA_KEYBIND_SPEC_PAUSE: 0xCF,
            DOTA_KEYBIND_SPEC_CHAT: 0xD0,
            DOTA_KEYBIND_SPEC_SCOREBOARD: 0xD1,
            DOTA_KEYBIND_SPEC_INCREASE_REPLAY_SPEED: 0xD2,
            DOTA_KEYBIND_SPEC_DECREASE_REPLAY_SPEED: 0xD3,
            DOTA_KEYBIND_SPEC_STATS_ITEM: 0xD4,
            DOTA_KEYBIND_SPEC_STATS_GOLD: 0xD5,
            DOTA_KEYBIND_SPEC_STATS_XP: 0xD6,
            DOTA_KEYBIND_SPEC_STATS_FANTASY: 0xD7,
            DOTA_KEYBIND_SPEC_STATS_WINCHANCE: 0xD8,
            DOTA_KEYBIND_SPEC_FOW_TOGGLEBOTH: 0xD9,
            DOTA_KEYBIND_SPEC_FOW_TOGGLERADIENT: 0xDA,
            DOTA_KEYBIND_SPEC_FOW_TOGGLEDIRE: 0xDB,
            DOTA_KEYBIND_SPEC_OPEN_BROADCASTER_MENU: 0xDC,
            DOTA_KEYBIND_SPEC_DROPDOWN_KDA: 0xDD,
            DOTA_KEYBIND_SPEC_DROPDOWN_LASTHITS_DENIES: 0xDE,
            DOTA_KEYBIND_SPEC_DROPDOWN_LEVEL: 0xDF,
            DOTA_KEYBIND_SPEC_DROPDOWN_XP_PER_MIN: 0xE0,
            DOTA_KEYBIND_SPEC_DROPDOWN_GOLD: 0xE1,
            DOTA_KEYBIND_SPEC_DROPDOWN_TOTALGOLD: 0xE2,
            DOTA_KEYBIND_SPEC_DROPDOWN_GOLD_PER_MIN: 0xE3,
            DOTA_KEYBIND_SPEC_DROPDOWN_BUYBACK: 0xE4,
            DOTA_KEYBIND_SPEC_DROPDOWN_NETWORTH: 0xE5,
            DOTA_KEYBIND_SPEC_DROPDOWN_FANTASY: 0xE6,
            DOTA_KEYBIND_SPEC_DROPDOWN_SORT: 0xE7,
            DOTA_KEYBIND_SPEC_DROPDOWN_CLOSE: 0xE8,
            DOTA_KEYBIND_SPEC_FOCUS_PLAYER_1: 0xE9,
            DOTA_KEYBIND_SPEC_FOCUS_PLAYER_2: 0xEA,
            DOTA_KEYBIND_SPEC_FOCUS_PLAYER_3: 0xEB,
            DOTA_KEYBIND_SPEC_FOCUS_PLAYER_4: 0xEC,
            DOTA_KEYBIND_SPEC_FOCUS_PLAYER_5: 0xED,
            DOTA_KEYBIND_SPEC_FOCUS_PLAYER_6: 0xEE,
            DOTA_KEYBIND_SPEC_FOCUS_PLAYER_7: 0xEF,
            DOTA_KEYBIND_SPEC_FOCUS_PLAYER_8: 0xF0,
            DOTA_KEYBIND_SPEC_FOCUS_PLAYER_9: 0xF1,
            DOTA_KEYBIND_SPEC_FOCUS_PLAYER_10: 0xF2,
            DOTA_KEYBIND_SPEC_COACH_VIEWTOGGLE: 0xF3,
            DOTA_KEYBIND_INSPECTHEROINWORLD: 0xF4,
            DOTA_KEYBIND_CAMERA_ZOOM_IN: 0xF5,
            DOTA_KEYBIND_CAMERA_ZOOM_OUT: 0xF6,
            DOTA_KEYBIND_CONTROL_GROUPCYCLEPREV: 0xF7,
            DOTA_KEYBIND_DOTA_ALT: 0xF8,
            DOTA_KEYBIND_DOTA_ALTERNATIVE_CAST_SWITCH: 0xF9,
            DOTA_KEYBIND_COUNT: 0xFA,
        },
        EOverworldFortuneRequirement: {
            k_eOverworldFortuneRequirement_Invalid: 0x0,
            k_eOverworldFortuneRequirement_PlayGame: 0x1,
            k_eOverworldFortuneRequirement_WinGame: 0x2,
            k_eOverworldFortuneRequirement_WinGameStrengthHero: 0x3,
            k_eOverworldFortuneRequirement_WinGameIntelligenceHero: 0x4,
            k_eOverworldFortuneRequirement_WinGameAgilityHero: 0x5,
            k_eOverworldFortuneRequirement_WinGameUniversalHero: 0x6,
        },
        EGuildAuditAction: {
            k_EGuildAuditAction_Invalid: 0x0,
            k_EGuildAuditAction_GuildCreated: 0x1,
            k_EGuildAuditAction_GuildLanguageChanged: 0x2,
            k_EGuildAuditAction_GuildFlagsChanged: 0x3,
            k_EGuildAuditAction_GuildMemberJoined: 0x5,
            k_EGuildAuditAction_GuildMemberLeft: 0x6,
            k_EGuildAuditAction_GuildMemberKicked: 0x7,
            k_EGuildAuditAction_GuildMemberRoleChanged: 0x8,
            k_EGuildAuditAction_GuildLogoChanged: 0x9,
            k_EGuildAuditAction_GuildRegionChanged: 0xA,
            k_EGuildAuditAction_GuildDescriptionChanged: 0xB,
            k_EGuildAuditAction_GuildPrimaryColorChanged: 0xC,
            k_EGuildAuditAction_GuildSecondaryColorChanged: 0xD,
            k_EGuildAuditAction_GuildPatternChanged: 0xE,
            k_EGuildAuditAction_AdminClearedLogo: 0xF,
            k_EGuildAuditAction_GuildRequiredRankChanged: 0x10,
            k_EGuildAuditAction_GuildMotDChanged: 0x12,
            k_EGuildAuditAction_AdminResetName: 0x13,
            k_EGuildAuditAction_AdminResetTag: 0x14,
            k_EGuildAuditAction_AdminLock: 0x15,
            k_EGuildAuditAction_GuildNameChanged: 0x16,
            k_EGuildAuditAction_GuildTagChanged: 0x17,
            k_EGuildAuditAction_AdminPermitted: 0x18,
            k_EGuildAuditAction_AdminBlocked: 0x19,
            k_EGuildAuditAction_AdminBannedUser: 0x1A,
            k_EGuildAuditAction_AdminExonerated: 0x1B,
        },
        EWeekendTourneyRichPresenceEvent: {
            k_EWeekendTourneyRichPresenceEvent_None: 0x0,
            k_EWeekendTourneyRichPresenceEvent_StartedMatch: 0x1,
            k_EWeekendTourneyRichPresenceEvent_WonMatch: 0x2,
            k_EWeekendTourneyRichPresenceEvent_Eliminated: 0x3,
        },
        EArtyTeam: {
            k_eNoTeam: 0x0,
            k_eYou: 0x1,
            k_eThem: 0x2,
            k_eNeutral: 0x4,
            k_ePlayers: 0x3,
            k_eAllTeams: 0xFF,
        },
        ESurvivorsUpgradeRarity: {
            RARITY_INVALID: 0xFFFFFFFFFFFFFFFF,
            RARITY_COMMON: 0x0,
            RARITY_COUNT: 0x1,
        },
        EFishingGameFishCategory: {
            k_eFishingGameFishCategory_Invalid: 0x0,
            k_eFishingGameFishCategory_Trash: 0x1,
            k_eFishingGameFishCategory_Common: 0x2,
            k_eFishingGameFishCategory_Uncommon: 0x3,
            k_eFishingGameFishCategory_Rare: 0x4,
            k_eFishingGameFishCategory_UltraRare: 0x5,
        },
        ESurvivorsAttackTargeting: {
            INVALID_TARGET: 0xFFFFFFFFFFFFFFFF,
            CLOSEST_TARGET: 0x0,
            RANDOM_TARGET_IN_RANGE: 0x1,
            STRONGEST_TARGET_IN_RANGE: 0x2,
            RANDOM_TARGET_AT_RANGE: 0x3,
            PLAYER_FACING: 0x4,
        },
        ECoachTeammateRating: {
            k_ECoachTeammateRating_None: 0x0,
            k_ECoachTeammateRating_Positive: 0x1,
            k_ECoachTeammateRating_Negative: 0x2,
            k_ECoachTeammateRating_Abusive: 0x3,
        },
        EShmupEventType: {
            k_eShmupEventType_Invalid: 0xFFFFFFFFFFFFFFFF,
            k_eShmupEventType_SpawnEnemy: 0x0,
            k_eShmupEventType_UI: 0x1,
        },
        DOTAVisualNovelTextColor_t: {
            VN_TEXT_COLOR_DEFAULT: 0x0,
            VN_TEXT_COLOR_GRAY: 0x1,
            VN_TEXT_COLOR_BLUE: 0x2,
            VN_TEXT_COLOR_LIGHT_BLUE: 0x3,
            VN_TEXT_COLOR_GREEN: 0x4,
            VN_TEXT_COLOR_RED: 0x5,
            VN_TEXT_COLOR_LIGHT_GRAY: 0x6,
            VN_TEXT_COLOR_DARK_GRAY: 0x7,
            VN_TEXT_COLOR_DARK_RED: 0x8,
            VN_TEXT_COLOR_ORANGE: 0x9,
            VN_TEXT_COLOR_WHITE: 0xA,
            VN_TEXT_COLOR_PURPLE: 0xB,
            VN_TEXT_COLOR_YELLOW: 0xC,
        },
        ECandyShopAuditAction: {
            k_ECandyShopAuditAction_Invalid: 0x0,
            k_ECandyShopAuditAction_SupportModify: 0x1,
            k_ECandyShopAuditAction_PurchaseReward: 0x2,
            k_ECandyShopAuditAction_OpenBags: 0x3,
            k_ECandyShopAuditAction_RerollRewards: 0x4,
            k_ECandyShopAuditAction_DoVariableExchange: 0x5,
            k_ECandyShopAuditAction_DoExchange: 0x6,
            k_ECandyShopAuditAction_DEPRECATED_EventActionGrantInventorySizeIncrease: 0x7,
            k_ECandyShopAuditAction_EventActionGrantRerollChargesIncrease: 0x8,
            k_ECandyShopAuditAction_EventActionGrantUpgrade_InventorySize: 0x64,
            k_ECandyShopAuditAction_EventActionGrantUpgrade_RewardShelf: 0x65,
            k_ECandyShopAuditAction_EventActionGrantUpgrade_ExtraExchangeRecipe: 0x66,
        },
        EShmupPathEventType: {
            k_eShmupPathEventType_Invalid: 0xFFFFFFFFFFFFFFFF,
            k_eShmupPathEventType_Speed: 0x0,
            k_eShmupPathEventType_Shoot: 0x1,
            k_eShmupPathEventType_StorePlayerPosition: 0x2,
            k_eShmupPathEventType_DestroySelf: 0x3,
        },
        PortraitUnitDirection_t: {
            PORTRAIT_UNIT_DIRECTION_INVALID: 0xFFFFFFFFFFFFFFFF,
            PORTRAIT_UNIT_DIRECTION_FORWARD: 0x0,
            PORTRAIT_UNIT_DIRECTION_BACKWARD: 0x1,
        },
        ESurvivorsBossState: {
            INVALID: 0xFFFFFFFFFFFFFFFF,
            IDLE: 0x0,
            CASTING_BURNING_GROUND: 0x1,
            START_CASTING_RADIATE_RAYS: 0x2,
            CASTING_RADIATE_RAYS: 0x3,
            END_CASTING_RADIATE_RAYS: 0x4,
            CASTING_DEMON_PORTALS: 0x5,
            START_DESTROYING_TOWER: 0x6,
            END_DESTROYING_TOWER: 0x7,
            CASTING_MAGIC_MISSILE: 0x8,
        },
        EArtyGraphicsType: {
            k_eSprite: 0x0,
            k_eAnimatedSprite: 0x1,
            k_eProgressBar: 0x2,
            k_eUnit: 0x3,
            k_eHero: 0x4,
            k_eParticle: 0x5,
        },
        DOTA_2013PassportSelectionIndices: {
            PP13_SEL_ALLSTAR_PLAYER_0: 0x0,
            PP13_SEL_ALLSTAR_PLAYER_1: 0x1,
            PP13_SEL_ALLSTAR_PLAYER_2: 0x2,
            PP13_SEL_ALLSTAR_PLAYER_3: 0x3,
            PP13_SEL_ALLSTAR_PLAYER_4: 0x4,
            PP13_SEL_ALLSTAR_PLAYER_5: 0x5,
            PP13_SEL_ALLSTAR_PLAYER_6: 0x6,
            PP13_SEL_ALLSTAR_PLAYER_7: 0x7,
            PP13_SEL_ALLSTAR_PLAYER_8: 0x8,
            PP13_SEL_ALLSTAR_PLAYER_9: 0x9,
            PP13_SEL_QUALPRED_WEST_0: 0xA,
            PP13_SEL_QUALPRED_WEST_1: 0xB,
            PP13_SEL_QUALPRED_WEST_2: 0xC,
            PP13_SEL_QUALPRED_WEST_3: 0xD,
            PP13_SEL_QUALPRED_WEST_4: 0xE,
            PP13_SEL_QUALPRED_WEST_5: 0xF,
            PP13_SEL_QUALPRED_WEST_6: 0x10,
            PP13_SEL_QUALPRED_WEST_7: 0x11,
            PP13_SEL_QUALPRED_WEST_8: 0x12,
            PP13_SEL_QUALPRED_WEST_9: 0x13,
            PP13_SEL_QUALPRED_WEST_10: 0x14,
            PP13_SEL_QUALPRED_WEST_11: 0x15,
            PP13_SEL_QUALPRED_WEST_12: 0x16,
            PP13_SEL_QUALPRED_WEST_13: 0x17,
            PP13_SEL_QUALPRED_WEST_14: 0x18,
            PP13_SEL_QUALPRED_EAST_0: 0x19,
            PP13_SEL_QUALPRED_EAST_1: 0x1A,
            PP13_SEL_QUALPRED_EAST_2: 0x1B,
            PP13_SEL_QUALPRED_EAST_3: 0x1C,
            PP13_SEL_QUALPRED_EAST_4: 0x1D,
            PP13_SEL_QUALPRED_EAST_5: 0x1E,
            PP13_SEL_QUALPRED_EAST_6: 0x1F,
            PP13_SEL_QUALPRED_EAST_7: 0x20,
            PP13_SEL_QUALPRED_EAST_8: 0x21,
            PP13_SEL_QUALPRED_EAST_9: 0x22,
            PP13_SEL_QUALPRED_EAST_10: 0x23,
            PP13_SEL_QUALPRED_EAST_11: 0x24,
            PP13_SEL_QUALPRED_EAST_12: 0x25,
            PP13_SEL_QUALPRED_EAST_13: 0x26,
            PP13_SEL_QUALPRED_EAST_14: 0x27,
            PP13_SEL_TEAMCUP_TEAM: 0x28,
            PP13_SEL_TEAMCUP_PLAYER: 0x29,
            PP13_SEL_TEAMCUP_TEAM_LOCK: 0x2A,
            PP13_SEL_TEAMCUP_PLAYER_LOCK: 0x2B,
            PP13_SEL_EVENTPRED_0: 0x2C,
            PP13_SEL_EVENTPRED_1: 0x2D,
            PP13_SEL_EVENTPRED_2: 0x2E,
            PP13_SEL_EVENTPRED_3: 0x2F,
            PP13_SEL_EVENTPRED_4: 0x30,
            PP13_SEL_EVENTPRED_5: 0x31,
            PP13_SEL_EVENTPRED_6: 0x32,
            PP13_SEL_EVENTPRED_7: 0x33,
            PP13_SEL_EVENTPRED_8: 0x34,
            PP13_SEL_EVENTPRED_9: 0x35,
            PP13_SEL_EVENTPRED_10: 0x36,
            PP13_SEL_EVENTPRED_11: 0x37,
            PP13_SEL_EVENTPRED_12: 0x38,
            PP13_SEL_EVENTPRED_13: 0x39,
            PP13_SEL_EVENTPRED_14: 0x3A,
            PP13_SEL_EVENTPRED_15: 0x3B,
            PP13_SEL_EVENTPRED_16: 0x3C,
            PP13_SEL_EVENTPRED_17: 0x3D,
            PP13_SEL_EVENTPRED_18: 0x3E,
            PP13_SEL_EVENTPRED_19: 0x3F,
            PP13_SEL_EVENTPRED_20: 0x40,
            PP13_SEL_EVENTPRED_21: 0x41,
            PP13_SEL_EVENTPRED_22: 0x42,
            PP13_SEL_EVENTPRED_23: 0x43,
            PP13_SEL_EVENTPRED_24: 0x44,
            PP13_SEL_EVENTPRED_25: 0x45,
            PP13_SEL_EVENTPRED_26: 0x46,
            PP13_SEL_EVENTPRED_27: 0x47,
            PP13_SEL_EVENTPRED_28: 0x48,
            PP13_SEL_EVENTPRED_29: 0x49,
            PP13_SEL_EVENTPRED_30: 0x4A,
            PP13_SEL_EVENTPRED_31: 0x4B,
            PP13_SEL_EVENTPRED_32: 0x4C,
            PP13_SEL_EVENTPRED_33: 0x4D,
            PP13_SEL_EVENTPRED_34: 0x4E,
            PP13_SEL_EVENTPRED_35: 0x4F,
            PP13_SEL_EVENTPRED_36: 0x50,
            PP13_SEL_EVENTPRED_37: 0x51,
            PP13_SEL_EVENTPRED_38: 0x52,
            PP13_SEL_EVENTPRED_39: 0x53,
            PP13_SEL_EVENTPRED_40: 0x54,
            PP13_SEL_EVENTPRED_41: 0x55,
            PP13_SEL_EVENTPRED_42: 0x56,
            PP13_SEL_EVENTPRED_43: 0x57,
            PP13_SEL_SOLO_0: 0x58,
            PP13_SEL_SOLO_1: 0x59,
            PP13_SEL_SOLO_2: 0x5A,
            PP13_SEL_SOLO_3: 0x5B,
            PP13_SEL_SOLO_4: 0x5C,
            PP13_SEL_SOLO_5: 0x5D,
            PP13_SEL_SOLO_6: 0x5E,
            PP13_SEL_SOLO_7: 0x5F,
        },
        SteamUGCMatchingUGCType: {
            Items: 0x0,
            Items_Mtx: 0x1,
            Items_ReadyToUse: 0x2,
            Collections: 0x3,
            Artwork: 0x4,
            Videos: 0x5,
            Screenshots: 0x6,
            AllGuides: 0x7,
            WebGuides: 0x8,
            IntegratedGuides: 0x9,
            UsableInGame: 0xA,
            ControllerBindings: 0xB,
            GameManagedItems: 0xC,
            All: 0xFFFFFFFFFFFFFFFF,
        },
        EStartFindingMatchResult: {
            k_EStartFindingMatchResult_Invalid: 0x0,
            k_EStartFindingMatchResult_OK: 0x1,
            k_EStartFindingMatchResult_AlreadySearching: 0x2,
            k_EStartFindingMatchResult_FailGeneric: 0x64,
            k_EStartFindingMatchResult_FailedIgnore: 0x65,
            k_EStartFindingMatchResult_MatchmakingDisabled: 0x66,
            k_EStartFindingMatchResult_RegionOffline: 0x67,
            k_EStartFindingMatchResult_MatchmakingCooldown: 0x68,
            k_EStartFindingMatchResult_ClientOutOfDate: 0x69,
            k_EStartFindingMatchResult_CompetitiveNoLowPriority: 0x6A,
            k_EStartFindingMatchResult_CompetitiveNotUnlocked: 0x6B,
            k_EStartFindingMatchResult_GameModeNotUnlocked: 0x6C,
            k_EStartFindingMatchResult_CompetitiveNotEnoughPlayTime: 0x6D,
            k_EStartFindingMatchResult_MissingInitialSkill: 0x6E,
            k_EStartFindingMatchResult_CompetitiveRankSpreadTooLarge: 0x6F,
            k_EStartFindingMatchResult_MemberAlreadyInLobby: 0x70,
            k_EStartFindingMatchResult_MemberNotVACVerified: 0x71,
            k_EStartFindingMatchResult_WeekendTourneyBadPartySize: 0x72,
            k_EStartFindingMatchResult_WeekendTourneyTeamBuyInTooSmall: 0x73,
            k_EStartFindingMatchResult_WeekendTourneyIndividualBuyInTooLarge: 0x74,
            k_EStartFindingMatchResult_WeekendTourneyTeamBuyInTooLarge: 0x75,
            k_EStartFindingMatchResult_MemberMissingEventOwnership: 0x76,
            k_EStartFindingMatchResult_WeekendTourneyNotUnlocked: 0x77,
            k_EStartFindingMatchResult_WeekendTourneyRecentParticipation: 0x78,
            k_EStartFindingMatchResult_MemberMissingAnchoredPhoneNumber: 0x79,
            k_EStartFindingMatchResult_NotMemberOfClan: 0x7A,
            k_EStartFindingMatchResult_CoachesChallengeBadPartySize: 0x7B,
            k_EStartFindingMatchResult_CoachesChallengeRequirementsNotMet: 0x7C,
            k_EStartFindingMatchResult_InvalidRoleSelections: 0x7D,
            k_EStartFindingMatchResult_PhoneNumberDiscrepancy: 0x7E,
            k_EStartFindingMatchResult_NoQueuePoints: 0x7F,
            k_EStartFindingMatchResult_MemberMissingGauntletFlag: 0x80,
            k_EStartFindingMatchResult_MemberGauntletTooRecent: 0x81,
            k_EStartFindingMatchResult_DifficultyNotUnlocked: 0x82,
            k_EStartFindingMatchResult_CoachesNotAllowedInParty: 0x83,
            k_EStartFindingMatchResult_MatchmakingBusy: 0x84,
            k_EStartFindingMatchResult_SteamChinaBanned: 0x85,
            k_EStartFindingMatchResult_SteamChinaInvalidMixedParty: 0x86,
            k_EStartFindingMatchResult_RestrictedFromRanked: 0x87,
            k_EStartFindingMatchResult_RankPreventsParties: 0x88,
            k_EStartFindingMatchResult_RegisteredNameRequired: 0x89,
        },
        EShmupBulletPattern: {
            k_eShmupBulletPattern_Invalid: 0xFFFFFFFFFFFFFFFF,
            k_eShmupBulletPattern_Radial: 0x0,
            k_eShmupBulletPattern_HorizontalLeft: 0x1,
            k_eShmupBulletPattern_HorizontalRight: 0x2,
            k_eShmupBulletPattern_VerticalInwards: 0x3,
            k_eShmupBulletPattern_VerticalOutwards: 0x4,
            k_eShmupBulletPattern_FixedDirection: 0x5,
            k_eShmupBulletPattern_Player: 0x6,
            k_eShmupBulletPattern_PlayerSpreadRandom: 0x7,
            k_eShmupBulletPattern_PlayerSpreadEven: 0x8,
        },
        EDOTATriviaAnswerResult: {
            k_EDOTATriviaAnswerResult_Success: 0x0,
            k_EDOTATriviaAnswerResult_InvalidQuestion: 0x1,
            k_EDOTATriviaAnswerResult_InvalidAnswer: 0x2,
            k_EDOTATriviaAnswerResult_QuestionLocked: 0x3,
            k_EDOTATriviaAnswerResult_AlreadyAnswered: 0x4,
            k_EDOTATriviaAnswerResult_TriviaDisabled: 0x5,
        },
        ETournamentNodeState: {
            k_ETournamentNodeState_Unknown: 0x0,
            k_ETournamentNodeState_Canceled: 0x1,
            k_ETournamentNodeState_TeamsNotYetAssigned: 0x2,
            k_ETournamentNodeState_InBetweenGames: 0x3,
            k_ETournamentNodeState_GameInProgress: 0x4,
            k_ETournamentNodeState_A_Won: 0x5,
            k_ETournamentNodeState_B_Won: 0x6,
            k_ETournamentNodeState_A_WonByForfeit: 0x7,
            k_ETournamentNodeState_B_WonByForfeit: 0x8,
            k_ETournamentNodeState_A_Bye: 0x9,
            k_ETournamentNodeState_A_Abandoned: 0xA,
            k_ETournamentNodeState_ServerFailure: 0xB,
            k_ETournamentNodeState_A_TimeoutForfeit: 0xC,
            k_ETournamentNodeState_A_TimeoutRefund: 0xD,
        },
        EShowcaseHeroPlusFlag: {
            k_eShowcaseHeroPlusFlag_None: 0x0,
            k_eShowcaseHeroPlusFlag_BadgePosTop: 0x1,
            k_eShowcaseHeroPlusFlag_BadgePosBottom: 0x2,
            k_eShowcaseHeroPlusFlag_BadgePosLeft: 0x4,
            k_eShowcaseHeroPlusFlag_BadgePosRight: 0x8,
            k_eShowcaseHeroPlusFlag_ShowRelics: 0x10,
        },
        ETalentContentAssetStatus: {
            k_eTalentContentAssetStatus_None: 0x0,
            k_eTalentContentAssetStatus_Approved: 0x1,
            k_eTalentContentAssetStatus_Rejected: 0x2,
        },
        EUnderDraftResponse: {
            k_eInternalError: 0x0,
            k_eSuccess: 0x1,
            k_eNoGold: 0x2,
            k_eInvalidSlot: 0x3,
            k_eNoBenchSpace: 0x4,
            k_eNoTickets: 0x5,
            k_eEventNotOwned: 0x6,
            k_eInvalidReward: 0x7,
            k_eHasBigReward: 0x8,
            k_eNoGCConnection: 0x9,
            k_eTooBusy: 0xA,
            k_eCantRollBack: 0xB,
        },
        DOTAKeybindTemplate_t: {
            DOTA_KEYBIND_TEMPLATE_ARROW: 0x0,
            DOTA_KEYBIND_TEMPLATE_WASD: 0x1,
            DOTA_KEYBIND_TEMPLATE_LEGACY: 0x2,
            DOTA_KEYBIND_TEMPLATE_MMO: 0x3,
            DOTA_KEYBIND_TEMPLATE_LOL: 0x4,
            DOTA_KEYBIND_TEMPLATE_HON: 0x5,
            DOTA_KEYBIND_TEMPLATE_SMITE: 0x6,
            DOTA_KEYBIND_TEMPLATE_COUNT: 0x7,
        },
        ECandyShopRewardType: {
            k_eCandyShopRewardType_None: 0x0,
            k_eCandyShopRewardType_Item: 0x1,
            k_eCandyShopRewardType_EventAction: 0x2,
            k_eCandyShopRewardType_EventPoints: 0x3,
        },
        ETournamentGameState: {
            k_ETournamentGameState_Unknown: 0x0,
            k_ETournamentGameState_Canceled: 0x1,
            k_ETournamentGameState_Scheduled: 0x2,
            k_ETournamentGameState_Active: 0x3,
            k_ETournamentGameState_RadVictory: 0x14,
            k_ETournamentGameState_DireVictory: 0x15,
            k_ETournamentGameState_RadVictoryByForfeit: 0x16,
            k_ETournamentGameState_DireVictoryByForfeit: 0x17,
            k_ETournamentGameState_ServerFailure: 0x28,
            k_ETournamentGameState_NotNeeded: 0x29,
        },
        EOverworldFortuneModifier: {
            k_eOverworldFortuneModifier_Invalid: 0x0,
            k_eOverworldFortuneModifier_DoubleReward: 0x1,
            k_eOverworldFortuneModifier_TripleReward: 0x2,
            k_eOverworldFortuneModifier_OneChance: 0x3,
            k_eOverworldFortuneModifier_ReceiveAdditionalRandomReward: 0x4,
            k_eOverworldFortuneModifier_RerollFortune: 0x5,
        },
        SteamUniverse: {
            Invalid: 0x0,
            Internal: 0x3,
            Dev: 0x4,
            Beta: 0x2,
            Public: 0x1,
        },
        ESurvivorsEnemySeparationLayer: {
            OFF: 0xFFFFFFFFFFFFFFFF,
            SMALL: 0x1,
            LARGE: 0x2,
            ELITE: 0x3,
        },
        EGuildEventAuditAction: {
            k_EGuildEventAuditAction_Invalid: 0x0,
            k_EGuildEventAuditAction_DevGrant: 0x1,
            k_EGuildEventAuditAction_CompleteContract: 0x2,
            k_EGuildEventAuditAction_CompleteChallenge: 0x3,
            k_EGuildEventAuditAction_CompleteMatch_Winner: 0x4,
            k_EGuildEventAuditAction_ChallengeProgress: 0x5,
            k_EGuildEventAuditAction_CompleteMatch_Loser: 0x6,
            k_EGuildEventAuditAction_WeeklyLeaderboard: 0x7,
            k_EGuildEventAuditAction_ManualGrant: 0x8,
        },
        EPrivateCoachingSessionState: {
            k_ePrivateCoachingSessionState_Invalid: 0x0,
            k_ePrivateCoachingSessionState_SearchingForCoach: 0x1,
            k_ePrivateCoachingSessionState_CoachAssigned: 0x2,
            k_ePrivateCoachingSessionState_Finished: 0x3,
            k_ePrivateCoachingSessionState_Expired: 0x4,
            k_ePrivateCoachingSessionState_Abandoned: 0x5,
        },
        ETeamInviteResult: {
            TEAM_INVITE_SUCCESS: 0x0,
            TEAM_INVITE_FAILURE_INVITE_REJECTED: 0x1,
            TEAM_INVITE_FAILURE_INVITE_TIMEOUT: 0x2,
            TEAM_INVITE_ERROR_TEAM_AT_MEMBER_LIMIT: 0x3,
            TEAM_INVITE_ERROR_TEAM_LOCKED: 0x4,
            TEAM_INVITE_ERROR_INVITEE_NOT_AVAILABLE: 0x5,
            TEAM_INVITE_ERROR_INVITEE_BUSY: 0x6,
            TEAM_INVITE_ERROR_INVITEE_ALREADY_MEMBER: 0x7,
            TEAM_INVITE_ERROR_INVITEE_AT_TEAM_LIMIT: 0x8,
            TEAM_INVITE_ERROR_INVITEE_INSUFFICIENT_PLAY_TIME: 0x9,
            TEAM_INVITE_ERROR_INVITER_INVALID_ACCOUNT_TYPE: 0xA,
            TEAM_INVITE_ERROR_INVITER_NOT_ADMIN: 0xB,
            TEAM_INVITE_ERROR_INCORRECT_USER_RESPONDED: 0xC,
            TEAM_INVITE_ERROR_UNSPECIFIED: 0xD,
        },
        PlayerOrderIssuer_t: {
            DOTA_ORDER_ISSUER_SELECTED_UNITS: 0x0,
            DOTA_ORDER_ISSUER_CURRENT_UNIT_ONLY: 0x1,
            DOTA_ORDER_ISSUER_HERO_ONLY: 0x2,
            DOTA_ORDER_ISSUER_PASSED_UNIT_ONLY: 0x3,
        },
        EShowcaseItemFlag_Hero: {
            k_eShowcaseItemFlag_Hero_None: 0x0,
            k_eShowcaseItemFlag_Hero_ShowPedestal: 0x1,
            k_eShowcaseItemFlag_Hero_UseCurrentLoadout: 0x2,
            k_eShowcaseItemFlag_Hero_ShowHeroCard: 0x4,
            k_eShowcaseItemFlag_Hero_HeroCardHideName: 0x8,
            k_eShowcaseItemFlag_Hero_HeroCardUseMovie: 0x10,
        },
        ESurvivorsAreaAttackOrigin: {
            INVALID_ORIGIN: 0xFFFFFFFFFFFFFFFF,
            PLAYER_ORIGIN: 0x0,
            RANDOM_ENEMY_ORIGIN: 0x1,
        },
        PortraitDisplayMode_t: {
            PORTRAIT_DISPLAY_MODE_INVALID: 0xFFFFFFFFFFFFFFFF,
            PORTRAIT_DISPLAY_MODE_LOADOUT: 0x0,
            PORTRAIT_DISPLAY_MODE_LOADOUT_DIRE: 0x1,
            PORTRAIT_DISPLAY_MODE_LOADOUT_SMALL: 0x2,
            PORTRAIT_DISPLAY_MODE_TREASURE_SMALL: 0x3,
        },
        ETeamFanContentAssetStatus: {
            k_eFanContentAssetStatus_None: 0x0,
            k_eFanContentAssetStatus_Approved: 0x1,
            k_eFanContentAssetStatus_Rejected: 0x2,
        },
        EPurchaseHeroRelicResult: {
            k_EPurchaseHeroRelicResult_Success: 0x0,
            k_EPurchaseHeroRelicResult_FailedToSend: 0x1,
            k_EPurchaseHeroRelicResult_NotEnoughPoints: 0x2,
            k_EPurchaseHeroRelicResult_InternalServerError: 0x3,
            k_EPurchaseHeroRelicResult_PurchaseNotAllowed: 0x4,
            k_EPurchaseHeroRelicResult_InvalidRelic: 0x5,
            k_EPurchaseHeroRelicResult_AlreadyOwned: 0x6,
            k_EPurchaseHeroRelicResult_InvalidRarity: 0x7,
        },
        ESurvivorsEnemySpawnPositionsLayer: {
            INVALID: 0xFFFFFFFFFFFFFFFF,
            ENEMY_MAIN: 0x1,
            DESTRUCTIBLE_MAIN: 0x2,
            ENEMY_BOSS_ROOM: 0x3,
        },
        ESurvivorsEnemyMovementBehavior: {
            ENEMY_MOVEMENT_BEHAVIOR_INVALID: 0xFFFFFFFFFFFFFFFF,
            ENEMY_MOVEMENT_BEHAVIOR_TRACKING: 0x0,
            ENEMY_MOVEMENT_BEHAVIOR_LINEAR: 0x1,
            ENEMY_MOVEMENT_BEHAVIOR_LINEAR_SIN_WAVE: 0x2,
            ENEMY_MOVEMENT_BEHAVIOR_STATIONARY: 0x3,
        },
        EPrivateCoachingSessionMemberFlag: {
            k_EPrivateCoachingSessionMemberFlag_Requester: 0x1,
            k_EPrivateCoachingSessionMemberFlag_Coach: 0x2,
            k_EPrivateCoachingSessionMemberFlag_LeftSession: 0x4,
        },
        ERoadToTIQuestType: {
            k_RoadToTIQuestType_Match: 0x0,
            k_RoadToTIQuestType_Player: 0x1,
        },
        OrderQueueBehavior_t: {
            DOTA_ORDER_QUEUE_DEFAULT: 0x0,
            DOTA_ORDER_QUEUE_NEVER: 0x1,
            DOTA_ORDER_QUEUE_ALWAYS: 0x2,
        },
        EDOTADraftTriviaAnswerResult: {
            k_EDOTADraftTriviaAnswerResult_Success: 0x0,
            k_EDOTADraftTriviaAnswerResult_InvalidMatchID: 0x1,
            k_EDOTADraftTriviaAnswerResult_AlreadyAnswered: 0x2,
            k_EDOTADraftTriviaAnswerResult_InternalError: 0x3,
            k_EDOTADraftTriviaAnswerResult_TriviaDisabled: 0x4,
            k_EDOTADraftTriviaAnswerResult_GCDown: 0x5,
        },
        EPlayerCoachMatchFlag: {
            k_EPlayerCoachMatchFlag_EligibleForRewards: 0x1,
            k_EPlayerCoachMatchFlag_PrivateCoach: 0x2,
        },
        NeutralCampStackPullAlarmType_t: {
            DOTA_NEUTRAL_CAMP_STACK_PULL_ALARM_TYPE_STACK: 0x1,
            DOTA_NEUTRAL_CAMP_STACK_PULL_ALARM_TYPE_PULL: 0x2,
        },
        SurvivorsAttackIndicatorShape_t: {
            k_eSurvivorsShape_Undefined: 0x0,
            k_eSurvivorsShape_Circle: 0x1,
            k_eSurvivorsShape_Rectangle: 0x2,
        },
        EFlappySkywrathInputAction: {
            Jump: 0x0,
            Dash: 0x1,
            Dive: 0x2,
            Glide: 0x3,
            COUNT: 0x4,
            None: 0xFFFFFFFFFFFFFFFF,
        },
        ETalentContentStatus: {
            TALENT_CONTENT_STATUS_INVALID: 0x0,
            TALENT_CONTENT_STATUS_PENDING: 0x1,
            TALENT_CONTENT_STATUS_EVALUATED: 0x2,
            TALENT_CONTENT_STATUS_REJECTED: 0x3,
            TALENT_CONTENT_STATUS_APPROVED: 0x4,
        },
        EFightingGameActionID: {
            INVALID_ACTION_DEFINITION: 0xFFFFFFFFFFFFFFFF,
            IDLE_ACTION_DEFINITION: 0x0,
            BLOCKSTUN_ACTION_DEFINITION: 0x1,
            HITSTUN_ACTION_DEFINITION: 0x2,
            KNOCKED_DOWN_ACTION_DEFINITION: 0x3,
            DASH_ACTION_DEFINITION: 0x4,
            BACKDASH_ACTION_DEFINITION: 0x5,
            GUARDBREAK_ACTION_DEFINITION: 0x6,
            VICTORY_ACTION_DEFINITION: 0x7,
            DEFEAT_ACTION_DEFINITION: 0x8,
            JAB_ACTION_DEFINITION: 0x9,
            JAB_2_ACTION_DEFINITION: 0xA,
            JAB_3_ACTION_DEFINITION: 0xB,
            JAB_4_ACTION_DEFINITION: 0xC,
            CROSS_ACTION_DEFINITION: 0xD,
            CROSS_2_ACTION_DEFINITION: 0xE,
            CROSS_3_ACTION_DEFINITION: 0xF,
            HEAVY_ACTION_DEFINITION: 0x10,
            SWEEP_ACTION_DEFINITION: 0x11,
            SWEEP_2_ACTION_DEFINITION: 0x12,
            KICK_1_ACTION_DEFINITION: 0x13,
            KICK_2_ACTION_DEFINITION: 0x14,
            KICK_3_ACTION_DEFINITION: 0x15,
            FINISHER_ACTION_DEFINITION: 0x16,
            FINISHER_2_ACTION_DEFINITION: 0x17,
            PROJECTILE_ACTION_DEFINITION: 0x18,
            WALRUS_PUNCH_ACTION_DEFINITION: 0x19,
            SWAP_ACTION_DEFINITION: 0x1A,
            SWAP_RECOVERY_ACTION_DEFINITION: 0x1B,
            QUILLSPRAY_START_ACTION_DEFINITION: 0x1C,
            QUILLSPRAY_2_ACTION_DEFINITION: 0x1D,
            QUILLSPRAY_3_ACTION_DEFINITION: 0x1E,
            QUILLSPRAY_4_ACTION_DEFINITION: 0x1F,
            QUILLSPRAY_FINISH_ACTION_DEFINITION: 0x20,
            UNLEASH_ACTION_DEFINITION: 0x21,
            DASH_STRIKE_ACTION_DEFINITION: 0x22,
            DASH_STRIKE_2_ACTION_DEFINITION: 0x23,
            STARBREAKER_1_ACTION_DEFINITION: 0x24,
            STARBREAKER_2_ACTION_DEFINITION: 0x25,
            STARBREAKER_3_ACTION_DEFINITION: 0x26,
            LUMINOSITY_ACTION_DEFINITION: 0x27,
        },
        EShowcaseItemFlag: {
            k_eShowcaseItemFlag_None: 0x0,
            k_eShowcaseItemFlag_FlipHorizontally: 0x1,
        },
        EFightingGameButtonBit: {
            kNONE_BIT: 0x0,
            kBUTTON_FORWARD_BIT: 0x1,
            kBUTTON_BACK_BIT: 0x2,
            kBUTTON_DOWN_BIT: 0x4,
            kBUTTON_UP_BIT: 0x8,
            kBUTTON_ATTACK_BIT: 0x10,
            kBUTTON_SPECIAL_BIT: 0x20,
        },
        ETournamentState: {
            k_ETournamentState_Unknown: 0x0,
            k_ETournamentState_CanceledByAdmin: 0x1,
            k_ETournamentState_Completed: 0x2,
            k_ETournamentState_Merged: 0x3,
            k_ETournamentState_ServerFailure: 0x4,
            k_ETournamentState_TeamAbandoned: 0x5,
            k_ETournamentState_TeamTimeoutForfeit: 0x6,
            k_ETournamentState_TeamTimeoutRefund: 0x7,
            k_ETournamentState_ServerFailureGrantedVictory: 0x8,
            k_ETournamentState_TeamTimeoutGrantedVictory: 0x9,
            k_ETournamentState_InProgress: 0x64,
            k_ETournamentState_WaitingToMerge: 0x65,
        },
        PlayerUltimateStateOrTime_t: {
            PLAYER_ULTIMATE_STATE_READY: 0x0,
            PLAYER_ULTIMATE_STATE_NO_MANA: 0xFFFFFFFFFFFFFFFF,
            PLAYER_ULTIMATE_STATE_NOT_LEVELED: 0xFFFFFFFFFFFFFFFE,
            PLAYER_ULTIMATE_STATE_HIDDEN: 0xFFFFFFFFFFFFFFFD,
        },
        EArtyGameObjectType: {
            k_eTypeObject: 0x0,
            k_eTypeShot: 0x1,
            k_eTypeTrail: 0x2,
            k_eTypeCannon: 0x3,
            k_eTypePlayer: 0x4,
            k_eTypeEnemy: 0x5,
            k_eTypeFX: 0x6,
            k_eTypeUI: 0x7,
        },
        ETeamFanContentStatus: {
            TEAM_FAN_CONTENT_STATUS_INVALID: 0x0,
            TEAM_FAN_CONTENT_STATUS_PENDING: 0x1,
            TEAM_FAN_CONTENT_STATUS_EVALUATED: 0x2,
        },
        EOverworldCharacterVisibility: {
            CompleteNode: 0x1,
            ActiveNode: 0x2,
            LockedNode: 0x4,
            AlwaysVisible: 0x8,
        },
        EMatch3LevelFlags: {
            k_eMatch3LevelFlag_None: 0x0,
            k_eMatch3LevelFlag_Boss: 0x1,
            k_eMatch3LevelFlag_Friendly: 0x2,
        },
        P2P_Messages: {
            p2p_TextMessage: 0x100,
            p2p_Voice: 0x101,
            p2p_Ping: 0x102,
            p2p_VRAvatarPosition: 0x103,
            p2p_WatchSynchronization: 0x104,
            p2p_FightingGame_GameData: 0x105,
            p2p_FightingGame_Connection: 0x106,
        },
        EOverworldNodeFlags: {
            Premium: 0x1,
            MainQuest: 0x2,
            SideQuest: 0x4,
            DelayStyles: 0x8,
            Shortcut: 0x10,
            InvisibleUntilNearby: 0x20,
            Secret: 0x40,
            FinalNode: 0x80,
            Invisible: 0x100,
        },
        DotaCustomUIType_t: {
            DOTA_CUSTOM_UI_TYPE_HUD: 0x0,
            DOTA_CUSTOM_UI_TYPE_HERO_SELECTION: 0x1,
            DOTA_CUSTOM_UI_TYPE_PREGAME_STRATEGY: 0x2,
            DOTA_CUSTOM_UI_TYPE_GAME_INFO: 0x3,
            DOTA_CUSTOM_UI_TYPE_GAME_SETUP: 0x4,
            DOTA_CUSTOM_UI_TYPE_FLYOUT_SCOREBOARD: 0x5,
            DOTA_CUSTOM_UI_TYPE_HUD_TOP_BAR: 0x6,
            DOTA_CUSTOM_UI_TYPE_END_SCREEN: 0x7,
            DOTA_CUSTOM_UI_TYPE_COUNT: 0x8,
            DOTA_CUSTOM_UI_TYPE_INVALID: 0xFFFFFFFFFFFFFFFF,
        },
        C_DOTA_BaseNPC_Shop: {
        },
        CDOTA_Item_SeedsOfSerenity: {
        },
        C_DOTA_Item_Recipe_Possessed_Mask: {
        },
        C_DOTA_Item_Arcane_Ring: {
        },
        C_DOTA_Item_Grove_Bow: {
        },
        C_DOTA_Item_Revenants_Brooch: {
        },
        C_DOTA_Ability_Dawnbreaker_BreakOfDawn: {
        },
        CDOTA_Ability_AbyssalUnderlord_Cancel_DarkRift: {
        },
        C_DOTA_Ability_Brewmaster_Primal_Split_Fire_Phase: {
        },
        C_DOTA_Ability_DragonKnight_WyrmsWrath: {
        },
        CDOTA_Ability_Morphling_EbbAndFlow: {
        },
        C_DOTA_Ability_Nevermore_Shadowraze1: {
        },
        CDOTA_Ability_FelBeast_Haunt: {
        },
        CDOTA_Modifier_AghsFort_Arcanist_Potion: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Luna_8: {
        },
        CDOTA_Modifier_Special_Bonus_Reincarnation: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Dazzle_5: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_60: {
        },
        C_DOTA_Ability_Special_Bonus_Gold_lvl20_l: {
        },
        CDOTA_Modifier_Hurricane_Pike_Range: {
        },
        CDOTA_Modifier_Item_Lotus_Orb_Active: {
        },
        CDOTA_Modifier_Item_EagleHorn: {
        },
        CDOTA_Modifier_Item_Gauntlets: {
        },
        CDOTA_Modifier_Magnus_Strength_Of_Joelrak: {
        },
        CDOTA_Modifier_Wisp_Equilibrium: {
        },
        CDOTA_Modifier_Lycan_Innate_Spirit_Wolves_Self: {
        },
        CDOTA_Modifier_Spectre_Arcana: {
        },
        CDOTA_Modifier_Omniknight_Marty: {
        },
        CDOTA_Modifier_Sniper_Headshot: {
        },
        CDOTA_Modifier_Tiny_Tree_Channel_Bonus: {
        },
        CDOTA_Modifier_Razor_EyeOfTheStorm: {
        },
        CDOTA_Modifier_Nevermore_Necromastery_Fear: {
        },
        CDOTA_Modifier_Bane_Nightmare_Invulnerable: {
        },
        CDOTA_Modifier_AntiMage_Counterspell: {
        },
        C_DOTAGameManager: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Item_Recipe_Mysterious_Hat: {
        },
        C_DOTA_Ability_Phoenix_IcarusDive: {
        },
        CDOTA_Ability_Shadow_Demon_Disruption: {
        },
        C_DOTA_Ability_Huskar_Berserkers_Blood: {
        },
        C_DOTA_Ability_Necrolyte_Death_Seeker: {
        },
        InGamePredictionData_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Modifier_AghsFort_Creature_Phoenix_Sun: {
        },
        CDOTA_Modifier_Seasonal_TI9_Shovel_BabyRoshan: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Meepo_4: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Riki_1: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lion_10: {
        },
        CDOTA_Modifier_Eul_Wind_Waker_Thinker: {
        },
        CDOTA_Modifier_Keen_Optic: {
        },
        CDOTA_Modifier_Grimstroke_InkCreature_Latched: {
        },
        CDOTA_Modifier_TrollWarlord_Whirling_Axes_Melee: {
        },
        CDOTA_Modifier_Treant_NaturesGuise_DamageCooldown: {
        },
        CDOTA_Modifier_Omniknight_Purification_Recast: {
        },
        CDOTA_Modifier_DragonKnight_DragonBlood: {
        },
        CDOTA_Modifier_Morphling_Accumulation: {
        },
        CDOTA_Modifier_Sven_Warcry: {
        },
        CDOTA_Modifier_Pet: {
        },
        CDOTA_BaseNPC_JungleVarmint: {
        },
        C_DOTAPropCustomTexture: {
        },
        C_DOTAWearableItem: {
        },
        C_DOTA_Item_Recipe_DivineRapier: {
        },
        CDOTA_Ability_Magnus_Magnetic_Horn: {
        },
        C_DOTA_Ability_Obsidian_Destroyer_Equilibrium: {
        },
        CDOTA_Item_Furion_Gold_Bag: {
        },
        C_DOTA_Ability_Tidehunter_LeviathansCatch: {
        },
        C_DOTA_Ability_Puck_IllusoryOrb: {
        },
        CDOTA_Modifier_Neutral_Upgrade: {
        },
        C_DOTA_Ability_HarpyScout_TakeOff: {
        },
        CDOTA_Ability_Furbolg_Enrage_Damage: {
        },
        CDOTA_Modifier_Aghsfort_Elemental_Wisp_Tether: {
        },
        CDOTA_Modifier_AghsFort_RestorativeFlower: {
        },
        CDOTA_Modifier_Special_Bonus_Spell_Block: {
        },
        C_DOTA_Ability_Special_Bonus_Respawn_Reduction_25: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Speed_120: {
        },
        CDOTA_Modifier_Item_UnstableWand_Critter: {
        },
        CDOTA_Modifier_PrimalBeast_Onslaught_Movement: {
        },
        CDOTA_Modifier_VoidSpirit_AstralStep_Caster: {
        },
        CDOTA_Modifier_Tusk_IceShard: {
        },
        CDOTA_Modifier_Alchemist_Scepter_Bonus_Damage: {
        },
        CDOTA_Modifier_Alchemist_UnstableConcoction: {
        },
        CDOTA_Modifier_SpiritBreaker_PlanarPocketAura: {
        },
        CDOTA_Modifier_Nian_Tail_Swipe_AirTime: {
        },
        CDOTA_Modifier_Lion_FingerOfDeath: {
        },
        CDOTA_Modifier_Necrolyte_Sadist: {
        },
        CDOTA_Modifier_Lich_FrostArmor: {
        },
        CDOTA_Modifier_Kunkka_Admirals_Rum: {
        },
        CDOTA_Modifier_Filler_Buff_Icon: {
        },
        CPulseCell_WaitForCursorsWithTag: {
        },
        C_DOTA_Unit_Hero_Life_Stealer: {
        },
        C_DOTA_Ability_Pangolier_LuckyShot: {
        },
        C_DOTA_Ability_MonkeyKing_UnTransform: {
        },
        CDOTA_Ability_Winter_Wyvern_EldwurmsEdda: {
        },
        C_DOTA_Ability_Warlock_Shadow_Word: {
        },
        CDOTA_Ability_Roshan_GrabThrow: {
        },
        C_DOTA_Ability_Razor_UnstableCurrent: {
        },
        CDOTA_Ability_Juggernaut_Innate_Mask_Spin_Crit: {
        },
        CDOTA_Modifier_BigThunderLizard_Wardrums: {
        },
        CDOTA_Ability_AghsFort_Ascension_Firefly: {
        },
        CDOTA_Modifier_Zombie_Berserk: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Queen_Of_Pain_6: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Bane_9: {
        },
        C_DOTA_Ability_Special_Bonus_Intelligence_14: {
        },
        CDOTA_Modifier_Mirror_Shield_Delay: {
        },
        CDOTA_Modifier_Item_Essence_Distiller_Heal: {
        },
        CDOTA_Modifier_DarkWillow_ShadowRealm_FadeTime: {
        },
        CDOTA_Modifier_MonkeyKing_FurArmy_SoldierHidden: {
        },
        CDOTA_Modifier_MonkeyKing_Strike_Stun: {
        },
        CDOTA_Modifier_Phoenix_Sun: {
        },
        CDOTA_Modifier_EarthSpirit_RollingBoulder_Caster: {
        },
        CDOTA_Modifier_EmberSpirit_SlightOfFist_ChargeCounter: {
        },
        CDOTA_Modifier_Disruptor_Glimpse_Thinker: {
        },
        CDOTA_Modifier_Spectre_SpectralDaggerInPath: {
        },
        CDOTA_Modifier_Life_Stealer_Ghoul_Frenzy: {
        },
        CDOTA_Modifier_FacelessVoid_Chronosphere_Freeze: {
        },
        CDOTA_Modifier_Earthshaker_Fissure_Shard_Pathing: {
        },
        CDOTA_Modifier_AncientApparition_ColdFeet_Freeze: {
        },
        C_SceneEntity__QueuedEvents_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Unit_Hero_Bane: {
        },
        C_DOTA_NPC_Lantern: {
        },
        C_DOTA_Item_PocketRoshan: {
        },
        C_DOTA_Ability_Rubick_Empty2: {
        },
        C_DOTA_Ability_Invoker_Invoke: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_ArenaOfBloodHPRegen: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Shadow_Demon_9: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Invoker_5: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lion_5: {
        },
        CDOTA_Modifier_Special_Bonus_Attack_Speed: {
        },
        C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_8: {
        },
        CDOTA_Modifier_Partisans_Brand: {
        },
        CDOTA_Modifier_AghanimsShard_WaitForUpgradeSelected: {
        },
        CDOTA_Modifier_Item_AsceticCapBuff: {
        },
        CDOTA_Modifier_Item_Mango_Tree: {
        },
        CDOTA_Modifier_Magnataur_ReversePolarity_Stats: {
        },
        CDOTA_Modifier_Lycan_Fear: {
        },
        CDOTA_Modifier_Beastmaster_Hawk_Perch_Perching: {
        },
        CDOTA_Modifier_Windrunner_ShackleShot_SelfDamageBuff: {
        },
        CDOTA_Modifier_StormSpirit_OverloadSlow: {
        },
        CDOTA_Modifier_Sven_Variant_Shieldbreaker: {
        },
        CDOTA_Modifier_Juggernaut_Vaulted_Strike: {
        },
        C_DOTA_Unit_Hero_Winter_Wyvern: {
        },
        C_DOTA_Unit_Hero_EmberSpirit: {
        },
        C_DOTA_Unit_Hero_TemplarAssassin: {
        },
        C_DOTA_Item_Sorcerers_Staff: {
        },
        C_DOTA_Item_Recipe_Essence_Distiller: {
        },
        C_DOTA_Item_NetherRaiment: {
        },
        CDOTA_Item_Recipe_BootsOfTravel_2: {
        },
        CDOTA_Ability_Centaur_Work_Horse: {
        },
        C_DOTA_Ability_Brewmaster_FireBrewling_FireStance: {
        },
        C_DOTA_Ability_Warlock_Upheaval: {
        },
        C_DOTA_Ability_Frogmen_CongregationOfTheDeep: {
        },
        CDOTA_Modifier_Creep_Piercing: {
        },
        CDOTA_Modifier_ForestTrollHighPriest_ManaAura: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Kunkka_4: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Amplify_8: {
        },
        FowBlocker_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Modifier_Snapfire_Magma_Burn_Slow: {
        },
        CDOTA_ModifierTreant_Innate_Attack_Damage: {
        },
        CDOTA_Modifier_Lycan_Apex_Predator_Aura: {
        },
        CDOTA_Modifier_Furion_Arboreal_Might_Armor: {
        },
        CDOTA_Modifier_Viper_ViperStrike_Debuff: {
        },
        CDOTA_Modifier_Miniboss_Minion_FollowingMovement: {
        },
        CDOTA_Modifier_Ursa_Enrage: {
        },
        CDOTA_Modifier_Tinker_Eureka: {
        },
        CDOTA_Modifier_LegacyChargeCounter: {
        },
        CDOTA_BaseNPC_Seasonal_TI9_Balloon: {
        },
        C_DOTA_Item_Enhancement_Manic: {
        },
        C_DOTA_Item_Enhancement_Keen_Eyed: {
        },
        CDOTA_Ability_EarthSpirit_Petrify: {
        },
        C_DOTA_Ability_EmberSpirit_SleightOfFist: {
        },
        C_DOTA_Ability_Chaos_Knight_Phantasmagoria: {
        },
        CDOTA_Ability_Miniboss_Minion_Unyielding_Shield: {
        },
        CDOTA_Ability_Roshan_SpellBlock: {
        },
        C_DOTA_Ability_Tidehunter_Ravage: {
        },
        C_DOTA_Ability_StormSpirit_ElectricVortex: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Underlord_3: {
        },
        C_DOTA_Ability_Special_Bonus_Evasion_12: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Base_Damage_50: {
        },
        CDOTA_Modifier_Item_EyeOfTheVizier: {
        },
        CDOTA_Modifier_Item_Chipped_Vest: {
        },
        CDOTA_Modifier_Item_Ward_Maker: {
        },
        CDOTA_Modifier_Pangolier_Swashbuckle_Attack: {
        },
        CDOTA_Modifier_MonkeyKing_FurArmy_Soldier: {
        },
        CDOTA_Modifier_Terrorblade_Metamorphosis_Transform_Aura_Applier: {
        },
        CDOTA_Modifier_Bristleback_QuillSprayStack: {
        },
        CDOTA_Modifier_Treant_LivingArmorPermanent: {
        },
        CDOTA_Modifier_Omniknight_GuardianAngel: {
        },
        CDOTA_Modifier_Dark_Seer_Innate_MasterTactician: {
        },
        CDOTA_Modifier_Lion_Impale: {
        },
        CDOTA_Modifier_Lich_Sinister_Gaze: {
        },
        CBaseTriggerAPI: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Item_Recipe_Cloak_Of_Flames: {
        },
        C_DOTA_Item_Chipped_Vest: {
        },
        CDOTA_Item_Pavise: {
        },
        CDOTA_Item_Recipe_Guardian_Greaves: {
        },
        C_DOTA_Item_Recipe_Boots_Of_Bearing: {
        },
        C_DOTA_Item_Shivas_Guard: {
        },
        C_DOTA_Item_RobeOfMagi: {
        },
        C_DOTA_Item_Recipe_BootsOfTravel: {
        },
        C_DOTA_Ability_Visage_GravekeepersCloak: {
        },
        C_DOTA_Ability_LoneDruid_SpiritBear: {
        },
        C_DOTA_Ability_Brewmaster_Pulverize: {
        },
        C_DOTA_Ability_Kunkka_Tidal_Wave: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_20: {
        },
        C_DOTABotChallengeGameMode: {
        },
        CDOTA_Modifier_Item_Urn_Heal: {
        },
        CDOTA_Modifier_Primalbeast_Trample: {
        },
        CDOTA_Modifier_VoidSpirit_AstralStep_ChargeCounter: {
        },
        CDOTA_Modifier_KeeperOfTheLight_Will_O_Wisp: {
        },
        CDOTA_Modifier_Brewmaster_Void_Brawler_Slow: {
        },
        CDOTA_Modifier_DoomBringer_Doom_Aura_Enemy: {
        },
        CDOTA_Modifier_Rattletrap_Overheated: {
        },
        CDOTA_Modifier_FacelessVoid_Chronosphere_SelfBuff: {
        },
        CDOTA_Modifier_FacelessVoid_TimeDilation_Distortion_Aura: {
        },
        CDOTA_Modifier_FacelessVoid_Backtrack: {
        },
        CDOTA_Modifier_DrowRanger_ArcanaKill_Delay: {
        },
        CDOTA_Modifier_Sand_King_Shard: {
        },
        CDOTA_Modifier_BattleCupEffigy: {
        },
        C_DOTA_BaseNPC_SDKTower: {
        },
        CDOTA_Item_Stonefeather_Satchel: {
        },
        C_DOTA_Item_Doubloon: {
        },
        C_DOTA_Item_Recipe_Mage_Slayer: {
        },
        C_DOTA_Ability_Ringmaster_EmptySouvenir: {
        },
        C_DOTA_Ability_Invoker_Quas: {
        },
        CDOTA_Ability_Life_Stealer_Control: {
        },
        C_DOTA_Ability_Nevermore_Presence: {
        },
        CDOTA_Modifier_Greevil_Miniboss_Green_Overgrowth: {
        },
        CDOTA_Modifier_AghsFort_Firefly_Burn: {
        },
        C_DOTA_Ability_Special_Bonus_Armor_3: {
        },
        CDOTA_Modifier_Item_Vampire_Fangs: {
        },
        CDOTA_Modifier_Item_Lotus_Orb_ChannelCheck: {
        },
        CDOTA_Modifier_Item_Pipe_Aura: {
        },
        CDOTA_Modifier_item_Splintmail: {
        },
        CDOTA_Modifier_Snapfire_Scatterblast_Slow: {
        },
        CDOTA_Modifier_Pangolier_ShieldCrash_Jump: {
        },
        CDOTA_Modifier_Banana: {
        },
        CDOTA_Modifier_Skywrath_Mage_Staff_Of_The_Scion: {
        },
        CDOTA_Modifier_Jakiro_LiquidFire_Burn: {
        },
        CDOTA_Modifier_Jakiro_DualBreath_Burn: {
        },
        CDOTA_Modifier_Leshrac_Decrepify_Aura: {
        },
        CDOTA_Modifier_Ursa_Overpower: {
        },
        CDOTA_Modifier_BookOfAgility: {
        },
        CDOTA_Modifier_Buyback_Gold_Penalty: {
        },
        CDOTA_Unit_Hero_Centaur: {
        },
        C_DOTA_Unit_Hero_Pudge: {
        },
        C_DOTA_BaseNPC_HoldoutTower_HeavySlow: {
        },
        C_DOTA_BaseNPC_Healer: {
        },
        CDOTA_Item_Harmonizer: {
        },
        CDOTA_Item_Guardian_Greaves: {
        },
        CDOTA_Ability_Techies_StickyBomb: {
        },
        C_DOTA_Ability_Rubick_Might_And_Magus: {
        },
        C_DOTA_Ability_Spectre_Spectral: {
        },
        C_DOTA_Ability_Rattletrap_Innate_Cogs_Small_Spread: {
        },
        C_DOTA_Ability_SandKing_BurrowStrike: {
        },
        CDOTA_Modifier_Mutation_StationaryDamageReduction_Aura: {
        },
        CDOTA_Modifier_AghsFort_Ascension_Silence: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Grimstroke_7: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Undying: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_13: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Speed_140: {
        },
        C_DOTA_DataDire: {
        },
        CDOTA_Modifier_Necronomicon_Warrior_ManaBurn: {
        },
        CDOTA_Modifier_Item_Crimson_Guard: {
        },
        CDOTA_Modifier_Manta: {
        },
        CDOTA_Modifier_Slark_Depth_Shroud: {
        },
        CDOTA_Modifier_Viper_CorrosiveSkin: {
        },
        CDOTA_Modifier_Pudge_Dismember: {
        },
        C_SpeechBubbleInfo: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_EnvWindShared: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_SkyCamera: {
        },
        CPulseCell_Base: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        C_DOTA_Item_Timeless_Relic: {
        },
        CDOTA_Ability_Magnataur_Skewer: {
        },
        C_DOTA_Ability_NagaSiren_SongOfTheSiren: {
        },
        C_DOTA_Ability_Undying_Tombstone_Zombie_DeathStrike: {
        },
        C_DOTA_Ability_LoneDruid_TrueForm_BattleCry: {
        },
        C_DOTA_Ability_LoneDruid_SpiritBear_Defender: {
        },
        CDOTA_Ability_DarkSeer_Surge: {
        },
        C_DOTA_Ability_QueenOfPain_SonicWave: {
        },
        C_DOTA_Ability_Tinker_Keen_Teleport: {
        },
        C_DOTA_Ability_Zuus_ArcLightning: {
        },
        C_FuncRotating: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Spirit_Breaker_4: {
        },
        CDOTA_Modifier_Special_Bonus_HP: {
        },
        C_DOTA_Ability_Special_Bonus_Night_Vision_600: {
        },
        C_DOTA_Ability_Special_Bonus_HP_450: {
        },
        CDOTA_Modifier_Roshans_Banner_HP: {
        },
        CDOTA_Modifier_Item_Veil_Of_Discord: {
        },
        CDOTA_Modifier_Item_BlinkDagger: {
        },
        CDOTA_Modifier_Invoker_Quas_Intrinsic: {
        },
        CDOTA_Modifier_Life_Stealer_Infest: {
        },
        CDOTA_Modifier_Viper_ViperStrike_Slow: {
        },
        CDOTA_Modifier_Attached_Unit: {
        },
        CDOTA_Modifier_AncientApparition_IceBlast: {
        },
        C_SoundOpvarSetPointBase: {
        },
        CDOTA_Unit_Hero_Marci: {
        },
        CDOTA_Item_Nether_Shawl: {
        },
        C_DOTA_Item_Recipe_Veil_Of_Discord: {
        },
        CDOTA_Ability_Magnataur_Empower: {
        },
        C_DOTA_Ability_Nyx_Assassin_SpikedCarapace: {
        },
        C_DOTA_Ability_Jakiro_IcePath: {
        },
        C_DOTA_Ability_Life_Stealer_Consume: {
        },
        CDOTA_Modifier_Greevil_Miniboss_Blue_IceVortex: {
        },
        C_EnvCubemapFog: {
        },
        C_DOTA_Ability_Special_Bonus_Lifesteal_15: {
        },
        CDOTA_Modifier_Item_Foragers_Stats: {
        },
        CDOTA_Modifier_Item_SuperArcane_Blink: {
        },
        CDOTA_Modifier_Item_Apex: {
        },
        CDOTA_Modifier_Item_RingOfAquila_Aura: {
        },
        CDOTA_Modifier_Item_DustofAppearance: {
        },
        CDOTA_Modifier_Phoenix_SunRay: {
        },
        CDOTA_Modifier_EarthSpirit_Geogmagnetic_Grip_Debuff: {
        },
        CDOTA_Modifier_Centaur_Mount_Toss: {
        },
        CDOTA_Modifier_Medusa_StoneGaze_Facing: {
        },
        CDOTA_Modifier_NagaSiren_RipTide_Passive: {
        },
        CDOTA_Modifier_Disruptor_ElectromagneticRepulsion_Passive: {
        },
        C_EnvParticleGlow: {
        },
        CDOTA_Item_Recipe_Swift_Blink: {
        },
        C_DOTA_Item_Fluffy_Hat: {
        },
        C_DOTA_Item_VoidwalkerScythe: {
        },
        C_DOTA_Ability_Kez_Shodo_Sai_Parry_Cancel: {
        },
        C_DOTA_Ability_Phoenix_IcarusDiveStop: {
        },
        CDOTA_Ability_Disruptor_ElectromagneticRepulsion: {
        },
        C_DOTA_Ability_Invoker_ForgeSpirit: {
        },
        C_DOTA_Jungle_Varmint_Dive: {
        },
        CDOTA_Ability_Creature_Flamestrike: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lion_11: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Drow_Ranger_3: {
        },
        C_DOTA_Ability_Special_Bonus_Exp_Boost_15: {
        },
        C_FoWRevealerEntity: {
        },
        CDOTA_Modifier_Item_Nemesis_Curse_Debuff: {
        },
        CDOTA_Modifier_Item_MaskOfDispair: {
        },
        CDOTA_Modifier_Item_LesserCritical: {
        },
        CDOTA_Modifier_Arc_Warden_Magnetic_Field_Thinker_Rune_Magnet: {
        },
        CDOTA_Modifier_Elder_Titan_EarthSplitter: {
        },
        CDOTA_Modifier_Abaddon_Frostmourne: {
        },
        CDOTA_Modifier_Wisp_Overcharge: {
        },
        CDOTA_Modifier_Nyx_Assassin_Vendetta: {
        },
        CDOTA_Modifier_Brewmaster_SpellImmunity: {
        },
        CDOTA_Modifier_Brewmaster_CinderBrew_Animation: {
        },
        CDOTA_Modifier_Broodmother_Tough: {
        },
        CDOTA_Modifier_PhantomAssassin_BlurActive: {
        },
        CDOTA_Modifier_Beastmaster_DrumsOfSlom: {
        },
        CDOTA_Modifier_Roshan_Grab_Thrown: {
        },
        CDOTA_Modifier_Riki_SmokeScreenThinker: {
        },
        CDOTA_Modifier_VengefulSpirit_WaveOfTerror: {
        },
        CDOTA_Modifier_FillerThinker: {
        },
        CDOTA_Modifier_Stacking_Multiple_Buff_Base: {
        },
        C_EconEntity__AttachedModelData_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CPulse_ResumePoint: {
        },
        C_NextBotCombatCharacter: {
        },
        CTriggerFan: {
        },
        CDOTA_Item_Apex: {
        },
        C_DOTA_Ability_Largo_Frogstomp: {
        },
        C_DOTA_Ability_Ringmaster_DarkCarnivalSouvenirs: {
        },
        CDOTA_Ability_Snapfire_Boomstick: {
        },
        CDOTA_Ability_Mars_Dauntless: {
        },
        C_DOTA_Ability_Winter_Wyvern_Essence_Of_The_Blueheart: {
        },
        CDOTA_Ability_Elder_Titan_EchoStomp_Spirit: {
        },
        C_DOTA_Ability_KeeperOfTheLight_Recall: {
        },
        CDOTA_Modifier_Seasonal_TI11_CongaLineSlow: {
        },
        CDOTA_AghsFort_BossPreview: {
        },
        C_DOTA_Aghsfort_Ability_CrystalMaiden_CrystalNova: {
        },
        CDOTA_Modifier_Seasonal_PartyHat: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Base_Damage_35: {
        },
        CDOTA_Modifier_Item_Enhancement_Feverish: {
        },
        CDOTA_Modifier_Item_Harpoon_EchoSabre_Component: {
        },
        CDOTA_Modifier_Item_Mind_Breaker: {
        },
        CDOTA_Modifier_Item_Faerie_Fire: {
        },
        CDOTA_Modifier_MjollnirChain: {
        },
        CDOTA_Modifier_InvisibilityEdge_WindWalk: {
        },
        CDOTA_Modifier_Item_Perseverance: {
        },
        CDOTA_Modifier_Kez_SwitchWeapon_Aghs: {
        },
        CDOTA_Modifier_Shredder_Flamethrower: {
        },
        CDOTA_Modifier_Rubick_Curiosity: {
        },
        CDOTA_Modifier_NetherStrike_GreaterBash_KnockbackAmp: {
        },
        CDOTA_Modifier_Ursa_Damage_Increase: {
        },
        CDOTA_Modifier_CrystalMaiden_FreezingField: {
        },
        C_DOTA_Unit_Hero_Lich: {
        },
        C_DOTASceneEntity: {
        },
        C_DOTA_Item_TiaraOfSelemene: {
        },
        CDOTA_Item_Recipe_HydrasBreath: {
        },
        C_DOTA_Item_VoidStone: {
        },
        C_DOTA_Ability_Medusa_Cold_Blooded: {
        },
        C_DOTA_Ability_Treant_LivingArmor: {
        },
        C_DOTA_Item_Vermillion_Robe: {
        },
        CDOTA_Item_Mask_Spin_Crit: {
        },
        C_DOTA_Ability_Greevil_Miniboss_Sight: {
        },
        C_FuncElectrifiedVolume: {
        },
        C_DOTASpectatorGraphManager: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lion_2: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lich_5: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Tiny_5: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Pangolier_3: {
        },
        C_DOTA_Ability_Special_Bonus_Exp_Boost_25: {
        },
        C_DOTA_Ability_Special_Bonus_Agility_81: {
        },
        C_DOTA_Ability_Special_Bonus_MP_Regen_2: {
        },
        CDOTA_Modifier_Item_RefresherShard: {
        },
        CDOTA_Modifier_Kez_Katana_Shard_Debuff: {
        },
        CDOTA_Modifier_AbyssalUnderlord_AtrophyAura_Permanent_HeroDmgBuff: {
        },
        CDOTA_Modifier_Slark_EssenceShift_Permanent_Buff: {
        },
        CDOTA_Modifier_Nyx_Assassin_Vendetta_Break: {
        },
        CDOTA_Modifier_Undying_FleshGolem_Slow: {
        },
        CDOTA_Modifier_Invoker_EMP: {
        },
        CDOTA_Modifier_Jakiro_Macropyre_Burn: {
        },
        CDOTA_Modifier_Enchantress_Enchant_Intrinsic_Aura: {
        },
        CDOTA_Modifier_PhantomAssassin_StiflingDagger_Recast: {
        },
        CDOTA_Modifier_Item_GrisGris_GoldCounter: {
        },
        CDOTA_Modifier_Tidehunter_AnchorSmash: {
        },
        CDOTA_Modifier_Sven_Stormbolt_Hide: {
        },
        CDOTA_Modifier_Lua_Horizontal_Motion: {
        },
        C_DOTA_NPC_BroodmotherStickyWeb: {
        },
        C_DOTA_Item_Arcane_Blink: {
        },
        CDOTA_Item_RefresherOrb_Shard: {
        },
        C_DOTA_Item_Recipe_GreaterCritical: {
        },
        C_DOTA_Ability_Largo_Encore: {
        },
        C_DOTA_Ability_Bristleback_Hairball: {
        },
        C_DOTA_Ability_Enchantress_Little_Friends: {
        },
        C_DOTA_Ability_Lich_ChainFrost: {
        },
        CDOTA_Ability_Kunkka_Admirals_Rum: {
        },
        CDOTA_Modifier_SatyrTrickster_Purge: {
        },
        C_EnvVolumetricFogVolume: {
        },
        CDOTA_Modifier_Special_Bonus_Status_Resistance: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_50: {
        },
        CDOTA_Modifier_Item_Harpoon: {
        },
        CDOTA_Modifier_Yasha_And_Kaya_Maim: {
        },
        CDOTA_Modifier_Kez_RaptorDance_Immune: {
        },
        CDOTA_Modifier_Kez_Echo_Slash_Echo_Damage: {
        },
        CDOTA_Modifier_Phoenix_IcarusDive: {
        },
        CDOTA_Modifier_Legion_Commander_OverwhelmingOdds: {
        },
        CDOTA_Modifier_Mangataur_Shockwave_Intrinsic: {
        },
        CDOTA_Modifier_Brewmaster_DrunkenHaze: {
        },
        CDOTA_Modifier_Batrider_FlamingLasso: {
        },
        CDOTA_Modifier_Night_Stalker_Heart_Of_Darkness: {
        },
        CDOTA_Modifier_Warlock_FatalBonds: {
        },
        CDOTA_Modifier_Roshan_Grab_Self: {
        },
        CDOTA_Modifier_Axe_Culling_Blade_Charge: {
        },
        CDOTA_Modifier_Axe_BattleHunger: {
        },
        CDOTA_Modifier_Juggernaut_Bladeform_Linger: {
        },
        CDOTA_Modifier_Rune_SuperArcane: {
        },
        CIngameEvent_Diretide2020: {
        },
        CPulseCell_PlaySequence: {
        },
        C_DOTA_Item_Keen_Optic: {
        },
        C_DOTA_Item_BladeOfAlacrity: {
        },
        C_DOTA_Ability_Treant_Innate_Attack_Damage: {
        },
        CDOTA_Ability_Shadow_Demon_Shadow_Poison_Release: {
        },
        C_DOTA_Ability_Invoker_ColdSnap: {
        },
        CDOTA_Ability_Broodmother_StickySnare: {
        },
        C_DOTA_Ability_Lifestealer_CorpseEater: {
        },
        C_DOTA_Ability_Necrolyte_Death_Pulse: {
        },
        CDOTA_Modifier_PolarFurbolgUrsaWarrior_ThunderClap: {
        },
        CDOTA_Modifier_Boss_DarkWillow_Bedlam: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Magnus: {
        },
        CDOTA_Modifier_Special_Bonus_Unique_Treant_3: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Juggernaut_2: {
        },
        C_DOTA_Ability_Special_Bonus_HP_Regen_8: {
        },
        C_DOTA_Ability_Special_Bonus_MP_700: {
        },
        CDOTA_Modifier_Item_Bottomless_Chalice: {
        },
        CDOTA_Modifier_Item_Blood_Grenade_Debuff: {
        },
        CDOTA_Modifier_Item_Blood_Grenade: {
        },
        CDOTA_Modifier_MonkeyKing_CloudRunStart: {
        },
        CDOTA_Modifier_Banana_Knockback: {
        },
        CDOTA_Modifier_EmberSpirit_FlameGuard: {
        },
        CDOTA_Modifier_Techies_SnareTrap: {
        },
        CDOTA_Modifier_Tidehunter_SmashAttack: {
        },
        CDOTA_Modifier_Morphling_Replicate_MorphedIllusionsEffect: {
        },
        CDOTA_Modifier_Fountain_Truesight_Aura: {
        },
        C_BaseEntityAPI: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_PortraitHero: {
        },
        C_DOTA_Item_Bloodthorn: {
        },
        CDOTA_Item_Recipe_GlimmerCape: {
        },
        C_DOTA_Ability_PrimalBeast_Uproar: {
        },
        C_DOTA_Ability_Rubick_Telekinesis: {
        },
        C_DOTA_Ability_Dazzle_Weave: {
        },
        C_DOTA_Ability_Greevil_Miniboss_Orange_DragonSlave: {
        },
        C_DOTA_Ability_KoboldTaskmaster_SpeedAura: {
        },
        CDOTA_Ability_Seasonal_TI11_CongaLine: {
        },
        CDOTA_Modifier_Wave_Blast_Knockback: {
        },
        CDOTA_Modifier_Ascension_Meteoric_Land: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Death_Prophet_5: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Tinker_7: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Tiny_3: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Sand_King_4: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Ancient_Apparition_6: {
        },
        C_DOTA_Ability_Special_Bonus_Agility_12: {
        },
        CDOTA_Modifier_ManaDraught_Regeneration: {
        },
        CDOTA_Modifier_Gungnir_Debuff: {
        },
        CDOTA_Modifier_Item_SentryWard: {
        },
        CDOTA_Modifier_AbyssalUnderlord_Underling_Spawn_Thinker: {
        },
        CDOTA_Modifier_Magnataur_Shockwave_Pull: {
        },
        CDOTA_Modifier_Slark_SaltwaterShiv: {
        },
        CDOTA_Modifier_BountyHunter_WindWalk: {
        },
        CDOTA_Modifier_Broodmother_SpawnSpiderite: {
        },
        CDOTA_Modifier_PhantomLancer_Juxtapose_Cache: {
        },
        CDOTA_Unit_Side_Gunner: {
        },
        C_DOTA_PortraitBuilding: {
        },
        C_DOTA_Item_RuneSpawner_XP: {
        },
        C_DOTA_Item_Recipe_Falcon_Blade: {
        },
        CDOTA_Item_Recipe_Nether_Shawl: {
        },
        C_DOTA_Item_Spell_Prism: {
        },
        C_DOTA_Item_Quarterstaff: {
        },
        C_DOTA_Ability_Rattletrap_PowerCogs: {
        },
        CDOTA_Modifier_Greevil_Miniboss_Blue_ColdFeet: {
        },
        CDOTA_Modifier_Mutation_Spellcast: {
        },
        C_DOTASpectatorGraphManagerProxy: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Bane_3: {
        },
        CDOTA_Modifier_Special_Bonus_Attributes: {
        },
        CDOTA_Modifier_Item_Faded_Broach: {
        },
        CDOTA_Modifier_Item_Cornucopia: {
        },
        CDOTA_Modifier_Item_Bloodstone_Active: {
        },
        CDOTA_Modifier_Special_Mars_Spear_Burning_Trail_Thinker: {
        },
        CDOTA_Modifier_EmberSpirit_Immolation_Debuff: {
        },
        CDOTA_Modifier_NagaSiren_SongOfTheSiren_Aura: {
        },
        CDOTA_Modifier_Treant_NaturesGuise_NearTreeDisplay: {
        },
        CDOTA_Modifier_Chaos_Knight_Phantasmagoria: {
        },
        CDOTA_Modifier_PhantomLancer_JuxtaposeIllusion: {
        },
        CDOTA_Modifier_Bloodseeker_Thirst: {
        },
        CDOTA_Modifier_Shadowraze_Counter: {
        },
        CDOTA_Modifier_Filler_LastDitch: {
        },
        CPulseCell_LerpCameraSettings: {
        },
        C_DOTA_Unit_Hero_MonkeyKing: {
        },
        C_DOTA_BaseNPC_ShadowShaman_SerpentWard: {
        },
        C_DOTA_Item_Recipe_Butterfly: {
        },
        CDOTA_Ability_Techies_RemoteMines_SelfDetonate: {
        },
        C_DOTA_Ability_Bristleback_QuillSpray: {
        },
        C_DOTA_Ability_TrollWarlord_Innate_Masterful: {
        },
        C_DOTA_Ability_LoneDruid_SpiritBear_Fetch: {
        },
        C_DOTA_Ability_LoneDruid_SpiritLink: {
        },
        C_DOTA_Ability_Luna_MoonGlaive: {
        },
        C_DOTA_Ability_Beastmaster_Mark_Of_The_Beast: {
        },
        CDOTA_Ability_AncientApparition_IceAge: {
        },
        C_DOTA_Ability_BigThunderLizard_Slam: {
        },
        CPointOffScreenIndicatorUi: {
        },
        CDOTA_Modifier_Creature_Flamestrike_Ground: {
        },
        CDOTA_Modifier_LootDrop_Thinker: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Hoodwink_AcornShotDamage: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Riki_6: {
        },
        CDOTA_Modifier_MetamorphicMandible_Active: {
        },
        CDOTA_Modifier_Aghanims_Shard: {
        },
        CDOTA_Modifier_Necronomicon_Archer_Purge: {
        },
        CDOTA_Modifier_Largo_Song_Speed_Burst_SlowRes: {
        },
        CDOTA_Modifier_Dawnbreaker_Converge: {
        },
        CDOTA_Modifier_Mars_Immovable: {
        },
        CDOTA_Modifier_Medusa_VenomedVolley: {
        },
        CDOTA_Modifier_Slark_ShadowDance_Visual: {
        },
        CDOTA_Modifier_Wisp_Tether_Stun_Tracker: {
        },
        CDOTA_Modifier_Shadow_Demon_Disseminate_HPLoss: {
        },
        CDOTA_Modifier_Jakiro_DualBreath_Thinker: {
        },
        CDOTA_Modifier_Rattletrap_Hookshot: {
        },
        CDOTA_Modifier_Miniboss_Minion_Deflecting_Shield_Buff: {
        },
        CDOTA_Modifier_Illusion: {
        },
        CDOTA_Unit_Falconers_Glove_Hawk: {
        },
        CDOTA_BaseNPC_Effigy_AghsFort: {
        },
        C_DOTA_Item_Dynamite_Jacket: {
        },
        CDOTA_Item_Paladin_Sword: {
        },
        C_DOTA_Ability_Dawnbreaker_Fire_Wreath: {
        },
        C_DOTA_Ability_Troll_Warlord_Rampage: {
        },
        C_DOTA_Ability_Meepo_DividedWeStand: {
        },
        C_DOTA_Ability_Brewmaster_DrunkenBrawler: {
        },
        C_DOTA_Ability_Leshrac_Split_Earth: {
        },
        C_DOTA_Ability_DeathProphet_CarrionSwarm: {
        },
        C_DOTA_Ability_StormSpirit_BallLightning: {
        },
        CDOTA_Modifier_BlueDragonspawnOverseer_DevotionAura: {
        },
        CDOTA_Modifier_SkeletonKing_Reincarnation_Scepter_Active: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lich_4: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Amplify_10: {
        },
        C_DOTA_Ability_Special_Bonus_Magic_Resistance_40: {
        },
        C_DOTA_Ability_Special_Bonus_HP_150: {
        },
        CDOTA_Modifier_Item_ForagersHealth: {
        },
        CDOTA_Modifier_Dawnbreaker_Solar_Guardian_Evasion: {
        },
        CDOTA_Modifier_EarthSpirit_Polarization_Damage_Timer: {
        },
        CDOTA_Modifier_Ogre_Magi_Bloodlust: {
        },
        CDOTA_Modifier_ChaosKnight_Chaos_Strike_Debuff: {
        },
        CDOTA_Modifier_BountyHunter_Big_Game_Hunter_Tracks: {
        },
        CDOTA_Modifier_Phantom_Assassin_Blur_Visual: {
        },
        CDOTA_Modifier_Nian_Tail_Swipe_Wall: {
        },
        CDOTA_Modifier_Razor_StaticLink_Buff: {
        },
        C_PostProcessingVolume: {
        },
        C_DOTA_Item_Enhancement_Crude: {
        },
        C_DOTA_Item_Orb_Of_Frost: {
        },
        C_DOTA_Item_Maelstrom: {
        },
        C_DOTA_Item_Dagon_Upgraded5: {
        },
        C_DOTA_Ability_Visage_SummonFamiliars: {
        },
        CDOTA_Ability_NagaSiren_SlithereenCutlass: {
        },
        CDOTA_Ability_Life_Stealer_Empty2: {
        },
        C_DOTA_Ability_Dazzle_Good_Juju: {
        },
        CDOTA_Ability_Miniboss_Unyielding_Shield: {
        },
        CDOTA_Ability_Roshan_Slam: {
        },
        C_DOTA_Ability_Courier_ReturnToBase: {
        },
        C_DOTA_Ability_Greevil_Miniboss_White_Degen_Aura: {
        },
        C_DOTA_Ability_Greevil_Miniboss_White_Purification: {
        },
        CDOTA_Ability_PineCone_ShieldBash: {
        },
        CDOTA_Modifier_UpgradedMortar: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lion_4: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Enchantress_4: {
        },
        CDOTA_Modifier_Special_Bonus_Spell_Amplify: {
        },
        C_DOTA_Ability_Special_Bonus_Exp_Boost_35: {
        },
        C_DOTA_Ability_Special_Bonus_Corruption_3: {
        },
        CDOTA_Modifier_Item_Ex_Machina: {
        },
        CDOTA_Modifier_Item_Ballista: {
        },
        CDOTA_Modifier_VoidSpirit_AetherRemnant_WatchThinker: {
        },
        CDOTA_Modifier_Skywrath_Mage_Shard: {
        },
        CDOTA_Modifier_Invoker_ExortInstance: {
        },
        CDOTA_Modifier_Dazzle_NothlProjection_SoulClone: {
        },
        CDOTA_Modifier_Tiny_Toss_ChargeCounter: {
        },
        CDOTA_Modifier_VengefulSpirit_Restitution_Revival: {
        },
        CDOTA_Modifier_StormSpirit_ElectricVortex_NoStack: {
        },
        C_BaseModelEntity__Emphasized_Phoneme: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Unit_Hero_Axe: {
        },
        C_DOTA_Ability_Kez_TalonToss: {
        },
        C_DOTA_Ability_Phoenix_SunRayToggleMove: {
        },
        C_DOTA_Ability_Tusk_FrozenSigil: {
        },
        CDOTA_Modifier_BerserkerTroll_Break: {
        },
        CDOTA_Modifier_AghsFort_DragonPotion: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lich_6: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_10: {
        },
        CDOTA_Modifier_Item_Titan_Sliver: {
        },
        CDOTA_Modifier_Item_MeteorHammer_Burn: {
        },
        CDOTA_Modifier_Ringmaster_Wheel_Mesmerize_Facing: {
        },
        CDOTA_Modifier_Grimstroke_Ink_Trail: {
        },
        CDOTA_Modifier_Oracle_DivinersDeck_TheWorld: {
        },
        CDOTA_Modifier_Medusa_StoneGaze_Tracker: {
        },
        CDOTA_Modifier_Brewmaster_Cyclone: {
        },
        CDOTA_Modifier_Earthshaker_Fissure_Shard: {
        },
        CDOTA_Modifier_SandKing_SandStorm_Slow: {
        },
        CPulseCell_PickBestOutflowSelector: {
        },
        C_DOTA_Unit_Hero_Windrunner: {
        },
        CIngameEvent_MonsterHunter_DummyModifierHolder: {
        },
        C_DOTA_Item_Recipe_Nether_Raiment: {
        },
        C_DOTA_Ability_Pangolier_GyroshellStop: {
        },
        C_DOTA_Ability_Shredder_WhirlingDeath: {
        },
        C_DOTA_Ability_Invoker_InvokableElement: {
        },
        C_DOTA_Ability_Enchantress_Impetus: {
        },
        C_DOTA_Ability_Rattletrap_Hookshot: {
        },
        C_DOTA_Ability_QueenOfPain_ScreamOfPain: {
        },
        C_DOTA_Ability_Warlock_Golem_Permanent_Immolation: {
        },
        C_DOTA_Ability_Courier_ReturnStashItems: {
        },
        C_DOTA_Ability_Kunkka_GhostShip: {
        },
        CInfoFan: {
        },
        CDOTA_Modifier_UrnUpheaval: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Meepo_3: {
        },
        C_DOTA_Ability_Special_Bonus_All_Stats_8: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Block_18: {
        },
        C_DOTA_Ability_Special_Bonus_Undefined: {
        },
        CPulseCell_ShmupWaitForDuration: {
        },
        CDOTA_Modifier_Dawnbreaker_Unbreakable: {
        },
        CDOTA_Modifier_Undying_Decay_Buff: {
        },
        CDOTA_Modifier_Invoker_GhostWalk_Enemy: {
        },
        CDOTA_Modifier_Broodmother_Spiders_Milk_Healing: {
        },
        CDOTA_Modifier_DragonKnight_DragonForm: {
        },
        CDOTA_Modifier_Lich_Chain_Frost_Frostbound: {
        },
        CDOTA_Modifier_Bane_Ichor_Of_Nyctasha_Debuff: {
        },
        CDOTA_Modifier_Bane_FiendsGrip_Cast_Illusion: {
        },
        C_DOTA_Unit_Hero_Zuus: {
        },
        CDOTA_Item_SuperSwift_Blink: {
        },
        C_DOTA_Item_Vampire_Fangs: {
        },
        CDOTA_Item_GreatFamango: {
        },
        C_DOTA_Ability_TrollWarlord_Innate_Unflinching: {
        },
        C_DOTA_Ability_Animation_LeftClawSwipe: {
        },
        C_DOTA_Ability_Kunkka_Torrent: {
        },
        C_DOTA_Ability_Greevil_Miniboss_Green_LivingArmor: {
        },
        CDOTA_Modifier_AghsFort_Creature_Venomancer_PoisonNova: {
        },
        CDOTA_Ability_AghsFort_ExplosiveBarrel: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Ember_Spirit_1: {
        },
        C_DOTA_Ability_Special_Bonus_Evasion_40: {
        },
        C_DOTA_Ability_Special_Bonus_HP_Regen_16: {
        },
        CDOTA_Modifier_Item_Enhancement_Keen_Eyed: {
        },
        CDOTA_Modifier_Orb_Of_Frost_Debuff: {
        },
        CDOTA_Modifier_Soul_Ring_Buff: {
        },
        CDOTA_Modifier_Item_PlateMail: {
        },
        CDOTA_Modifier_Skywrath_Mage_Shield_Barrier: {
        },
        CDOTA_Modifier_Lycan_Shapeshift_Thinker: {
        },
        CDOTA_Modifier_MeltingStrike_Debuff: {
        },
        CDOTA_Modifier_Dazzle_NothlBoon_Shield: {
        },
        CDOTA_Modifier_Nian_GreaterBash_Speed: {
        },
        CDOTA_Modifier_Slardar_Puddle_Thinker: {
        },
        CDOTA_Modifier_Building_DispelsSmoke: {
        },
        C_DOTA_Item_Vanguard: {
        },
        C_DOTA_Item_SobiMask: {
        },
        C_DOTA_Item_BladesOfAttack: {
        },
        C_DOTA_Ability_Silencer_LastWord: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Drow_Ranger_6: {
        },
        CDOTA_Modifier_Item_Pavise_Shield: {
        },
        CDOTA_Modifier_Hurricane_Pike_Active: {
        },
        CDOTA_Modifier_AbyssalUnderlord_AtrophyAura: {
        },
        CDOTA_Modifier_Tusk_WalrusPunch: {
        },
        CDOTA_Modifier_Obsidian_Destroyer_Sanity_Eclipse_Thinker: {
        },
        CDOTA_Modifier_Warlock_Shadow_Word: {
        },
        CDOTA_Modifier_Razor_EyeOfTheStorm_Armor: {
        },
        CDOTA_Modifier_Axe_CullingBlade_Boost: {
        },
        CDOTA_Modifier_Hexed: {
        },
        C_DOTA_Unit_Hero_Medusa: {
        },
        C_DOTA_NPC_DataDriven: {
        },
        C_DOTA_Ability_Grimstroke_Ink_Trail: {
        },
        C_DOTA_Ability_Techies_ReactiveTazer_Stop: {
        },
        C_DOTA_Beastmaster_Axe: {
        },
        C_DOTA_Ability_Riki_TricksOfTheTrade: {
        },
        CDOTA_Modifier_DoNotCastRock: {
        },
        CDOTA_Modifier_Spawnlord_Master_Freeze_Root: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Medusa_2: {
        },
        C_DOTA_Ability_Special_Bonus_24_Crit_2: {
        },
        CDOTA_Modifier_Item_Broom_Handle: {
        },
        CDOTA_Modifier_Item_Ocean_Heart: {
        },
        CDOTA_Modifier_Shredder_WhirlingDeath_Debuff: {
        },
        CDOTA_Modifier_NightStalker_Void_ZoneThinker: {
        },
        CDOTA_Modifier_FacelessVoid_TimeDilation_SelfBuff: {
        },
        CDOTA_Modifier_Roshan_Knockback_Timeout: {
        },
        CDOTA_Modifier_ShadowShaman_SerpentWard_Intrinsic: {
        },
        CDOTA_Modifier_Lina_LagunaBlade: {
        },
        CDOTA_Modifier_SkeletonKing_Reincarnation_Scepter: {
        },
        CDOTA_Modifier_Rune_Super_Invisibility: {
        },
        C_DOTA_Unit_TemplarAssassin_PsionicTrap: {
        },
        C_DOTA_Item_Whisper_Of_The_Dread: {
        },
        C_DOTA_Item_HydrasBreath: {
        },
        C_DOTA_Item_Recipe_Holy_Locket: {
        },
        CDOTA_Item_GreaterFamango: {
        },
        C_DOTA_Ability_Kez_Kazurai_Katana: {
        },
        C_DOTA_Ability_Visage_SummonFamiliars_StoneForm: {
        },
        C_DOTA_Ability_NagaSiren_Crit: {
        },
        CDOTA_Ability_Nyx_Assassin_Unburrow: {
        },
        CDOTA_Ability_Obsidian_Destroyer_AstralImprisonment: {
        },
        CDOTA_Ability_Broodmother_Spiders_Milk: {
        },
        CDOTA_Ability_Dark_Seer_Innate_MasterTactician: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Beastmaster_6: {
        },
        CDOTA_Ability_Frogmen_WaterBubble_Medium: {
        },
        CDOTA_Ability_BerserkerTroll_Break: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Sand_King_5: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Puck_5: {
        },
        C_DOTA_Ability_Special_Bonus_Lifesteal_30: {
        },
        C_DOTA_Ability_Special_Bonus_Vision_200: {
        },
        C_DOTA_Ability_Special_Bonus_Magic_Resistance_8: {
        },
        CBaseTrackedStatsEntity: {
        },
        CDOTA_Modifier_Item_Orb_of_Pestilence: {
        },
        CDOTA_Modifier_Item_GreaterCritical: {
        },
        CDOTA_Modifier_Hoodwink_Caltrops: {
        },
        CDOTA_Modifier_EarthSpirit_Magnetize: {
        },
        CDOTA_Modifier_Shadow_Demon_Disruption_Bonus_Damage: {
        },
        CDOTA_Modifier_Brewmaster_Void_Void_Strike: {
        },
        CDOTA_Modifier_Venomancer_PoisonNova: {
        },
        CDOTA_Modifier_Venomancer_PoisonSting_Applier: {
        },
        CDOTA_Modifier_Zuus_ArcLightningSlow: {
        },
        CDOTA_Modifier_DrowRanger_WaveOfSilence: {
        },
        CDOTA_Modifier_Nevermore_Requiem_InvisBreak: {
        },
        CDOTA_Modifier_AntiMage_Empowered_ManaBreak: {
        },
        C_BreakableProp: {
        },
        C_DOTA_Item_Recipe_Ancient_Janggo: {
        },
        C_DOTA_Item_TalismanOfEvasion: {
        },
        C_DOTA_Ability_ArcWarden_SparkWraith: {
        },
        CDOTA_Ability_Winter_Wyvern_Cold_Embrace: {
        },
        C_DOTA_Ability_Invoker_Empty2: {
        },
        C_DOTA_Ability_Chen_HolyPersuasion: {
        },
        CDOTA_Ability_Broodmother_SpawnSpiderite: {
        },
        CDOTA_Ability_AncientApparition_ColdFeet: {
        },
        CDOTA_Modifier_Greevil_Miniboss_Orange_LightStrikeArray_Thinker: {
        },
        CDOTA_Modifier_GnollAssassin_EnvenomedWeapon_Poison: {
        },
        CDOTA_Modifier_Watch_Tower_Invulnerable: {
        },
        C_DOTA_Aghsfort_Ability_Creature_Magnus_Push_Skewer: {
        },
        CDOTA_Modifier_Seasonal_TI9_MonkeyPoop: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Huskar_5: {
        },
        CDOTA_Modifier_Special_Bonus_Cooldown_Reduction: {
        },
        C_DOTA_Ability_Special_Bonus_Armor_7: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Speed_250: {
        },
        CDOTA_Modifier_Item_Demonicon: {
        },
        CDOTA_Modifier_Muerta_DeadShot_Slow: {
        },
        CDOTA_Modifier_Underlord_Fear: {
        },
        CDOTA_Modifier_Tusk_WalrusPunch_Slow: {
        },
        CDOTA_Modifier_Magnataur_Solid_Core: {
        },
        CDOTA_Modifier_TrollWarlord_BattleTrance: {
        },
        CDOTA_Modifier_BountyHunter_Lookout: {
        },
        CDOTA_Modifier_Broodmother_Spider_HP: {
        },
        CDOTA_Modifier_Enchantress_Untouchable_Slow: {
        },
        CDOTA_Modifier_Tidehunter_LeviathansCatch_Counter: {
        },
        CDOTA_Modifier_Windrunner_EasyBreezy: {
        },
        CDOTA_Modifier_Skeleton_King_Innate_VampiricSpirit_Aura: {
        },
        CDOTA_Modifier_Juggernaut_Omnislash_Invulnerability: {
        },
        CDOTA_Modifier_TeamShowcase_Global: {
        },
        CDOTA_Modifier_Tutorial_SpeechBubble: {
        },
        C_DOTA_Unit_ZeusCloud: {
        },
        CIngameEvent_MuertaReleaseSpring2023_DummyModifierHolder: {
        },
        C_DOTA_Item_Recipe_Elven_Tunic: {
        },
        C_DOTA_Item_Mjollnir: {
        },
        C_DOTA_Ability_Chen_TestOfFaith: {
        },
        CDOTA_Modifier_Greevil_Miniboss_Black_Nightmare_Invulnerable: {
        },
        CPrecipitationVData: {
        },
        CDOTA_Modifier_AghsFort_Creature_Venomancer_PoisonSting_Applier: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_VoidSpirit_7: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_VoidSpirit_3: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Medusa_5: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Rubick_7: {
        },
        CDOTA_Modifier_Special_Bonus_Tree_Walking: {
        },
        CDOTA_Modifier_Item_Ring_Of_Basilius_Effect: {
        },
        CDOTA_Modifier_Kez_SwitchWeapon_Flutter_Katana: {
        },
        CDOTA_Modifier_VoidSpirit_ResonantPulse_Ring: {
        },
        CDOTA_Modifier_Terrorblade_Transforming: {
        },
        CDOTA_Modifier_Magnataur_Horn_Toss: {
        },
        CDOTA_Modifier_Slark_Fish_Bait_Pre: {
        },
        CDOTA_Modifier_Fountain_Fury_Swipes_Damage_Increase: {
        },
        CDOTA_Modifier_VengefulSpirit_Arcana_Kill_Effect: {
        },
        CDOTA_Modifier_BookOfStrength: {
        },
        CDOTA_Modifier_Break: {
        },
        CPulseCell_WaitForObservable: {
        },
        C_DOTA_Unit_Hero_Furion: {
        },
        C_DOTA_Item_Recipe_AbyssalBlade: {
        },
        CDOTA_Ability_Snapfire_FiresnapCookie: {
        },
        CDOTA_Ability_Mars_ArenaOfBlood: {
        },
        C_DOTA_Ability_Grimstroke_Scepter: {
        },
        CDOTA_Ability_Shadow_Demon_Demonic_Purge: {
        },
        C_DOTA_Ability_Dazzle_Rain_Of_Vermin: {
        },
        C_DOTA_Ability_Razor_StormSurge: {
        },
        C_DOTA_Ability_Mirana_Selemenes_Faithful: {
        },
        CDOTA_Modifier_Greevil_Miniboss_White_Degen_Aura_Effect: {
        },
        CDOTA_Modifier_PushWave_Movement: {
        },
        C_SoundAreaEntitySphere: {
        },
        CDOTAInGamePredictionState: {
        },
        CDOTA_Modifier_Aghsfort_Walrus_Pudge_Harpoon: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Pangolier_6: {
        },
        C_DOTA_Ability_Special_Bonus_Lifesteal_35: {
        },
        CDOTA_Modifier_Bloodthorn_Debuff: {
        },
        CDOTA_Modifier_Grimstroke_Shard_Buff: {
        },
        CDOTA_Modifier_Oracle_FortunesEndPurge: {
        },
        CDOTA_Modifier_TrollWarlord_BattleTrance_Aura: {
        },
        CDOTA_Modifier_Wisp_Tentacles: {
        },
        CDOTA_Modifier_Ogre_Magi_Bloodlust_Autocast: {
        },
        CDOTA_Modifier_Gyrocopter_Homing_Missile: {
        },
        CDOTA_Modifier_Chen_Zealot: {
        },
        CDOTA_Modifier_Life_Stealer_Assimilate_Effect: {
        },
        CDOTA_Modifier_Nian_Flag_Trapped: {
        },
        CDOTA_Modifier_Mirana_CelestialQuiver: {
        },
        CDOTA_Modifier_Knockback: {
        },
        CDOTA_Modifier_Armor_Per_Barracks: {
        },
        CPulseCell_Step_EntFire: {
        },
        C_DOTA_Item_Yasha_And_Kaya: {
        },
        C_DOTA_Item_Recipe_HeavensHalberd: {
        },
        C_DOTA_Item_Recipe_HelmOfTheDominator_2: {
        },
        C_DOTA_Ability_Ogre_Magi_Smash: {
        },
        C_DOTA_Ability_Nian_Tail_Swipe: {
        },
        C_DOTA_Ability_Ursa_Fury_Swipes: {
        },
        C_DOTA_Ability_Zuus_Heavenly_Jump: {
        },
        CDOTA_Modifier_OgreSmash_Swing: {
        },
        CDOTA_Ability_Spawnlord_Master_Bash: {
        },
        CDOTA_Modifier_GnollAssassin_EnvenomedWeapon: {
        },
        C_BaseButton: {
        },
        CDOTA_Modifier_AghsFort_Watch_Tower: {
        },
        CDOTA_Modifier_Ascension_Bulwark: {
        },
        C_DOTA_Ability_Lesser_NightCrawler_Pounce: {
        },
        CDOTA_Ability_Seasonal_TI9_Instruments: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Warlock_2: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Spectre_5: {
        },
        CDOTA_Modifier_Special_Bonus_Unique_Morphling_4: {
        },
        C_DOTA_Ability_Special_Bonus_Exp_Boost_20: {
        },
        C_DOTA_DataRadiant: {
        },
        CDOTA_Modifier_Item_Urn_Damage: {
        },
        CDOTA_Modifier_Kez_Shodo_Sai_Mark: {
        },
        CDOTA_Modifier_DeathProphet_Silence_Debuff: {
        },
        CDOTA_Modifier_Enigma_Event_Horizon_Aura_Effect: {
        },
        CDOTA_Modifier_AntiMage_Mana_Thirst: {
        },
        CDOTA_Modifier_Dominated: {
        },
        CDOTA_Modifier_MagicImmune: {
        },
        CIngameEvent_FV2023: {
        },
        CHitboxComponent: {
        },
        C_DOTA_BaseNPC_Creep: {
        },
        CDOTA_Item_Lotus_Orb: {
        },
        CDOTA_Item_Recipe_ForceStaff: {
        },
        C_DOTA_Ability_TrollWarlord_BerserkersRage: {
        },
        C_DOTA_Ability_NightStalker_Void: {
        },
        C_DOTA_Ability_Earthshaker_Aftershock: {
        },
        CDOTA_Ability_Juggernaut_Innate_Mask_Crit_Lifesteal: {
        },
        CDOTA_Modifier_AncientRockGolem_Weakening_Aura: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Luna_1: {
        },
        C_DOTA_Ability_Special_Bonus_Cleave_35: {
        },
        C_DOTA_Ability_Special_Bonus_HP_275: {
        },
        CDOTA_Modifier_Arc_Warden_Scepter: {
        },
        CDOTA_Modifier_ArcWarden_Flux: {
        },
        CDOTA_Modifier_Batrider_FlamingLasso_Damage: {
        },
        CDOTA_Ability_Batrider_Flamebreak_Knockback: {
        },
        CDOTA_Modifier_Broodmother_SpawnSpiderlings_Slow: {
        },
        CDOTA_Modifier_Clinkz_SearingArrows: {
        },
        CDOTA_Modifier_Viper_Nethertoxin: {
        },
        CDOTA_Modifier_Pugna_Decrepify: {
        },
        CDOTA_Modifier_Windrunner_Windrun_Invis_Thinker: {
        },
        CDOTA_Modifier_Morphling_Scepter: {
        },
        C_DOTA_BaseNPC_Clinkz_Skeleton_Army: {
        },
        C_DOTA_BaseNPC_Launchpad: {
        },
        CDOTA_Item_IdolOfScreeauk: {
        },
        C_DOTA_Item_Butterfly: {
        },
        C_DOTA_Ability_Medusa_MysticSnake: {
        },
        C_DOTA_Ability_Visage_Lurker: {
        },
        C_DOTA_Ability_Meepo_Fling: {
        },
        CDOTA_Ability_Shadow_Demon_Disseminate: {
        },
        C_DOTA_Ability_Batrider_StickyNapalm_Application_Damage: {
        },
        C_DOTA_Ability_TemplarAssassin_SelfTrap: {
        },
        C_DOTA_Ability_CrystalMaiden_CrystalNova: {
        },
        CDOTA_Ability_Scout_Bonuses: {
        },
        CDOTA_Modifier_Wildkin_Tornado: {
        },
        CDOTA_Modifier_Mutation_KillstreakPower_Aura: {
        },
        C_DOTA_Ability_Special_Bonus_HP_Regen_14: {
        },
        C_DOTA_Ability_Special_Bonus_Cleave_60: {
        },
        C_DOTA_Ability_Special_Bonus_Base_Attack_Rate_1: {
        },
        CDOTA_Modifier_Item_Imp_Claw: {
        },
        CDOTA_Modifier_Item_CrellasCrozier_Debuff: {
        },
        CDOTA_Modifier_Elder_Titan_EarthSplitter_Caster: {
        },
        CDOTA_Modifier_Skywrath_Mage_Arcana: {
        },
        CDOTA_Modifier_Tusk_IceShard_Slow_Aura: {
        },
        CDOTA_Modifier_Silencer_GlaivesOfWisdom_SilenceDebuffCounter: {
        },
        C_DOTA_Unit_Hero_Brewmaster: {
        },
        CDOTA_BaseNPC_AghsFort_Watch_Tower: {
        },
        C_DOTAAmbientCreatureParticleZone: {
        },
        C_DOTA_Item_Caster_Rapier: {
        },
        C_DOTA_Ability_ArcWarden_Runic_Infusion: {
        },
        C_DOTA_Ability_Legion_Commander_PressTheAttack: {
        },
        C_DOTA_Ability_SkeletonKing_VampiricAura: {
        },
        C_DOTA_Ability_BackdoorProtectionInBase: {
        },
        CDOTA_Ability_ForestTrollHighPriest_HealAmp_Aura: {
        },
        CDOTA_Modifier_EnragedWildkin_Hurricane: {
        },
        CDOTA_Modifier_GreaterClarity: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Underlord_9: {
        },
        C_DOTA_Ability_Special_Bonus_Magic_Resistance_15: {
        },
        CDOTA_Modifier_Item_Disperser_Movespeed_Buff: {
        },
        CDOTA_Modifier_Shadow_Demon_Disruption_ChargeCounter: {
        },
        CDOTA_Modifier_Lycan_Howl: {
        },
        CDOTA_Modifier_Obsidian_Destroyer_Ominous_Discernment: {
        },
        CDOTA_Modifier_DarkSeer_Surge_Trail_Thinker: {
        },
        CDOTA_Modifier_Bloodseeker_Bloodbath_Thinker: {
        },
        CPathQueryComponent: {
        },
        C_DOTA_Unit_Hero_Mirana: {
        },
        C_DOTA_Item_Penta_Edged_Sword: {
        },
        C_DOTA_Item_Paintball: {
        },
        C_DOTA_Item_Recipe_MaskOfMadness: {
        },
        C_DOTA_Item_PlateMail: {
        },
        CDOTA_Ability_Ringmaster_WeightedPie: {
        },
        C_DOTA_Ability_Techies_Spoons_Stash: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Rubick_3: {
        },
        C_DOTA_Ability_Clinkz_WindWalk: {
        },
        C_DOTA_Ability_Necrolyte_Sadist: {
        },
        C_DOTA_Ability_AntiMage_Counterspell_Ally: {
        },
        CLogicRelay: {
        },
        CDOTA_Ability_AghsFort_Ascension_Invis: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Techies_3: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Naga_Siren_5: {
        },
        C_DOTA_Ability_Special_Bonus_Movement_Speed_65: {
        },
        C_DOTA_Ability_Special_Bonus_Cast_Speed_30: {
        },
        CDOTA_Modifier_Ringmaster_CrystalBall_Truesight_Aura: {
        },
        CDOTA_Modifier_Mars_Bulwark_Soldier_Thinker: {
        },
        CDOTA_Modifier_Earth_Spirit_StoneCaller_ChargeCounter: {
        },
        CDOTA_Modifier_Shredder_Exposure_Therapy: {
        },
        CDOTA_Modifier_KeeperOfTheLight_Recall: {
        },
        CDOTA_Modifier_LoneDruid_SpiritBear_Fetch_Damage: {
        },
        CDOTA_Modifier_Luna_LunarBlessingAura: {
        },
        CDOTA_Modifier_VengefulSpirit_Command_Aura_Illusion: {
        },
        C_IngameEvent_NB2020: {
        },
        SequenceHistory_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CPlayer_ItemServices: {
        },
        CPulse_OutflowConnection: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Item_WeightedDice: {
        },
        C_DOTA_Item_PyrrhicCloak: {
        },
        C_DOTA_Item_ShadowAmulet: {
        },
        C_DOTA_Ability_Primal_Beast_Innate_Slow_Resistance_Per_Time: {
        },
        C_DOTA_Ability_Naga_Siren_Reel_In: {
        },
        C_DOTA_Ability_Alchemist_Berserk_Potion: {
        },
        C_DOTA_Ability_TemplarAssassin_Meld: {
        },
        C_DOTA_Ability_Warlock_Eldritch_Summoning: {
        },
        C_DOTA_Ability_DrowRanger_Trueshot: {
        },
        C_DOTA_Ability_Storm_Spirit_Electric_Rave: {
        },
        CDOTA_Ability_Aghsfort_Bonus_Pudge_MeatHook: {
        },
        CDOTA_Modifier_AghsFort_EchoSlamPotion_Debuff: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Vengeful_Spirit_1: {
        },
        C_DOTA_Ability_Special_Bonus_Cast_Range_400: {
        },
        CDOTA_Modifier_Item_Enhancement_Brawny: {
        },
        CDOTA_Modifier_Ringmaster_WeightedPie_Debuff: {
        },
        CDOTA_Modifier_Ringmaster_FunhouseMirror_ModelScale: {
        },
        CDOTA_Modifier_TrollWarlord_BerserkersRage: {
        },
        CDOTA_Modifier_Visage_TalentBaseDamage: {
        },
        CDOTA_Modifier_Wisp_Tether_Haste: {
        },
        CDOTA_Modifier_BountyHunter_Track: {
        },
        CDOTA_Modifier_DragonKnight_BreatheFire_Reduction: {
        },
        CDOTA_Modifier_Windrunner_GaleForce_Aura: {
        },
        CDOTA_Modifier_Morphling_ScepterStatsDrain_All_Debuff: {
        },
        CDOTA_Modifier_TrueSightAll: {
        },
        C_DOTA_Item_InvisibilityEdge: {
        },
        C_DOTA_Ability_ChaosKnight_Phantasm: {
        },
        C_DOTA_Ability_Kunkka_Return: {
        },
        C_DOTA_TempTree: {
        },
        CDOTA_Ability_Plus_GuildBanner: {
        },
        CDOTA_Modifier_PudgeMiniboss_ArmorCorruptionStack: {
        },
        C_DOTA_Ability_AghsFort_StonehallGeneral_OverwhelmingOdds: {
        },
        CDOTA_Modifier_Lesser_NightCrawler_Pounce: {
        },
        CDOTA_Ability_Seasonal_Summon_Dragon: {
        },
        C_DOTA_Ability_Special_Bonus_Respawn_Reduction_45: {
        },
        C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_30: {
        },
        C_DOTA_Ability_Special_Bonus_Mana_Break_35: {
        },
        C_DOTA_Ability_Special_Bonus_MP_200: {
        },
        C_DOTA_Ability_Special_Bonus_HP_350: {
        },
        CDOTA_Modifier_Item_Timeless_Relic: {
        },
        CDOTA_Modifier_Nullifier_Mute: {
        },
        CDOTA_Modifier_MaelstromChain: {
        },
        CDOTA_Modifier_Aegis_Regen: {
        },
        CDOTA_Modifier_Item_Reaver: {
        },
        CDOTA_Modifier_Kez_FalconRush: {
        },
        CDOTA_Modifier_MonkeyKing_BounceLeap: {
        },
        CDOTA_Modifier_Oracle_DivinersDeck_TheLovers: {
        },
        CDOTA_Modifier_Bristleback_Warpath: {
        },
        CDOTA_Modifier_Rubick_CuriosityFromHeroes_Tracker: {
        },
        CDOTA_Modifier_Rubick_FadeBolt: {
        },
        CDOTA_Modifier_Treant_NaturesGrasp_Damage_Bonus: {
        },
        CDOTA_Modifier_Obsidian_Destroyer_ObjurgationCooldown: {
        },
        CDOTA_Modifier_Enchantress_Enchant_Intrinsic: {
        },
        CDOTA_Modifier_Luna_MoonGlaive: {
        },
        CDOTA_Modifier_Warlock_Upheaval_Ally: {
        },
        CDOTA_Modifier_TeamShowcase_Showcase: {
        },
        CDOTA_BaseNPC_CustomEffigy: {
        },
        C_DOTA_Item_Armlet: {
        },
        C_DOTA_Item_PointBooster: {
        },
        C_DOTA_Ability_Bristleback_ViscousNasalGoo: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Undying_3: {
        },
        C_DOTA_Ability_Obsidian_Destroyer_EssenceAura: {
        },
        C_DOTA_Ability_Obsidian_Destroyer_ArcaneOrb: {
        },
        C_DOTA_Ability_Invoker_Tornado: {
        },
        C_DOTA_Ability_Dazzle_Poison_Touch: {
        },
        CDOTA_Ability_PhantomLancer_IllusoryArmaments: {
        },
        C_DOTA_Ability_Lua: {
        },
        CDOTA_Modifier_BlackDragon_SplashAttack: {
        },
        C_DOTA_Ability_Launchpad: {
        },
        CDOTA_Modifier_Stacking_Gold_Rate_Boost: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Earth_Spirit_2: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_30: {
        },
        CDOTA_Modifier_Item_Trickster_Cloak_Invis: {
        },
        CDOTA_Modifier_Item_Enchanted_Quiver: {
        },
        CDOTA_Modifier_Item_Radiance: {
        },
        CDOTA_Modifier_Item_WardTrueSight: {
        },
        CDOTA_Modifier_Item_PoorMansShield: {
        },
        CDOTA_Modifier_Largo_CatchyLick_Intrinsic: {
        },
        CDOTA_Modifier_Largo_FroglingBand2_Frogling_Leave: {
        },
        CDOTA_Modifier_Kez_GrapplingClaw_Movement: {
        },
        CDOTA_Modifier_Ringmaster_Spotlight_Thinker: {
        },
        CDOTA_Modifier_Wisp_Tether_Attack_Damage_Penalty: {
        },
        CDOTA_Modifier_Chaos_Knight_Chaos_Bolt_Additional_Debuff_Delay: {
        },
        CDOTA_Modifier_Gyrocopter_Flak_Cannon_Scepter: {
        },
        CDOTA_Modifier_TemplarAssassin_PsiBlades_Slow: {
        },
        CDOTA_Modifier_Tinker_LaserBlind: {
        },
        CDOTA_Modifier_AntiMage_Puritan_Slow: {
        },
        CIngameEvent_Fall2021: {
        },
        CDOTA_Unit_Hero_Alchemist: {
        },
        C_DOTA_Unit_Hero_PhantomLancer: {
        },
        C_DOTA_Item_Recipe_Desolator: {
        },
        C_DOTA_Item_Recipe_HelmOfTheOverlord: {
        },
        C_DOTA_Ability_EmberSpirit_FireRemnant: {
        },
        CDOTA_Ability_Magnus_Strength_Of_Joelrak: {
        },
        C_DOTA_Ability_Life_Stealer_Rage: {
        },
        C_DOTA_Ability_DragonKnight_ElderDragonForm: {
        },
        C_DOTA_Ability_Juggernaut_Bladeform: {
        },
        C_EnvVolumetricFogController: {
        },
        CDOTA_Modifier_AghsFort_TreantMiniboss_NaturesGuise_NearTreeDisplay: {
        },
        C_DOTA_Ability_Seasonal_Festive_Firework: {
        },
        CDOTA_Modifier_Seasonal_Penguin: {
        },
        CDOTA_Ability_ShootFirework: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Silencer_4: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Pugna_5: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Riki_3: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Earthshaker_6: {
        },
        C_DOTA_Ability_Special_Bonus_Night_Vision_1000: {
        },
        C_DOTA_Ability_Special_Bonus_Magic_Resistance_80: {
        },
        C_DOTA_Ability_Special_Bonus_Magic_Resistance_12: {
        },
        CDOTA_Modifier_Item_Enhancement_Alert: {
        },
        CDOTA_Modifier_Item_Giants_Ring: {
        },
        CDOTA_Modifier_Orb_Of_Revelations: {
        },
        CDOTA_Modifier_Item_Arcane_Blink: {
        },
        CDOTA_Modifier_ShadowAmulet_Fade: {
        },
        CDOTA_Modifier_MonkeyKing_Transfiguration_Hidden: {
        },
        CDOTA_Modifier_Visage_GravekeepersCloak_Secondary: {
        },
        CDOTA_Modifier_Rubick_Telekinesis: {
        },
        CDOTA_Modifier_Silencer_CurseOfTheSilent_Intrinsic: {
        },
        CDOTA_Modifier_Weaver_Rewoven: {
        },
        CDOTA_Modifier_Huskar_Inner_Fire_Disarm: {
        },
        CDOTA_Modifier_Slardar_Puddle: {
        },
        CDOTA_Modifier_Tiny_CraggyExterior_Debuff: {
        },
        CDOTA_Modifier_Razor_UnstableCurrent: {
        },
        CPulseGraphDef: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        C_DynamicPropClientside: {
        },
        C_DOTA_Item_Recipe_Mind_Breaker: {
        },
        CDOTA_Item_Recipe_Kaya_And_Sange: {
        },
        CDOTA_Item_Famango: {
        },
        C_DOTA_Item_Radiance: {
        },
        C_DOTA_Item_Bracer: {
        },
        C_DOTA_Ability_Abyssal_Underlord_Raid_Boss: {
        },
        C_DOTA_Ability_Elder_Titan_AncestralSpirit: {
        },
        C_DOTA_Ability_KeeperOfTheLight_Illuminate: {
        },
        C_DOTA_Ability_Lone_Druid_Bear_Necessities: {
        },
        CDOTA_Ability_Gyrocopter_Side_Gunner_SpawnAbility: {
        },
        C_DOTA_Ability_Jakiro_Liquid_Ice: {
        },
        C_DOTA_Ability_Courier_TransferItems: {
        },
        C_DOTA_Ability_Riki_Innate_Backstab: {
        },
        CDOTA_Modifier_Aghsfort_Hoodwink_ArcingBoomerang: {
        },
        CDOTA_Ability_AghsFort_RestorativeFlower: {
        },
        CDOTA_Modifier_AghsFort_Ascension_Invis: {
        },
        CDOTA_Modifier_AghsFort_Ascension_Silence_Display: {
        },
        C_DOTA_Ability_Special_Bonus_Gold_Income_150: {
        },
        CDOTA_Modifier_Kez_ShodoSai_Parry: {
        },
        CDOTA_Modifier_Muerta_TheCalling_Silence: {
        },
        CDOTA_Modifier_Furion_Teleport_Shield: {
        },
        CDOTA_Modifier_Razor_PlasmaField_Slow: {
        },
        C_IngameEvent_FM2016: {
        },
        C_EnvDetailController: {
        },
        C_DOTA_Item_VindicatorsAxe: {
        },
        CDOTA_Item_Recipe_Ballista: {
        },
        C_DOTA_Item_AeonDisk: {
        },
        C_DOTA_Item_HelmOfTheDominator: {
        },
        C_DOTA_Item_Recipe_UltimateScepter: {
        },
        C_DOTA_Ability_Kez_FalconRush: {
        },
        C_DOTA_Ability_Oracle_FortunesEnd: {
        },
        CDOTA_Ability_Elder_Titan_FundamentalFury: {
        },
        C_DOTA_Ability_KeeperOfTheLight_BlindingLight: {
        },
        C_DOTA_Ability_Rubick_Hidden3: {
        },
        C_DOTA_Ability_DeathProphet_Witchcraft: {
        },
        C_EnvWindVolume: {
        },
        C_DOTA_Ability_Special_Bonus_Lifesteal_12: {
        },
        C_DOTA_Ability_Special_Bonus_Night_Vision_400: {
        },
        CDOTA_Modifier_Holy_Locket_ActiveBuff: {
        },
        CDOTA_Modifier_Black_King_Bar_Immune: {
        },
        CDOTA_Modifier_Hoodwink_Caltrops_Debuff: {
        },
        CDOTA_Modifier_EarthSpirit_StoneThinker: {
        },
        CDOTA_Modifier_EmberSpirit_FireRemnant: {
        },
        CDOTA_Modifier_Elder_Titan_NaturalOrder_Armor: {
        },
        CDOTA_Modifier_Broodmother_PoisonStingDebuff: {
        },
        CDOTA_Modifier_Tinker_Innate_Keen_Teleport_Gold_On_Death: {
        },
        CDOTA_Modifier_Riki_BlinkStrike: {
        },
        CDOTA_Modifier_Lua_Motion_Both: {
        },
        CDOTA_Modifier_FixedNumberOfHitsToKill: {
        },
        CBasePlayerControllerAPI: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Unit_Hero_Invoker: {
        },
        C_CrownfallShmupEnemy: {
        },
        C_DOTA_GuildBannerDynamic: {
        },
        CEnvSoundscapeAlias_snd_soundscape: {
        },
        C_DOTA_Item_Enhancement_Quickened: {
        },
        CDOTA_Item_Recipe_Essence_Ring: {
        },
        C_DOTA_Item_Recipe_MantaStyle: {
        },
        C_DOTA_Item_NullTalisman: {
        },
        C_DOTA_Ability_Brewmaster_EarthBrewling_EarthStance: {
        },
        C_DOTA_Ability_Invoker_ChaosMeteor: {
        },
        C_DOTA_Ability_Jakiro_DoubleTrouble: {
        },
        C_DOTA_Ability_BountyHunter_Cutpurse: {
        },
        C_DOTA_Ability_Windrunner_FocusFire_End: {
        },
        CDOTA_Ability_SkeletonKing_SpectralBlade: {
        },
        C_DOTA_Ability_Greevil_Miniboss_Purple_VenomousGale: {
        },
        CDOTA_Modifier_HillTroll_Rally: {
        },
        CDOTA_Ability_EnragedWildkin_Hurricane: {
        },
        CDOTA_Modifier_Tornado_Tempest: {
        },
        C_DOTA_Ability_Spawnlord_Master_Freeze: {
        },
        CDOTA_Modifier_AghsFort_BossWinterWyvern_Cold_Embrace_Thinker: {
        },
        CDOTA_Modifier_AghsFort_AmoebaBoss_Summoned_Knockback: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Riki_2: {
        },
        CDOTA_Modifier_Special_Bonus_Mana_Break: {
        },
        C_DOTA_Ability_Special_Bonus_Intelligence_30: {
        },
        CDOTA_Modifier_Item_Enhancement_Evolved: {
        },
        CDOTA_Modifier_Elder_Titan_AncestralSpirit: {
        },
        CDOTA_Modifier_Abaddon_Withering_Mist_Debuff: {
        },
        CDOTA_Modifier_Magnataur_Skewer_Slow: {
        },
        CDOTA_Modifier_Centaur_DoubleEdge_Buff: {
        },
        CDOTA_Modifier_Visage_SummonFamiliars_In_Formation: {
        },
        CDOTA_Modifier_Lycan_SummonWolves_GeistForm: {
        },
        CDOTA_Modifier_Spectre_Arcana_Kill_Effect: {
        },
        CDOTA_Modifier_BountyHunter_WindWalk_Slow: {
        },
        CDOTA_Modifier_Windrunner_TailwindIntrinsic: {
        },
        CDOTA_Modifier_Bloodseeker_Thirst_Vision: {
        },
        CDOTA_Modifier_VengefulSpirit_WaveOfTerror_Buff: {
        },
        CDOTA_Modifier_Nevermore_Presence_Aura: {
        },
        CDOTA_Modifier_MP_Regen: {
        },
        CDOTA_ArcanaDataEntity_DrowRanger: {
        },
        C_CSequenceTransitioner2: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_GameRulesProxy: {
        },
        CRenderComponent: {
        },
        CDOTA_Item_Recipe_Quickening_Charm: {
        },
        C_DOTA_Item_Arcane_Boots: {
        },
        C_DOTA_Ability_Gyrocopter_Flak_Cannon: {
        },
        C_DOTA_Ability_DragonKnight_DragonTail: {
        },
        C_DOTA_Ability_FacelessVoid_TimeLock: {
        },
        C_DOTA_Ability_CrystalMaiden_FreezingField: {
        },
        CDOTA_Ability_SkeletonKing_BoneGuard_DamageTracker: {
        },
        C_Team: {
        },
        C_DOTA_Ability_AghsFort_Creature_Venomancer_PoisonNova: {
        },
        CDOTA_Ability_Aghsfort_Reward_HPAura: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Meepo_2: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Enigma_2: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_40: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Range_60: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Base_Damage_20: {
        },
        CDOTA_Modifier_Item_Rabbits_Foot: {
        },
        CDOTA_Modifier_Item_Enhancement_Restorative: {
        },
        CDOTA_Modifier_Item_Illusionsts_Cape: {
        },
        CDOTA_Modifier_Item_Blight_Stone: {
        },
        CDOTA_Modifier_Item_HandOfMidas: {
        },
        CDOTA_Modifier_Dark_Willow_Pixie_Dust: {
        },
        CDOTA_Modifier_MonkeyKing_Bounce: {
        },
        CDOTA_Modifier_Techies_MutuallyAssuredDestruction: {
        },
        CDOTA_Modifier_Naga_Siren_Reel_In: {
        },
        CDOTA_Modifier_SpiritBreaker_ChargeOfDarkness_Untargetable: {
        },
        CDOTA_Modifier_Enchantress_Little_Friends_Aura: {
        },
        CDOTA_Modifier_Drow_Ranger_Multishot: {
        },
        CDOTA_Modifier_Creep_Haste: {
        },
        C_DOTA_Item_Divine_Regalia_Broken: {
        },
        C_DOTA_Item_Enhancement_Tough: {
        },
        C_DOTA_Item_Royal_jelly: {
        },
        C_DOTA_Item_Necronomicon_Level3: {
        },
        C_DOTA_Ability_Nyx_Assassin_Impale: {
        },
        C_DOTA_Ability_PhantomLancer_Juxtapose: {
        },
        C_PathParticleRopeAlias_path_particle_rope_clientside: {
        },
        CPointChildModifier: {
        },
        CDOTA_Modifier_Morty_Hop: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Techies_4: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lich_3: {
        },
        C_DOTA_Ability_Special_Bonus_Cast_Range_275: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Range_325: {
        },
        CDOTA_Modifier_Item_Eternal_Shroud_Barrier: {
        },
        CDOTA_Modifier_EmberSpirit_SearingChains: {
        },
        CDOTA_Modifier_Centaur_Return_Counter: {
        },
        CDOTA_Modifier_Wisp_Relocate_Return: {
        },
        CDOTA_Modifier_Invoker_SunStrike: {
        },
        CDOTA_Modifier_Clinkz_Burning_Army_Thinker: {
        },
        CDOTA_Modifier_Beastmaster_InnerBeast: {
        },
        CDOTA_Modifier_Miniboss_Fortification: {
        },
        CDOTA_Modifier_Miniboss_UnyieldingShield: {
        },
        CDOTA_Modifier_SkeletonKing_HellfireBlast_Skeleton_Buff: {
        },
        CDOTA_Modifier_Nevermore_Requiem_Fear: {
        },
        C_BaseAnimatingController: {
        },
        C_DOTA_Unit_Undying_Zombie: {
        },
        C_TriggerMultiple: {
        },
        C_DOTA_Item_Tome_Of_Knowledge: {
        },
        CDOTA_Item_Silver_Edge: {
        },
        CDOTA_Ability_Snapfire_MortimerKisses: {
        },
        CDOTA_Ability_Brewmaster_Void_Astral_Pull: {
        },
        C_DOTA_Ability_Chen_TestOfFaithTeleport: {
        },
        C_DOTA_Ability_Omniknight_GuardianAngel: {
        },
        C_DOTA_Ability_Tinker_Eureka: {
        },
        CDOTA_Modifier_Frogmen_Riverborn_Aura: {
        },
        CDOTA_Modifier_BlackDragon_Fireball_Thinker: {
        },
        CDOTA_Modifier_Creature_IceSlam_Thinker: {
        },
        CDOTA_Modifier_Seasonal_TI9_Shovel_Stasis_Trap: {
        },
        C_DOTA_Ability_Special_Bonus_Exp_Boost_40: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Range_200: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Range_100: {
        },
        C_DOTA_Ability_Special_Bonus_Status_Resistance_10: {
        },
        C_DOTA_Ability_Special_Bonus_Agility_10: {
        },
        C_DOTA_Ability_Special_Bonus_HP_125: {
        },
        CDOTA_Modifier_Mechanical_Arm_Counter: {
        },
        CDOTA_Modifier_Item_EmptyBottle: {
        },
        CDOTA_Modifier_Muerta_TheCallingAuraSlow: {
        },
        CDOTA_Modifier_Shadow_Demon_Disruption: {
        },
        CDOTA_Modifier_Lycan_SummonWolves_PermanentInvisibility: {
        },
        CDOTA_Modifier_SpiritBreaker_ChargeOfDarknessVision: {
        },
        CDOTA_Modifier_Venomancer_Snakebite: {
        },
        CDOTA_Modifier_CDOTA_Modifier_CrystalMaiden_IceRink_Movement_Ricochet: {
        },
        CDOTA_Modifier_Earthshaker_Echoslam_Heating_Up: {
        },
        C_BodyComponentBaseAnimatingOverlay: {
        },
        CDOTA_NPC_Observer_Ward: {
        },
        C_DOTA_Item_Enhancement_Alert: {
        },
        C_DOTA_Item_Witch_Blade: {
        },
        CDOTA_Item_Recipe_Dragon_Scale: {
        },
        CDOTA_Item_Recipe_Silver_Edge: {
        },
        CDOTA_Ability_PrimalBeast_Rock_Throw: {
        },
        CDOTA_Ability_Winter_Wyvern_Winters_Curse: {
        },
        C_DOTA_Ability_Animation_Attack: {
        },
        C_DOTA_Ability_Razor_PlasmaField: {
        },
        CDOTA_Ability_Pudge_FleshHeap: {
        },
        C_DOTA_Ability_FlagBearer_Creep_Aura_Effect: {
        },
        CDOTA_Modifier_Aghsfort_Elemental_Wisp_Tether_Haste: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Broodmother_5: {
        },
        CDOTA_Modifier_Item_RingOfTarrasque: {
        },
        CDOTA_Modifier_Item_Arcane_Boots: {
        },
        CDOTA_Modifier_Item_Boots_Of_Bearing_Active: {
        },
        CDOTA_Modifier_ForceStaff: {
        },
        CDOTA_Modifier_Snapfire_LilShredder_Buff: {
        },
        CDOTA_Modifier_Tusk_Snowball_Visible: {
        },
        CDOTA_Modifier_Ogre_Magi_Fireblast_Attack_Proc: {
        },
        CDOTA_Modifier_LoneDruid_SpiritBear_Fetch: {
        },
        CDOTA_Modifier_Lycan_Wolf_Uncontrollable: {
        },
        CDOTA_Modifier_Viper_PoisonAttack_Slow: {
        },
        CDOTA_Modifier_Necrolyte_ReapersScythe: {
        },
        CDOTA_Modifier_Earthshaker_EnchantTotem_Leap: {
        },
        CDOTA_Unit_Hero_AbyssalUnderlord: {
        },
        C_DOTA_Item_Tier4Token: {
        },
        CDOTA_Item_Riftshadow_Prism: {
        },
        C_DOTA_Item_Falcon_Blade: {
        },
        C_DOTA_Item_PhaseBoots: {
        },
        C_DOTA_Ability_MonkeyKing_Transfiguration: {
        },
        C_DOTA_Ability_Jakiro_Macropyre: {
        },
        C_DOTA_Ability_Nian_Roar: {
        },
        C_DOTA_Ability_Bane_BrainSap: {
        },
        CDOTA_Ability_AncientApparition_ChillingTouch: {
        },
        CDOTA_Modifier_Watch_Tower_Capturing: {
        },
        CDOTA_Modifier_Healing_Campfire_Heal: {
        },
        CDOTA_Modifier_Item_Angels_Demise: {
        },
        CDOTA_Modifier_Aether_Lens: {
        },
        CDOTA_Modifier_Item_Ward_Dispenser: {
        },
        CDOTA_Modifier_Item_RingOfProtection: {
        },
        CDOTA_Modifier_Broodmother_SpinWebInvisibleApplier: {
        },
        CDOTA_Modifier_Beastmaster_PrimalRoar_Push: {
        },
        CDOTA_Modifier_Animation_LeftClawSwipe: {
        },
        CDOTA_Unit_Hero_Largo: {
        },
        C_ColorCorrectionVolume: {
        },
        CDOTA_Item_Recipe_Pirate_Hat: {
        },
        C_DOTA_Item_MagicWand: {
        },
        C_DOTA_Ability_Wisp_Empty2: {
        },
        C_DOTA_Ability_Rubick_NullField: {
        },
        C_DOTA_Ability_Brewmaster_PermanentImmolation: {
        },
        C_DOTA_Ability_Invoker_SunStrike_AD: {
        },
        C_DOTA_Ability_SpiritBreaker_Bulldoze: {
        },
        CDOTA_Ability_Batrider_Smoldering_Resin: {
        },
        C_DOTA_Ability_Huskar_Burning_Spear: {
        },
        C_DOTA_Ability_Viper_ViperStrike: {
        },
        C_DOTA_Ability_Necrolyte_ReapersScythe: {
        },
        CDOTA_Modifier_BlackDrake_MagicAmplification: {
        },
        CDOTA_Modifier_Twin_Gate_FX: {
        },
        CDOTA_Modifier_AghsFort_TorrentEffectPotion_Torrent_Slow: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Ember_Spirit_2: {
        },
        C_DOTA_Ability_Special_Bonus_Intelligence_6: {
        },
        C_DOTA_Ability_Special_Bonus_All_Stats_6: {
        },
        C_DOTA_Ability_Special_Bonus_Cleave_100: {
        },
        CDOTA_Modifier_Item_Headdress: {
        },
        CDOTA_Modifier_Item_Vladmir: {
        },
        CDOTA_Modifier_Slark_SaltwaterShiv_Slow: {
        },
        CDOTA_Modifier_Invoker_EMP_Pull_Thinker: {
        },
        CDOTA_Modifier_Omniknight_Degen_Aura: {
        },
        CDOTA_Modifier_Prosperous_Soul: {
        },
        CPlayer_MovementServices: {
        },
        CDOTA_Unit_Hero_Dawnbreaker: {
        },
        CInfoDynamicShadowHintBox: {
        },
        C_DOTA_Item_Mind_Breaker: {
        },
        CDOTA_Item_Solar_Crest: {
        },
        C_DOTA_Ability_Luna_LunarBlessing: {
        },
        C_DOTA_Ability_Viper_Innate_Corrosive_Skin_Vile: {
        },
        CDOTA_Modifier_PushWave: {
        },
        CBaseAnimGraphController: {
        },
        CDOTA_Modifier_Seasonal_Firecrackers: {
        },
        C_DOTA_Ability_Special_Bonus_Exp_Boost_30: {
        },
        C_DOTA_Ability_Special_Bonus_Armor_20: {
        },
        C_DOTA_Ability_Special_Bonus_All_Stats_10: {
        },
        C_DOTA_Ability_Special_Bonus_MP_Regen_Amp_10: {
        },
        C_DOTA_Ability_Special_Bonus_Reincarnation_250: {
        },
        C_DOTA_Ability_Special_Bonus_Cleave_40: {
        },
        CDOTAPlayer_MovementServices: {
        },
        CDOTA_Modifier_Item_Chasm_Stone: {
        },
        CDOTA_Modifier_Item_Swift_Blink: {
        },
        CDOTA_Modifier_Ringmaster_Unicycle_Cycle: {
        },
        CDOTA_Modifier_Ringmaster_TheBox_Buff: {
        },
        CDOTA_Modifier_Hoodwink_AcornShot_DelayThinker: {
        },
        CDOTA_Modifier_Skywrath_Mage_Shard_Bonus: {
        },
        CDOTA_Modifier_SpiritBreaker_HerdMentality_Aura: {
        },
        CDOTA_Modifier_QueenOfPain_Innate_Spell_Reflect: {
        },
        CDOTA_Modifier_Sniper_Shrapnel_Slow: {
        },
        C_ColorCorrection: {
        },
        CBuoyancyHelper: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Unit_AghsFort_SpectralTusk_Tombstone: {
        },
        C_DOTA_Ability_MonkeyKing_Boundless_Strike: {
        },
        C_DOTA_Ability_Visage_SoulAssumption: {
        },
        C_DOTA_Ability_Undying_SoulRip: {
        },
        C_DOTA_Ability_Chen_DivineFavor: {
        },
        C_DOTA_Ability_Beastmaster_WildAxes: {
        },
        CDOTA_Ability_Sniper_TakeAimStop: {
        },
        C_PhysBox: {
        },
        C_DOTA_Ability_AghanimsFortress_SkeletonKing_VampiricAura: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Zeus: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Drow_Ranger_1: {
        },
        CDOTA_Modifier_Special_Bonus_Corruption: {
        },
        C_DOTA_Ability_Special_Bonus_20_Crit_15: {
        },
        CDOTA_Modifier_Item_Revenants_Brooch_Active: {
        },
        CDOTA_Modifier_DarkWillow_ShadowRealm_Buff_Attack_Logic: {
        },
        CDOTA_Modifier_Shredder_TwistedChakram: {
        },
        CDOTA_Modifier_Treant_NaturesGrasp_Latch_Thinker: {
        },
        CDOTA_Modifier_Invoker_WexInstance: {
        },
        CDOTA_Modifier_Dazzle_NothlProjection_ProjectileReturn: {
        },
        CDOTA_Modifier_QueenOfPain_SonicWave_Damage: {
        },
        CDOTA_Modifier_Innate_Riki_Backstab: {
        },
        CDOTA_Modifier_Razor_Link_Vision: {
        },
        C_DOTA_PortraitTree: {
        },
        C_DOTA_Item_Light_Collector: {
        },
        CDOTA_Item_Recipe_Quicksilver_Amulet: {
        },
        CDOTA_Item_Recipe_Seer_Stone: {
        },
        C_DOTA_Item_TeleportScroll: {
        },
        C_DOTA_Item_BootsOfTravel: {
        },
        C_DOTA_Ability_MonkeyKing_Spring_Early: {
        },
        C_DOTA_Ability_Phoenix_SunRayToggleMoveEmpty: {
        },
        C_DOTA_Ability_Phoenix_SunRayStop: {
        },
        CDOTA_Ability_DoomBringer_Empty2: {
        },
        C_DOTA_Ability_Beastmaster_Hawk_Perch: {
        },
        C_DOTA_Ability_Tinker_Innate_Keen_Teleport_Gold_On_Death: {
        },
        C_DOTA_Ability_Nevermore_Shadowraze3: {
        },
        CDOTA_Modifier_Spawnlord_Aura_Bonus: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Lone_Druid_8: {
        },
        C_DOTA_Ability_Special_Bonus_Cast_Range_150: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Range_275: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_12: {
        },
        CDOTA_Modifier_Item_Book_Of_Shadows: {
        },
        CDOTA_Modifier_Item_Orb_of_Pestilence_Slow: {
        },
        CDOTA_Modifier_Item_Fallen_Sky_Land: {
        },
        CDOTA_Modifier_Ringmaster_WeightedPie_Blind: {
        },
        CDOTA_Modifier_AbyssalUnderlord_Firestorm_Burn: {
        },
        CDOTA_Modifier_Medusa_SplitShot: {
        },
        CDOTA_Modifier_SpiritBreaker_SpecialAttack: {
        },
        CDOTA_Modifier_DoomBringer_Devour_Hero_Debuff: {
        },
        CDOTA_Modifier_DoomBringer_Devour_Intrinsic: {
        },
        CDOTA_Modifier_Jakiro_Macropyre_Ice_EdgeThinker: {
        },
        CDOTA_Modifier_Dazzle_Innate_Weave_Armor: {
        },
        CDOTA_Modifier_Courier_Passive_Bonus: {
        },
        CDOTA_Modifier_Enigma_BlackHoleThinker: {
        },
        CDOTA_Modifier_Windrunner_Windrun: {
        },
        CDOTA_Modifier_NotOnMinimap: {
        },
        CDOTA_Modifier_Item_Editor: {
        },
        CDOTA_Modifier_BackdoorProtection: {
        },
        CDOTA_Item_OgreSealTotem: {
        },
        CDOTA_Item_RiverPainter3: {
        },
        C_DOTA_Ability_Nyx_Assassin_Vendetta: {
        },
        C_DOTA_Ability_Meepo_FairShare: {
        },
        C_DOTA_Ability_Morphling_MorphReplicate: {
        },
        CDOTA_Ability_Bloodseeker_BloodMist: {
        },
        CDOTA_Modifier_Greevil_Miniboss_Yellow_IonShell: {
        },
        CFilterMultiple: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Invoker_2: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Storm_Spirit_4: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Dazzle_1: {
        },
        CDOTA_Modifier_FlaskHealing: {
        },
        CDOTA_Modifier_Obsidian_Destroyer_Equilibrium_Buff: {
        },
        CDOTA_Modifier_Batrider_FlamingLasso_Self: {
        },
        CDOTA_Modifier_Life_Stealer_InfestDot: {
        },
        CDOTA_Modifier_Furion_Sprout_Tether: {
        },
        CDOTA_Modifier_Beastmaster_Axe_Invulnerable: {
        },
        CDOTA_Modifier_AncientApparition_IceVortexThinker: {
        },
        C_IngameEvent_FM2015: {
        },
        CPulseCell_FireCursors: {
        },
        CDOTA_Unit_Hero_PrimalBeast: {
        },
        C_DOTA_Unit_Hero_Riki: {
        },
        C_DOTA_Item_Force_Field: {
        },
        C_DOTA_Item_Vladmir: {
        },
        C_DOTA_Ability_Magnataur_Solid_Core: {
        },
        C_DOTA_Ability_Visage_Silent_As_The_Grave: {
        },
        C_DOTA_Ability_Silencer_GlobalSilence: {
        },
        CDOTA_Modifier_Frogmen_Riverborn_Aura_Bonus: {
        },
        CDOTA_Modifier_Watch_Tower: {
        },
        CEnvSoundscape: {
        },
        C_SoundEventEntityAlias_snd_event_point: {
        },
        C_FogController: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Broodmother_1: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Phantom_Assassin_4: {
        },
        CDOTA_Modifier_Item_MantaStyle: {
        },
        CDOTA_Modifier_Dawnbreaker_Hearthfire: {
        },
        CDOTA_Modifier_Special_Bonus_Unique_Elder_Titan_4: {
        },
        CDOTA_Modifier_Meepo_MegaMeepo_Self: {
        },
        CDOTA_Modifier_Lone_Druid_Spirit_Bear_Self_Item_Restrictions: {
        },
        CDOTA_Modifier_Gyrocopter_Chop_Shop: {
        },
        CDOTA_Modifier_Jakiro_Macropyre_Thinker: {
        },
        CDOTA_Modifier_Sniper_Assassinate_CastRange: {
        },
        CDOTA_Modifier_PhantomLancer_Juxtapose_Thinker: {
        },
        CDOTA_Modifier_Juggernaut_Bladeform: {
        },
        CDOTA_Modifier_FountainAura: {
        },
        C_SoundOpvarSetOBBWindEntity: {
        },
        C_DOTA_Unit_Hero_Lion: {
        },
        C_NetTestBaseCombatCharacter: {
        },
        C_DOTA_Item_OgreAxe: {
        },
        C_DOTA_Item_Recipe_PowerTreads: {
        },
        C_DOTA_Ability_EarthSpirit_RollingBoulder: {
        },
        C_DOTA_Ability_NagaSiren_Ensnare: {
        },
        C_DOTA_Ability_SpiritBreaker_GreaterBash: {
        },
        C_DOTA_Ability_Dazzle_Innate_Weave: {
        },
        C_DOTA_Ability_PhantomAssassin_CoupdeGrace: {
        },
        C_DOTA_Ability_BlueDragonspawnOverseer_DevotionAura: {
        },
        CDOTA_Modifier_Seasonal_Summon_TI11_Balloon_Visuals: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Naga_Siren_6: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Luna_5: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Templar_Assassin_4: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Enigma_5: {
        },
        C_DOTA_Ability_Special_Bonus_All_Stats_15: {
        },
        C_DOTA_Ability_Special_Bonus_HP_Regen_25: {
        },
        C_DOTA_Ability_Special_Bonus_HP_325: {
        },
        CDOTA_Modifier_Item_Rattlecage: {
        },
        CDOTA_Modifier_Riki_Poison_Dart: {
        },
        CDOTA_Modifier_PhantomLancer_PhantomEdge_Boost: {
        },
        CDOTA_Modifier_CrystalMaiden_BrillianceAuraEffect: {
        },
        CBodyComponentPoint: {
        },
        CDOTA_Ability_Techies_LandMines: {
        },
        CDOTA_Ability_Dazzle_NothlProjection: {
        },
        C_DOTA_Ability_Sniper_Assassinate: {
        },
        C_DOTA_Ability_Mirana_Leap: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_15: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Speed_90: {
        },
        CPlayerTrackedStatsEntity: {
        },
        CDOTA_Modifier_Illusionsts_Cape_Marker: {
        },
        CDOTA_Modifier_Item_WraithPact_DeathAura: {
        },
        CDOTA_Modifier_Item_Mekansm_Aura: {
        },
        CDOTA_Modifier_Winter_Wyvern_Essence_Of_The_Blueheart: {
        },
        CDOTA_Modifier_Underlord_Portal_Buff: {
        },
        CDOTA_Modifier_Slark_EssenceShift_Buff: {
        },
        CDOTA_Modifier_Slark_Pounce: {
        },
        CDOTA_Modifier_Visage_SummonFamiliars_StoneForm_Timer: {
        },
        CDOTA_Modifier_Obsidian_Destroyer_EssenceAura: {
        },
        CDOTA_Modifier_Invoker_SunStrike_Cataclysm: {
        },
        CDOTA_Modifier_SpiritBreaker_GreaterBash: {
        },
        CDOTA_Modifier_Weaver_Rewoven_Stacks: {
        },
        CDOTA_Modifier_Life_Stealer_Infest_Enemy_Hero: {
        },
        CDOTA_Modifier_SkeletonKing_BoneGuard_DamageTracker: {
        },
        C_DOTA_Unit_Hero_Shadow_Demon: {
        },
        C_DOTA_Unit_Poogie: {
        },
        C_DOTA_Unit_AghsFort_Creature_Batrider: {
        },
        CDOTAPropConsumableBanner: {
        },
        CDOTA_Item_GlimmerCape: {
        },
        C_DOTA_Item_Dagon_Upgraded2: {
        },
        CDOTA_Item_Battlefury: {
        },
        CDOTA_Ability_Tusk_WalrusKick: {
        },
        CDOTA_Ability_Filler_Tooltip: {
        },
        C_DOTA_Ability_Creep_Siege: {
        },
        C_EconItemView: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Doom_2: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Amplify_5: {
        },
        C_DOTA_Ability_Special_Bonus_Armor_30: {
        },
        C_DOTA_Ability_Special_Bonus_Strength_11: {
        },
        C_DOTA_Ability_Special_Bonus_Mana_Reduction_11: {
        },
        CDOTA_Modifier_Item_Blade_Mail_Reflect: {
        },
        CDOTA_Modifier_Item_BootsOfElven: {
        },
        CDOTA_Modifier_Item_BootsOfTravel_2: {
        },
        CDOTA_Modifier_Largo_Frogstomp_Debuff: {
        },
        CDOTA_Modifier_Hoodwink_Sharpshooter_Recoil_EndAnim: {
        },
        CDOTA_Modifier_ArcWarden_MagneticField_Evasion: {
        },
        CDOTA_Modifier_EarthSpirit_GeomagneticGrip: {
        },
        CDOTA_Modifier_Bristleback_Bristleback: {
        },
        CDOTA_Modifier_Brewmaster_DrunkenBrawler_Slow: {
        },
        CDOTA_Modifier_Silencer_GlaivesOfWisdom_BuffCounter: {
        },
        CDOTA_Modifier_Chen_HolyPersuasion: {
        },
        CDOTA_Modifier_Venomancer_NoxiousPlagueSecondary: {
        },
        CDOTA_Modifier_Crystal_Maiden_GlacialGuard: {
        },
        CDOTA_Modifier_Sven_Vanquisher: {
        },
        CDOTA_Modifier_Juggernaut_Duelist: {
        },
        CPulseCell_Timeline__TimelineEvent_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        C_DOTA_Unit_Hero_Earthshaker: {
        },
        C_DOTA_Item_Recipe_VoidwalkerScythe: {
        },
        CDOTA_Ability_Shadow_Demon_Demonic_Cleanse: {
        },
        C_DOTA_Ability_Furion_Teleportation: {
        },
        C_DOTA_Ability_TemplarAssassin_PsionicTrap: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Lina_2: {
        },
        CDOTA_Ability_AghsFort_Capture: {
        },
        CDOTA_Modifier_AghsFort_Creature_SpikedCarapace: {
        },
        C_DOTA_Ability_Seasonal_Throw_Snowball: {
        },
        C_DOTA_Ability_Special_Bonus_Cleave_20: {
        },
        CDOTA_Modifier_Item_Enhancement_Tough: {
        },
        CDOTA_Modifier_Item_Spark_Of_Courage: {
        },
        CDOTA_Modifier_Item_Occult_Bracelet_Stack: {
        },
        CDOTA_Modifier_Item_VoidwalkerScythe: {
        },
        CDOTA_Modifier_Kez_Echo_Slash_Slow: {
        },
        CDOTA_Modifier_Ringmaster_WhoopeeCushion_Debuff: {
        },
        CDOTA_Modifier_Snapfire_MortimerKisses: {
        },
        CDOTA_Modifier_Mars_Scepter_Damage_Slow: {
        },
        CDOTA_Modifier_Winter_Wyvern_Winters_Curse: {
        },
        CDOTA_Modifier_Winter_Wyvern_Arctic_Burn_Frost_Attack: {
        },
        CDOTA_Modifier_EarthSpirit_Polarization: {
        },
        CDOTA_Modifier_Tusk_WalrusKick_Slow: {
        },
        CDOTA_Modifier_LoneDruid_SpiritBear_Entangle_Effect: {
        },
        CDOTA_Modifier_Obsidian_Destroyer_ArcaneOrb: {
        },
        CDOTA_Modifier_Venomancer_PoisonSting: {
        },
        CDOTA_Modifier_Tinker_Warp_Grenade: {
        },
        CDOTA_Modifier_Tidehunter_Blubber: {
        },
        CDOTA_Modifier_Morphling_EbbAndFlow_Intrinsic: {
        },
        CDOTA_Modifier_Item_Mask_Spin_Crit_Consumed: {
        },
        CIngameEvent_10thAnniversary: {
        },
        CPulseCell_IntervalTimer__CursorState_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CPulseCell_BaseRequirement: {
        },
        C_DOTA_Item_Recipe_PhaseBoots: {
        },
        C_DOTA_Ability_Terrorblade_Terror_Wave: {
        },
        C_DOTA_Ability_Brewmaster_Cyclone: {
        },
        C_DOTA_Ability_Jakiro_Liquid_Fire: {
        },
        CDOTA_Ability_Huskar_Blood_Magic: {
        },
        C_DOTA_Ability_Warlock_Imp_Explode: {
        },
        C_DOTA_Ability_Tinker_Rearm: {
        },
        CDOTA_Ability_Axe_CounterHelix: {
        },
        C_DOTA_Ability_Seasonal_Summon_Snowman: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Medusa: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Zeus_6: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Pangolier_4: {
        },
        C_DOTA_Ability_Special_Bonus_Respawn_Reduction_50: {
        },
        C_DOTA_Ability_Special_Bonus_Strength_18: {
        },
        CDOTA_Modifier_Item_GaleGuard: {
        },
        CDOTA_Modifier_Item_Bottomless_Chalice_Regen: {
        },
        CDOTA_Modifier_ArcaneBoots_ManaRegen: {
        },
        CDOTA_Modifier_Gem_Active_TrueSight: {
        },
        CDOTA_Modifier_Visage_SummonFamiliars_Talents: {
        },
        CDOTA_Modifier_Treant_SuperBloom: {
        },
        CDOTA_Modifier_Brewmaster_LiquidCourage: {
        },
        CDOTA_Modifier_WitchDoctor_Maledict: {
        },
        CDOTA_Modifier_Mirana_SolarFlare: {
        },
        CDOTA_Modifier_SandKing_SandStorm_Slow_Aura_Thinker: {
        },
        CPulseCell_BaseState: {
        },
        OutflowWithRequirements_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        C_DOTA_Unit_Tidehunter_Anchor: {
        },
        CDOTA_Unit_AghsFort_Creature_DragonKnight: {
        },
        C_DOTA_Item_Phoenix_Ash: {
        },
        CDOTA_Item_Recipe_Aether_Lens: {
        },
        CDOTA_Ability_Necronomicon_Archer_Purge: {
        },
        C_DOTA_Item_GhostScepter: {
        },
        CDOTA_Ability_Courier_AutoDeliver: {
        },
        C_DOTA_Ability_Riki_Permanent_Invisibility: {
        },
        C_DOTA_Ability_Zuus_LightningBolt: {
        },
        CDOTA_Ability_Axe_CullingBlade: {
        },
        CDOTA_Modifier_Seasonal_TI9_Monkey_Thinker: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Outworld_Devourer_5: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_35: {
        },
        CDOTA_Modifier_EldwurmsEdda_Intrinsic: {
        },
        CDOTA_Modifier_Item_Enhancement_Vampiric: {
        },
        CDOTA_Modifier_Item_Enhancement_Quickened: {
        },
        CDOTA_Modifier_PrimalBeast_Onslaught_Windup: {
        },
        CDOTA_Modifier_Spectre_Haunt_Fear: {
        },
        CDOTA_Modifier_DarkSeer_WallOfReplica_Illusion: {
        },
        CDOTA_Modifier_Nian_Roar: {
        },
        CDOTA_Modifier_Tiny_Tree_Grab_Damage: {
        },
        CDOTA_Modifier_AncientApparition_ChillingTouch: {
        },
        CPulseCell_IsRequirementValid: {
        },
        C_InfoPlayerStartGoodGuys: {
        },
        CDOTA_Item_SerratedShiv: {
        },
        C_DOTA_Item_Crown: {
        },
        C_DOTA_Item_Black_King_Bar: {
        },
        C_DOTA_Item_Slippers: {
        },
        C_DOTA_Ability_KeeperOfTheLight_ManaLeak: {
        },
        C_SoundEventPathCornerEntity: {
        },
        C_InfoVisibilityBox: {
        },
        C_DotaQuestBase: {
        },
        CDOTA_Modifier_Aghsfort_Reward_MagicResistAura_Bonus: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Viper_2: {
        },
        CDOTA_Modifier_Special_Bonus_Cast_Speed: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Juggernaut_3: {
        },
        CDOTA_Modifier_Item_Harpoon_Pull: {
        },
        CDOTA_Modifier_Item_Heart: {
        },
        CDOTA_Modifier_Item_MysticStaff: {
        },
        CDOTA_Modifier_Techies_ReactiveTazer_Disarm: {
        },
        CDOTA_Modifier_Medusa_GorgonGrasp_Root: {
        },
        CDOTA_Modifier_Slark_DarkPact: {
        },
        CDOTA_Modifier_Necrolyte_Sadist_Counter: {
        },
        CDOTA_Modifier_StormSpirit_ElectricVortex_Pull: {
        },
        CDOTA_Modifier_StormSpirit_ElectricVortex_SelfSlow: {
        },
        CPulseCell_Value_Gradient: {
        },
        CDOTA_Item_Recipe_Harpoon: {
        },
        C_DOTA_Item_Orb_of_Pestilence: {
        },
        C_DOTA_Item_Essence_Distiller: {
        },
        C_DOTA_Item_Recipe_Spirit_Vessel: {
        },
        CDOTA_Ability_BlackDragon_DragonhideAura: {
        },
        CDOTA_Ability_GiantWolf_Intimidate: {
        },
        CDOTA_Modifier_BlackDrake_MagicAmplification_Aura: {
        },
        CDOTA_Modifier_Neutral_SpellImmunity: {
        },
        C_DOTA_Item_GreaterClarity: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Invoker_13: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Storm_Spirit_5: {
        },
        C_DOTA_Ability_Special_Bonus_Gold_Income_420: {
        },
        C_DOTA_Ability_Special_Bonus_Movement_Speed_35: {
        },
        CDOTA_Modifier_Item_Harpoon_InternalCD: {
        },
        CDOTA_Modifier_Item_Kaya_And_Sange: {
        },
        CDOTA_Buff_Wards: {
        },
        CDOTA_Modifier_Techies_Suicide_Leap: {
        },
        CDOTA_Modifier_SpiritBreaker_Bulldoze: {
        },
        CDOTA_Modifier_Spectre_Desolate: {
        },
        CDOTA_Modifier_Threads_Of_Fate: {
        },
        CDOTA_Modifier_DarkSeer_WallOfReplica: {
        },
        CDOTA_Modifier_Pugna_LifeDrain_SpellAmp: {
        },
        CDOTA_Modifier_Venomancer_Sepsis: {
        },
        CDOTA_Modifier_Tidehunter_LeviathansCatch_Stacks: {
        },
        CDOTA_Modifier_FountainAuraBuff: {
        },
        CDOTA_Modifier_CameraFollow: {
        },
        CDOTA_ItemStockInfo: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        IntervalTimer: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        audioparams_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_CrownfallShmupCamera: {
        },
        C_DOTA_Item_RiverPainter: {
        },
        C_DOTA_Ability_Jakiro_DualBreath: {
        },
        C_DOTA_Item_GrisGris: {
        },
        C_DOTA_Ability_Tinker_Laser: {
        },
        CDOTA_Ability_Morphling_Hybrid: {
        },
        CDOTA_Modifier_Neutral_Creep_Lost: {
        },
        CDOTA_Modifier_Radar_Thinker: {
        },
        C_PathParticleRope: {
        },
        C_DOTA_Ability_Special_Bonus_Agility_15: {
        },
        C_DOTA_Ability_Special_Bonus_Intelligence_8: {
        },
        CDOTA_Modifier_VoidSpirit_AstralStep_Debuff: {
        },
        CDOTA_Modifier_Special_Mars_Spear_Burning_Trail_Burn: {
        },
        CDOTA_Modifier_Winter_Wyvern_Eldwurm_Scholar: {
        },
        CDOTA_Modifier_CDOTA_Ability_Oracle_Clairvoyant_Cure: {
        },
        CDOTA_Modifier_Centaur_Stampede_Slow: {
        },
        CDOTA_Modifier_Medusa_ManaShield: {
        },
        CDOTA_Modifier_Visage_Stone_Form_Self_Cast_Cooldown_Manager: {
        },
        CDOTA_Modifier_LoneDruid_SpiritBear_TowerKillTracker: {
        },
        CDOTA_Modifier_IonShell_Slow: {
        },
        CDOTA_Modifier_TemplarAssassin_Trap: {
        },
        CDOTA_Modifier_DeathProphet_Witchcraft: {
        },
        CDOTA_Modifier_Tiny_TossTree_Bonus: {
        },
        CDOTA_Modifier_Morphling_ScepterStatsDrain_Agility_Debuff: {
        },
        CDOTA_Modifier_Axe_BatleHunger_Self_Movespeed: {
        },
        CIngameEvent_Spring2021: {
        },
        CDOTAPropArenaOfBloodWarrior: {
        },
        C_DOTA_RoshanSpawner: {
        },
        C_DOTA_Item_Diffusal_Blade: {
        },
        C_DOTA_Ability_Muerta_Ofrenda: {
        },
        C_DOTA_Ability_Pangolier_RollupStop: {
        },
        C_DOTA_Ability_Invoker_EMP: {
        },
        C_DOTA_Ability_CrystalMaiden_BrillianceAura: {
        },
        C_DOTA_Ability_DrowRanger_Marksmanship: {
        },
        CDOTA_Modifier_GraniteGolem_HPAura: {
        },
        CDOTA_Modifier_PineCone_AcornShot_TreeThinker: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Dark_Seer_3: {
        },
        CDOTA_Modifier_ConjuersCatalyst_Overheat: {
        },
        CDOTA_Modifier_Item_Desolator: {
        },
        CDOTA_Modifier_Item_OrchidMalevolence: {
        },
        CDOTA_Modifier_GhostScepter_Active: {
        },
        CDOTA_Modifier_Item_BootsOfTravel: {
        },
        CDOTA_Modifier_Largo_CatchyLick_Rune: {
        },
        CDOTA_Modifier_Rubick_Telekinesis_Stun: {
        },
        CDOTA_Modifier_Weaver_TimeLapse: {
        },
        CDOTA_Modifier_Turbo_Courier_Haste: {
        },
        CDOTA_Modifier_ShadowShaman_SerpentWardInvulnAura: {
        },
        CDOTA_Modifier_Morphling_Replicate_Manager: {
        },
        CDOTA_Modifier_DrowRanger_Arcana: {
        },
        C_DOTA_Item_Bullwhip: {
        },
        C_DOTA_Item_RingOfTarrasque: {
        },
        C_DOTA_Ability_Chen_Zealot: {
        },
        CDOTA_Ability_Venomancer_Snakebite: {
        },
        C_DOTA_Ability_CrystalMaiden_Frostbite: {
        },
        CDOTA_Ability_Seasonal_TI11_Balloon: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_DarkWillow_2: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Broodmother_2: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Riki_8: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Zeus_2: {
        },
        CDOTA_Modifier_Item_Possessed_Mask: {
        },
        CDOTA_Modifier_Item_PlaneswalkersCloak: {
        },
        CDOTA_Modifier_Item_RingOfRegeneration: {
        },
        CDOTA_Modifier_Muerta_DeadShot_Fear: {
        },
        CDOTA_Modifier_Snapfire_MortimerKisses_VisionSource: {
        },
        CDOTA_Modifier_FacelessVoid_Chronosphere_Speed: {
        },
        CDOTA_Modifier_QueenOfPain_Masochist: {
        },
        CDOTA_Modifier_Earthshaker_Echoslam_Debuff: {
        },
        CTimeline: {
        },
        CPulseCursorFuncs: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Item_Enhancement_Titanic: {
        },
        C_DOTA_Item_AsceticCap: {
        },
        CDOTA_Item_Recipe_Illusionsts_Cape: {
        },
        CDOTA_Item_Recipe_Mind_Breaker2: {
        },
        C_DOTA_Item_Recipe_UltimateScepter_2: {
        },
        C_DOTA_Ability_DoomBringer_Empty1: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Batrider_4: {
        },
        C_DOTA_Ability_Omniknight_Purification: {
        },
        C_DOTA_Ability_FacelessVoid_TimeDilation: {
        },
        CDOTA_Ability_VengefulSpirit_SoulStrike: {
        },
        C_DOTA_Ability_DrowRanger_Silence: {
        },
        C_DOTA_Ability_AntiMage_Puritan: {
        },
        CDOTA_Ability_AntiMage_Scepter: {
        },
        C_DOTA_Ability_Greevil_Miniboss_Purple_PlagueWard: {
        },
        CDOTA_Modifier_Tornado_ExpirationTime: {
        },
        CDOTA_Modifier_CentaurKhan_EnduranceAura_Bonus: {
        },
        C_TonemapController2: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Earth_Spirit_6: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Lone_Druid_4: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Range_150: {
        },
        C_DOTA_Ability_Special_Bonus_HP_Regen_35: {
        },
        CDOTA_Modifier_Item_Foragers_Mana: {
        },
        CDOTA_Modifier_Item_Swift_Blink_Buff: {
        },
        CDOTA_Modifier_Item_Assault_Cuirass_Positive: {
        },
        CDOTA_Modifier_Item_RefresherOrb: {
        },
        CDOTA_Modifier_Dawnbreaker_Celestial_Hammer_Movement: {
        },
        CDOTA_Modifier_Disruptor_Thunder_Strike_Speed: {
        },
        CDOTA_Modifier_Undying_FleshGolem: {
        },
        CDOTA_Modifier_Brewmaster_CinderBrew: {
        },
        CDOTA_Modifier_Rattletrap_RocketFlare_Slow: {
        },
        CDOTA_Modifier_FacelessVoid_Timelock_TimeWalk_Proc_Marker: {
        },
        CDOTA_Modifier_Miniboss_Alleviation: {
        },
        CDOTA_Modifier_Windrunner_UnfocusedFire: {
        },
        CDOTA_Modifier_SandKing_CausticFinaleOrb: {
        },
        CountdownTimer: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        PulseNodeDynamicOutflows_t__DynamicOutflow_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CDOTA_BaseNPC_Seasonal_TI11_Balloon_Dire: {
        },
        C_DOTA_Item_Enhancement_Fleetfooted: {
        },
        C_DOTA_Item_EagleEye: {
        },
        C_DOTA_Ability_Centaur_HoofStomp: {
        },
        C_DOTA_Ability_Rattletrap_Overclocking: {
        },
        CDOTA_Ability_Beastmaster_SummonRazorback: {
        },
        C_DOTA_Ability_Tidehunter_Gush: {
        },
        CDOTA_Ability_Axe_One_Man_Army: {
        },
        C_DOTA_Ability_Greevil_Miniboss_Black_BrainSap: {
        },
        C_DOTA_Ability_Special_Bonus_Strength_14: {
        },
        C_DOTA_Ability_Special_Bonus_MP_Regen_8: {
        },
        DataTeamPlayer_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Modifier_Item_Phoenix_Ash: {
        },
        CDOTA_Modifier_Item_RingOfAquila_Aura_Bonus: {
        },
        CDOTA_Modifier_Item_SuperBlinkDagger: {
        },
        CDOTA_Modifier_Item_Bracer: {
        },
        CDOTA_Modifier_PrimalBeast_Uproar_Roared_Self: {
        },
        CDOTA_Modifier_Dawnbreaker_Luminosity_Blaze_Buff: {
        },
        CDOTA_Modifier_Dawnbreaker_Celestial_Hammer_Thinker: {
        },
        CDOTA_Modifier_Winter_Wyvern_Winters_Curse_Aura: {
        },
        CDOTA_Modifier_Underlord_Portal_Fire_Effect: {
        },
        CDOTA_Modifier_Elder_Titan_AncestralSpirit_Buff: {
        },
        CDOTA_Modifier_Broodmother_IncapacitatingBite: {
        },
        CDOTA_Modifier_DeathProphet_SpiritSiphon_Debuff: {
        },
        CDOTA_Modifier_Sniper_Shrapnel_ChargeCounter: {
        },
        CDOTA_Modifier_Zuus_Static_Field: {
        },
        CIngameEvent_TI2022: {
        },
        C_DOTA_Item_Psychic_Headband: {
        },
        C_DOTA_Item_UltimateScepter: {
        },
        C_DOTA_Item_DustofAppearance: {
        },
        C_DOTA_Item_PlaneswalkersCloak: {
        },
        C_DOTA_Ability_Dawnbreaker_Converge: {
        },
        CDOTA_Ability_Mars_Bulwark: {
        },
        C_DOTA_Ability_LoneDruid_SpiritBear_Demolish: {
        },
        C_DOTA_Ability_Brewmaster_Primal_Split_Cancel: {
        },
        CDOTA_Ability_Dark_Seer_Quick_Wit: {
        },
        CDOTA_Ability_AncientApparition_IceBlast_Release: {
        },
        CDOTA_Ability_HillTroll_Rally: {
        },
        CDOTA_Modifier_Ghost_FrostAttack_Slow: {
        },
        CDOTA_Modifier_AghsFort_PoisonNova_Creature_Thinker: {
        },
        C_DOTA_Ability_Special_Bonus_Cleave_25: {
        },
        C_DOTABaseAbility: {
        },
        CDOTA_Modifier_Desolator_2_Corruption: {
        },
        CDOTA_Modifier_Item_Essence_Distiller_Vision: {
        },
        CDOTA_Modifier_Item_RingOfAquila: {
        },
        CDOTA_Modifier_EmberSpirit_SleightOfFist_Caster_EchoDamagePenalty: {
        },
        CDOTA_Modifier_NightStalker_HunterInTheNight: {
        },
        CDOTA_Modifier_Lich_Ice_Spire_Debuff: {
        },
        CBasePulseGraphInstance: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_DarkWillow_Creature: {
        },
        CDOTA_BaseNPC_Seasonal_TI11_Balloon: {
        },
        C_DOTA_Ability_KeeperOfTheLight_ChakraMagic: {
        },
        C_DOTA_Ability_ChaosKnight_Reality_Rift: {
        },
        CDOTA_Ability_Phantom_Assassin_Immaterial: {
        },
        C_DOTA_Ability_Courier_TakeStashAndTransferItems: {
        },
        C_DOTA_Ability_Slardar_Amplify_Damage: {
        },
        C_DOTA_Ability_DrowRanger_WaveOfSilence: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Hoodwink_SharpshooterPureDamage: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_65: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_10: {
        },
        CDOTA_Modifier_Huskar_Burning_Spear_Self: {
        },
        CDOTA_Modifier_Nian_GreaterBash: {
        },
        CDOTA_Modifier_Shadow_Shaman_Ally_Voodoo_Invuln: {
        },
        CDOTA_Modifier_Antimage_DampenMagic: {
        },
        FilterHealth: {
        },
        CDOTA_Item_Enhancement_Evolved: {
        },
        C_DOTA_Item_Recipe_Sphere: {
        },
        C_DOTA_Item_EmptyBottle: {
        },
        CDOTA_Ability_Obsidian_Destroyer_Ominous_Discernment: {
        },
        C_DOTA_Ability_TemplarAssassin_Refraction: {
        },
        C_DOTA_Ability_AntiMage_ManaVoid: {
        },
        CDOTA_Modifier_FrostbittenGolem_TimeWarpAura: {
        },
        C_PointClientUIHUD: {
        },
        C_DOTA_Ability_Aghsfort_Aziyog_Underlord_Portal_Warp: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Wisp_10: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Bloodseeker_5: {
        },
        CDOTA_Modifier_Largo_CatchyLick_Buff: {
        },
        CDOTA_Modifier_Marci_Unleash_FlurryPulse_Debuff: {
        },
        CDOTA_Modifier_Hoodwink_Decoy_Invisibility: {
        },
        CDOTA_Modifier_Visage_Silent_As_The_GravePassive: {
        },
        CDOTA_Modifier_Wisp_EssenceConduction: {
        },
        CDOTA_Modifier_Chen_DivineFavor_Aura: {
        },
        CDOTA_Modifier_NightStalker_Darkness: {
        },
        CDOTA_Modifier_Enchantress_Enchant: {
        },
        CDOTA_Modifier_Rattletrap_CogPush: {
        },
        CDOTA_Modifier_Enigma_Malefice: {
        },
        CDOTA_Modifier_Puck_PhaseShift: {
        },
        CPulseCell_Inflow_GraphHook: {
        },
        CAmbientCreatures: {
        },
        C_DOTA_Item_Trickster_Cloak: {
        },
        CDOTA_Item_Pirate_Hat: {
        },
        C_DOTA_Item_Nullifier: {
        },
        C_DOTA_Item_Heart: {
        },
        C_DOTA_Ability_Ringmaster_TameTheBeasts_Crack: {
        },
        CDOTA_Ability_Grimstroke_InkCreature: {
        },
        C_DOTA_Ability_Lycan_SummonWolves: {
        },
        CDOTA_Ability_SpiritBreaker_BullRush: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Batrider_7: {
        },
        C_DOTA_Ability_Sniper_KeenScope: {
        },
        C_DOTA_Ability_Windrunner_Windrun: {
        },
        CDOTA_Modifier_Filler_Tooltip: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_VoidSpirit_1: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lycan_7: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Phantom_Assassin_3: {
        },
        C_DOTA_Ability_Special_Bonus_MP_Regen_14: {
        },
        CDOTA_Modifier_Item_Bullwhip: {
        },
        CDOTA_Modifier_Item_Fluffy_Hat: {
        },
        CDOTA_Modifier_Item_UltimateOrb: {
        },
        CDOTA_Modifier_Ringmaster_WhoopeeCushion_Thinker: {
        },
        CDOTA_Modifier_DarkWillow_BrambleMaze: {
        },
        CDOTA_Modifier_Oracle_FatesEdict: {
        },
        CDOTA_Modifier_Nyx_Assassin_Burrow: {
        },
        CDOTA_Modifier_Disruptor_StaticStorm: {
        },
        CDOTA_Modifier_Undying_Tombstone_Bunker: {
        },
        CDOTA_Modifier_Brewmaster_DrunkenBrawler_BrewedUp: {
        },
        CDOTA_Modifier_DoomBringer_Doom_Aura_Self: {
        },
        CDOTA_Modifier_DoomBringer_Devour_Hero_Ability: {
        },
        CDOTA_Modifier_Broodmother_SpinWeb_Thinker: {
        },
        CDOTA_Modifier_Morphling_ScepterStatsDrain_Intelligence_Buff: {
        },
        CDOTA_Modifier_Skeleton_King_Arcana: {
        },
        CDOTA_Modifier_Invulnerable_Hidden: {
        },
        SignatureOutflow_Resume: {
        },
        C_DOTA_Item_PigletPole: {
        },
        CDOTA_Item_Dragon_Scale: {
        },
        C_DOTA_Item_Recipe_Armlet: {
        },
        C_DOTA_Ability_NightStalker_Darkness: {
        },
        C_DOTA_Ability_Tidehunter_AnchorSmash: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Wisp_8: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Razor_4: {
        },
        CDOTA_Modifier_Item_Necronomicon_Mana_Aura_2: {
        },
        CDOTA_Modifier_Grimstroke_InkCreature_Spawning: {
        },
        CDOTA_Modifier_Oracle_RainOfDestiny_Aura: {
        },
        CDOTA_Modifier_Elder_Titan_EarthSplitterScepter: {
        },
        CDOTA_Modifier_Shredder_Chakram_Disarm: {
        },
        CDOTA_Modifier_Treant_LivingArmor: {
        },
        CDOTA_Modifier_LoneDruid_SpiritBear_Demolish: {
        },
        CDOTA_Modifier_Lycan_Wolf_Bite_Lifesteal: {
        },
        CDOTA_Modifier_Broodmother_SpawnSpideriteDebuff: {
        },
        CDOTA_Modifier_Leshrac_Lightning_Storm_Scepter_Thinker: {
        },
        CDOTA_Modifier_Dazzle_Good_Juju: {
        },
        CDOTA_Modifier_Warlock_Rain_Of_Chaos_Death_Trigger: {
        },
        CDOTA_Modifier_Juggernaut_Healing_Ward_Aura: {
        },
        CDOTA_Modifier_Muerta_PartingShot_PhysicalBodyDebuff: {
        },
        CDOTA_Modifier_Fear: {
        },
        CPathSimpleAPI: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_InfoPlayerStartBadGuys: {
        },
        C_DOTA_Ability_QueenOfPain_Masochist: {
        },
        C_DOTA_Ability_Creep_Piercing: {
        },
        CDOTA_Modifier_Ghost_FrostAttack: {
        },
        CDOTA_Modifier_Outpost_Summoning: {
        },
        CDOTA_Modifier_Seasonal_TI11_CongaLineDancer: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Clockwerk_3: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Bloodseeker_2: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Furion_3: {
        },
        CDOTA_Modifier_Special_Bonus_MP: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Juggernaut_4: {
        },
        C_DOTA_Ability_Special_Bonus_Exp_Boost_50: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_18: {
        },
        C_DOTA_Ability_Special_Bonus_Agility_13: {
        },
        C_DOTA_Ability_Special_Bonus_Movement_Speed_Percentage_5: {
        },
        CDOTA_Modifier_FlayersBotaActive: {
        },
        CDOTA_Modifier_Item_Spellslinger: {
        },
        CDOTA_Modifier_Item_Mage_Slayer_Debuff: {
        },
        CDOTA_Modifier_EchoSabre_Debuff: {
        },
        CDOTA_Modifier_Largo_Groovin: {
        },
        CDOTA_Modifier_Muerta_OfrendaIntrinsic: {
        },
        CDOTA_Modifier_VoidSpirit_AetherRemnant_Pull: {
        },
        CDOTA_Modifier_ObliterateSoldier: {
        },
        CDOTA_Modifier_Elder_Titan_AncestralSpirit_Invincibility: {
        },
        CDOTA_Modifier_Undying_Tombstone_Zombie_Deathstrike_Slow_Counter: {
        },
        CDOTA_Modifier_Meepo_Earthbind: {
        },
        CDOTA_Modifier_Gyrocopter_Call_Down_Slow: {
        },
        CDOTA_Modifier_Jakiro_LiquidFire: {
        },
        CDOTA_Modifier_BountyHunter_WindWalk_Fade: {
        },
        CDOTA_Modifier_Warlock_Rain_Of_Chaos_Golem: {
        },
        CDOTA_Modifier_Slardar_Amplify_Damage: {
        },
        CDOTA_Modifier_Lich_FrostArmor_Slow: {
        },
        CDOTA_Modifier_PhantomLancer_SpiritLance: {
        },
        C_DOTA_Unit_Fountain: {
        },
        C_PointCommentaryNode: {
        },
        C_DOTA_Item_MadstoneBundle: {
        },
        C_DOTA_Item_Recipe_Necronomicon_2: {
        },
        C_DOTA_Item_Recipe_MagicWand: {
        },
        C_DOTA_Item_TranquilBoots2: {
        },
        CDOTA_Ability_Marci_Grapple: {
        },
        C_DOTA_Ability_Winter_Wyvern_Accelerated_Learning: {
        },
        CDOTA_Ability_Chaos_Knight_FundamentalForging: {
        },
        C_DOTA_Ability_Omniknight_Degen_Aura: {
        },
        CDOTA_Modifier_BlueDragonspawnOverseer_Evasion: {
        },
        CDOTA_Modifier_ForestTrollHighPriest_ManaAura_Bonus: {
        },
        C_DOTA_Ability_GiantWolf_CriticalStrike: {
        },
        CDOTA_Modifier_Neutral_Sleep_AI: {
        },
        CSpriteOriented: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Nyx: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Huskar_7: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_120: {
        },
        CDOTA_Modifier_Item_Grove_Bow_Debuff: {
        },
        CDOTA_Modifier_Item_Skadi: {
        },
        CDOTA_Modifier_Armlet_UnholyStrength: {
        },
        CDOTA_Modifier_Item_HelmOfIronWill: {
        },
        CDOTA_Modifier_Kez_SwitchWeapon_Flutter_Sai: {
        },
        CDOTA_Modifier_Hoodwink_Scurry_Active: {
        },
        CDOTA_Modifier_ArcWarden_MagneticField_Thinker_Attack_Speed: {
        },
        CDOTA_Modifier_Elder_Titan_NaturalOrder_MagicResistance: {
        },
        CDOTA_Modifier_Undying_Tombstone_Zombie_Deathlust: {
        },
        CDOTA_Modifier_Obsidian_Destroyer_Equilibrium_Buff_Counter: {
        },
        CDOTA_Modifier_Spectre_Desolate_Blind: {
        },
        CDOTA_Modifier_Clinkz_DeathPact_Permanent_Buff: {
        },
        CDOTA_Modifier_CallOfTheWild_Boar_BonusDamage: {
        },
        CDOTA_Modifier_Sniper_Headshot_Slow: {
        },
        C_DOTA_NPC_Treant_EyesInTheForest: {
        },
        CDOTA_Unit_Hero_AncientApparition: {
        },
        C_DOTA_Item_Avianas_Feather: {
        },
        C_DOTA_Item_Tenderizer: {
        },
        CDOTA_Ability_Marci_Unleash: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Batrider_5: {
        },
        C_DOTA_Ability_Earthshaker_EnchantTotem: {
        },
        CDOTA_Modifier_Aghsfort_Aziyog_Underlord_Firestorm_Thinker: {
        },
        CDOTA_Modifier_Seasonal_FestiveFirework: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Undying_2: {
        },
        C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_50: {
        },
        C_DOTA_Ability_Special_Bonus_Intelligence_15: {
        },
        CDOTA_Modifier_Item_LanceOfPursuit: {
        },
        CDOTA_Modifier_Item_Overwhelming_Blink_Debuff: {
        },
        CDOTA_Modifier_Item_Hood_Of_Defiance_Barrier: {
        },
        CDOTA_Modifier_Item_Black_King_Bar: {
        },
        CDOTA_Modifier_Legion_Commander_Outfight_Them_Buff: {
        },
        CDOTA_Modifier_Meepo_Fling_Slow: {
        },
        CDOTA_Modifier_Chaos_Knight_Reins_Of_Chaos: {
        },
        CDOTA_Modifier_Morphling_Morph_Str_Intrinsic: {
        },
        C_DOTA_Unit_Hero_DeathProphet: {
        },
        C_DOTA_Unit_Hero_ShadowShaman: {
        },
        C_DOTA_BaseNPC_Fort: {
        },
        C_DOTA_Ability_Necronomicon_Warrior_LastWill: {
        },
        C_DOTA_Ability_Legion_Commander_OverwhelmingOdds: {
        },
        C_DOTA_Ability_Invoker_DeafeningBlast: {
        },
        C_DOTA_Ability_Dark_Seer_Aggrandize: {
        },
        C_DOTA_Ability_Venomancer_PoisonNova: {
        },
        C_DOTA_Ability_Courier_DequeuePickupFromStash: {
        },
        C_PointClientUIWorldTextPanel: {
        },
        CDOTA_Modifier_Plus_HighFiveRequested: {
        },
        CDOTA_Modifier_AghsFort_Ascension_PlasmaField_Thinker: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Tinker_4: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Arc_Warden_3: {
        },
        C_DOTA_Ability_Special_Bonus_Gold_Income_90: {
        },
        CDOTA_Modifier_OgreSealTotem_Active: {
        },
        CDOTA_Modifier_Item_Consecrated_Wraps_Auto_Barrier: {
        },
        CDOTA_Modifier_Oracle_DivinersDeck_Death_Next: {
        },
        CDOTA_Modifier_Invoker_Tornado_TwisterDamage: {
        },
        CDOTA_Modifier_SpiritBreaker_GreaterBash_Break: {
        },
        CDOTA_Modifier_Tinker_Rearmor: {
        },
        CDOTA_Modifier_Tidehunter_DeadInTheWater: {
        },
        CDOTA_Modifier_Lich_FrostArmor_Autocast: {
        },
        CDOTA_Modifier_Axe_CounterHelix: {
        },
        C_DOTA_BaseNPC_Invoker_Forged_Spirit: {
        },
        CDOTAPropPlusPlayerGuildBanner: {
        },
        C_DOTA_BaseNPC_Creep_Lane: {
        },
        CDOTA_Item_Recipe_Ex_Machina: {
        },
        CDOTA_Item_RiverPainter5: {
        },
        C_DOTA_Item_UltimateScepter_Roshan: {
        },
        C_DOTA_Item_Tango: {
        },
        CDOTA_Ability_Ringmaster_StrongmanTonic: {
        },
        C_DOTA_Ability_Pudge_Rot: {
        },
        CDOTA_Modifier_FrostbittenGolem_TimeWarpAura_Bonus: {
        },
        CDOTA_Modifier_XP_Fountain_Aura: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Enigma_6: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Abaddon: {
        },
        C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_40: {
        },
        CDOTA_Modifier_Item_Witch_Blade_Slow: {
        },
        CDOTA_Modifier_Largo_FroglingBand2_Frogling: {
        },
        CDOTA_Modifier_Hoodwink_HuntersQuiver_Mark: {
        },
        CDOTA_Modifier_MonkeyKing_CloudRun: {
        },
        CDOTA_Modifier_Treant_NaturesGuise_Root: {
        },
        CDOTA_Modifier_Invoker_QuasInstance: {
        },
        CDOTA_Modifier_Weaver_GeminateAttack_Bonus: {
        },
        CDOTA_BountyHunter_Cutpurse: {
        },
        CDOTA_Modifier_Furion_Sprout_Tether_Aura: {
        },
        CDOTA_Modifier_Necrolyte_ReapersScythe_RespawnTime: {
        },
        CDOTA_Modifier_Bane_Nightmare: {
        },
        CDOTA_Modifier_Rune_Arcane: {
        },
        C_DOTA_Unit_Hero_SkeletonKing: {
        },
        C_DOTA_AncientDecorationProp: {
        },
        CDOTA_Item_Tombstone_Drop: {
        },
        C_DOTA_Item_Recipe_Ceremonial_Robe: {
        },
        C_DOTA_Item_DivineRapier: {
        },
        CDOTA_Ability_LegionCommander_Intimidate: {
        },
        CDOTA_Ability_Elder_Titan_MoveSpirit: {
        },
        C_DOTA_Ability_Courier_Burst: {
        },
        C_DOTA_Ability_Morphling_Replicate: {
        },
        CDOTA_Modifier_Aghsfort_Reward_ArmorAura: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Phantom_Lancer_5: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Bane_8: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Amplify_6: {
        },
        CDOTA_Modifier_Item_PyrrhicCloak: {
        },
        CDOTA_Modifier_Pangolier_GyroshellBounce: {
        },
        CDOTA_Modifier_Shredder_TimberChain: {
        },
        CDOTA_Modifier_KeeperOfTheLight_Illuminate: {
        },
        CDOTA_Modifier_Brewmaster_WindWalk: {
        },
        CDOTA_Modifier_Silencer_GlaivesOfWisdom_DebuffCounter: {
        },
        CDOTA_Modifier_DeathProphet_SpiritSiphon_ChargeCounter: {
        },
        CDOTA_Modifier_Tiny_Tree_Channel: {
        },
        CDOTA_Modifier_VengefulSpirit_WaveOfTerror_Fear: {
        },
        CDOTA_Modifier_AncientApparition_IceAge: {
        },
        C_DOTA_Unit_Hero_Tinker: {
        },
        C_World: {
        },
        C_DOTA_Item_Recipe_Penta_Edged_Sword: {
        },
        CDOTA_Item_Recipe_Orb_of_Pestilence: {
        },
        C_DOTA_Ability_ChaosKnight_Chaos_Bolt: {
        },
        C_DOTA_Ability_Brewmaster_DrunkenHaze: {
        },
        C_DOTA_Ability_Spectre_Reality: {
        },
        C_DOTA_Ability_Rattletrap_JetPack: {
        },
        C_DOTA_Ability_Pugna_LifeDrain: {
        },
        C_DOTA_Ability_Mirana_Arrow: {
        },
        CDOTA_Modifier_PineCone_ShieldBash_Slow: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Viper_5: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Range_300: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_252: {
        },
        CDOTA_Modifier_Item_Harmonizer: {
        },
        CDOTA_Modifier_Item_Enhancement_Timeless: {
        },
        CDOTA_Modifier_Item_Philosophers_Stone: {
        },
        CDOTA_Modifier_KeeperOfTheLight_BlindingLight: {
        },
        CDOTA_Modifier_Silencer_GlaivesOfWisdom_Buff: {
        },
        CDOTA_Modifier_SpiritBreaker_NetherStrike: {
        },
        CDOTA_Modifier_Huskar_Burning_Spear_Debuff: {
        },
        CDOTA_Modifier_Miniboss_Alleviation_Active_Aura: {
        },
        CDOTA_Modifier_Sven_Warcry_Aura: {
        },
        C_EconItemAttribute: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Item_Enhancement_Feverish: {
        },
        C_DOTA_Item_Repair_kit: {
        },
        C_DOTA_Item_Recipe_Phoenix_Ash: {
        },
        C_DOTA_Item_Recipe_Crellas_Crozier: {
        },
        C_DOTA_Ability_Ringmaster_TheBox: {
        },
        C_DOTA_Ability_Undying_TombstoneGrab: {
        },
        C_DOTA_Ability_Templar_Assassin_MeditationEnd: {
        },
        C_DOTA_Ability_Tiny_Rocksteady: {
        },
        CDOTA_Ability_Nevermore_Necromastery: {
        },
        CDOTA_Modifier_ARDM_NewHero: {
        },
        CDOTA_Modifier_Mutation_Cooldown_Reduction: {
        },
        CDOTA_Modifier_Mutation_Cooldown_Reduction_Team_Aura: {
        },
        C_DOTA_Aghsfort_Ability_Hoodwink_HuntersBoomerang: {
        },
        CDOTA_Modifier_Unwavering_Condition: {
        },
        CDOTA_Modifier_Item_Pirate_Hat: {
        },
        CDOTA_Modifier_Item_Bloodthorn: {
        },
        CDOTA_Modifier_MonkeyKing_Strike_Crit: {
        },
        CDOTA_Modifier_Abyssal_Underlord_Raid_Boss: {
        },
        CDOTA_Modifier_Visage_Lurker: {
        },
        CDOTA_Modifier_LoneDruid_SpiritBear_Entangle: {
        },
        CDOTA_Modifier_Furion_SpiritOfTheForest: {
        },
        CDOTA_Modifier_Plague_Wards_Bonus_Range: {
        },
        CDOTA_Modifier_ShadowShaman_Chicken_Speed: {
        },
        CDOTA_Modifier_Bloodseeker_Rupture_ChargeCounter: {
        },
        CDOTA_Modifier_Antimage_DampenMagic_Aura_Strong: {
        },
        CDOTA_Modifier_Creep_Slow: {
        },
        CPulseCell_Inflow_BaseEntrypoint: {
        },
        C_DOTA_Unit_Hero_Disruptor: {
        },
        C_DOTA_Item_Enhancement_Mystical: {
        },
        C_DOTA_Item_Recipe_Tenderizer: {
        },
        C_DOTA_Ability_Lycan_Wolf_Bite: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Batrider_2: {
        },
        C_DOTA_Ability_Sven_Warcry: {
        },
        CDOTA_Modifer_Furbolg_Enrage_Damage: {
        },
        C_DOTA_Ability_IceShaman_IncendiaryBomb: {
        },
        CDOTA_Ability_AghsFort_Dragon_Potion: {
        },
        CDOTA_Modifier_AghsFort_Ascension_PlasmaField_Slow: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Invoker_3: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Furion: {
        },
        CDOTA_Modifier_Ringmaster_TheBox_Visual: {
        },
        CDOTA_Modifier_DarkWillow_Debuff_Fear: {
        },
        CDOTA_Modifier_Terrorblade_Dark_Unity: {
        },
        CDOTA_Modifier_EmberSpirit_SleightOfFist_Caster: {
        },
        CDOTA_Modifier_Visage_SoulAssumption: {
        },
        CDOTA_Modifier_Meepo_Fling_Held_Caster: {
        },
        CDOTA_Modifier_Brewmaster_PrimalSplit_Scepter_UI: {
        },
        CDOTA_Modifier_Invoker_IceWall_AllyBuff: {
        },
        CDOTA_Modifier_SpiritBreaker_EmpoweringHasteAura: {
        },
        CDOTA_Modifier_Enigma_Splitting_Image: {
        },
        CDOTA_Modifier_Tiny_Tree_Grab: {
        },
        CDOTA_Modifier_Kunkka_GhostShip_Loaded: {
        },
        CDOTA_Modifier_Skeleton_King_Scepter_Tracker: {
        },
        CPulseCell_WaitForCursorsWithTagBase: {
        },
        C_DOTA_Unit_Hero_Oracle: {
        },
        C_DOTA_Item_Ethereal_Blade: {
        },
        C_DOTA_Item_UltimateOrb: {
        },
        C_DOTA_Ability_Enchantress_NaturesAttendants: {
        },
        C_DOTA_Ability_Omniknight_Innate_HealingHammer: {
        },
        C_DOTA_Ability_Tinker_Defensive_Matrix: {
        },
        C_DOTA_Ability_Pudge_GraftFlesh: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Luna_7: {
        },
        PingConfirmationState_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Modifier_Item_Lance_of_Pursuit_Slow: {
        },
        CDOTA_Modifier_Item_Spy_Gadget_Aura: {
        },
        CDOTA_Modifier_Item_Vambrace: {
        },
        CDOTA_Modifier_EmberSpirit_Immolation_Aura: {
        },
        CDOTA_Modifier_EmberSpirit_SleightOfFist_Marker: {
        },
        CDOTA_Modifier_Visage_GraveChill_Buff: {
        },
        CDOTA_Modifier_Ogre_Magi_DumbLuck: {
        },
        CDOTA_Modifier_Obsidian_Destroyer_Equilibrium: {
        },
        CDOTA_Modifier_Obsidian_Destroyer_StackingBonusMana: {
        },
        CDOTA_Modifier_Viper_BecomeUniversal: {
        },
        CDOTA_Modifier_CrystalMaiden_CrystalNova: {
        },
        CDOTA_Modifier_Mirana_MoonlightShadow_KillTracker: {
        },
        CDOTA_VR_Modifier_Statue_Mode: {
        },
        CDOTACustomShopItemInfo: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Item_EldwurmsEdda: {
        },
        CDOTA_Item_Minotaur_Horn: {
        },
        C_DOTA_Item_Recipe_Ironwood_tree: {
        },
        C_DOTA_Item_Aether_Lens: {
        },
        C_DOTA_Item_RingOfAquila: {
        },
        C_DOTA_Item_Soul_Booster: {
        },
        C_DOTA_Ability_Brewmaster_ThunderClap: {
        },
        CDOTA_Modifier_BlueDragonspawnOverseer_DevotionAura_Bonus: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Clinkz_9: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_250: {
        },
        C_DOTA_Ability_Special_Bonus_Armor_10: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Speed_40: {
        },
        CDOTA_Modifier_Item_Cloak_Of_Flames: {
        },
        CDOTA_Modifier_Item_WraithPact: {
        },
        CDOTA_Modifier_Manta_Phase: {
        },
        CDOTA_Modifier_Marci_Guardian_Buff: {
        },
        CDOTA_Modifier_Hoodwink_AcornShot_Slow: {
        },
        CDOTA_Modifier_AbyssalUnderlord_AtrophyAura_CreepDmgBuff: {
        },
        CDOTA_Modifier_EarthSpirit_Boulder_Smash_Debuff: {
        },
        CDOTA_Modifier_SpiritBreaker_GreaterBash_CascadingBashesTracker: {
        },
        CDOTA_Modifier_Life_Stealer_Infest_Creep: {
        },
        CDOTA_Modifier_Furion_WrathOfNature_BuffCounter: {
        },
        CDOTA_Modifier_PhantomAssassin_CoupdeGrace: {
        },
        CDOTA_Modifier_Roshan_RevengeRoar_Disarm: {
        },
        CDOTA_Modifier_Tinker_Rearm: {
        },
        CDOTA_Modifier_VengefulSpirit_Restitution_EnemyDebuff: {
        },
        C_fogplayerparams_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Item_Mysterious_Hat: {
        },
        CDOTA_Item_Ocean_Heart: {
        },
        C_DOTA_Ability_Ringmaster_Spotlight: {
        },
        C_DOTA_Ability_Disruptor_KineticField: {
        },
        C_DOTA_Ability_Chaos_Knight_Reins_Of_Chaos: {
        },
        C_DOTA_Ability_Morphling_Ebb: {
        },
        C_DOTA_Ability_Crystal_Maiden_Crystal_Clone: {
        },
        C_DOTA_Ability_Throw_Snowball: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Base_Damage_45: {
        },
        C_DOTA_Ability_Special_Bonus_Armor_2: {
        },
        CFoWBlockerRegion: {
        },
        CDOTA_Modifier_Item_UnstableWand: {
        },
        CDOTA_Modifier_Item_Repair_Kit: {
        },
        CDOTA_Modifier_Desolator_Corruption: {
        },
        CDOTA_Modifier_Item_Ring_Of_Basilius: {
        },
        CDOTA_Modifier_Largo_CroakOfGenius_Buff: {
        },
        CDOTA_Modifier_VoidSpirit_AstralStep_Intrinsic: {
        },
        CDOTA_Modifier_ArcWarden_SparkWraith_Purge: {
        },
        CDOTA_Modifier_NyxAssassin_NeuroSting_Debuff: {
        },
        CDOTA_Modifier_Undying_Decay_DebuffCounter: {
        },
        CDOTA_Modifier_Treant_LivingArmorPassive: {
        },
        CDOTA_Modifier_FacelessVoid_TimeLock: {
        },
        CDOTA_Modifier_Riki_TricksOfTheTrade_Phase: {
        },
        CDOTA_Modifier_PhantomLancer_Dopplewalk_Phase: {
        },
        CGameSceneNode: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Item_Mage_Slayer: {
        },
        C_DOTA_Item_Recipe_Fluffy_Hat: {
        },
        CDOTA_Item_Trusty_Shovel: {
        },
        C_DOTA_Item_Recipe_Bloodstone: {
        },
        CDOTA_Ability_Magnataur_ReversePolarity: {
        },
        C_DOTA_Ability_Furion_ForceOfNature: {
        },
        C_DOTA_Ability_Lina_LagunaBlade: {
        },
        CDOTA_Modifier_ForestTrollHighPriest_HealAutocast: {
        },
        CDOTA_Modifier_Effigy_AghsFort: {
        },
        C_DOTA_Ability_Special_Bonus_HP_700: {
        },
        CDOTA_Modifier_Item_BootsOfSpeed: {
        },
        CDOTA_Modifier_Muerta_PartingShot_Knockback: {
        },
        CDOTA_Modifier_Slark_DarkPact_Pulses: {
        },
        CDOTA_Modifier_Visage_Familiar_Innate: {
        },
        CDOTA_Modifier_Brewmaster_PrimalSplit_Scepter: {
        },
        CDOTA_Modifier_Enchantress_Untouchable: {
        },
        CDOTA_Modifier_Furion_Sprout_Healing_Aura: {
        },
        CDOTA_Modifier_Roshan_Grab: {
        },
        CDOTA_Modifier_Lion_Innate_ToHellAndBack: {
        },
        CDOTA_Modifier_Ursa_Bear_Down: {
        },
        CDOTA_Modifier_Earthshaker_Aftershock: {
        },
        CPlayer_ObserverServices: {
        },
        C_DOTA_Unit_Hero_Treant: {
        },
        C_DOTA_BaseNPC_Barracks: {
        },
        CDOTAPlayerPawn: {
        },
        CDOTA_Item_Greater_Mango: {
        },
        C_DOTA_Item_Recipe_Dagon: {
        },
        C_DOTA_Ability_Ringmaster_SummonUnicycle: {
        },
        CDOTA_Ability_Winter_Wyvern_Splinter_Blast: {
        },
        C_DOTA_Ability_Visage_GraveChill: {
        },
        C_DOTA_Ability_Weaver_TheSwarm: {
        },
        C_SoundAreaEntityBase: {
        },
        C_PlayerVisibility: {
        },
        CDOTA_Modifier_AghsFort_ShadowShaman_Shackles: {
        },
        CDOTA_Modifier_Special_Bonus_HP_Regen: {
        },
        C_DOTA_Ability_Special_Bonus_Movement_Speed_10: {
        },
        CDOTA_Modifier_Giant_Maul: {
        },
        CDOTA_Modifier_Item_Fallen_Sky_Burn: {
        },
        CDOTA_Modifier_Item_MeteorHammer_Land: {
        },
        CDOTA_Modifier_Item_Ancient_Janggo_Active: {
        },
        CDOTA_Modifier_Item_Blade_Mail: {
        },
        CDOTA_Modifier_Item_DivineRapier: {
        },
        CDOTA_Modifier_Item_NetherRaiment: {
        },
        CDOTA_Modifier_Kez_Sai: {
        },
        CDOTA_Modifier_Terrorblade_Arcana_Kill_Effect: {
        },
        CDOTA_Modifier_Keeper_of_the_Light_BrightSpeed: {
        },
        CDOTA_Modifier_Silencer_GlaivesOfWisdom: {
        },
        CDOTA_Modifier_Spectre_Dispersion_Boost: {
        },
        CDOTA_Modifier_Warlock_Upheaval: {
        },
        CAttributeManager__cached_attribute_float_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Unit_Hero_Hoodwink: {
        },
        C_DOTA_Unit_Hero_Wisp: {
        },
        C_DOTA_BaseNPC_Warlock_Golem: {
        },
        C_PortraitWorldUnit: {
        },
        C_BasePlayerWeapon: {
        },
        C_DOTA_Item_Orb_Of_Corrosion: {
        },
        CDOTA_Item_Ballista: {
        },
        C_DOTA_Item_HelmOfIronWill: {
        },
        C_DOTA_Ability_Hoodwink_HuntersBoomerang: {
        },
        CDOTA_Ability_Life_Stealer_Assimilate: {
        },
        C_DOTA_Ability_Viper_PoisonAttack: {
        },
        CDOTA_Ability_Roshan_InherentBuffs: {
        },
        CRagdollManager: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Keeper_of_the_Light_14: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Earth_Spirit_5: {
        },
        C_DOTA_Ability_Special_Bonus_Movement_Speed_90: {
        },
        C_DOTA_Ability_Special_Bonus_Movement_Speed_45: {
        },
        CDOTA_Modifier_Item_Disperser_Attack_Debuff: {
        },
        CDOTA_Modifier_Item_PointBooster: {
        },
        CDOTA_Modifier_Item_Circlet: {
        },
        CDOTA_Modifier_Marci_Bodyguarded: {
        },
        CDOTA_Modifier_Mars_ArenaOfBlood_Leash: {
        },
        CDOTA_Modifier_MonkeyKing_QuadrupleTap_Bonuses: {
        },
        CDOTA_Modifier_Invoker_Quas_Intrinsic_DoT: {
        },
        CDOTA_Modifier_Riki_Backstab: {
        },
        CDOTA_Modifier_Kunkka_Torrent_Slow: {
        },
        CDOTA_Modifier_Morphling_Morph_Agi: {
        },
        CDOTA_Modifier_Mirana_Leap_ChargeCounter: {
        },
        CDOTA_Modifier_AntiMage_ManaBreak: {
        },
        CDOTA_Modifier_Tower_Armor_Bonus: {
        },
        C_EnvSky: {
        },
        C_DOTA_Item_Rune: {
        },
        C_DOTA_Item_Enhancement_Greedy: {
        },
        C_DOTA_Item_Devastator: {
        },
        CDOTA_Ability_Hoodwink_Decoy: {
        },
        C_DOTA_Ability_Riki_SmokeScreen: {
        },
        C_DOTA_Ability_StormSpirit_Galvanized: {
        },
        C_DOTA_Ability_Juggernaut_Trinity: {
        },
        CDOTA_Modifier_731_Teaser_Stun: {
        },
        C_DotaSubquestEntityDeath: {
        },
        CDOTA_Modifier_Aghsfort_Walrus_Pudge_Harpoon_PathingFix: {
        },
        CDOTA_Modifier_ItemWiggle_Thinker: {
        },
        CDOTA_Ability_Seasonal_TI9_Banner: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lifestealer_3: {
        },
        CDOTA_Modifier_Special_Bonus_Day_Vision: {
        },
        C_DOTA_Ability_Special_Bonus_HP_Regen_7: {
        },
        CDOTA_Modifier_Item_Diffusal_Blade: {
        },
        CDOTA_Modifier_Hoodwink_Sharpshooter_Debuff: {
        },
        CDOTA_Modifier_Phoenix_Dying_Light_Debuff: {
        },
        CDOTA_Modifier_Slark_ShadowDance_Aura: {
        },
        CDOTA_Modifier_Invoker_DeafeningBlast_Disarm: {
        },
        CDOTA_Modifier_Life_Stealer_UnfetteredFury: {
        },
        CDOTA_Modifier_Drow_Ranger_Glaicer_Intrinsic: {
        },
        CIngameEvent_Frostivus2024: {
        },
        CPulse_InvokeBinding: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CDOTA_Item_Clumsy_Net: {
        },
        CDOTA_Item_Recipe_Solar_Crest: {
        },
        C_DOTA_Ability_Brewmaster_StormBrewling_StormStance: {
        },
        C_DOTA_Ability_Invoker_ColdSnap_AD: {
        },
        C_DOTA_Ability_Morphling_AdaptiveStrike_Agi: {
        },
        C_DOTA_Ability_Lina_LightStrikeArray: {
        },
        CDOTA_Modifier_MudGolem_CloakAura: {
        },
        C_DOTA_Ability_BlackDrake_MagicAmplification_Aura: {
        },
        CDOTA_Modifier_KoboldTaskmaster_SpeedAura: {
        },
        CDOTA_Modifier_Jungle_Varmint: {
        },
        C_EnvWindController: {
        },
        C_DOTA_Ability_AghsFort_Waveblaster_Leap: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Ember_Spirit_5: {
        },
        C_DOTA_Ability_Special_Bonus_Movement_Speed_Percentage_8: {
        },
        CDOTA_Modifier_Item_Dormant_CurioPermanent: {
        },
        CDOTA_Modifier_Item_Essence_Ring: {
        },
        CDOTA_Modifier_Nullifier: {
        },
        CDOTA_Modifier_Item_WindLace: {
        },
        CDOTA_Modifier_Item_OgreAxe: {
        },
        CDOTA_Modifier_Largo_Song_Speed_Burst: {
        },
        CDOTA_Modifier_Kez_RavensVeil_Buff: {
        },
        CDOTA_Modifier_Techies_StickyBomb_Slow_Secondary: {
        },
        CDOTA_Modifier_Slark_Depth_Shroud_Thinker: {
        },
        CDOTA_Modifier_Luna_LucentBeam_Damage_Buff_Counter: {
        },
        CDOTA_Modifier_Viper_Nethertoxin_Thinker: {
        },
        CDOTA_Modifier_PhantomAssassin_Fan_Of_Knives: {
        },
        CDOTA_Modifier_Tiny_Toss_Land_Debuff: {
        },
        CDOTA_Modifier_Morphling_Waveform_ChargeCounter: {
        },
        CDOTA_Modifier_DrowRanger_Trueshot: {
        },
        CDOTA_Modifier_Rooted: {
        },
        CDOTA_Modifier_Generic_Hidden: {
        },
        InventoryQuickBuyState_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_GameRules: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Item_Recipe_Consecrated_Wraps: {
        },
        C_DOTA_Item_Recipe_Buckler: {
        },
        CDOTA_Ability_Techies_RemoteMines: {
        },
        C_DOTA_Ability_Broodmother_WebWalk: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lion_6: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Vengeful_Spirit_5: {
        },
        C_DOTA_Ability_Special_Bonus_Movement_Speed_100: {
        },
        C_DOTAPortraitWorldCallbackHandler: {
        },
        CDOTA_Modifier_Hurricane_Pike_Active_Alternate: {
        },
        CDOTA_Modifier_Hoodwink_Hunters_Mark: {
        },
        CDOTA_Modifier_Oracle_FortunesEndChannelTarget: {
        },
        CDOTA_Modifier_Underlord_Portal_Fire: {
        },
        CDOTA_Modifier_Bristleback_Prickly: {
        },
        CDOTA_Modifier_Chen_HandOfGod_InvulnAura: {
        },
        CDOTA_Modifier_Warlock_Golem_Permanent_Immolation: {
        },
        CDOTA_Modifier_Ursa_Fury_Swipes: {
        },
        CDOTA_Modifier_Enigma_Innate_EventHorizon: {
        },
        CDOTA_Modifier_PhantomLancer_Juxtapose: {
        },
        CDOTA_Modifier_DrowRanger_FrostArrows_Hypothermia: {
        },
        CDOTA_Modifier_Activity_Modifier: {
        },
        C_DOTA_BaseNPC_Building: {
        },
        CDOTA_Item_Polliwog_Charm: {
        },
        C_DOTA_Item_Recipe_Gungir: {
        },
        C_DOTA_Item_Recipe_Yasha_And_Kaya: {
        },
        C_DOTA_Ability_Kez_SwitchWeapons: {
        },
        CDOTA_Ability_Meepo_Poof: {
        },
        C_DOTA_Ability_SpiritBreaker_NetherStrike: {
        },
        C_DOTA_Ability_Clinkz_Tar_Bomb: {
        },
        CDOTA_Item_Tidehunter_Fish: {
        },
        C_PointWorldText: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Chen_7: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Phantom_Assassin_5: {
        },
        C_DOTA_Ability_Special_Bonus_Gold_Income_60: {
        },
        C_DOTA_Ability_Special_Bonus_Agility_14: {
        },
        CDOTA_UI_ScenePanelAPI: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Modifier_Item_Shivas_Guard_Thinker: {
        },
        CDOTA_Modifier_Item_VoidStone: {
        },
        CDOTA_Modifier_Pangolier_Fortune_Favors_The_Bold_Effect: {
        },
        CDOTA_Modifier_EmberSpirit_FireRemnant_RemnantTracker: {
        },
        CDOTA_Modifier_Elder_Titan_Fundamental_Fury: {
        },
        CDOTA_Modifier_Nyx_Assassin_SpikedCarapace: {
        },
        CDOTA_Modifier_Luna_MoonGlaive_Shield: {
        },
        CDOTA_Modifier_TemplarAssassin_PsionicTrap_Counter: {
        },
        CDOTA_Modifier_Beastmaster_PrimalRoar_Slow: {
        },
        CDOTA_Modifier_Razor_Arcana: {
        },
        C_IngameEvent_TI9: {
        },
        CDOTASubChallengeInfo: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Item_Yasha: {
        },
        CDOTA_Ability_Pangolier_Fortune_Favors_The_Bold: {
        },
        C_DOTA_Ability_Wisp_Overcharge: {
        },
        CDOTA_Ability_Lycan_SummonWolves_Bash: {
        },
        C_DOTA_Ability_Greevil_Miniboss_Red_Earthshock: {
        },
        C_RopeKeyframe: {
        },
        C_BaseToggle: {
        },
        C_EnvCubemapBox: {
        },
        CDOTA_Modifier_AghsFort_Creature_Impale: {
        },
        CDOTA_Modifier_Special_Bonus_Attack_Base_Damage: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Dazzle_3: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Immunity: {
        },
        CDOTA_Modifier_AttributeBonus: {
        },
        CDOTA_Modifier_Item_Blood_Grenade_Flight_Thinker: {
        },
        CDOTA_Modifier_Pangolier_LuckyShot_Silence: {
        },
        CDOTA_Modifier_Techies_Spoons_Stash: {
        },
        CDOTA_Modifier_Elder_Titan_NaturalOrder_Aura_Armor: {
        },
        CDOTA_Modifier_Medusa_GorgonGrasp_Thinker: {
        },
        CDOTA_Modifier_Meepo_Poof_Damage_Sharing: {
        },
        CDOTA_Modifier_Chen_Divine_Favor_Teleport: {
        },
        CDOTA_Modifier_Chen_Penitence_Attack_Speed_Buff: {
        },
        CDOTA_Modifier_DarkSeer_Normal_Punch_Illusion_Thinker: {
        },
        CDOTA_Modifier_FacelessVoid_TimeLock_Freeze: {
        },
        CDOTA_Modifier_Axe_CullingBlade_Permanent: {
        },
        CDOTA_Modifier_Pudge_Meat_Hook_Followthrough: {
        },
        CDOTA_Modifier_HeroStatue: {
        },
        CDOTA_Item_Quicksilver_Amulet: {
        },
        C_DOTA_Item_Recipe_MonkeyKingBar: {
        },
        CDOTA_Item_ObserverWard: {
        },
        C_DOTA_Ability_Ringmaster_TameTheBeasts: {
        },
        C_DOTA_Ability_TrollWarlord_WhirlingAxes_Ranged: {
        },
        C_DOTA_Ability_Invoker_Alacrity: {
        },
        C_DOTA_Ability_Nian_Hurricane: {
        },
        CDOTA_Ability_Miniboss_Fortification: {
        },
        CDOTA_Ability_Courier_Morph: {
        },
        C_DOTA_Ability_DrowRanger_Multishot: {
        },
        CDOTA_Modifier_Greevil_Miniboss_Casting: {
        },
        CDOTA_Modifier_Kobold_Disarm: {
        },
        C_DOTA_Ability_VengefulSpirit_Command_Aura: {
        },
        CDOTA_Modifier_AghsFort_RockGolem_Avalanche: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Jakiro_3: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Lone_Druid_2: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_251: {
        },
        CDOTA_Ability_Special_Bonus_Corruption_25: {
        },
        C_DOTA_DataNonSpectator: {
        },
        CDOTA_Modifier_Item_GunpowderGauntlets: {
        },
        CDOTA_Modifier_Stormcrafter_Slow: {
        },
        CDOTA_Modifier_Item_Essence_Distiller_Damage: {
        },
        CDOTA_Modifier_Item_Necronomicon: {
        },
        CDOTA_Modifier_Abaddon_BorrowedTime_ImmolationAura: {
        },
        CDOTA_Modifier_Slark_SaltwaterShiv_Effect: {
        },
        CDOTA_Modifier_Undying_Decay_Shard: {
        },
        CDOTA_Modifier_Lycan_FeralImpulse: {
        },
        CDOTA_Modifier_Broodmother_StickySnare_Root: {
        },
        CDOTA_Modifier_QueenOfPain_SonicWave_Effect: {
        },
        CDOTA_Modifier_Tinker_Turret_Debuff: {
        },
        CDOTA_Modifier_Kunkka_Ghost_Ship_Fire_Cannons: {
        },
        CDOTA_Modifier_Razor_PlasmaField_Thinker: {
        },
        CDOTA_Modifier_Pudge_Meat_Hook_PathingFix: {
        },
        CDOTA_Modifier_Invisible: {
        },
        CDOTA_Modifier_BaseBlocker_Thinker: {
        },
        C_RopeKeyframe__CPhysicsDelegate: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CAdditionalWearable: {
        },
        CInfoDynamicShadowHint: {
        },
        CDOTA_Item_SuperOverwhelming_Blink: {
        },
        C_DOTA_Ability_Warlock_Golem_Flaming_Fists: {
        },
        C_DOTA_Ability_Nian_Leap: {
        },
        C_DOTA_Ability_Lion_ManaDrain: {
        },
        C_DOTA_Ability_Mirana_MoonlightShadow: {
        },
        CPathNode: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Wisp_6: {
        },
        C_DOTA_Ability_Special_Bonus_Movement_Speed_25: {
        },
        CDOTA_Modifier_Whisper_Of_The_Dread: {
        },
        CDOTA_Modifier_Woodland_Striders_Active: {
        },
        CDOTA_Modifier_Item_Sphere_Target: {
        },
        CDOTA_Modifier_Item_SacredRelic: {
        },
        CDOTA_Modifier_Legion_Commander_Outfight_Them_AuraMarker: {
        },
        CDOTA_Modifier_Elder_Titan_EchoStomp: {
        },
        CDOTA_Modifier_Shadow_Demon_Menace_Crushed: {
        },
        CDOTA_Modifier_Huskar_Burning_Spear_Counter: {
        },
        CDOTA_Modifier_DeathProphet_SpiritSiphon: {
        },
        CDOTA_Modifier_Zuus_ArcLightning: {
        },
        CDOTA_Modifier_Sven_GreatCleave: {
        },
        CDOTA_Unit_Announcer_Killing_Spree: {
        },
        C_DOTA_Unit_Hero_Sven: {
        },
        C_FuncMoveLinear: {
        },
        C_DOTA_Item_Pupils_gift: {
        },
        C_DOTA_Item_Recipe_Assault_Cuirass: {
        },
        C_DOTA_Item_SacredRelic: {
        },
        CDOTA_Ability_VoidSpirit_Variant_Equilibrium: {
        },
        C_DOTA_Ability_Terrorblade_ConjureImage: {
        },
        CDOTA_Ability_AbyssalUnderlord_AtrophyAura: {
        },
        C_DOTA_Ability_Rubick_TelekinesisLand: {
        },
        CDOTA_Ability_Furbolg_Enrage_AttackSpeed: {
        },
        C_DOTA_BinaryObject: {
        },
        CDOTA_Ability_AghsFort_Lifestealer_Enraged_Pulse: {
        },
        CDOTA_Ability_Aghsfort_Walrus_Pudge_Harpoon: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Mirana_6: {
        },
        C_DOTA_Ability_Special_Bonus_MP_175: {
        },
        TierNeutralInfo_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Modifier_Muerta_PierceTheVeil: {
        },
        CDOTA_Modifier_Oracle_FalsePromise: {
        },
        CDOTA_Modifier_Oracle_FortunesEnd_PurgeRepeatedly: {
        },
        CDOTA_Modifier_Abaddon_Withering_Mist: {
        },
        CDOTA_Modifier_Tusk_Tag_Team_Attack_Slow_Aura: {
        },
        CDOTA_Modifier_Invoker_Alacrity: {
        },
        CDOTA_Modifier_Chen_TestOfFaith_Teleport: {
        },
        CDOTA_Modifier_BountyHunter_Jinada: {
        },
        CDOTA_Modifier_Rattletrap_RocketFlare: {
        },
        CDOTA_Modifier_Item_Vermillion_Robe: {
        },
        CDOTA_Modifier_Nian_Leap: {
        },
        CDOTA_Modifier_Nevermore_Requiem_Aura: {
        },
        CDOTA_Modifier_Pudge_Swallow_Effect: {
        },
        CDOTA_Unit_Frogling_Event: {
        },
        CDOTA_Item_Foragers_Stats: {
        },
        C_DOTA_Item_Princes_Knife: {
        },
        CDOTA_Item_SentryWard: {
        },
        CDOTA_Item_BootsOfTravel_2: {
        },
        C_DOTA_Item_Circlet: {
        },
        CDOTA_Ability_VoidSpirit_Dissimilate: {
        },
        C_DOTA_Ability_Shredder_ReturnChakram: {
        },
        CDOTA_Ability_Disruptor_KineticFence: {
        },
        CDOTA_Ability_Treant_SuperBloom: {
        },
        C_DOTA_Ability_Viper_CorrosiveSkin: {
        },
        CDOTA_Item_Recipe_Vermillion_Robe: {
        },
        CDOTA_Modifier_ForestTrollHighPriest_HealAmp_Bonus: {
        },
        CDOTA_Modifier_GraniteGolem_HPAura_Bonus: {
        },
        CDOTA_Modifier_MudGolem_RockDestroy: {
        },
        CServerOnlyModelEntity: {
        },
        CDOTA_Ability_AghsFort_EchoSlamPotion: {
        },
        CDOTA_Modifier_AghsFort_Ascension_Invis_Warning: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Underlord_4: {
        },
        C_DOTA_Ability_Special_Bonus_Lifesteal_100: {
        },
        CDOTA_Modifier_DaggerOfRistul_Buff: {
        },
        CDOTA_Modifier_Item_SeedsOfSerenity_Active: {
        },
        CDOTA_Modifier_Item_Tree_Processor: {
        },
        CDOTA_Modifier_Item_Panic_Button: {
        },
        CDOTA_Modifier_Clumsy_Net_Ensnare: {
        },
        CDOTA_Modifier_Item_Vanguard: {
        },
        CDOTA_Modifier_Ward_RecentlySpawned: {
        },
        CDOTA_Modifier_Item_Claymore: {
        },
        CDOTA_Modifier_Marci_Unleash_Pulse_Silence: {
        },
        CDOTA_Modifier_Shredder_Chakram_Debuff_2: {
        },
        CDOTA_Modifier_Centaur_Innate_Rawhide: {
        },
        CDOTA_Modifier_TrollWarlord_Axe_Invulnerable: {
        },
        CDOTA_Modifier_Meepo_Geostrike_Debuff: {
        },
        CDOTA_Modifier_ForgedSpirit_MeltingStrike: {
        },
        CDOTA_Modifier_Batrider_Smoldering_Resin_Debuff: {
        },
        CDOTA_Modifier_Furion_OneWithNature: {
        },
        CDOTA_Modifier_Furion_Sprout_Blind: {
        },
        CDOTA_Modifier_Tidehunter_Ravage: {
        },
        CDOTA_Modifier_SandKing_Epicenter_Slow: {
        },
        CPulseCell_IntervalTimer: {
        },
        C_DOTA_Unit_Hero_Skywrath_Mage: {
        },
        C_DOTA_Item_Kobold_Cup: {
        },
        CDOTA_Item_Recipe_Hurricane_Pike: {
        },
        C_DOTA_Item_Urn_Of_Shadows: {
        },
        CDOTA_Item_SuperBlinkDagger: {
        },
        C_DOTA_Ability_EmberSpirit_SearingChains: {
        },
        C_DOTA_Ability_Ogre_Magi_Fireblast: {
        },
        C_DOTA_Ability_Silencer_BrainDrain: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Beastmaster_5: {
        },
        C_DOTA_Ability_Kunkka_Tidebringer: {
        },
        CDOTA_Modifier_Gold_Bag_Launch: {
        },
        C_SoundEventOBBEntity: {
        },
        CDOTA_Modifier_Aghsfort_Aziyog_Underlord_Portal_Warp_Channel_Soundstop: {
        },
        CDOTA_Modifier_Creature_Flamestrike: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Bane_5: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Axe_4: {
        },
        CDOTA_Modifier_Stacking_Exp_Rate_Boost: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Abaddon_3: {
        },
        C_DOTA_Ability_Special_Bonus_Gold_Income_120: {
        },
        C_DOTA_Hero_Recorder: {
        },
        CDOTA_Modifier_Item_HydrasBreath_Poison: {
        },
        CDOTA_Modifier_Muerta_PartingShot_SoulClone: {
        },
        CDOTA_Modifier_Hoodwink_Sharpshooter_Windup: {
        },
        CDOTA_Modifier_VoidSpirit_Dissimilate_Phase: {
        },
        CDOTA_Modifier_Abaddon_AphoticShield: {
        },
        CDOTA_Modifier_FacelessVoid_TimeWalk: {
        },
        CIngameEvent_MuertaReleaseSpring2023: {
        },
        CPulseTestScriptLib: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Ability_Special_Bonus_Unique_Hoodwink_AcornShot_ArmorCorruption: {
        },
        CDOTA_Ability_VoidSpirit_IntrinsicEdge: {
        },
        CDOTA_Ability_Slark_ShadowDance: {
        },
        C_DOTA_Ability_NightStalker_HunterInTheNight: {
        },
        C_DOTA_Ability_NightStalker_CripplingFear: {
        },
        C_DOTA_Ability_Clinkz_Infernal_Shred: {
        },
        C_DOTA_Ability_Lion_Impale: {
        },
        C_DOTA_Ability_AntiMage_Blink_Fake: {
        },
        CDOTA_Item_BookAgility: {
        },
        CDOTA_Modifier_Special_Bonus_Manaloss_Reduction: {
        },
        CDOTA_Ability_Special_Bonus_Spell_AoE_25: {
        },
        C_DOTA_Ability_Special_Bonus_MP_Regen_10: {
        },
        CDOTA_Modifier_Item_Soul_Booster: {
        },
        CDOTA_Modifier_Muerta_PartingShot_ProjectileReturn: {
        },
        CDOTA_Modifier_Marci_Unleash: {
        },
        CDOTA_Modifier_VoidSpirit_Variant_Equilibrium: {
        },
        CDOTA_Modifier_Pangolier_ShieldCrash_Buff: {
        },
        CDOTA_Modifier_MonkeyKing_QuadrupleTap: {
        },
        CDOTA_Modifier_Brewmaster_Void_Astral_Pull_Movement: {
        },
        CDOTA_Modifier_Brewmaster_Primal_Companion: {
        },
        CDOTA_Modifier_Huskar_Life_Break_Taunt: {
        },
        CDOTA_Modifier_Enchantress_Enchant_Slow: {
        },
        CDOTA_Modifier_Dazzle_Poison_Touch: {
        },
        CDOTA_Modifier_Kunkka_GhostShip_Knockback: {
        },
        CDOTA_Modifier_AncientApparition_ChillingTouch_Slow: {
        },
        CPulseCell_BaseLerp: {
        },
        C_DOTA_Unit_Brewmaster_PrimalStorm: {
        },
        C_DOTA_Unit_Hero_Beastmaster_Beasts: {
        },
        C_BasePropDoor: {
        },
        C_DOTAWorldParticleSystem: {
        },
        CDOTA_Item_Forage_Base: {
        },
        C_DOTA_Item_CripplingCrossbow: {
        },
        C_DOTA_Item_MagicStick: {
        },
        C_DOTA_Ability_Largo_AmphibianRhapsody_GoodVibrations: {
        },
        C_DOTA_Ability_Batrider_Stoked: {
        },
        C_DOTA_Ability_DragonKnight_Fireball: {
        },
        C_DOTA_Ability_Warlock_RainOfChaos: {
        },
        C_DOTA_Ability_Nian_Apocalypse: {
        },
        CDOTA_Modifier_Mutation_KillstreakPower: {
        },
        CDOTA_Ability_AghsFort_Ascension_PlasmaField: {
        },
        C_DOTA_Ability_Special_Bonus_Agility_16: {
        },
        C_DOTA_Ability_Special_Bonus_Agility_8: {
        },
        C_DOTA_Ability_Special_Bonus_MP_125: {
        },
        CPulseCell_ShmupWaitForDuration__CursorState_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CDOTA_Modifier_Item_DaggerOfRistul: {
        },
        CDOTA_Modifier_Item_Arcane_Ring: {
        },
        CDOTA_Modifier_ElixerHealing: {
        },
        CDOTA_Modifier_Item_Kaya: {
        },
        CDOTA_Modifier_Item_Orb_of_Venom_Slow: {
        },
        CDOTA_Modifier_Item_HelmOfTheDominator: {
        },
        CDOTA_Modifier_AbyssalUnderlord_DarkRift_Bonus_Health: {
        },
        CDOTA_Modifier_TrollWarlord_BerserkersRage_Maim: {
        },
        CDOTA_Modifier_Visage_Stone_Form_PreCastRoot: {
        },
        CDOTA_Modifier_KeeperOfTheLight_Recall_Movespeed: {
        },
        CDOTA_Modifier_Gyrocopter_Rocket_Barrage: {
        },
        CDOTA_Modifier_Weaver_Shukuchi: {
        },
        CDOTA_Modifier_Broodmother_WebWalk_Slow: {
        },
        CDOTA_Modifier_TemplarAssassin_RefractionDamage: {
        },
        CDOTA_Modifier_Special_Bonus_Unique_Beastmaster_5_Aura: {
        },
        CDOTA_Modifier_CallOfTheWild_Boar_PoisonBase: {
        },
        CDOTA_Modifier_Tidehunter_LeviathansCatch: {
        },
        CDOTA_Modifier_CrystalMaiden_Let_It_Go_Slow: {
        },
        CDOTA_Modifier_DisableTauntAnimationCancel: {
        },
        CDOTA_Modifier_Kill: {
        },
        C_DOTA_ArcanaDataEntity: {
        },
        C_DOTA_Ability_BotChallenge_SkeletonKing_BoneGuard: {
        },
        CDOTA_Item_Recipe_Enchanted_Quiver: {
        },
        CDOTA_Ability_Abaddon_Frostmourne: {
        },
        C_DOTA_Ability_TrollWarlord_Fervor: {
        },
        C_DOTA_Ability_LoneDruid_TrueForm: {
        },
        C_DOTA_Ability_Brewmaster_DispelMagic: {
        },
        C_DOTA_Ability_DoomBringer_InfernalBlade: {
        },
        CDOTA_Ability_AncientApparition_IceVortex: {
        },
        CDOTA_Modifier_OgreMagi_FrostArmor: {
        },
        CDOTA_Modifier_Aghsfort_Reward_HPAura_Bonus: {
        },
        CDOTA_Modifier_Creature_Full_Avoidance: {
        },
        CDOTA_Modifier_Corpselord_Revive: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lich_8: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Morphling_1: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Crystal_Maiden_6: {
        },
        C_DOTA_Ability_Special_Bonus_Cleave_30: {
        },
        C_DOTA_Ability_Special_Bonus_Gold_lvl10_l: {
        },
        DOTAThreatLevelInfo_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Modifier_Item_Flicker: {
        },
        CDOTA_Modifier_Item_Holy_Locket_Aura: {
        },
        CDOTA_Modifier_Item_Satanic_Unholy: {
        },
        CDOTA_Modifier_Item_Buckler: {
        },
        CDOTA_Modifier_Item_SheepStick: {
        },
        CDOTA_Modifier_Kez_ShodoSai_Mark_Slow: {
        },
        CDOTA_Modifier_Mars_ArenaOfBlood_Buff: {
        },
        CDOTA_Modifier_Terrorblade_Dark_Unity_Aura: {
        },
        CDOTA_Modifier_Abyssal_Underling_Archer_AoE_Aura: {
        },
        CDOTA_Modifier_Shredder_Reactive_Armor_Stack: {
        },
        CDOTA_Modifier_DoomBringer_Devour: {
        },
        CDOTA_Modifier_Night_Stalker_MidnightFeast: {
        },
        CDOTA_Modifier_Lich_FrostShield_Slow: {
        },
        CDOTA_Unit_Elder_Titan_AncestralSpirit: {
        },
        C_DOTA_Unit_Hero_KeeperOfTheLight: {
        },
        CDOTA_Item_Recipe_EchoSabre: {
        },
        CDOTA_Ability_Ringmaster_CrystalBall: {
        },
        C_DOTA_Ability_Slark_EssenceShift: {
        },
        C_DOTA_Ability_LoneDruid_SpiritBear_Return: {
        },
        CDOTA_Ability_DoomBringer_Lvl_Pain: {
        },
        C_DOTA_Ability_PhantomAssassin_Fan_Of_Knives: {
        },
        C_DOTA_Ability_QueenOfPain_Innate_Spell_Reflect: {
        },
        CDOTA_Ability_Beastmaster_DrumsOfSlom_Stop: {
        },
        C_DOTA_Ability_Courier_GoToSideShop2: {
        },
        CDOTA_Modifier_Mutation_Vampire: {
        },
        CChoreoInfoTarget: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Morphling_10: {
        },
        CDOTA_Modifier_Special_Bonus_Magic_Resistance: {
        },
        C_DOTA_Ability_Special_Bonus_Movement_Speed_Percentage_6: {
        },
        C_DOTACameraBounds: {
        },
        CDOTA_Modifier_Item_Caster_Rapier: {
        },
        CDOTA_Modifier_Item_Crimson_Guard_Extra: {
        },
        CDOTA_Modifier_Kez_Katana: {
        },
        CDOTA_Modifier_Abaddon_BorrowedTime_Passive: {
        },
        CDOTA_Modifier_TrollWarlord_SwitchStance: {
        },
        CDOTA_Modifier_Alchemist_Self_Corrosive_Weaponry: {
        },
        CDOTA_Modifier_SpiritBreaker_BullRush_Intrinsic: {
        },
        CDOTA_Modifier_FacelessVoid_TimeZone_Effect: {
        },
        CDOTA_Modifier_FacelessVoid_Backtrack_Temp: {
        },
        CTakeDamageResultAPI: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CNetworkedSequenceOperation: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Item_Chasm_Stone: {
        },
        C_DOTA_Ability_Muerta_PartingShot: {
        },
        C_DOTA_Ability_DarkWillow_BrambleMaze: {
        },
        C_DOTA_Ability_Invoker_GhostWalk: {
        },
        C_DOTA_Ability_ShadowShaman_Serpentine: {
        },
        C_DOTA_Ability_Tidehunter_Blubber: {
        },
        CDOTA_Modifier_AghsFort_RockGolem_Avalanche_Stun: {
        },
        CDOTA_Modifier_Aghsfort_Reward_CritAura_Bonus: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Crystal_Maiden_3: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Range_250: {
        },
        C_DOTA_Ability_Special_Bonus_All_Stats_7: {
        },
        CDOTA_DB_Page_StickerEntity: {
        },
        CDOTA_Modifier_Item_DefiantShell_Attack: {
        },
        CDOTA_Modifier_Item_Phylactery: {
        },
        CDOTA_Modifier_Item_Boots_Of_Bearing_Aura: {
        },
        CDOTA_Modifier_MonkeyKing_TreeJump_Hidden: {
        },
        CDOTA_Modifier_Weaver_Shukuchi_GeminateAttackManager: {
        },
        CDOTA_Modifier_Furion_Treant_HeroDamage: {
        },
        CDOTA_Modifier_Beastmaster_WildAxes: {
        },
        CDOTA_Modifier_Nian_EruptionPendingThinker: {
        },
        CDOTA_Modifier_Shadow_Shaman_Voodoo_Hands: {
        },
        CDOTA_Modifier_Windrunner_Powershot_Slow: {
        },
        CDOTA_Modifier_VengefulSpirit_Revenge_Tracker: {
        },
        CDOTA_Modifier_StormSpirit_StaticRemnant_Talent: {
        },
        CDOTA_Modifier_Bane_Enfeeble: {
        },
        CDOTA_Modifier_IceSlide: {
        },
        CDOTA_Modifier_Rune_FlyingHaste: {
        },
        C_DOTANewPlayerPoolGameMode: {
        },
        CEntityInstance: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Unit_Hero_Visage: {
        },
        C_DOTA_Unit_Hero_Venomancer: {
        },
        C_DOTA_Unit_Hero_Enigma: {
        },
        C_DOTA_Item_Nemesis_Curse: {
        },
        C_DOTA_Item_Recipe_Eternal_Shroud: {
        },
        CDOTA_Item_Recipe_Paladin_Sword: {
        },
        C_DOTA_Item_Elixer: {
        },
        C_DOTA_Item_Headdress: {
        },
        C_DOTA_Item_Recipe_TranquilBoots: {
        },
        CDOTA_Ability_Tusk_Tag_Team: {
        },
        C_DOTA_Ability_Brewmaster_WindWalk: {
        },
        C_DOTA_Ability_Gyrocopter_Mounting_Platform: {
        },
        C_DOTA_Ability_Lion_Voodoo: {
        },
        C_DOTA_Ability_WitchDoctor_VoodooRestoration: {
        },
        C_DOTA_Ability_ShadowShaman_Shackles: {
        },
        CDOTA_Modifier_XP_Fountain_Building: {
        },
        CDOTA_Ability_Seasonal_TI11_RockPaperScissors: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Bloodseeker_6: {
        },
        CDOTA_Modifier_Special_Bonus_Agility: {
        },
        C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_12: {
        },
        CDOTA_Modifier_Item_Ninja_Gear: {
        },
        CDOTA_Modifier_Kez_TalonToss_Damage: {
        },
        CDOTA_Modifier_Oracle_DivinersDeck_Intrinsic: {
        },
        CDOTA_Modifier_Phoenix_Sun_Debuff: {
        },
        CDOTA_Modifier_EmberSpirit_FlameGuard_Debuff: {
        },
        CDOTA_Modifier_Legion_Commander_Outfight_Them: {
        },
        CDOTA_Modifier_Huskar_Cauterize_Delay: {
        },
        CDOTA_Modifier_Warlock_Rain_Of_Chaos_Thinker: {
        },
        CDOTA_Modifier_Slardar_SeabornSentinel_Passive: {
        },
        CDOTA_Modifier_Sniper_Assassinate: {
        },
        CDOTA_Modifier_Earthshaker_Arcana: {
        },
        CDOTA_Modifier_Juggernaut_Omnislash: {
        },
        C_IngameEvent_TI8: {
        },
        C_DOTA_Ability_Grimstroke_Dark_Portrait: {
        },
        CDOTA_Ability_Magnataur_Horn_Toss: {
        },
        C_DOTA_Ability_Visage_SummonFamiliars_Recall: {
        },
        CDOTA_Ability_NagaSiren_Eelskin: {
        },
        C_DOTA_Ability_Windrunner_Tailwind: {
        },
        CDOTA_Modifier_BigThunderLizard_Wardrums_Aura: {
        },
        C_BaseModelEntity: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Nevermore_3: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_75: {
        },
        DOTATeleportInfo_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Modifier_Item_PyrrhicCloak_Target: {
        },
        CDOTA_Modifier_Item_Enchanted_Mango: {
        },
        CDOTA_Modifier_Primal_Beast_Trample_Haste: {
        },
        CDOTA_Modifier_Dawnbreaker_Fire_Wreath_Smash_Stun: {
        },
        CDOTA_Modifier_LegionCommander_DuelAura: {
        },
        CDOTA_Modifier_Medusa_StoneGaze_Stone: {
        },
        CDOTA_Modifier_Medusa_Mystic_Snake_Slow: {
        },
        CDOTA_Modifier_NagaSiren_Ensnare: {
        },
        CDOTA_Modifier_Nyx_Assassin_ManaBurn: {
        },
        CDOTA_Modifier_Disruptor_StaticStormThinker: {
        },
        CDOTA_Modifier_Shadow_Demon_Purge_Slow: {
        },
        CDOTA_Modifier_Life_Stealer_Feast: {
        },
        CDOTA_Modifier_PhantomAssassin_StiflingDagger_Caster: {
        },
        CDOTA_Modifier_Windrunner_FocusFire: {
        },
        C_SoundOpvarSetAutoRoomEntity: {
        },
        CDOTA_Item_Recipe_TranquilBoots2: {
        },
        CDOTA_Ability_Techies_SnareTrap: {
        },
        C_DOTA_Ability_Shredder_Flamethrower: {
        },
        CDOTA_Ability_LoneDruid_Entangle: {
        },
        C_DOTA_Ability_DoomBringer_Devour: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Batrider_6: {
        },
        CDOTA_Modifier_SatyrHellcaller_UnholyAura_Bonus: {
        },
        CDOTA_Modifier_Mutation_DeathExplosion_TeamAura: {
        },
        C_EnvCombinedLightProbeVolume: {
        },
        CDOTA_Modifier_Aghsfort_Elemental_Wisp_Tether_Slow: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Slark_7: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Slardar_7: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Antimage_2: {
        },
        CDOTA_Modifier_Special_Bonus_MP_Regen: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Amplify_7: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_400: {
        },
        CDOTA_Modifier_Item_Shivas_Guard_Blast: {
        },
        CDOTA_Modifier_Underlord_Portal_Warp_Channel: {
        },
        CDOTA_Modifier_Techies_LandMine_Burn: {
        },
        CDOTA_Modifier_Ogre_Magi_Arcana: {
        },
        CDOTA_Modifier_SpiritBreaker_BullsHit: {
        },
        CDOTA_Modifier_DeathProphet_SpiritSiphon_Slow: {
        },
        CDOTA_Modifier_Nian_Intrinsic: {
        },
        CDOTA_Modifier_Slardar_Sprint: {
        },
        CDOTA_Modifier_Pudge_Swallow: {
        },
        CDOTA_Modifier_Antimage_Persectur: {
        },
        C_MultiplayRules: {
        },
        CPlayer_AutoaimServices: {
        },
        C_DOTA_Item_DragonLance: {
        },
        C_DOTA_Ability_Muerta_PierceTheVeil: {
        },
        CDOTA_Ability_Abyssal_Underling_Warrior_ManaBurn: {
        },
        C_DOTA_Ability_Viper_Innate_Corrosive_Skin_Virulent: {
        },
        C_DOTA_Ability_ShadowShamanVoodoo: {
        },
        C_DOTA_Aghsfort_AbilityCrystalMaiden_FreezingField: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Elder_Titan: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Slark_6: {
        },
        CDOTA_Modifier_Special_Bonus_20_Bash: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Mirana_2: {
        },
        CDOTA_Ability_Special_Bonus_Spell_AoE_50: {
        },
        NagaSiren_SongOfTheSiren_Healing: {
        },
        CDOTA_Modifier_Chaos_Knight_Illusions_Damage_Reduction_Aura: {
        },
        CDOTA_Modifier_ForgedSpirit_Stats: {
        },
        CDOTA_Modifier_Rattletrap_RocketFlare_Overclock: {
        },
        CDOTA_Modifier_FacelessVoid_TimeZone_Projectile_Effect: {
        },
        CDOTA_Modifier_Ursa_Fury_Swipes_Damage_Increase: {
        },
        CDOTA_Modifier_Shadow_Shaman_Fowl_Play_Damage_Reduction: {
        },
        CDOTA_Modifier_Zuus_ThundergodsWrathThinker: {
        },
        CDOTA_Modifier_Drow_Ranger_Glacier_Shard: {
        },
        CDOTACourierController: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_LightDirectionalEntity: {
        },
        C_DOTA_Item_Giant_Maul: {
        },
        C_DOTA_Item_Recipe_Orb_Of_Corrosion: {
        },
        C_DOTA_Item_Recipe_Pipe: {
        },
        C_DOTA_Item_Recipe_LesserCritical: {
        },
        C_DOTA_Item_EnergyBooster: {
        },
        CDOTA_Modifier_Aghsfort_Wildwing_Tornado_Blast_Debuff: {
        },
        C_DOTA_Ability_Special_Bonus_Evasion_8: {
        },
        C_DOTA_Ability_Special_Bonus_Haste: {
        },
        CDOTA_Modifier_Item_Conjurers_Catalyst: {
        },
        CDOTA_Modifier_Item_InvisibilityEdge: {
        },
        CDOTA_Modifier_Largo_Encore: {
        },
        CDOTA_Modifier_Hoodwink_Camouflage: {
        },
        CDOTA_Modifier_Mars_ArenaOfBlood_Animation: {
        },
        CDOTA_Modifier_Meepo_MegaMeepo_Frame_Invulnerable: {
        },
        CDOTA_Modifier_LoneDruid_TrueForm_BattleCry: {
        },
        CDOTA_Modifier_LoneDruid_SpiritBear_AttackCheck: {
        },
        CDOTA_Modifier_Brewmaster_ThunderClap: {
        },
        CDOTA_Modifier_Invoker_DeafeningBlast_Knockback: {
        },
        CDOTA_Modifier_Spectre_Spectral_Dagger_Illusions: {
        },
        CDOTA_Modifier_Chen_Zealot_Buff: {
        },
        CDOTA_Modifier_Clinkz_Strafe_Debuff: {
        },
        CDOTA_Modifier_Earthshaker_Fissure_Thinker: {
        },
        CDOTA_Modifier_SkeletonKing_Reincarnation_SpawnSkeletons: {
        },
        CDOTA_Modifier_AncientApparition_IceVortex: {
        },
        CDOTACustomShopInfo: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_BaseEntity: {
        },
        ActiveModelConfig_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Item_Gossamer_Cape: {
        },
        CDOTA_Item_PogoStick: {
        },
        C_DOTA_Item_Recipe_Urn_Of_Shadows: {
        },
        C_DOTA_Ability_Pangolier_Swashbuckle: {
        },
        CDOTA_Ability_Abaddon_BorrowedTime: {
        },
        CDOTA_Ability_Warlock_Black_Grimoire: {
        },
        C_DOTA_Ability_Bane_Ichor_Of_Nyctasha: {
        },
        CDOTA_Item_Aghsfort_BootsOfTravel_2: {
        },
        CDOTA_Modifier_AghsFort_Firefly: {
        },
        CDOTA_Ability_Seasonal_Summon_CNY_Balloon: {
        },
        C_DOTA_Ability_Special_Bonus_Armor_15: {
        },
        C_DOTA_Ability_Special_Bonus_Strength_12: {
        },
        CDOTA_Modifier_Item_Greater_Faerie_Fire: {
        },
        CDOTA_Modifier_Item_MagicStick_Hero: {
        },
        CDOTA_Modifier_TangoHeal: {
        },
        CDOTA_Modifier_Muerta_PierceTheVeil_MagicImmunityDamageCancel: {
        },
        CDOTA_Modifier_Disruptor_Glimpse: {
        },
        CDOTA_Modifier_Undying_Tombstone_Bunker_Friendly: {
        },
        CDOTA_Modifier_Gyrocopter_Call_Down: {
        },
        CDOTA_Modifier_Rattletrap_Cog_Thinker_Talent: {
        },
        CDOTA_Modifier_Beastmaster_DrumsOfSlom_Aura: {
        },
        CDOTA_Modifier_NianCharge: {
        },
        CDOTA_Modifier_Roshan_RevengeRoar_Aura: {
        },
        CDOTA_Modifier_Roshan_Devotion: {
        },
        CDOTA_Modifier_WitchDoctor_Cask_Thinker: {
        },
        CDOTA_Modifier_FountainInvulnerabilityBuff: {
        },
        CPulseCell_Value_Curve: {
        },
        C_DOTA_Unit_SpiritBear: {
        },
        C_DOTA_Unit_Hero_AntiMage: {
        },
        C_BasePlayerPawn: {
        },
        C_DOTA_Item_Enhancement_Fierce: {
        },
        C_DOTA_Item_Recipe_Necronomicon: {
        },
        C_DOTA_Item_Recipe_PoorMansShield: {
        },
        C_DOTA_Ability_PrimalBeast_Onslaught: {
        },
        C_DOTA_Ability_Life_Stealer_Infest: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Visage_3: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Shadow_Shaman_1: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_200: {
        },
        C_DOTA_Ability_Special_Bonus_HP_Regen_10: {
        },
        CDOTA_Modifier_BotChallenge_SkeletonKing_BoneGuard_DamageTracker: {
        },
        CDOTA_Modifier_Silver_Edge_Debuff: {
        },
        CDOTA_Modifier_Ringmaster_WhoopeeCushion_Active: {
        },
        CDOTA_Modifier_Dawnbreaker_Solar_Guardian_Disable: {
        },
        CDOTA_Modifier_Hoodwink_AcornShot_BonusDamage: {
        },
        CDOTA_Modifier_Undying_CeaselessDirge_Buff: {
        },
        CDOTA_Modifier_Undying_Tombstone_Zombie_Deathstrike: {
        },
        CDOTA_Modifier_Meepo_Divided_We_Stand_SupportGroup: {
        },
        CDOTA_Modifier_Invoker_Tornado: {
        },
        CDOTA_Modifier_Spectre_Spectral: {
        },
        CDOTA_Modifier_DarkSeer_Surge: {
        },
        CDOTA_Modifier_Slardar_SeabornSentinel_River: {
        },
        CDOTA_Modifier_Sniper_Assassinate_Crit: {
        },
        CDOTA_Modifier_Enigma_MidnightPulse_Damage: {
        },
        CDOTA_Modifier_Bashed: {
        },
        C_SoundOpvarSetAABBEntity: {
        },
        C_DOTA_Unit_Aghsfort_Aziyog_Underlord_Portal: {
        },
        C_DOTA_Item_Foragers_Kit: {
        },
        C_DOTA_Item_Enhancement_Audacious: {
        },
        C_DOTA_Item_Recipe_Kaya: {
        },
        C_DOTA_Item_GemOfTrueSight: {
        },
        C_DOTA_Ability_Kez_RaptorDance: {
        },
        CDOTA_Ability_Elder_Titan_ReturnSpirit: {
        },
        C_DOTA_Ability_Rubick_SpellSteal: {
        },
        C_DOTA_Ability_Omniknight_Innate_Degen_Aura_Radius: {
        },
        CDOTA_Ability_Omniknight_Angelic_Flight: {
        },
        C_DOTA_Ability_Pugna_Decrepify: {
        },
        C_DOTA_Ability_ShadowShaman_EtherShock: {
        },
        C_DOTA_Ability_Mirana_Nightveil: {
        },
        C_DOTA_Ability_Bane_NightmareEnd: {
        },
        C_DOTA_Ability_BlackDragon_SplashAttack: {
        },
        CDOTA_Modifier_AghsFort_Morphling_Waveform: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Zeus_5: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_55: {
        },
        CDOTA_Modifier_Item_Craggy_Coat: {
        },
        CDOTA_Modifier_Item_CrellasCrozier_Movespeed_Buff: {
        },
        CDOTA_Modifier_Primalbeast_Trample_Attack_Bonus: {
        },
        CDOTA_Modifier_Spectre_SpectralDaggerPath: {
        },
        CDOTA_Modifier_Jakiro_LiquidIce_Debuff: {
        },
        CDOTA_Modifier_PhantomAssassin_Fan_Of_Knives_Thinker: {
        },
        CDOTA_Modifier_Nevermore_Requiem_Slow: {
        },
        CQuickBuyController: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Unit_Hero_Snapfire: {
        },
        C_DOTA_BaseNPC_Filler: {
        },
        C_DOTA_Item_Recipe_Bullwhip: {
        },
        C_DOTA_Item_Recipe_Book_Of_Shadows: {
        },
        C_DOTA_Item_Buckler: {
        },
        C_DOTA_Item_LesserCritical: {
        },
        C_DOTA_Item_Tango_Single: {
        },
        C_DOTA_Ability_Largo_CatchyLick: {
        },
        C_DOTA_Ability_ArcWarden_Scepter: {
        },
        C_DOTA_Ability_Silencer_Irrepressible: {
        },
        CDOTA_Ability_Alchemist_UnstableConcoctionThrow: {
        },
        C_DOTA_Ability_Enigma_Innate_EventHorizon: {
        },
        CDOTA_Ability_VengefulSpirit_Magic_Missile: {
        },
        CDOTA_Modifier_Greevil_Miniboss_Green_LivingArmor: {
        },
        C_DOTA_Ability_XP_Fountain: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Undying_7: {
        },
        CDOTA_Modifier_Special_Bonus_Spell_Lifesteal: {
        },
        CDOTA_Modifier_Special_Bonus_Cleave: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Omniknight_6: {
        },
        C_DOTA_Ability_Special_Bonus_Agility_20: {
        },
        C_DOTA_Ability_Special_Bonus_Strength_7: {
        },
        CDOTA_Modifier_Item_SeedsOfSerenity_Active_Aura: {
        },
        CDOTA_Modifier_Mars_ArenaOfBlood_BuffAura: {
        },
        CDOTA_Modifier_Abyssal_Underling_Warrior_Sight: {
        },
        CDOTA_Modifier_Medusa_Cold_Blooded: {
        },
        CDOTA_Modifier_Visage_Silent_As_The_Grave: {
        },
        CDOTA_Modifier_LoneDruid_SpiritLink: {
        },
        CDOTA_Modifier_Gyrocopter_Call_Down_Thinker_Tracking: {
        },
        CDOTA_Modifier_Chen_Divine_Favor_Armor: {
        },
        CDOTA_Modifier_TemplarAssassin_MeldArmor: {
        },
        CDOTA_Modifier_Lion_FingerOfDeath_Kill_Counter: {
        },
        CDOTA_Modifier_Kunkka_Tidebringer_ArmorPiercing: {
        },
        CDOTA_Modifier_AncientApparition_ChillingTouch_SuperSlow: {
        },
        CDOTA_Modifier_ChangeTreeModel_Thinker: {
        },
        CDOTA_Modifier_Bonus_Mres: {
        },
        C_DOTA_Item_Blitz_Knuckles: {
        },
        C_DOTA_Item_Recipe_Dagon2: {
        },
        C_DOTA_Item_Recipe_Cyclone: {
        },
        C_DOTA_Item_StaffOfWizardry: {
        },
        C_DOTA_Ability_EarthSpirit_BoulderSmash: {
        },
        C_DOTA_Ability_Shredder_TwistedChakram: {
        },
        C_DOTA_Ability_Wisp_Tether: {
        },
        C_DOTA_Ability_Meepo_Earthbind: {
        },
        CDOTA_Ability_Shadow_Demon_Shadow_Poison: {
        },
        C_DOTA_Ability_Batrider_FlamingLasso: {
        },
        C_DOTA_Ability_Lich_Ice_Spire: {
        },
        CEnvSoundscapeProxy: {
        },
        C_SoundEventEntity: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Tusk_4: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Troll_Warlord_4: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Centaur_4: {
        },
        CDOTA_Modifier_Item_Stonefeather_Satchel: {
        },
        CDOTA_Modifier_Item_Searing_Signet: {
        },
        CDOTA_Modifier_Muerta_TheCalling_Aura_HPRegen: {
        },
        CDOTA_Modifier_Techies_StickyBomb_Chase: {
        },
        CDOTA_Modifier_Undying_Tombstone_Zombie_Aura: {
        },
        CDOTA_Modifier_Weaver_Threads_Of_Fate_Established: {
        },
        CDOTA_Modifier_Huskar_Blood_Magic: {
        },
        CDOTA_Modifier_Furion_WrathOfNature_Thinker: {
        },
        CDOTA_Modifier_Nian_Torrent_Thinker: {
        },
        CDOTA_Modifier_Puck_Coil_Break_Stun: {
        },
        CDOTA_ModifierManager: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CPulseCell_Inflow_EventHandler: {
        },
        C_DOTA_Unit_IngisFatuus: {
        },
        C_DOTA_BaseNPC_XP_Fountain: {
        },
        C_LightOrthoEntity: {
        },
        C_DOTA_Item_Enhancement_Nimble: {
        },
        C_DOTA_Item_Recipe_MaskOfDispair: {
        },
        C_DOTA_Item_BeltOfStrength: {
        },
        C_DOTA_Ability_Invoker_DeafeningBlast_AD: {
        },
        C_DOTA_Ability_Spectre_ShadowStep: {
        },
        C_DOTA_Ability_Earthshaker_Slugger: {
        },
        C_DOTA_Ability_Sven_StormBolt: {
        },
        CDOTA_Ability_Frogmen_WaterBubble_Small: {
        },
        CDOTA_Modifier_Flagbearer_Creep_Aura_Effect: {
        },
        CDOTA_Modifier_WarpineRaider_SeedShot_Slow: {
        },
        CDOTA_Modifier_Mutation_DeathExplosion: {
        },
        CDOTA_Item_Book: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Jakiro_6: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Clockwerk: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Templar_Assassin: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Dark_Seer_13: {
        },
        CDOTA_Modifier_Special_Bonus_TrueStrike: {
        },
        C_DOTA_Ability_Special_Bonus_Strength_20: {
        },
        C_DOTA_Ability_Special_Bonus_Strength_10: {
        },
        C_DOTA_Ability_Special_Bonus_Mana_Reduction_8: {
        },
        CDOTA_Modifier_Item_SeedsOfSerenity: {
        },
        CDOTA_Modifier_Ringmaster_Spotlight_Debuff: {
        },
        CDOTA_Modifier_Tusk_Drinking_Buddies_Buff: {
        },
        CDOTA_Modifier_Tusk_Tag_Team_Slow: {
        },
        CDOTA_Modifier_Slark_EssenceShift_Permanent_Debuff: {
        },
        CDOTA_Modifier_Meepo_Innate_ToughChoices: {
        },
        CDOTA_Modifier_Broodmother_SpinWeb: {
        },
        CDOTA_Modifier_Warlock_Golem_Flaming_Fists: {
        },
        CDOTA_Modifier_WitchDoctor_Voodoo_Restoration_Aura: {
        },
        CDOTA_Modifier_Riki_BlinkStrike_Slow: {
        },
        CDOTA_Modifier_Morphling_EbbAndFlow: {
        },
        CDOTA_Modifier_Crystal_Maiden_Crystal_Clone_Statue: {
        },
        CDOTA_Modifier_SkeletonKing_Reincarnation_Slow: {
        },
        CPulseCell_BaseFlow: {
        },
        C_DOTA_Unit_Hero_Clinkz: {
        },
        CDOTA_Ability_Grimstroke_SoulChain: {
        },
        C_DOTA_Ability_Techies_MutuallyAssuredDestruction: {
        },
        C_DOTA_Ability_FacelessVoid_Backtrack: {
        },
        C_DOTA_Item_UpgradedMortar: {
        },
        C_DOTA_Ability_Special_Bonus_Magic_Resistance_6: {
        },
        C_DOTA_Item: {
        },
        CDOTA_Modifer_Item_WeightedDice: {
        },
        CDOTA_Modifier_Outworld_Staff: {
        },
        CDOTA_Modifier_Item_SerratedShiv: {
        },
        CDOTA_Modifier_Item_HydrasBreath_ProcDamage: {
        },
        CDOTA_Modifier_Muerta_PierceTheVeil_Buff: {
        },
        CDOTA_Modifier_Mars_ArenaOfBlood_Marker: {
        },
        CDOTA_Modifier_Grimstroke_SoulChain: {
        },
        CDOTA_Modifier_ArcWarden_SparkWraith_Thinker: {
        },
        CDOTA_Modifier_Skywrath_Mage_Ancient_Seal: {
        },
        CDOTA_Modifier_Undying_CeaselessDirge: {
        },
        CDOTA_Modifier_Ogre_Magi_Item_Multicast: {
        },
        CDOTA_Modifier_Batrider_Stoked: {
        },
        CDOTA_Modifier_QueenOfPain_Arcana_Kill_Effect: {
        },
        CDOTA_Modifier_Morphling_Syntropy_Slow: {
        },
        CDOTA_Modifier_DrowRanger_WaveOfSilence_Knockback: {
        },
        CDOTA_Modifier_StormSpirit_Overload_Passive: {
        },
        CDOTA_Modifier_Rune_DoubleDamage: {
        },
        CDOTA_Unit_Hero_Void_Spirit: {
        },
        C_DOTA_Unit_Hero_TrollWarlord: {
        },
        C_DOTA_Unit_Nian: {
        },
        CDOTA_Item_SuperArcane_Blink: {
        },
        C_DOTA_Item_MaskOfDispair: {
        },
        C_DOTA_Item_Blight_Stone: {
        },
        C_DOTA_Item_Claymore: {
        },
        CDOTA_Ability_VoidSpirit_AstralStep: {
        },
        C_DOTA_Ability_BlueDragonspawnSorcerer_Evasion: {
        },
        CDOTA_Ability_Spawnlord_Aura: {
        },
        CDOTA_Modifier_PineCone_AcornShot_BonusDamage: {
        },
        CDOTA_Ability_AghsFort_Spectre_ActiveDispersion: {
        },
        CDOTA_Ability_Ascension_AcidBlood: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Medusa_3: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Treant_7: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Luna_2: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Pudge_1: {
        },
        C_DOTA_Ability_Special_Bonus_Magic_Resistance_20: {
        },
        C_DOTA_Ability_Special_Bonus_MP_250: {
        },
        CDOTA_Modifier_Item_Pipe_Barrier: {
        },
        CDOTA_Modifier_Item_Battlefury: {
        },
        CDOTA_Modifier_Item_Shawl: {
        },
        CDOTA_Modifier_NyxAssassin_NeuroSting: {
        },
        CDOTA_Modifier_SpiritBreaker_ChargeOfDarkness: {
        },
        CDOTA_Modifier_DragonKnight_Fireball_Burn: {
        },
        CDOTA_Modifier_AntiMage_Empowered_ManaBreak_Debuff: {
        },
        CSkeletonInstance: {
        },
        CEntityComponent: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_BaseNPC_Watch_Tower: {
        },
        CDOTA_Item_Recipe_Iron_Talon: {
        },
        C_DOTA_Item_Gauntlets: {
        },
        C_DOTA_Ability_Ringmaster_WhoopeeCushion: {
        },
        CDOTA_Modifier_Furbolg_Enrage_Damage_OnDeath: {
        },
        CDOTA_Modifier_Mutation_DeathExplosion_Aura: {
        },
        C_DOTA_Ability_Aghsfort_Aziyog_Underlord_Dark_Portal: {
        },
        CDOTA_Modifier_Special_Bonus_Spell_AoE: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_150: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_25: {
        },
        C_DOTA_Ability_Special_Bonus_Magic_Resistance_100: {
        },
        CDOTA_Modifier_BottleRegeneration: {
        },
        CDOTA_Modifier_Hoodwink_Sharpshooter_Recoil: {
        },
        CDOTA_Modifier_EmberSpirit_FireRemnant_Timer: {
        },
        CDOTA_Modifier_Lycan_SummonWolves_Spirited: {
        },
        CDOTA_Modifier_Broodmother_IncapacitatingBiteOrb: {
        },
        CDOTA_Modifier_Beastmaster_Mark_Of_The_Beast: {
        },
        CDOTA_Modifier_Puck_Puckish: {
        },
        CDOTA_Modifier_PhantomLancer_IllusoryArmaments: {
        },
        CDOTA_Modifier_DrowRanger_Marksmanship_Aura_Bonus: {
        },
        CDOTA_Modifier_Oracle_Prognosticate: {
        },
        C_DOTA_PortraitEntity_FullBody: {
        },
        CDOTA_Item_Recipe_Nullifier: {
        },
        CDOTA_Item_Recipe_Octarine_Core: {
        },
        C_DOTA_Item_MantaStyle: {
        },
        C_DOTA_Ability_PrimalBeast_Pulverize: {
        },
        C_DOTA_Ability_Oracle_FalsePromise: {
        },
        C_DOTA_Ability_Slark_Fish_Bait: {
        },
        C_DOTA_Ability_Nyxth_Sense: {
        },
        C_DOTA_Ability_Treant_NaturesGuise: {
        },
        C_DOTA_Ability_Enigma_DemonicConversion: {
        },
        C_DOTA_Ability_ShadowShaman_MassSerpentWard: {
        },
        C_DOTA_Ability_StormSpirit_StaticRemnant: {
        },
        CDOTA_Modifier_KoboldTunneler_ProspectingAura_Money: {
        },
        CDOTA_Modifier_SatyrSoulstealer_ManaBurn: {
        },
        C_DOTA_Ability_EnragedWildkin_Tornado: {
        },
        CDOTA_Modifier_Lotus_Pool_Pickup: {
        },
        C_DOTA_Ability_Special_Bonus_Lifesteal_10: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_253: {
        },
        CDOTA_Modifier_Item_Consecrated_Wraps: {
        },
        CDOTA_Modifier_Silver_Edge_WindWalk: {
        },
        CDOTA_Modifier_Item_EnergyBooster: {
        },
        CDOTA_Modifier_Ringmaster_TheBox_Rescale: {
        },
        CDOTA_Modifier_Oracle_Clairvoyant_Curse: {
        },
        CDOTA_Modifier_WitchDoctor_DeathWard: {
        },
        CDOTA_Modifier_Riki_BlinkStrike_ChargeCounter: {
        },
        CDOTA_Modifier_Puck_PhaseShift_AttackBonus: {
        },
        CDOTA_Modifier_Kunkka_XMarksTheSpot_Thinker: {
        },
        CDOTA_Modifier_Lina_FierySoul: {
        },
        CPulseCell_Outflow_CycleShuffled__InstanceState_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CPulseCell_BaseLerp__CursorState_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CDOTA_Item_Woodland_Striders: {
        },
        C_DOTA_Item_DemonEdge: {
        },
        C_DOTA_Ability_Bear_Empty1: {
        },
        C_DOTA_Ability_LoneDruid_TrueForm_Druid: {
        },
        C_DOTA_Ability_QueenOfPain_ShadowStrike: {
        },
        C_DOTA_Ability_AncientApparition_Innate_Absolute_Zero: {
        },
        CDOTA_Modifier_Launchpad_Aura: {
        },
        CDOTA_Modifier_Aghsfort_Minor_Stats_Upgrade: {
        },
        C_DOTA_Ability_Throw_Coal: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Underlord: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_66: {
        },
        C_DOTA_Ability_Special_Bonus_Reincarnation_200: {
        },
        CDOTA_Modifier_Item_AsceticCap: {
        },
        CDOTA_Modifier_Item_Assault_Cuirass_Positive_Aura: {
        },
        CDOTA_Modifier_LoneDruid_TrueForm: {
        },
        CDOTA_Modifier_Invoker_ColdSnap_Freeze: {
        },
        CDOTA_Modifier_Furion_WrathOfNature_Spawn: {
        },
        CDOTA_Modifier_Dazzle_Shallow_Grave: {
        },
        CDOTA_Modifier_Dazzle_Rain_Of_Vermin: {
        },
        CDOTA_Modifier_QueenOfPain_ShadowStrike: {
        },
        CDOTA_Modifier_Special_Bonus_Unique_Beastmaster_6: {
        },
        CDOTA_Modifier_Warlock_EldritchSummoning: {
        },
        CDOTA_Modifier_Nian_Flag_Trap_Thinker: {
        },
        CDOTA_Modifier_Necrophos_Death_Seeker_Ethereal: {
        },
        CDOTA_Modifier_Riki_Poison_Dart_Debuff: {
        },
        CDOTA_Modifier_SkeletonKing_SpectralBladeIntrinsic: {
        },
        CDOTA_Modifier_SkeletonKing_HellfireBlast: {
        },
        C_DOTA_Unit_Hero_Muerta: {
        },
        C_DOTA_Unit_Hero_Viper: {
        },
        C_DOTA_BaseNPC_Tower: {
        },
        C_DOTA_Ability_Ringmaster_FunhouseMirror: {
        },
        C_DOTA_Ability_Wisp_Tether_Break: {
        },
        C_DOTA_Ability_SpiritBreaker_EmpoweringHaste: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Batrider_3: {
        },
        C_DOTA_Ability_VengefulSpirit_WaveOfTerror: {
        },
        C_DOTA_Ability_Sven_Wrath_Of_God: {
        },
        C_DOTA_Ability_AncientGolem_Rockslide: {
        },
        CDOTA_Modifier_DarkTrollWarlord_Ensnare: {
        },
        CDOTA_Modifier_PudgeMiniboss_ArmorCorruptionDebuff: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Hoodwink_SharpshooterPierceHeroes: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Slardar: {
        },
        C_DOTA_Ability_Special_Bonus_Magic_Resistance_5: {
        },
        C_DotaTutorialNetworker: {
        },
        CDOTA_Modifier_Item_SpecialistsArray: {
        },
        CDOTA_Modifier_Item_Witless_shako: {
        },
        CDOTA_Modifier_Item_Royal_Jelly: {
        },
        CDOTA_Modifier_ClarityPotion: {
        },
        CDOTA_Modifier_Hoodwink_HuntersQuiver: {
        },
        CDOTA_Modifier_Underlord_Portal_FX: {
        },
        CDOTA_Modifier_Leshrac_Split_Earth_Thinker: {
        },
        CDOTA_Modifier_Drow_Ranger_Glacier_Hilltop_Aura: {
        },
        CPulseAnimFuncs: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Item_Satanic: {
        },
        C_DOTA_Item_GreaterCritical: {
        },
        CDOTA_Ability_Miniboss_Radiance: {
        },
        CDOTA_Modifier_Greevil_Miniboss_Yellow_Surge: {
        },
        CDOTA_Modifier_IceShaman_IncendiaryBomb: {
        },
        C_BaseClientUIEntity: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Pudge_3: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Omniknight_3: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Ancient_Apparition_3: {
        },
        C_DOTA_Ability_Special_Bonus_Movement_Speed_Percentage_12: {
        },
        CDOTA_Modifier_Penta_Edged_Sword_Maim: {
        },
        CDOTA_Modifier_Pogo_Stick_Active: {
        },
        CDOTA_Modifier_Item_Ironwood_tree: {
        },
        CDOTA_Modifier_Item_Skadi_Slow: {
        },
        CDOTA_Modifier_Item_Bloodstone_Drained: {
        },
        CDOTA_Modifier_Item_TalismanOfEvasion: {
        },
        CDOTA_Modifier_Item_StoutShield: {
        },
        CDOTA_Modifier_Item_VitalityBooster: {
        },
        CDOTA_Modifier_Tusk_Tag_Team: {
        },
        CDOTA_Modifier_Bristleback_QuillSpray_Thinker: {
        },
        CDOTA_Modifier_Wisp_Tether_Slow: {
        },
        CDOTA_Modifier_Disruptor_StaticStormMute: {
        },
        CDOTA_Modifier_Dazzle_Weave_Armor: {
        },
        CDOTA_Modifier_PhantomAssassin_Blur: {
        },
        CDOTA_Modifier_Tiny_Avalanche: {
        },
        CDOTA_Modifier_NoHealthBar: {
        },
        CPulseCell_WaitForCursorsWithTagBase__CursorState_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CPulseArraylib: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_TriggerLerpObject: {
        },
        C_DOTA_Item_Phylactery: {
        },
        CDOTA_Ability_Wisp_Equilibrium: {
        },
        CDOTA_Ability_LoneDruid_Innate_GiftBearer: {
        },
        C_DOTA_Ability_LoneDruid_SavageRoar_Bear: {
        },
        CDOTA_Ability_Bloodseeker_Bloodbath: {
        },
        CDOTA_Ability_Plus_HighFive: {
        },
        CDOTA_Item_Aghsfort_BootsOfTravel: {
        },
        CDOTA_Modifier_Aghsfort_AggroOverride: {
        },
        CDOTA_Ability_AghsFort_Ascension_Silence: {
        },
        CDOTA_Modifier_Aghsfort_TempBuff_CorpseExplosion_Aura: {
        },
        CDOTA_Modifier_Item_Falcon_Blade: {
        },
        CDOTA_Modifier_Item_Octarine_Core: {
        },
        CDOTA_Modifier_Item_OblivionStaff: {
        },
        CDOTA_Modifier_Largo_FroglingBand2_Frogling_Vanish: {
        },
        CDOTA_Modifier_Mars_Spear_Stun: {
        },
        CDOTA_Modifier_Pangolier_LuckyShot_Disarm: {
        },
        CDOTA_Modifier_Winter_Wyvern_Splinter_Blast_Slow: {
        },
        CDOTA_Modifier_Terrorblade_Reflection_Slow: {
        },
        CDOTA_Modifier_Slark_ShadowDance: {
        },
        CDOTA_Modifier_Batrider_Firefly: {
        },
        CDOTA_Modifier_Pugna_Oblivion_Savant: {
        },
        CDOTA_Modifier_Beastmaster_Summon_Raptor: {
        },
        CDOTA_Modifier_VengefulSpirit_Command_Aura: {
        },
        CDOTA_Modifier_Sand_King_Scorpion_Strike_Attack_Bonus: {
        },
        CDOTA_Modifier_Bane_Ichor_Of_Nyctasha: {
        },
        CDOTA_Modifier_AntiMage_Thirst_Effect: {
        },
        CDOTA_Modifier_Disarmed: {
        },
        CPointTemplateAPI: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Unit_Hero_Tidehunter: {
        },
        C_DOTA_Unit_Hero_SandKing: {
        },
        C_DynamicPropAlias_cable_dynamic: {
        },
        CBaseProp: {
        },
        CDOTA_Item_Recipe_Apex: {
        },
        CDOTA_Item_Ninja_Gear: {
        },
        CDOTA_Item_Recipe_Trusty_Shovel: {
        },
        C_DOTA_Ability_Hoodwink_Sharpshooter_Release: {
        },
        CDOTA_Ability_VoidSpirit_ResonantPulse: {
        },
        CDOTA_Ability_Mars_Immovable: {
        },
        C_DOTA_Ability_Phoenix_BlindingSun: {
        },
        CDOTA_Ability_Treant_EyesInTheForest: {
        },
        C_DOTA_Ability_Rattletrap_JetPack_Toggle: {
        },
        C_DOTA_Ability_Enigma_Event_Horizon: {
        },
        CDOTA_Modifier_MonsterHunterEvent: {
        },
        CDOTA_Ability_BigThunderLizard_Wardrums: {
        },
        CDOTA_Modifier_WarpineRaider_SeedShot: {
        },
        CInfoOffscreenPanoramaTexture: {
        },
        CDOTA_Modifier_AghsFort_Spectre_ActiveDispersion_Thinker: {
        },
        CDOTA_Modifier_AghsFort_Ravage_Potion: {
        },
        C_DOTA_Ability_Special_Bonus_Exp_Boost_60: {
        },
        C_DOTA_Ability_Special_Bonus_Mana_Break_40: {
        },
        C_DOTA_Ability_Special_Bonus_HP_250: {
        },
        CDOTA_Modifier_Item_Force_Field: {
        },
        CDOTA_Modifier_Muerta_Revenant_HealingThinker: {
        },
        CDOTA_Modifier_VoidSpirit_Dissimilate_Invis: {
        },
        CDOTA_Modifier_Abyssal_Underling_Warrior_LastWill: {
        },
        CDOTA_Modifier_Techies_Minefield_Sign_Aura: {
        },
        CDOTA_Modifier_Elder_Titan_AncestralSpirit_CastTime: {
        },
        CDOTA_Modifier_Treant_NaturesGuise_Tree_Walking: {
        },
        CDOTA_Modifier_Spectre_SpectralDaggerPath_ActivityModifierOnly: {
        },
        CDOTA_Dazzle_NothlProjection_PhysicalBodyDebuff: {
        },
        CDOTA_Modifier_FacelessVoid_Arcana_Kill_Effect: {
        },
        CDOTA_Modifier_Teleporting: {
        },
        C_DOTA_Unit_Hero_DrowRanger: {
        },
        C_DOTA_Item_DuelistGloves: {
        },
        C_DOTA_Item_Recipe_Sorcerers_Staff: {
        },
        C_DOTA_Item_Blood_Grenade: {
        },
        C_DOTA_Ability_Hoodwink_Caltrops: {
        },
        C_DOTA_Ability_Phoenix_LaunchFireSpirit: {
        },
        C_DOTA_Ability_Kunkka_XMarksTheSpot: {
        },
        CDOTA_Modifier_AlphaWolf_CommandAura_Bonus: {
        },
        C_DOTA_Ability_Aghsfort_Elemental_Wisp_Tether: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Kunkka_2: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Crystal_Maiden_5: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Furion_4: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Axe_6: {
        },
        C_DOTA_Ability_Special_Bonus_Gold_Income_180: {
        },
        CDOTA_Modifier_Marci_Unleash_Flurry: {
        },
        CDOTA_Modifier_VoidSpirit_IntrinsicEdge: {
        },
        CDOTA_Modifier_NagaSiren_SongOfTheSiren_Healing_Aura: {
        },
        CDOTA_Modifier_Rubick_FadeBoltDebuff: {
        },
        CDOTA_Modifier_Meepo_Poof_Casting: {
        },
        CDOTA_Modifier_Weaver_Swarm: {
        },
        CDOTA_Modifier_Windrunner_Arcana_Kill_Effect: {
        },
        CDOTA_Modifier_Kunkka_GhostShip_DamageAbsorb: {
        },
        CDOTA_Modifier_StormSpirit_Enemy_Overload: {
        },
        CDOTA_Modifier_Lua_Vertical_Motion: {
        },
        CDOTA_Modifier_PreventTaunts: {
        },
        C_BodyComponentBaseAnimating: {
        },
        CAttributeManager: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        SignatureOutflow_Continue: {
        },
        C_DOTA_NPC_Techies_Minefield_Sign: {
        },
        C_DOTA_Unit_Hero_Luna: {
        },
        C_DOTA_NPC_WitchDoctor_Ward: {
        },
        C_Artillery_PortraitWorldUnit: {
        },
        C_DOTA_Tiny_ScepterTree: {
        },
        C_DOTA_Item_Physical: {
        },
        C_DOTA_Item_Eaglehorn: {
        },
        C_DOTA_Item_BootsOfElven: {
        },
        C_DOTA_Ability_Muerta_OfrendaDestroy: {
        },
        CDOTA_Ability_Snapfire_SpitCreep: {
        },
        C_DOTA_Ability_Techies_Squees_Scope: {
        },
        CDOTA_Ability_Centaur_Mount: {
        },
        C_DOTA_Ability_Meepo_Innate_ToughChoices: {
        },
        C_DOTA_Ability_Tidehunter_KrakenShell: {
        },
        C_DOTA_Ability_Crystal_Maiden_GlacialGuard: {
        },
        CDOTA_Modifier_BigThunderLizard_Slam: {
        },
        CInfoTarget: {
        },
        C_DOTA_Ability_Special_Bonus_Cleave_140: {
        },
        CDOTA_Modifier_EldwurmsEdda: {
        },
        CDOTA_Modifer_Item_DandelionAmulet: {
        },
        CDOTA_Modifier_Item_GhostScepter: {
        },
        CDOTA_Modifier_Item_TranquilBoots2: {
        },
        CDOTA_Modifier_Muerta_Gunslinger_Increased_Proc_Chance: {
        },
        CDOTA_Modifier_Pangolier_Swashbuckle_Damage_Penalty: {
        },
        CDOTA_Modifier_Oracle_DivinersDeck_Death: {
        },
        CDOTA_Modifier_Brewmaster_BrewUp: {
        },
        CDOTA_Modifier_Batrider_Flamebreak_Damage: {
        },
        CDOTA_Modifier_ShadowShaman_Voodoo: {
        },
        CDOTA_Modifier_CrystalMaiden_FreezingField_Tracker: {
        },
        CDOTA_Modifier_NPXBuff: {
        },
        CPlayer_CameraServices: {
        },
        CPulseCell_Timeline: {
        },
        CPulseCell_Inflow_EntOutputHandler: {
        },
        CDOTA_Unit_Hero_Elder_Titan: {
        },
        C_DOTA_Unit_Hero_Obsidian_Destroyer: {
        },
        C_DOTA_Item_Ceremonial_Robe: {
        },
        C_DOTA_Ability_Ringmaster_Impalement: {
        },
        C_DOTA_Ability_Muerta_Supernatural: {
        },
        CDOTA_Ability_Mars_GodsRebuke: {
        },
        C_DOTA_Ability_Elder_Titan_NaturalOrder: {
        },
        C_DOTA_Ability_Alchemist_Corrosive_Weaponry: {
        },
        C_DOTA_Ability_Clinkz_Empty1: {
        },
        C_DOTA_Ability_Animation_RightClawSwipe: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Lina_6: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Antimage_4: {
        },
        CDOTA_Modifier_Special_Bonus_Vision: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_50: {
        },
        C_DOTA_Ability_Special_Bonus_Gold_lvl15_r: {
        },
        C_DOTA_TrackingProjectileInfoParticleAPI: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Modifier_Roshans_Banner_Aura: {
        },
        CDOTA_Modifier_Mars_ArenaOfBlood_Kill_Buff: {
        },
        CDOTA_Modifier_Winter_Wyvern_Arctic_Burn_Slow: {
        },
        CDOTA_Modifier_Meepo_Petrify: {
        },
        CDOTA_Modifier_Lina_SuperCharged: {
        },
        CDOTA_Modifier_Razor_Arcana_EmpoweredState: {
        },
        CDOTA_Modifier_Juggernaut_BladeFury: {
        },
        CDOTA_Modifier_ScriptedMotionController: {
        },
        CHeroesPerPlayer: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CScenePayloadVData: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        C_DOTA_Unit_Hero_Magnataur: {
        },
        C_DOTA_Ability_Pangolier_HeartPiercer: {
        },
        CDOTA_Ability_Broodmother_SpinWeb: {
        },
        C_DOTA_Ability_Lion_FingerOfDeath: {
        },
        C_DOTA_Ability_Nevermore_Shadowraze2: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Keeper_of_the_Light_7: {
        },
        C_DOTA_Ability_Special_Bonus_HP_900: {
        },
        PlayerResourcePlayerPeriodicResourceData_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Modifier_Tome_of_Aghanim: {
        },
        CDOTA_Modifier_Ringmaster_TameTheBeasts: {
        },
        CDOTA_Modifier_Dawnbreaker_Fire_Wreath_Magic_Immunity_Tooltip: {
        },
        CDOTA_Modifier_Techies_StickyBomb_Slow: {
        },
        CDOTA_Modifier_Slark_Armor_Reduction_Debuff: {
        },
        CDOTA_Modifier_Meepo_Flung: {
        },
        CDOTA_Modifier_NightStalker_Void_Zone: {
        },
        CDOTA_Modifier_Luna_LucentBeam_Damage_Buff: {
        },
        CDOTA_Modifier_DeathProphet_Scepter: {
        },
        CDOTA_Modifier_Courier_TakeStashItems: {
        },
        CDOTA_Modifier_Sven_Warcry_Passive: {
        },
        CFilterAttributeInt: {
        },
        C_DOTA_Item_DaggerOfRistul: {
        },
        C_DOTA_Item_Fusion_rune: {
        },
        C_DOTA_Ability_Invoker_SunStrike: {
        },
        C_DOTA_Ability_PhantomLancer_PhantomEdge: {
        },
        CDOTA_Ability_Axe_BattleHunger: {
        },
        CDOTA_Ability_VengefulSpirit_Revenge: {
        },
        C_DOTA_Ability_Sven_GreatCleave: {
        },
        CPointTemplate: {
        },
        C_DOTA_Ability_PudgeMiniboss_ArmorCorruption: {
        },
        CDOTA_Modifier_Seasonal_Summon_Common_Thinker: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Weaver_1: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Slardar_5: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Range_400: {
        },
        C_DOTA_Ability_Special_Bonus_Attributes: {
        },
        CDOTA_Modifier_Ringmaster_UnicycleMovement: {
        },
        CDOTA_Modifier_Phoenix_FireSpiritBurn: {
        },
        CDOTA_Modifier_Abyssal_Underling_Warrior_ManaBurn: {
        },
        CDOTA_Modifier_Spirit_Bear_Attack_Damage: {
        },
        CDOTA_Modifier_Clinkz_WindWalk: {
        },
        CDOTA_Modifier_Lion_Innate_ToHellAndBack_Buff: {
        },
        CDOTA_Modifier_Sniper_Shrapnel_Thinker: {
        },
        CDOTA_Modifier_Kunkka_GhostShip_DamageDelay: {
        },
        CDOTA_Modifier_Bloodseeker_BloodMist_Slow: {
        },
        CDOTA_Modifier_Stunned: {
        },
        CPlayer_FlashlightServices: {
        },
        C_DOTA_Unit_Hero_Beastmaster_Hawk: {
        },
        C_DOTA_Item_GaleGuard: {
        },
        C_DOTA_Ability_Largo_AmphibianRhapsody: {
        },
        C_DOTA_Ability_Kez_Shodo_Sai: {
        },
        C_DOTA_Ability_Invoker_Tornado_AD: {
        },
        CDOTA_Ability_DarkSeer_Vacuum: {
        },
        C_DOTA_Ability_Luna_Eclipse: {
        },
        C_DOTA_Ability_WitchDoctor_DeathWard: {
        },
        CDOTA_Modifier_Greevil_Miniboss_Blue_IceVortexThinker: {
        },
        CDOTA_Modifier_Mutation_CreateTombstone: {
        },
        CDOTA_Modifier_AghsFort_Waveblaster_Leap: {
        },
        CDOTA_Ability_Creature_IceSlam: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Night_Stalker: {
        },
        C_DOTA_Ability_Special_Bonus_HP_175: {
        },
        PlayerResourcePlayerData_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Modifier_Item_Nether_Shawl: {
        },
        CDOTA_Modifier_Item_WraithBand: {
        },
        CDOTA_Modifier_Hoodwink_Bushwhack_Trap: {
        },
        CDOTA_Modifier_Magnataur_Horn_Toss_Slow: {
        },
        CDOTA_Modifier_Gyrocopter_Side_Gunner: {
        },
        CDOTA_Modifier_Life_Stealer_Infest_Effect: {
        },
        CDOTA_Modifier_FacelessVoid_ArcanaDamageRouter: {
        },
        CDOTA_Modifier_Razor_Arcana_Kill_Effect: {
        },
        CDOTA_Unit_Miniboss_Minion: {
        },
        CDOTA_Item_MetamorphicMandible: {
        },
        C_DOTA_Item_DandelionAmulet: {
        },
        C_DOTA_Item_MeteorHammer: {
        },
        C_DOTA_Item_Hood_Of_Defiance: {
        },
        C_DOTA_Ability_Hoodwink_HeavyQuiver: {
        },
        C_DOTA_Ability_Bristleback_Prickly: {
        },
        CDOTA_Ability_Night_Stalker_Night_Reign: {
        },
        C_DOTA_Ability_Sand_King_Scorpion_Strike: {
        },
        C_DOTA_Ability_Juggernaut_Vaulted_Strike: {
        },
        C_DOTA_Ability_Juggernaut_BladeFury: {
        },
        CDOTA_Modifier_AghsFort_TrapRoom_MeatHook_PathingFix: {
        },
        CDOTA_Modifier_AghsFort_Spectre_ActiveDispersion: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Wisp_4: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Clockwerk_4: {
        },
        CDOTA_Modifier_Item_Witch_Blade: {
        },
        CDOTA_Modifier_Item_Pavise: {
        },
        CDOTA_Modifier_MaskOfMadness_Berserk: {
        },
        CDOTA_Modifier_MonkeyKing_FurArmy_SoldierInactive: {
        },
        CDOTA_Modifier_AbyssalUnderlord_AtrophyAura_Effect: {
        },
        CDOTA_Modifier_Nyx_Assassin_Vendetta_Armor_Reduction: {
        },
        CDOTA_Modifier_Meepo_Divided_We_Stand: {
        },
        CDOTA_Modifier_SpiritBreaker_GreaterBash_Knockback: {
        },
        CDOTA_Modifier_Jakiro_Macropyre_Ice_Edge_Slow: {
        },
        CDOTA_Modifier_Roshan_Bash: {
        },
        CDOTA_Modifier_Razor_StaticLink: {
        },
        CDOTA_Modifier_SandKing_Epicenter: {
        },
        CDOTA_Modifier_HP_Regen: {
        },
        CPulseCell_Outflow_CycleOrdered__InstanceState_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        C_DOTA_Item_Recipe_Angels_Demise: {
        },
        C_DOTA_Item_Recipe_Cornucopia: {
        },
        C_DOTA_Item_Spirit_Vessel: {
        },
        C_DOTA_Item_Aegis: {
        },
        C_DOTA_Item_Clarity: {
        },
        CDOTA_Ability_Dark_Willow_Pixie_Dust: {
        },
        C_DOTA_Ability_DarkWillow_CursedCrown: {
        },
        C_DOTA_Ability_Phoenix_Supernova: {
        },
        CDOTA_Ability_Treant_LifeBomb_Explode: {
        },
        CDOTA_Ability_Lycan_SummonWolves_GeistForm: {
        },
        C_DOTA_Ability_Shadow_Shaman_Fowl_Play: {
        },
        CDOTA_Ability_Frogmen_WaterBubble_Large: {
        },
        CDOTA_Modifier_Greevil_Miniboss_White_Degen_Aura: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Rubick_5: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Huskar_3: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lifestealer_4: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_8: {
        },
        C_DOTA_Ability_Special_Bonus_All_Stats_5: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Speed_70: {
        },
        CDOTA_Modifier_Item_SuperSwift_Blink: {
        },
        CDOTA_Modifier_Item_Voodoo_Mask: {
        },
        CDOTA_Modifier_Item_CrellasCrozier_Movespeed_Debuff: {
        },
        CDOTA_Modifier_Hoodwink_Boomerang_Invulnerable: {
        },
        CDOTA_Modifier_Snapfire_GobbleUp_Creep: {
        },
        CDOTA_Modifier_Visage_GraveChill_Debuff: {
        },
        CDOTA_Modifier_Undying_Decay_BuffCounter: {
        },
        CDOTA_Modifier_Undying_Decay_Debuff: {
        },
        CDOTA_Modifier_Shadow_Demon_Shadow_Poison: {
        },
        CDOTA_Modifier_Silencer_BlabberMouth: {
        },
        CDOTA_Modifier_Chen_Penitence: {
        },
        CDOTA_Modifier_Batrider_StickyNapalm_Application: {
        },
        CDOTA_Modifier_PhantomAssassin_PhantomStrike: {
        },
        CDOTA_Modifier_FacelessVoid_Timelock_Basic_Proc_Marker: {
        },
        CDOTA_Modifier_Beastmaster_Rugged: {
        },
        CDOTA_Modifier_Shadowraze_Debuff: {
        },
        CDOTA_Ability_AntiMage_BlinkParticleAPI: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Modifier_Item_Imbue_Choice: {
        },
        C_DOTA_Unit_Hero_Tusk: {
        },
        C_DOTA_BaseNPC: {
        },
        CDOTA_Item_Recipe_Diffusal_Blade2: {
        },
        C_DOTA_Item_Sphere: {
        },
        C_DOTA_Item_Dagon_Upgraded4: {
        },
        CDOTA_Ability_AbyssalUnderlord_PitOfMalice: {
        },
        CDOTA_Ability_Abaddon_AphoticShield: {
        },
        C_DOTA_Ability_Viper_Predator: {
        },
        C_DOTA_Ability_Nian_Whirlpool: {
        },
        C_DOTA_Ability_Riki_Poison_Dart: {
        },
        C_DOTA_Ability_Frogmen_TendrilsOfTheDeep: {
        },
        CDOTA_Modifier_OgreMagi_FrostArmor_Slow: {
        },
        C_SoundEventAABBEntity: {
        },
        C_DOTA_Ability_AghsFort_Creature_Phoenix_FireSpirits: {
        },
        C_DOTA_Ability_Special_Bonus_Cast_Range_100: {
        },
        PlayerResourcePlayerTeamData_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        PlayerResourcePlayerEventData_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Modifier_Item_Misericorde: {
        },
        CDOTA_Modifier_Item_Clumsy_Net: {
        },
        CDOTA_Modifier_Hoodwink_Decoy_Illusion: {
        },
        CDOTA_Modifier_Mars_ArenaOfBlood_AnimationAura: {
        },
        CDOTA_Modifier_DarkWillow_ShadowRealm_Aura_Debuff: {
        },
        CDOTA_Modifier_AbyssalUnderlord_AtrophyAura_DmgBuffCounter: {
        },
        CDOTA_Modifier_AbyssalUnderlord_PitOfMalice_Buff_Placer: {
        },
        CDOTA_Modifier_Rubick_NullField_Effect: {
        },
        CDOTA_Modifier_Brewmaster_StormBrewling_StormStance: {
        },
        CDOTA_Modifier_Alchemist_AcidSpray: {
        },
        CDOTA_Modifier_Clinkz_Infernal_Shred: {
        },
        CDOTA_Modifier_Rattletrap_Cog_Pinball: {
        },
        CDOTA_Modifier_Dazzle_Innate_Weave_Armor_Counter: {
        },
        CDOTA_Modifier_TemplarAssassin_Meditation: {
        },
        CDOTA_Modifier_Shadow_Shaman_Serpent_Ward_Shackle_Tracker: {
        },
        CDOTA_Modifier_Holdout_StaticRemnantThinker: {
        },
        CDOTA_Modifier_Mirana_Leap_Root: {
        },
        CDOTA_Modifier_Rune_ExtraDamage: {
        },
        CDOTA_Modifier_RootedUndispellable: {
        },
        CDOTA_Modifier_Invulnerable: {
        },
        C_TintController: {
        },
        C_DOTA_Item_Spy_Gadget: {
        },
        C_DOTA_Item_Dagon_Upgraded: {
        },
        C_DOTA_Item_RingOfRegeneration: {
        },
        C_DOTA_Item_GlovesOfHaste: {
        },
        C_DOTA_Ability_EmberSpirit_Immolation: {
        },
        C_DOTA_Ability_Skywrath_Mage_Staff_Of_The_Scion: {
        },
        CDOTA_Ability_Broodmother_PoisonSting: {
        },
        C_DOTA_Ability_Mirana_Starfall: {
        },
        CDOTA_Ability_Lamp_Use: {
        },
        CDOTA_Modifier_AghsFort_SwampSickness: {
        },
        CDOTA_Modifier_Healing_Campfire_Aura: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Tiny_7: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Base_Damage_15: {
        },
        C_DOTA_Ability_Special_Bonus_MP_300: {
        },
        C_DOTA_Ability_Special_Bonus_HP_100: {
        },
        CDOTA_Modifier_Item_Mysterious_Hat: {
        },
        CDOTA_Modifier_Dawnbreaker_Solar_Guardian_After_Land: {
        },
        CDOTA_Modifier_Pangolier_Fortune_Favors_The_Bold: {
        },
        CDOTA_Modifier_Techies_Minefield_Sign_Scepter: {
        },
        CDOTA_Modifier_Abaddon_TheQuickening: {
        },
        CDOTA_Modifier_Rattletrap_BatteryAssault: {
        },
        CDOTA_Modifier_Luna_LunarBlessing: {
        },
        CDOTA_Modifier_Slardar_Amplify_Damage_SelfBuff: {
        },
        CDOTA_Modifier_Bloodseeker_BloodMist_Passive: {
        },
        CDOTA_NPC_Observer_Ward_TrueSight: {
        },
        C_DOTA_Unit_Hero_Silencer: {
        },
        C_DOTA_BaseNPC_HoldoutTower_ReduceSpeed: {
        },
        C_DOTA_Item_Giants_Ring: {
        },
        C_DOTA_Item_Bloodstone: {
        },
        C_DOTA_Item_Assault_Cuirass: {
        },
        C_DOTA_Item_Hyperstone: {
        },
        CDOTA_Ability_GraniteGolem_Bash: {
        },
        C_DOTA_Ability_Neutral_SpellImmunity: {
        },
        CDOTA_Modifier_Mutation_CritChance_Team_Aura: {
        },
        C_FuncBrush: {
        },
        C_DOTA_Ability_Special_Bonus_Strength_40: {
        },
        C_DOTA_Ability_Special_Bonus_Movement_Speed_30: {
        },
        CDOTA_Modifier_Jidi_Pollen_Bag: {
        },
        CDOTA_Modifier_Seer_Stone: {
        },
        CDOTA_Modifier_Hoodwink_Tomokan_Tracker_Aura: {
        },
        CDOTA_Modifier_Oracle_RainOfDestiny: {
        },
        CDOTA_Modifier_Oracle_PurifyingFlames: {
        },
        CDOTA_Modifier_KeeperOfTheLight_SpiritForm: {
        },
        CDOTA_Modifier_NightStalker_Void: {
        },
        CDOTA_Modifier_Rattletrap_Cogs_Movement: {
        },
        CDOTA_Modifier_FacelessVoid_TimeZone: {
        },
        CDOTA_Modifier_Tiny_CraggyExterior: {
        },
        CDOTA_Modifier_BackdoorProtectionActive: {
        },
        CIngameEvent_SeasonFall2025: {
        },
        C_DOTA_Ability_Muerta_DeadShot: {
        },
        CDOTA_Ability_Slark_DarkPact: {
        },
        C_DOTA_Ability_Spectre_Haunt: {
        },
        CDOTA_Modifier_Aghsfort_Reward_CritAura: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Rubick_6: {
        },
        CDOTA_Modifier_Special_Bonus_Night_Vision: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_40: {
        },
        C_DOTA_Ability_Special_Bonus_Agility_7: {
        },
        C_DOTA_Ability_Special_Bonus_MP_Regen_6: {
        },
        CDOTA_Modifier_IdolOfScreeauk: {
        },
        CDOTA_Modifier_Item_Terror_Mask_Fear: {
        },
        CDOTA_Modifier_Jacket_Blind: {
        },
        CDOTA_Modifier_Marci_Special_Delivery_Teleport_Effect: {
        },
        CDOTA_Modifier_Mars_Scepter_Damage_Tracker: {
        },
        CDOTA_Modifier_ArcWarden_Flux_Alone: {
        },
        CDOTA_Modifier_Undying_Decay_Buff_Shared: {
        },
        CDOTA_Modifier_Spectre_Dispersion_Memory: {
        },
        CDOTA_Modifier_Leshrac_Decrepify: {
        },
        CDOTA_Modifier_SandKing_SandStorm: {
        },
        C_DOTATurboGameMode: {
        },
        C_DOTA_Item_Ancient_Janggo: {
        },
        C_DOTA_Item_Recipe_WraithBand: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Disruptor_9: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Amplify_9: {
        },
        C_DOTA_Ability_Special_Bonus_HP_Regen_12: {
        },
        CDOTA_Modifier_ForgargersHealth_Buff: {
        },
        CDOTA_Modifier_Item_Essence_Distiller_Thinker_VFX: {
        },
        CDOTA_Modifier_BootsOfTravel_Incoming: {
        },
        CDOTA_Modifier_Snapfire_FiresnapCookie_LandingGesture: {
        },
        CDOTA_Modifier_Terrorblade_Metamorphosis: {
        },
        CDOTA_Modifier_TrollWarlord_WhirlingAxes_Blind: {
        },
        CDOTA_Modifier_Ogre_Magi_Smash_Buff: {
        },
        CDOTA_Modifier_Lycan_Shapeshift: {
        },
        CDOTA_Modifier_Batrider_StickyNapalm: {
        },
        CDOTA_Modifier_Omniknight_Angelic_Flight: {
        },
        CDOTA_Modifier_DeathProphet_Exorcism: {
        },
        CDOTA_Modifier_Beastmaster_Hawk_Dive: {
        },
        C_EconEntity__AttachedParticleInfo_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        PhysicsRagdollPose_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CPropDataComponent: {
        },
        C_DOTA_Unit_Hero_Ursa: {
        },
        CDOTA_Item_Guardian_Shell: {
        },
        CDOTA_Item_Trident: {
        },
        C_DOTA_Item_Philosophers_Stone: {
        },
        C_DOTA_Item_CrellasCrozier: {
        },
        C_DOTA_Ability_Legion_Commander_Outfight_Them: {
        },
        C_DOTA_Ability_Courier_TransferItems_ToOtherPlayer: {
        },
        C_DOTA_Ability_Enigma_Splitting_Image: {
        },
        C_DOTA_Ability_Razor_Dynamo: {
        },
        C_DOTA_Ability_BackdoorProtection: {
        },
        CDOTA_Modifier_Passive_Lotus_Pool: {
        },
        CDOTA_Modifier_Seasonal_Summon_TI11_Balloon_Thinker: {
        },
        C_DOTA_Ability_AghsFort_TrapRoom_Hookshot: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Centaur_2: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Axe_5: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Ember_Spirit_6: {
        },
        C_DOTA_LinearProjectileInfoParticleAPI: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Modifier_Brewmaster_PermanentImmolation: {
        },
        CDOTA_Modifier_Broodmother_StickySnare: {
        },
        CDOTA_Modifier_Phantom_Assassin_Coup_Counter: {
        },
        CDOTA_Modifier_Nian_Big_Flinch: {
        },
        CDOTA_Modifier_Roshan_InherentBuffs: {
        },
        CDOTA_Modifier_Bloodseeker_Thirst_Speed: {
        },
        CDOTA_Modifier_CrystalMaiden_Let_It_Go_Thinker: {
        },
        CDOTA_AbilityDraftAbilityState: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CPulseCell_LimitCount__InstanceState_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // X��@�
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        C_DOTA_BaseNPC_RotatableBuilding: {
        },
        C_DOTA_Item_Recipe_Trickster_Cloak: {
        },
        C_DOTA_Item_Recipe_Witch_Blade: {
        },
        CDOTA_Ability_Shadow_Demon_Soul_Catcher: {
        },
        C_DOTA_Ability_Rattletrap_RocketFlare: {
        },
        C_DOTA_Ability_Holdout_ScourgeWard: {
        },
        CDOTA_Ability_Bloodseeker_Sanguivore: {
        },
        C_DOTA_Ability_VengefulSpirit_Nether_Swap: {
        },
        CDOTA_Modifier_HarpyStorm_ChainLightning: {
        },
        CDOTA_Item_BookIntelligence: {
        },
        CDOTA_Ability_AghsFort_Creature_SpikedCarapace: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lifestealer_5: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Ancient_Apparition_1: {
        },
        C_DOTA_Ability_Special_Bonus_Cast_Range_225: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Range_50: {
        },
        C_DOTA_Ability_Special_Bonus_Intelligence_35: {
        },
        C_DOTA_Ability_Special_Bonus_All_Stats_12: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Speed_200: {
        },
        CDOTA_Modifier_Item_Naginata: {
        },
        CDOTA_Modifier_Item_Trident: {
        },
        CDOTA_Modifier_Item_ChainMail: {
        },
        CDOTA_Modifier_CallOfTheWild_Boar_PoisonEffect: {
        },
        CDOTA_Modifier_Beastmaster_Axe_Stack_Counter: {
        },
        CDOTA_Modifier_Tidehunter_KrakenShell_Boost: {
        },
        CDOTA_Modifier_Juggernaut_BladeDance: {
        },
        C_DOTA_Item_Recipe_RingOfAquila: {
        },
        C_DOTA_Ability_KeeperOfTheLight_Will_O_Wisp: {
        },
        C_DOTA_Ability_PhantomAssassin_Blur: {
        },
        C_DOTA_Ability_AntiMage_Blink: {
        },
        C_DOTA_Ability_Greevil_Miniboss_Green_Overgrowth: {
        },
        CDOTA_Modifier_Fountain_Glyph: {
        },
        C_DynamicLight: {
        },
        C_DOTA_Ability_Special_Bonus_30_Crit_2: {
        },
        C_DOTA_Ability_Special_Bonus_Reincarnation_300: {
        },
        TrackedStatNetworkData_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Modifier_Item_Enhancement_Vast: {
        },
        CDOTA_Modifier_Snapfire_FiresnapCookie_PreHop: {
        },
        CDOTA_Modifier_Phoenix_Sun_Ray_Blind: {
        },
        CDOTA_Modifier_Nyx_Assassin_Jolt_Debuff: {
        },
        CDOTA_Modifier_ChaosKnight_Chaos_Strike: {
        },
        CDOTA_Modifier_Alchemist_ChemicalRageTransform: {
        },
        CDOTA_Modifier_Rattletrap_Hookshot_Barrier: {
        },
        CDOTA_Modifier_Roshan_RevengeRoar_Windup: {
        },
        CDOTA_Modifier_Ursa_Innate_Maul: {
        },
        CDOTA_Modifier_CrystalMaiden_FrozenExpanse: {
        },
        CDOTA_Modifier_DebuffImmune: {
        },
        EngineCountdownTimer: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Item_WindLace: {
        },
        C_DOTA_Item_Pipe: {
        },
        C_DOTA_Item_ChainMail: {
        },
        C_DOTA_Ability_Dawnbreaker_Luminosity: {
        },
        C_DOTA_Ability_Elder_Titan_NaturalOrder_Spirit: {
        },
        CDOTA_Ability_Medusa_GorgonGrasp: {
        },
        C_CDOTA_Ability_Treant_LeechSeed: {
        },
        CDOTA_Ability_Invoker_InvokedBase: {
        },
        C_DOTA_Ability_Jakiro_IcePath_Detonate: {
        },
        C_DOTA_Ability_Slardar_Slithereen_Crush: {
        },
        C_DOTA_Ability_Sniper_Concussive_Grenade: {
        },
        C_DOTA_Ability_AncientApparition_BoneChill: {
        },
        C_SoundEventSphereEntity: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Rubick_8: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Enigma: {
        },
        C_DOTA_Ability_Special_Bonus_HP_800: {
        },
        CDOTA_Modifier_Item_Enhancement_Titanic: {
        },
        CDOTA_Modifier_Item_ConsecratedWraps_Effect: {
        },
        CDOTA_Modifier_Orb_Of_Destruction_Debuff: {
        },
        CDOTA_Modifier_Item_Moonshard: {
        },
        CDOTA_Modifier_ArcWarden_MagneticField_Attack_Speed: {
        },
        CDOTA_Modifier_Oracle_DivinersDeck_TheWorld_Next: {
        },
        CDOTA_Modifier_Visage_Lurker_Linger: {
        },
        CDOTA_Modifier_Weaver_Swarm_Debuff: {
        },
        CDOTA_Modifier_Clinkz_Strafe: {
        },
        CDOTA_Modifier_Rattletrap_Cog_Self_Bonuses: {
        },
        CDOTA_Modifier_Puck_Coiled: {
        },
        CDOTA_Modifier_Kunkka_Ghost_Ship_Fleet: {
        },
        CDOTA_Modifier_Jugg_Caster: {
        },
        CDOTA_Modifier_MoveSpeed_Percentage: {
        },
        CIngameEvent_TI2020: {
        },
        C_DOTA_BaseNPC_Promo: {
        },
        C_DOTA_Item_Angels_Demise: {
        },
        CDOTA_Item_Faded_Broach: {
        },
        CDOTA_Ability_Snapfire_Scatterblast: {
        },
        C_DOTA_Ability_Keeper_of_the_Light_SpecialReserve: {
        },
        C_DOTA_Ability_Batrider_Firefly: {
        },
        C_DOTA_Ability_Venomancer_VenomousGale: {
        },
        C_DOTA_Ability_Slardar_Sprint: {
        },
        C_DOTA_Ability_ForestTrollHighPriest_ManaAura: {
        },
        CDOTA_Ability_AghsFort_Ascension_MagneticField: {
        },
        CDOTA_Modifier_Seasonal_Summon_TI9_Balloon_Thinker: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Luna_3: {
        },
        C_DOTA_Ability_Special_Bonus_Armor_6: {
        },
        CDOTA_Modifier_Item_Revenants_Brooch: {
        },
        CDOTA_Modifier_Item_Dagon: {
        },
        CDOTA_Modifier_Primal_Beast_Colossal: {
        },
        CDOTA_Modifier_PrimalBeast_Uproar_Slow: {
        },
        CDOTA_Modifier_Grimstroke_DarkArtistry_Slow: {
        },
        CDOTA_Modifier_Grimstroke_InkCreature_Debuff: {
        },
        CDOTA_Modifier_Magnetic_Horn_Items: {
        },
        CDOTA_Modifier_DoomBringer_Doom: {
        },
        CDOTA_Modifier_Batrider_Displacement_Buff: {
        },
        CDOTA_Modifier_DeathProphet_Silence: {
        },
        CDOTA_Modifier_WitchDoctor_Voodoo_Switcheroo: {
        },
        CDOTA_Modifier_Windrunner_GaleForce: {
        },
        CDOTA_Modifier_DrowRanger_FrostArrows_Shard_Slow: {
        },
        CDOTA_Modifier_Pudge_Dismember_Pull: {
        },
        CDOTA_Modifier_Cyclone: {
        },
        C_DOTAGamerules: {
        },
        CDOTA_BaseNPC_Phantom_Assassin_GroundDagger: {
        },
        C_DOTA_Item_Tier2Token: {
        },
        C_TonemapController2Alias_env_tonemap_controller2: {
        },
        C_DOTA_Item_Jidi_Pollen_Bag: {
        },
        C_DOTA_Ability_Winter_Wyvern_Eldwurm_Scholar: {
        },
        C_DOTA_Ability_Tusk_IceShards: {
        },
        CDOTA_Ability_Centaur_Horsepower: {
        },
        C_DOTA_Ability_DeathProphet_SpiritSiphon: {
        },
        C_DOTA_Ability_Windrunner_FocusFire: {
        },
        C_DOTA_Ability_Zuus_Cloud: {
        },
        C_DOTA_Item_BotChallenge_MeteorStaff: {
        },
        CDOTA_Ability_MudGolem_RockDestroy: {
        },
        C_DOTA_Ability_WarpineRaider_SeedShot: {
        },
        CFilterLOS: {
        },
        CPointOrient: {
        },
        CDOTA_Modifier_Seasonal_TI11_DuelAccepted: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Antimage_3: {
        },
        C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_10: {
        },
        CDeferredLightBase: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_GlobalLight: {
        },
        C_EnvWindClientside: {
        },
        CDOTA_Modifier_HeavensHalberd_Debuff: {
        },
        CDOTA_Modifier_Item_MonkeyKingBar: {
        },
        CDOTA_Modifier_Item_MagicWand: {
        },
        CDOTA_Modifier_Item_PowerTreads: {
        },
        CDOTA_Modifier_Dawnbreaker_Converge_Thinker: {
        },
        CDOTA_Modifier_Elder_Titan_NaturalOrder_Aura_MagicResistance: {
        },
        CDOTA_Modifier_Nian_EruptionThinker: {
        },
        CDOTA_Modifier_Riki_TricksOfTheTrade_Move: {
        },
        CDOTA_Modifier_Zuus_Cloud: {
        },
        CDOTA_Modifier_PhantomLancer_PhantomEdge_Agility: {
        },
        sky3dparams_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_FightingGame_EffectsEntity: {
        },
        C_DOTA_Item_Enhancement_Vampiric: {
        },
        C_DOTA_Item_Swift_Blink: {
        },
        C_DOTA_Item_Craggy_Coat: {
        },
        C_DOTA_Item_Recipe_Heart: {
        },
        C_DOTA_Ability_Abaddon_Withering_Mist: {
        },
        C_DOTA_Ability_Shredder_ChakramAlias_shredder_chakram_2: {
        },
        C_DOTA_Ability_Visage_Stone_Form_Self_Cast: {
        },
        CDOTA_Ability_NagaSiren_RipTide: {
        },
        C_DOTA_Ability_Meepo_MegaMeepo: {
        },
        C_DOTA_Ability_Pugna_Oblivion_Savant: {
        },
        C_DOTA_Ability_Venomancer_Latent_Poison: {
        },
        C_DOTA_Ability_Animation_TailSpin: {
        },
        C_DOTA_Ability_Razor_EyeOfTheStorm: {
        },
        C_DOTA_Ability_Greevil_Miniboss_Blue_IceVortex: {
        },
        C_SoundEventConeEntity: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Doom_3: {
        },
        C_DOTA_Ability_Special_Bonus_Evasion_25: {
        },
        CDOTA_Modifier_Item_Black_Grimoire: {
        },
        CDOTA_Modifier_Item_Bullwhip_Buff_Thinker: {
        },
        CDOTA_Modifier_Item_CraniumBasher: {
        },
        CDOTA_Modifier_Muerta_PierceTheVeilTransform: {
        },
        CDOTA_Modifier_Techies_ReactiveTazer: {
        },
        CDOTA_Modifier_Disruptor_KineticWall_Thinker: {
        },
        CDOTA_Modifier_Rubick_Arcane_Supremacy: {
        },
        CDOTA_Modifier_Treant_LeechSeed_Slow: {
        },
        CDOTA_Modifier_LoneDruid_Entangle_Counter: {
        },
        CDOTA_Modifier_Brewmaster_Fire_Phase: {
        },
        CDOTA_Modifier_Jakiro_IcePath_Thinker: {
        },
        CDOTA_Modifier_Leshrac_Defilement: {
        },
        CDOTA_Modifier_TemplarAssassin_InnerPeace_Passive: {
        },
        CDOTA_Modifier_Miniboss_Team_Unreliable_Gold_Tracker_Thinker: {
        },
        CDOTA_Modifier_Kunkka_Torrent_Storm: {
        },
        CDOTA_Modifier_Lina_LagunaBlade_Barrier: {
        },
        CDOTA_Modifier_StormSpirit_Electric_Rave: {
        },
        CDOTA_Modifier_Mirana_Leap_Buff: {
        },
        CDOTA_Modifier_SandKing_Impale: {
        },
        CDestructiblePartsComponent: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        C_DOTA_NPC_TechiesMines: {
        },
        C_DOTA_Unit_Hero_Beastmaster_Boar: {
        },
        C_DOTA_PortraitBaseModel: {
        },
        C_EnvDeferredSpotLightClientOnly: {
        },
        C_DOTA_Item_Tree_Processor: {
        },
        C_DOTA_Item_Recipe_SheepStick: {
        },
        C_DOTA_Item_PoorMansShield: {
        },
        C_DOTA_Ability_Hoodwink_AcornShot: {
        },
        C_DOTA_Ability_EmberSpirit_FlameGuard: {
        },
        C_DOTA_Ability_Alchemist_GoblinsGreed: {
        },
        C_DOTA_Ability_BountyHunter_WindWalk: {
        },
        C_DOTA_Ability_Clinkz_Strafe: {
        },
        CDOTA_Ability_Life_Stealer_AssimilateEject: {
        },
        CDOTA_Ability_Miniboss_Reflect: {
        },
        C_DOTA_Ability_Greevil_Miniboss_Yellow_IonShell: {
        },
        C_DOTA_Ability_Greevil_Miniboss_Red_Overpower: {
        },
        C_EnvWind: {
        },
        CDOTA_Modifier_Seasonal_TI11_RockPaperScissors_Playing: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Hoodwink_BushwhackCooldown: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Ursa_3: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Morphling_4: {
        },
        C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_15: {
        },
        C_DOTA_Ability_Special_Bonus_Movement_Speed_Percentage_16: {
        },
        C_DOTA_Ability_Special_Bonus_HP_Regen_80: {
        },
        C_DOTA_Ability_AttributeBonus: {
        },
        CDOTA_Modifier_MaskOfDispair_Berserk: {
        },
        CDOTA_Modifier_Item_Spider_Legs_Active: {
        },
        CDOTA_Modifier_Marci_Grapple_VictimMotion: {
        },
        CDOTA_Modifier_Bristleback_QuillSpray_AutoCast: {
        },
        CDOTA_Modifier_Shredder_TwistedChakram_Invulnerable: {
        },
        CDOTA_Modifier_Wisp_Relocate_Thinker: {
        },
        CDOTA_Modifier_Silencer_CurseOfTheSilent: {
        },
        CDOTA_Modifier_Broodmother_Silken_Bola: {
        },
        CDOTA_Modifier_Leshrac_Lightning_Storm: {
        },
        CDOTA_Modifier_ShadowShaman_Self_Hex: {
        },
        CDOTA_Modifier_VengefulSpirit_Nether_Swap_DamageReduction: {
        },
        CDOTA_Modifier_Item_Mask_Spin_Crit: {
        },
        CDOTA_Modifier_Bane_Nightmare_AttackSpeed_Bonus: {
        },
        CPulseCell_Step_DebugLog: {
        },
        C_DOTA_Unit_Hero_Slardar: {
        },
        C_DOTA_Item_Bottomless_Chalice: {
        },
        C_DOTA_Item_Faerie_Fire: {
        },
        C_DOTA_Ability_Weaver_Threads_Of_Fate: {
        },
        C_DOTA_Ability_AntiMage_ManaThirst: {
        },
        C_DOTA_Ability_Special_Bonus_HP_500: {
        },
        CDOTA_Modifier_Item_VindicatorsAxe: {
        },
        CDOTA_Modifier_Item_Fallen_Sky: {
        },
        CDOTA_Modifier_Eul_Cyclone: {
        },
        CDOTA_Modifier_Item_CrellasCrozier_Active: {
        },
        CDOTA_Modifier_Dawnbreaker_Celestial_Hammer_Caster: {
        },
        CDOTA_Modifier_Legion_Commander_MomentOfCourage: {
        },
        CDOTA_Modifier_TrollWarlord_Fervor: {
        },
        CDOTA_Modifier_Medusa_FixedMovespeed: {
        },
        CDOTA_Modifier_Wisp_Spirits: {
        },
        CDOTA_Modifier_KeeperOfTheLight_Radiant_Bind: {
        },
        CDOTA_Modifier_KeeperOfTheLight_ManaLeak: {
        },
        CDOTA_Modifier_Meepo_Innate_PackRat: {
        },
        CDOTA_Modifier_Beastmaster_WildAxes_Intrinsic: {
        },
        CDOTA_Modifier_Warlock_Rain_of_Chaos_Leash: {
        },
        CDOTA_Modifier_Courier_Morph_Effect: {
        },
        C_DOTA_Item_Roshans_Banner: {
        },
        C_DOTA_Item_LanceOfPursuit: {
        },
        C_DOTA_Item_Naginata: {
        },
        CDOTA_Ability_Abyssal_Underling_Warrior_Sight: {
        },
        CDOTA_Ability_Slark_Barracuda: {
        },
        C_DOTA_Ability_Rubick_Hidden2: {
        },
        CDOTA_Ability_Furion_CurseOfTheForest: {
        },
        CDOTA_Ability_Bloodseeker_Bloodrage: {
        },
        C_DOTA_Ability_Creature_Fire_Breath: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Abaddon_4: {
        },
        CDOTA_Modifier_Item_Enhancement_Greedy: {
        },
        CDOTA_Modifier_Item_Eternal_Shroud_Bonus_Magic_Resist: {
        },
        CDOTA_Modifier_StaffOfWizardry: {
        },
        CDOTA_Modifier_Largo_Frogstomp_TreeShake: {
        },
        CDOTA_Modifier_VoidSpirit_ResonantPulse_PhysicalBuff: {
        },
        CDOTA_Modifier_Elder_Titan_AncestralSpirit_Hidden: {
        },
        CDOTA_Modifier_Treant_Lifebomb_Explode: {
        },
        CDOTA_Modifier_DoomBringer_InfernalBlade_Burn: {
        },
        CDOTA_Modifier_Enchantress_Bunny_Hop: {
        },
        CDOTA_Modifier_Venomancer_Latent_Poison: {
        },
        CDOTA_Modifier_Death_Seeker_OutOfWorld: {
        },
        CDOTA_Modifier_Necrolyte_Heartstopper_Aura_Effect: {
        },
        CBodyComponentBaseAnimGraph: {
        },
        CDOTA_Survivors_EffectsEntity: {
        },
        C_DOTA_Item_Possessed_Mask: {
        },
        C_DOTA_Item_Mirror_Shield: {
        },
        C_DOTA_Item_Holy_Locket: {
        },
        C_DOTA_Item_Recipe_Headdress: {
        },
        C_DOTA_Ability_AbyssalUnderlord_Portal_Warp: {
        },
        C_DOTA_Ability_Undying_CeaselessDirge: {
        },
        C_DOTA_Ability_Ogre_Magi_Unrefined_Fireblast: {
        },
        CDOTA_Ability_Life_Stealer_UnfetteredFury: {
        },
        C_DOTA_Ability_Pugna_NetherWard: {
        },
        C_DOTA_Ability_Beastmaster_Rugged: {
        },
        C_DOTA_Ability_Beastmaster_DrumsOfSlom: {
        },
        C_DOTA_Ability_Lich_FrostShield: {
        },
        CDOTA_Modifier_AghsFort_Ascension_MagneticField_Evasion: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Riki_5: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lion_3: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Enigma_9: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Arc_Warden: {
        },
        C_DOTA_Ability_Special_Bonus_Evasion_50: {
        },
        C_DOTA_Ability_Special_Bonus_Cast_Range_350: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_90: {
        },
        C_DOTA_Ability_Special_Bonus_HP_Regen_20: {
        },
        CDOTA_Modifier_Item_Enhancement_Mystical: {
        },
        CDOTA_Modifier_Item_Diadem: {
        },
        CDOTA_Modifier_Snapfire_Boomstick: {
        },
        CDOTA_Modifier_Obsidian_Destroyer_Equilibrium_Debuff_Counter: {
        },
        CDOTA_Modifier_SpiritBreaker_GreaterBash_Speed: {
        },
        CDOTA_Modifier_Huskar_Berserkers_Blood: {
        },
        CDOTA_Modifier_Life_Stealer_Assimilate: {
        },
        CDOTA_Modifier_Tinker_MarchOfTheMachinesThinker: {
        },
        CDOTA_Modifier_Tidehunter_AnchorSmash_Caster: {
        },
        CDOTA_Modifier_Lich_Sinister_Gaze_Self: {
        },
        CDOTA_Modifier_Kunkka_Torrent_Thinker: {
        },
        CDOTA_Modifier_CrystalMaiden_IceRink_Thinker: {
        },
        CDOTA_Modifier_AncientApparition_BoneChill: {
        },
        CPulseCell_BaseYieldingInflow: {
        },
        PulseNodeDynamicOutflows_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CDOTA_Unit_Hero_Mars: {
        },
        C_DOTA_Item_Harpoon: {
        },
        C_DOTA_Item_Witless_shako: {
        },
        C_DOTA_Item_Flying_Courier: {
        },
        C_DOTA_Ability_Bristleback_Warpath: {
        },
        C_DOTA_Ability_Disruptor_StaticStorm: {
        },
        C_DOTA_Ability_Rubick_Hidden4: {
        },
        C_DOTA_Ability_BountyHunter_Lookout: {
        },
        C_DOTA_Ability_Life_Stealer_Open_Wounds: {
        },
        C_DOTA_Ability_AntiMage_Counterspell: {
        },
        CDOTA_Modifier_SatyrHellcaller_UnholyAura: {
        },
        CDOTA_Ability_FrostbittenGolem_TimeWarpAura: {
        },
        C_DOTA_Ability_Ghost_FrostAttack: {
        },
        C_DOTA_Ability_Aghanim_UrnUpheaval: {
        },
        C_DOTA_Ability_PineCone_AcornShot: {
        },
        C_DOTAFogOfWarTempViewers: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Kunkka_3: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Spectre: {
        },
        C_DOTA_Ability_Special_Bonus_Agility_80: {
        },
        CDOTA_Modifier_Item_Assault_Cuirass: {
        },
        CDOTA_Modifier_Eul_Cyclone_Thinker: {
        },
        CDOTA_Modifier_Grimstroke_Scepter_Buff: {
        },
        CDOTA_Modifier_Winter_Wyvern_Arctic_Burn_Flight: {
        },
        CDOTA_Modifier_Skywrath_Mage_Arcane_Bolt_Lifesteal: {
        },
        CDOTA_Modifier_DoomBringer_Lvl_Pain: {
        },
        CDOTA_Modifier_Rattletrap_Overclocking: {
        },
        CPlayer_MovementServices_Humanoid: {
        },
        C_DOTA_Unit_Hero_Necrolyte: {
        },
        CDOTA_Item_Recipe_Arcane_Blink: {
        },
        CDOTA_Item_Moonshard: {
        },
        C_DOTA_Item_Soul_Ring: {
        },
        C_DOTA_Item_Recipe_OblivionStaff: {
        },
        C_DOTA_Ability_Tusk_Bitter_Chill: {
        },
        C_DOTA_Ability_Centaur_Innate_Rawhide: {
        },
        C_DOTA_Ability_SpiritBreaker_KnockbackAmplficiation: {
        },
        C_DOTA_Ability_Ursa_Earthshock: {
        },
        C_DOTA_Ability_Morphling_Accumulation: {
        },
        C_DotaSubquestPlayerStat: {
        },
        CDOTA_Modifier_AghsFort_BossWinterWyvern_Cold_Embrace_Debuff: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Pudge_6: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Amplify_16: {
        },
        C_DOTA_Ability_Special_Bonus_HP_650: {
        },
        CDOTA_Modifier_Wind_Waker: {
        },
        CDOTA_Modifier_Item_Wind_Waker: {
        },
        CDOTA_Modifier_Item_Nullifier_Slow: {
        },
        CDOTA_Modifier_Item_Infused_Raindrop: {
        },
        CDOTA_Modifier_Largo_AmphibianRhapsody_Self_Tracker: {
        },
        CDOTA_Modifier_MonkeyKing_Transform: {
        },
        CDOTA_Modifier_Phoenix_IcarusYoink: {
        },
        CDOTA_Modifier_Terrorblade_Demon_Zeal_Aura: {
        },
        CDOTA_Modifier_Medusa_StoneGaze: {
        },
        CDOTA_Modifier_Medusa_Mystic_Snake_AttackBuff: {
        },
        CDOTA_Modifier_Keeper_of_the_Light_SpecialReserve: {
        },
        CDOTA_Modifier_NightStalker_CripplingFear: {
        },
        CDOTA_Modifier_Dazzle_Poison_Touch_Split: {
        },
        CDOTA_Modifier_Nian_Knockdown: {
        },
        CDOTA_Modifier_Lion_FingerPunch: {
        },
        CDOTA_Modifier_Shadow_Shaman_Fowl_Play: {
        },
        CDOTA_Modifier_InvisibleTrueSightImmune: {
        },
        C_DOTA_DisplacementVisibility: {
        },
        CPulseCell_IsRequirementValid__Criteria_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Unit_Hero_Chen: {
        },
        C_DOTA_Unit_Hero_Kunkka: {
        },
        C_DOTA_BaseNPC_Effigy_Statue: {
        },
        C_PhysPropClientside: {
        },
        CDOTA_Item_Forage_Health: {
        },
        CDOTA_Item_Spark_Of_Courage: {
        },
        CDOTA_Item_Helm_Of_The_Undying: {
        },
        C_DOTA_Item_Recipe_Arcane_Boots: {
        },
        C_DOTA_Item_Recipe_RefresherOrb: {
        },
        CDOTA_Ability_Broodmother_SpinWeb_Destroy: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Lina_1: {
        },
        C_DOTA_Ability_SandKing_CausticFinale: {
        },
        C_BaseAnimatingOverlay: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Oracle_7: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Spectre_4: {
        },
        C_DOTA_Ability_Special_Bonus_20_Bash_2: {
        },
        C_DOTA_Ability_Special_Bonus_Evasion_75: {
        },
        C_DOTA_Ability_Special_Bonus_All_Stats_20: {
        },
        CDOTA_Modifier_Item_Enhancement_Boundless: {
        },
        CDOTA_Modifier_Item_Terror_Mask_Fear_Aura: {
        },
        CDOTA_Modifier_Nemesis_Curse: {
        },
        CDOTA_Modifier_Item_Sorcerers_Staff: {
        },
        CDOTA_Modifier_Item_Grove_Bow: {
        },
        CDOTA_Modifier_Primal_Beast_Innate_Status_Resistance_Per_Time: {
        },
        CDOTA_Modifier_Mars_Bulwark: {
        },
        CDOTA_Modifier_Pangolier_HeartPiercer_Delay: {
        },
        CDOTA_Modifier_Techies_LandMine: {
        },
        CDOTA_Modifier_LegionCommander_IntimidateSlow: {
        },
        CDOTA_Modifier_Silencer_LastWord_Disarm: {
        },
        CDOTA_Modifier_Shukuchi_Slow: {
        },
        CDOTA_Modifier_Furion_Sprout_Entangle: {
        },
        CDOTA_Modifier_QueenOfPain_Innate_Seduction: {
        },
        CDOTA_Modifier_Sniper_TakeAim_Bonus: {
        },
        CDOTA_Modifier_Earthshaker_Shard: {
        },
        CDOTA_Modifier_Juggernaut_Vaulted_Strike_Slashes: {
        },
        C_BaseDoor: {
        },
        CDOTA_Ability_BotChallenge_SkeletonKing_BoneGuard_DamageTracker: {
        },
        CDOTA_Item_Recipe_Orb_Of_Destruction: {
        },
        C_DOTA_Item_Horizon: {
        },
        C_DOTA_Item_Recipe_InvisibilityEdge: {
        },
        C_DOTA_Ability_Rubick_Hidden1: {
        },
        C_DOTA_Ability_Tiny_Tree_Grab: {
        },
        C_DOTA_Ability_Lina_Slow_Burn: {
        },
        C_DOTA_Ability_CentaurKhan_WarStomp: {
        },
        CDOTA_Ability_Seasonal_TI11_Rock: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Spirit_Breaker_2: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_80: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Speed_175: {
        },
        C_DOTA_PlayerResource: {
        },
        CDOTA_Modifier_Item_Enhancement_Nimble: {
        },
        CDOTA_Modifier_Item_Silver_Edge: {
        },
        CDOTA_Modifier_Largo_CroakOfGenius_Debuff: {
        },
        CDOTA_Modifier_Snapfire_FiresnapCookie_ShortHop: {
        },
        CDOTA_Modifier_TrollWarlord_Melee: {
        },
        CDOTA_Modifier_Lycan_Shapeshift_Speed: {
        },
        CDOTA_Modifier_Earthshaker_EnchantTotem_Animation: {
        },
        CDOTA_Modifier_SkeletonKing_MortalStrikeBonus: {
        },
        CDOTA_Modifier_Mirana_Starfall_Starstruck: {
        },
        C_DOTABaseGameMode: {
        },
        EntityRenderAttribute_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CPulseCell_Inflow_ObservableVariableListener: {
        },
        CDOTA_BaseNPC_Largo_Frogling: {
        },
        C_DOTA_Item_Eternal_Shroud: {
        },
        CDOTA_Item_Recipe_Force_Boots: {
        },
        C_DOTA_Item_Recipe_Blade_Mail: {
        },
        CDOTA_Ability_VoidSpirit_AetherRemnant: {
        },
        C_DOTA_Ability_Rubick_TelekinesisLand_Self: {
        },
        C_DOTA_Ability_Invoker_Empty1: {
        },
        CDOTA_Ability_Broodmother_SpawnSpiderlings: {
        },
        CDOTA_Ability_Night_Stalker_Heart_Of_Darkness: {
        },
        C_DOTA_Ability_Courier_GoToEnemySecretShop: {
        },
        CDOTA_Ability_Pudge_Dismember: {
        },
        CDOTA_Modifier_Spawnlord_Master_Stomp: {
        },
        CDOTA_Modifier_Mutation_Treecutter_Aura: {
        },
        C_DOTA_Item_BagOfGold: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Nyx_6: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Drow_Ranger_8: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_25: {
        },
        CDOTA_Modifier_Item_Ethereal_Blade_Slow: {
        },
        CDOTA_Modifier_VoidSpirit_AetherRemnantCreepDamage: {
        },
        CDOTA_Modifier_Mars_ArenaOfBlood_Spear: {
        },
        CDOTA_Modifier_Bristleback_Warpath_Stack: {
        },
        CDOTA_Modifier_Disruptor_Thunder_Strike_Slow: {
        },
        CDOTA_Modifier_Ogre_Magi_Ignite: {
        },
        CDOTA_Modifier_Shadow_Demon_Soul_Catcher_SpellAmp: {
        },
        CDOTA_Modifier_FacelessVoid_TimeWalk_ShardBuff: {
        },
        CDOTA_Modifier_Miniboss_InherentBuffs: {
        },
        CDOTA_Modifier_Kunkka_Tidebringer_Slow: {
        },
        CDOTA_Modifier_Zuus_LightningBoltThinker: {
        },
        CDOTA_Modifier_Morphling_Replicate_Illusion: {
        },
        CDOTA_Modifier_Razor_Dynamo: {
        },
        CDOTA_Modifier_CrystalMaiden_FreezingField_Slow: {
        },
        CDOTA_Modifier_VengefulSpirit_Nether_Swap_ChargeCounter: {
        },
        CDOTA_Modifier_Drow_Ranger_Multishot_Damage: {
        },
        CDOTA_Modifier_Nevermore_Frenzy: {
        },
        CDOTA_Modifier_Rune_SuperRegen: {
        },
        C_DOTA_Unit_Twin_Gate: {
        },
        C_DOTA_Item_Sange: {
        },
        C_DOTA_Ability_Oracle_Clairvoyant_Curse: {
        },
        C_DOTA_Ability_LoneDruid_SpiritBear_SpiritLink: {
        },
        C_DOTA_Ability_Invoker_AttributeBonus: {
        },
        C_DOTA_Ability_DoomBringer_Doom: {
        },
        C_DOTA_Ability_Weaver_Rewoven: {
        },
        CDOTA_Ability_Rattletrap_Armor_Power: {
        },
        C_DOTA_Ability_Puck_DreamCoil: {
        },
        C_DOTA_Ability_Special_Bonus_Cast_Range_175: {
        },
        CDOTA_Modifier_Item_Enhancement_Dominant: {
        },
        CDOTA_Modifier_Item_Guardian_Shell: {
        },
        CDOTA_Modifier_Item_Force_Field_Bonus: {
        },
        CDOTA_Modifier_Item_GlimmerCape: {
        },
        CDOTA_Modifier_Item_Medallion_Of_Courage_Armor_Addition: {
        },
        CDOTA_Modifier_Item_PhaseBoots: {
        },
        CDOTA_Modifier_ArcWarden_MagneticField_Thinker_Evasion: {
        },
        CDOTA_Modifier_TrollWarlord_BattleTrance_Vision: {
        },
        CDOTA_Modifier_Shadow_Demon_Soul_Catcher: {
        },
        CDOTA_Modifier_Lycan_Shapeshift_Transform: {
        },
        CDOTA_Modifier_Lifestealer_CorpseEater: {
        },
        CDOTA_Modifier_Pugna_NetherWard_Aura: {
        },
        CDOTA_Modifier_Roshan_Moving: {
        },
        CDOTA_Modifier_Riki_TricksOfTheTrade: {
        },
        CDOTA_Modifier_Axe_BerserkersCall: {
        },
        CIngameEvent_SeasonalRewardLine: {
        },
        CFilterMultipleAPI: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Unit_Hero_Dazzle: {
        },
        C_DOTA_Item_Recipe_Vanguard: {
        },
        C_DOTA_Ability_ArcWarden_Flux: {
        },
        C_DOTA_Ability_Terrorblade_Reflection: {
        },
        CDOTA_Ability_Nevermore_Frenzy: {
        },
        C_DOTA_Ability_LotusPool: {
        },
        CDOTA_Modifier_Aghsfort_Aziyog_Underlord_Portal_Warp_Channel: {
        },
        CDOTA_Ability_Aghsfort_Aziyog_Underlord_Firestorm: {
        },
        CDOTA_Ability_Aghanim_Spear: {
        },
        C_DOTA_Ability_AghsFort_Gyrocopter_Multi_Homing_Missile: {
        },
        CDOTA_Item_AghsFort_RefresherOrb_Shard: {
        },
        C_DOTA_Ability_Corspselord_Revive: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Techies_2: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Puck_2: {
        },
        C_DOTA_Ability_Special_Bonus_Lifesteal_20: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Range_75: {
        },
        C_DOTA_Ability_Special_Bonus_Agility_9: {
        },
        CDOTA_Modifier_Item_Guardian_Greaves_Aura: {
        },
        CDOTA_Modifier_Item_NullTalisman: {
        },
        CDOTA_Modifier_VoidSpirit_AetherRemnantThinker: {
        },
        CDOTA_Modifier_MonkeyKing_RightClickJump_Activity: {
        },
        CDOTA_Modifier_Techies_StasisTrap: {
        },
        CDOTA_Modifier_Brewmaster_EarthBrewling_EarthStance: {
        },
        CDOTA_Modifier_Brewmaster_PermanentImmolation_Aura: {
        },
        CDOTA_Modifier_Rattletrap_Cog_Marker: {
        },
        CDOTA_Modifier_FacelessVoid_TimeWalk_Tracker: {
        },
        CDOTA_Modifier_Sand_King_Scorpion_Strike_Slow: {
        },
        TransitioningLayer_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Unit_Hero_Shredder: {
        },
        C_DOTA_Unit_Hero_Rubick: {
        },
        C_DOTA_Item_Kaya: {
        },
        C_DOTA_Item_PowerTreads: {
        },
        C_DOTA_Ability_Venomancer_Sepsis: {
        },
        C_DOTA_Ability_Warlock_Fatal_Bonds: {
        },
        C_DOTA_Ability_Courier_Shield: {
        },
        TempViewerInfo_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Wraith_King_10: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Juggernaut_5: {
        },
        C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_65: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Speed_110: {
        },
        CDOTA_Modifier_Item_Enhancement_Manic: {
        },
        CDOTA_Modifier_Divine_Regalia: {
        },
        CDOTA_Modifier_Seer_Stone_Truesight: {
        },
        CDOTA_Modifier_Item_Ancient_Janggo_Aura: {
        },
        CDOTA_Modifier_Item_DustofAppearance_Thinker: {
        },
        CDOTA_Modifier_Item_QuellingBlade: {
        },
        CDOTA_Modifier_Dawnbreaker_Luminosity: {
        },
        CDOTA_Modifier_Hoodwink_Sharpshooter_VisionThinker: {
        },
        CDOTA_Modifier_AbyssalUnderlord_PitOfMalice_Thinker: {
        },
        CDOTA_Modifier_Techies_StickyBombThrow: {
        },
        CDOTA_Modifier_Slark_EssenceShift_DebuffCounter: {
        },
        CDOTA_Modifier_Broodmother_SpinWeb_ChargeCounter: {
        },
        CDOTA_Modifier_Nian_Dive: {
        },
        CDOTA_Modifier_Lion_FingerOfDeath_Delay: {
        },
        CDOTA_Modifier_Sniper_Assassinate_Trigger: {
        },
        CDOTA_Modifier_Morphling_ScepterStatsDrain_Strength_Buff: {
        },
        C_IngameEvent_TI6: {
        },
        C_DOTA_Unit_Hero_DoomBringer: {
        },
        C_DOTA_Item_RuneSpawner_Powerup: {
        },
        C_DOTA_Item_Enhancement_Vital: {
        },
        C_DOTA_Item_Unwavering_Condition: {
        },
        CDOTA_Item_Orb_Of_Destruction: {
        },
        CDOTA_Item_Recipe_Fallen_Sky: {
        },
        C_DOTA_Item_Recipe_Phylactery: {
        },
        C_DOTA_Item_RingOfProtection: {
        },
        C_DOTA_Ability_Legion_Commander_Duel: {
        },
        C_DOTA_Ability_Rubick_Hidden5: {
        },
        C_DOTA_Ability_Undying_Decay: {
        },
        C_DOTA_Ability_Leshrac_Lightning_Storm: {
        },
        C_DOTA_Ability_Venomancer_PlagueWard: {
        },
        CDOTA_Modifier_Tutorial_Sleep: {
        },
        CDOTA_Modifier_Mutation_StationaryDamageReduction: {
        },
        CBaseAnimatingActivity: {
        },
        CDOTA_Modifier_AghsFort_AssaultCaptain_SunRay: {
        },
        C_DOTA_Ability_Special_Bonus_Gold_Income_300: {
        },
        C_DOTA_Ability_Special_Bonus_Strength_8: {
        },
        CDOTA_Modifier_Item_Enchanters_Bauble: {
        },
        CDOTA_Modifier_Item_Paintball_Debuff: {
        },
        CDOTA_Modifier_Grimstroke_SoulChain_ChannelCheck: {
        },
        CDOTA_Modifier_Winter_Wyvern_Cold_Embrace: {
        },
        CDOTA_Modifier_AbyssalUnderlord_AtrophyAura_HeroDmgBuff: {
        },
        CDOTA_Modifier_TrollWarlord_BattleTrance_Ally: {
        },
        CDOTA_Modifier_Medusa_ManaShield_AttackBuff: {
        },
        CDOTA_Modifier_Nyx_Assassin_Vendetta_Fast: {
        },
        CDOTA_Modifier_Obsidian_Destroyer_Equilibrium_Barrier: {
        },
        CDOTA_Modifier_Invoker_IceWall_VectorTarget_Thinker: {
        },
        CDOTA_Modifier_Lina_LightStrikeArray_Thinker: {
        },
        CDOTA_Modifier_SkeletonKing_BoneGuard_Summon: {
        },
        CDOTA_Modifier_HeroStatuePedestal: {
        },
        CIngameEvent_TI2025: {
        },
        CModelState: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CPulseCell_LerpCameraSettings__CursorState_t: {
        },
        CPulseCell_Outflow_CycleOrdered: {
        },
        CDOTA_BaseNPC_Tinker_Turret: {
        },
        C_DOTA_Item_Diadem: {
        },
        CDOTA_Item_Kaya_And_Sange: {
        },
        C_DOTA_Item_Recipe_Bloodthorn: {
        },
        CDOTA_Item_Recipe_DragonLance: {
        },
        C_DOTA_Item_IronwoodBranch: {
        },
        CDOTA_Ability_Muerta_SpectralSlug: {
        },
        C_DOTA_Ability_ArcWarden_TempestDouble: {
        },
        CDOTA_Ability_AbyssalUnderlord_Firestorm: {
        },
        C_DOTA_Ability_Elder_Titan_Tip_The_Scales: {
        },
        C_DOTA_Ability_NagaSiren_MirrorImage: {
        },
        C_DOTA_Ability_Silencer_Oppressive_Silence: {
        },
        C_DOTA_Ability_Invoker_IceWall: {
        },
        C_DOTA_Ability_FacelessVoid_TimeWalk: {
        },
        C_DOTA_Ability_Beastmaster_Hawk_Dive: {
        },
        CDOTA_Ability_FillerAbility: {
        },
        CDOTA_Modifier_FelBeast_Haunt_OnDeath: {
        },
        CDOTA_Modifier_Spawnlord_Aura: {
        },
        CDOTA_Modifier_AncientRockGolem_Weakening: {
        },
        CDOTA_Ability_Seasonal_TI11_Duel: {
        },
        CDOTA_Ability_Seasonal_TI11_Paper: {
        },
        CDOTA_Ability_Seasonal_TI11_RockPaperScissors_Base: {
        },
        CDOTA_Modifier_Aghsfort_Reward_ArmorAura_Bonus: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Tidehunter_8: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Pudge_5: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_45: {
        },
        C_DOTA_Ability_Special_Bonus_Agility_6: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Speed_225: {
        },
        CDOTA_Modifier_Item_Gungir: {
        },
        CDOTA_Modifier_Item_EchoSabre: {
        },
        CDOTA_Modifier_Necronomicon_Warrior_Sight: {
        },
        CDOTA_Modifier_NagaSiren_SongOfTheSiren_IgnoreMe: {
        },
        CDOTA_Modifier_SpiritBreaker_GreaterBash_Guaranteed: {
        },
        CDOTA_Modifier_SpiritBreaker_GreaterBash_CascadingBashes: {
        },
        CDOTA_Modifier_Miniboss_Minion_Summoner_Buff: {
        },
        CDOTA_Modifier_Lina_CritDebuff: {
        },
        CIngameEvent_TI2023: {
        },
        CCollisionProperty: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_BaseNPC_Hero: {
        },
        C_DOTA_Item_Tier1Token: {
        },
        CFilterMassGreater: {
        },
        C_DOTA_Item_ManaDraught: {
        },
        CDOTA_Item_Recipe_Broom_Handle: {
        },
        C_DOTA_Item_Recipe_HandOfMidas: {
        },
        C_DOTA_Ability_Skywrath_Mage_Ancient_Seal: {
        },
        CDOTA_Ability_Nyx_Assassin_Burrow: {
        },
        C_EntityDissolve: {
        },
        C_DOTA_Ability_Special_Bonus_Lifesteal_18: {
        },
        C_DOTA_Ability_Special_Bonus_Respawn_Reduction_60: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Amplify_15: {
        },
        C_DOTA_Ability_Special_Bonus_Cast_Range_75: {
        },
        CDOTA_Modifier_OgreSealTotem_Slow: {
        },
        CDOTA_Modifier_Item_Elven_Tunic: {
        },
        CDOTA_Modifier_Largo_Song_Attack_Burst: {
        },
        CDOTA_Modifier_EarthSpirit_BoulderSmash: {
        },
        CDOTA_Modifier_Slark_ShadowDance_Passive: {
        },
        CDOTA_Modifier_Wisp_Tether_Cooldown: {
        },
        CDOTA_Modifier_Lycan_SummonWolves_Hamstring: {
        },
        CDOTA_Modifier_DarkSeer_IonShell_IllusionInvulnerability: {
        },
        CDOTA_Modifier_Bloodseeker_Sanguivore: {
        },
        QuickBuySlot_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_SoundOpvarSetOBBEntity: {
        },
        C_DOTA_Item_EyeOfTheVizier: {
        },
        C_DOTA_Item_Voodoo_Mask: {
        },
        CDOTA_Item_Enchanted_Mango: {
        },
        C_DOTA_Item_Recipe_Radiance: {
        },
        C_DOTA_Ability_DarkWillow_Bedlam: {
        },
        C_DOTA_Ability_Medusa_StoneGaze: {
        },
        CDOTA_Ability_Neutral_Upgrade: {
        },
        C_DOTA_Ability_HarpyStorm_ChainLightning: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lycan_1: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Phantom_Assassin: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Enigma_4: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Ember_Spirit_3: {
        },
        C_DOTA_Ability_Special_Bonus_Strength_6: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Speed_50: {
        },
        CPulseCell_WaitForPanelClass__CursorState_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CDOTA_Modifier_Item_PigletPole: {
        },
        CDOTA_Modifier_Item_Angels_Demise_Break: {
        },
        CDOTA_Modifier_Item_Quarterstaff: {
        },
        CDOTA_Modifier_Elder_Titan_Tip_The_Scales_Effect: {
        },
        CDOTA_Modifier_Rubick_FadeBoltBuff: {
        },
        CDOTA_Modifier_Undying_Tombstone_Zombie_Deathstrike_Slow: {
        },
        CDOTA_Modifier_Treant_LeechSeedPassive: {
        },
        CDOTA_Modifier_Lycan_SummonWolves_Maim: {
        },
        CDOTA_Modifier_Truesight_Aura: {
        },
        C_DOTA_Item_Recipe_Caster_Rapier: {
        },
        C_DOTA_Item_Ironwood_tree: {
        },
        CDOTA_Item_Recipe_MeteorHammer: {
        },
        C_DOTA_Item_Skadi: {
        },
        C_DOTA_Item_VitalityBooster: {
        },
        C_DOTA_Ability_DarkWillow_Terrorize: {
        },
        C_DOTA_Ability_Lina_DragonSlave: {
        },
        C_DOTA_Ability_CrystalMaiden_FreezingFieldStop: {
        },
        CDOTA_Item_BookStrength: {
        },
        CDOTA_Modifier_UpheavalUrn_Reincarnation: {
        },
        CDOTA_Modifier_PudgeMiniboss_HatefulStrike: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Chen_3: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Centaur_1: {
        },
        CDOTA_Modifier_Special_Bonus_Crit: {
        },
        C_DOTA_Ability_Special_Bonus_MP_Regen_1: {
        },
        CDOTA_Modifier_Item_Enhancement_Wise: {
        },
        CDOTA_Modifier_Item_The_Leveller: {
        },
        CDOTA_Modifier_Tome_Of_Knowledge: {
        },
        CDOTA_Modifier_Undying_Tombstone_Bunker_Cooldown: {
        },
        CDOTA_Modifier_Chen_HandOfGod_Invuln: {
        },
        CDOTA_Modifier_Lich_Sinister_Gaze_Cryophobia_Debuff: {
        },
        CDOTA_Modifier_SkeletonKing_BoneGuard: {
        },
        CDOTA_Modifier_Pudge_RottenCore: {
        },
        CDOTA_Modifier_Teleporting_Root_Logic: {
        },
        CTormentorSpawnPhase: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_SimpleObstruction: {
        },
        CAttributeContainer: {
        },
        C_DOTA_Unit_Hero_Bristleback: {
        },
        CDOTA_Unit_Announcer: {
        },
        C_ClientRagdoll: {
        },
        CDOTA_Item_Enchanted_Quiver: {
        },
        C_DOTA_Ability_Meepo_Ransack: {
        },
        C_DOTA_Ability_Sniper_Shrapnel: {
        },
        C_DOTA_Ability_Lich_DeathCharge: {
        },
        CDOTA_Modifier_Passive_Lotus_Pool_Building: {
        },
        CDOTA_Modifier_AghsFort_Blessings_Debuff_Duration_Increase: {
        },
        CDOTA_Ability_AghsFort_Ravage_Potion: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Grimstroke_3: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lycan_5: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lich_2: {
        },
        CDOTA_Modifier_BotChallenge_SkeletonKing_BoneGuard: {
        },
        CDOTA_Modifier_Item_AeonDisk: {
        },
        CDOTA_Modifier_Item_Javelin: {
        },
        CDOTA_Modifier_Primal_Beast_Trample_HasteAura: {
        },
        CDOTA_Modifier_Mars_ArenaOfBlood_SelfLeash: {
        },
        CDOTA_Modifier_MonkeyKing_QuadrupleTap_Counter: {
        },
        CDOTA_Modifier_Shredder_Flamethrower_TreeFire_Thinker: {
        },
        CDOTA_Modifier_Obsidian_Destroyer_Equilibrium_Debuff: {
        },
        CDOTA_Modifier_SpiritBreaker_PlanarPocket: {
        },
        CDOTA_Modifier_DarkSeer_Vacuum: {
        },
        CDOTA_Modifier_Kunkka_XMarksTheSpot: {
        },
        CDOTA_Modifier_Sand_King_BurrowStrike: {
        },
        CDOTA_Modifier_AncientApparition_ColdFeet: {
        },
        PulseSelectorOutflowList_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        C_DOTA_Unit_Hero_Legion_Commander: {
        },
        C_DOTA_Unit_Hero_Warlock: {
        },
        C_DOTA_Item_Gungir: {
        },
        C_DOTA_Item_Recipe_Disperser: {
        },
        C_DOTA_Ability__Primal_Beast_Colossal: {
        },
        C_DOTA_Ability_Skywrath_Mage_Concussive_Shot: {
        },
        C_DOTA_Ability_Treant_Overgrowth: {
        },
        C_DOTA_Ability_Invoker_Exort: {
        },
        C_DOTA_Ability_DrowRanger_CreepRally: {
        },
        C_DOTA_Ability_SatyrSoulstealer_ManaBurn: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Oracle_2: {
        },
        C_DOTA_Ability_Special_Bonus_Agility_30: {
        },
        C_DOTA_Ability_Special_Bonus_Agility_25: {
        },
        C_DOTA_Ability_Special_Bonus_MP_Regen_150: {
        },
        CShmupPanoramaFuncs: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        ClientQuickBuyItemState: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Modifier_Item_Gossamer_Cape: {
        },
        CDOTA_Modifier_Item_Buckler_Effect: {
        },
        CDOTA_Modifier_DarkWillow_Creature_Invulnerable: {
        },
        CDOTA_Modifier_Techies_RemoteMine: {
        },
        CDOTA_Modifier_Invoker_ColdSnap: {
        },
        CDOTA_Modifier_Huskar_Life_Break_Charge: {
        },
        CDOTA_Modifier_Enchantress_RabbleRouser: {
        },
        CDOTA_Modifier_Omniknight_Innate_HealingHammer: {
        },
        CDOTA_Modifier_Leshrac_ChronopticNourishment: {
        },
        CDOTA_Modifier_ShadowShaman_Shackles: {
        },
        CDOTA_Modifier_Morphling_Max_Attribute_Shift_Tracker: {
        },
        CDOTA_Modifier_Disable_Healing: {
        },
        CDOTA_Modifier_Filler_Heal_Aura: {
        },
        CPulseCell_PlaySequence__CursorState_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CBodyComponentSkeletonInstance: {
        },
        C_DOTA_Unit_Hero_Razor: {
        },
        C_DOTA_Item_Recipe_Soul_Booster: {
        },
        C_DOTA_Item_Recipe_RodOfAtos: {
        },
        C_DOTA_Ability_Terrorblade_Demon_Zeal: {
        },
        C_DOTA_Ability_Skywrath_Mage_Shield_Of_The_Scion: {
        },
        CDOTA_Ability_Keeper_of_the_Light_BrightSpeed: {
        },
        C_DOTA_Ability_Weaver_TimeLapse: {
        },
        C_DOTA_Ability_Leshrac_Defilement: {
        },
        C_DOTA_Ability_DragonKnight_DragonBlood: {
        },
        C_DOTA_Ability_Nian_Waterball: {
        },
        CDOTA_Ability_Roshan_Devotion: {
        },
        CDOTA_Ability_KoboldTunneler_Prospecting: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lycan_2: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Crystal_Maiden_1: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Centaur_3: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Amplify_14: {
        },
        CDOTA_Ability_Special_Bonus_Spell_AoE_100: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Speed_10: {
        },
        CDOTA_Modifier_BotChallenge_SkeletonKing_BoneGuard_Summon_Thinker: {
        },
        CDOTA_Modifier_Item_MaskOfMadness: {
        },
        CDOTA_Modifier_Ringmaster_Wheel_Mesmerize_Pull: {
        },
        CDOTA_Modifier_Muerta_Supernatural: {
        },
        CDOTA_Modifier_Centaur_Return: {
        },
        CDOTA_Modifier_Furion_Money_Tree: {
        },
        CDOTA_Modifier_Special_Bonus_Unique_Beastmaster_6_Aura: {
        },
        CDOTA_Modifier_PhantomLancer_Doppelwalk_Illusion: {
        },
        CDOTA_Modifier_Pudge_GraftFlesh: {
        },
        CDOTA_Modifier_AncientApparition_DeathRime: {
        },
        CDOTA_Modifier_Muted: {
        },
        CDOTA_Modifier_ScoutVisible: {
        },
        CDOTA_Item_Recipe_Havoc_Hammer: {
        },
        C_DOTA_Item_Iron_Talon: {
        },
        CDOTA_Ability_Elder_Titan_Momentum: {
        },
        CDOTA_Ability_Broodmother_Silken_Bola: {
        },
        C_DOTA_Ability_Firework_Mine: {
        },
        C_DOTA_Ability_Ursa_Overpower: {
        },
        CDOTA_Ability_Shadow_Shaman_Urnaconda: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Lina_3: {
        },
        C_DOTA_Ability_Juggernaut_BladeDance: {
        },
        CDOTA_Modifier_Greevil_Miniboss_Red_Overpower: {
        },
        CDOTA_Modifier_DoNotCastEnsnare: {
        },
        C_DOTA_Ability_AghsFort_Wave_Blast: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Underlord_8: {
        },
        C_DOTA_Ability_Special_Bonus_HP_400: {
        },
        CDOTA_Modifier_Item_Giants_Ring_Visual: {
        },
        CDOTA_Modifier_Havoc_Hammer_Slow: {
        },
        CDOTA_Modifier_Item_GlimmerCape_Building_Limit: {
        },
        CDOTA_Modifier_Item_Diffusal_Blade_Slow: {
        },
        CDOTA_Modifier_Marci_CompanionRun_Intrinsic: {
        },
        CDOTA_Modifier_ArcWarden_TempestDouble_Debuff: {
        },
        CDOTA_Modifier_LegionCommander_OverwhelmingOdds_Shield: {
        },
        CDOTA_Modifier_Ogre_Magi_Ignite_Multicast: {
        },
        CDOTA_Modifier_Brewmaster_BottomsUp_Passive: {
        },
        CDOTA_Modifier_Alchemist_GoblinsGreed: {
        },
        CDOTA_Modifier_Tinker_Turret_Drop_Thinker: {
        },
        CDOTA_Modifier_ShadowShaman_SerpentWardInvuln: {
        },
        CDOTA_Modifier_Windrunner_Tailwind: {
        },
        CDOTA_Modifier_Bloodseeker_BloodMist_Barrier: {
        },
        CDOTA_Modifier_Earthshaker_EnchantTotem: {
        },
        CDOTA_Modifier_DrowRanger_WaveOfSilence_Buff: {
        },
        CDOTA_Modifier_Bane_Enfeeble_Effect: {
        },
        C_DOTA_Unit_Undying_Tombstone: {
        },
        CDOTA_Item_The_Leveller: {
        },
        CDOTA_Item_Demonicon: {
        },
        C_DOTA_Item_Flicker: {
        },
        C_DOTA_Item_Recipe_Yasha: {
        },
        CDOTA_Ability_Abyssal_Underling_Warrior_LastWill: {
        },
        C_DOTA_Ability_Medusa_SplitShot: {
        },
        CDOTA_Ability_Shadow_Demon_Shadow_Servant: {
        },
        C_DOTA_Ability_FacelessVoid_Chronosphere: {
        },
        C_DOTA_Ability_Nian_Frenzy: {
        },
        C_DOTA_Item_Tombstone: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Amplify_11: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_16: {
        },
        C_DOTA_Ability_Special_Bonus_HP_Regen_4: {
        },
        CDOTA_Modifier_Item_Enhancement_Fleetfooted: {
        },
        CDOTA_Modifier_Item_Pipe: {
        },
        CDOTA_Modifier_Item_Sphere_Upgrade_Absorb: {
        },
        CDOTA_Modifier_Kez_Katana_Shard_Active: {
        },
        CDOTA_Modifier_Pangolier_Rollup: {
        },
        CDOTA_Modifier_AbyssalUnderlord_DarkRift: {
        },
        CDOTA_Modifier_Centaur_Mounted: {
        },
        CDOTA_Modifier_Jakiro_LiquidIce: {
        },
        CDOTA_Modifier_BountyHunter_Track_Bear_Trap: {
        },
        CDOTA_Modifier_Enchantress_NaturesAttendants_DamageTracker: {
        },
        CDOTA_Modifier_Rattletrap_Cog: {
        },
        CDOTA_Modifier_Courier_Flying: {
        },
        CDOTA_Modifier_Necrolyte_GhostShroud_Aura_Effect: {
        },
        CScriptComponent: {
        },
        C_DOTA_BaseNPC_Trap_Ward: {
        },
        C_DOTA_Ability_Pangolier_ShieldCrash: {
        },
        C_DOTA_Ability_MonkeyKing_Transform: {
        },
        C_DOTA_Ability_Bristleback_Bristleback: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Disruptor_2: {
        },
        C_DOTA_Ability_Invoker_ForgeSpirit_AD: {
        },
        CDOTA_Ability_DarkSeer_WallOfReplica: {
        },
        C_DOTA_Ability_Tidehunter_DeadInTheWater: {
        },
        C_DOTA_Ability_Sven_GodsStrength: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Tusk_6: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Invoker_9: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Luna_6: {
        },
        C_DOTA_Ability_Special_Bonus_Strength_25: {
        },
        CDOTA_Modifier_Item_Orb_Of_Frost: {
        },
        CDOTA_Modifier_Blight_Stone_Corruption: {
        },
        CDOTA_Modifier_PrimalBeast_Uproar: {
        },
        CDOTA_Modifier_Snapfire_FiresnapCookie_AllyFlailAnim: {
        },
        CDOTA_Modifier_MonkeyKing_TreeDance_Activity: {
        },
        CDOTA_Modifier_AbyssalUnderlord_PitOfMalice_Ensnare: {
        },
        CDOTA_Modifier_LegionCommander_DuelAura_Effect: {
        },
        CDOTA_Modifier_Abaddon_Mist_Coil_Damage_Penalty: {
        },
        CDOTA_Modifier_NagaSiren_Eelskin: {
        },
        CDOTA_Modifier_Nyx_Assassin_Nyxth_Sense: {
        },
        CDOTA_Modifier_Huskar_Life_Break: {
        },
        CDOTA_Modifier_DragonKnight_Fireball_Thinker: {
        },
        CDOTA_Modifier_Nevermore_Necromastery: {
        },
        CDOTA_Modifier_Juggernaut_Healing_Ward_Tracker: {
        },
        CDOTA_Modifier_Tutorial_Disable_Healing: {
        },
        C_PortraitWorldCallbackHandler: {
        },
        C_DynamicProp: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Batrider_1: {
        },
        C_DOTA_Ability_Omniknight_Hammer_Of_Purity: {
        },
        CDOTA_Ability_Puck_WaningRift: {
        },
        C_DOTA_Ability_Greevil_Miniboss_Yellow_Surge: {
        },
        C_DOTA_Ability_DarkTrollWarlord_RaiseDead: {
        },
        CDOTA_Modifier_Aghsfort_Enrage: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Silencer_7: {
        },
        C_DOTA_Ability_Special_Bonus_Evasion_15: {
        },
        C_DOTA_Ability_Special_Bonus_Magic_Resistance_35: {
        },
        C_DOTA_Ability_Special_Bonus_Movement_Speed_15: {
        },
        C_DOTA_Ability_Special_Bonus_HP_300: {
        },
        CDOTA_Modifier_Roshans_Banner_Effect: {
        },
        CDOTA_Modifier_Item_Book_Of_Shadows_Buff: {
        },
        CDOTA_Modifier_Item_Mechanical_Arm: {
        },
        CDOTA_Modifier_Item_EagleEye: {
        },
        CDOTA_Modifier_Techies_Minefield_Sign_Thinker: {
        },
        CDOTA_Modifier_Techies_StasisTrap_Stunned: {
        },
        CDOTA_Modifier_Obsidian_Destroyer_Mana_Allergy: {
        },
        CDOTA_Modifier_DragonKnight_WyrmsWrath: {
        },
        CDOTA_Modifier_WitchDoctor_DeathWard_Voodoo_Switcheroo_AttackSpeedReduction: {
        },
        CDOTA_Modifier_Bloodseeker_BloodMist: {
        },
        CDOTA_Modifier_Axe_CullingBlade_JungleWeaponGesture: {
        },
        C_DOTA_Unit_Hero_Jakiro: {
        },
        C_EnvDeferredLight: {
        },
        CDOTA_Item_Searing_Signet: {
        },
        C_DOTA_Item_BootsOfSpeed: {
        },
        C_DOTA_Ability_Lich_Sinister_Gaze: {
        },
        CDOTA_Ability_Frogmen_WaterBubble_Base: {
        },
        CDOTA_Modifier_EnragedWildkin_ToughnessAura_Bonus: {
        },
        CDOTA_Modifier_Ascension_AcidBlood: {
        },
        CDOTA_Modifier_Special_Bonus_Exp_Boost: {
        },
        CDOTA_Modifier_Item_WizardHat: {
        },
        CDOTA_Modifier_Cloak_Of_Flames_Debuff: {
        },
        CDOTA_Modifier_Item_Iron_Talon: {
        },
        CDOTA_Modifier_Muerta_TheCalling_Invulnerable: {
        },
        CDOTA_Modifier_Skywrath_Mage_Ruin_And_Restoration: {
        },
        CDOTA_Modifier_Tusk_WalrusPunch_AirTime: {
        },
        CDOTA_Modifier_Gyrocopter_Rocket_Barrage_Movespeed: {
        },
        CDOTA_Modifier_Weaver_GeminateAttack: {
        },
        CDOTA_Modifier_Broodmother_SpawnSpiderlings: {
        },
        CDOTA_Modifier_PhantomAssassin_StiflingDagger: {
        },
        CDOTA_Modifier_Morphling_Adaptive_Strike: {
        },
        CDOTA_Modifier_Bloodseeker_Rupture: {
        },
        CDOTA_Modifier_Earthshaker_Spirit_Cairn: {
        },
        CDOTA_Modifier_Sven_Wrath_Of_God: {
        },
        CDOTA_Modifier_Hidden_NoDamage: {
        },
        CIngameEvent_NewBloom2019: {
        },
        C_DOTA_Unit_Miniboss: {
        },
        CDOTA_Unit_CustomGameAnnouncerAghanim: {
        },
        CDOTA_Item_Falconers_Glove: {
        },
        CDOTA_Item_Recipe_SpecialistsArray: {
        },
        CDOTA_Item_Recipe_Panic_Button: {
        },
        CDOTA_Item_Broom_Handle: {
        },
        CDOTA_Item_ForceStaff: {
        },
        CDOTA_Ability_Grimstroke_SpiritWalk: {
        },
        C_DOTA_Ability_Legion_Commander_MomentOfCourage: {
        },
        C_DOTA_Ability_Batrider_Flamebreak: {
        },
        C_DOTA_Ability_Huskar_Inner_Fire: {
        },
        C_DOTA_Ability_Witch_Doctor_Innate_Voodoo_Jinx: {
        },
        C_DOTA_Ability_Tinker_HeatSeekingMissile: {
        },
        C_DOTA_Ability_Tornado_Tempest: {
        },
        CDOTA_Ability_Consumable_Hidden: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Brewmaster_4: {
        },
        C_DOTA_Ability_Special_Bonus_All_Stats_4: {
        },
        CDOTA_Modifier_Item_Psychic_Headband: {
        },
        CDOTA_Modifier_Item_Necronomicon_Mana_Aura: {
        },
        CDOTA_Modifier_Item_AbyssalBlade: {
        },
        CDOTA_Modifier_ArcWarden_TempestDouble: {
        },
        CDOTA_Modifier_Oracle_DivinersDeck_TheFool_Next: {
        },
        CDOTA_Modifier_Oracle_FalsePromiseTimer: {
        },
        CDOTA_Modifier_Medusa_StoneGaze_Slow: {
        },
        CDOTA_Modifier_Undying_Tombstone_Death_Trigger: {
        },
        CDOTA_Modifier_LoneDruid_SpiritBear_VFX: {
        },
        CDOTA_Modifier_Invoker_IceWall_SlowDebuff: {
        },
        CDOTA_Modifier_Nian_Attachment_Regrow: {
        },
        CDOTA_Modifier_Lich_Chain_Frost_Thinker: {
        },
        C_IngameEvent_WM2016: {
        },
        C_TextureBasedAnimatable: {
        },
        C_LightEnvironmentEntity: {
        },
        C_DOTA_NeutralSpawner: {
        },
        C_DOTA_Item_Terror_Mask: {
        },
        C_DOTA_Ability_Hoodwink_Sharpshooter: {
        },
        C_DOTA_Ability_Terrorblade_Metamorphosis: {
        },
        CDOTA_Ability_Gyrocopter_Side_Gunner: {
        },
        C_DOTA_Ability_Spectre_Dispersion: {
        },
        C_DOTA_Ability_Tiny_Avalanche: {
        },
        C_DOTA_Ability_SkeletonKing_HellfireBlast: {
        },
        C_DOTA_Ability_GnollAssassin_EnvenomedWeapon: {
        },
        CDOTA_Modifier_AghsFort_PlayerTransform: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Skywrath: {
        },
        C_DOTA_Ability_Special_Bonus_Status_Resistance_15: {
        },
        C_DOTA_Ability_Special_Bonus_Gold_lvl25_r: {
        },
        CDOTA_Modifier_Item_Dormant_Curio: {
        },
        CDOTA_Modifier_Spell_Prism: {
        },
        CDOTA_Modifier_Item_Pupils_gift: {
        },
        CDOTA_Modifier_Item_WraithPact_Thinker: {
        },
        CDOTA_Modifier_Item_RodOfAtos: {
        },
        CDOTA_Modifier_Kez_RavensVeil_Thinker: {
        },
        CDOTA_Modifier_MonkeyKing_ArcToGround: {
        },
        CDOTA_Modifier_DoomBringer_InfernalBlade: {
        },
        CDOTA_Modifier_Viper_Nose_Dive: {
        },
        CDOTA_Modifier_Rune_Regen: {
        },
        CDOTA_Modifier_Bonus_Armor: {
        },
        CDOTA_Survivors_UnitEntity: {
        },
        C_DOTA_Item_Cloak_Of_Flames: {
        },
        C_DOTA_Item_Recipe_Timeless_Relic: {
        },
        C_DOTA_Item_Boots_Of_Bearing: {
        },
        C_DOTA_Item_Recipe_Vladmir: {
        },
        C_DOTA_Item_Recipe_Bracer: {
        },
        C_DOTA_Ability_Marci_Special_Delivery: {
        },
        C_DOTA_Ability_TrollWarlord_SwitchStance: {
        },
        CDOTA_Ability_Viper_Nethertoxin: {
        },
        C_DOTA_Item_Lua: {
        },
        C_DOTA_Ability_PolarFurbolgUrsaWarrior_ThunderClap: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Enigma_3: {
        },
        C_DOTA_Ability_Special_Bonus_Evasion_30: {
        },
        C_DOTA_Ability_Special_Bonus_Strength_13: {
        },
        C_DOTA_Ability_Special_Bonus_HP_Regen_5: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Block_15: {
        },
        C_DOTA_Ability_Special_Bonus_MP_150: {
        },
        C_DOTA_Ability_Special_Bonus_HP_1000: {
        },
        CMatchTrackedStatsEntity: {
        },
        C_DOTAReflectionSkybox: {
        },
        CDOTA_Modifier_Item_Bloodstone_Aura: {
        },
        CDOTA_Modifier_Largo_AmphibianRhapsody_Ambient_Buff: {
        },
        CDOTA_Modifier_Shadow_Demon_Shadow_Servant: {
        },
        CDOTA_Modifier_FacelessVoid_TimeDilation_Distortion_Aura_Applicator: {
        },
        CDOTA_Modifier_Lion_ManaDrain: {
        },
        CDOTA_Modifier_Tower_Truesight_Aura: {
        },
        DestructiblePartDamageRequestAPI: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Unit_Hero_Grimstroke: {
        },
        C_DOTA_BaseNPC_Creep_Siege: {
        },
        C_DOTA_Item_Outworld_Staff: {
        },
        C_DOTA_Item_HelmOfTheOverlord: {
        },
        C_DOTA_Item_Recipe_Shivas_Guard: {
        },
        C_DOTA_Ability_Kez_GrapplingClaw: {
        },
        C_DOTA_Ability_Grimstroke_Ink_Over: {
        },
        C_DOTA_Ability_Gyrocopter_Rocket_Barrage: {
        },
        C_DOTA_Ability_Dazzle_Shallow_Grave: {
        },
        C_DOTA_Ability_TemplarAssassin_Trap: {
        },
        C_DOTA_Ability_AlphaWolf_CommandAura: {
        },
        CDOTA_Ability_MudGolem_HurlBoulder: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Necrophos: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Phantom_Lancer_4: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_100: {
        },
        C_DOTA_Ability_Special_Bonus_Intelligence_75: {
        },
        C_FoWBlockerEntity: {
        },
        CDOTA_Modifier_Foragers_Kit_Thinker: {
        },
        CDOTA_Modifier_Item_Helm_Of_The_Undying: {
        },
        CDOTA_Modifier_Muerta_Ofrenda_Invulnerable: {
        },
        CDOTA_Modifier_VoidSpirit_AetherRemnantUnit_Truesight: {
        },
        CDOTA_Modifier_NagaSiren_RipTideCounter: {
        },
        CDOTA_Modifier_QueenOfPain_SonicWave_Delay: {
        },
        CDOTA_Modifier_ShadowShaman_SerpentWard: {
        },
        CDOTA_Modifier_Mirana_Nightveil: {
        },
        CLogicRelayAPI: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_BaseNPC_LotusPool: {
        },
        C_TriggerPhysics: {
        },
        C_PropDoorRotating: {
        },
        CDOTA_Item_Recipe_Eagle_Eye: {
        },
        C_DOTA_Item_BlinkDagger: {
        },
        C_DOTA_Ability_Phoenix_Dying_Light: {
        },
        C_DOTA_Ability_Shredder_ReturnChakramAlias_shredder_return_chakram_2: {
        },
        C_DOTA_Ability_TrollWarlord_Whirling_Axes_Melee: {
        },
        C_DOTA_Ability_Slark_Pounce: {
        },
        CDOTA_Ability_TemplarAssassin_InnerPeace: {
        },
        C_DOTA_Ability_StormSpirit_Overload: {
        },
        CDOTA_Modifier_Creep_Irresolute: {
        },
        CDOTA_Modifier_Launchpad: {
        },
        CInfoWorldLayer: {
        },
        CDOTA_Modifier_AghsFort_TrapRoom_MeatHook: {
        },
        CDOTA_Modifier_AghsFort_TorrentEffectPotion_Torrent: {
        },
        C_DOTA_Item_UpgradedBarricade: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Warlock_10: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Axe: {
        },
        C_DOTA_Ability_Special_Bonus_Movement_Speed_75: {
        },
        CDOTA_Modifier_Item_Dynamite_Jacket: {
        },
        CDOTA_Modifier_Item_Crown: {
        },
        CDOTA_Modifier_Item_Ancient_Janggo: {
        },
        CDOTA_Modifier_Item_Pipe_Debuff: {
        },
        CDOTA_Modifier_Item_Radiance_Debuff: {
        },
        CDOTA_Modifier_Item_MaskOfDeath: {
        },
        CDOTA_Modifier_Disruptor_KineticField_TouchSlow: {
        },
        CDOTA_Modifier_Lycan_SummonWolves_CriticalStrike: {
        },
        CDOTA_Modifier_Dazzle_Bad_Juju_Manacost: {
        },
        CDOTA_Modifier_Pugna_DrainSouls: {
        },
        CDOTA_Modifier_Lion_Arcana_Kill_Effect: {
        },
        CDOTA_Modifier_Morphling_Replicate: {
        },
        CDOTA_Modifier_Lina_LagunaBlade_Line: {
        },
        CDOTA_Modifier_Razor_StormSurge_Slow: {
        },
        CDOTA_Modifier_Pudge_Rot: {
        },
        CBodyComponentBaseModelEntity: {
        },
        C_DOTA_Ability_Kez_RavensVeil: {
        },
        CDOTA_Ability_Marci_Companion_Run: {
        },
        C_DOTA_Ability_Dawnbreaker_Unbreakable: {
        },
        C_DOTA_Ability_Gyrocopter_Afterburner: {
        },
        CDOTA_Ability_Night_Stalker_MidnightFeast: {
        },
        CDOTA_Ability_Tinker_DeployTurrets: {
        },
        C_DOTA_Ability_Frogmen_Riverborn_Aura: {
        },
        CDOTA_Ability_AghsFort_Morphling_Waveform: {
        },
        CDOTA_Ability_Seasonal_TI9_Monkey: {
        },
        CDOTA_Modifier_Special_Bonus_Attack_Damage: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Ember_Spirit_4: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_6: {
        },
        C_DOTA_Ability_Special_Bonus_Cast_Range_300: {
        },
        CDOTA_Modifier_Item_SpecialistsArray_ProcDamage: {
        },
        CDOTA_Modifier_Item_Penta_Edged_Sword: {
        },
        CDOTA_Modifier_Ringmaster_FunhouseMirror_Phase: {
        },
        CDOTA_Modifier_Primal_Beast_Innate_Slow_Resistance_Per_Time: {
        },
        CDOTA_Modifier_Visage_Amor_Reduction: {
        },
        CDOTA_Modifier_KeeperOfTheLight_BlindingLight_Thinker: {
        },
        CDOTA_Modifier_Lycan_Apex_Predator: {
        },
        CDOTA_Modifier_DoomBringer_ScorchedEarthEffect: {
        },
        CDOTA_Modifier_Death_Prophet_Attack_Scepter: {
        },
        CDOTA_Modifier_Tinker_Repair_Bots: {
        },
        CDOTA_Modifier_StormSpirit_Overload_Attack_Speed_Buff: {
        },
        CDOTA_Modifier_Sven_GodsStrength: {
        },
        CDOTA_Modifier_Bane_FiendsGrip_Self: {
        },
        CDOTA_Modifier_No_Invisibility: {
        },
        C_DOTA_Unit_Hero_Ogre_Magi: {
        },
        C_BaseTrigger: {
        },
        C_DOTA_Ability_Techies_Suicide: {
        },
        C_DOTA_Ability_Nian_Sigils: {
        },
        C_DOTA_Ability_Lina_Flame_Cloak: {
        },
        C_DOTA_Ability_Mirana_SolarFlare: {
        },
        C_DOTA_Ability_SandKing_SandStorm: {
        },
        CDOTA_Modifier_Frogmen_ArmOfTheDeep_Stun: {
        },
        CDOTA_Ability_AghsFort_Arcanist_Potion: {
        },
        CDOTA_Modifier_Special_Bonus_Cast_Range: {
        },
        C_DOTA_Ability_Special_Bonus_HP_Regen_30: {
        },
        CPulsePanoramaFuncs: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Modifier_Item_Disperser: {
        },
        CDOTA_Modifier_Item_MeteorHammer: {
        },
        CDOTA_Modifier_Dawnbreaker_BreakOfDawn: {
        },
        CDOTA_Modifier_MonkeyKing_UnperchedStunned: {
        },
        CDOTA_Modifier_Techies_Squees_Scope: {
        },
        CDOTA_Modifier_Techies_StickyBomb_Countdown: {
        },
        CDOTA_Modifier_Spectre_SpectralDaggerPathPhased: {
        },
        CDOTA_Modifier_BountyHunter_TrackGold: {
        },
        CDOTA_Modifier_Broodmother_SpinWeb_NoTreeWalking: {
        },
        CDOTA_Modifier_Life_Stealer_Ghoul_Frenzy_Slow: {
        },
        CDOTA_Modifier_Death_Prophet_MourningRitual: {
        },
        C_DOTA_Unit_Hero_PhantomAssassin: {
        },
        CDOTA_BaseNPC_Seasonal_Penguin: {
        },
        C_DynamicPropClientFadeOut: {
        },
        FilterDamageType: {
        },
        C_DOTA_Ability_LoneDruid_SavageRoar: {
        },
        C_DOTA_Ability_Sniper_TakeAim: {
        },
        CDOTA_Ability_Bloodseeker_Rupture: {
        },
        C_DOTA_Ability_Sven_Vanquisher: {
        },
        C_DOTA_Ability_DeathProphet_Exorcism: {
        },
        CDOTA_Ability_AghsFort_Creature_Impale: {
        },
        CDOTA_Modifier_Special_Bonus_Movement_Speed: {
        },
        C_DOTA_Ability_Special_Bonus_Lifesteal_40: {
        },
        C_DOTA_Ability_Special_Bonus_Agility_40: {
        },
        CDOTA_Modifier_Item_Force_Field_Bonus_Aura: {
        },
        CDOTA_Modifier_Item_AeonDisk_Buff: {
        },
        CDOTA_Modifier_Item_Sphere: {
        },
        CDOTA_Modifier_Snapfire_LilShredder_Debuff: {
        },
        CDOTA_Modifier_Mars_Scepter_Damage: {
        },
        CDOTA_Modifier_Abaddon_Frostmourne_Buff: {
        },
        CDOTA_Modifier_Disruptor_Thunder_Strike: {
        },
        CDOTA_Modifier_Invoker_IceWall_AllyBuffAura: {
        },
        CDOTA_Modifier_PhantomLancer_PhantomEdge_Debuff: {
        },
        CDOTA_Modifier_Mirana_SelemenesFaithful: {
        },
        CAttributeList: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CPulseCell_Inflow_Wait: {
        },
        C_DOTA_Unit_Scout: {
        },
        CDOTA_Item_Hurricane_Pike: {
        },
        CDOTA_Ability_Techies_ReactiveTazer: {
        },
        C_DOTA_Ability_Furion_WrathOfNature: {
        },
        C_DOTA_Ability_Nian_Eruption: {
        },
        C_DOTA_Item_Miniboss_Minion_Summoner: {
        },
        C_DOTA_Ability_Windrunner_Powershot: {
        },
        CDOTA_Modifier_BigThunderLizard_Frenzy: {
        },
        C_DOTA_Ability_SatyrTrickster_Purge: {
        },
        CFilterProximity: {
        },
        CDOTA_Ability_Seasonal_TI11_BubbleGun: {
        },
        CDOTA_Modifier_Special_Bonus_Movement_Speed_Percentage: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Ancient_Apparition_2: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Range_175: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Base_Damage_25: {
        },
        C_DOTA_Ability_Special_Bonus_MP_Regen_125: {
        },
        CTeamTrackedStatsEntity: {
        },
        CDOTA_Modifier_Hoodwink_AcornShot_ArmorCorruption: {
        },
        CDOTA_Modifier_Treant_NaturesGrasp_Creation_Thinker: {
        },
        CDOTA_Modifier_Meepo_Ransack: {
        },
        CDOTA_Modifier_LoneDruid_SpiritBear_Fetch_Self: {
        },
        CDOTA_Modifier_Roshan_SpellBlock: {
        },
        CDOTA_Modifier_FountainPassive: {
        },
        CDOTA_Modifier_Enigma_Event_Horizon_Aura: {
        },
        CEffectData: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Unit_Hero_EarthSpirit: {
        },
        C_ParticleSystem: {
        },
        CDOTA_Ability_Kez_Echo_Slash: {
        },
        C_DOTA_Ability_Primal_Companion: {
        },
        CDOTA_Ability_Beastmaster_PrimalRoar: {
        },
        CDOTA_Ability_Seasonal_TI11_Scissors: {
        },
        C_DOTA_Ability_Morty_Hop_Launch: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Windranger_3: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Storm_Spirit_7: {
        },
        C_DOTA_Ability_Special_Bonus_Movement_Speed_40: {
        },
        C_DOTA_Ability_Special_Bonus_MP_Regen_175: {
        },
        C_DOTA_Ability_Special_Bonus_TrueStrike: {
        },
        CDOTA_Modifier_Item_SuperOverwhelming_Blink: {
        },
        CDOTA_Modifier_Force_Boots: {
        },
        CDOTA_Modifier_Marci_Lunge_Debuff: {
        },
        CDOTA_Modifier_Pangolier_Gyroshell_Ricochet: {
        },
        CDOTA_Modifier_Huskar_Berserkers_Blood_Aura: {
        },
        CDOTA_Modifier_Enchantress_Impetus: {
        },
        CDOTA_Modifier_Enchantress_NaturesAttendants: {
        },
        CDOTA_Modifier_Omniknight_GuardianAngel_Aura: {
        },
        CDOTA_Modifier_Clinkz_BurningBarrage: {
        },
        CDOTA_Modifier_Dark_Seer_Quick_Wit: {
        },
        CDOTA_Modifier_Furion_Natures_Profit: {
        },
        CDOTA_Modifier_Viper_CorrosiveSkin_Slow: {
        },
        CDOTA_Modifier_FacelessVoid_TimeLock_ForceProc: {
        },
        CDOTA_Modifier_QueenOfPain_ScreamOfPain_Fear: {
        },
        CDOTA_Modifier_Nian_Invulnerable: {
        },
        CDOTA_Modifier_Miniboss_Radiance_Debuff: {
        },
        CDOTA_Modifier_Enigma_Innate_EventHorizonAura: {
        },
        CDOTA_Modifier_Tiny_TossTree_Slow: {
        },
        CPulseCell_Outflow_CycleShuffled: {
        },
        CDOTA_Unit_Hero_ArcWarden: {
        },
        C_DOTA_Unit_Hero_Juggernaut: {
        },
        C_DOTA_Item_Cornucopia: {
        },
        CDOTA_Item_RiverPainter7: {
        },
        C_DOTA_Item_HelmOfTheDominator_2: {
        },
        C_DOTA_Item_OblivionStaff: {
        },
        C_DOTA_Item_MysticStaff: {
        },
        C_DOTA_Ability_Ogre_Magi_Multicast: {
        },
        C_DOTA_Ability_SpiritBreaker_HerdMentality: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Phantom_Lancer_2: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Pangolier_5: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Amplify_18: {
        },
        C_DOTA_Ability_Special_Bonus_HP_600: {
        },
        CDOTA_Modifier_Terrorblade_Reflection_Invulnerability: {
        },
        CDOTA_Modifier_Slark_ShadowDance_PassiveRegen: {
        },
        CDOTA_Modifier_Disruptor_KineticFieldThinker: {
        },
        CDOTA_Modifier_Undying_SoulRip_Share_Strength: {
        },
        CDOTA_Modifier_Alchemist_ChemicalRage: {
        },
        CDOTA_Modifier_Rattletrap_Jetpack_Tracker: {
        },
        CDOTA_Modifier_Warlock_Imp_AutoAttack: {
        },
        CDOTA_Modifier_Earthshaker_Arcana_Kill_Effect: {
        },
        CDOTA_Modifier_StormSpirit_Overload: {
        },
        CDOTA_Modifier_Item_Mask_Crit_Lifesteal_Consumed: {
        },
        CDOTA_Modifier_TempAttackSpeedBoosted: {
        },
        C_DOTA_Unit_Hero_CrystalMaiden: {
        },
        C_FuncMover: {
        },
        C_DOTA_Item_RuneSpawner_Bounty: {
        },
        C_DOTA_Item_Enhancement_Timelss: {
        },
        C_DOTA_Item_Recipe_SangeAndYasha: {
        },
        C_DOTA_Ability_Oracle_Clairvoyant_Cure: {
        },
        CDOTA_Ability_Shadow_Demon_Menace: {
        },
        C_DOTA_Ability_Clinkz_DeathPact: {
        },
        C_DOTA_Ability_Shadow_Shaman_Voodoo_Hands: {
        },
        C_DOTA_Ability_Puck_PhaseShift: {
        },
        C_DOTA_Ability_Morphling_Waveform: {
        },
        CDOTA_Modifier_Legion_Commander_Duel: {
        },
        CDOTA_Ability_Seasonal_Summon_TI9_Balloon: {
        },
        CDOTA_Modifier_Special_Bonus_Gold: {
        },
        C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_14: {
        },
        C_DOTA_Ability_Special_Bonus_Armor_12: {
        },
        C_DOTA_Ability_Special_Bonus_Mana_Reduction_9: {
        },
        CDOTA_Modifier_DarkWillow_Terrorize_Thinker: {
        },
        CDOTA_Modifier_NagaSiren_SongOfTheSiren: {
        },
        CDOTA_Modifier_Brewmaster_Pulverize: {
        },
        CDOTA_Modifier_SpiritBreaker_EmpoweringHaste: {
        },
        CDOTA_Modifier_Clinkz_Tar_Bomb_SearingArrows: {
        },
        CLightComponent: {
        },
        CDOTA_Ability_Grimstroke_DarkArtistry: {
        },
        C_DOTA_Ability_Oracle_DivinersDeck: {
        },
        C_DOTA_Ability_Oracle_RainOfDestiny: {
        },
        C_DOTA_Ability_Spectre_SpectralDagger: {
        },
        C_DOTA_Ability_Ursa_Enrage: {
        },
        C_DOTA_Ability_PhantomLancer_Doppelwalk: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Snapfire_8: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Snapfire_6: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_70: {
        },
        C_DOTA_Ability_Special_Bonus_Cast_Range_250: {
        },
        CDOTAPlayer_CameraServices: {
        },
        CDOTA_Modifier_DarkWillow_BrambleMaze_Thinker: {
        },
        CDOTA_Modifier_Tusk_FrozenSigil: {
        },
        CDOTA_Modifier_NagaSiren_RipTide: {
        },
        CDOTA_Modifier_Shadow_Demon_Soul_Catcher_Illusion: {
        },
        CDOTA_Modifier_Earthshaker_Fissure_Stun: {
        },
        CDOTA_Modifier_Nevermore_FeastOfSouls_Collection: {
        },
        CDOTA_Modifier_Item_Mask_Crit_Lifesteal: {
        },
        CDOTA_Modifier_PersistentInvisibility: {
        },
        C_IngameEvent_DotaPrime: {
        },
        C_DOTA_Item_WizardHat: {
        },
        C_DOTA_Ability_Dawnbreaker_Land: {
        },
        C_DOTA_Ability_NagaSiren_SongOfTheSiren_Cancel: {
        },
        C_DOTA_Ability_Ogre_Magi_School: {
        },
        C_DOTA_Ability_Faceless_Void_Innate_Time_Walk_Backtrack_Duration: {
        },
        CDOTA_Modifier_ContextualTips: {
        },
        C_EnvCubemap: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Witch_Doctor_2: {
        },
        CDOTA_Modifier_Item_FlayersBota: {
        },
        CDOTA_Modifier_Item_Rattlecage_Slow: {
        },
        CDOTA_Modifier_Psychic_Headband: {
        },
        CDOTA_Modifier_Item_ConsecratedWraps_Speed_Buff: {
        },
        CDOTA_Modifier_Wisp_Spirits_Slow: {
        },
        CDOTA_Modifier_Chen_HolyPersuasion_Intrinsic: {
        },
        CDOTA_Modifier_Life_Stealer_Open_Wounds_Attack_Speed_Bonus: {
        },
        CDOTA_Modifier_Enigma_Black_Hole_Pull_Scepter: {
        },
        CDOTA_Modifier_Antimage_DampenMagic_Aura: {
        },
        CDOTA_Modifier_BackdoorProtectionInBase: {
        },
        CBodyComponent: {
        },
        CPulseCell_Inflow_Method: {
        },
        C_DOTA_Wisp_Spirit: {
        },
        C_DOTA_Unit_Hero_Nyx_Assassin: {
        },
        C_DOTA_MinibossSpawner: {
        },
        C_DOTA_Item_PocketTower: {
        },
        C_DOTA_Item_MonkeyKingBar: {
        },
        C_DOTA_Ability_Phoenix_FireSpirits: {
        },
        C_DOTA_Ability_EmberSpirit_Activate_FireRemnant: {
        },
        CDOTA_Ability_KeeperOfTheLight_SpiritForm: {
        },
        CDOTA_Ability_DarkSeer_IonShell: {
        },
        C_DOTA_Ability_Earthshaker_EchoSlam: {
        },
        CDOTA_Modifier_HillTroll_RallyAura: {
        },
        CDOTA_Modifier_Aghsfort_Pugna_Grandmaster_NetherWard: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Omniknight_7: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Ancient_Apparition_5: {
        },
        C_DOTA_Ability_Special_Bonus_Cast_Range_60: {
        },
        C_DOTA_Ability_Special_Bonus_Agility_And_Intelligence_6: {
        },
        CDOTA_Modifier_Item_Essence_Ring_Active: {
        },
        CDOTA_Modifier_Item_Princes_Knife: {
        },
        CDOTA_Modifier_MonkeyKing_BoundlessStrike_ShardMovement: {
        },
        CDOTA_Modifier_Terrorblade_Metamorphosis_Transform_Aura: {
        },
        CDOTA_Modifier_AbyssalUnderlord_PitOfMalice_Slow: {
        },
        CDOTA_Modifier_Tusk_Drinking_Buddies_Pull: {
        },
        CDOTA_Modifier_Slark_Pounce_ChargeCounter: {
        },
        CDOTA_Modifier_Undying_FleshGolem_Intrinsic: {
        },
        CDOTA_Modifier_Meepo_MegaMeepo: {
        },
        CDOTA_Modifier_Shadow_Demon_Disseminate: {
        },
        CDOTA_Modifier_Invoker_EMP_Pull: {
        },
        CDOTA_Modifier_Viper_PoisonAttack: {
        },
        CDOTA_Modifier_Miniboss_Minion_Deflecting_Shield_Aura: {
        },
        CDOTA_Modifier_Kunkka_Tidebringer: {
        },
        CDOTA_Modifier_Item_BotChallenge_MeteorStaff_Land: {
        },
        CDOTA_Modifier_MonkeyKing_BouncePerch: {
        },
        C_DOTA_UnitInventory: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_BaseCombatCharacter: {
        },
        C_DOTATeam: {
        },
        C_DOTA_Item_SplintMail: {
        },
        C_DOTA_Ability_Rubick_FadeBolt: {
        },
        C_DOTA_Ability_Omniknight_Repel: {
        },
        C_DOTA_Ability_Morphling_Flow: {
        },
        CDOTA_Modifer_Furbolg_Enrage_AttackSpeed: {
        },
        CDOTA_Modifier_PineCone_ShieldBash_Crit: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Undying_5: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Bloodseeker_4: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Sniper_4: {
        },
        C_DOTA_Ability_Special_Bonus_Evasion_10: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Speed_15: {
        },
        CDOTA_Modifier_Item_Ethereal_Blade: {
        },
        CDOTA_Modifier_EarthSpirit_Petrify: {
        },
        CDOTA_Modifier_EmberSpirit_FireRemnant_ChargeCounter: {
        },
        CDOTA_Modifier_Tusk_FrozenSigil_Aura: {
        },
        CDOTA_Modifier_Bristleback_Active_ConicalQuillSpray: {
        },
        CDOTA_Modifier_Shredder_Flamethrower_Damage: {
        },
        CDOTA_Modifier_Invoker_ChaosMeteor_Land: {
        },
        CDOTA_Modifier_Invoker_Instance: {
        },
        CDOTA_Modifier_Broodmother_InsatiableHunger_Aura: {
        },
        CDOTA_Modifier_Broodmother_SpawnSpiderlingsShard: {
        },
        CDOTA_Modifier_Omniknight_Hammer_Of_Purity_Bomb: {
        },
        CDOTA_Modifier_Dazzle_Poison_Touch_Self: {
        },
        CDOTA_Modifier_JumpBootsIntrinsic: {
        },
        CDOTA_Modifier_Mirana_Starfall_Thinker: {
        },
        CDOTA_Modifier_Nevermore_FeastOfSouls_Linger: {
        },
        CDOTA_Modifier_HallOfFame_Glow: {
        },
        CRoshanPhaseInfo: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CGlowProperty: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Item_Enhancement_Dominant: {
        },
        C_DOTA_Item_Recipe_Wind_Waker: {
        },
        C_DOTA_Item_Orb_of_Venom: {
        },
        C_DOTA_Item_Recipe_Dagon4: {
        },
        C_DOTA_Ability_BountyHunter_Jinada: {
        },
        C_DOTA_Ability_Necrolyte_GhostShroud: {
        },
        C_DOTA_Ability_SkeletonKing_Reincarnation: {
        },
        CDOTA_Modifier_Mutation_CreateTombstone_Team_Aura: {
        },
        CDOTA_Ability_Capture: {
        },
        C_PointClientUIDialog: {
        },
        C_DotaQuest: {
        },
        C_DOTA_Ability_AghsFort_ShadowShaman_Shackles: {
        },
        CDOTA_Ability_Ascension_Bulwark: {
        },
        CDOTA_Modifier_Muerta_SpectralSlugEthereal: {
        },
        CDOTA_Modifier_Snapfire_Magma_Thinker: {
        },
        CDOTA_Modifier_Mars_ArenaOfBlood_VisionObstruction: {
        },
        CDOTA_Modifier_Terrorblade_Fear: {
        },
        CDOTA_Modifier_Terrorblade_ConjureImage: {
        },
        CDOTA_Modifier_Undying_FleshGolem_PlagueAura: {
        },
        CDOTA_Modifier_Furion_WrathOfNature_Buff: {
        },
        CPulseCell_BaseValue: {
        },
        C_DOTA_Item_Seer_Stone: {
        },
        CDOTA_Item_RiverPainter4: {
        },
        C_DOTA_Item_RefresherOrb: {
        },
        C_DOTA_Item_Broadsword: {
        },
        C_DOTA_Ability_Disruptor_Glimpse: {
        },
        C_DOTA_Ability_Meepo_Petrify: {
        },
        C_DOTA_Ability_Brewmaster_CinderBrew: {
        },
        CDOTA_Modifier_Creep_Siege: {
        },
        C_DOTA_Ability_OgreBruiser_OgreSmash: {
        },
        CCitadelSoundOpvarSetOBB: {
        },
        CDOTA_Modifier_AghsFort_TreantMiniboss_NaturesGuise: {
        },
        CDOTA_Ability_AghsFort_Tower_BlastWave: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Underlord_5: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lycan_4: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Bristleback: {
        },
        C_DOTA_Ability_Special_Bonus_Evasion_20: {
        },
        C_DOTA_Ability_Special_Bonus_Respawn_Reduction_20: {
        },
        CDOTA_Modifier_Item_Doubloon: {
        },
        CDOTA_Modifier_Item_Terror_Mask: {
        },
        CDOTA_Modifier_Mars_ArenaOfBlood_Thinker: {
        },
        CDOTA_Modifier_Slark_Fish_Bait_Post: {
        },
        CDOTA_Modifier_Alchemist_Berserk_Potion: {
        },
        CDOTA_Modifier_Spectre_Reality: {
        },
        CDOTA_Modifier_DeathProphet_CryptSwarm_Slow: {
        },
        CDOTA_Modifier_StormSpirit_Galvanized: {
        },
        CDOTA_Modifier_Rune_Illusion: {
        },
        CPlayer_WaterServices: {
        },
        CPulseCell_BooleanSwitchState: {
        },
        C_DOTA_Unit_Hero_Sniper: {
        },
        C_DOTA_Item_Enhancement_Vast: {
        },
        C_DOTA_Item_Consecrated_Wraps: {
        },
        C_DOTA_Item_OrchidMalevolence: {
        },
        C_DOTA_Item_Shawl: {
        },
        C_DOTA_Ability_EarthSpirit_Magnetize: {
        },
        C_DOTA_Ability_Rubick_Arcane_Supremacy: {
        },
        C_DOTA_Ability_Huskar_Inner_Vitality: {
        },
        C_DOTA_Ability_PhantomAssassin_Stifling_Dagger: {
        },
        CDOTA_Ability_Miniboss_Minion_Deflecting_Shield: {
        },
        C_DOTA_Ability_DataDriven: {
        },
        CDOTA_Modifier_Furbolg_Enrage_AttackSpeed_OnDeath: {
        },
        C_DOTA_Ability_AghsFort_Shadow_Demon_Shadow_Poison: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Dark_Seer_5: {
        },
        C_DOTA_Ability_Special_Bonus_Evasion_16: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_15: {
        },
        CDOTA_Modifier_Orb_Of_Revelations_Reveal: {
        },
        CDOTA_Modifier_Necronomicon_Warrior_LastWill: {
        },
        CDOTA_Modifier_Item_Necronomicon_3: {
        },
        CDOTA_Modifier_Item_Shivas_Guard: {
        },
        CDOTA_Modifier_AbyssalUnderlord_Underling_Autoattack: {
        },
        CDOTA_Modifier_Shredder_ReactiveArmor_Bomb: {
        },
        CDOTA_Modifier_TrollWarlord_BerserkersRage_Ensnare: {
        },
        CDOTA_Modifier_Invoker_IceWall_Thinker: {
        },
        CDOTA_Modifier_Broodmother_PoisonSting: {
        },
        CDOTA_Modifier_Beastmaster_SummonRaptor_Summoning: {
        },
        CDOTA_Modifier_Ursa_Earthshock: {
        },
        CDOTA_Modifier_Tinker_Defensive_Matrix: {
        },
        CDOTA_Modifier_Mirana_MoonlightShadow: {
        },
        C_IngameEvent_DotaPlus: {
        },
        CDOTA_PlayerChallengeInfo: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        VPhysicsCollisionAttribute_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Unit_Hero_DarkWillow: {
        },
        C_DOTA_Unit_Hero_Pangolier: {
        },
        C_DOTA_BaseNPC_Tusk_Sigil: {
        },
        C_DOTA_Unit_Hero_DarkSeer: {
        },
        C_DOTA_Unit_Hero_Pugna: {
        },
        C_DOTA_NPC_Lich_Ice_Spire: {
        },
        C_PlayerCosmeticPropClientside: {
        },
        C_DynamicPropAlias_dynamic_prop: {
        },
        CDOTA_Item_Rabbits_Foot: {
        },
        C_DOTA_Item_Mechanical_Arm: {
        },
        CDOTA_Item_Recipe_Crimson_Guard: {
        },
        C_DOTA_Ability_Brewmaster_SpellImmunity: {
        },
        C_DOTA_Ability_Tiny_Insurmountable: {
        },
        C_DOTA_Ability_Juggernaut_Duelist: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_DarkWillow_1: {
        },
        CDOTA_Modifier_Special_Bonus_Evasion: {
        },
        C_DOTA_Ability_Special_Bonus_MP_800: {
        },
        C_DOTA_Ability_Special_Bonus_MP_225: {
        },
        C_DOTACheers: {
        },
        CDOTA_Modifier_Dragon_Scale_Burn: {
        },
        CDOTA_Modifier_Item_Desolator_2: {
        },
        CDOTA_Modifier_Tome_Of_Knowledge_Consumed: {
        },
        CDOTA_Modifier_GlimmerCape_Fade: {
        },
        CDOTA_Modifier_Tenderizer_Weaken: {
        },
        CDOTA_Modifier_Item_ObserverWard: {
        },
        CDOTA_Modifier_Snapfire_GobbleUp_BellyHasUnit: {
        },
        CDOTA_Modifier_Mars_Dauntless_Effect: {
        },
        CDOTA_Modifier_Tusk_WalrusKick_AirTime: {
        },
        CDOTA_Modifier_Visage_Silent_As_The_Grave_Bonus: {
        },
        CDOTA_Modifier_LoneDruid_SavageRoar: {
        },
        CDOTA_Modifier_Gyrocopter_Flak_Cannon: {
        },
        CDOTA_Modifier_Nian_WhirlpoolThinker: {
        },
        CDOTA_Modifier_Lich_FrostShield: {
        },
        CDOTA_Modifier_Morphling_Morph_Str: {
        },
        CDOTA_Modifier_PhantomLancer_PhantomEdge: {
        },
        CDOTA_Modifier_VengefulSpirit_Command_Negative_Aura_Effect: {
        },
        CDOTA_Modifier_DrowRanger_FrostArrows_Slow: {
        },
        CDOTA_Modifier_Sand_King_Scorpion_Strike: {
        },
        CDOTA_Modifier_TrueSightFoW: {
        },
        CDOTA_Unit_Hero_Gyrocopter: {
        },
        CEnvSoundscapeProxyAlias_snd_soundscape_proxy: {
        },
        CDOTA_Item_Havoc_Hammer: {
        },
        C_DOTA_Item_Necronomicon_Level2: {
        },
        C_DOTA_Item_Recipe_Ring_Of_Basilius: {
        },
        C_DOTA_Ability_MonkeyKing_QuadrupleTap: {
        },
        C_DOTA_Ability_Meepo_Megameepo_Fling: {
        },
        CDOTA_Modifier_BlueDragonspawnSorcerer_Evasion: {
        },
        C_SceneEntity: {
        },
        CDOTA_Modifier_PudgeMiniboss_ArmorCorruption: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Drow_Ranger_7: {
        },
        C_SpeechBubbleManager: {
        },
        CDOTA_Modifier_Item_SistersShroud: {
        },
        CDOTA_Modifier_Item_Mirror_Shield: {
        },
        CDOTA_Modifier_Royale_With_Cheese: {
        },
        CDOTA_Modifier_Item_Necronomicon_Mana_Aura_3: {
        },
        CDOTA_Modifier_Item_Assault_Cuirass_Negative_Armor_Aura: {
        },
        CDOTA_Modifier_Bristleback_BrawlersGrit: {
        },
        CDOTA_Modifier_Invoker_ChaosMeteor_Burn: {
        },
        CDOTA_Modifier_Huskar_Inner_Fire_Knockback: {
        },
        CDOTA_Modifier_Miniboss_Reflect: {
        },
        CDOTA_Modifier_Windrunner_Windrun_Invis: {
        },
        CDOTA_Modifier_DrowRanger_Marksmanship: {
        },
        CDOTA_Modifier_SkeletonKing_BoneGuard_Summon_Thinker: {
        },
        CIngameEvent_MonsterHunter: {
        },
        CPulseCell_Inflow_Yield: {
        },
        CPulseMathlib: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Unit_Hero_Enchantress: {
        },
        C_DOTA_Unit_Hero_Puck: {
        },
        CDOTA_BaseNPC_Seasonal_CNY_Balloon: {
        },
        C_DOTA_BaseNPC_NeutralItemStash: {
        },
        CDOTA_Artillery_EffectsEntity: {
        },
        CDOTA_Item_Recipe_Clumsy_Net: {
        },
        C_DOTA_Item_Recipe_WraithPact: {
        },
        C_DOTA_Item_Ring_Of_Basilius: {
        },
        C_DOTA_Item_Recipe_Black_King_Bar: {
        },
        C_DOTA_Ability_Lycan_SummonWolves_PermanentInvisibility: {
        },
        C_DOTA_Ability_Beastmaster_InnerBeast: {
        },
        C_DOTA_Ability_Razor_StaticLink: {
        },
        C_DOTA_Ability_Greevil_Miniboss_Orange_LightStrikeArray: {
        },
        CDOTA_Ability_Kobold_Disarm: {
        },
        CDOTA_Modifier_Aghsfort_Reward_HPAura: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Bounty_Hunter_8: {
        },
        C_DOTA_Ability_Special_Bonus_Night_Vision_800: {
        },
        C_DOTA_Ability_Special_Bonus_Strength_4: {
        },
        CDOTA_Modifier_Item_Enhancement_Vital: {
        },
        CDOTA_Modifier_Item_Spirit_Vessel: {
        },
        CDOTA_Modifier_Item_Solar_Crest_Armor_Addition: {
        },
        CDOTA_Modifier_Item_Maelstrom: {
        },
        CDOTA_Modifier_Item_RobeOfMagi: {
        },
        CDOTA_Modifier_Ringmaster_TameTheBeasts_Fear: {
        },
        CDOTA_Modifier_Marci_Special_Delivery: {
        },
        CDOTA_Modifier_Mars_Spear_Impale: {
        },
        CDOTA_Modifier_Skywrath_Mage_Concussive_Shot: {
        },
        CDOTA_Modifier_Visage_SoulAssumption_Delay: {
        },
        CDOTA_Modifier_LoneDruid_SpiritLink_BearFear: {
        },
        CDOTA_Modifier_Spirit_Bear_Necessities: {
        },
        CDOTA_Modifier_Lycan_Shard: {
        },
        CDOTA_Modifier_DoomBringer_Doom_Intrinsic: {
        },
        CDOTA_Modifier_Lion_Innate_ToHellAndBack_RespawnBuff: {
        },
        CDOTA_Modifier_Pudge_FleshHeap_Block: {
        },
        CDOTA_Modifier_Tutorial_LowAttackPriority: {
        },
        CIngameEvent_PermanentGrants: {
        },
        CDOTA_Unit_Templar_Gate: {
        },
        C_EconEntity: {
        },
        C_DOTA_Item_RuneSpawner: {
        },
        C_DOTA_Item_Recipe_Devastator: {
        },
        C_DOTA_Item_Elven_Tunic: {
        },
        CDOTA_Ability_Mars_Spear: {
        },
        C_DOTA_Ability_Shredder_TimberChain: {
        },
        CDOTA_Modifier_Greevil_Miniboss_Purple_VenomousGale: {
        },
        CDOTA_Modifier_Greevil_Miniboss_Red_Earthshock: {
        },
        CDOTA_Modifier_CentaurKhan_EnduranceAura: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Sven_3: {
        },
        CDOTA_Modifier_Item_Orb_Of_Corrosion: {
        },
        CDOTA_Modifier_Ringmaster_Decoy_Wheel: {
        },
        CDOTA_Modifier_Dawnbreaker_Fire_Wreath_Slow: {
        },
        CDOTA_Modifier_Keeper_of_the_Light_ManaMagnifier_Boost: {
        },
        CDOTA_Modifier_Brewmaster_FireBrewling_FireStance: {
        },
        CDOTA_Modifier_Obsidian_Destroyer_Sanity_Eclipse_Counter: {
        },
        CDOTA_Modifier_Clockwerk_RocketFlare_Thinker: {
        },
        CDOTA_Modifier_Rattletrap_Cog_Thinker_Self_Bonuses: {
        },
        CDOTA_Modifier_QueenOfPain_Arcana: {
        },
        C_BaseAnimatingOverlayController: {
        },
        CPlayer_UseServices: {
        },
        C_DOTA_Unit_Hero_Weaver: {
        },
        C_DOTA_Item_Magnifying_Monocle: {
        },
        C_DOTA_Item_Recipe_Titan_Sliver: {
        },
        CDOTA_Item_Recipe_GreaterFamango: {
        },
        C_DOTA_Item_Recipe_Necronomicon_3: {
        },
        C_DOTA_Item_Recipe_Sange: {
        },
        C_DOTA_Ability_Obsidian_Destroyer_SanityEclipse: {
        },
        C_DOTA_Ability_Slardar_Bash: {
        },
        C_PointValueRemapper: {
        },
        C_DotaSubquestTutorialEvent: {
        },
        CDOTA_Ability_AghsFort_TrapRoom_MeatHook: {
        },
        CDOTA_Modifier_Aghsfort_Bonus_Pudge_Meat_Hook: {
        },
        C_DOTA_Ability_Special_Bonus_Agility_100: {
        },
        CDOTA_Modifier_PrimalBeast_Pulverize_AoeBuff: {
        },
        CDOTA_Modifier_Legion_Commander_Duel_DamageBoost: {
        },
        CDOTA_Modifier_Elder_Titan_Tip_The_Scales_Aura: {
        },
        CDOTA_Modifier_Visage_SummonFamiliars_Recall: {
        },
        CDOTA_Modifier_Brewmaster_Fear: {
        },
        CDOTA_Modifier_Clinkz_Infernal_Shred_ArmorPiercing: {
        },
        CDOTA_Modifier_Lion_ManaDrain_Immunity: {
        },
        CDOTA_Modifier_Kunkka_Torrent: {
        },
        CGameSceneNodeHandle: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Unit_Hero_NightStalker: {
        },
        C_DOTA_NPC_BaseBlocker: {
        },
        CDOTA_Match3_PieceEntity: {
        },
        C_DOTA_Item_Aghanims_Shard_Roshan: {
        },
        C_DOTA_Ability_Kez_BaseAbility: {
        },
        C_DOTA_Ability_AbyssalUnderlord_DarkRift: {
        },
        C_DOTA_Ability_SandKing_Epicenter: {
        },
        CDOTA_Ability_AncientApparition_IceBlast: {
        },
        CDOTA_Modifier_PineCone_AcornShot_DelayThinker: {
        },
        CDOTA_Modifier_Special_Bonus_MP_Regen_Amp: {
        },
        C_DOTA_Ability_Special_Bonus_Movement_Speed_60: {
        },
        CDOTA_Modifier_Ringmaster_Wheel_Debuff: {
        },
        CDOTA_Modifier_Tusk_Snowball_Movement: {
        },
        CDOTA_Modifier_Undying_Tombstone_HP: {
        },
        CDOTA_Modifier_Brewmaster_DrunkenBrawler_BrewedUp_Hangover: {
        },
        CDOTA_Modifier_Brewmaster_DrunkenBrawler_Passive: {
        },
        CDOTA_Modifier_Obsidian_Destroyer_Equilibrium_BonusMana: {
        },
        CDOTA_Modifier_Broodmother_WebWalk_Thinker: {
        },
        CDOTA_Modifier_DarkSeer_Normal_Punch_Illusion: {
        },
        CDOTA_Modifier_ShadowShaman_EtherShock: {
        },
        CDOTA_Modifier_Lina_Slow_Burn: {
        },
        CDOTA_Modifier_Earthshaker_EchoSlam_Delay: {
        },
        CDOTA_Modifier_Mirana_Leap: {
        },
        CDOTA_Modifier_Nevermore_Presence: {
        },
        C_DOTACustomGameHeroPickRules: {
        },
        CPulseCell_Unknown: {
        },
        C_DOTA_Item_Aghanims_Shard: {
        },
        CDOTA_Ability_Dawnbreaker_Celestial_Hammer: {
        },
        CDOTA_Ability_Skywrath_Mage_Ruin_And_Restoration: {
        },
        C_DOTA_Ability_Shredder_Reactive_Armor: {
        },
        C_DOTA_Ability_TrollWarlord_BattleTrance: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Disruptor: {
        },
        C_DOTA_Ability_Omniknight_Martyr: {
        },
        CDOTA_Ability_Life_Stealer_Empty3: {
        },
        C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_25: {
        },
        C_DOTA_Ability_Special_Bonus_Strength_3: {
        },
        C_DOTA_DataCustomTeam: {
        },
        CDOTA_Modifier_Item_TranquilBoots: {
        },
        CDOTA_Modifier_Bristleback_Warpath_Active: {
        },
        CDOTA_Modifier_Magnataur_ReversePolarity_Stun: {
        },
        CDOTA_Modifier_Tiny_Toss: {
        },
        CDOTA_Modifier_Morphling_ScepterStatsDrain_Agility_Buff: {
        },
        CDOTA_Modifier_Morphling_ScepterStatsDrain_Strength_Debuff: {
        },
        CDOTA_Modifier_Bloodseeker_Thirst_Active: {
        },
        C_IngameEvent_FV2018: {
        },
        C_DOTA_Unit_Brewmaster_PrimalFire: {
        },
        CDOTA_Item_Conjurers_Catalyst: {
        },
        CDOTA_Ability_Hoodwink_MistwoodsWayfarer: {
        },
        C_DOTA_Ability_Snapfire_Buckshot: {
        },
        CDOTA_Ability_Nyx_Assassin_ManaBurn: {
        },
        C_DOTA_Ability_Nevermore_Requiem: {
        },
        CDOTA_Modifier_AghsFort_TreantMiniboss_NaturesGuise_Root: {
        },
        CDOTA_Modifier_PineCone_AcornShot_Slow: {
        },
        CDOTA_Ability_Aghsfort_Reward_ArmorAura: {
        },
        CDOTA_Ability_Seasonal_Summon_Penguin: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Broodmother_3: {
        },
        C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_20: {
        },
        C_DOTA_Ability_Special_Bonus_MP_Regen_3: {
        },
        C_DOTA_Ability_Special_Bonus_Gold_Ability_Draft: {
        },
        CDOTA_Modifier_Item_MartyrsPlate_Effect: {
        },
        CDOTA_Modifier_Item_Stormcrafter: {
        },
        CDOTA_Modifier_Item_Spirit_Vessel_Heal: {
        },
        CDOTA_Modifier_Item_Cyclone: {
        },
        CDOTA_Modifier_Item_BeltOfStrength: {
        },
        CDOTA_Modifier_Muerta_PierceTheVeil_SpellAmpBoost: {
        },
        CDOTA_Modifier_DarkWillow_Bedlam: {
        },
        CDOTA_Modifier_MonkeyKing_SpringSlow: {
        },
        CDOTA_Modifier_Phoenix_IcarusDiveBurn: {
        },
        CDOTA_Modifier_Centaur_Return_Bonus_Damage: {
        },
        CDOTA_Modifier_SpiritBreaker_BullRush: {
        },
        CDOTA_Modifier_Rattletrap_JetPack: {
        },
        CDOTA_Modifier_Special_Bonus_Unique_Beastmaster_5: {
        },
        CDOTA_Modifier_Axe_CounterHelix_DamageReduction: {
        },
        CDOTA_Modifier_BookOfIntelligence: {
        },
        CDOTA_Unit_Grimstroke_InkCreature: {
        },
        C_DOTA_Unit_Courier: {
        },
        CDOTA_Item_GunpowderGauntlets: {
        },
        C_DOTA_Item_Recipe_HelmOfTheDominator: {
        },
        C_DOTA_Ability_Hoodwink_Bushwhack: {
        },
        C_DOTA_Ability_Invoker_Alacrity_AD: {
        },
        C_DOTA_Ability_Enchantress_RabbleRouser: {
        },
        CDOTA_Ability_Leshrac_ChronopticNourishment: {
        },
        C_DOTA_Ability_Roshan_RevengeRoar: {
        },
        CDOTA_Modifier_Watch_Tower_Invulnerable_Temporary: {
        },
        CDOTA_Modifier_UpgradedBarricade: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Snapfire_3: {
        },
        CDOTA_Modifier_Foragers_Kit: {
        },
        CDOTA_Modifier_Searing_Signet_Burn: {
        },
        CDOTA_Modifier_DarkWillow_BrambleMaze_Creation_Thinker: {
        },
        CDOTA_Modifier_Silencer_GlobalSilence: {
        },
        CDOTA_Modifier_Invoker_GhostWalk_Self: {
        },
        CDOTA_Modifier_Spectre_ShadowStep_Illusion: {
        },
        CDOTA_Modifier_Broodmother_WebWalk: {
        },
        CDOTA_Modifier_Tiny_Insurmountable: {
        },
        CDOTA_Modifier_Bane_FiendsGrip_Illusion: {
        },
        CDOTA_Modifier_Antimage_DampenMagic_Bonus: {
        },
        C_DOTA_Unit_Hero_WitchDoctor: {
        },
        CDOTA_Item_Recipe_Faded_Broach: {
        },
        C_DOTA_Ability_Gyrocopter_Chop_Shop: {
        },
        C_DOTA_Ability_Enchantress_Enchant: {
        },
        C_DOTA_Ability_DeathProphet_Silence: {
        },
        CDOTA_Ability_FacelessVoid_TimeZone: {
        },
        C_DOTA_Item_Nian_Flag_Trap: {
        },
        C_DOTA_Ability_Nian_GreaterBash: {
        },
        CDOTA_Modifier_HillTroll_Rally_Stack: {
        },
        C_DOTA_Ability_EnragedWildkin_ToughnessAura: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Storm_Spirit: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Mirana_1: {
        },
        C_DOTA_Ability_Special_Bonus_Gold_Income_240: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Base_Damage_40: {
        },
        C_DOTA_Ability_Special_Bonus_Intelligence_13: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Speed_35: {
        },
        CDOTA_Modifier_Item_Occult_Bracelet: {
        },
        CDOTA_Modifier_Third_Eye: {
        },
        CDOTA_Modifier_OrchidMalevolence_Debuff: {
        },
        CDOTA_Modifier_Hoodwink_AcornShot_Dummy: {
        },
        CDOTA_Modifier_Pangolier_Gyroshell: {
        },
        CDOTA_Modifier_Oracle_DivinersDeck_TheLovers_Next: {
        },
        CDOTA_Modifier_Magnataur_Skewer_Impact: {
        },
        CDOTA_Modifier_Shadow_Demon_Menace: {
        },
        CDOTA_Modifier_Lycan_Wolf_Bite_Attack_Range: {
        },
        CDOTA_Modifier_Clinkz_BurningBarrage_DamageReduction: {
        },
        CDOTA_Modifier_Leshrac_Lightning_Storm_Slow: {
        },
        CDOTA_Modifier_SandKing_SandStorm_Blind: {
        },
        CPulseCell_Outflow_CycleRandom: {
        },
        CPulseCell_Step_PublicOutput: {
        },
        C_DOTA_BaseNPC_Additive: {
        },
        CDOTA_Match3_EffectsEntity: {
        },
        C_DOTA_Item_Enhancement_Thick: {
        },
        C_DOTA_Item_UltimateScepter_2: {
        },
        CDOTA_Ability_Tusk_WalrusPunch: {
        },
        C_DOTA_Ability_Wisp_Relocate: {
        },
        C_DOTA_Ability_Rattletrap_BatteryAssault: {
        },
        C_DOTA_Ability_Leshrac_Diabolic_Edict: {
        },
        C_DOTA_Ability_TemplarAssassin_Trap_Teleport: {
        },
        C_DOTA_Ability_Sniper_Headshot: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_30: {
        },
        C_DOTA_Ability_Special_Bonus_Gold_lvl15_l: {
        },
        CDOTA_Modifier_Spellslinger_Restore: {
        },
        CDOTA_Modifier_Item_Hood_Of_Defiance: {
        },
        CDOTA_Modifier_Terrorblade_Metamorphosis_Transform: {
        },
        CDOTA_Modifier_DoomBringer_Devils_Bargain: {
        },
        CDOTA_Modifier_DoomBringer_Doom_Break: {
        },
        CDOTA_Modifier_Luna_Eclipse: {
        },
        CDOTA_Modifier_TemplarAssassin_Meld: {
        },
        CDOTA_Modifier_Venomancer_PoisonStingBase: {
        },
        CDOTA_Modifier_Beastmaster_InnerBeast_Aura: {
        },
        CDOTA_Modifier_Slardar_Bash_Active: {
        },
        CDOTA_Modifier_Necrolyte_Heartstopper_Aura: {
        },
        C_DOTA_Unit_Hero_Leshrac: {
        },
        C_DOTA_Item_Overwhelming_Blink: {
        },
        CDOTA_Item_Force_Boots: {
        },
        CDOTA_Ability_Magnataur_Shockwave: {
        },
        C_DOTA_Ability_Chen_Penitence: {
        },
        CDOTA_Ability_Furion_SpiritOfTheForest: {
        },
        C_DOTA_Ability_Luna_LucentBeam: {
        },
        C_DOTA_Ability_Sven_Shieldbreaker: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Terrorblade: {
        },
        C_DOTA_Ability_Special_Bonus_Cooldown_Reduction_6: {
        },
        CDOTA_Modifier_Item_Arcane_Blink_Buff: {
        },
        CDOTA_Modifier_Techies_DeployRemoteMine: {
        },
        CDOTA_Modifier_Keeper_of_the_Light_ManaMagnifier: {
        },
        CDOTA_Modifier_DarkSeer_Surge_Trail: {
        },
        CDOTA_Modifier_Rattletrap_Cog_Thinker: {
        },
        CDOTA_Modifier_Leshrac_Diabolic_Edict: {
        },
        CDOTA_Modifier_Viper_Nose_Dive_Effect: {
        },
        CDOTA_Modifier_PhantomAssassin_Gravestone_Thinker: {
        },
        CDOTA_Modifier_Enigma_Black_Hole_Thinker_Scepter: {
        },
        CDOTA_Modifier_Lich_ChainFrost_Slow: {
        },
        CDOTA_Modifier_Mirana_Starfall_Scepter_Thinker: {
        },
        CDOTA_Modifier_AncientApparition_BoneChill_Debuff: {
        },
        C_DOTA_Unit_Hero_Terrorblade: {
        },
        C_DOTA_Item_Recipe_Desolator_2: {
        },
        C_DOTA_Ability_ChaosKnight_Chaos_Strike: {
        },
        C_DOTA_Ability_Lycan_SummonWolves_CriticalStrike: {
        },
        CDOTA_Ability_DarkSeer_Normal_Punch: {
        },
        C_DOTA_Ability_WitchDoctor_Voodoo_Switcheroo: {
        },
        CDOTA_Ability_Tinker_Shrink_Ray: {
        },
        CDOTA_Item_Tombstone_Mutation: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Invoker_11: {
        },
        CDOTA_Modifier_Safety_Bubble: {
        },
        CDOTA_Modifier_Item_Spy_Gadget: {
        },
        CDOTA_Modifier_NagaSiren_SlithereenCutlass_MissChance: {
        },
        CDOTA_Modifier_Meepo_Poof_Slow: {
        },
        CDOTA_Modifier_Brewmaster_PrimalSplitDuration: {
        },
        CDOTA_Modifier_SpiritBreaker_KnockbackAmp: {
        },
        CDOTA_Modifier_SkeletonKing_Reincarnation: {
        },
        CDOTA_Modifier_Aura: {
        },
        C_DOTA_Unit_Hero_Batrider: {
        },
        C_DOTA_BaseNPC_Creep_Talking: {
        },
        C_DOTA_Item_Recipe_OrchidMalevolence: {
        },
        CDOTA_Ability_Slark_SaltwaterShiv: {
        },
        C_DOTA_Ability_Furion_Sprout: {
        },
        C_DOTA_Ability_Furion_Greater_Sprout: {
        },
        CDOTA_Ability_Death_Prophet_MourningRitual: {
        },
        C_DOTA_Ability_Courier_GoToSecretShop: {
        },
        C_DOTA_Ability_Courier_TakeStashItems: {
        },
        C_DOTA_Ability_Enigma_BlackHole: {
        },
        C_DOTA_Ability_Tiny_Tree_Channel: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Furion_2: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Mirana_7: {
        },
        CDOTA_Ability_Special_Bonus_Spell_AoE_75: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Block_20: {
        },
        CDOTA_Modifier_BotChallenge_SkeletonKing_BoneGuard_Summon: {
        },
        CDOTA_Modifier_Item_TiaraOfSelemene: {
        },
        CDOTA_Modifier_Item_Urn_Of_Shadows: {
        },
        CDOTA_Modifier_Item_Bloodstone: {
        },
        CDOTA_Modifier_Item_Aegis: {
        },
        CDOTA_Modifier_MonkeyKing_FurArmyThinker: {
        },
        CDOTA_Modifier_Shredder_Timberchain_Splinter_Slow: {
        },
        CDOTA_Modifier_Spectre_Haunt: {
        },
        CDOTA_Modifier_Pugna_NetherBlast_Thinker: {
        },
        CDOTA_Modifier_Roshan_DevotionAura: {
        },
        CDOTA_Modifier_Enigma_MidnightPulseThinker: {
        },
        CDOTA_Modifier_VengefulSpirit_Command_Aura_Effect: {
        },
        CDOTA_Modifier_AntiMage_ManaVoid: {
        },
        CDOTA_Modifier_Filler_Heal: {
        },
        C_IngameEvent_TI7: {
        },
        CPulse_BlackboardReference: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        C_DOTA_Unit_VisageFamiliar: {
        },
        CDOTA_Item_Enchanters_Bauble: {
        },
        CDOTA_Item_Octarine_Core: {
        },
        C_DOTA_Item_Recipe_Ethereal_Blade: {
        },
        C_DOTA_Item_Recipe_Maelstrom: {
        },
        C_DOTA_Item_RingOfHealth: {
        },
        C_DOTA_Ability_Brewmaster_AstralPulse: {
        },
        C_DOTA_Ability_Leshrac_Greater_Lightning_Storm: {
        },
        C_DOTA_Ability_Twin_Gate_Portal_Warp: {
        },
        CDOTA_Modifier_Seasonal_TI11_RockPaperScissors: {
        },
        CDOTA_Ability_UpheavalUrn_Reincarnation: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Pudge_7: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Earthshaker_3: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Ancient_Apparition_7: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Arc_Warden_5: {
        },
        CDOTA_Modifier_Item_Moonshard_Consumed: {
        },
        CDOTA_Modifier_Kez_GrapplingClaw_Slow: {
        },
        CDOTA_Modifier_Phoenix_Sun_Ray_Slow: {
        },
        CDOTA_Modifier_Techies_Suicide_Leap_Animation: {
        },
        CDOTA_Modifier_Slark_EssenceShift: {
        },
        CDOTA_Modifier_Invoker_Exort_Intrinsic: {
        },
        CDOTA_Modifier_Venomancer_Plague_Carrier: {
        },
        CDOTA_Modifier_Enigma_DemonicConversion: {
        },
        CDOTA_Modifier_PhantomLancer_JuxtaposeIllusionUncontrollable: {
        },
        C_DOTA_Item_AncientGuardian: {
        },
        C_DOTA_Item_Disperser: {
        },
        C_DOTA_Ability_Rubick_Curiosity: {
        },
        C_DOTA_Ability_Lycan_FeralImpulse: {
        },
        CDOTA_Ability_Obsidian_Destroyer_Objurgation: {
        },
        CDOTA_Ability_Ursa_Bear_Down: {
        },
        CDOTA_Ability_AghsFort_Creature_Phoenix_LaunchFireSpirit: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Broodmother_4: {
        },
        CDOTA_Modifier_Special_Bonus_All_Stats: {
        },
        C_DOTA_Ability_Special_Bonus_MP_Regen_5: {
        },
        C_DOTA_Ability_Special_Bonus_HP_Regen_50: {
        },
        C_DOTA_Ability_Special_Bonus_Cleave_150: {
        },
        CDOTA_Modifier_Muerta_Revenant_Thinker: {
        },
        CDOTA_Modifier_Marci_Lunge_LandingAnim: {
        },
        CDOTA_Modifier_Winter_Wyvern_Dragon_Sight: {
        },
        CDOTA_Modifier_Centaur_Return_Aura: {
        },
        CDOTA_Modifier_Chaos_Knight_FundamentalForging: {
        },
        CDOTA_Modifier_Enchantress_Little_Friends_Kill_Credit: {
        },
        CDOTA_Modifier_QueenOfPain_SonicWave_Knockback: {
        },
        CDOTA_Modifier_Warlock_Golem_Permanent_Immolation_Debuff: {
        },
        CDOTA_Modifier_Morphling_Become_Agility: {
        },
        CODTA_Unit_Warlock_Imp: {
        },
        CDOTA_Item_Recipe_Demonicon: {
        },
        C_DOTA_Item_Third_eye: {
        },
        C_DOTA_Ability_Necronomicon_Warrior_ManaBurn: {
        },
        C_DOTA_Ability_ArcWarden_MagneticField: {
        },
        C_DOTA_Ability_Tusk_Snowball: {
        },
        C_DOTA_Ability_Meepo_FlingRelease: {
        },
        C_DOTA_Ability_FacelessVoid_TimeWalk_Reverse: {
        },
        CDOTA_Ability_Tinker_WarpGrenade: {
        },
        C_DOTA_Ability_Zuus_StaticField: {
        },
        C_DOTA_Ability_Morphling_Syntropy: {
        },
        CDOTA_Modifier_ForestTrollHighPriest_HealAmp: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Amplify_25: {
        },
        CDOTA_Modifier_Item_MartyrsPlate: {
        },
        CDOTA_Modifier_Item_Vladmir_Aura: {
        },
        CDOTA_Modifier_Item_SobiMask: {
        },
        CDOTA_Modifier_Kez_FalconRush_Intrinsic: {
        },
        CDOTA_Modifier_Elder_Titan_EchoStomp_Magic_Immune: {
        },
        CDOTA_Modifier_Centaur_Cart: {
        },
        CDOTA_Modifier_NagaSiren_RipTide_Slow: {
        },
        CDOTA_Modifier_Ogre_Magi_Fireblast_Multicast: {
        },
        CDOTA_Modifier_Treant_NaturesGuise: {
        },
        CDOTA_Modifier_Treant_EyesInTheForest_Thinker: {
        },
        CDOTA_Modifier_SpiritBreaker_ChargeOfDarknessTarget: {
        },
        CDOTA_Modifier_DragonKnight_FrostBreath_Slow: {
        },
        CDOTA_Modifier_Phantom_Assassin_Immaterial: {
        },
        CDOTA_Modifier_Miniboss_Radiance: {
        },
        CDOTA_Modifier_Courier_TransferItems: {
        },
        CDOTA_Modifier_Sniper_KeenScope: {
        },
        CDOTA_Modifier_CrystalMaiden_Let_It_Go_Bonus: {
        },
        CDOTA_Modifier_AntiMage_Mana_Thirst_Vision: {
        },
        C_DOTAMutationGameMode: {
        },
        CAnimGraphNetworkedVariables: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Unit_AghsFort_Creature_DungeonBat: {
        },
        C_DOTA_Item_Recipe_Skadi: {
        },
        C_DOTA_Ability_Shredder_Chakram: {
        },
        CDOTA_Ability_Centaur_DoubleEdge: {
        },
        C_DOTA_Ability_Viper_Nose_Dive: {
        },
        CDOTA_Ability_Spawnlord_Master_Stomp: {
        },
        C_DOTA_Ability_Special_Bonus_Strength_9: {
        },
        C_DOTA_Ability_Special_Bonus_Intelligence_12: {
        },
        CDOTA_Modifier_Item_Quicksilver_Amulet: {
        },
        CDOTA_Modifier_RodOfAtos_Debuff: {
        },
        CDOTA_Modifier_Item_BladesOfAttack: {
        },
        CDOTA_Modifier_Item_Mantle: {
        },
        CDOTA_Modifier_Hoodwink_AcornShot_TreeThinker: {
        },
        CDOTA_Modifier_Meepo_Geomancy: {
        },
        CDOTA_Modifier_LoneDruid_DruidForm_Transform: {
        },
        CDOTA_Modifier_Silencer_LastWord: {
        },
        CDOTA_Modifier_Windrunner_Arcana: {
        },
        CDOTA_Modifier_Kunkka_XMarksTheSpotMarker: {
        },
        CDOTA_Modifier_Mirana_SelemenesFaithful_Aura: {
        },
        CDOTA_Modifier_Sand_King_BurrowStrike_Tracker: {
        },
        CDOTA_Modifier_Nevermore_Requiem_Thinker: {
        },
        CDOTA_ArcanaDataEntity_FacelessVoid: {
        },
        CFilterModel: {
        },
        C_DOTA_Item_MartyrsPlate: {
        },
        CDOTA_Item_Spider_Legs: {
        },
        C_DOTA_Ability_Chen_Martyrdom: {
        },
        CDOTA_Ability_Skeleton_King_Innate_VampiricSpirit: {
        },
        C_DOTA_Ability_BlueDragonspawnOverseer_Evasion: {
        },
        C_DOTA_Ability_AlphaWolf_CriticalStrike: {
        },
        C_SoundAreaEntityOrientedBox: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Leshrac_4: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Axe_2: {
        },
        C_DOTA_Ability_Special_Bonus_Cleave_15: {
        },
        CDOTA_Modifier_Item_Angels_Demise_InternalCD: {
        },
        CDOTA_Modifier_Item_HydrasBreath: {
        },
        CDOTA_Modifier_Item_HelmOfTheDominator_2: {
        },
        CDOTA_Modifier_Item_SangeAndYasha: {
        },
        CDOTA_Modifier_Marci_Unleash_FlurryCooldown: {
        },
        CDOTA_Modifier_Abaddon_Frostmourne_Debuff: {
        },
        CDOTA_Modifier_Skywrath_Mage_Mystic_Flare: {
        },
        CDOTA_Modifier_Slark_Pounce_Leash: {
        },
        CDOTA_Modifier_Disruptor_KineticField: {
        },
        CDOTA_Modifier_Spectre_Dispersion: {
        },
        CDOTA_Modifier_Jakiro_DoubleTrouble: {
        },
        CDOTA_Modifier_Tidehunter_Anchor_Unit: {
        },
        CDOTA_Modifier_Windrunner_ShackleShot: {
        },
        CDOTA_Modifier_ProjectileVisionOnMinimap: {
        },
        DOTA_AssassinMinigameNetworkState: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_SoundOpvarSetPointEntity: {
        },
        C_DOTA_Unit_Hero_Naga_Siren: {
        },
        C_DOTA_Item_Dormant_Curio: {
        },
        C_DOTA_Item_Rattlecage: {
        },
        C_DOTA_Ability_Oracle_FatesEdict: {
        },
        CDOTA_Ability_Techies_Minefield_Sign: {
        },
        C_DOTA_Ability_Chen_HandOfGod: {
        },
        CDOTA_Ability_Miniboss_Alleviation: {
        },
        C_DOTA_Ability_Earthshaker_Fissure: {
        },
        CPulseGameBlackboard: {
        },
        C_DOTA_Ability_Special_Bonus_Intelligence_10: {
        },
        C_DOTA_Ability_Special_Bonus_MP_100: {
        },
        TreeModelReplacement_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Modifier_Item_Enhancement_Crude: {
        },
        CDOTA_Modifier_Item_Illusionists_Cape_Aura: {
        },
        CDOTA_Modifier_Item_Force_Boots: {
        },
        CDOTA_Modifier_Muerta_PartingShot_SoulDebuff: {
        },
        CDOTA_Modifier_Earth_Spirit_Magnetize_Hero_Self_Buff: {
        },
        CDOTA_Modifier_EmberSpirit_FireRemnantThinker: {
        },
        CDOTA_Modifier_Shadow_Demon_DemonicPurge_ChargeCounter: {
        },
        CDOTA_Modifier_Invoker_Tornado_Twister: {
        },
        CDOTA_Modifier_Chen_Penitence_Self_Attack_Range: {
        },
        CDOTA_Modifier_Batrider_Smoldering_Resin: {
        },
        CDOTA_Modifier_Dark_Seer_Mental_Fortitude: {
        },
        CDOTA_Modifier_Lina_Overheat: {
        },
        CDOTA_Modifier_CrystalMaiden_IceRink_Movement: {
        },
        C_IngameEvent_WM2017: {
        },
        CChoreoComponent: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CPulseCell_Value_RandomInt: {
        },
        C_DOTA_Item_Recipe_Perseverance: {
        },
        C_DOTA_Ability_Elder_Titan_EchoStomp: {
        },
        C_DOTA_Ability_Shredder_Exposure_Therapy: {
        },
        C_DOTA_Ability_BountyHunter_ShurikenToss: {
        },
        C_DOTA_Ability_Dazzle_NothlBoon: {
        },
        C_DOTA_Ability_Lion_Innate_ToHellAndBack: {
        },
        C_DOTA_Ability_Earthshaker_Spirit_Cairn: {
        },
        CDOTA_Modifier_AghsFort_DragonKnight_BreatheFire_Debuff: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Crystal_Maiden_2: {
        },
        CDOTA_Modifier_Gungnir_Debuff_MagicResistance: {
        },
        CDOTA_Modifier_Item_Yasha_And_Kaya: {
        },
        CDOTA_Modifier_Item_ForceStaff: {
        },
        CDOTA_Modifier_Ringmaster_TheBox_Transform: {
        },
        CDOTA_Modifier_PrimalBeast_Pulverize_Self: {
        },
        CDOTA_Modifier_Marci_Lunge_TrackingMotion: {
        },
        CDOTA_Modifier_EmberSpirit_SleightOfFist_Caster_Invulnerability: {
        },
        CDOTA_Modifier_Centaur_DoubleEdge_Slow: {
        },
        CDOTA_Modifier_NagaSiren_SlithereenCutlass: {
        },
        CDOTA_Modifier_FacelessVoid_TimeDilation_Distortion: {
        },
        CDOTA_Modifier_Venomancer_PoisonStingWard: {
        },
        CDOTA_Modifier_Nian_Attachment: {
        },
        CDOTA_Modifier_Glyph_Reset: {
        },
        CDOTA_Modifier_Stacking_Base: {
        },
        C_IngameEvent_FV2019: {
        },
        C_RagdollPropAttached: {
        },
        CDOTA_Item_Recipe_Overwhelming_Blink: {
        },
        C_DOTA_Item_Dagon: {
        },
        C_DOTA_Item_Recipe_Dagon3: {
        },
        C_DOTA_Ability_Slark_Depth_Shroud: {
        },
        CDOTA_Ability_NagaSiren_Deluge: {
        },
        C_DOTA_Ability_Batrider_StickyNapalm: {
        },
        CDOTA_Ability_Clinkz_SearingArrows: {
        },
        C_ModelPointEntity: {
        },
        CDOTA_Modifier_Seasonal_Summon_CNY_Balloon_Thinker: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Timbersaw: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Speed_45: {
        },
        CDOTA_Modifier_RippersLash_Debuff: {
        },
        CDOTA_Modifier_Item_Heavy_Blade: {
        },
        CDOTA_Modifier_Lotus_Orb_Delay: {
        },
        CDOTA_Modifier_Item_UltimateScepter_Consumed: {
        },
        CDOTA_Modifier_Ringmaster_DarkCarnivalSouvenirs: {
        },
        CDOTA_Modifier_Pangolier_Swashbuckle_Stunned: {
        },
        CDOTA_Modifier_Phoenix_SunRayVision: {
        },
        CDOTA_Modifier_Tusk_Tag_Team_Attack_Slow: {
        },
        CDOTA_Modifier_Wisp_Spirit_Invulnerable: {
        },
        CDOTA_Modifier_Omniknight_HammerOfPurity: {
        },
        CDOTA_Modifier_Rattletrap_Junk_Mail: {
        },
        CDOTA_Modifier_Rattletrap_Cog_Immune: {
        },
        CDOTA_Modifier_Ursa_Earthshock_Move: {
        },
        CDOTA_Modifier_Lina_Slow_BurnIntrinsic: {
        },
        C_DOTA_Unit_Hero_Tiny: {
        },
        CDOTA_Item_Ex_Machina: {
        },
        C_DOTA_Item_Tome_of_aghanim: {
        },
        CDOTA_Item_RiverPainter6: {
        },
        C_DOTA_Item_MithrilHammer: {
        },
        C_DOTA_Ability_Ringmaster_Wheel: {
        },
        C_DOTA_Ability_Oracle_Prognosticate: {
        },
        C_DOTA_Ability_NyxAssassin_Innate_Mana_Burn: {
        },
        C_DOTA_Ability_Templar_Assassin_Hidden_Gates: {
        },
        CDOTA_Ability_Miniboss_Minion_FollowingMovement: {
        },
        C_DOTA_Ability_WitchDoctor_ParalyzingCask: {
        },
        C_DOTA_Ability_Morphling_Morph_Agi: {
        },
        C_DOTA_Ability_AncientRockGolem_Weakening_Aura: {
        },
        CDOTA_Modifier_AghsFort_Tower_BlastWave_Thinker: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Ursa_2: {
        },
        C_DOTA_Ability_Special_Bonus_Status_Resistance_25: {
        },
        C_DOTA_Ability_Special_Bonus_Corruption_5: {
        },
        CDOTA_Modifier_Ceremonial_Robe_Aura: {
        },
        CDOTA_Modifier_Item_Orb_of_Venom: {
        },
        CDOTA_Modifier_Visage_SummonFamiliars_StoneForm_Buff: {
        },
        CDOTA_Modifier_Ogre_Magi_School: {
        },
        CDOTA_Modifier_Life_Stealer_Rage: {
        },
        CDOTA_Modifier_Viper_NethertoxinMute: {
        },
        CDOTA_Modifier_Necrolyte_GhostShroud_Active: {
        },
        CDOTA_Modifier_Lina_LagunaBlade_Superheated: {
        },
        CDOTA_Modifier_PhantomLancer_Juxtapose_Invisibility: {
        },
        CDOTA_Modifier_Sven_Warcry_Barrier: {
        },
        C_DOTA_CombatLogQueryProgress: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CAnimationLayer: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Unit_Hero_StormSpirit: {
        },
        C_DOTA_BaseNPC_Effigy_BattleCup: {
        },
        C_DOTA_Item_Recipe_Orb_Of_Revelations: {
        },
        C_DOTA_Item_Recipe_Chipped_Vest: {
        },
        CDOTA_Item_Mango_Tree: {
        },
        C_DOTA_Item_QuellingBlade: {
        },
        C_DOTA_Ability_Lycan_Shapeshift: {
        },
        C_DOTA_Ability_Brewmaster_HurlBoulder: {
        },
        C_DOTA_Ability_Alchemist_UnstableConcoction: {
        },
        C_DOTA_Ability_Drow_Ranger_Glacier: {
        },
        CDOTA_Modifier_HarpyScout_TakeOff: {
        },
        C_DOTA_Ability_SatyrHellcaller_UnholyAura: {
        },
        C_DOTA_Ability_SatyrHellcaller_Shockwave: {
        },
        CDOTA_Modifier_Plus_HighFiveSuccess: {
        },
        CDOTA_Modifier_Special_Bonus_Base_Attack_Rate: {
        },
        C_DOTA_Ability_Special_Bonus_Magic_Resistance_30: {
        },
        CDOTA_Modifier_Item_PigletPole_Critter: {
        },
        CDOTA_Modifier_Item_Paintball: {
        },
        CDOTA_Modifier_Item_PhaseBoots_Active: {
        },
        CDOTA_Modifier_Pangolier_Swashbuckle: {
        },
        CDOTA_Modifier_Bristleback_BrawlersGrit_Buff: {
        },
        CDOTA_Modifier_SpiritBreaker_NetherStrike_Vision: {
        },
        CDOTA_Modifier_Dazzle_NothlBoon: {
        },
        CDOTA_Modifier_Courier_Burst: {
        },
        CDOTA_Modifier_WitchDoctor_Voodoo_Restoration_Heal: {
        },
        CDOTA_Modifier_Morphling_Waveform: {
        },
        C_DOTA_Unit_Hero_Meepo: {
        },
        CDOTA_Item_Prophets_Pendulum: {
        },
        C_DOTA_Item_Enhancement_Brawny: {
        },
        C_DOTA_Item_Fallen_Sky: {
        },
        CDOTA_Item_Recipe_Ocean_Heart: {
        },
        C_DOTA_Ability_AbyssalUnderlord_Abyssal_Horde: {
        },
        C_DOTA_Ability_Skywrath_Mage_Arcane_Bolt: {
        },
        CDOTA_Ability_Treant_LifeBomb: {
        },
        CDOTA_Ability_Lycan_SummonWolves_Hamstring: {
        },
        C_DOTA_Ability_Omniknight_Pacify: {
        },
        CDOTA_Modifier_MudGolem_Rock_Shard: {
        },
        CPathSimple: {
        },
        CDOTA_Modifier_AghsFort_ShadowWaveEffectPotion: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Riki_9: {
        },
        C_DOTA_Ability_Special_Bonus_Respawn_Reduction_35: {
        },
        C_DOTA_Ability_Special_Bonus_All_Stats_14: {
        },
        CDOTA_Modifier_Marci_CompanionRun_AllyMovespeed: {
        },
        CDOTA_Modifier_Hoodwink_MistwoodsWayfarer_Passive: {
        },
        CDOTA_Modifier_Pudge_Dismember_Gluttony_Strength_Stack: {
        },
        C_DOTA_BaseNPC_HoldoutTower: {
        },
        C_DOTA_Item_Recipe_Dagon5: {
        },
        C_DOTA_Item_WraithBand: {
        },
        C_DOTA_Ability_PrimalBeast_Onslaught_Release: {
        },
        CDOTA_Ability_Special_Bonus_Unique_Marci_Grapple_StunDuration: {
        },
        C_DOTA_Ability_Techies_FocusedDetonate: {
        },
        C_DOTA_Ability_QueenOfPain_Blink: {
        },
        C_DOTA_Ability_Necrolyte_Sadist_Stop: {
        },
        CDOTA_Ability_Bloodseeker_Thirst: {
        },
        CDOTA_Ability_Mirana_CelestialQuiver: {
        },
        C_DOTA_Ability_Nevermore_Shadowraze: {
        },
        C_DOTA_Ability_Bane_FiendsGrip: {
        },
        C_FuncTrackTrain: {
        },
        CDOTA_Modifier_Aghsfort_Aziyog_Underlord_Portal_FX: {
        },
        C_DOTA_Ability_Aghsfort_Wildwing_Tornado_Blast: {
        },
        CDOTA_Modifier_Ascension_AcidBlood_Thinker: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Tusk_5: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Ursa_4: {
        },
        C_DOTA_Ability_Special_Bonus_Cast_Range_200: {
        },
        C_DOTA_Ability_Special_Bonus_Night_Vision_500: {
        },
        C_DOTA_Ability_Special_Bonus_Movement_Speed_Percentage_14: {
        },
        C_DOTA_Ability_Special_Bonus_Mana_Break_15: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Speed_100: {
        },
        CDOTAPulsePanoramaFuncs: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Modifier_Item_Mage_Slayer: {
        },
        CDOTA_Modifier_Item_Phylactery_Slow: {
        },
        CDOTA_Modifier_Item_Guardian_Greaves: {
        },
        CDOTA_Modifier_Elder_Titan_EarthSplitter_Thinker: {
        },
        CDOTA_Modifier_Nyx_Assassin_Impale: {
        },
        CDOTA_Modifier_TargetDummy_Unkillable: {
        },
        CDOTA_Modifier_FacelessVoid_Arcana: {
        },
        CDOTA_Modifier_Sniper_TakeAim: {
        },
        CDOTA_Modifier_Riki_Permanent_Invisibility: {
        },
        CDOTA_Modifier_Razor_StormSurge: {
        },
        CDOTA_Modifier_Earthshaker_EnchantTotem_BodyProjectile: {
        },
        CDOTA_Modifier_SkeletonKing_MortalStrike: {
        },
        CDOTA_Modifier_Bane_FiendsGrip: {
        },
        CDOTA_Modifier_Hide_On_Minimap: {
        },
        C_DOTATurboHeroPickRules: {
        },
        C_EconWearable: {
        },
        CDOTA_Item_AshLegionShield: {
        },
        C_DOTA_Ability_Winter_Wyvern_Bookwyrm: {
        },
        C_DOTA_Ability_Rubick_Empty1: {
        },
        C_DOTA_Ability_Dazzle_Bad_Juju: {
        },
        CDOTA_Ability_Beastmaster_Summon_Raptor: {
        },
        C_DOTA_Ability_Brewmaster_PrimalSplit: {
        },
        C_EnvDecal: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Skywrath_6: {
        },
        C_DOTA_Ability_Special_Bonus_Mana_Reduction_10: {
        },
        PlayerResourceBroadcasterData_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Modifier_Item_Necronomicon_2: {
        },
        CDOTA_Modifier_Event_Frogling: {
        },
        CDOTA_Modifier_VoidSpirit_AetherRemnant_Unit: {
        },
        CDOTA_Modifier_Pangolier_Swashbuckle_Slow: {
        },
        CDOTA_Modifier_Legion_Commander_PressTheAttack: {
        },
        CDOTA_Modifier_Legion_Commander_OverwhelmingOdds_Armor: {
        },
        CDOTA_Modifier_Abaddon_Frostmourne_Debuff_Bonus: {
        },
        CDOTA_Modifier_Treant_LeechSeed: {
        },
        CDOTA_Modifier_Dark_Seer_Aggrandize: {
        },
        CDOTA_Modifier_Tidehunter_Gush: {
        },
        fogparams_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Unit_Hero_Undying: {
        },
        C_EnvDeferredLightClientOnly: {
        },
        C_DOTA_Item_Enhancement_Hulking: {
        },
        CDOTA_Ability_Life_Stealer_Empty1: {
        },
        C_DOTA_Ability_Witch_Doctor_Innate_Maledict_Heal_Reduction: {
        },
        CDOTA_Modifier_KoboldTunneler_ProspectingAura: {
        },
        C_DOTA_Ability_Special_Bonus_20_Crit_2: {
        },
        C_DOTA_Ability_Special_Bonus_Respawn_Reduction_15: {
        },
        C_DOTA_Ability_Special_Bonus_MP_350: {
        },
        C_DOTA_Ability_Special_Bonus_MP_275: {
        },
        CDOTA_Modifier_Item_Enhancement_Fierce: {
        },
        CDOTA_Modifier_Item_Holy_Locket: {
        },
        CDOTA_Modifier_Item_Lotus_Orb: {
        },
        CDOTA_Modifier_Item_CrellasCrozier: {
        },
        CDOTA_Modifier_Largo_CatchyLick_Knockback: {
        },
        CDOTA_Modifier_Dawnbreaker_Solar_Guardian_AirTime: {
        },
        CDOTA_Modifier_Phoenix_Dying_Light: {
        },
        CDOTA_Modifier_Brewmaster_PrimalSplit: {
        },
        CDOTA_Modifier_TemplarAssassin_RefractionAbsorb: {
        },
        CDOTA_Modifier_Beastmaster_InnerBeast_Berserk: {
        },
        CDOTA_Modifier_VengefulSpirit_Command_Aura_Effect_Debuff: {
        },
        CDOTA_Modifier_SandKing_SandStorm_Invisibility: {
        },
        CDOTA_Modifier_SecondaryUnit_Taunt: {
        },
        C_DOTA_Unit_Hero_Huskar: {
        },
        C_DOTA_Item_BlackGrimoire: {
        },
        C_DOTA_Item_Wind_Waker: {
        },
        C_DOTA_Item_Recipe_Pupils_gift: {
        },
        C_DOTA_Item_HandOfMidas: {
        },
        C_DOTA_Item_Mantle: {
        },
        CDOTA_Ability_Special_Bonus_Unique_Marci_Guardian_Lifesteal: {
        },
        CDOTA_Ability_Abaddon_DeathCoil: {
        },
        C_DOTA_Ability_Dazzle_ShadowWave: {
        },
        C_DOTA_Ability_Nian_Dive: {
        },
        C_DOTA_Ability_Zuus_ThundergodsWrath: {
        },
        CDOTA_Modifier_MudGolem_CloakAura_Bonus: {
        },
        C_Beam: {
        },
        C_EnvLightProbeVolume: {
        },
        C_DOTA_Ability_AghsFort_AssaultCaptain_SunRay: {
        },
        CDOTA_AghsFort_Ability_Creature_Venomancer_PoisonSting: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Silencer_2: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Dawnbreaker_FireWreath_Swipe: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Mirana_3: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Earthshaker_4: {
        },
        C_DOTA_Ability_Special_Bonus_Exp_Boost_5: {
        },
        C_DOTA_Ability_Special_Bonus_MP_Regen_4: {
        },
        CDOTA_Modifier_Item_HydrasBreath_Thinker: {
        },
        CDOTA_Modifier_Item_Flicker_Damaged: {
        },
        CDOTA_Modifier_Centaur_Double_Edge_Damage_Tracking: {
        },
        CDOTA_Modifier_Nyx_Assassin_Jolt_Damage_Tracker: {
        },
        CDOTA_Modifier_SpiritBreaker_ChargeOfDarkness_Linger: {
        },
        CDOTA_Modifier_Omniknight_Degen_Aura_Effect: {
        },
        CDOTA_Modifier_Venomancer_VenomousGale: {
        },
        CDOTA_Modifier_Windrunner_Tailwind_Counter: {
        },
        CDOTA_Modifier_Bloodseeker_Bloodrage: {
        },
        CDOTA_Modifier_AttackImmune: {
        },
        CExplosionTypeData: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CDOTA_Item_Partisans_Brand: {
        },
        C_DOTA_Item_Recipe_Arcane_Ring: {
        },
        CDOTA_Item_Recipe_Grove_Bow: {
        },
        C_DOTA_Item_Recipe_Mekansm: {
        },
        C_DOTA_Item_MaskOfMadness: {
        },
        C_DOTA_Item_StoutShield: {
        },
        C_DOTA_Ability_DoomBringer_ScorchedEarth: {
        },
        C_DOTA_Ability_Enchantress_Untouchable: {
        },
        C_DOTA_Ability_Tiny_Grow: {
        },
        C_DOTA_Ability_Morphling_Morph_Str: {
        },
        C_DOTA_Ability_AntiMage_SpellShield: {
        },
        C_DOTA_Ability_DarkTrollWarlord_Ensnare: {
        },
        CDOTA_Modifier_Watch_Tower_Marker: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lion_8: {
        },
        C_DOTA_Ability_Special_Bonus_Exp_Boost_10: {
        },
        C_DOTA_Ability_Special_Bonus_MP_600: {
        },
        CDOTA_Modifier_Oracle_DivinersDeck_TheFool: {
        },
        CDOTA_Modifier_Legion_Commander_MomentOfCourage_Lifesteal: {
        },
        CDOTA_Modifier_Shredder_Chakram_Thinker: {
        },
        CDOTA_Modifier_Meepo_Fling_Held: {
        },
        CDOTA_Modifier_TemplarAssassin_Meld_Animation: {
        },
        CDOTA_Modifier_Pugna_LifeDrain: {
        },
        CDOTA_Modifier_CrystalMaiden_Arcane_Overflow_Active: {
        },
        CDOTA_Modifier_DrowRanger_FrostArrows_Hypothermia_Active: {
        },
        CDOTA_Modifier_Juggernaut_Trinity_Movement: {
        },
        C_DOTA_Unit_Broodmother_Web: {
        },
        C_DOTA_Item_Sisters_Shroud: {
        },
        CDOTA_Item_Ward_Dispenser: {
        },
        C_DOTA_Ability_Hoodwink_Scurry: {
        },
        C_DOTA_Ability_Terrorblade_Dark_Unity: {
        },
        CDOTA_Ability_Dazzle_NothlProjectionEnd: {
        },
        C_DOTA_Ability_Riki_BlinkStrike: {
        },
        C_DOTA_Ability_SkeletonKing_MortalStrike: {
        },
        CDOTA_Modifier_AghsFort_Venomancer_PoisonSting: {
        },
        CDOTA_Modifier_AghsFort_SkeletonKing_VampiricAura: {
        },
        CDOTA_Ability_Aghsfort_TempBuff_CorpseExplosion: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lich_1: {
        },
        CDOTA_Modifier_Item_Solar_Crest: {
        },
        CDOTA_Modifier_Item_HeavensHalberd: {
        },
        CDOTA_Modifier_Item_Veil_Of_Discord_Debuff: {
        },
        CDOTA_Modifier_Ringmaster_CrystalBall_Truesight: {
        },
        CDOTA_Modifier_Marci_Lunge_Buff: {
        },
        CDOTA_Modifier_Invoker_AttackVisuals: {
        },
        CDOTA_Modifier_Rattletrap_Armor_Power: {
        },
        CDOTA_Modifier_DragonKnight_BlackDragon_Tooltip: {
        },
        CDOTA_Modifier_Slardar_Slithereen_Crush: {
        },
        CDOTA_Modifier_Axe_Culling_Blade_NoMinHealth: {
        },
        CDOTA_Modifier_Taunt: {
        },
        C_DOTA_MapTree: {
        },
        CDOTA_Ability_Abyssal_Underling_Archer_AoE: {
        },
        C_DOTA_Ability_FacelessVoid_Innate_DistortionField: {
        },
        CDOTA_Modifier_GiantWolf_CriticalStrike: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Underlord_6: {
        },
        C_DOTA_Ability_Special_Bonus_MP_500: {
        },
        CDOTA_Modifier_Ringmaster_StrongmanTonic_Buff: {
        },
        CDOTA_Modifier_Dawnbreaker_Fire_Wreath_Caster: {
        },
        CDOTA_Modifier_Techies_Suicide_RespawnTime: {
        },
        CDOTA_Modifier_Centaur_Horsepower: {
        },
        CDOTA_Modifier_LoneDruid_Rabid: {
        },
        CDOTA_Modifier_Fissure_Rooted: {
        },
        CDOTA_Modifier_Pudge_Swallow_Hide: {
        },
        CDOTA_Modifier_ScoutBonuses: {
        },
        CIngameEvent_TI2024: {
        },
        C_DOTA_Unit_Roshans_Banner: {
        },
        C_DOTA_BaseNPC_HoldoutTower_LightFast: {
        },
        CEnvSoundscapeTriggerableAlias_snd_soundscape_triggerable: {
        },
        CDOTA_Item_Spellslinger: {
        },
        C_DOTA_Item_Book_Of_Shadows: {
        },
        C_DOTA_Item_Veil_Of_Discord: {
        },
        CDOTA_Ability_Techies_StasisTrap: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Abaddon_5: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Disruptor_4: {
        },
        C_DOTA_Ability_Lycan_Apex_Predator: {
        },
        C_DOTA_Ability_Invoker_IceWall_AD: {
        },
        CDOTA_Ability_Alchemist_ChemicalRage: {
        },
        C_DOTA_Ability_Life_Stealer_Ghoul_Frenzy: {
        },
        CDOTA_Ability_Venomancer_PoisonSting: {
        },
        C_DOTA_Ability_Juggernaut_HealingWard: {
        },
        C_DOTA_Ability_Bane_Enfeeble: {
        },
        C_Breakable: {
        },
        CDOTA_Modifier_AghsFort_Potion_SpendCharge: {
        },
        CDOTA_Modifier_Aghsfort_TempBuff_CorpseExplosion_Debuff: {
        },
        C_DOTA_Ability_Healing_Campfire: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Oracle_8: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Templar_Assassin_8: {
        },
        CDOTA_Modifier_Item_AshLegionShield: {
        },
        CDOTA_Modifier_Devastator_Debuff: {
        },
        CDOTA_Modifier_Light_Collector_Blind: {
        },
        CDOTA_Modifier_Item_Tree_Processor_Speed: {
        },
        CDOTA_Modifier_Item_Trusty_Shovel: {
        },
        CDOTA_Modifier_Item_Essence_Distiller_Thinker: {
        },
        CDOTA_Modifier_Item_Mekansm: {
        },
        CDOTA_Modifier_Mars_Dauntless: {
        },
        CDOTA_Modifier_Shredder_Reactive_Armor: {
        },
        CDOTA_Modifier_Rubick_NullField: {
        },
        CDOTA_Modifier_ChaosKnight_Phantasm_Illusion_Shard: {
        },
        CDOTA_Modifier_Lycan_SummonWolves_BonusDamage: {
        },
        CDOTA_Modifier_Treant_Large_Bonus: {
        },
        CDOTA_Modifier_Treant_Bonus: {
        },
        CDOTA_Modifier_Windrunner_Windrun_ChargeCounter: {
        },
        C_DOTA_PortraitEntity: {
        },
        C_DOTA_Unit_Earth_Spirit_Stone: {
        },
        C_DOTA_BaseNPC_Creep_Neutral: {
        },
        CDOTA_Item_FlayersBota: {
        },
        CDOTA_Item_Illusionsts_Cape: {
        },
        C_DOTA_Ability_Wisp_Spirits_In: {
        },
        CDOTA_Modifier_PathMoveSpeed: {
        },
        C_DOTA_Ability_Special_Bonus_Day_Vision_400: {
        },
        CDOTA_Modifier_Avianas_Feather: {
        },
        CDOTA_Modifier_Item_Bullwhip_Buff: {
        },
        CDOTA_Modifier_Item_IronwoodBranch: {
        },
        CDOTA_Modifier_Furion_CurseOfTheForest: {
        },
        CDOTA_Modifier_Furion_Sprout_Marker: {
        },
        CDOTA_Modifier_Furion_Sprout_Heal: {
        },
        CDOTA_Modifier_Provide_Vision: {
        },
        CDOTA_Modifier_Tower_Aura_Bonus: {
        },
        CIngameEvent_Crownfall: {
        },
        CDOTA_ArcanaDataEntity_Razor: {
        },
        CFilterName: {
        },
        C_DOTA_Ability_Grimstroke_Return: {
        },
        C_DOTA_Ability_Oracle_PurifyingFlames: {
        },
        C_DOTA_Ability_Lich_FrostNova: {
        },
        CDOTA_Modifier_Greevil_Miniboss_Blue_ColdFeet_Freeze: {
        },
        CDOTA_Modifier_Greevil_Miniboss_Black_Nightmare: {
        },
        CDOTA_Modifier_JungleVarmint_Dive: {
        },
        C_DOTA_Ability_PudgeMiniboss_HatefulStrike: {
        },
        C_DOTA_Ability_Morty_Hop: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Bane_2: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Antimage_6: {
        },
        CDOTA_Modifier_Special_Bonus_Attack_Range: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Range_125: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Speed_60: {
        },
        CDOTA_Modifier_Item_HelmOfTheDominator_BonusHealth: {
        },
        CDOTA_Modifier_Ringmaster_Wheel_Mesmerize: {
        },
        CDOTA_Modifier_Snapfire_SpitCreep_ArcingUnit: {
        },
        CDOTA_Modifier_Oracle_DivinersDeck_TheTower: {
        },
        CDOTA_Modifier_Nyx_Assassin_Nyxth_Sense_Effect: {
        },
        CDOTA_Modifier_Lycan_FeralImpulse_Aura: {
        },
        CDOTA_Modifier_Spectre_SpectralDagger: {
        },
        CDOTA_Modifier_NightStalker_CripplingFear_Aura: {
        },
        CDOTA_Modifier_DarkSeer_Normal_Punch: {
        },
        CDOTA_Modifier_Dazzle_NothlProjection_SoulDebuff: {
        },
        CDOTA_Modifier_Nian_Apocalypse: {
        },
        CDOTA_Modifier_Zuus_Heavenly_Jump: {
        },
        CDOTA_Modifier_SkeletonKing_Reincarnation_Scepter_RespawnTime: {
        },
        CDOTA_Modifier_SetScaleset: {
        },
        C_DOTA_Unit_Hero_Slark: {
        },
        C_DOTA_Unit_Hero_SpiritBreaker: {
        },
        C_DOTA_Item_Tier5Token: {
        },
        C_RagdollProp: {
        },
        C_DOTA_Item_AbyssalBlade: {
        },
        C_DOTA_Ability_Largo_CroakOfGenius: {
        },
        C_DOTA_Ability_Largo_AmphibianRhapsody_FightSong: {
        },
        CDOTA_Ability_Snapfire_GobbleUp: {
        },
        CDOTA_Ability_Snapfire_LilShredder: {
        },
        C_DOTA_Ability_Invoker_Innate_Mastermind: {
        },
        C_DOTA_Ability_Enigma_MidnightPulse: {
        },
        C_DOTA_Ability_Tiny_TossTree: {
        },
        CDOTA_Modifier_Item_Helm_Of_The_Undying_Active: {
        },
        C_DotaSubquestBuyItems: {
        },
        CDOTA_Modifier_Seasonal_TI11_CongaLine: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Shadow_Demon_3: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Warlock_4: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Phantom_Assassin_7: {
        },
        C_DOTA_Ability_Special_Bonus_Respawn_Reduction_40: {
        },
        C_DOTA_Ability_Special_Bonus_Intelligence_16: {
        },
        C_DOTA_Ability_Special_Bonus_Mana_Break_20: {
        },
        C_DOTA_Ability_Special_Bonus_Gold_lvl20_r: {
        },
        CDOTA_Modifier_Item_AncientGuardian: {
        },
        CDOTA_Modifier_Repair_Kit: {
        },
        CDOTA_Modifier_Smoke_Of_Deceit_Secondary_Application_Cooldown: {
        },
        CDOTA_Modifier_Marci_Dispose_Debuff: {
        },
        CDOTA_Modifier_Pangolier_HeartPiercer: {
        },
        CDOTA_Modifier_Oracle_DivinersDeck_TheTower_Next: {
        },
        CDOTA_Modifier_DoomBringer_Lvl_Pain_Debuff: {
        },
        CDOTA_Modifier_PhantomAssassin_MarkOfDeath: {
        },
        CDOTA_Modifier_Nian_Waterball: {
        },
        CDOTA_Modifier_Lich_FrostNova_Slow: {
        },
        C_DOTA_Item_Misericorde: {
        },
        C_DOTA_Item_Imp_Claw: {
        },
        C_DOTA_Item_Desolator_2: {
        },
        C_DOTA_Item_Recipe_Soul_Ring: {
        },
        C_DOTA_Item_TranquilBoots: {
        },
        C_DOTA_Item_Flask: {
        },
        CDOTA_Ability_Pudge_MeatHook: {
        },
        CDOTA_Modifier_DoNotCastSmash: {
        },
        CDOTA_Modifier_KoboldTaskmaster_SpeedAura_Bonus: {
        },
        C_DOTA_Ability_AghsFort_AssaultCaptain_SearingChains: {
        },
        C_DOTA_Ability_AghsFort_Shadow_Demon_Shadow_Poison_Release: {
        },
        CDOTA_Ability_Seasonal_Summon_CNY_Tree: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Mirana_5: {
        },
        C_DOTA_Ability_Special_Bonus_Armor_9: {
        },
        CDOTA_Modifier_Item_Prophets_Pendulum: {
        },
        CDOTA_Modifier_Item_Quickening_Charm: {
        },
        CDOTA_Modifier_Item_Sange: {
        },
        CDOTA_Modifier_Item_BladeOfAlacrity: {
        },
        CDOTA_Modifier_Largo_AmphibianRhapsody_Self: {
        },
        CDOTA_Modifier_PrimalBeast_Onslaught_Knockback: {
        },
        CDOTA_Modifier_Marci_Special_Delivery_Aura: {
        },
        CDOTA_Modifier_Elder_Titan_Momentum: {
        },
        CDOTA_Modifier_Treant_NaturesGuise_Invis: {
        },
        CDOTA_Modifier_Chaos_Knight_Illusions_Damage_Reduction: {
        },
        CDOTA_Modifier_Animation_RightClawSwipe: {
        },
        CDOTA_Modifier_Sniper_Concussive_Grenade: {
        },
        CDOTA_Modifier_Juggernaut_Healing_Ward_Heal: {
        },
        CDOTA_Modifier_Pudge_Dismember_Gluttony_Strength_BuffCounter: {
        },
        CDotaMoveSpeedModifierPath: {
        },
        C_DOTA_Item_Enhancement_Restorative: {
        },
        CDOTA_Item_Recipe_Ward_Dispenser: {
        },
        C_DOTA_Item_MaskOfDeath: {
        },
        C_DOTA_Ability_Medusa_FixedMovespeed: {
        },
        C_DOTA_Ability_Disruptor_Thunder_Strike: {
        },
        C_DOTA_Ability_Silencer_CurseOfTheSilent: {
        },
        C_DOTA_Ability_ForgedSpirit_MeltingStrike: {
        },
        CDOTA_Ability_Broodmother_InsatiableHunger: {
        },
        C_DOTA_Ability_Tidehunter_ArmOfTheDeep: {
        },
        CDOTA_Modifier_Spawnlord_Master_Freeze: {
        },
        CDOTA_Modifier_Mutation_CreateTombstone_Aura: {
        },
        CDOTA_Ability_Aghsfort_Minor_Stats_Upgrade: {
        },
        C_DOTA_Ability_Special_Bonus_Armor_8: {
        },
        C_DOTA_Ability_Special_Bonus_Strength_16: {
        },
        C_DOTA_Ability_Special_Bonus_Intelligence_20: {
        },
        C_DOTA_Ability_Special_Bonus_Base: {
        },
        CDOTA_Modifier_Kobold_Cup: {
        },
        CDOTA_Modifier_Item_Devastator: {
        },
        CDOTA_Modifier_Item_Disperser_Evasion_Buff: {
        },
        CDOTA_Modifier_Snapfire_Scatterblast_Disarm: {
        },
        CDOTA_Modifier_MonkeyKing_FurArmy_SoldierInPosition: {
        },
        CDOTA_Modifier_NagaSiren_Deluge_Status_Resistance: {
        },
        CDOTA_Modifier_ChaosKnight_Phantasm_Illusion: {
        },
        CDOTA_Modifier_LoneDruid_SpiritLink_Active: {
        },
        CDOTA_Modifier_Invoker_SunStrike_Cataclysm_Thinker: {
        },
        CDOTA_Modifier_NightStalker_Void_Intrinsic: {
        },
        CDOTA_Modifier_Leshrac_Pulse_Nova: {
        },
        CDOTA_Modifier_PoisonNova_Thinker: {
        },
        C_DOTA_Unit_Hero_Rattletrap: {
        },
        CDOTA_Item_Recipe_Woodland_Striders: {
        },
        C_DOTA_Ability_KeeperOfTheLight_Radiant_Bind: {
        },
        C_DOTA_Ability_Invoker_Wex: {
        },
        CDOTA_Ability_Alchemist_AcidSpray: {
        },
        CDOTA_Ability_Dark_Seer_Mental_Fortitude: {
        },
        CDOTA_Modifier_Tornado_Tempest_Debuff: {
        },
        CDOTA_Modifier_AghsFort_TrapRoom_Hookshot: {
        },
        CDOTA_Modifier_Wave_Blast_Disarm: {
        },
        C_DOTA_Ability_Special_Bonus_Strength_15: {
        },
        CDOTA_Modifier_Mars_ArenaOfBlood: {
        },
        CDOTA_Modifier_Techies_Minefield_Sign_Scepter_Aura: {
        },
        CDOTA_Modifier_CallOfTheWild_Boar_Poison: {
        },
        CDOTA_Modifier_Tiny_Avalanche_Stun: {
        },
        CDOTA_Modifier_Zuus_ThundergodsWrathVisionThinker: {
        },
        CDOTA_Modifier_Tutorial_HideNPC: {
        },
        C_DOTABaseCustomHeroPickRules: {
        },
        CPulse_CallInfo: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        C_DOTA_Unit_Hero_BountyHunter: {
        },
        C_DOTA_Unit_Hero_Beastmaster: {
        },
        CDOTA_Unit_CustomGameAnnouncer: {
        },
        C_DOTA_BaseNPC_Creature: {
        },
        CBaseAnimGraph: {
        },
        C_DOTA_Item_RippersLash: {
        },
        C_DOTA_Item_Occult_Bracelet: {
        },
        C_DOTA_Ability_Necronomicon_Archer_AoE: {
        },
        CDOTA_Item_Recipe_Battlefury: {
        },
        CDOTA_Ability_Marci_Guardian: {
        },
        C_DOTA_Ability_Undying_TombstoneUnitGrab: {
        },
        CDOTA_Ability_Broodmother_IncapacitatingBite: {
        },
        CDOTA_Ability_Miniboss_InherentBuffs: {
        },
        C_DOTA_Ability_Witch_Doctor_GrisGris: {
        },
        CDOTA_Modifier_BlackDragon_DragonhideAura_Bonus: {
        },
        CDOTAEmptyAbility: {
        },
        C_DOTA_Ability_Wisp_Spirits: {
        },
        CDOTA_Modifier_Aghsfort_Aziyog_Underlord_Firestorm_Burn: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lycan_8: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lich_7: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Zeus_4: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Ogre_Magi_3: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_20: {
        },
        C_DOTA_Ability_Special_Bonus_Intelligence_7: {
        },
        CDOTA_Modifier_Item_DuelistGloves: {
        },
        CDOTA_Modifier_Item_Headdress_Aura: {
        },
        CDOTA_Modifier_Item_Yasha: {
        },
        CDOTA_Modifier_Item_Broadsword: {
        },
        CDOTA_Modifier_Largo_Frogstomp_Thinker: {
        },
        CDOTA_Modifier_Ringmaster_Impalement_Bleed: {
        },
        CDOTA_Modifier_DarkWillow_CursedCrown: {
        },
        CDOTA_Modifier_Abaddon_BorrowedTime: {
        },
        CDOTA_Modifier_Meepo_Earthbind_ChainDuration: {
        },
        CDOTA_Modifier_ChaosKnight_Phantasm: {
        },
        CDOTA_Modifier_Shadow_Demon_InnateSoulCatcher_HPLoss: {
        },
        CDOTA_Modifier_Alchemist_Corrosive_Weaponry_Debuff: {
        },
        CDOTA_Modifier_BountyHunter_Big_Game_Hunter: {
        },
        CDOTA_Modifier_WitchDoctor_MaledictDoT: {
        },
        CDOTA_Modifier_Pudge_Meat_Hook_Reveal: {
        },
        CDOTA_Modifier_AntiMage_Counterspell_Passive: {
        },
        C_IngameEvent_Base: {
        },
        CPulseCell_InlineNodeSkipSelector: {
        },
        CDOTA_BaseNPC_Seasonal_TI9_Drums: {
        },
        C_LightEntity: {
        },
        C_DOTA_Item_Titan_Sliver: {
        },
        CDOTA_Item_Recipe_Lotus_Orb: {
        },
        C_DOTA_Item_Recipe_Diffusal_Blade: {
        },
        C_DOTA_Item_RodOfAtos: {
        },
        CDOTA_Ability_Gyrocopter_Call_Down: {
        },
        C_DOTA_Ability_Tiny_Toss: {
        },
        C_DOTA_Ability_AntiMage_Mana_Overload: {
        },
        C_DotaSubquestAbilityCastCount: {
        },
        CDOTA_Modifier_Special_Bonus_Haste: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Earthshaker: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Faceless_Void_4: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_160: {
        },
        C_DOTA_Ability_Special_Bonus_Movement_Speed_50: {
        },
        CPulseGraphInstance_PanoramaPanel: {
        },
        CDOTA_Modifier_Ethereal_Blade_Ethereal: {
        },
        CDOTA_Modifier_Largo_Frogling: {
        },
        CDOTA_Modifier_MonkeyKing_FurArmy_BonusDamage: {
        },
        CDOTA_Modifier_Abyssal_Underling_Archer_AoE: {
        },
        CDOTA_Modifier_Techies_LandMines_ChargeCounter: {
        },
        CDOTA_Modifier_Shredder_Chakram_Debuff: {
        },
        CDOTA_Modifier_Omniknight_Pacify: {
        },
        CDOTA_Modifier_Clinkz_DeathPact: {
        },
        CDOTA_Modifier_Tidehunter_KrakenShell: {
        },
        CDOTA_Modifier_Lina_Combustion: {
        },
        CDOTA_Modifier_CrystalMaiden_Frostbite: {
        },
        CDOTA_Modifier_Pudge_Meat_Hook: {
        },
        CDOTA_Modifier_Followthrough: {
        },
        CDOTA_Modifier_Tower_Aura: {
        },
        CDOTA_AbilityDraftHeroState: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Unit_Hero_Abaddon: {
        },
        C_DOTA_Unit_TargetDummy: {
        },
        C_LocalTempEntity: {
        },
        CDOTA_Item_Panic_Button: {
        },
        C_DOTA_Item_Courier: {
        },
        C_DOTA_Item_Necronomicon: {
        },
        CDOTA_Ability_Elder_Titan_FundamentalFury_Spirit: {
        },
        C_DOTA_Ability_KeeperOfTheLight_IlluminateEnd: {
        },
        C_DOTA_Ability_Weaver_GeminateAttack: {
        },
        CDOTA_Modifier_AghsFort_Ascension_MagneticField_Thinker_Evasion: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_VoidSpirit_8: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Skywrath_4: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Slardar_4: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Windranger_4: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Pangolier: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Amplify_12: {
        },
        C_DOTA_LightInfo: {
        },
        CDOTA_Modifier_Item_Trickster_Cloak: {
        },
        CDOTA_Modifier_Demonicon_Bonus: {
        },
        CDOTA_Modifier_Item_Tenderizer: {
        },
        CDOTA_Modifier_Muerta_Gunslinger: {
        },
        CDOTA_Modifier_Silencer_Oppressive_Silence: {
        },
        CDOTA_Modifier_Warlock_Imp_ExplodeThinker: {
        },
        CDOTA_Modifier_Item_GrisGris: {
        },
        CDOTA_Modifier_Zuus_Static_Field_Slow: {
        },
        C_DOTA_Unit_Roshan: {
        },
        C_DOTA_Item_Heavy_Blade: {
        },
        CDOTA_Item_Recipe_The_Leveller: {
        },
        CDOTA_Item_Medallion_Of_Courage: {
        },
        C_DOTA_Item_Recipe_Revenants_Brooch: {
        },
        C_DOTA_Ability_DarkWillow_ShadowRealm: {
        },
        C_DOTA_Ability_Lina_FierySoul: {
        },
        C_DOTA_Ability_CrystalMaiden_Let_It_Go: {
        },
        CDOTA_Modifier_Frogmen_WaterBubble: {
        },
        CDOTA_Modifier_JungleVarmint_Creator: {
        },
        C_PointEntity: {
        },
        C_DOTA_Ability_Special_Bonus_Cast_Range_125: {
        },
        C_DOTA_Ability_Special_Bonus_Magic_Resistance_14: {
        },
        C_DOTA_Ability_Special_Bonus_Agility_5: {
        },
        CDOTA_Modifier_Item_Enhancement_Hulking: {
        },
        CDOTA_Modifier_Item_SistersShroud_Buff: {
        },
        CDOTA_Modifier_Item_Polliwog_Charm_Buff: {
        },
        CDOTA_Modifier_Kez_FalconRush_Echo_Damage: {
        },
        CDOTA_Modifier_Kez_Katana_Bleed: {
        },
        CDOTA_Modifier_Primal_Beast_Uproar_Projectile_Thinker: {
        },
        CDOTA_Modifier_Dawnbreaker_Hearthfire_Thinker: {
        },
        CDOTA_Modifier_Mars_Gods_Rebuke: {
        },
        CDOTA_Modifier_Grimstroke_Ink_Trail_Debuff: {
        },
        CDOTA_Modifier_Invoke_Bonuses: {
        },
        CDOTA_Modifier_Broodmother_SpinWeb_Degen: {
        },
        CDOTA_Modifier_FacelessVoid_Chronosphere: {
        },
        CDOTA_Modifier_Courier_Shield: {
        },
        CDOTA_Modifier_Tinker_Turret: {
        },
        CDOTA_Modifier_VengefulSpirit_Revenge: {
        },
        CDOTA_Modifier_Rune_Haste: {
        },
        C_SingleplayRules: {
        },
        CLogicalEntity: {
        },
        C_DOTA_Item_Diffusal_Blade_Level2: {
        },
        C_DOTA_Item_CraniumBasher: {
        },
        C_DOTA_Item_JumpBoots: {
        },
        C_DOTA_Ability_Courier_QueuePickupFromStash: {
        },
        C_DOTA_Ability_Necrolyte_Heartstopper_Aura: {
        },
        C_DOTA_Ability_WitchDoctor_Maledict: {
        },
        CDOTA_Modifier_XP_Fountain: {
        },
        CDOTA_Modifier_Mutation_NoHealthBars_Aura: {
        },
        C_DOTA_Ability_Zombie_Berserk: {
        },
        CDOTA_Ability_Seasonal_PartyHat: {
        },
        C_DOTA_Ability_Seasonal_Decorate_Tree: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Slardar_3: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Antimage_7: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Dragon_Knight_3: {
        },
        C_DOTA_Ability_Special_Bonus_50_Crit_40: {
        },
        C_DOTA_Ability_Special_Bonus_MP_400: {
        },
        CDOTA_Modifier_Item_Craggy_Coat_Tanky: {
        },
        CDOTA_Modifier_Item_Mjollnir_Static: {
        },
        CDOTA_Modifier_Item_Shivas_Guard_Aura: {
        },
        CDOTA_Modifier_Hoodwink_TomokanTracker_Tracks: {
        },
        CDOTA_Modifier_Mars_Bulwark_Soldier_Bonus: {
        },
        CDOTA_Modifier_Techies_Arcana_DamageListener: {
        },
        CDOTA_Modifier_Slark_EssenceShift_Debuff: {
        },
        CDOTA_Modifier_Chen_DivineFavor: {
        },
        CDOTA_Modifier_Leshrac_Greater_Lightning_Storm: {
        },
        CDOTA_Modifier_Venomancer_NoxiousPlaguePrimary: {
        },
        CDOTA_Modifier_Courier_ReturnStashItems: {
        },
        CDOTA_Modifier_Axe_BerserkersCallArmor: {
        },
        CDOTA_Modifier_VengefulSpirit_Nether_Swap_PathingFix: {
        },
        CDOTA_Modifier_VengefulSpirit_Command_Negative_Aura: {
        },
        CDOTA_Muerta_Revenant: {
        },
        C_DOTA_Unit_Hero_ChaosKnight: {
        },
        C_DOTA_BaseNPC_Seasonal_Snowman: {
        },
        C_DOTA_Item_Cyclone: {
        },
        CDOTA_Ability_Tusk_IceShards_Stop: {
        },
        CDOTA_Ability_DoomBringer_Devils_Bargain: {
        },
        CDOTA_Ability_Lina_Combustion: {
        },
        CDOTA_Modifier_EnragedWildkin_ToughnessAura: {
        },
        CDOTA_Modifier_Twin_Gate_Warp_Channel: {
        },
        C_PrecipitationBlocker: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Furion_5: {
        },
        CDOTA_Modifier_Special_Bonus_Lifesteal: {
        },
        CPulseCell_WaitForPanelClass: {
        },
        CDOTA_Modifier_Item_OgreSealTotem: {
        },
        CDOTA_Modifier_Item_Overwhelming_Blink: {
        },
        CDOTA_Modifier_Terrorblade_Demon_Zeal: {
        },
        CDOTA_Modifier_AbyssalUnderlord_Firestorm_Thinker: {
        },
        CDOTA_Modifier_Abaddon_Borrowed_Time_Damage_Redirect: {
        },
        CDOTA_Modifier_Gyrocopter_HomingMissile_ChargeCounter: {
        },
        CDOTA_Modifier_FacelessVoid_TimeWalk_Slow: {
        },
        CDOTA_Modifier_Tiny_Grow: {
        },
        CDOTA_Modifier_StormSpirit_StaticRemnantThinker: {
        },
        CDOTA_Buff_Item: {
        },
        C_SoundOpvarSetPathCornerEntity: {
        },
        CPlayer_WeaponServices: {
        },
        CDOTA_Unit_Hero_Kez: {
        },
        C_DOTA_Unit_Hero_Techies: {
        },
        C_DOTA_Unit_Hero_Spectre: {
        },
        C_DOTA_Unit_Hero_DragonKnight: {
        },
        C_DOTA_Item_Orb_Of_Revelations: {
        },
        CDOTA_Item_Recipe_Keen_Optic: {
        },
        C_DOTA_Ability_Muerta_Gunslinger: {
        },
        C_DOTA_Ability_Pangolier_Rollup: {
        },
        CDOTA_Ability_Centaur_Return: {
        },
        CDOTA_Ability_Ursa_Innate_Maul: {
        },
        C_DOTA_Item_DataDriven: {
        },
        C_DOTA_Ability_Frogmen_ArmOfTheDeep: {
        },
        C_DOTA_Ability_Greevil_Miniboss_Blue_ColdFeet: {
        },
        C_DOTA_Ability_Special_Bonus_Armor_5: {
        },
        C_DOTA_Ability_Special_Bonus_Strength_30: {
        },
        C_DOTA_Ability_Special_Bonus_Gold_lvl25_l: {
        },
        CDOTA_Modifier_Item_Satanic: {
        },
        CDOTA_Modifier_Item_Hyperstone: {
        },
        CDOTA_Modifier_Hoodwink_ArcingBoomerang: {
        },
        CDOTA_Modifier_MonkeyKing_Spring_Thinker: {
        },
        CDOTA_Modifier_LegionCommander_Intimidate_DamageReduction: {
        },
        CDOTA_Modifier_Legion_Commander_PressTheAttack_Immunity: {
        },
        CDOTA_Modifier_Skywrath_Mystic_Flare_Aura_Effect: {
        },
        CDOTA_Modifier_Wisp_Tether: {
        },
        CDOTA_Modifier_Treant_Overgrowth: {
        },
        CDOTA_Modifier_Silencer_BrainDrain: {
        },
        CDOTA_Modifier_Beastmaster_PrimalRoar_Speed: {
        },
        CDOTA_Modifier_Earthshaker_Slugger_Intrinsic: {
        },
        C_DOTA_Unit_Hero_VengefulSpirit: {
        },
        CDOTA_Item_Foragers_Mana: {
        },
        C_DOTA_Item_Enhancement_Boundless: {
        },
        C_DOTA_Item_RiverPainter2: {
        },
        C_DOTA_Item_Javelin: {
        },
        C_DOTA_Ability_Pangolier_Gyroshell: {
        },
        C_DOTA_Ability_Bristleback_BrawlersGrit: {
        },
        C_DOTA_Ability_Enchantress_Bunny_Hop: {
        },
        CDOTA_Ability_Furion_Natures_Profit: {
        },
        C_DOTA_Ability_AntiMage_ManaBreak: {
        },
        CDOTA_Modifier_Mutation_Crit_Chance: {
        },
        C_TriggerVolume: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Windranger_2: {
        },
        C_DOTA_Ability_Special_Bonus_Gold_Income_30: {
        },
        CDOTA_Modifier_Item_Enhancement_Thick: {
        },
        CDOTA_Modifier_Item_UnrelentingEye: {
        },
        CDOTA_Modifier_Item_Orb_Of_Destruction: {
        },
        CDOTA_Modifier_Item_Spirit_Vessel_Damage: {
        },
        CDOTA_Modifier_Item_Armlet: {
        },
        CDOTA_Modifier_Item_DemonEdge: {
        },
        CDOTA_Modifier_Hoodwink_HeavyQuiver: {
        },
        CDOTA_Modifier_Pangolier_LuckyShot: {
        },
        CDOTA_Modifier_ArcWarden_Runic_Infusion: {
        },
        CDOTA_Modifier_Visage_GravekeepersCloak_Stack: {
        },
        CDOTA_Modifier_LoneDruid_TrueForm_Transform: {
        },
        CDOTA_Modifier_Brewmaster_Void_AstralPulse: {
        },
        CDOTA_Modifier_Clinkz_Tar_Bomb_Slow: {
        },
        CDOTA_Modifier_Pugna_NetherWard: {
        },
        CDOTA_Modifier_Beastmaster_Hawk_Perch_Flight: {
        },
        CDOTA_Modifier_Firecracker_Debuff: {
        },
        CDOTA_Modifier_Zuus_ThunderTrail_Debuff: {
        },
        C_DOTA_Unit_Hero_Lycan: {
        },
        C_DOTA_Unit_Hero_Morphling: {
        },
        C_DOTA_Item_Enhancement_Wise: {
        },
        CDOTA_Item_Recipe_Pavise: {
        },
        C_DOTA_Item_Crimson_Guard: {
        },
        C_DOTA_Ability_NyxAssassin_NeuroSting: {
        },
        C_DOTA_Ability_Tinker_MarchOfTheMachines: {
        },
        C_DOTA_Ability_Creep_Irresolute: {
        },
        C_DOTA_Ability_Special_Bonus_HP_Regen_6: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Speed_20: {
        },
        CDOTA_Modifier_Magnifying_Monocle: {
        },
        CDOTA_Modifier_Item_Medallion_Of_Courage_Armor_Reduction: {
        },
        CDOTA_Modifier_Broodmother_Spiders_Milk: {
        },
        CDOTA_Modifier_Phantom_Assassin_Blur_Manacost_Reduction: {
        },
        CDOTA_Modifier_Turbo_Courier_Invulnerable: {
        },
        CDOTA_Modifier_WitchDoctor_DeathWard_SecondaryAttack: {
        },
        CPulseCell_LimitCount: {
        },
        CPulseCell_Step_CallExternalMethod: {
        },
        C_DOTA_Unit_Hero_FacelessVoid: {
        },
        C_DynamicPropAlias_prop_dynamic_override: {
        },
        CDOTA_Item_Ward_Maker: {
        },
        C_DOTA_Item_SheepStick: {
        },
        C_DOTA_Ability_Largo_AmphibianRhapsody_Song: {
        },
        C_DOTA_Ability_Phoenix_SunRay: {
        },
        C_DOTA_Ability_SpiritBreaker_ChargeOfDarkness: {
        },
        C_DOTA_Ability_Tinker_Innate_Keen_Teleport_XP_On_Death: {
        },
        CEnvSoundscapeTriggerable: {
        },
        CDOTA_Ability_AghsFort_Creature_Phoenix_Supernova: {
        },
        CDOTA_Modifier_AghsFort_Creature_Phoenix_FireSpiritCount: {
        },
        CDOTA_Modifier_AghsFort_StonehallGeneral_OverwhelmingOdds_Thinker: {
        },
        CDOTA_Ability_Seasonal_TI9_Shovel: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Nyx_5: {
        },
        CDOTA_Modifier_Special_Bonus_Agility_And_Intelligence: {
        },
        C_DOTA_Ability_Special_Bonus_Magic_Resistance_25: {
        },
        C_DOTA_Ability_Special_Bonus_HP_Regen_3: {
        },
        CDOTA_Modifier_Item_Force_Field_Effect: {
        },
        CDOTA_Modifier_Dagon: {
        },
        CDOTA_Modifier_Magnataur_Skewer_Movement: {
        },
        CDOTA_Modifier_TrollWarlord_WhirlingAxes_Slow: {
        },
        CDOTA_Modifier_Gyrocopter_Afterburner: {
        },
        CDOTA_Modifier_Weaver_Threads_Of_Fate_Linger: {
        },
        CDOTA_Modifier_Nian_Hurricane_Whirlpool: {
        },
        CDOTA_Modifier_Crystal_Maiden_Crystal_Clone: {
        },
        CDOTA_Modifier_Juggernaut_BladeFury_Pull: {
        },
        C_DOTA_BaseNPC_Venomancer_PlagueWard: {
        },
        C_DOTA_GuildBannerProp: {
        },
        C_DOTA_Item_Dezun_Bloodrite: {
        },
        C_DOTA_Item_Safety_Bubble: {
        },
        C_DOTA_Item_Stormcrafter: {
        },
        C_DOTA_Item_Desolator: {
        },
        C_DOTA_Item_Blade_Mail: {
        },
        C_DOTA_Ability_MonkeyKing_Spring: {
        },
        C_DOTA_Ability_EarthSpirit_StoneCaller: {
        },
        C_DOTA_Ability_Medusa_ManaShield: {
        },
        C_DOTA_Ability_Lycan_Howl: {
        },
        C_DOTA_Ability_BountyHunter_WindWalk_Ally: {
        },
        C_DOTA_Ability_Clinkz_Burning_Army: {
        },
        C_DOTA_Ability_Leshrac_Pulse_Nova: {
        },
        CDOTA_Ability_CrystalMaiden_IceRink: {
        },
        C_DOTA_Ability_Juggernaut_Omnislash: {
        },
        CDOTA_Modifier_AlphaWolf_CriticalStrike: {
        },
        CDOTA_Ability_Seasonal_Firecrackers: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Pugna_1: {
        },
        CDOTA_Modifier_Special_Bonus_Intelligence: {
        },
        C_DOTA_Ability_Special_Bonus_Lifesteal_25: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Lifesteal_12: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Speed_55: {
        },
        CDOTA_Modifier_Item_Mekansm_Noheal: {
        },
        CDOTA_Modifier_SheepStick_Debuff: {
        },
        CDOTA_Modifier_Pangolier_Gyroshell_Stun: {
        },
        CDOTA_Modifier_Winter_Wyvern_Accelerated_Learning: {
        },
        CDOTA_Modifier_EmberSpirit_SleightOfFist_InProgress: {
        },
        CDOTA_Modifier_Obsidian_Destroyer_ManaCost: {
        },
        CDOTA_Modifier_DoomBringer_ScorchedEarthEffectAura: {
        },
        CDOTA_Modifier_Enchantress_Little_Friends: {
        },
        CDOTA_Modifier_Life_Stealer_Open_Wounds: {
        },
        CDOTA_Modifier_Viper_Predator: {
        },
        CDOTA_Modifier_DeathProphet_SpiritSiphon_Fear: {
        },
        CDOTA_Modifier_Roshan_RevengeRoar: {
        },
        CDOTA_Modifier_CDOTA_Ability_Axe_One_Man_Army: {
        },
        CDOTA_Modifier_VengefulSpirit_XP_Earn: {
        },
        CDOTA_Modifier_Silence: {
        },
        CDOTA_Modifier_ModelScaleAnimateTime: {
        },
        CDOTA_Modifier_StackedNeutral: {
        },
        C_DOTA_Unit_Hero_Omniknight: {
        },
        CDOTA_Item_Essence_Ring: {
        },
        C_DOTA_Ability_Meepo_Geomancy: {
        },
        C_DOTA_Ability_QueenOfPain_Innate_Seduction: {
        },
        CDOTA_Ability_Pudge_Eject: {
        },
        CDOTA_Modifier_GiantWolf_Intimidate: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Centaur_5: {
        },
        CDOTA_Modifier_Special_Bonus_Corruption_Debuff: {
        },
        CDOTA_Modifier_Falconers_Glove: {
        },
        CDOTA_Modifier_Orb_Of_Corrosion_Debuff: {
        },
        CDOTA_Modifier_Item_GemOfTrueSight: {
        },
        CDOTA_Modifer_Item_GlovesOfHaste: {
        },
        CDOTA_Modifier_Pangolier_HeartPiercer_Debuff: {
        },
        CDOTA_Modifier_Razor_EyeOfTheStorm_Passive: {
        },
        CDOTA_Modifier_Razor_StaticLink_Debuff: {
        },
        CHeroStatueLiked: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CSkyboxReference: {
        },
        C_PortraitWorldPet: {
        },
        C_DOTA_Ability_Terrorblade_Sunder: {
        },
        C_DOTA_Ability_AbyssalUnderlord_Dark_Portal: {
        },
        CDOTA_Ability_Roshan_Bash: {
        },
        C_DOTA_Ability_Kunkka_Torrent_Storm: {
        },
        C_DOTA_Ability_BlackDragon_Fireball: {
        },
        C_DOTA_Ability_MonkeyKing_FurArmy: {
        },
        CDOTA_Modifier_AghsFort_Ascension_Silence_Charge: {
        },
        C_DOTA_Ability_Creature_Ice_Breath: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Medusa_4: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Amplify_20: {
        },
        C_DOTA_Ability_Special_Bonus_Cleave_175: {
        },
        C_DOTA_Ability_Special_Bonus_Corruption_4: {
        },
        CDOTA_Modifier_Item_Harpoon_Slow: {
        },
        CDOTA_Modifier_Item_Dragon_Scale: {
        },
        CDOTA_Modifier_Item_Soul_Ring: {
        },
        CDOTA_Modifier_Dawnbreaker_Luminosity_Attack_Buff: {
        },
        CDOTA_Modifier_Wisp_Tether_Scepter: {
        },
        CDOTA_Modifier_Invoker_Magic_Amp_Debuff: {
        },
        CDOTA_Modifier_Omniknight_GuardianAngel_Self_Buff: {
        },
        CDOTA_Modifier_TemplarAssassin_Trap_Slow: {
        },
        CDOTA_Modifier_FacelessVoid_TimeDilation_Slow: {
        },
        CDOTA_Modifier_Nian_Frenzy: {
        },
        CDOTA_Modifier_Morphling_ScepterStatsDrain_All_Buff: {
        },
        CDOTA_Modifier_Tutorial_ForceAnimation: {
        },
        CDOTA_Item_Recipe_Moonshard: {
        },
        C_DOTA_Item_SangeAndYasha: {
        },
        C_DOTA_Item_Recipe_CraniumBasher: {
        },
        C_DOTA_Ability_Wisp_Spirits_Out: {
        },
        C_DOTA_Ability_Ogre_Magi_DumbLuck: {
        },
        C_DOTA_Ability_Treant_NaturesGrasp: {
        },
        C_DOTA_Ability_Gyrocopter_Homing_Missile: {
        },
        C_DOTA_Ability_Huskar_Life_Break: {
        },
        C_DOTA_Ability_DeathProphet_SpiritCollector: {
        },
        C_DOTA_Ability_Puck_EtherealJaunt: {
        },
        CDOTA_Ability_Axe_BerserkersCall: {
        },
        CFilterClass: {
        },
        C_PointCameraVFOV: {
        },
        C_PointCamera: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Chen_2: {
        },
        CDOTA_Modifier_Special_Bonus_Spell_Immunity: {
        },
        C_DOTA_Ability_Special_Bonus_Mana_Break_25: {
        },
        CDOTA_Modifier_Light_Collector: {
        },
        CDOTA_Modifier_Necronomicon_Archer_AoE: {
        },
        CDOTA_Modifier_Item_UltimateScepter: {
        },
        CDOTA_Modifier_Item_MithrilHammer: {
        },
        CDOTA_Modifier_Dawnbreaker_Solar_Guardian_Thinker: {
        },
        CDOTA_Modifier_Bristleback_QuillSpray: {
        },
        CDOTA_Modifier_Centaur_Hitched_Into_Cart: {
        },
        CDOTA_Modifier_Lycan_SummonWolves_Bash: {
        },
        CDOTA_Modifier_Invoker_Wex_Intrinsic: {
        },
        CDOTA_Modifier_Clinkz_Burning_Army: {
        },
        CDOTA_Modifier_DeathProphet_SpiritSiphon_Buff: {
        },
        CDOTA_Modifier_Venomancer_WardCounter: {
        },
        CDOTA_Modifier_Nian_Whirlpool_Pull: {
        },
        CDOTA_Modifier_Lich_ChainFrost_OnDeath: {
        },
        CDOTA_Modifier_VengeulSpirit_SoulStrike: {
        },
        C_DOTA_PhantomAssassin_Gravestone: {
        },
        C_DOTA_Item_Recipe_Naginata: {
        },
        C_DOTA_Item_Mekansm: {
        },
        C_DOTA_Ability_EarthSpirit_GeomagneticGrip: {
        },
        C_DOTA_Ability_Meepo_Innate_PackRat: {
        },
        CDOTA_Modifier_Greevil_Miniboss_Sight: {
        },
        C_DOTA_Ability_BigThunderLizard_Frenzy: {
        },
        C_DOTA_Ability_CentaurKhan_EnduranceAura: {
        },
        CPathWithDynamicNodes: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Muerta_TrickShotCharges: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Death_Prophet_3: {
        },
        C_DOTA_Ability_Special_Bonus_Cast_Range_325: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Speed_160: {
        },
        CPanel2DAPI: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Modifer_Item_Blitz_Knuckles: {
        },
        CDOTA_Modifier_EarthSpirit_StoneCaller_Innate: {
        },
        CDOTA_Modifier_Techies_SnareTrap_Slowed: {
        },
        CDOTA_Modifier_Magnataur_Shockwave: {
        },
        CDOTA_Modifier_Magnataur_Shockwave_Erupt: {
        },
        CDOTA_Modifier_NagaSiren_MirrorImage: {
        },
        CDOTA_Modifier_Undying_Tombstone_Zombie_Modifier: {
        },
        CDOTA_Modifier_ChaosKnight_Reality_Rift_Debuff: {
        },
        CDOTA_Modifier_Silencer_GlaivesOfWisdom_Debuff_Bonus: {
        },
        CDOTA_Modifier_SpiritBreaker_KnockbackAmpAura: {
        },
        CDOTA_Modifier_BountyHunter_TrackEffect: {
        },
        CDOTA_Modifier_CrystalMaiden_CrystalNova_Barrier: {
        },
        C_DOTA_Item_Recipe_Vambrace: {
        },
        C_DOTA_Ability_Primal_Beast_Innate_Status_Resistance_Per_Time: {
        },
        C_DOTA_Ability_Invoker_ChaosMeteor_AD: {
        },
        CBaseFilter: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_VoidSpirit_4: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Vengeful_Spirit_9: {
        },
        C_DOTA_Ability_Special_Bonus_Gold_Income_210: {
        },
        C_DOTA_Ability_Special_Bonus_Magic_Resistance_50: {
        },
        C_DOTAAppCheers: {
        },
        CDOTA_Modifier_Item_Angels_Demise_Slow: {
        },
        CDOTA_Modifier_Item_Havoc_Hammer: {
        },
        CDOTA_Modifier_Item_Butterfly_Extra: {
        },
        CDOTA_Modifier_Visage_SummonFamiliars_StoneForm_Thinker: {
        },
        CDOTA_Modifier_DarkSeer_WallOfReplica_Slow: {
        },
        CDOTA_Modifier_NianChargePinned: {
        },
        CDOTA_Modifier_SkeletonKing_SpectralBlade_Curse: {
        },
        PulseObservableBoolExpression_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        C_DOTA_Unit_Hero_Phoenix: {
        },
        C_DOTA_Unit_Hero_Bloodseeker: {
        },
        C_PortraitWorldLight: {
        },
        C_PointEntityAlias_info_target_portrait_root: {
        },
        CDOTA_Item_Recipe_Trident: {
        },
        CDOTA_Ability_Marci_Bodyguard: {
        },
        CDOTA_Ability_Centaur_Stampede: {
        },
        C_DOTA_Ability_Keeper_of_the_Light_ManaMagnifier: {
        },
        C_DOTA_Ability_Undying_FleshGolem: {
        },
        C_DOTA_Ability_Windrunner_EasyBreezy: {
        },
        C_DOTA_Ability_ForestTrollHighPriest_Heal: {
        },
        CDOTA_Modifier_Watcher_State: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Venomancer_5: {
        },
        CDOTA_Modifier_Special_Bonus_Armor: {
        },
        C_DOTA_Ability_Special_Bonus_Strength_5: {
        },
        C_DOTA_Ability_Special_Bonus_HP_475: {
        },
        C_DOTA_Ability_Special_Bonus_HP_375: {
        },
        CDOTA_Modifier_Item_Falcon_Blade_Mana_Stack: {
        },
        CDOTA_Modifier_Item_Essence_Distiller: {
        },
        CDOTA_Modifier_AghanimsScepter_WaitForUpgradeSelected: {
        },
        CDOTA_Modifier_Ringmaster_Wheel_Knockback: {
        },
        CDOTA_Modifier_NagaSiren_Crit_Passive: {
        },
        CDOTA_Modifier_Enchantress_RabbleRouser_Aura: {
        },
        CDOTA_Modifier_Miniboss_Alleviation_Active: {
        },
        CDOTA_Modifier_Lina_Fiery_Cloak: {
        },
        CDOTA_Modifier_DrowRanger_FrostArrows: {
        },
        C_DOTA_Unit_Broodmother_Spiderling: {
        },
        C_DOTA_BaseNPC_HallofFame: {
        },
        CDOTA_Minesweeper_EffectsEntity: {
        },
        C_DOTA_Item_SpecialistsArray: {
        },
        C_DOTA_Item_Recipe_Psychic_Headband: {
        },
        C_DOTA_Ability_Necronomicon_Archer_ManaBurn: {
        },
        C_DOTA_Ability_Invoker_GhostWalk_AD: {
        },
        C_DOTA_Ability_DragonKnight_BreatheFire: {
        },
        C_DOTA_Ability_Luna_LunarOrbit: {
        },
        C_DOTA_Ability_Enigma_Malefice: {
        },
        C_DOTA_Ability_Bane_Nightmare: {
        },
        C_DOTA_Ability_Greevil_Miniboss_Black_Nightmare: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Luna_4: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_70: {
        },
        C_DOTA_Ability_Special_Bonus_Intelligence_25: {
        },
        C_DOTA_Ability_Special_Bonus_MP_Regen_250: {
        },
        C_DOTA_Ability_Special_Bonus_HP_225: {
        },
        CDOTA_Modifier_Item_ManaDraught: {
        },
        CDOTA_Modifier_Item_DragonLance: {
        },
        CDOTA_Modifier_Item_Mjollnir: {
        },
        CDOTA_Modifier_Kez_TalonToss_Silence: {
        },
        CDOTA_Modifier_Marci_Lunge_Arc: {
        },
        CDOTA_Modifier_Phoenix_SupernovaHiding: {
        },
        CDOTA_Modifier_Abaddon_BorrowedTime_ImmolationDamage: {
        },
        CDOTA_Modifier_Tusk_Snowball_Target: {
        },
        CDOTA_Modifier_Visage_GravekeepersCloak: {
        },
        CDOTA_Modifier_Silencer_GlaivesOfWisdom_Debuff: {
        },
        CDOTA_Modifier_Chen_HandOfGod_Hot: {
        },
        CDOTA_Modifier_Dazzle_NothlProjection_Knockback: {
        },
        CDOTA_Modifier_AntiMage_Blink_Illusion: {
        },
        C_DOTA_Ability_Clinkz_BurningBarrage: {
        },
        C_DOTA_Ability_Templar_Assassin_Third_Eye: {
        },
        C_DOTA_Ability_Courier_GoToSideShop: {
        },
        CDOTA_Modifier_Creep_Bonus_XP: {
        },
        CDOTA_Modifier_AghsFort_TorrentEffectPotion_Thinker: {
        },
        CDOTA_Modifier_Morty_Hop_Controller: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Snapfire_5: {
        },
        CDOTA_Modifier_Item_MartyrsPlate_Aura: {
        },
        CDOTA_Modifier_Item_Sphere_Target_Enemy: {
        },
        CDOTA_Modifier_Kez_GrapplingClaw_LandingAnim: {
        },
        CDOTA_Modifier_Pangolier_Swashbuckle_ChargeCounter: {
        },
        CDOTA_Modifier_AbyssalUnderlord_AtrophyAura_Scepter: {
        },
        CDOTA_Modifier_Troll_Warlord_Rampage: {
        },
        CDOTA_Modifier_PhantomAssassin_Gravestone: {
        },
        CDOTA_Modifier_DeathProphet_SpiritCollector: {
        },
        CDOTA_Modifier_Morphling_Syntropy: {
        },
        CEntityIdentity: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CPulseCell_LimitCount__Criteria_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Unit_Underlord_Portal: {
        },
        C_DOTA_Item_WraithPact: {
        },
        C_DOTA_Ability_MonkeyKing_TreeDance: {
        },
        C_DOTA_Ability_Skywrath_Mage_Mystic_Flare: {
        },
        C_DOTA_Ability_Ogre_Magi_Bloodlust: {
        },
        C_DOTA_Ability_Ogre_Magi_Ignite: {
        },
        C_DOTA_Ability_Bear_Empty2: {
        },
        C_DOTA_Ability_SpiritBreaker_PlanarPocket: {
        },
        C_DOTA_Ability_Venomancer_NoxiousPlague: {
        },
        C_DOTA_Ability_Windrunner_GaleForce: {
        },
        C_DOTA_Ability_Zuus_Lightning_Hands: {
        },
        CDOTA_Modifier_Mutation_Vampire_Aura: {
        },
        CDOTA_Modifier_Seasonal_TI11_BubbleGun: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Base_Damage_30: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Speed_80: {
        },
        C_DOTA_Ability_Special_Bonus_MP_1000: {
        },
        CDOTA_Modifier_Item_Slippers: {
        },
        CDOTA_Modifier_Ringmaster_TameTheBeasts_Thinker: {
        },
        CDOTA_Modifier_PrimalBeast_Pulverize: {
        },
        CDOTA_Modifier_Dawnbreaker_Solar_Guardian_LandingStun: {
        },
        CDOTA_Modifier_Snapfire_LilShredder_Attack: {
        },
        CDOTA_Modifier_Grimstroke_InkCreature: {
        },
        CDOTA_Modifier_Pangolier_GyroshellTimeout: {
        },
        CDOTA_Modifier_Winter_Wyvern_Winters_Curse_Kill_Credit: {
        },
        CDOTA_Modifier_BlindingLight_Knockback: {
        },
        CDOTA_Modifier_PhantomAssassin_ScreenSplatter: {
        },
        CDOTA_Modifier_Enigma_BlackHole_Pull: {
        },
        CDOTA_Modifier_Morphling_ScepterStatsDrain_Intelligence_Debuff: {
        },
        CDOTA_Modifier_Bane_FiendsGrip_Illusion_Can_Only_Channel: {
        },
        CDOTA_Modifier_DataDriven: {
        },
        C_DOTA_Unit_Hero_Broodmother: {
        },
        C_DOTA_Item_EchoSabre: {
        },
        C_DOTA_Item_TierToken: {
        },
        C_DOTA_Ability_Largo_AmphibianRhapsody_DoubleTime: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Hoodwink_Bushwhack_Duration: {
        },
        CDOTA_Ability_Winter_Wyvern_Arctic_Burn: {
        },
        CDOTA_Ability_Nyx_Assassin_Jolt: {
        },
        C_DOTA_Ability_Undying_Tombstone: {
        },
        C_DOTA_Ability_PhantomAssassin_PhantomStrike: {
        },
        CDOTA_Ability_Puck_Puckish: {
        },
        C_DOTA_Ability_PhantomLancer_SpiritLance: {
        },
        CDOTA_Modifier_FelBeast_Haunt: {
        },
        CDOTA_Modifier_Neutral_SpellImmunity_Visible: {
        },
        C_DOTA_PushWave: {
        },
        CDOTA_Modifier_Mutation_Treecutter: {
        },
        CBasePlayerVData: {
        },
        CDOTA_Modifier_Special_Bonus_Gold_Income: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Visage_5: {
        },
        CDOTA_Modifier_Item_Enhancement_Audacious: {
        },
        CDOTA_Modifier_Item_Paladin_Sword: {
        },
        CDOTA_Modifier_Ringmaster_StrongmanTonic_ScaleSpeed: {
        },
        CDOTA_Modifier_Muerta_Ofrenda_DamageAmp: {
        },
        CDOTA_Modifier_Snapfire_Buckshot: {
        },
        CDOTA_Modifier_Terrorblade_Metamorphosis_Fear_Thinker: {
        },
        CDOTA_Modifier_LegionCommander_Intimidate: {
        },
        CDOTA_Modifier_Magnataur_ReversePolarity: {
        },
        CDOTA_Modifier_Omniknight_Repel: {
        },
        CDOTA_Modifier_Item_Vermillion_Robe_Flames: {
        },
        CDOTA_Modifier_Riki_SmokeScreen: {
        },
        CDOTA_BaseNPC_Seasonal_Dragon: {
        },
        C_LightSpotEntity: {
        },
        C_DOTA_Item_HeavensHalberd: {
        },
        C_DOTA_Item_Cheese: {
        },
        C_DOTA_Ability_Muerta_TheCalling: {
        },
        C_DOTA_Ability_LoneDruid_SpiritBear_Entangle: {
        },
        C_DOTA_Ability_TemplarAssassin_PsiBlades: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Slardar_2: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Mirana_4: {
        },
        C_DOTA_Ability_Special_Bonus_Magic_Resistance_10: {
        },
        CDOTA_Modifier_Item_Eternal_Shroud: {
        },
        CDOTA_Modifier_Magnataur_Shockwave_Eruption_Slow: {
        },
        CDOTA_Modifier_DeathProphet_Slow: {
        },
        CDOTA_Modifier_Enigma_DemonicConversion_ModelScale: {
        },
        CPulseCell_CursorQueue: {
        },
        CPulseCell_Value_RandomFloat: {
        },
        CPulseExecCursor: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_DOTA_Unit_Brewmaster_PrimalEarth: {
        },
        C_DOTA_Unit_LoopingSound: {
        },
        CDOTA_Item_Recipe_Ninja_Gear: {
        },
        C_DOTA_Ability_Dawnbreaker_Solar_Guardian: {
        },
        C_DOTA_Ability_Lich_FrostArmor: {
        },
        C_Sprite: {
        },
        C_DotaSubquestBase: {
        },
        CDOTA_Modifier_AghsFort_Watch_Tower_Capturing: {
        },
        CDOTA_Ability_AghsFort_ShadowWaveEffectPotion: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Riki_7: {
        },
        C_DOTA_Ability_Special_Bonus_Movement_Speed_20: {
        },
        CDOTA_Modifier_Item_Princes_Knife_Hex: {
        },
        CDOTA_Modifier_Skywrath_Mage_Arcana_Kill_Effect: {
        },
        CDOTA_Modifier_Weaver_Shukuchi_GeminateAttackMark: {
        },
        C_DOTA_Unit_Hero_LoneDruid: {
        },
        C_DOTA_Item_Recipe_Giants_Ring: {
        },
        C_DOTA_Item_Infused_Raindrop: {
        },
        C_DOTA_Ability_Necronomicon_Warrior_Sight: {
        },
        C_DOTA_Item_Dagon_Upgraded3: {
        },
        C_DOTA_Item_Reaver: {
        },
        C_DOTA_Item_Recipe_NullTalisman: {
        },
        C_DOTA_Ability_Invoker_EMP_AD: {
        },
        CDOTA_Ability_CallOfTheWild_Boar_Poison: {
        },
        C_DOTA_Ability_DrowRanger_FrostArrows: {
        },
        CDOTA_Ability_Juggernaut_Swift_Slash: {
        },
        C_DOTA_Item_DeathGoldDropped: {
        },
        CDOTA_Ability_Generic_Hidden: {
        },
        CDOTA_Modifier_AghsFort_TreantMiniboss_NaturesGuise_Tree_Walking: {
        },
        CDOTA_Ability_AghsFort_DragonKnight_BreatheFire: {
        },
        CDOTA_Modifier_AghsFort_ExplosiveBarrel: {
        },
        CDOTA_Ability_Aghsfort_Reward_CritAura: {
        },
        CDOTA_Modifier_Seasonal_Summon_Snowman_Thinker: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Oracle_5: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Doom_9: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Amplify_4: {
        },
        CDOTA_Modifier_Magnataur_Empower_StackCounter: {
        },
        CDOTA_Modifier_Magnataur_Empower: {
        },
        CDOTA_Modifier_Centaur_Stampede: {
        },
        CDOTA_Modifier_Broodmother_InsatiableHunger: {
        },
        CDOTA_Modifier_Phantom_Assassin_GroundDagger: {
        },
        CDOTA_Modifier_Lich_Ice_Spire: {
        },
        CDOTA_Modifier_Puck_DreamCoil_Thinker: {
        },
        CDOTA_Modifier_VengefulSpirit_Arcana: {
        },
        CDOTA_Modifier_TrueSight: {
        },
        CDOTA_Modifier_Bonus_Damage: {
        },
        C_PhysicsProp: {
        },
        CFilterTeam: {
        },
        CDOTA_Item_Quickening_Charm: {
        },
        C_DOTA_Item_Recipe_Mjollnir: {
        },
        C_DOTA_Ability_Silencer_GlaivesOfWisdom: {
        },
        C_DOTA_Ability_Tiny_CraggyExterior: {
        },
        C_DOTA_Ability_OgreMagi_FrostArmor: {
        },
        CDOTA_Aghsfort_Modifier_Magnus_Push_Skewer_Movement: {
        },
        CDOTA_Modifier_Seasonal_TI9_Drums_Thinker: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Damage_60: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Base_Damage_100: {
        },
        C_DOTA_Ability_Special_Bonus_HP_Regen_40: {
        },
        CDOTA_Modifier_Dezun_Bloodrite: {
        },
        CDOTA_Modifier_Item_Woodland_Striders: {
        },
        CDOTA_Modifier_Item_Boots_Of_Bearing: {
        },
        CDOTA_Modifier_Mars_GodsRebuke_Crit: {
        },
        CDOTA_Modifier_Mars_ArenaOfBlood_VisionObstructionAura: {
        },
        CDOTA_Modifier_DarkWillow_ShadowRealm_Buff: {
        },
        CDOTA_Modifier_Winter_Wyvern_Winterproof: {
        },
        CDOTA_Modifier_Phoenix_FireSpiritCount: {
        },
        CDOTA_Modifier_Brewmaster_PrimalSplitDelay: {
        },
        CDOTA_Modifier_TemplarAssassin_Meditation_Illusion: {
        },
        CDOTA_Modifier_Nian_Damage_Reflection: {
        },
        CDOTA_Modifier_SandKing_CausticFinale: {
        },
        CDOTA_Unit_Hero_Ringmaster: {
        },
        C_InfoPlayerStartDota: {
        },
        C_DOTA_Item_Royale_With_Cheese: {
        },
        CDOTA_Item_Mask_Crit_Lifesteal: {
        },
        CInfoInteraction: {
        },
        CDOTA_Modifier_Creature_HybridFlyer: {
        },
        C_DOTA_Ability_Slithereen_Riptide: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Ursa_8: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Drow_Ranger_2: {
        },
        CDOTA_Modifier_Special_Bonus_Strength: {
        },
        C_DOTA_Ability_Special_Bonus_Lifesteal_8: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Speed_30: {
        },
        C_DOTA_Ability_Special_Bonus_HP_200: {
        },
        CDOTA_Modifier_Item_Ceremonial_Robe: {
        },
        CDOTA_Modifier_Item_Hurricane_Pike: {
        },
        CDOTA_Modifier_Item_Solar_Crest_Armor_Reduction: {
        },
        CDOTA_Modifier_Smoke_Of_Deceit: {
        },
        CDOTA_Modifier_Item_RingOfHealth: {
        },
        CDOTA_Modifier_Mars_Bulwark_Active: {
        },
        CDOTA_Modifier_Visage_SummonFamiliars_DamageCharge: {
        },
        CDOTA_Modifier_Obsidian_Destroyer_ObjurgationMana: {
        },
        CDOTA_Modifier_Jakiro_IcePath_Stun: {
        },
        CDOTA_Modifier_Enchantress_Enchant_Controlled: {
        },
        CDOTA_Modifier_QueenOfPain_ShadowStrike_Buff: {
        },
        CDOTA_Modifier_Firework_Mine: {
        },
        CDOTA_Modifier_Animation_TailSpin: {
        },
        CDOTA_Modifier_StormSpirit_BallLightning: {
        },
        CBasePlayerWeaponVData: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        C_DOTA_Item_Tier3Token: {
        },
        C_DOTA_Item_Recipe_Hood_Of_Defiance: {
        },
        C_DOTA_Ability_Hoodwink_HuntersQuiver: {
        },
        CDOTA_Ability_Tusk_Drinking_Buddies: {
        },
        CDOTA_Ability_Brewmaster_LiquidCourage: {
        },
        C_DOTA_Ability_Weaver_Shukuchi: {
        },
        CDOTA_Ability_MudGolem_CloakAura: {
        },
        CDOTA_Modifier_AlphaWolf_CommandAura: {
        },
        CDOTA_Modifier_AghsFort_Lifestealer_Enraged_Pulse: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Keeper_of_the_Light_2: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Antimage_5: {
        },
        CDOTA_Modifier_Special_Bonus_Respawn_Reduction: {
        },
        C_DOTA_Ability_Special_Bonus_Attack_Speed_25: {
        },
        C_DOTA_DataSpectator: {
        },
        CDOTA_Modifier_Phylactery_KillCredit: {
        },
        CDOTA_Modifier_Voidwalker_Phased: {
        },
        CDOTA_Modifier_Item_Butterfly: {
        },
        CDOTA_Modifier_Tusk_Snowball_Movement_Friendly: {
        },
        CDOTA_Modifier_Gyrocopter_Homing_Rocket_Barrage: {
        },
        CDOTA_Modifier_IonShell: {
        },
        CDOTA_Modifier_DragonKnight_CorrosiveBreathDoT: {
        },
        CDOTA_Modifier_CallOfTheWild_Hawk_Reveal: {
        },
        CDOTA_Modifier_JumpBoots: {
        },
        CDOTA_Modifier_Tiny_Rocksteady: {
        },
        CDOTA_Modifier_CrystalMaiden_BrillianceAura: {
        },
        CDOTA_Modifier_VengefulSpirit_Hybrid_Special: {
        },
        CDOTA_Modifier_Juggernaut_BladeFury_PullAura: {
        },
        CDOTA_Modifier_Lua: {
        },
        CDOTA_Modifier_Rune_Invisibility: {
        },
        CDOTA_Modifier_ProjectileVision: {
        },
        C_DOTAGamerulesProxy: {
        },
        C_DOTAGameManagerProxy: {
        },
        C_DOTA_Unit_Hero_Lina: {
        },
        C_DOTA_Item_Recipe_Satanic: {
        },
        CDOTA_Ability_Elder_Titan_EarthSplitter: {
        },
        C_DOTA_Ability_Wisp_Empty1: {
        },
        C_DOTA_Ability_Life_Stealer_Feast: {
        },
        C_DOTA_Ability_Pugna_NetherBlast: {
        },
        C_DOTA_Ability_Slardar_Slardar_SeabornSentinel: {
        },
        C_DOTA_Ability_Windrunner_Shackleshot: {
        },
        CDOTA_Ability_Zuus_Thunder_Trail: {
        },
        CInfoParticleTarget: {
        },
        C_DOTA_Ability_Aghsfort_Pugna_Grandmaster_NetherWard: {
        },
        CDOTA_Ability_AghsFort_TreantMiniboss_NaturesGuise: {
        },
        CDOTA_Ability_Aghsfort_Reward_MagicResistAura: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Arc_Warden_4: {
        },
        C_DOTA_Ability_Special_Bonus_Unique_Lone_Druid_11: {
        },
        C_DOTA_Ability_Special_Bonus_Cast_Range_50: {
        },
        C_DOTA_Ability_Special_Bonus_HP_Regen_15: {
        },
        C_DOTA_Ability_Special_Bonus_Cleave_130: {
        },
        CDOTA_Modifier_CripplingCrossbow_Debuff: {
        },
        CDOTA_Modifier_Fusion_runeHealing: {
        },
        CDOTA_Modifier_Marci_Bodyguard_Self: {
        },
        CDOTA_Modifier_Dawnbreaker_Fire_Wreath_Attack_Bonus: {
        },
        CDOTA_Modifier_Grimstroke_SpiritWalk_Buff: {
        },
        CDOTA_Modifier_Centaur_HoofStomp_Windup: {
        },
        CDOTA_Modifier_KeeperOfTheLight_Will_O_Wisp_Aura: {
        },
        CDOTA_Modifier_Treant_NaturesGrasp_Damage: {
        },
        CDOTA_Modifier_Silencer_GlaivesOfWisdom_AttackCounter: {
        },
        CDOTA_Modifier_Alchemist_AcidSpray_Thinker: {
        },
        CDOTA_Modifier_Crystal_Maiden_GlacialGuard_Passive: {
        },
        CDOTA_Modifier_Drow_Ranger_Glacier_Hilltop_Removal: {
        },
        CDOTA_Modifier_Drow_Ranger_Glacier_Hilltop: {
        },
        C_DOTA_Unit_Hero_QueenOfPain: {
        },
        C_DOTA_Item_Divine_Regalia: {
        },
        C_DOTA_Item_UnrelentingEye: {
        },
        C_DOTA_Item_UnstableWand: {
        },
        C_DOTA_Item_Vambrace: {
        },
        CDOTA_Item_Recipe_GreatFamango: {
        },
        C_DOTA_Ability_NianCharge: {
        },
        C_DOTA_Ability_Riki_Backstab: {
        },
        CDOTA_Modifier_BlackDragon_DragonhideAura: {
        },
        C_PointClientUIWorldPanel: {
        },
        CDOTA_Ability_Seasonal_Summon_Dragon_Thinker: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Lycan_3: {
        },
        PlayerNeutralChoices_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTA_Modifier_Item_PogoStick: {
        },
        CDOTA_Modifier_Item_Necronomicon_Archer_Aura: {
        },
        CDOTA_Modifier_Item_UltimateScepter_Consumed_Alchemist: {
        },
        CDOTA_Modifier_Tusk_Tag_Team_Aura: {
        },
        CDOTA_Modifier_Medusa_VenomedVolley_Slow: {
        },
        CDOTA_Modifier_Rubick_SpellSteal: {
        },
        CDOTA_Modifier_SpiritBreaker_HerdMentality: {
        },
        CDOTA_Modifier_Huskar_Inner_Vitality: {
        },
        CDOTA_Modifier_TemplarAssassin_PsiBlades: {
        },
        CDOTA_Modifier_Roshan_Slam: {
        },
        CDOTA_Modifier_Lion_Voodoo: {
        },
        CDOTA_Modifier_Zuus_Lightning_Hands: {
        },
        CDOTA_Modifier_SkeletonKing_SpectralBlade_CurseCooldown: {
        },
        C_EntityFlame: {
        },
        C_DOTA_Item_Recipe_AeonDisk: {
        },
        C_DOTA_Ability_Hoodwink_TomokanTracker: {
        },
        C_DOTA_Ability_Tusk_Launch_Snowball: {
        },
        C_DOTA_Ability_BountyHunter_Big_Game_Hunter: {
        },
        CDOTA_Modifier_AghsFort_TreantMiniboss_NaturesGuise_Invis: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Hoodwink_AcornShotCharges: {
        },
        C_DOTA_Ability_Special_Bonus_Respawn_Reduction_30: {
        },
        C_DOTA_Ability_Special_Bonus_Spell_Amplify_3: {
        },
        C_DOTA_Ability_Special_Bonus_Movement_Speed_Percentage_10: {
        },
        CDOTA_Modifier_Giant_Maul_Debuff: {
        },
        CDOTA_Modifier_Spell_Prism_Active: {
        },
        CDOTA_Modifier_Item_Assault_Cuirass_Negative_Armor: {
        },
        CDOTA_Modifier_Largo_FroglingBand_Self: {
        },
        CDOTA_Modifier_Kez_RavensVeil_Blind: {
        },
        CDOTA_Modifier_Bristleback_ViscousNasalGoo: {
        },
        CDOTA_Modifier_Lycan_SummonWolves_Hamstring_DamageAmp: {
        },
        CDOTA_Modifier_Lycan_SummonWolves_Health: {
        },
        CDOTA_Modifier_Obsidian_Destroyer_EssenceAura_Effect: {
        },
        CDOTA_Modifier_Clinkz_Tar_Bomb_Thinker: {
        },
        CDOTA_Modifier_Furion_Sprout_Blind_Aura: {
        },
        CDOTA_Modifier_Phased: {
        },
        CDOTA_Modifier_Rune_Shield: {
        },
        C_DOTA_Unit_Hero_Nevermore: {
        },
        CDOTA_BaseNPC_Seasonal_TI9_Monkey: {
        },
        C_DOTAPlayerController: {
        },
        C_DOTA_Item_Recipe_Stormcrafter: {
        },
        C_DOTA_Item_Greater_Faerie_Fire: {
        },
        C_DOTA_Item_Recipe_Blood_Grenade: {
        },
        C_DOTA_Item_Smoke_Of_Deceit: {
        },
        C_DOTA_Item_Perseverance: {
        },
        C_DOTA_Ability_PrimalBeast_Trample: {
        },
        C_DOTA_Ability_LoneDruid_Rabid: {
        },
        C_DOTA_Ability_Spectre_Desolate: {
        },
        C_DOTA_Ability_BountyHunter_Track: {
        },
        C_DOTA_DeathProphet_Exorcism_Spirit: {
        },
        CDOTA_Ability_GraniteGolem_HPAura: {
        },
        CDOTA_Modifier_731_Teaser_Thinker: {
        },
        CDOTA_Modifier_Mutation_DeathExplosionDelayed: {
        },
        CBasePlayerController: {
        },
        CDOTA_Modifier_AghsFort_Shadow_Demon_Shadow_Poison: {
        },
        C_DOTA_Ability_AghsFort_RockGolem_Smash: {
        },
        CDOTA_Modifier_Aghsfort_Reward_MagicResistAura: {
        },
        C_DOTA_Ability_CDOTA_Ability_Special_Bonus_Unique_Razor_3: {
        },
        C_DOTA_Ability_Special_Bonus_Status_Resistance_20: {
        },
        C_DOTA_Ability_Special_Bonus_Armor_4: {
        },
        CDOTA_Modifier_Minotaur_Horn_Immune: {
        },
        CDOTA_Modifier_Royal_Jelly_Regen: {
        },
        CDOTA_Modifier_Item_Crimson_Guard_NoStack: {
        },
        CDOTA_Modifier_Skywrath_Mage_Shard_Bonus_Counter: {
        },
        CDOTA_Modifier_Rubick_Might_And_Magus: {
        },
        CDOTA_Modifier_LoneDruid_GiftBearer: {
        },
        CDOTA_Modifier_Obsidian_Destroyer_AstralImprisonment_Prison: {
        },
        CDOTA_Modifier_Jakiro_DualBreath_Slow: {
        },
        CDOTA_Modifier_Tinker_Innate_Keen_Teleport_XP_On_Death: {
        },
        CDOTA_Modifier_Morphling_Become_Strength: {
        },
        CDOTA_Modifier_Earthshaker_Fissure_Line_Thinker: {
        },
        ArtyProgressBarDef_t: {
        },
        CDOTACrownfallCreditsMapSceneAnimateableDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        ArtyWeaponID_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTAFightingGameActionDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsEnemyDefinition__PickupChance: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        sControlGroupElem: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CSurvivorsEnemyDefinition_Pillar: {
        },
        CDOTAOverworldCharacterConditional: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        Match3GameModeID_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CShmupPlayerDefinition: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CSurvivorsSpawnerTowerDefinition: {
        },
        ArtyCannonDef_t: {
        },
        CandyShopCandyType_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsSpawnerGolem: {
        },
        ArtyGameModeInfo_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CandyShopRewardSlot_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CAnnouncerDescriptor: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        C_VerticalMotionController: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CSurvivorsPowerUpDefinition_AreaAttack: {
        },
        CSurvivorsPowerUpDefinition_AreaAttack_CircleConstant: {
        },
        CDOTAOverworldNode: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CDOTACrownfallCreditsDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsPowerUp_AreaAttack_Circle: {
        },
        CDOTAFishingGameFish: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CShmupTrackDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsLevelDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CBasePortraitData: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        ShmupPathID_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        SettingsSectionIndex_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTAOverworldClickable: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        PlayerSeatAssignment_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CSurvivorsGameModeDefinition__SeparationLayerData: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CSurvivorsSpawnerDestructibles: {
        },
        CSurvivorsGameSnapshot: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        Match3AbilityID_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CMatch3OpponentActionInstanceDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        screenshake_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CSurvivorsPowerUpDefinition_ArcaneBolt: {
        },
        C_DotaTree: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CSurvivorsLootTable__CLootEntry: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CSurvivorsPowerUp_Frostbite: {
        },
        CDOTAFightingGameHeroStyleDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CDOTARoadToTIChallengeDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        ArtyWeaponInfo_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsSpawnerEliteTurretDefinition: {
        },
        CSurvivorsPowerUp_LandMine: {
        },
        SettingsSubSectionIndex_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTAFlappySkywrathInputAction: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CDOTAOverworldTarotCard: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsAttributeDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsEnemyPillar: {
        },
        CSurvivorsPowerUpDefinition_Spirits: {
        },
        CrownfallSurvivorsLightingEnvironment_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CDOTAFlappySkywrathCharacter: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CSurvivorsPowerUp_LagunaBlade: {
        },
        CSurvivorsDifficultyDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsPowerUp_AreaAttack_Line: {
        },
        CSurvivorsSpawnerDestructiblesDefinition: {
        },
        CSurvivorsEnemyDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CDOTAOverworldRoom: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CDOTAFishingGameDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsSpawnerGolemDefinition: {
        },
        CSurvivorsEnemyEventDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        SZooSetAnnotationsConfig_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        ArtyGraphicID_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CSurvivorsUnit_Snotty: {
        },
        ArtyConstants_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CSurvivorsAttributeDefinition__MetaProgressionTier_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsPowerUp_InstantAttack: {
        },
        Match3OpponentActionID_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTAOverworldPath: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsPowerUp_Stampede: {
        },
        CSurvivorsPowerUpDefinition_InstantAttack: {
        },
        CSurvivorsPowerUpDefinition_Frostbite: {
        },
        CDOTAOverworldCharacter: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        SZooSetAnnotations_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsPowerUpDefinition_MagicMissile: {
        },
        CSurvivorsPickupDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CMatch3OpponentActionDefinition: {
        },
        CDOTAOverworldHero: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsAttackParticleInfo: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        PointCameraSettings_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CSurvivorsAttributeValue: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CrownfallCreditsAABB_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CSurvivorsPowerUp: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CMatch3AbilityParamDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CInterpolatedValue: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        ArtySpawnerDef_t: {
        },
        CSurvivorsPowerUpDefinition_MortimerKisses: {
        },
        CSurvivorsPowerUpDefinition_Track: {
        },
        ArtyLevelObjectInstance_t: {
        },
        SurvivorsPickupID_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CShmupBossDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        NewSettingsID_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        ArtyLevelID_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CSurvivorsEnemyDefinition_ImperiaBoss: {
        },
        CShmupBulletInfo: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // �
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        SZooSetAnnotation_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        SurvivorsPhysicsBodyID_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTACrownfallCreditsBlockDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsPowerUp_AreaAttack: {
        },
        SurvivorsHeroID_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTAOverworldCharacterBase: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsPowerUpDefinition_EchoStrike: {
        },
        CDOTAFlappySkywrathDifficulty: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        TimedEvent: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CFlashlightEffect: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CSurvivorsEnemyDefinition__Attack: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CShmupGameDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsHeroDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsEnemyDefinition_Snotty: {
        },
        CSurvivorsPowerUp_MortimerKisses: {
        },
        CDOTAFlappySkywrathDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsEnemyGolem: {
        },
        CShmupPathDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CShmupEventTime: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CMatch3GameModeDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CDOTAOverworldDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        SNewSettingsDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsPowerUp_Spirits: {
        },
        CShmupEnemyDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsEnemy: {
        },
        SurvivorsUpgradeID_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        ArtyUnitDef_t: {
        },
        CSurvivorsLootTable__CLootEntryCollection: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        ArtyGameObjectDef_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CShmupEventEnemySpawn: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        SurvivorsLevelID_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CSurvivorsPowerUp_EchoStrike: {
        },
        CSurvivorsPowerUp_Track: {
        },
        RoadToTIQuestDefinition_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        Match3OpponentID_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CSurvivorsEnemyAbsorber: {
        },
        CDOTAMinesweeperStageDefinition: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        C_CommandContext: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CSurvivorsUnit: {
        },
        CPortraitData: {
        },
        CSurvivorsPowerUpDefinition_Stampede: {
        },
        ArtyGameModeLevelInfo_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        OverworldSplineInfo_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CSurvivorsEnemyDefinition_Golem: {
        },
        CSurvivorsEnemyResurrector: {
        },
        CSurvivorsPowerUp_ProjectileAttack: {
        },
        ShmupEnemyID_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CSurvivorsEntity: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CandyShopDefinitionGC_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CDOTACandyShopDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        SurvivorsParticleID_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        ArtyGameObjectInstance_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        SettingsSearchDataIndex_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        SurvivorsDifficultyID_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CSurvivorsPowerUpDefinition_Snotty: {
        },
        CSurvivorsLootTable: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CMatch3HeroDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsEnemySnapshot: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CSurvivorsPowerUp_KnifeThrow: {
        },
        CDOTAFightingGameCancelOptionDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CDOTAOverworldTheme: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsPowerUp_CounterHelix: {
        },
        CSurvivorsPowerUp_AreaAttack_CircleConstant: {
        },
        CSurvivorsEnemyDefinition_Resurrector: {
        },
        CShmupBossPhase: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsPowerUpSnapshot: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CShmupEventDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsPowerUpDefinition_KnifeThrow: {
        },
        CSurvivorsAttributeDefinition__MetaProgressionTierCost_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        ShmupEventID_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTAMinesweeperPlayerDefinition: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        ArtyLevelWeaponInstance_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsPowerUp_MagicMissile: {
        },
        SurvivorsGameModeID_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CSurvivorsPlayerSnapshot: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CandyShopRewardOption_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsPowerUp_ArcaneBolt: {
        },
        CShmupPathEvent: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        SurvivorsPowerUpID_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTAOverworldEncounterReward: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CDOTAOverworldRoomGroup: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsSpawner: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CSurvivorsPowerUpDamageTickInfo: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CDOTAMinesweeperGameDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CrownfallSurvivorsLightingOverride_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CClientAlphaProperty: {
        },
        CSurvivorsPowerUpDefinition_CounterHelix: {
        },
        CDOTAOverworldHeroReward: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        ArtyLevelInfo_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsPowerUpDefinition_AreaAttack_Line: {
        },
        CSurvivorsPowerUpDefinition_LagunaBlade: {
        },
        C_HorizontalMotionController: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        screenfade_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        ArtyEnemyOrder_t: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CMatch3Level: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        ArtyGraphicInfo_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CMatch3AbilityBaseDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CGlobalLightBase: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        SurvivorsEnemyID_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CMatch3AbilityDefinition: {
        },
        CDOTACrownfallCreditsMapSceneDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsSpawnerEliteTurret: {
        },
        CandyShopRewardOptionGC_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsPowerUp_Snotty: {
        },
        CSurvivorsSpawnerTower: {
        },
        CDOTACrownfallCreditsCharacterDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        ArtyGameObjectID_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CSurvivorsPowerUpDefinition_LandMine: {
        },
        CMatch3OpponentHeroItemDefinition: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        IClientAlphaProperty: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        ArtyEnemyDef_t: {
        },
        CDOTAEventActionTrigger: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsUpgradeDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsImperiaBoss: {
        },
        CDOTAEventActionGrantAndClaimPair: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        SurvivorsUnitID_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CSurvivorsPowerUpDefinition_ProjectileAttack: {
        },
        CDOTAFightingGameHeroDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CDOTAEventActionGrantAndClaimPairTrigger: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CLightInfoBase: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CSurvivorsPowerUpDefinition_Swashbuckle: {
        },
        CDOTAOverworldEncounter: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsSpawnerDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        DOTAOverworldCharacterOverrideConditional_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        ArtyPlayerDef_t: {
        },
        CDOTAOverworldPathColorRule: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CSurvivorsPowerUp_Swashbuckle: {
        },
        CDOTAOverworldToken: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        NeutralCampStackPullAlarm_t: {
            : 0x10110FF, // 
            : 0x0, // 
            : 0x45, // 
            : 0x0, // 
            : 0x0, // 
            : 0x400, // 
            : 0x0, // 
            : 0x1032000, // 
            : 0x0, // 
            : 0x1015000, // 
            : 0x0, // 
            : 0x61007400, // 
            : 0x6F006400, // 
            : 0x74006100, // 
            : 0x2E003000, // 
            : 0x3A004300, // 
            : 0x63006900, // 
            : 0x44002000, // 
            : 0x6D007200, // 
            : 0x5C003000, // 
            : 0x66006E00, // 
            : 0x61004400, // 
            : 0x6E006900, // 
            : 0x6C005000, // 
            : 0x32003000, // 
            : 0x6E006500, // 
            : 0x7FF98B, // 
            : 0x0, // 
            : 0x1, // 
            : 0x69006400, // 
            : 0x1015000, // 
            : 0x44005C00, // 
            : 0x6E006900, // 
            : 0x0, // 
            : 0x0, // 
            : 0x7700, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D004D00, // 
            : 0x600, // 
            : 0x0, // 
            : 0x0, // 
            : 0x100F003, // 
            : 0x1100000, // 
        },
        CSurvivorsEnemyDefinition_Absorber: {
        },
        CSurvivorsPowerUpDefinition_AreaAttack_Circle: {
        },
        CSurvivorsPickupSnapshot: {
            : 0x1, // 
            : 0x0, // 
            : 0x1, // 
            : 0x0, // 
            : 0xFF00, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x50005C, // 
            : 0x6F0072, // 
            : 0x660065, // 
            : 0x34005C, // 
            : 0x70004D, // 
            : 0x670069, // 
            : 0x610074, // 
            : 0x6F0064, // 
            : 0x740061, // 
            : 0x2E0030, // 
            : 0x0, // 
            : 0x0, // 
            : 0xE02FB640, // 
            : 0x700073, // 
            : 0x0, // 
            : 0x730069, // 
            : 0x720065, // 
            : 0xEE, // 
            : 0x80, // 
            : 0x20004D, // 
            : 0x520002, // 
            : 0x0, // 
            : 0x15, // 
            : 0x0, // 
            : 0x0, // 
            : 0x0, // 
        },
        CDOTAMinesweeperStageProgressionChoice: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsPowerUpDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CSurvivorsGameModeDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CMatch3OpponentDefinition: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        PortraitWorldLightConfig_t: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
        CShmupBossBodyPart: {
            : 0x10120, // 
            : 0x10, // 
            : 0x0, // 
            `: 0x0, // 
            : 0xEEFFEEFF, // 
            : 0x1000, // 
            : 0x0, // 
            : 0x10FD0, // �
            : 0x10110, // 
            : 0x10750, // P
            : 0x1FE000, // 
            : 0x1, // 
            P: 0x10330, // 
            : 0x10150, // P
            : 0xE02FA810, // 
            : 0x630069, // 
            : 0x440020, // 
            : 0x6D0072, // 
            : 0x5C0030, // 
            : 0x6F0072, // 
            : 0x6F0073, // 
            : 0x6E0065, // 
            : 0x31002E, // 
            : 0x61004F, // 
            : 0x6C0000, // 
            : 0x4D005C, // 
            : 0x730077, // 
            : 0x6F0066, // 
            : 0x2D0036, // 
            : 0x0, // 
            : 0x0, // 
            : 0x4B, // 
            : 0x0, // 
            : 0x5D00005D, // 
            P: 0x10150, // P
            : 0x6C0070, // 
            : 0x5C, // 
            : 0x0, // 
            : 0x0, // 
            : 0xEE, // 
            : 0x3, // 
            : 0x790079, // 
            : 0x70, // 
            : 0x0, // 
            : 0x99A46D76, // 
            : 0x1D00001D, // 
            : 0x100F0, // �
            �: 0xF000, // 
        },
    },
};
