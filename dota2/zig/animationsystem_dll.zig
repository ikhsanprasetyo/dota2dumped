// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-03-30 13:31:39.632920200 +07:00

pub const source2_dumper = struct {
    pub const schemas = struct {
        // Module: animationsystem.dll
        // Class count: 663
        // Enum count: 141
        pub const animationsystem_dll = struct {
            // Alignment: 4
            // Member count: 2
            pub const PulseBestOutflowRules_t = enum(u32) {
                SORT_BY_NUMBER_OF_VALID_CRITERIA = 0x0,
                SORT_BY_OUTFLOW_INDEX = 0x1
            };
            // Alignment: 4
            // Member count: 4
            pub const PulseCursorCancelPriority_t = enum(u32) {
                None = 0x0,
                CancelOnSucceeded = 0x1,
                SoftCancel = 0x2,
                HardCancel = 0x3
            };
            // Alignment: 4
            // Member count: 2
            pub const PulseMethodCallMode_t = enum(u32) {
                SYNC_WAIT_FOR_COMPLETION = 0x0,
                ASYNC_FIRE_AND_FORGET = 0x1
            };
            // Alignment: 4
            // Member count: 6
            pub const ParticleSetMethod_t = enum(u32) {
                PARTICLE_SET_REPLACE_VALUE = 0x0,
                PARTICLE_SET_SCALE_INITIAL_VALUE = 0x1,
                PARTICLE_SET_ADD_TO_INITIAL_VALUE = 0x2,
                PARTICLE_SET_RAMP_CURRENT_VALUE = 0x3,
                PARTICLE_SET_SCALE_CURRENT_VALUE = 0x4,
                PARTICLE_SET_ADD_TO_CURRENT_VALUE = 0x5
            };
            // Alignment: 4
            // Member count: 17
            pub const SeqCmd_t = enum(u32) {
                SeqCmd_Nop = 0x0,
                SeqCmd_LinearDelta = 0x1,
                SeqCmd_FetchFrameRange = 0x2,
                SeqCmd_Slerp = 0x3,
                SeqCmd_Add = 0x4,
                SeqCmd_Subtract = 0x5,
                SeqCmd_Scale = 0x6,
                SeqCmd_Copy = 0x7,
                SeqCmd_Blend = 0x8,
                SeqCmd_Worldspace = 0x9,
                SeqCmd_Sequence = 0xA,
                SeqCmd_FetchCycle = 0xB,
                SeqCmd_FetchFrame = 0xC,
                SeqCmd_IKLockInPlace = 0xD,
                SeqCmd_IKRestoreAll = 0xE,
                SeqCmd_ReverseSequence = 0xF,
                SeqCmd_Transform = 0x10
            };
            // Alignment: 4
            // Member count: 3
            pub const CNmEventRelevance_t = enum(u32) {
                ClientOnly = 0x0,
                ServerOnly = 0x1,
                ClientAndServer = 0x2
            };
            // Alignment: 4
            // Member count: 4
            pub const BoneTransformSpace_t = enum(u32) {
                BoneTransformSpace_Invalid = 0xFFFFFFFF,
                BoneTransformSpace_Parent = 0x0,
                BoneTransformSpace_Model = 0x1,
                BoneTransformSpace_World = 0x2
            };
            // Alignment: 4
            // Member count: 5
            pub const CAnimationGraphVisualizerPrimitiveType = enum(u32) {
                ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Text = 0x0,
                ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Sphere = 0x1,
                ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Line = 0x2,
                ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Pie = 0x3,
                ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Axis = 0x4
            };
            // Alignment: 1
            // Member count: 3
            pub const NmTransitionRule_t = enum(u8) {
                AllowTransition = 0x0,
                ConditionallyAllowTransition = 0x1,
                BlockTransition = 0x2
            };
            // Alignment: 4
            // Member count: 3
            pub const BinaryNodeTiming = enum(u32) {
                UseChild1 = 0x0,
                UseChild2 = 0x1,
                SyncChildren = 0x2
            };
            // Alignment: 1
            // Member count: 3
            pub const NmFollowBoneMode_t = enum(u8) {
                RotationAndTranslation = 0x0,
                RotationOnly = 0x1,
                TranslationOnly = 0x2
            };
            // Alignment: 4
            // Member count: 7
            pub const SolveIKChainAnimNodeDebugSetting = enum(u32) {
                SOLVEIKCHAINANIMNODEDEBUGSETTING_None = 0x0,
                SOLVEIKCHAINANIMNODEDEBUGSETTING_X_Axis_Circle = 0x1,
                SOLVEIKCHAINANIMNODEDEBUGSETTING_Y_Axis_Circle = 0x2,
                SOLVEIKCHAINANIMNODEDEBUGSETTING_Z_Axis_Circle = 0x3,
                SOLVEIKCHAINANIMNODEDEBUGSETTING_Forward = 0x4,
                SOLVEIKCHAINANIMNODEDEBUGSETTING_Up = 0x5,
                SOLVEIKCHAINANIMNODEDEBUGSETTING_Left = 0x6
            };
            // Alignment: 4
            // Member count: 2
            pub const CNmParticleEvent__Type_t = enum(u32) {
                Create = 0x0,
                Create_CFG = 0x1
            };
            // Alignment: 4
            // Member count: 5
            pub const ParticleFloatBiasType_t = enum(u32) {
                PF_BIAS_TYPE_INVALID = 0xFFFFFFFF,
                PF_BIAS_TYPE_STANDARD = 0x0,
                PF_BIAS_TYPE_GAIN = 0x1,
                PF_BIAS_TYPE_EXPONENTIAL = 0x2,
                PF_BIAS_TYPE_COUNT = 0x3
            };
            // Alignment: 1
            // Member count: 4
            pub const CNmTargetWarpNode__TargetUpdateRule_t = enum(u8) {
                None = 0x0,
                Recalculate = 0x1,
                Offset = 0x2,
                RecalculateOrOffset = 0x3
            };
            // Alignment: 1
            // Member count: 6
            pub const SharedMovementGait_t = enum(u8) {
                eInvalid = 0xFF,
                eSlow = 0x0,
                eMedium = 0x1,
                eFast = 0x2,
                eVeryFast = 0x3,
                eCount = 0x4
            };
            // Alignment: 4
            // Member count: 3
            pub const OrientationWarpRootMotionSource_t = enum(u32) {
                eAnimationOrProcedural = 0x0,
                eAnimationOnly = 0x1,
                eProceduralOnly = 0x2
            };
            // Alignment: 4
            // Member count: 3
            pub const IKTargetCoordinateSystem = enum(u32) {
                IKTARGETCOORDINATESYSTEM_WorldSpace = 0x0,
                IKTARGETCOORDINATESYSTEM_ModelSpace = 0x1,
                IKTARGETCOORDINATESYSTEM_COUNT = 0x2
            };
            // Alignment: 4
            // Member count: 33
            pub const ParticleFloatType_t = enum(u32) {
                PF_TYPE_INVALID = 0xFFFFFFFF,
                PF_TYPE_LITERAL = 0x0,
                PF_TYPE_NAMED_VALUE = 0x1,
                PF_TYPE_RANDOM_UNIFORM = 0x2,
                PF_TYPE_RANDOM_BIASED = 0x3,
                PF_TYPE_COLLECTION_AGE = 0x4,
                PF_TYPE_ENDCAP_AGE = 0x5,
                PF_TYPE_CONTROL_POINT_COMPONENT = 0x6,
                PF_TYPE_CONTROL_POINT_CHANGE_AGE = 0x7,
                PF_TYPE_CONTROL_POINT_SPEED = 0x8,
                PF_TYPE_PARTICLE_DETAIL_LEVEL = 0x9,
                PF_TYPE_CONCURRENT_DEF_COUNT = 0xA,
                PF_TYPE_CLOSEST_CAMERA_DISTANCE = 0xB,
                PF_TYPE_SNAPSHOT_COUNT = 0xC,
                PF_TYPE_SNAPSHOT_CHANGED = 0xD,
                PF_TYPE_CONTROL_POINT_IS_SET = 0xE,
                PF_TYPE_RENDERER_CAMERA_DISTANCE = 0xF,
                PF_TYPE_RENDERER_CAMERA_DOT_PRODUCT = 0x10,
                PF_TYPE_PARTICLE_NOISE = 0x11,
                PF_TYPE_PARTICLE_AGE = 0x12,
                PF_TYPE_PARTICLE_AGE_NORMALIZED = 0x13,
                PF_TYPE_PARTICLE_FLOAT = 0x14,
                PF_TYPE_PARTICLE_INITIAL_FLOAT = 0x15,
                PF_TYPE_PARTICLE_VECTOR_COMPONENT = 0x16,
                PF_TYPE_PARTICLE_INITIAL_VECTOR_COMPONENT = 0x17,
                PF_TYPE_PARTICLE_SPEED = 0x18,
                PF_TYPE_PARTICLE_NUMBER = 0x19,
                PF_TYPE_PARTICLE_NUMBER_NORMALIZED = 0x1A,
                PF_TYPE_PARTICLE_ROPE_SEGMENT = 0x1B,
                PF_TYPE_PARTICLE_ROPE_SEGMENT_NORMALIZED = 0x1C,
                PF_TYPE_PARTICLE_SCREENSPACE_CAMERA_DISTANCE = 0x1D,
                PF_TYPE_PARTICLE_SCREENSPACE_CAMERA_DOT_PRODUCT = 0x1E,
                PF_TYPE_COUNT = 0x1F
            };
            // Alignment: 1
            // Member count: 4
            pub const CNmFloatAngleMathNode__Operation_t = enum(u8) {
                ClampTo180 = 0x0,
                ClampTo360 = 0x1,
                FlipHemisphere = 0x2,
                FlipHemisphereNegate = 0x3
            };
            // Alignment: 4
            // Member count: 3
            pub const VPhysXAggregateData_t__VPhysXFlagEnum_t = enum(u32) {
                FLAG_IS_POLYSOUP_GEOMETRY = 0x1,
                FLAG_LEVEL_COLLISION = 0x10,
                FLAG_IGNORE_SCALE_OBSOLETE_DO_NOT_USE = 0x20
            };
            // Alignment: 1
            // Member count: 5
            pub const CNmRootMotionOverrideNode__OverrideFlags_t = enum(u8) {
                AllowMoveX = 0x0,
                AllowMoveY = 0x1,
                AllowMoveZ = 0x2,
                AllowFacingPitch = 0x3,
                ListenForEvents = 0x4
            };
            // Alignment: 1
            // Member count: 23
            pub const NmEasingOperation_t = enum(u8) {
                Linear = 0x0,
                InQuad = 0x1,
                OutQuad = 0x2,
                InOutQuad = 0x3,
                InCubic = 0x4,
                OutCubic = 0x5,
                InOutCubic = 0x6,
                InQuart = 0x7,
                OutQuart = 0x8,
                InOutQuart = 0x9,
                InQuint = 0xA,
                OutQuint = 0xB,
                InOutQuint = 0xC,
                InSine = 0xD,
                OutSine = 0xE,
                InOutSine = 0xF,
                InExpo = 0x10,
                OutExpo = 0x11,
                InOutExpo = 0x12,
                InCirc = 0x13,
                OutCirc = 0x14,
                InOutCirc = 0x15,
                None = 0x16
            };
            // Alignment: 4
            // Member count: 5
            pub const EIKEndEffectorRotationFixUpMode = enum(u32) {
                None = 0x0,
                MatchTargetOrientation = 0x1,
                LookAtTargetForward = 0x2,
                MaintainParentOrientation = 0x3,
                Count = 0x4
            };
            // Alignment: 4
            // Member count: 2
            pub const MatterialAttributeTagType_t = enum(u32) {
                MATERIAL_ATTRIBUTE_TAG_VALUE = 0x0,
                MATERIAL_ATTRIBUTE_TAG_COLOR = 0x1
            };
            // Alignment: 4
            // Member count: 6
            pub const PFNoiseTurbulence_t = enum(u32) {
                PF_NOISE_TURB_NONE = 0x0,
                PF_NOISE_TURB_HIGHLIGHT = 0x1,
                PF_NOISE_TURB_FEEDBACK = 0x2,
                PF_NOISE_TURB_LOOPY = 0x3,
                PF_NOISE_TURB_CONTRAST = 0x4,
                PF_NOISE_TURB_ALTERNATE = 0x5
            };
            // Alignment: 1
            // Member count: 4
            pub const NmTargetWarpAlgorithm_t = enum(u8) {
                Lerp = 0x0,
                Hermite = 0x1,
                HermiteFeaturePreserving = 0x2,
                Bezier = 0x3
            };
            // Alignment: 4
            // Member count: 5
            pub const ParticleColorBlendMode_t = enum(u32) {
                PARTICLEBLEND_DEFAULT = 0x0,
                PARTICLEBLEND_OVERLAY = 0x1,
                PARTICLEBLEND_DARKEN = 0x2,
                PARTICLEBLEND_LIGHTEN = 0x3,
                PARTICLEBLEND_MULTIPLY = 0x4
            };
            // Alignment: 4
            // Member count: 13
            pub const ParticleColorBlendType_t = enum(u32) {
                PARTICLE_COLOR_BLEND_MULTIPLY = 0x0,
                PARTICLE_COLOR_BLEND_MULTIPLY2X = 0x1,
                PARTICLE_COLOR_BLEND_DIVIDE = 0x2,
                PARTICLE_COLOR_BLEND_ADD = 0x3,
                PARTICLE_COLOR_BLEND_SUBTRACT = 0x4,
                PARTICLE_COLOR_BLEND_MOD2X = 0x5,
                PARTICLE_COLOR_BLEND_SCREEN = 0x6,
                PARTICLE_COLOR_BLEND_MAX = 0x7,
                PARTICLE_COLOR_BLEND_MIN = 0x8,
                PARTICLE_COLOR_BLEND_REPLACE = 0x9,
                PARTICLE_COLOR_BLEND_AVERAGE = 0xA,
                PARTICLE_COLOR_BLEND_NEGATE = 0xB,
                PARTICLE_COLOR_BLEND_LUMINANCE = 0xC
            };
            // Alignment: 1
            // Member count: 4
            pub const NmTransitionRuleCondition_t = enum(u8) {
                AnyAllowed = 0x0,
                FullyAllowed = 0x1,
                ConditionallyAllowed = 0x2,
                Blocked = 0x3
            };
            // Alignment: 1
            // Member count: 7
            pub const ModelMeshBufferUsage_t = enum(u8) {
                MESH_BUFFER_USAGE_NONE = 0x0,
                MESH_BUFFER_USAGE_VB = 0x1,
                MESH_BUFFER_USAGE_IB = 0x2,
                MESH_BUFFER_USAGE_ADJACENCY = 0x4,
                MESH_BUFFER_USAGE_MESHLET_TRIS = 0x8,
                MESH_BUFFER_USAGE_RT_PROXY = 0x10,
                MESH_BUFFER_USAGE_VERTEX_ALBEDO = 0x20
            };
            // Alignment: 4
            // Member count: 2
            pub const NmGraphDebugMode_t = enum(u32) {
                Off = 0x0,
                On = 0x1
            };
            // Alignment: 4
            // Member count: 2
            pub const TargetWarpTimingMethod = enum(u32) {
                ReachDestinationOnRootMotionEnd = 0x0,
                ReachDestinationOnWarpTagEnd = 0x1
            };
            // Alignment: 4
            // Member count: 6
            pub const ScriptedMoveTo_t = enum(u32) {
                eWait = 0x0,
                eMoveWithGait = 0x3,
                eTeleport = 0x4,
                eWaitFacing = 0x5,
                eObsoleteBackCompat1 = 0x1,
                eObsoleteBackCompat2 = 0x2
            };
            // Alignment: 4
            // Member count: 2
            pub const EDemoBoneSelectionMode = enum(u32) {
                CaptureAllBones = 0x0,
                CaptureSelectedBones = 0x1
            };
            // Alignment: 4
            // Member count: 2
            pub const StepPhase = enum(u32) {
                StepPhase_OnGround = 0x0,
                StepPhase_InAir = 0x1
            };
            // Alignment: 4
            // Member count: 26
            pub const FlexOpCode_t = enum(u32) {
                FLEX_OP_CONST = 0x1,
                FLEX_OP_FETCH1 = 0x2,
                FLEX_OP_FETCH2 = 0x3,
                FLEX_OP_ADD = 0x4,
                FLEX_OP_SUB = 0x5,
                FLEX_OP_MUL = 0x6,
                FLEX_OP_DIV = 0x7,
                FLEX_OP_NEG = 0x8,
                FLEX_OP_EXP = 0x9,
                FLEX_OP_OPEN = 0xA,
                FLEX_OP_CLOSE = 0xB,
                FLEX_OP_COMMA = 0xC,
                FLEX_OP_MAX = 0xD,
                FLEX_OP_MIN = 0xE,
                FLEX_OP_2WAY_0 = 0xF,
                FLEX_OP_2WAY_1 = 0x10,
                FLEX_OP_NWAY = 0x11,
                FLEX_OP_COMBO = 0x12,
                FLEX_OP_DOMINATE = 0x13,
                FLEX_OP_DME_LOWER_EYELID = 0x14,
                FLEX_OP_DME_UPPER_EYELID = 0x15,
                FLEX_OP_SQRT = 0x16,
                FLEX_OP_REMAPVALCLAMPED = 0x17,
                FLEX_OP_SIN = 0x18,
                FLEX_OP_COS = 0x19,
                FLEX_OP_ABS = 0x1A
            };
            // Alignment: 4
            // Member count: 2
            pub const NmCachedValueMode_t = enum(u32) {
                OnEntry = 0x0,
                OnExit = 0x1
            };
            // Alignment: 4
            // Member count: 2
            pub const AnimNodeNetworkMode = enum(u32) {
                ServerAuthoritative = 0x0,
                ClientSimulate = 0x1
            };
            // Alignment: 4
            // Member count: 6
            pub const VPhysXBodyPart_t__VPhysXFlagEnum_t = enum(u32) {
                FLAG_STATIC = 0x1,
                FLAG_KINEMATIC = 0x2,
                FLAG_JOINT = 0x4,
                FLAG_MASS = 0x8,
                FLAG_ALWAYS_DYNAMIC_ON_CLIENT = 0x10,
                FLAG_DISABLE_CCD = 0x20
            };
            // Alignment: 1
            // Member count: 9
            pub const AnimParamType_t = enum(u8) {
                ANIMPARAM_UNKNOWN = 0x0,
                ANIMPARAM_BOOL = 0x1,
                ANIMPARAM_ENUM = 0x2,
                ANIMPARAM_INT = 0x3,
                ANIMPARAM_FLOAT = 0x4,
                ANIMPARAM_VECTOR = 0x5,
                ANIMPARAM_QUATERNION = 0x6,
                ANIMPARAM_GLOBALSYMBOL = 0x7,
                ANIMPARAM_COUNT = 0x8
            };
            // Alignment: 1
            // Member count: 9
            pub const NmEasingFunction_t = enum(u8) {
                Linear = 0x0,
                Quad = 0x1,
                Cubic = 0x2,
                Quart = 0x3,
                Quint = 0x4,
                Sine = 0x5,
                Expo = 0x6,
                Circ = 0x7,
                Back = 0x8
            };
            // Alignment: 4
            // Member count: 5
            pub const ParticleModelType_t = enum(u32) {
                PM_TYPE_INVALID = 0x0,
                PM_TYPE_NAMED_VALUE_MODEL = 0x1,
                PM_TYPE_NAMED_VALUE_EHANDLE = 0x2,
                PM_TYPE_CONTROL_POINT = 0x3,
                PM_TYPE_COUNT = 0x4
            };
            // Alignment: 4
            // Member count: 3
            pub const IKTargetSource = enum(u32) {
                IKTARGETSOURCE_Bone = 0x0,
                IKTARGETSOURCE_AnimgraphParameter = 0x1,
                IKTARGETSOURCE_COUNT = 0x2
            };
            // Alignment: 4
            // Member count: 15
            pub const PermModelInfo_t__FlagEnum = enum(u32) {
                FLAG_TRANSLUCENT = 0x1,
                FLAG_TRANSLUCENT_TWO_PASS = 0x2,
                FLAG_MODEL_IS_RUNTIME_COMBINED = 0x4,
                FLAG_SOURCE1_IMPORT = 0x8,
                FLAG_MODEL_PART_CHILD = 0x10,
                FLAG_NAV_GEN_NONE = 0x20,
                FLAG_NAV_GEN_HULL = 0x40,
                FLAG_NO_FORCED_FADE = 0x800,
                FLAG_HAS_SKINNED_MESHES = 0x400,
                FLAG_DO_NOT_CAST_SHADOWS = 0x20000,
                FLAG_FORCE_PHONEME_CROSSFADE = 0x1000,
                FLAG_NO_ANIM_EVENTS = 0x100000,
                FLAG_ANIMATION_DRIVEN_FLEXES = 0x200000,
                FLAG_IMPLICIT_BIND_POSE_SEQUENCE = 0x400000,
                FLAG_MODEL_DOC = 0x800000
            };
            // Alignment: 1
            // Member count: 12
            pub const CNmFloatMathNode__Operator_t = enum(u8) {
                Add = 0x0,
                Sub = 0x1,
                Mul = 0x2,
                Div = 0x3,
                Mod = 0x4,
                Abs = 0x5,
                Negate = 0x6,
                Floor = 0x7,
                Ceiling = 0x8,
                IntegerPart = 0x9,
                FractionalPart = 0xA,
                InverseFractionalPart = 0xB
            };
            // Alignment: 1
            // Member count: 2
            pub const CNmSyncEventIndexConditionNode__TriggerMode_t = enum(u8) {
                ExactlyAtEventIndex = 0x0,
                GreaterThanEqualToEventIndex = 0x1
            };
            // Alignment: 4
            // Member count: 5
            pub const ParticleFloatRoundType_t = enum(u32) {
                PF_ROUND_TYPE_INVALID = 0xFFFFFFFF,
                PF_ROUND_TYPE_NEAREST = 0x0,
                PF_ROUND_TYPE_FLOOR = 0x1,
                PF_ROUND_TYPE_CEIL = 0x2,
                PF_ROUND_TYPE_COUNT = 0x3
            };
            // Alignment: 4
            // Member count: 4
            pub const PFNoiseType_t = enum(u32) {
                PF_NOISE_TYPE_PERLIN = 0x0,
                PF_NOISE_TYPE_SIMPLEX = 0x1,
                PF_NOISE_TYPE_WORLEY = 0x2,
                PF_NOISE_TYPE_CURL = 0x3
            };
            // Alignment: 4
            // Member count: 3
            pub const ParticleDirectionNoiseType_t = enum(u32) {
                PARTICLE_DIR_NOISE_PERLIN = 0x0,
                PARTICLE_DIR_NOISE_CURL = 0x1,
                PARTICLE_DIR_NOISE_WORLEY_BASIC = 0x2
            };
            // Alignment: 4
            // Member count: 3
            pub const AnimParamNetworkSetting = enum(u32) {
                Auto = 0x0,
                AlwaysNetwork = 0x1,
                NeverNetwork = 0x2
            };
            // Alignment: 4
            // Member count: 4
            pub const MorphFlexControllerRemapType_t = enum(u32) {
                MORPH_FLEXCONTROLLER_REMAP_PASSTHRU = 0x0,
                MORPH_FLEXCONTROLLER_REMAP_2WAY = 0x1,
                MORPH_FLEXCONTROLLER_REMAP_NWAY = 0x2,
                MORPH_FLEXCONTROLLER_REMAP_EYELID = 0x3
            };
            // Alignment: 4
            // Member count: 8
            pub const MeshDrawPrimitiveFlags_t = enum(u32) {
                MESH_DRAW_FLAGS_NONE = 0x0,
                MESH_DRAW_FLAGS_USE_SHADOW_FAST_PATH = 0x1,
                MESH_DRAW_FLAGS_USE_COMPRESSED_NORMAL_TANGENT = 0x2,
                MESH_DRAW_INPUT_LAYOUT_IS_NOT_MATCHED_TO_MATERIAL = 0x8,
                MESH_DRAW_FLAGS_USE_COMPRESSED_PER_VERTEX_LIGHTING = 0x10,
                MESH_DRAW_FLAGS_USE_UNCOMPRESSED_PER_VERTEX_LIGHTING = 0x20,
                MESH_DRAW_FLAGS_CAN_BATCH_WITH_DYNAMIC_SHADER_CONSTANTS = 0x40,
                MESH_DRAW_FLAGS_DRAW_LAST = 0x80
            };
            // Alignment: 4
            // Member count: 2
            pub const TargetWarpAngleMode_t = enum(u32) {
                eFacingHeading = 0x0,
                eMoveHeading = 0x1
            };
            // Alignment: 1
            // Member count: 2
            pub const NmIKBlendMode_t = enum(u8) {
                Effector = 0x0,
                Pose = 0x1
            };
            // Alignment: 4
            // Member count: 4
            pub const ModelBoneFlexComponent_t = enum(u32) {
                MODEL_BONE_FLEX_INVALID = 0xFFFFFFFF,
                MODEL_BONE_FLEX_TX = 0x0,
                MODEL_BONE_FLEX_TY = 0x1,
                MODEL_BONE_FLEX_TZ = 0x2
            };
            // Alignment: 1
            // Member count: 2
            pub const CNmStateNode__TimedEvent_t__Comparison_t = enum(u8) {
                LessThanEqual = 0x0,
                GreaterThanEqual = 0x1
            };
            // Alignment: 1
            // Member count: 3
            pub const PoseType_t = enum(u8) {
                POSETYPE_STATIC = 0x0,
                POSETYPE_DYNAMIC = 0x1,
                POSETYPE_INVALID = 0xFF
            };
            // Alignment: 1
            // Member count: 2
            pub const CNmRootMotionData__SamplingMode_t = enum(u8) {
                Delta = 0x0,
                WorldSpace = 0x1
            };
            // Alignment: 1
            // Member count: 9
            pub const NmEventConditionRules_t = enum(u8) {
                LimitSearchToSourceState = 0x0,
                IgnoreInactiveEvents = 0x1,
                PreferHighestWeight = 0x2,
                PreferHighestProgress = 0x3,
                OperatorOr = 0x4,
                OperatorAnd = 0x5,
                SearchOnlyGraphEvents = 0x6,
                SearchOnlyAnimEvents = 0x7,
                SearchBothGraphAndAnimEvents = 0x8
            };
            // Alignment: 4
            // Member count: 3
            pub const AnimationType_t = enum(u32) {
                ANIMATION_TYPE_FIXED_RATE = 0x0,
                ANIMATION_TYPE_FIT_LIFETIME = 0x1,
                ANIMATION_TYPE_MANUAL_FRAMES = 0x2
            };
            // Alignment: 4
            // Member count: 37
            pub const AnimValueSource = enum(u32) {
                MoveHeading = 0x0,
                MoveSpeed = 0x1,
                ForwardSpeed = 0x2,
                StrafeSpeed = 0x3,
                FacingHeading = 0x4,
                LookHeading = 0x5,
                LookHeadingNormalized = 0x6,
                LookPitch = 0x7,
                LookDistance = 0x8,
                Parameter = 0x9,
                WayPointHeading = 0xA,
                WayPointDistance = 0xB,
                BoundaryRadius = 0xC,
                TargetMoveHeading = 0xD,
                TargetMoveSpeed = 0xE,
                AccelerationHeading = 0xF,
                AccelerationSpeed = 0x10,
                SlopeHeading = 0x11,
                SlopeAngle = 0x12,
                SlopePitch = 0x13,
                SlopeYaw = 0x14,
                GoalDistance = 0x15,
                AccelerationLeftRight = 0x16,
                AccelerationFrontBack = 0x17,
                RootMotionSpeed = 0x18,
                RootMotionTurnSpeed = 0x19,
                MoveHeadingRelativeToLookHeading = 0x1A,
                MaxMoveSpeed = 0x1B,
                FingerCurl_Thumb = 0x1C,
                FingerCurl_Index = 0x1D,
                FingerCurl_Middle = 0x1E,
                FingerCurl_Ring = 0x1F,
                FingerCurl_Pinky = 0x20,
                FingerSplay_Thumb_Index = 0x21,
                FingerSplay_Index_Middle = 0x22,
                FingerSplay_Middle_Ring = 0x23,
                FingerSplay_Ring_Pinky = 0x24
            };
            // Alignment: 1
            // Member count: 4
            pub const CNmTimeConditionNode__Operator_t = enum(u8) {
                LessThan = 0x0,
                LessThanEqual = 0x1,
                GreaterThan = 0x2,
                GreaterThanEqual = 0x3
            };
            // Alignment: 4
            // Member count: 3
            pub const LinearRootMotionBlendMode_t = enum(u32) {
                LERP = 0x0,
                NLERP = 0x1,
                SLERP = 0x2
            };
            // Alignment: 4
            // Member count: 1
            pub const RagdollPoseControl = enum(u32) {
                Absolute = 0x0
            };
            // Alignment: 4
            // Member count: 6
            pub const IKSolverType = enum(u32) {
                IKSOLVER_Perlin = 0x0,
                IKSOLVER_TwoBone = 0x1,
                IKSOLVER_Fabrik = 0x2,
                IKSOLVER_DogLeg3Bone = 0x3,
                IKSOLVER_CCD = 0x4,
                IKSOLVER_COUNT = 0x5
            };
            // Alignment: 4
            // Member count: 2
            pub const TargetWarpCorrectionMethod = enum(u32) {
                ScaleMotion = 0x0,
                AddCorrectionDelta = 0x1
            };
            // Alignment: 4
            // Member count: 2
            pub const TargetSelectorAngleMode_t = enum(u32) {
                eFacingHeading = 0x0,
                eMoveHeading = 0x1
            };
            // Alignment: 4
            // Member count: 2
            pub const Blend2DMode = enum(u32) {
                Blend2DMode_General = 0x0,
                Blend2DMode_Directional = 0x1
            };
            // Alignment: 1
            // Member count: 3
            pub const HandshakeTagState_t = enum(u8) {
                eInactive = 0x0,
                eActive = 0x1,
                eMomentarilyInactive = 0x2
            };
            // Alignment: 4
            // Member count: 3
            pub const ChoiceChangeMethod = enum(u32) {
                OnReset = 0x0,
                OnCycleEnd = 0x1,
                OnResetOrCycleEnd = 0x2
            };
            // Alignment: 4
            // Member count: 2
            pub const ChoiceBlendMethod = enum(u32) {
                SingleBlendTime = 0x0,
                PerChoiceBlendTimes = 0x1
            };
            // Alignment: 4
            // Member count: 4
            pub const VPhysXConstraintParams_t__EnumFlags0_t = enum(u32) {
                FLAG0_SHIFT_INTERPENETRATE = 0x0,
                FLAG0_SHIFT_CONSTRAIN = 0x1,
                FLAG0_SHIFT_BREAKABLE_FORCE = 0x2,
                FLAG0_SHIFT_BREAKABLE_TORQUE = 0x3
            };
            // Alignment: 4
            // Member count: 9
            pub const ParticleFloatMapType_t = enum(u32) {
                PF_MAP_TYPE_INVALID = 0xFFFFFFFF,
                PF_MAP_TYPE_DIRECT = 0x0,
                PF_MAP_TYPE_MULT = 0x1,
                PF_MAP_TYPE_REMAP = 0x2,
                PF_MAP_TYPE_REMAP_BIASED = 0x3,
                PF_MAP_TYPE_CURVE = 0x4,
                PF_MAP_TYPE_NOTCHED = 0x5,
                PF_MAP_TYPE_ROUND = 0x6,
                PF_MAP_TYPE_COUNT = 0x7
            };
            // Alignment: 4
            // Member count: 5
            pub const AnimParamVectorType_t = enum(u32) {
                ANIMPARAM_VECTOR_TYPE_NONE = 0x0,
                ANIMPARAM_VECTOR_TYPE_POSITION_WS = 0x1,
                ANIMPARAM_VECTOR_TYPE_POSITION_LS = 0x2,
                ANIMPARAM_VECTOR_TYPE_DIRECTION_WS = 0x3,
                ANIMPARAM_VECTOR_TYPE_DIRECTION_LS = 0x4
            };
            // Alignment: 1
            // Member count: 3
            pub const CNmCurrentSyncEventNode__InfoType_t = enum(u8) {
                IndexAndPercentage = 0x0,
                IndexOnly = 0x1,
                PercentageOnly = 0x2
            };
            // Alignment: 4
            // Member count: 4
            pub const BlendKeyType = enum(u32) {
                BlendKey_UserValue = 0x0,
                BlendKey_Velocity = 0x1,
                BlendKey_Distance = 0x2,
                BlendKey_RemainingDistance = 0x3
            };
            // Alignment: 4
            // Member count: 5
            pub const StateActionBehavior = enum(u32) {
                STATETAGBEHAVIOR_ACTIVE_WHILE_CURRENT = 0x0,
                STATETAGBEHAVIOR_FIRE_ON_ENTER = 0x1,
                STATETAGBEHAVIOR_FIRE_ON_EXIT = 0x2,
                STATETAGBEHAVIOR_FIRE_ON_ENTER_AND_EXIT = 0x3,
                STATETAGBEHAVIOR_ACTIVE_WHILE_FULLY_BLENDED = 0x4
            };
            // Alignment: 1
            // Member count: 4
            pub const NmRootMotionBlendMode_t = enum(u8) {
                Blend = 0x0,
                Additive = 0x1,
                IgnoreSource = 0x2,
                IgnoreTarget = 0x3
            };
            // Alignment: 1
            // Member count: 7
            pub const NmFootPhaseCondition_t = enum(u8) {
                LeftFootDown = 0x0,
                LeftFootPassing = 0x1,
                LeftPhase = 0x4,
                RightFootDown = 0x2,
                RightFootPassing = 0x3,
                RightPhase = 0x5,
                None = 0x6
            };
            // Alignment: 4
            // Member count: 22
            pub const ModelSkeletonData_t__BoneFlags_t = enum(u32) {
                FLAG_NO_BONE_FLAGS = 0x0,
                FLAG_BONEFLEXDRIVER = 0x4,
                FLAG_CLOTH = 0x8,
                FLAG_PHYSICS = 0x10,
                FLAG_ATTACHMENT = 0x20,
                FLAG_ANIMATION = 0x40,
                FLAG_MESH = 0x80,
                FLAG_HITBOX = 0x100,
                FLAG_BONE_USED_BY_VERTEX_LOD0 = 0x400,
                FLAG_BONE_USED_BY_VERTEX_LOD1 = 0x800,
                FLAG_BONE_USED_BY_VERTEX_LOD2 = 0x1000,
                FLAG_BONE_USED_BY_VERTEX_LOD3 = 0x2000,
                FLAG_BONE_USED_BY_VERTEX_LOD4 = 0x4000,
                FLAG_BONE_USED_BY_VERTEX_LOD5 = 0x8000,
                FLAG_BONE_USED_BY_VERTEX_LOD6 = 0x10000,
                FLAG_BONE_USED_BY_VERTEX_LOD7 = 0x20000,
                FLAG_BONE_MERGE_READ = 0x40000,
                FLAG_BONE_MERGE_WRITE = 0x80000,
                FLAG_ALL_BONE_FLAGS = 0xFFFFF,
                BLEND_PREALIGNED = 0x100000,
                FLAG_RIGIDLENGTH = 0x200000,
                FLAG_PROCEDURAL = 0x400000
            };
            // Alignment: 4
            // Member count: 3
            pub const GPUParticleCollisionMode_t = enum(u32) {
                PARTICLE_GPU_COLLISION_MODE_RT = 0x0,
                PARTICLE_GPU_COLLISION_MODE_DEPTH = 0x1,
                PARTICLE_GPU_COLLISION_MODE_HYBRID = 0x2
            };
            // Alignment: 4
            // Member count: 4
            pub const MorphBundleType_t = enum(u32) {
                MORPH_BUNDLE_TYPE_NONE = 0x0,
                MORPH_BUNDLE_TYPE_POSITION_SPEED = 0x1,
                MORPH_BUNDLE_TYPE_NORMAL_WRINKLE = 0x2,
                MORPH_BUNDLE_TYPE_COUNT = 0x3
            };
            // Alignment: 1
            // Member count: 2
            pub const CNmIDComparisonNode__Comparison_t = enum(u8) {
                Matches = 0x0,
                DoesntMatch = 0x1
            };
            // Alignment: 1
            // Member count: 3
            pub const NmPoseBlendMode_t = enum(u8) {
                Overlay = 0x0,
                Additive = 0x1,
                ModelSpace = 0x2
            };
            // Alignment: 4
            // Member count: 4
            pub const ParticleFloatInputMode_t = enum(u32) {
                PF_INPUT_MODE_INVALID = 0xFFFFFFFF,
                PF_INPUT_MODE_CLAMPED = 0x0,
                PF_INPUT_MODE_LOOPED = 0x1,
                PF_INPUT_MODE_COUNT = 0x2
            };
            // Alignment: 4
            // Member count: 5
            pub const ResetCycleOption = enum(u32) {
                Beginning = 0x0,
                SameCycleAsSource = 0x1,
                InverseSourceCycle = 0x2,
                FixedValue = 0x3,
                SameTimeAsSource = 0x4
            };
            // Alignment: 1
            // Member count: 6
            pub const CNmVectorInfoNode__Info_t = enum(u8) {
                X = 0x0,
                Y = 0x1,
                Z = 0x2,
                Length = 0x3,
                AngleHorizontal = 0x4,
                AngleVertical = 0x5
            };
            // Alignment: 4
            // Member count: 4
            pub const IKChannelMode = enum(u32) {
                TwoBone = 0x0,
                TwoBone_Translate = 0x1,
                OneBone = 0x2,
                OneBone_Translate = 0x3
            };
            // Alignment: 1
            // Member count: 9
            pub const NmGraphValueType_t = enum(u8) {
                Unknown = 0x0,
                Bool = 0x1,
                ID = 0x2,
                Float = 0x3,
                Vector = 0x4,
                Target = 0x5,
                BoneMask = 0x6,
                Pose = 0x7,
                Special = 0x8
            };
            // Alignment: 4
            // Member count: 4
            pub const ParticleFloatRandomMode_t = enum(u32) {
                PF_RANDOM_MODE_INVALID = 0xFFFFFFFF,
                PF_RANDOM_MODE_CONSTANT = 0x0,
                PF_RANDOM_MODE_VARYING = 0x1,
                PF_RANDOM_MODE_COUNT = 0x2
            };
            // Alignment: 4
            // Member count: 4
            pub const PFNoiseModifier_t = enum(u32) {
                PF_NOISE_MODIFIER_NONE = 0x0,
                PF_NOISE_MODIFIER_LINES = 0x1,
                PF_NOISE_MODIFIER_CLUMPS = 0x2,
                PF_NOISE_MODIFIER_RINGS = 0x3
            };
            // Alignment: 4
            // Member count: 21
            pub const ParticleVecType_t = enum(u32) {
                PVEC_TYPE_INVALID = 0xFFFFFFFF,
                PVEC_TYPE_LITERAL = 0x0,
                PVEC_TYPE_LITERAL_COLOR = 0x1,
                PVEC_TYPE_NAMED_VALUE = 0x2,
                PVEC_TYPE_PARTICLE_VECTOR = 0x3,
                PVEC_TYPE_PARTICLE_INITIAL_VECTOR = 0x4,
                PVEC_TYPE_PARTICLE_VELOCITY = 0x5,
                PVEC_TYPE_PARTICLE_GRAVITY = 0x6,
                PVEC_TYPE_CP_VALUE = 0x7,
                PVEC_TYPE_CP_RELATIVE_POSITION = 0x8,
                PVEC_TYPE_CP_RELATIVE_DIR = 0x9,
                PVEC_TYPE_CP_RELATIVE_RANDOM_DIR = 0xA,
                PVEC_TYPE_FLOAT_COMPONENTS = 0xB,
                PVEC_TYPE_FLOAT_INTERP_CLAMPED = 0xC,
                PVEC_TYPE_FLOAT_INTERP_OPEN = 0xD,
                PVEC_TYPE_FLOAT_INTERP_GRADIENT = 0xE,
                PVEC_TYPE_RANDOM_UNIFORM = 0xF,
                PVEC_TYPE_RANDOM_UNIFORM_OFFSET = 0x10,
                PVEC_TYPE_CP_DELTA = 0x11,
                PVEC_TYPE_CLOSEST_CAMERA_POSITION = 0x12,
                PVEC_TYPE_COUNT = 0x13
            };
            // Alignment: 1
            // Member count: 5
            pub const NmFootPhase_t = enum(u8) {
                LeftFootDown = 0x0,
                RightFootPassing = 0x1,
                RightFootDown = 0x2,
                LeftFootPassing = 0x3,
                None = 0x4
            };
            // Alignment: 4
            // Member count: 8
            pub const CNmTargetInfoNode__Info_t = enum(u32) {
                AngleHorizontal = 0x0,
                AngleVertical = 0x1,
                Distance = 0x2,
                DistanceHorizontalOnly = 0x3,
                DistanceVerticalOnly = 0x4,
                DeltaOrientationX = 0x5,
                DeltaOrientationY = 0x6,
                DeltaOrientationZ = 0x7
            };
            // Alignment: 4
            // Member count: 3
            pub const FootstepLandedFootSoundType_t = enum(u32) {
                FOOTSOUND_Left = 0x0,
                FOOTSOUND_Right = 0x1,
                FOOTSOUND_UseOverrideSound = 0x2
            };
            // Alignment: 4
            // Member count: 2
            pub const FootLockSubVisualization = enum(u32) {
                FOOTLOCKSUBVISUALIZATION_ReachabilityAnalysis = 0x0,
                FOOTLOCKSUBVISUALIZATION_IKSolve = 0x1
            };
            // Alignment: 4
            // Member count: 5
            pub const CNmSoundEvent__Position_t = enum(u32) {
                None = 0x0,
                World = 0x1,
                EntityPos = 0x2,
                EntityEyePos = 0x3,
                EntityAttachment = 0x4
            };
            // Alignment: 1
            // Member count: 4
            pub const FootstepJumpPhase_t = enum(u8) {
                Unknown = 0x0,
                NotJumping = 0x1,
                Jumping = 0x2,
                Landing = 0x4
            };
            // Alignment: 4
            // Member count: 2
            pub const NmFrameSnapEventMode_t = enum(u32) {
                Floor = 0x0,
                Round = 0x1
            };
            // Alignment: 4
            // Member count: 3
            pub const FootPinningTimingSource = enum(u32) {
                FootMotion = 0x0,
                Tag = 0x1,
                Parameter = 0x2
            };
            // Alignment: 4
            // Member count: 4
            pub const DampingSpeedFunction = enum(u32) {
                NoDamping = 0x0,
                Constant = 0x1,
                Spring = 0x2,
                AsymmetricSpring = 0x3
            };
            // Alignment: 4
            // Member count: 6
            pub const AnimationProcessingType_t = enum(u32) {
                ANIMATION_PROCESSING_SERVER_SIMULATION = 0x0,
                ANIMATION_PROCESSING_CLIENT_SIMULATION = 0x1,
                ANIMATION_PROCESSING_CLIENT_PREDICTION = 0x2,
                ANIMATION_PROCESSING_CLIENT_INTERPOLATION = 0x3,
                ANIMATION_PROCESSING_CLIENT_RENDER = 0x4,
                ANIMATION_PROCESSING_MAX = 0x5
            };
            // Alignment: 4
            // Member count: 3
            pub const JiggleBoneSimSpace = enum(u32) {
                SimSpace_Local = 0x0,
                SimSpace_Model = 0x1,
                SimSpace_World = 0x2
            };
            // Alignment: 4
            // Member count: 2
            pub const StanceOverrideMode = enum(u32) {
                Sequence = 0x0,
                Node = 0x1
            };
            // Alignment: 4
            // Member count: 2
            pub const IkEndEffectorType = enum(u32) {
                IkEndEffector_Attachment = 0x0,
                IkEndEffector_Bone = 0x1
            };
            // Alignment: 2
            // Member count: 3
            pub const AnimScriptType = enum(u16) {
                ANIMSCRIPT_TYPE_INVALID = 0xFFFF,
                ANIMSCRIPT_FUSE_GENERAL = 0x0,
                ANIMSCRIPT_FUSE_STATEMACHINE = 0x1
            };
            // Alignment: 1
            // Member count: 3
            pub const CNmTimeConditionNode__ComparisonType_t = enum(u8) {
                PercentageThroughState = 0x0,
                PercentageThroughSyncEvent = 0x1,
                ElapsedTime = 0x2
            };
            // Alignment: 4
            // Member count: 4
            pub const SeqPoseSetting_t = enum(u32) {
                SEQ_POSE_SETTING_CONSTANT = 0x0,
                SEQ_POSE_SETTING_ROTATION = 0x1,
                SEQ_POSE_SETTING_POSITION = 0x2,
                SEQ_POSE_SETTING_VELOCITY = 0x3
            };
            // Alignment: 4
            // Member count: 13
            pub const AnimParamButton_t = enum(u32) {
                ANIMPARAM_BUTTON_NONE = 0x0,
                ANIMPARAM_BUTTON_DPAD_UP = 0x1,
                ANIMPARAM_BUTTON_DPAD_RIGHT = 0x2,
                ANIMPARAM_BUTTON_DPAD_DOWN = 0x3,
                ANIMPARAM_BUTTON_DPAD_LEFT = 0x4,
                ANIMPARAM_BUTTON_A = 0x5,
                ANIMPARAM_BUTTON_B = 0x6,
                ANIMPARAM_BUTTON_X = 0x7,
                ANIMPARAM_BUTTON_Y = 0x8,
                ANIMPARAM_BUTTON_LEFT_SHOULDER = 0x9,
                ANIMPARAM_BUTTON_RIGHT_SHOULDER = 0xA,
                ANIMPARAM_BUTTON_LTRIGGER = 0xB,
                ANIMPARAM_BUTTON_RTRIGGER = 0xC
            };
            // Alignment: 4
            // Member count: 3
            pub const SelectorTagBehavior_t = enum(u32) {
                SelectorTagBehavior_OnWhileCurrent = 0x0,
                SelectorTagBehavior_OffWhenFinished = 0x1,
                SelectorTagBehavior_OffBeforeFinished = 0x2
            };
            // Alignment: 4
            // Member count: 4
            pub const HandshakeTagType_t = enum(u32) {
                eInvalid = 0xFFFFFFFF,
                eTask = 0x0,
                eMovement = 0x1,
                eCount = 0x2
            };
            // Alignment: 4
            // Member count: 4
            pub const OrientationWarpTargetOffsetMode_t = enum(u32) {
                eLiteralValue = 0x0,
                eParameter = 0x1,
                eAnimationMovementHeading = 0x2,
                eAnimationMovementHeadingAtEnd = 0x3
            };
            // Alignment: 4
            // Member count: 3
            pub const OrientationWarpMode_t = enum(u32) {
                eInvalid = 0x0,
                eAngle = 0x1,
                eWorldPosition = 0x2
            };
            // Alignment: 4
            // Member count: 5
            pub const ParticleTransformType_t = enum(u32) {
                PT_TYPE_INVALID = 0x0,
                PT_TYPE_NAMED_VALUE = 0x1,
                PT_TYPE_CONTROL_POINT = 0x2,
                PT_TYPE_CONTROL_POINT_RANGE = 0x3,
                PT_TYPE_COUNT = 0x4
            };
            // Alignment: 4
            // Member count: 18
            pub const ParticleAttachment_t = enum(u32) {
                PATTACH_INVALID = 0xFFFFFFFF,
                PATTACH_ABSORIGIN = 0x0,
                PATTACH_ABSORIGIN_FOLLOW = 0x1,
                PATTACH_CUSTOMORIGIN = 0x2,
                PATTACH_CUSTOMORIGIN_FOLLOW = 0x3,
                PATTACH_POINT = 0x4,
                PATTACH_POINT_FOLLOW = 0x5,
                PATTACH_EYES_FOLLOW = 0x6,
                PATTACH_OVERHEAD_FOLLOW = 0x7,
                PATTACH_WORLDORIGIN = 0x8,
                PATTACH_ROOTBONE_FOLLOW = 0x9,
                PATTACH_RENDERORIGIN_FOLLOW = 0xA,
                PATTACH_MAIN_VIEW = 0xB,
                PATTACH_WATERWAKE = 0xC,
                PATTACH_CENTER_FOLLOW = 0xD,
                PATTACH_CUSTOM_GAME_STATE_1 = 0xE,
                PATTACH_HEALTHBAR = 0xF,
                MAX_PATTACH_TYPES = 0x10
            };
            // Alignment: 4
            // Member count: 4
            pub const CNmEventTargetEntity_t = enum(u32) {
                Self = 0x0,
                Weapon = 0x1,
                HeldItem = 0x2,
                Custom = 0x3
            };
            // Alignment: 4
            // Member count: 3
            pub const FieldNetworkOption = enum(u32) {
                Auto = 0x0,
                ForceEnable = 0x1,
                ForceDisable = 0x2
            };
            // Alignment: 1
            // Member count: 6
            pub const NmGraphEventTypeCondition_t = enum(u8) {
                Entry = 0x0,
                FullyInState = 0x1,
                Exit = 0x2,
                Timed = 0x3,
                Generic = 0x4,
                Any = 0x5
            };
            // Alignment: 1
            // Member count: 10
            pub const CNmTransitionNode__TransitionOptions_t = enum(u8) {
                None = 0x0,
                ClampDuration = 0x1,
                Synchronized = 0x2,
                MatchSourceTime = 0x3,
                MatchSyncEventIndex = 0x4,
                MatchSyncEventID = 0x5,
                MatchSyncEventPercentage = 0x6,
                PreferClosestSyncEventID = 0x7,
                MatchTimeInSeconds = 0x8,
                OffsetTimeInSeconds = 0x9
            };
            // Alignment: 1
            // Member count: 5
            pub const CNmFloatComparisonNode__Comparison_t = enum(u8) {
                GreaterThanEqual = 0x0,
                LessThanEqual = 0x1,
                NearEqual = 0x2,
                GreaterThan = 0x3,
                LessThan = 0x4
            };
            // Alignment: 4
            // Member count: 3
            pub const VPhysXJoint_t__Flags_t = enum(u32) {
                JOINT_FLAGS_NONE = 0x0,
                JOINT_FLAGS_BODY1_FIXED = 0x1,
                JOINT_FLAGS_USE_BLOCK_SOLVER = 0x2
            };
            // Alignment: 4
            // Member count: 4
            pub const ScriptedHeldWeaponBehavior_t = enum(u32) {
                eInvalid = 0xFFFFFFFF,
                eHolster = 0x0,
                eDeploy = 0x1,
                eDrop = 0x2
            };
            // Alignment: 1
            // Member count: 3
            pub const VelocityMetricMode = enum(u8) {
                DirectionOnly = 0x0,
                MagnitudeOnly = 0x1,
                DirectionAndMagnitude = 0x2
            };
            // Alignment: 1
            // Member count: 5
            pub const FacingMode = enum(u8) {
                FacingMode_Invalid = 0x0,
                FacingMode_Manual = 0x1,
                FacingMode_Path = 0x2,
                FacingMode_LookTarget = 0x3,
                FacingMode_ManualPosition = 0x4
            };
            // Alignment: 1
            // Member count: 3
            pub const VertexAlbedoFormat_t = enum(u8) {
                VERTEX_ALBEDO_NONE = 0x0,
                VERTEX_ALBEDO_8888 = 0x1,
                VERTEX_ALBEDO_565 = 0x2
            };
            // Alignment: 4
            // Member count: 4
            pub const AimMatrixBlendMode = enum(u32) {
                AimMatrixBlendMode_None = 0x0,
                AimMatrixBlendMode_Additive = 0x1,
                AimMatrixBlendMode_ModelSpaceAdditive = 0x2,
                AimMatrixBlendMode_BoneMask = 0x3
            };
            // Alignment: 4
            // Member count: 7
            pub const AnimationSnapshotType_t = enum(u32) {
                ANIMATION_SNAPSHOT_SERVER_SIMULATION = 0x0,
                ANIMATION_SNAPSHOT_CLIENT_SIMULATION = 0x1,
                ANIMATION_SNAPSHOT_CLIENT_PREDICTION = 0x2,
                ANIMATION_SNAPSHOT_CLIENT_INTERPOLATION = 0x3,
                ANIMATION_SNAPSHOT_CLIENT_RENDER = 0x4,
                ANIMATION_SNAPSHOT_FINAL_COMPOSITE = 0x5,
                ANIMATION_SNAPSHOT_MAX = 0x6
            };
            // Alignment: 1
            // Member count: 5
            pub const NmTargetWarpRule_t = enum(u8) {
                WarpXY = 0x0,
                WarpZ = 0x1,
                WarpXYZ = 0x2,
                RotationOnly = 0x3,
                FixedSection = 0x4
            };
            // Alignment: 4
            // Member count: 8
            pub const FootFallTagFoot_t = enum(u32) {
                FOOT1 = 0x0,
                FOOT2 = 0x1,
                FOOT3 = 0x2,
                FOOT4 = 0x3,
                FOOT5 = 0x4,
                FOOT6 = 0x5,
                FOOT7 = 0x6,
                FOOT8 = 0x7
            };
            // Alignment: 4
            // Member count: 4
            pub const ChoiceMethod = enum(u32) {
                WeightedRandom = 0x0,
                WeightedRandomNoRepeat = 0x1,
                Iterate = 0x2,
                IterateRandom = 0x3
            };
            // Alignment: 4
            // Member count: 14
            pub const AnimVectorSource = enum(u32) {
                MoveDirection = 0x0,
                FacingPosition = 0x1,
                LookDirection = 0x2,
                VectorParameter = 0x3,
                WayPointDirection = 0x4,
                Acceleration = 0x5,
                SlopeNormal = 0x6,
                SlopeNormal_WorldSpace = 0x7,
                LookTarget = 0x8,
                LookTarget_WorldSpace = 0x9,
                WayPointPosition = 0xA,
                GoalPosition = 0xB,
                RootMotionVelocity = 0xC,
                ManualTarget_WorldSpace = 0xD
            };
            // Alignment: 4
            // Member count: 4
            pub const IkTargetType = enum(u32) {
                IkTarget_Attachment = 0x0,
                IkTarget_Bone = 0x1,
                IkTarget_Parameter_ModelSpace = 0x2,
                IkTarget_Parameter_WorldSpace = 0x3
            };
            // Alignment: 1
            // Member count: 3
            pub const RenderMeshSlotType_t = enum(u8) {
                RENDERMESH_SLOT_INVALID = 0xFF,
                RENDERMESH_SLOT_PER_VERTEX = 0x0,
                RENDERMESH_SLOT_PER_INSTANCE = 0x1
            };
            // Alignment: 4
            // Member count: 4
            pub const BoneMaskBlendSpace = enum(u32) {
                BlendSpace_Parent = 0x0,
                BlendSpace_Model = 0x1,
                BlendSpace_Model_RotationOnly = 0x2,
                BlendSpace_Model_TranslationOnly = 0x3
            };
            // Alignment: 4
            // Member count: 10
            pub const MovementCapability_t = enum(u32) {
                eStrafe = 0x0,
                eIdleTurn = 0x1,
                eStart = 0x2,
                eStop = 0x3,
                eInstantStop = 0x4,
                eShuffle = 0x5,
                ePlantedTurn = 0x6,
                eUseStartAsPlantedTurn = 0x7,
                eLean = 0x8,
                eCount = 0x9
            };
            // Alignment: 4
            // Member count: 5
            pub const ModelConfigAttachmentType_t = enum(u32) {
                MODEL_CONFIG_ATTACHMENT_INVALID = 0xFFFFFFFF,
                MODEL_CONFIG_ATTACHMENT_BONE_OR_ATTACHMENT = 0x0,
                MODEL_CONFIG_ATTACHMENT_ROOT_RELATIVE = 0x1,
                MODEL_CONFIG_ATTACHMENT_BONEMERGE = 0x2,
                MODEL_CONFIG_ATTACHMENT_COUNT = 0x3
            };
            // Alignment: 4
            // Member count: 2
            pub const BinaryNodeChildOption = enum(u32) {
                Child1 = 0x0,
                Child2 = 0x1
            };
            // Alignment: 4
            // Member count: 5
            pub const NPCPhysicsHullType_t = enum(u32) {
                eInvalid = 0x0,
                eGroundCapsule = 0x1,
                eCenteredCapsule = 0x2,
                eGenericCapsule = 0x3,
                eGroundBox = 0x4
            };
            // Alignment: 4
            // Member count: 2
            pub const JumpCorrectionMethod = enum(u32) {
                ScaleMotion = 0x0,
                AddCorrectionDelta = 0x1
            };
            // Alignment: 4
            // Member count: 2
            pub const MoodType_t = enum(u32) {
                eMoodType_Head = 0x0,
                eMoodType_Body = 0x1
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_WaitForCursorsWithTag = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CPulseCell_Base = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPulse_ResumePoint = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_PickBestOutflowSelector = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CParticleBindingRealPulse = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_WaitForObservable = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CPulse_OutflowConnection = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CPulseGraphDef = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_FireCursors = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CPulseCell_Timeline__TimelineEvent_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CPulseCell_IntervalTimer__CursorState_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_BaseRequirement = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_BaseState = struct {
            };
            // Parent: None
            // Field count: 48
            pub const OutflowWithRequirements_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_IsRequirementValid = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Value_Gradient = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CPulseCursorFuncs = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const PulseNodeDynamicOutflows_t__DynamicOutflow_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 45
            pub const CBasePulseGraphInstance = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_Inflow_GraphHook = struct {
            };
            // Parent: None
            // Field count: 0
            pub const SignatureOutflow_Resume = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Inflow_BaseEntrypoint = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_WaitForCursorsWithTagBase = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CPulse_InvokeBinding = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_IntervalTimer = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CPulseTestScriptLib = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_BaseLerp = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Value_Curve = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_Inflow_EventHandler = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_BaseFlow = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CPulseCell_Outflow_CycleShuffled__InstanceState_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CPulseCell_BaseLerp__CursorState_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 45
            pub const CPulseCell_WaitForCursorsWithTagBase__CursorState_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 45
            pub const CPulseArraylib = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 0
            pub const SignatureOutflow_Continue = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Timeline = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Inflow_EntOutputHandler = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CPulseCell_Outflow_CycleOrdered__InstanceState_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CParticleCollectionBindingInstance = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CPulseCell_LimitCount__InstanceState_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_Step_DebugLog = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_BaseYieldingInflow = struct {
            };
            // Parent: None
            // Field count: 48
            pub const PulseNodeDynamicOutflows_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 45
            pub const CPulseCell_IsRequirementValid__Criteria_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Inflow_ObservableVariableListener = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_Outflow_CycleOrdered = struct {
            };
            // Parent: None
            // Field count: 48
            pub const PulseSelectorOutflowList_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_Inflow_Wait = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_Outflow_CycleShuffled = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Inflow_Method = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_BaseValue = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_BooleanSwitchState = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_Inflow_Yield = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CPulseMathlib = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Unknown = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_Outflow_CycleRandom = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_Step_PublicOutput = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CPulse_BlackboardReference = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Value_RandomInt = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CAnimationLayer = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CPulse_CallInfo = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_InlineNodeSkipSelector = struct {
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_LimitCount = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Step_CallExternalMethod = struct {
            };
            // Parent: None
            // Field count: 48
            pub const PulseObservableBoolExpression_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 45
            pub const CPulseCell_LimitCount__Criteria_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub const CPulseCell_CursorQueue = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPulseCell_Value_RandomFloat = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CPulseExecCursor = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CAnimFrameBlockAnim = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CZeroPoseUpdateNode = struct {
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CSlowDownOnSlopesUpdateNode = struct {
            };
            // Parent: None
            // Field count: 0
            pub const COrientationWarpUpdateNode = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CAnimParameterBase = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CSequenceFinishedAnimTag = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CSceneObjectData = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: modellib
            // Field count: 0
            pub const CFootCycle = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CParticleModelInput = struct {
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CInputStreamUpdateNode = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CIntAnimParameter = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmParameterizedBlendNode__CDefinition = struct {
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmScaleTask = struct {
            };
            // Parent: None
            // Field count: 54
            pub const PerTickSettings_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmFrameSnapEvent = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CAnimNodePath = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const AnimNodeOutputID = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CNmSyncTrack = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CSeqBoneMaskList = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CAnimEncodedFrames = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 57
            pub const SampleCode = struct {
                pub const @"": usize = 0x1FFEEFF; // 
                pub const @"": usize = 0x100; // 
                pub const @"": usize = 0x1000000; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x2C000; // 
                pub const @"": usize = 0x10F; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x107; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x103; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0xE02FA7; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5C006100; // 
                pub const @"": usize = 0x77006F00; // 
                pub const @"": usize = 0x66007400; // 
                pub const @"": usize = 0x36002E00; // 
                pub const @"": usize = 0x5C003A00; // 
                pub const @"": usize = 0x72006300; // 
                pub const @"": usize = 0x65004400; // 
                pub const @"": usize = 0x5C006D00; // 
                pub const @"": usize = 0x4D005C00; // 
                pub const @"": usize = 0x69006600; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x64006E00; // 
                pub const @"": usize = 0x61006C00; // 
                pub const @"": usize = 0x30003200; // 
                pub const @"": usize = 0x6E00; // 
                pub const @"": usize = 0x7F; // 
                pub const @"": usize = 0x40000000; // 
                pub const @"": usize = 0x73006900; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0x69004400; // 
                pub const @"": usize = 0x65006E00; // 
                pub const @"": usize = 0x8E000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE000000; // 
                pub const @"": usize = 0x80000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x2000000; // 
                pub const @"": usize = 0xFFFF00; // 
                pub const @"": usize = 0x15050000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100; // 
                pub const @"": usize = 0x110; // 
            };
            // Parent: None
            // Field count: 48
            pub const CCachedPose = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmTransitionEventConditionNode__CDefinition = struct {
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CTaskStatusAnimTag = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CStateNodeInstanceData = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CMotionGraphConfig = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CMotionSearchNode = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CAnimActivity = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CAnimScriptManager = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const AnimationDecodeDebugDumpElement_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CChoreoInstanceData = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 56
            pub const ConfigIndex = struct {
                pub const @"": usize = 0x1FFEE; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x10000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x2C0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0xE02F; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5C0061; // 
                pub const @"": usize = 0x77006F; // 
                pub const @"": usize = 0x660074; // 
                pub const @"": usize = 0x36002E; // 
                pub const @"": usize = 0x5C003A; // 
                pub const @"": usize = 0x720063; // 
                pub const @"": usize = 0x650044; // 
                pub const @"": usize = 0x5C006D; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x690066; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x64006E; // 
                pub const @"": usize = 0x61006C; // 
                pub const @"": usize = 0x300032; // 
                pub const @"": usize = 0x6E; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xB6400000; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x690044; // 
                pub const @"": usize = 0x65006E; // 
                pub const @"": usize = 0xAA8E0000; // 
                pub const @"": usize = 0xEE0000; // 
                pub const @"": usize = 0x800000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D; // 
                pub const @"": usize = 0x20000; // 
                pub const @"": usize = 0xFFFF; // 
                pub const @"": usize = 0x150500; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x1; // 
            };
            // Parent: None
            // Field count: 48
            pub const VPhysXCollisionAttributes_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CSequenceUpdateNode = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CStateMachineInstanceData = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmPassthroughNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmClipReferenceNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CNmGraphDefinition__ExternalGraphSlot_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CAimCameraUpdateNode = struct {
            };
            // Parent: None
            // Field count: 48
            pub const LookData = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 57
            pub const CSeqSeqDescFlag = struct {
                pub const @"": usize = 0x1FFEEFF; // 
                pub const @"": usize = 0x100; // 
                pub const @"": usize = 0x1000000; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x2C000; // 
                pub const @"": usize = 0x10F; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x107; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x103; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0xE02FA7; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5C006100; // 
                pub const @"": usize = 0x77006F00; // 
                pub const @"": usize = 0x66007400; // 
                pub const @"": usize = 0x36002E00; // 
                pub const @"": usize = 0x5C003A00; // 
                pub const @"": usize = 0x72006300; // 
                pub const @"": usize = 0x65004400; // 
                pub const @"": usize = 0x5C006D00; // 
                pub const @"": usize = 0x4D005C00; // 
                pub const @"": usize = 0x69006600; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x64006E00; // 
                pub const @"": usize = 0x61006C00; // 
                pub const @"": usize = 0x30003200; // 
                pub const @"": usize = 0x6E00; // 
                pub const @"": usize = 0x7F; // 
                pub const @"": usize = 0x40000000; // 
                pub const @"": usize = 0x73006900; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0x69004400; // 
                pub const @"": usize = 0x65006E00; // 
                pub const @"": usize = 0x8E000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE000000; // 
                pub const @"": usize = 0x80000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x2000000; // 
                pub const @"": usize = 0xFFFF00; // 
                pub const @"": usize = 0x15050000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100; // 
                pub const @"": usize = 0x110; // 
            };
            // Parent: None
            // Field count: 48
            pub const PermModelInfo_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const StanceInfo_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 45
            pub const IKBoneNameAndIndex_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CStanceScaleUpdateNode = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmBoneMaskBlendNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmIDEventNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const HitReactFixedSettings_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmPoseNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CAnimTagBase = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPerParticleVecInput = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CMotionGraph = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmVectorInfoNode__CDefinition = struct {
            };
            // Parent: ___Y__
            // Field count: 0
            pub const AnimationSnapshot_t = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmBlendTask = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmVectorValueNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CSeqTransition = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 45
            pub const CNmPoseTask = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CAnimGraphSettingsGroup = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmVirtualParameterTargetNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CAnimGraphDebugReplay = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CVectorAnimParameter = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmCachedPoseReadTask = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CNmStateMachineNode__StateDefinition_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CStanceOverrideUpdateNode = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CNmGraphDefinition__ExternalPoseSlot_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const PhysShapeMarkup_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CBlend2DInstanceData = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const LookAtBone_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CDirectPlaybackInstanceData = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmTransitionNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmFloatEaseNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CAnimationGraphInstance = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CAnimEncodeDifference = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CRootUpdateNode = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmTargetOffsetNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 54
            pub const MotionMatchingInstanceData = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CMotionGraphUpdateNode = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmFloatAngleMathNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 57
            pub const CAnimParamHandle = struct {
                pub const @"": usize = 0x1FFEEFF; // 
                pub const @"": usize = 0x100; // 
                pub const @"": usize = 0x1000000; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x2C000; // 
                pub const @"": usize = 0x10F; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x107; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x103; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0xE02FA7; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5C006100; // 
                pub const @"": usize = 0x77006F00; // 
                pub const @"": usize = 0x66007400; // 
                pub const @"": usize = 0x36002E00; // 
                pub const @"": usize = 0x5C003A00; // 
                pub const @"": usize = 0x72006300; // 
                pub const @"": usize = 0x65004400; // 
                pub const @"": usize = 0x5C006D00; // 
                pub const @"": usize = 0x4D005C00; // 
                pub const @"": usize = 0x69006600; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x64006E00; // 
                pub const @"": usize = 0x61006C00; // 
                pub const @"": usize = 0x30003200; // 
                pub const @"": usize = 0x6E00; // 
                pub const @"": usize = 0x7F; // 
                pub const @"": usize = 0x40000000; // 
                pub const @"": usize = 0x73006900; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0x69004400; // 
                pub const @"": usize = 0x65006E00; // 
                pub const @"": usize = 0x8E000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE000000; // 
                pub const @"": usize = 0x80000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x2000000; // 
                pub const @"": usize = 0xFFFF00; // 
                pub const @"": usize = 0x15050000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100; // 
                pub const @"": usize = 0x110; // 
            };
            // Parent: None
            // Field count: 48
            pub const WeightList = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 56
            pub const MotionIndex = struct {
                pub const @"": usize = 0x1FFEE; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x10000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x2C0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0xE02F; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5C0061; // 
                pub const @"": usize = 0x77006F; // 
                pub const @"": usize = 0x660074; // 
                pub const @"": usize = 0x36002E; // 
                pub const @"": usize = 0x5C003A; // 
                pub const @"": usize = 0x720063; // 
                pub const @"": usize = 0x650044; // 
                pub const @"": usize = 0x5C006D; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x690066; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x64006E; // 
                pub const @"": usize = 0x61006C; // 
                pub const @"": usize = 0x300032; // 
                pub const @"": usize = 0x6E; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xB6400000; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x690044; // 
                pub const @"": usize = 0x65006E; // 
                pub const @"": usize = 0xAA8E0000; // 
                pub const @"": usize = 0xEE0000; // 
                pub const @"": usize = 0x800000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D; // 
                pub const @"": usize = 0x20000; // 
                pub const @"": usize = 0xFFFF; // 
                pub const @"": usize = 0x150500; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x1; // 
            };
            // Parent: None
            // Field count: 54
            pub const FollowAttachmentSettings_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmReferencedGraphNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const AnimNodeID = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmClipSelectorNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CLookComponentUpdater = struct {
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmIsExternalPoseSetNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CAnimGraphNetworkSettings = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CSingleFrameUpdateNode = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmChainLookatNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmFollowBoneNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CBlockSelectionMetricEvaluator = struct {
            };
            // Parent: None
            // Field count: 48
            pub const MovementData = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CRemapValueComponentUpdater = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CModelConfigList = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 57
            pub const CSeqAutoLayerFlag = struct {
                pub const @"": usize = 0x1FFEEFF; // 
                pub const @"": usize = 0x100; // 
                pub const @"": usize = 0x1000000; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x2C000; // 
                pub const @"": usize = 0x10F; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x107; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x103; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0xE02FA7; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5C006100; // 
                pub const @"": usize = 0x77006F00; // 
                pub const @"": usize = 0x66007400; // 
                pub const @"": usize = 0x36002E00; // 
                pub const @"": usize = 0x5C003A00; // 
                pub const @"": usize = 0x72006300; // 
                pub const @"": usize = 0x65004400; // 
                pub const @"": usize = 0x5C006D00; // 
                pub const @"": usize = 0x4D005C00; // 
                pub const @"": usize = 0x69006600; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x64006E00; // 
                pub const @"": usize = 0x61006C00; // 
                pub const @"": usize = 0x30003200; // 
                pub const @"": usize = 0x6E00; // 
                pub const @"": usize = 0x7F; // 
                pub const @"": usize = 0x40000000; // 
                pub const @"": usize = 0x73006900; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0x69004400; // 
                pub const @"": usize = 0x65006E00; // 
                pub const @"": usize = 0x8E000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE000000; // 
                pub const @"": usize = 0x80000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x2000000; // 
                pub const @"": usize = 0xFFFF00; // 
                pub const @"": usize = 0x15050000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100; // 
                pub const @"": usize = 0x110; // 
            };
            // Parent: None
            // Field count: 48
            pub const AnimStateID = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmIDEventConditionNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const ModelBoneFlexDriver_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CMorphData = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CFootPinningUpdateNode = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CSeqPoseSetting = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmVirtualParameterFloatNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 54
            pub const CAttachment = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CSequenceUpdateNodeBase = struct {
            };
            // Parent: None
            // Field count: 57
            pub const CDrawCullingData = struct {
                pub const @"": usize = 0x1FFEEFF; // 
                pub const @"": usize = 0x100; // 
                pub const @"": usize = 0x1000000; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x2C000; // 
                pub const @"": usize = 0x10F; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x107; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x103; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0xE02FA7; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5C006100; // 
                pub const @"": usize = 0x77006F00; // 
                pub const @"": usize = 0x66007400; // 
                pub const @"": usize = 0x36002E00; // 
                pub const @"": usize = 0x5C003A00; // 
                pub const @"": usize = 0x72006300; // 
                pub const @"": usize = 0x65004400; // 
                pub const @"": usize = 0x5C006D00; // 
                pub const @"": usize = 0x4D005C00; // 
                pub const @"": usize = 0x69006600; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x64006E00; // 
                pub const @"": usize = 0x61006C00; // 
                pub const @"": usize = 0x30003200; // 
                pub const @"": usize = 0x6E00; // 
                pub const @"": usize = 0x7F; // 
                pub const @"": usize = 0x40000000; // 
                pub const @"": usize = 0x73006900; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0x69004400; // 
                pub const @"": usize = 0x65006E00; // 
                pub const @"": usize = 0x8E000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE000000; // 
                pub const @"": usize = 0x80000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x2000000; // 
                pub const @"": usize = 0xFFFF00; // 
                pub const @"": usize = 0x15050000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100; // 
                pub const @"": usize = 0x110; // 
            };
            // Parent: None
            // Field count: 54
            pub const VPhysXJoint_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CStateNodeTransitionData = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CTimeRemainingMetricEvaluator = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CParticleInput = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 45
            pub const CModelConfigElement = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmControlParameterVectorNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 54
            pub const FootFixedSettings = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmBoneMaskSwitchNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CModelConfig = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const AnimComponentID = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const VPhysXAggregateData_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CNmGraphDefinition = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmSelectorNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CAnimDemoCaptureSettings = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const IKDemoCaptureSettings_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 57
            pub const CSeqMultiFetchFlag = struct {
                pub const @"": usize = 0x1FFEEFF; // 
                pub const @"": usize = 0x100; // 
                pub const @"": usize = 0x1000000; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x2C000; // 
                pub const @"": usize = 0x10F; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x107; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x103; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0xE02FA7; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5C006100; // 
                pub const @"": usize = 0x77006F00; // 
                pub const @"": usize = 0x66007400; // 
                pub const @"": usize = 0x36002E00; // 
                pub const @"": usize = 0x5C003A00; // 
                pub const @"": usize = 0x72006300; // 
                pub const @"": usize = 0x65004400; // 
                pub const @"": usize = 0x5C006D00; // 
                pub const @"": usize = 0x4D005C00; // 
                pub const @"": usize = 0x69006600; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x64006E00; // 
                pub const @"": usize = 0x61006C00; // 
                pub const @"": usize = 0x30003200; // 
                pub const @"": usize = 0x6E00; // 
                pub const @"": usize = 0x7F; // 
                pub const @"": usize = 0x40000000; // 
                pub const @"": usize = 0x73006900; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0x69004400; // 
                pub const @"": usize = 0x65006E00; // 
                pub const @"": usize = 0x8E000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE000000; // 
                pub const @"": usize = 0x80000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x2000000; // 
                pub const @"": usize = 0xFFFF00; // 
                pub const @"": usize = 0x15050000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100; // 
                pub const @"": usize = 0x110; // 
            };
            // Parent: None
            // Field count: 48
            pub const VPhysXRange_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CStopAtGoalUpdateNode = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmIDEventPercentageThroughNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CAnimBoneDifference = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmCachedBoolNode__CDefinition = struct {
            };
            // Parent: modellib
            // Field count: 0
            pub const CAnimCycle = struct {
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CActionComponentUpdater = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CDirectionalBlendInstanceData = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CDampedPathAnimMotorUpdater = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmCachedFloatNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmFloatSwitchNode__CDefinition = struct {
            };
            // Parent: modellib
            // Field count: 0
            pub const CModelConfigElement_SetMaterialGroupOnAttachedModels = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmEntityAttributeEventBase = struct {
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmVirtualParameterBoneMaskNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const SequenceData = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPathHelperUpdateNode = struct {
            };
            // Parent: None
            // Field count: 48
            pub const MaterialGroup_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CMorphBundleData = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: modellib
            // Field count: 0
            pub const CModelConfigElement_UserPick = struct {
            };
            // Parent: None
            // Field count: 57
            pub const CSolveIKTargetHandle_t = struct {
                pub const @"": usize = 0x1FFEEFF; // 
                pub const @"": usize = 0x100; // 
                pub const @"": usize = 0x1000000; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x2C000; // 
                pub const @"": usize = 0x10F; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x107; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x103; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0xE02FA7; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5C006100; // 
                pub const @"": usize = 0x77006F00; // 
                pub const @"": usize = 0x66007400; // 
                pub const @"": usize = 0x36002E00; // 
                pub const @"": usize = 0x5C003A00; // 
                pub const @"": usize = 0x72006300; // 
                pub const @"": usize = 0x65004400; // 
                pub const @"": usize = 0x5C006D00; // 
                pub const @"": usize = 0x4D005C00; // 
                pub const @"": usize = 0x69006600; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x64006E00; // 
                pub const @"": usize = 0x61006C00; // 
                pub const @"": usize = 0x30003200; // 
                pub const @"": usize = 0x6E00; // 
                pub const @"": usize = 0x7F; // 
                pub const @"": usize = 0x40000000; // 
                pub const @"": usize = 0x73006900; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0x69004400; // 
                pub const @"": usize = 0x65006E00; // 
                pub const @"": usize = 0x8E000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE000000; // 
                pub const @"": usize = 0x80000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x2000000; // 
                pub const @"": usize = 0xFFFF00; // 
                pub const @"": usize = 0x15050000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100; // 
                pub const @"": usize = 0x110; // 
            };
            // Parent: None
            // Field count: 0
            pub const CFootAdjustmentUpdateNode = struct {
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CChoreoUpdateNode = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CAimConstraint = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CMaterialDrawDescriptor__RigidMeshPart_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CFootFallAnimTag = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CMaterialDrawDescriptor = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CNmSkeleton = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CStepsRemainingMetricEvaluator = struct {
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmVelocityBlendNode__CDefinition = struct {
            };
            // Parent: _
            // Field count: 0
            pub const CNmZeroPoseTask = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CBodyGroupSetting = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CVectorQuantizer = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const ParticleNamedValueConfiguration_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmVectorCreateNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CParticleVecInput = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmStateNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmBoneMaskSelectorNode__CDefinition = struct {
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmVirtualParameterVectorNode__CDefinition = struct {
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CFootCycleMetricEvaluator = struct {
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmConstIDNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 54
            pub const CHitBox = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmParameterizedSelectorNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const ModelSkeletonData_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 45
            pub const CNmEvent = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const NmCompressionSettings_t__QuantizationRange_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmBlendTaskBase = struct {
            };
            // Parent: None
            // Field count: 48
            pub const TagSpan_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmConstTargetNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CRemapValueUpdateItem = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const PermModelDataAnimatedMaterialAttribute_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CMotionDataSet = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CProductQuantizer = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CChoiceUpdateNode = struct {
            };
            // Parent: None
            // Field count: 54
            pub const SkeletonAnimCapture_t__Bone_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CExpressionActionUpdater = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmFloatMathNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CAnimationGraphVisualizerLine = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CCycleControlUpdateNode = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmTargetInfoNode__CDefinition = struct {
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CDampedValueComponentUpdater = struct {
            };
            // Parent: _
            // Field count: 0
            pub const CBaseConstraint = struct {
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmControlParameterFloatNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const BlendItem_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const DampedPathMotorInstanceData_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmIsExternalGraphSlotFilledNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNewParticleEffect = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmClipNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmMaterialAttributeEvent = struct {
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmBlend1DNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CConcreteAnimParameter = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CNmBoneWeightList = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CPairedSequenceUpdateNode = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CModelConfigElement_RandomPick = struct {
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmReferencePoseTask = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmScaleNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const VPhysXConstraint2_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 54
            pub const CConstraintTarget = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const SkeletonAnimCapture_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CAnimUser = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CRagdollComponentUpdater = struct {
            };
            // Parent: None
            // Field count: 48
            pub const ParticleNamedValueSource_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CFootDefinition = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CSeqSynthAnimDesc = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 56
            pub const CNmLayerBlendNode__LayerDefinition_t = struct {
                pub const @"": usize = 0x1FFEE; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x10000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x2C0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0xE02F; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5C0061; // 
                pub const @"": usize = 0x77006F; // 
                pub const @"": usize = 0x660074; // 
                pub const @"": usize = 0x36002E; // 
                pub const @"": usize = 0x5C003A; // 
                pub const @"": usize = 0x720063; // 
                pub const @"": usize = 0x650044; // 
                pub const @"": usize = 0x5C006D; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x690066; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x64006E; // 
                pub const @"": usize = 0x61006C; // 
                pub const @"": usize = 0x300032; // 
                pub const @"": usize = 0x6E; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xB6400000; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x690044; // 
                pub const @"": usize = 0x65006E; // 
                pub const @"": usize = 0xAA8E0000; // 
                pub const @"": usize = 0xEE0000; // 
                pub const @"": usize = 0x800000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D; // 
                pub const @"": usize = 0x20000; // 
                pub const @"": usize = 0xFFFF; // 
                pub const @"": usize = 0x150500; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x1; // 
            };
            // Parent: None
            // Field count: 45
            pub const CAnimComponentUpdater = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CBlend2DUpdateNode = struct {
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmVelocityBasedSpeedScaleNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CNPCPhysicsHull = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 54
            pub const PermModelExtPart_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmCachedIDNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CLeanMatrixUpdateNode = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CSymbolAnimParameter = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmFloatCurveEventNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 45
            pub const IAnimationGraphInstance = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 56
            pub const CNmStateMachineNode__TransitionDefinition_t = struct {
                pub const @"": usize = 0x1FFEE; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x10000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x2C0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0xE02F; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5C0061; // 
                pub const @"": usize = 0x77006F; // 
                pub const @"": usize = 0x660074; // 
                pub const @"": usize = 0x36002E; // 
                pub const @"": usize = 0x5C003A; // 
                pub const @"": usize = 0x720063; // 
                pub const @"": usize = 0x650044; // 
                pub const @"": usize = 0x5C006D; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x690066; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x64006E; // 
                pub const @"": usize = 0x61006C; // 
                pub const @"": usize = 0x300032; // 
                pub const @"": usize = 0x6E; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xB6400000; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x690044; // 
                pub const @"": usize = 0x65006E; // 
                pub const @"": usize = 0xAA8E0000; // 
                pub const @"": usize = 0xEE0000; // 
                pub const @"": usize = 0x800000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D; // 
                pub const @"": usize = 0x20000; // 
                pub const @"": usize = 0xFFFF; // 
                pub const @"": usize = 0x150500; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x1; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmFloatSelectorNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CTargetSelectorUpdateNode = struct {
            };
            // Parent: None
            // Field count: 48
            pub const SlopeData = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CHitBoxSetList = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmIsInactiveBranchConditionNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const MotionDBIndex = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CDirectPlaybackTagData = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CParticleCollectionVecInput = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CNmGraphEventConditionNode__Condition_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CNmFloatRemapNode__RemapRange_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmCurrentSyncEventIDNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmStateCompletedConditionNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmTargetWarpEvent = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CParamSpanUpdater = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 45
            pub const CAnimActionUpdater = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CParticleAnimTag = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CCycleBase = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const FollowTargetOpFixedSettings_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmIDSwitchNode__CDefinition = struct {
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmRootMotionEvent = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CChoiceInstanceData = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 54
            pub const AnimationSnapshotBase_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CEmitTagActionUpdater = struct {
            };
            // Parent: None
            // Field count: 48
            pub const DynamicMeshDeformParams_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmTimeConditionNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmSyncEventIndexConditionNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CAnimStateMachineUpdater = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CUnaryUpdateNode = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmIDToFloatNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const PlayerInputMotorInstanceData_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 45
            pub const CCompressorGroup = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CSelectorUpdateNode = struct {
            };
            // Parent: None
            // Field count: 48
            pub const NmPercent_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CStateMachineComponentUpdater = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CAnimUpdateNodeRef = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CPhysSurfaceProperties = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmBoneMaskValueNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CSeqPoseParamDesc = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CModelConfigElement_SetBodygroup = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CAnimMovement = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CParticleRemapFloatInput = struct {
            };
            // Parent: None
            // Field count: 48
            pub const FollowAttachmentData = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const FootStepTrigger = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CNmSyncTrack__EventMarker_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPathMetricEvaluator = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CFootLockUpdateNode = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmOrientationWarpEvent = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmIDValueNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const JiggleBoneSettings_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CJumpHelperUpdateNode = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmControlParameterBoolNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmChainLookatTask = struct {
            };
            // Parent: None
            // Field count: 48
            pub const FootLockPoseOpFixedSettings = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CBoneVelocityMetricEvaluator = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CWarpSectionAnimTag = struct {
            };
            // Parent: None
            // Field count: 45
            pub const SelectorInstanceData_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: modellib
            // Field count: 0
            pub const CModelConfigElement_RandomColor = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CFootMotion = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CJiggleBoneUpdateNode = struct {
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmConstVectorNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const MoodAnimation_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CLODComponentUpdater = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmIsTargetSetNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CPhysSurfacePropertiesVehicle = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CAnimDataChannelDesc = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CPairedSequenceComponentUpdater = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmParticleEvent = struct {
            };
            // Parent: None
            // Field count: 48
            pub const MotionSelection = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CMorphRectData = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CAnimFoot = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: modellib
            // Field count: 0
            pub const CParentConstraint = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmCachedVectorNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CNmStateNode__TimedEvent_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CFootPositionMetricEvaluator = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CParticleVariableRef = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CMoverInstanceData = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 45
            pub const HSequence = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CDampedValueUpdateItem = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CAnimLocalHierarchy = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CBlendUpdateNode = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CQuaternionAnimParameter = struct {
            };
            // Parent: None
            // Field count: 48
            pub const PermModelData_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: _
            // Field count: 0
            pub const CNmSampleTask = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CFollowPathInstanceData = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CStaticPoseCacheBuilder = struct {
            };
            // Parent: None
            // Field count: 48
            pub const ModelMeshBufferData_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CSequenceGroupData = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CStateMachineUpdateNode = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmIDBasedSelectorNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CAimMatrixUpdateNode = struct {
            };
            // Parent: None
            // Field count: 54
            pub const SkeletonAnimCapture_t__Frame_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CModelConfigElement_Command = struct {
            };
            // Parent: None
            // Field count: 45
            pub const TargetSelectorInstanceData_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CAnimEventDefinition = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 54
            pub const CNmRootMotionData = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const AnimScriptHandle = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CFlexOp = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CHitReactUpdateNode = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CLookAtUpdateNode = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CAnimBone = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CAnimParameterManagerUpdater = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const BoneDemoCaptureSettings_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const PairedSequenceData = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 45
            pub const CBoneConstraintPoseSpaceMorph__Input_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const MoodAnimationLayer_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CSpeedScaleUpdateNode = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CSequenceTagSpans = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CModelConfigElement_SetBodygroupOnAttachedModels = struct {
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CCurrentVelocityMetricEvaluator = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmBoolValueNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 54
            pub const CAnimReplayFrame = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmTransitionEvent = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmFloatCurveEvent = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CParticleTransformInput = struct {
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CPathAnimMotorUpdaterBase = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmFootstepEventPercentageThroughNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const AimCameraOpFixedSettings_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 54
            pub const SkeletonAnimCapture_t__Camera_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmGraphEventConditionNode__CDefinition = struct {
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CFollowAttachmentUpdateNode = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CMovementComponentUpdater = struct {
            };
            // Parent: modellib
            // Field count: 0
            pub const CModelConfigElement_SetMaterialGroup = struct {
            };
            // Parent: None
            // Field count: 45
            pub const IParticleEffect = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CFutureFacingMetricEvaluator = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmIDBasedClipSelectorNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CLeanMatrixInstanceData = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 54
            pub const NmCompressionSettings_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 45
            pub const PARTICLE_EHANDLE__ = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: MGetKV3ClassDefaults
            // Field count: 0
            pub const CAddUpdateNode = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CAnimEnum = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CDemoSettingsComponentUpdater = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CAnimDecoder = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmCachedTargetNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CPerParticleFloatInput = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmConstBoolNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CNmBitFlags = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CFootStride = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CSceneObjectData__RTProxyDrawDescriptor_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: MGetKV3ClassDefaults
            // Field count: 0
            pub const CNmLegacyEvent = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CStateNodeStateData = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CFootstepLandedAnimTag = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CEnumAnimParameter = struct {
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CHandshakeAnimTagBase = struct {
            };
            // Parent: None
            // Field count: 48
            pub const AnimTagID = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CWayPointHelperInstanceData = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CAnimFrameSegment = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CSlopeComponentUpdater = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CAnimInputDamping = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CHitBoxSet = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 54
            pub const ChainToSolveData_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 54
            pub const CConstraintSlave = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CFootCycleDefinition = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CVirtualAnimParameter = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CMotionMetricEvaluator = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CFutureVelocityMetricEvaluator = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CNmSkeleton__SecondarySkeleton_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 45
            pub const CAnimUpdateNodeBase = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CSetParameterActionUpdater = struct {
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmFixedWeightBoneMaskNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const SkeletonAnimCapture_t__FrameStamp_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CBonePositionMetricEvaluator = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CRenderBufferBinding = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CParticleCollectionRendererFloatInput = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CNmParameterizedBlendNode__Parameterization_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmEntityAttributeIntEvent = struct {
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmNotNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CMeshletDescriptor = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CMorphSetData = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmAndNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CMorphConstraint = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CClothSettingsAnimTag = struct {
            };
            // Parent: None
            // Field count: 57
            pub const CAnimDesc_Flag = struct {
                pub const @"": usize = 0x1FFEEFF; // 
                pub const @"": usize = 0x100; // 
                pub const @"": usize = 0x1000000; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x2C000; // 
                pub const @"": usize = 0x10F; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x107; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x103; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0xE02FA7; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5C006100; // 
                pub const @"": usize = 0x77006F00; // 
                pub const @"": usize = 0x66007400; // 
                pub const @"": usize = 0x36002E00; // 
                pub const @"": usize = 0x5C003A00; // 
                pub const @"": usize = 0x72006300; // 
                pub const @"": usize = 0x65004400; // 
                pub const @"": usize = 0x5C006D00; // 
                pub const @"": usize = 0x4D005C00; // 
                pub const @"": usize = 0x69006600; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x64006E00; // 
                pub const @"": usize = 0x61006C00; // 
                pub const @"": usize = 0x30003200; // 
                pub const @"": usize = 0x6E00; // 
                pub const @"": usize = 0x7F; // 
                pub const @"": usize = 0x40000000; // 
                pub const @"": usize = 0x73006900; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0x69004400; // 
                pub const @"": usize = 0x65006E00; // 
                pub const @"": usize = 0x8E000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE000000; // 
                pub const @"": usize = 0x80000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x2000000; // 
                pub const @"": usize = 0xFFFF00; // 
                pub const @"": usize = 0x15050000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100; // 
                pub const @"": usize = 0x110; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmBlend2DNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CBlendCurve = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 45
            pub const CNmGraphNode__CDefinition = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CAnimationGraphVisualizerSphere = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CNmClip__ModelSpaceSamplingChainLink_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CAnimUserDifference = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmDurationScaleNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 57
            pub const CTransitionUpdateData = struct {
                pub const @"": usize = 0x1FFEEFF; // 
                pub const @"": usize = 0x100; // 
                pub const @"": usize = 0x1000000; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x2C000; // 
                pub const @"": usize = 0x10F; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x107; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x103; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0xE02FA7; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5C006100; // 
                pub const @"": usize = 0x77006F00; // 
                pub const @"": usize = 0x66007400; // 
                pub const @"": usize = 0x36002E00; // 
                pub const @"": usize = 0x5C003A00; // 
                pub const @"": usize = 0x72006300; // 
                pub const @"": usize = 0x65004400; // 
                pub const @"": usize = 0x5C006D00; // 
                pub const @"": usize = 0x4D005C00; // 
                pub const @"": usize = 0x69006600; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x64006E00; // 
                pub const @"": usize = 0x61006C00; // 
                pub const @"": usize = 0x30003200; // 
                pub const @"": usize = 0x6E00; // 
                pub const @"": usize = 0x7F; // 
                pub const @"": usize = 0x40000000; // 
                pub const @"": usize = 0x73006900; // 
                pub const @"": usize = 0x101; // 
                pub const @"": usize = 0x69004400; // 
                pub const @"": usize = 0x65006E00; // 
                pub const @"": usize = 0x8E000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE000000; // 
                pub const @"": usize = 0x80000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x2000000; // 
                pub const @"": usize = 0xFFFF00; // 
                pub const @"": usize = 0x15050000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100; // 
                pub const @"": usize = 0x110; // 
            };
            // Parent: None
            // Field count: 0
            pub const CMovementHandshakeAnimTag = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CBoneConstraintPoseSpaceBone__Input_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CSeqMultiFetch = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CPhysSurfacePropertiesSoundNames = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const NmBoneMaskSetDefinition_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CAudioAnimTag = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CTurnHelperUpdateNode = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CAnimationGroup = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const NmSyncTrackTime_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 54
            pub const CNmClip = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CToggleComponentActionUpdater = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmOverlayBlendTask = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmSpeedScaleBaseNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmValueNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CAnimationGraphVisualizerPrimitiveBase = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CBindPoseUpdateNode = struct {
            };
            // Parent: None
            // Field count: 48
            pub const NmSyncTrackTimeRange_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CAnimationGraphVisualizerText = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CFootTrajectory = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CMotionMatchingUpdateNode = struct {
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmReferencePoseNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CSeqS1SeqDesc = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CFlexController = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const SkeletonDemoDb_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmFloatRangeComparisonNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CMotionNodeSequence = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmTwoBoneIKNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CNmGraphInstance = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CFlexDesc = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CRenderMesh = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CAnimSkeleton = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CFootAdjustmentInstanceData = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const VPhysics2ShapeDef_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmFloatCurveNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CAnimationGraphVisualizerAxis = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CTwistConstraint = struct {
            };
            // Parent: None
            // Field count: 48
            pub const ModelBoneFlexDriverControl_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CParticleCollectionFloatInput = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CSeqCmdLayer = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const FootPinningPoseOpFixedData_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const RenderSkeletonBone_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CDistanceRemainingMetricEvaluator = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CDirectPlaybackUpdateNode = struct {
            };
            // Parent: MGetKV3ClassDefaults
            // Field count: 0
            pub const CNmIDEvent = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CFlexRule = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const ScriptInfo_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CBoneConstraintRbf = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CNetworkedCycle = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmSoundEvent = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CRenderGroom = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmParameterizedClipSelectorNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CTiltTwistConstraint = struct {
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmSpeedScaleNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CVPhysXSurfacePropertiesList = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmFloatSpringNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const RenderHairStrandInfo_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CCycleControlClipUpdateNode = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmTargetPointNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmTargetSelectorNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmFloatValueNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CWarpSectionAnimTagBase = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CNmGraphVariationUserData = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CStaticPoseCache = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 54
            pub const LookAtOpFixedSettings_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CRootMotion = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CAnimGraphSettingsManager = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CAnimGraphModelBinding = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: modellib
            // Field count: 0
            pub const CBoneConstraintPoseSpaceBone = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmTwoBoneIKTask = struct {
            };
            // Parent: None
            // Field count: 48
            pub const MotionBlendItem = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CBoneConstraintPoseSpaceMorph = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CFloatAnimParameter = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CStateActionUpdater = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CPlayerInputAnimMotorUpdater = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CRenderSkeleton = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CSeqIKLock = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CAnimParamHandleMap = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmFloatClampNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmExternalPoseNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 54
            pub const CAnimDesc = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmRootMotionOverrideNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmTargetWarpNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CPhysSurfacePropertiesPhysics = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CAnimationGraphVisualizerPie = struct {
            };
            // Parent: None
            // Field count: 48
            pub const NmFloatCurveCompressionSettings_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CBoneConstraintDotToMorph = struct {
            };
            // Parent: modellib
            // Field count: 0
            pub const CPointConstraint = struct {
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CCurrentRotationVelocityMetricEvaluator = struct {
            };
            // Parent: None
            // Field count: 48
            pub const TraceSettings_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const ParamSpanSample_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmFollowBoneTask = struct {
            };
            // Parent: None
            // Field count: 48
            pub const VPhysXConstraintParams_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmFloatComparisonNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmChainSolverTask = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CStateUpdateData = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CTurnHelperInstanceData = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmTargetValueNode__CDefinition = struct {
            };
            // Parent: modellib
            // Field count: 0
            pub const COrientConstraint = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CCycleClipInstanceData = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CMoverUpdateNode = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmLayerBlendNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CAnimMotorUpdaterBase = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const JiggleBoneSettingsList_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CSubtractUpdateNode = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmFloatRemapNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 56
            pub const CPoseHandle = struct {
                pub const @"": usize = 0x1FFEE; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x10000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x2C0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0xE02F; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5C0061; // 
                pub const @"": usize = 0x77006F; // 
                pub const @"": usize = 0x660074; // 
                pub const @"": usize = 0x36002E; // 
                pub const @"": usize = 0x5C003A; // 
                pub const @"": usize = 0x720063; // 
                pub const @"": usize = 0x650044; // 
                pub const @"": usize = 0x5C006D; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x690066; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x64006E; // 
                pub const @"": usize = 0x61006C; // 
                pub const @"": usize = 0x300032; // 
                pub const @"": usize = 0x6E; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xB6400000; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x690044; // 
                pub const @"": usize = 0x65006E; // 
                pub const @"": usize = 0xAA8E0000; // 
                pub const @"": usize = 0xEE0000; // 
                pub const @"": usize = 0x800000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D; // 
                pub const @"": usize = 0x20000; // 
                pub const @"": usize = 0xFFFF; // 
                pub const @"": usize = 0x150500; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x1; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmFootstepEventIDNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CSolveIKChainUpdateNode = struct {
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmOrNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CSeqCmdSeqDesc = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CBoneMaskUpdateNode = struct {
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CLeafUpdateNode = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CParticleFloatInput = struct {
            };
            // Parent: None
            // Field count: 48
            pub const SolveIKChainPoseOpFixedSettings_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmEntityAttributeFloatEvent = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmIDSelectorNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 54
            pub const TwoBoneIKSettings_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmCachedPoseWriteTask = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CAnimUpdateSharedData = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 54
            pub const CAnimAttachment = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CFootTrajectories = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmIDComparisonNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const PairedSequence_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CEditableMotionGraph = struct {
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmConstFloatNode__CDefinition = struct {
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CStringAnimTag = struct {
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CBoolAnimParameter = struct {
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CRagdollAnimTag = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CMotionNode = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmStateMachineNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CBodyGroupAnimTag = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmModelSpaceBlendTask = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmZeroPoseNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmOrientationWarpNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const AnimParamID = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CNmBoneMaskNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const ModelAnimGraph2Ref_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const AnimationDecodeDebugDump_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmVirtualParameterIDNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CSeqScaleSet = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CDirectionalBlendUpdateNode = struct {
            };
            // Parent: None
            // Field count: 56
            pub const CNmGraphDefinition__ReferencedGraphSlot_t = struct {
                pub const @"": usize = 0x1FFEE; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x10000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF000000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x2C0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0xE02F; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5C0061; // 
                pub const @"": usize = 0x77006F; // 
                pub const @"": usize = 0x660074; // 
                pub const @"": usize = 0x36002E; // 
                pub const @"": usize = 0x5C003A; // 
                pub const @"": usize = 0x720063; // 
                pub const @"": usize = 0x650044; // 
                pub const @"": usize = 0x5C006D; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x690066; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x64006E; // 
                pub const @"": usize = 0x61006C; // 
                pub const @"": usize = 0x300032; // 
                pub const @"": usize = 0x6E; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xB6400000; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x690044; // 
                pub const @"": usize = 0x65006E; // 
                pub const @"": usize = 0xAA8E0000; // 
                pub const @"": usize = 0xEE0000; // 
                pub const @"": usize = 0x800000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D; // 
                pub const @"": usize = 0x20000; // 
                pub const @"": usize = 0xFFFF; // 
                pub const @"": usize = 0x150500; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x1; // 
            };
            // Parent: None
            // Field count: 48
            pub const ParamSpan_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CFootStepTriggerUpdateNode = struct {
            };
            // Parent: None
            // Field count: 45
            pub const IKTargetSettings_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: modellib
            // Field count: 0
            pub const CModelConfigElement_SetRenderColor = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmAdditiveBlendTask = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmAnimationPoseNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CMotionSearchDB = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CMotionNodeBlend1D = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmCurrentSyncEventNode__CDefinition = struct {
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmControlParameterTargetNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const SkeletonBoneBounds_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CTwoBoneIKUpdateNode = struct {
            };
            // Parent: None
            // Field count: 48
            pub const VPhysXBodyPart_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CTaskHandshakeAnimTag = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CFollowPathUpdateNode = struct {
            };
            // Parent: None
            // Field count: 54
            pub const AimMatrixOpFixedSettings_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CAnimScriptComponentUpdater = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CRagdollUpdateNode = struct {
            };
            // Parent: None
            // Field count: 45
            pub const CBoneConstraintBase = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: animgraphlib
            // Field count: 0
            pub const CPathAnimMotorUpdater = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CMaterialAttributeAnimTag = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmFootEventConditionNode__CDefinition = struct {
            };
            // Parent: _
            // Field count: 0
            pub const CNmFootEvent = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CPhysSurfacePropertiesAudio = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CSeqAutoLayer = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const ModelEmbeddedMesh_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const PhysSoftbodyDesc_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CAnimSequenceParams = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 0
            pub const CTargetWarpUpdateNode = struct {
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmVirtualParameterBoolNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CFollowTargetUpdateNode = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CCPPScriptComponentUpdater = struct {
            };
            // Parent: None
            // Field count: 48
            pub const NetVarConfigIndex = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CAnimData = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 45
            pub const IKSolverSettings_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CBinaryUpdateNode = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CBlendNodeInstanceData = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CNmParameterizedBlendNode__BlendRange_t = struct {
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xFF00; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x70004D; // 
                pub const @"": usize = 0x670069; // 
                pub const @"": usize = 0x610074; // 
                pub const @"": usize = 0x6F0064; // 
                pub const @"": usize = 0x740061; // 
                pub const @"": usize = 0x2E0030; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xE02FB640; // 
                pub const @"": usize = 0x700073; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x730069; // 
                pub const @"": usize = 0x720065; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x80; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x520002; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x15; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CAnimKeyData = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 45
            pub const AttachmentHandle_t = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
            // Parent: None
            // Field count: 48
            pub const CAnimMorphDifference = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 54
            pub const FootFixedData_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: None
            // Field count: 48
            pub const CMotionGraphGroup = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmVectorNegateNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CNmSyncTrack__Event_t = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CParticleCollectionRendererVecInput = struct {
            };
            // Parent: None
            // Field count: 54
            pub const CNmTarget = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10740; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100000; // 
                pub const @"": usize = 0x88; // 
                pub const @"�": usize = 0x1F; // �
                pub const @"": usize = 0x10018; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x102C0; // P
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x8B7F8EA0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"P": usize = 0x4; // P
                pub const @"": usize = 0xE02FA840; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6D0061; // 
                pub const @"": usize = 0x57005C; // 
                pub const @"": usize = 0x5C0072; // 
                pub const @"": usize = 0x360032; // 
                pub const @"": usize = 0x6C0064; // 
                pub const @"": usize = 0x50005C; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x660065; // 
                pub const @"": usize = 0x34005C; // 
                pub const @"": usize = 0x360000; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x96; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x10150; // 
                pub const @"P": usize = 0x310038; // P
                pub const @"": usize = 0x4E002E; // 
                pub const @"": usize = 0x55002D; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20004D; // 
                pub const @"": usize = 0x10150; // 
                pub const @"": usize = 0x640064; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x20009; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5467122E; // 
                pub const @"": usize = 0x10150; // 
                pub const @"�": usize = 0x10060; // �
                pub const @"": usize = 0x0; // 
            };
            // Parent: animlib
            // Field count: 0
            pub const CNmControlParameterIDNode__CDefinition = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CNmBodyGroupEvent = struct {
            };
            // Parent: None
            // Field count: 0
            pub const CWayPointHelperUpdateNode = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CMoodVData = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 0
            pub const CModelConfigElement_AttachedModel = struct {
            };
            // Parent: None
            // Field count: 48
            pub const CAnimTagManagerUpdater = struct {
                pub const @"": usize = 0x10120; // 
                pub const @"": usize = 0x10; // 
                pub const @"": usize = 0x0; // 
                pub const @"`": usize = 0x0; // 
                pub const @"": usize = 0xEEFFEEFF; // 
                pub const @"": usize = 0x1000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x10FD0; // �
                pub const @"": usize = 0x10110; // 
                pub const @"": usize = 0x10750; // P
                pub const @"": usize = 0x1FE000; // 
                pub const @"": usize = 0x1; // 
                pub const @"P": usize = 0x10330; // 
                pub const @"": usize = 0x10150; // P
                pub const @"": usize = 0xE02FA810; // 
                pub const @"": usize = 0x630069; // 
                pub const @"": usize = 0x440020; // 
                pub const @"": usize = 0x6D0072; // 
                pub const @"": usize = 0x5C0030; // 
                pub const @"": usize = 0x6F0072; // 
                pub const @"": usize = 0x6F0073; // 
                pub const @"": usize = 0x6E0065; // 
                pub const @"": usize = 0x31002E; // 
                pub const @"": usize = 0x61004F; // 
                pub const @"": usize = 0x6C0000; // 
                pub const @"": usize = 0x4D005C; // 
                pub const @"": usize = 0x730077; // 
                pub const @"": usize = 0x6F0066; // 
                pub const @"": usize = 0x2D0036; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x5D00005D; // 
                pub const @"P": usize = 0x10150; // P
                pub const @"": usize = 0x6C0070; // 
                pub const @"": usize = 0x5C; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0xEE; // 
                pub const @"": usize = 0x3; // 
                pub const @"": usize = 0x790079; // 
                pub const @"": usize = 0x70; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x99A46D76; // 
                pub const @"": usize = 0x1D00001D; // 
                pub const @"": usize = 0x100F0; // �
                pub const @"�": usize = 0xF000; // 
            };
            // Parent: None
            // Field count: 45
            pub const CParticleProperty = struct {
                pub const @"": usize = 0x10110FF; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x45; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x400; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1032000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x61007400; // 
                pub const @"": usize = 0x6F006400; // 
                pub const @"": usize = 0x74006100; // 
                pub const @"": usize = 0x2E003000; // 
                pub const @"": usize = 0x3A004300; // 
                pub const @"": usize = 0x63006900; // 
                pub const @"": usize = 0x44002000; // 
                pub const @"": usize = 0x6D007200; // 
                pub const @"": usize = 0x5C003000; // 
                pub const @"": usize = 0x66006E00; // 
                pub const @"": usize = 0x61004400; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x6C005000; // 
                pub const @"": usize = 0x32003000; // 
                pub const @"": usize = 0x6E006500; // 
                pub const @"": usize = 0x7FF98B; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x1; // 
                pub const @"": usize = 0x69006400; // 
                pub const @"": usize = 0x1015000; // 
                pub const @"": usize = 0x44005C00; // 
                pub const @"": usize = 0x6E006900; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x7700; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x4D004D00; // 
                pub const @"": usize = 0x600; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x0; // 
                pub const @"": usize = 0x100F003; // 
                pub const @"": usize = 0x1100000; // 
            };
        };
    };
};
