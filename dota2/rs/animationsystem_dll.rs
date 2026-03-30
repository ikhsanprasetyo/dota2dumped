// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-03-30 13:31:39.632920200 +07:00

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]

pub mod source2_dumper {
    pub mod schemas {
        // Module: animationsystem.dll
        // Class count: 663
        // Enum count: 141
        pub mod animationsystem_dll {
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum PulseBestOutflowRules_t {
                SORT_BY_NUMBER_OF_VALID_CRITERIA = 0x0,
                SORT_BY_OUTFLOW_INDEX = 0x1
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum PulseCursorCancelPriority_t {
                None = 0x0,
                CancelOnSucceeded = 0x1,
                SoftCancel = 0x2,
                HardCancel = 0x3
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum PulseMethodCallMode_t {
                SYNC_WAIT_FOR_COMPLETION = 0x0,
                ASYNC_FIRE_AND_FORGET = 0x1
            }
            // Alignment: 4
            // Member count: 6
            #[repr(u32)]
            pub enum ParticleSetMethod_t {
                PARTICLE_SET_REPLACE_VALUE = 0x0,
                PARTICLE_SET_SCALE_INITIAL_VALUE = 0x1,
                PARTICLE_SET_ADD_TO_INITIAL_VALUE = 0x2,
                PARTICLE_SET_RAMP_CURRENT_VALUE = 0x3,
                PARTICLE_SET_SCALE_CURRENT_VALUE = 0x4,
                PARTICLE_SET_ADD_TO_CURRENT_VALUE = 0x5
            }
            // Alignment: 4
            // Member count: 17
            #[repr(u32)]
            pub enum SeqCmd_t {
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
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum CNmEventRelevance_t {
                ClientOnly = 0x0,
                ServerOnly = 0x1,
                ClientAndServer = 0x2
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum BoneTransformSpace_t {
                BoneTransformSpace_Invalid = u32::MAX,
                BoneTransformSpace_Parent = 0x0,
                BoneTransformSpace_Model = 0x1,
                BoneTransformSpace_World = 0x2
            }
            // Alignment: 4
            // Member count: 5
            #[repr(u32)]
            pub enum CAnimationGraphVisualizerPrimitiveType {
                ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Text = 0x0,
                ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Sphere = 0x1,
                ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Line = 0x2,
                ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Pie = 0x3,
                ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Axis = 0x4
            }
            // Alignment: 1
            // Member count: 3
            #[repr(u8)]
            pub enum NmTransitionRule_t {
                AllowTransition = 0x0,
                ConditionallyAllowTransition = 0x1,
                BlockTransition = 0x2
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum BinaryNodeTiming {
                UseChild1 = 0x0,
                UseChild2 = 0x1,
                SyncChildren = 0x2
            }
            // Alignment: 1
            // Member count: 3
            #[repr(u8)]
            pub enum NmFollowBoneMode_t {
                RotationAndTranslation = 0x0,
                RotationOnly = 0x1,
                TranslationOnly = 0x2
            }
            // Alignment: 4
            // Member count: 7
            #[repr(u32)]
            pub enum SolveIKChainAnimNodeDebugSetting {
                SOLVEIKCHAINANIMNODEDEBUGSETTING_None = 0x0,
                SOLVEIKCHAINANIMNODEDEBUGSETTING_X_Axis_Circle = 0x1,
                SOLVEIKCHAINANIMNODEDEBUGSETTING_Y_Axis_Circle = 0x2,
                SOLVEIKCHAINANIMNODEDEBUGSETTING_Z_Axis_Circle = 0x3,
                SOLVEIKCHAINANIMNODEDEBUGSETTING_Forward = 0x4,
                SOLVEIKCHAINANIMNODEDEBUGSETTING_Up = 0x5,
                SOLVEIKCHAINANIMNODEDEBUGSETTING_Left = 0x6
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum CNmParticleEvent__Type_t {
                Create = 0x0,
                Create_CFG = 0x1
            }
            // Alignment: 4
            // Member count: 5
            #[repr(u32)]
            pub enum ParticleFloatBiasType_t {
                PF_BIAS_TYPE_INVALID = u32::MAX,
                PF_BIAS_TYPE_STANDARD = 0x0,
                PF_BIAS_TYPE_GAIN = 0x1,
                PF_BIAS_TYPE_EXPONENTIAL = 0x2,
                PF_BIAS_TYPE_COUNT = 0x3
            }
            // Alignment: 1
            // Member count: 4
            #[repr(u8)]
            pub enum CNmTargetWarpNode__TargetUpdateRule_t {
                None = 0x0,
                Recalculate = 0x1,
                Offset = 0x2,
                RecalculateOrOffset = 0x3
            }
            // Alignment: 1
            // Member count: 6
            #[repr(u8)]
            pub enum SharedMovementGait_t {
                eInvalid = u8::MAX,
                eSlow = 0x0,
                eMedium = 0x1,
                eFast = 0x2,
                eVeryFast = 0x3,
                eCount = 0x4
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum OrientationWarpRootMotionSource_t {
                eAnimationOrProcedural = 0x0,
                eAnimationOnly = 0x1,
                eProceduralOnly = 0x2
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum IKTargetCoordinateSystem {
                IKTARGETCOORDINATESYSTEM_WorldSpace = 0x0,
                IKTARGETCOORDINATESYSTEM_ModelSpace = 0x1,
                IKTARGETCOORDINATESYSTEM_COUNT = 0x2
            }
            // Alignment: 4
            // Member count: 33
            #[repr(u32)]
            pub enum ParticleFloatType_t {
                PF_TYPE_INVALID = u32::MAX,
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
            }
            // Alignment: 1
            // Member count: 4
            #[repr(u8)]
            pub enum CNmFloatAngleMathNode__Operation_t {
                ClampTo180 = 0x0,
                ClampTo360 = 0x1,
                FlipHemisphere = 0x2,
                FlipHemisphereNegate = 0x3
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum VPhysXAggregateData_t__VPhysXFlagEnum_t {
                FLAG_IS_POLYSOUP_GEOMETRY = 0x1,
                FLAG_LEVEL_COLLISION = 0x10,
                FLAG_IGNORE_SCALE_OBSOLETE_DO_NOT_USE = 0x20
            }
            // Alignment: 1
            // Member count: 5
            #[repr(u8)]
            pub enum CNmRootMotionOverrideNode__OverrideFlags_t {
                AllowMoveX = 0x0,
                AllowMoveY = 0x1,
                AllowMoveZ = 0x2,
                AllowFacingPitch = 0x3,
                ListenForEvents = 0x4
            }
            // Alignment: 1
            // Member count: 23
            #[repr(u8)]
            pub enum NmEasingOperation_t {
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
            }
            // Alignment: 4
            // Member count: 5
            #[repr(u32)]
            pub enum EIKEndEffectorRotationFixUpMode {
                None = 0x0,
                MatchTargetOrientation = 0x1,
                LookAtTargetForward = 0x2,
                MaintainParentOrientation = 0x3,
                Count = 0x4
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum MatterialAttributeTagType_t {
                MATERIAL_ATTRIBUTE_TAG_VALUE = 0x0,
                MATERIAL_ATTRIBUTE_TAG_COLOR = 0x1
            }
            // Alignment: 4
            // Member count: 6
            #[repr(u32)]
            pub enum PFNoiseTurbulence_t {
                PF_NOISE_TURB_NONE = 0x0,
                PF_NOISE_TURB_HIGHLIGHT = 0x1,
                PF_NOISE_TURB_FEEDBACK = 0x2,
                PF_NOISE_TURB_LOOPY = 0x3,
                PF_NOISE_TURB_CONTRAST = 0x4,
                PF_NOISE_TURB_ALTERNATE = 0x5
            }
            // Alignment: 1
            // Member count: 4
            #[repr(u8)]
            pub enum NmTargetWarpAlgorithm_t {
                Lerp = 0x0,
                Hermite = 0x1,
                HermiteFeaturePreserving = 0x2,
                Bezier = 0x3
            }
            // Alignment: 4
            // Member count: 5
            #[repr(u32)]
            pub enum ParticleColorBlendMode_t {
                PARTICLEBLEND_DEFAULT = 0x0,
                PARTICLEBLEND_OVERLAY = 0x1,
                PARTICLEBLEND_DARKEN = 0x2,
                PARTICLEBLEND_LIGHTEN = 0x3,
                PARTICLEBLEND_MULTIPLY = 0x4
            }
            // Alignment: 4
            // Member count: 13
            #[repr(u32)]
            pub enum ParticleColorBlendType_t {
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
            }
            // Alignment: 1
            // Member count: 4
            #[repr(u8)]
            pub enum NmTransitionRuleCondition_t {
                AnyAllowed = 0x0,
                FullyAllowed = 0x1,
                ConditionallyAllowed = 0x2,
                Blocked = 0x3
            }
            // Alignment: 1
            // Member count: 7
            #[repr(u8)]
            pub enum ModelMeshBufferUsage_t {
                MESH_BUFFER_USAGE_NONE = 0x0,
                MESH_BUFFER_USAGE_VB = 0x1,
                MESH_BUFFER_USAGE_IB = 0x2,
                MESH_BUFFER_USAGE_ADJACENCY = 0x4,
                MESH_BUFFER_USAGE_MESHLET_TRIS = 0x8,
                MESH_BUFFER_USAGE_RT_PROXY = 0x10,
                MESH_BUFFER_USAGE_VERTEX_ALBEDO = 0x20
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum NmGraphDebugMode_t {
                Off = 0x0,
                On = 0x1
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum TargetWarpTimingMethod {
                ReachDestinationOnRootMotionEnd = 0x0,
                ReachDestinationOnWarpTagEnd = 0x1
            }
            // Alignment: 4
            // Member count: 6
            #[repr(u32)]
            pub enum ScriptedMoveTo_t {
                eWait = 0x0,
                eMoveWithGait = 0x3,
                eTeleport = 0x4,
                eWaitFacing = 0x5,
                eObsoleteBackCompat1 = 0x1,
                eObsoleteBackCompat2 = 0x2
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum EDemoBoneSelectionMode {
                CaptureAllBones = 0x0,
                CaptureSelectedBones = 0x1
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum StepPhase {
                StepPhase_OnGround = 0x0,
                StepPhase_InAir = 0x1
            }
            // Alignment: 4
            // Member count: 26
            #[repr(u32)]
            pub enum FlexOpCode_t {
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
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum NmCachedValueMode_t {
                OnEntry = 0x0,
                OnExit = 0x1
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum AnimNodeNetworkMode {
                ServerAuthoritative = 0x0,
                ClientSimulate = 0x1
            }
            // Alignment: 4
            // Member count: 6
            #[repr(u32)]
            pub enum VPhysXBodyPart_t__VPhysXFlagEnum_t {
                FLAG_STATIC = 0x1,
                FLAG_KINEMATIC = 0x2,
                FLAG_JOINT = 0x4,
                FLAG_MASS = 0x8,
                FLAG_ALWAYS_DYNAMIC_ON_CLIENT = 0x10,
                FLAG_DISABLE_CCD = 0x20
            }
            // Alignment: 1
            // Member count: 9
            #[repr(u8)]
            pub enum AnimParamType_t {
                ANIMPARAM_UNKNOWN = 0x0,
                ANIMPARAM_BOOL = 0x1,
                ANIMPARAM_ENUM = 0x2,
                ANIMPARAM_INT = 0x3,
                ANIMPARAM_FLOAT = 0x4,
                ANIMPARAM_VECTOR = 0x5,
                ANIMPARAM_QUATERNION = 0x6,
                ANIMPARAM_GLOBALSYMBOL = 0x7,
                ANIMPARAM_COUNT = 0x8
            }
            // Alignment: 1
            // Member count: 9
            #[repr(u8)]
            pub enum NmEasingFunction_t {
                Linear = 0x0,
                Quad = 0x1,
                Cubic = 0x2,
                Quart = 0x3,
                Quint = 0x4,
                Sine = 0x5,
                Expo = 0x6,
                Circ = 0x7,
                Back = 0x8
            }
            // Alignment: 4
            // Member count: 5
            #[repr(u32)]
            pub enum ParticleModelType_t {
                PM_TYPE_INVALID = 0x0,
                PM_TYPE_NAMED_VALUE_MODEL = 0x1,
                PM_TYPE_NAMED_VALUE_EHANDLE = 0x2,
                PM_TYPE_CONTROL_POINT = 0x3,
                PM_TYPE_COUNT = 0x4
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum IKTargetSource {
                IKTARGETSOURCE_Bone = 0x0,
                IKTARGETSOURCE_AnimgraphParameter = 0x1,
                IKTARGETSOURCE_COUNT = 0x2
            }
            // Alignment: 4
            // Member count: 15
            #[repr(u32)]
            pub enum PermModelInfo_t__FlagEnum {
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
            }
            // Alignment: 1
            // Member count: 12
            #[repr(u8)]
            pub enum CNmFloatMathNode__Operator_t {
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
            }
            // Alignment: 1
            // Member count: 2
            #[repr(u8)]
            pub enum CNmSyncEventIndexConditionNode__TriggerMode_t {
                ExactlyAtEventIndex = 0x0,
                GreaterThanEqualToEventIndex = 0x1
            }
            // Alignment: 4
            // Member count: 5
            #[repr(u32)]
            pub enum ParticleFloatRoundType_t {
                PF_ROUND_TYPE_INVALID = u32::MAX,
                PF_ROUND_TYPE_NEAREST = 0x0,
                PF_ROUND_TYPE_FLOOR = 0x1,
                PF_ROUND_TYPE_CEIL = 0x2,
                PF_ROUND_TYPE_COUNT = 0x3
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum PFNoiseType_t {
                PF_NOISE_TYPE_PERLIN = 0x0,
                PF_NOISE_TYPE_SIMPLEX = 0x1,
                PF_NOISE_TYPE_WORLEY = 0x2,
                PF_NOISE_TYPE_CURL = 0x3
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum ParticleDirectionNoiseType_t {
                PARTICLE_DIR_NOISE_PERLIN = 0x0,
                PARTICLE_DIR_NOISE_CURL = 0x1,
                PARTICLE_DIR_NOISE_WORLEY_BASIC = 0x2
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum AnimParamNetworkSetting {
                Auto = 0x0,
                AlwaysNetwork = 0x1,
                NeverNetwork = 0x2
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum MorphFlexControllerRemapType_t {
                MORPH_FLEXCONTROLLER_REMAP_PASSTHRU = 0x0,
                MORPH_FLEXCONTROLLER_REMAP_2WAY = 0x1,
                MORPH_FLEXCONTROLLER_REMAP_NWAY = 0x2,
                MORPH_FLEXCONTROLLER_REMAP_EYELID = 0x3
            }
            // Alignment: 4
            // Member count: 8
            #[repr(u32)]
            pub enum MeshDrawPrimitiveFlags_t {
                MESH_DRAW_FLAGS_NONE = 0x0,
                MESH_DRAW_FLAGS_USE_SHADOW_FAST_PATH = 0x1,
                MESH_DRAW_FLAGS_USE_COMPRESSED_NORMAL_TANGENT = 0x2,
                MESH_DRAW_INPUT_LAYOUT_IS_NOT_MATCHED_TO_MATERIAL = 0x8,
                MESH_DRAW_FLAGS_USE_COMPRESSED_PER_VERTEX_LIGHTING = 0x10,
                MESH_DRAW_FLAGS_USE_UNCOMPRESSED_PER_VERTEX_LIGHTING = 0x20,
                MESH_DRAW_FLAGS_CAN_BATCH_WITH_DYNAMIC_SHADER_CONSTANTS = 0x40,
                MESH_DRAW_FLAGS_DRAW_LAST = 0x80
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum TargetWarpAngleMode_t {
                eFacingHeading = 0x0,
                eMoveHeading = 0x1
            }
            // Alignment: 1
            // Member count: 2
            #[repr(u8)]
            pub enum NmIKBlendMode_t {
                Effector = 0x0,
                Pose = 0x1
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum ModelBoneFlexComponent_t {
                MODEL_BONE_FLEX_INVALID = u32::MAX,
                MODEL_BONE_FLEX_TX = 0x0,
                MODEL_BONE_FLEX_TY = 0x1,
                MODEL_BONE_FLEX_TZ = 0x2
            }
            // Alignment: 1
            // Member count: 2
            #[repr(u8)]
            pub enum CNmStateNode__TimedEvent_t__Comparison_t {
                LessThanEqual = 0x0,
                GreaterThanEqual = 0x1
            }
            // Alignment: 1
            // Member count: 3
            #[repr(u8)]
            pub enum PoseType_t {
                POSETYPE_STATIC = 0x0,
                POSETYPE_DYNAMIC = 0x1,
                POSETYPE_INVALID = 0xFF
            }
            // Alignment: 1
            // Member count: 2
            #[repr(u8)]
            pub enum CNmRootMotionData__SamplingMode_t {
                Delta = 0x0,
                WorldSpace = 0x1
            }
            // Alignment: 1
            // Member count: 9
            #[repr(u8)]
            pub enum NmEventConditionRules_t {
                LimitSearchToSourceState = 0x0,
                IgnoreInactiveEvents = 0x1,
                PreferHighestWeight = 0x2,
                PreferHighestProgress = 0x3,
                OperatorOr = 0x4,
                OperatorAnd = 0x5,
                SearchOnlyGraphEvents = 0x6,
                SearchOnlyAnimEvents = 0x7,
                SearchBothGraphAndAnimEvents = 0x8
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum AnimationType_t {
                ANIMATION_TYPE_FIXED_RATE = 0x0,
                ANIMATION_TYPE_FIT_LIFETIME = 0x1,
                ANIMATION_TYPE_MANUAL_FRAMES = 0x2
            }
            // Alignment: 4
            // Member count: 37
            #[repr(u32)]
            pub enum AnimValueSource {
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
            }
            // Alignment: 1
            // Member count: 4
            #[repr(u8)]
            pub enum CNmTimeConditionNode__Operator_t {
                LessThan = 0x0,
                LessThanEqual = 0x1,
                GreaterThan = 0x2,
                GreaterThanEqual = 0x3
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum LinearRootMotionBlendMode_t {
                LERP = 0x0,
                NLERP = 0x1,
                SLERP = 0x2
            }
            // Alignment: 4
            // Member count: 1
            #[repr(u32)]
            pub enum RagdollPoseControl {
                Absolute = 0x0
            }
            // Alignment: 4
            // Member count: 6
            #[repr(u32)]
            pub enum IKSolverType {
                IKSOLVER_Perlin = 0x0,
                IKSOLVER_TwoBone = 0x1,
                IKSOLVER_Fabrik = 0x2,
                IKSOLVER_DogLeg3Bone = 0x3,
                IKSOLVER_CCD = 0x4,
                IKSOLVER_COUNT = 0x5
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum TargetWarpCorrectionMethod {
                ScaleMotion = 0x0,
                AddCorrectionDelta = 0x1
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum TargetSelectorAngleMode_t {
                eFacingHeading = 0x0,
                eMoveHeading = 0x1
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum Blend2DMode {
                Blend2DMode_General = 0x0,
                Blend2DMode_Directional = 0x1
            }
            // Alignment: 1
            // Member count: 3
            #[repr(u8)]
            pub enum HandshakeTagState_t {
                eInactive = 0x0,
                eActive = 0x1,
                eMomentarilyInactive = 0x2
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum ChoiceChangeMethod {
                OnReset = 0x0,
                OnCycleEnd = 0x1,
                OnResetOrCycleEnd = 0x2
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum ChoiceBlendMethod {
                SingleBlendTime = 0x0,
                PerChoiceBlendTimes = 0x1
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum VPhysXConstraintParams_t__EnumFlags0_t {
                FLAG0_SHIFT_INTERPENETRATE = 0x0,
                FLAG0_SHIFT_CONSTRAIN = 0x1,
                FLAG0_SHIFT_BREAKABLE_FORCE = 0x2,
                FLAG0_SHIFT_BREAKABLE_TORQUE = 0x3
            }
            // Alignment: 4
            // Member count: 9
            #[repr(u32)]
            pub enum ParticleFloatMapType_t {
                PF_MAP_TYPE_INVALID = u32::MAX,
                PF_MAP_TYPE_DIRECT = 0x0,
                PF_MAP_TYPE_MULT = 0x1,
                PF_MAP_TYPE_REMAP = 0x2,
                PF_MAP_TYPE_REMAP_BIASED = 0x3,
                PF_MAP_TYPE_CURVE = 0x4,
                PF_MAP_TYPE_NOTCHED = 0x5,
                PF_MAP_TYPE_ROUND = 0x6,
                PF_MAP_TYPE_COUNT = 0x7
            }
            // Alignment: 4
            // Member count: 5
            #[repr(u32)]
            pub enum AnimParamVectorType_t {
                ANIMPARAM_VECTOR_TYPE_NONE = 0x0,
                ANIMPARAM_VECTOR_TYPE_POSITION_WS = 0x1,
                ANIMPARAM_VECTOR_TYPE_POSITION_LS = 0x2,
                ANIMPARAM_VECTOR_TYPE_DIRECTION_WS = 0x3,
                ANIMPARAM_VECTOR_TYPE_DIRECTION_LS = 0x4
            }
            // Alignment: 1
            // Member count: 3
            #[repr(u8)]
            pub enum CNmCurrentSyncEventNode__InfoType_t {
                IndexAndPercentage = 0x0,
                IndexOnly = 0x1,
                PercentageOnly = 0x2
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum BlendKeyType {
                BlendKey_UserValue = 0x0,
                BlendKey_Velocity = 0x1,
                BlendKey_Distance = 0x2,
                BlendKey_RemainingDistance = 0x3
            }
            // Alignment: 4
            // Member count: 5
            #[repr(u32)]
            pub enum StateActionBehavior {
                STATETAGBEHAVIOR_ACTIVE_WHILE_CURRENT = 0x0,
                STATETAGBEHAVIOR_FIRE_ON_ENTER = 0x1,
                STATETAGBEHAVIOR_FIRE_ON_EXIT = 0x2,
                STATETAGBEHAVIOR_FIRE_ON_ENTER_AND_EXIT = 0x3,
                STATETAGBEHAVIOR_ACTIVE_WHILE_FULLY_BLENDED = 0x4
            }
            // Alignment: 1
            // Member count: 4
            #[repr(u8)]
            pub enum NmRootMotionBlendMode_t {
                Blend = 0x0,
                Additive = 0x1,
                IgnoreSource = 0x2,
                IgnoreTarget = 0x3
            }
            // Alignment: 1
            // Member count: 7
            #[repr(u8)]
            pub enum NmFootPhaseCondition_t {
                LeftFootDown = 0x0,
                LeftFootPassing = 0x1,
                LeftPhase = 0x4,
                RightFootDown = 0x2,
                RightFootPassing = 0x3,
                RightPhase = 0x5,
                None = 0x6
            }
            // Alignment: 4
            // Member count: 22
            #[repr(u32)]
            pub enum ModelSkeletonData_t__BoneFlags_t {
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
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum GPUParticleCollisionMode_t {
                PARTICLE_GPU_COLLISION_MODE_RT = 0x0,
                PARTICLE_GPU_COLLISION_MODE_DEPTH = 0x1,
                PARTICLE_GPU_COLLISION_MODE_HYBRID = 0x2
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum MorphBundleType_t {
                MORPH_BUNDLE_TYPE_NONE = 0x0,
                MORPH_BUNDLE_TYPE_POSITION_SPEED = 0x1,
                MORPH_BUNDLE_TYPE_NORMAL_WRINKLE = 0x2,
                MORPH_BUNDLE_TYPE_COUNT = 0x3
            }
            // Alignment: 1
            // Member count: 2
            #[repr(u8)]
            pub enum CNmIDComparisonNode__Comparison_t {
                Matches = 0x0,
                DoesntMatch = 0x1
            }
            // Alignment: 1
            // Member count: 3
            #[repr(u8)]
            pub enum NmPoseBlendMode_t {
                Overlay = 0x0,
                Additive = 0x1,
                ModelSpace = 0x2
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum ParticleFloatInputMode_t {
                PF_INPUT_MODE_INVALID = u32::MAX,
                PF_INPUT_MODE_CLAMPED = 0x0,
                PF_INPUT_MODE_LOOPED = 0x1,
                PF_INPUT_MODE_COUNT = 0x2
            }
            // Alignment: 4
            // Member count: 5
            #[repr(u32)]
            pub enum ResetCycleOption {
                Beginning = 0x0,
                SameCycleAsSource = 0x1,
                InverseSourceCycle = 0x2,
                FixedValue = 0x3,
                SameTimeAsSource = 0x4
            }
            // Alignment: 1
            // Member count: 6
            #[repr(u8)]
            pub enum CNmVectorInfoNode__Info_t {
                X = 0x0,
                Y = 0x1,
                Z = 0x2,
                Length = 0x3,
                AngleHorizontal = 0x4,
                AngleVertical = 0x5
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum IKChannelMode {
                TwoBone = 0x0,
                TwoBone_Translate = 0x1,
                OneBone = 0x2,
                OneBone_Translate = 0x3
            }
            // Alignment: 1
            // Member count: 9
            #[repr(u8)]
            pub enum NmGraphValueType_t {
                Unknown = 0x0,
                Bool = 0x1,
                ID = 0x2,
                Float = 0x3,
                Vector = 0x4,
                Target = 0x5,
                BoneMask = 0x6,
                Pose = 0x7,
                Special = 0x8
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum ParticleFloatRandomMode_t {
                PF_RANDOM_MODE_INVALID = u32::MAX,
                PF_RANDOM_MODE_CONSTANT = 0x0,
                PF_RANDOM_MODE_VARYING = 0x1,
                PF_RANDOM_MODE_COUNT = 0x2
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum PFNoiseModifier_t {
                PF_NOISE_MODIFIER_NONE = 0x0,
                PF_NOISE_MODIFIER_LINES = 0x1,
                PF_NOISE_MODIFIER_CLUMPS = 0x2,
                PF_NOISE_MODIFIER_RINGS = 0x3
            }
            // Alignment: 4
            // Member count: 21
            #[repr(u32)]
            pub enum ParticleVecType_t {
                PVEC_TYPE_INVALID = u32::MAX,
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
            }
            // Alignment: 1
            // Member count: 5
            #[repr(u8)]
            pub enum NmFootPhase_t {
                LeftFootDown = 0x0,
                RightFootPassing = 0x1,
                RightFootDown = 0x2,
                LeftFootPassing = 0x3,
                None = 0x4
            }
            // Alignment: 4
            // Member count: 8
            #[repr(u32)]
            pub enum CNmTargetInfoNode__Info_t {
                AngleHorizontal = 0x0,
                AngleVertical = 0x1,
                Distance = 0x2,
                DistanceHorizontalOnly = 0x3,
                DistanceVerticalOnly = 0x4,
                DeltaOrientationX = 0x5,
                DeltaOrientationY = 0x6,
                DeltaOrientationZ = 0x7
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum FootstepLandedFootSoundType_t {
                FOOTSOUND_Left = 0x0,
                FOOTSOUND_Right = 0x1,
                FOOTSOUND_UseOverrideSound = 0x2
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum FootLockSubVisualization {
                FOOTLOCKSUBVISUALIZATION_ReachabilityAnalysis = 0x0,
                FOOTLOCKSUBVISUALIZATION_IKSolve = 0x1
            }
            // Alignment: 4
            // Member count: 5
            #[repr(u32)]
            pub enum CNmSoundEvent__Position_t {
                None = 0x0,
                World = 0x1,
                EntityPos = 0x2,
                EntityEyePos = 0x3,
                EntityAttachment = 0x4
            }
            // Alignment: 1
            // Member count: 4
            #[repr(u8)]
            pub enum FootstepJumpPhase_t {
                Unknown = 0x0,
                NotJumping = 0x1,
                Jumping = 0x2,
                Landing = 0x4
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum NmFrameSnapEventMode_t {
                Floor = 0x0,
                Round = 0x1
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum FootPinningTimingSource {
                FootMotion = 0x0,
                Tag = 0x1,
                Parameter = 0x2
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum DampingSpeedFunction {
                NoDamping = 0x0,
                Constant = 0x1,
                Spring = 0x2,
                AsymmetricSpring = 0x3
            }
            // Alignment: 4
            // Member count: 6
            #[repr(u32)]
            pub enum AnimationProcessingType_t {
                ANIMATION_PROCESSING_SERVER_SIMULATION = 0x0,
                ANIMATION_PROCESSING_CLIENT_SIMULATION = 0x1,
                ANIMATION_PROCESSING_CLIENT_PREDICTION = 0x2,
                ANIMATION_PROCESSING_CLIENT_INTERPOLATION = 0x3,
                ANIMATION_PROCESSING_CLIENT_RENDER = 0x4,
                ANIMATION_PROCESSING_MAX = 0x5
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum JiggleBoneSimSpace {
                SimSpace_Local = 0x0,
                SimSpace_Model = 0x1,
                SimSpace_World = 0x2
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum StanceOverrideMode {
                Sequence = 0x0,
                Node = 0x1
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum IkEndEffectorType {
                IkEndEffector_Attachment = 0x0,
                IkEndEffector_Bone = 0x1
            }
            // Alignment: 2
            // Member count: 3
            #[repr(u16)]
            pub enum AnimScriptType {
                ANIMSCRIPT_TYPE_INVALID = u16::MAX,
                ANIMSCRIPT_FUSE_GENERAL = 0x0,
                ANIMSCRIPT_FUSE_STATEMACHINE = 0x1
            }
            // Alignment: 1
            // Member count: 3
            #[repr(u8)]
            pub enum CNmTimeConditionNode__ComparisonType_t {
                PercentageThroughState = 0x0,
                PercentageThroughSyncEvent = 0x1,
                ElapsedTime = 0x2
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum SeqPoseSetting_t {
                SEQ_POSE_SETTING_CONSTANT = 0x0,
                SEQ_POSE_SETTING_ROTATION = 0x1,
                SEQ_POSE_SETTING_POSITION = 0x2,
                SEQ_POSE_SETTING_VELOCITY = 0x3
            }
            // Alignment: 4
            // Member count: 13
            #[repr(u32)]
            pub enum AnimParamButton_t {
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
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum SelectorTagBehavior_t {
                SelectorTagBehavior_OnWhileCurrent = 0x0,
                SelectorTagBehavior_OffWhenFinished = 0x1,
                SelectorTagBehavior_OffBeforeFinished = 0x2
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum HandshakeTagType_t {
                eInvalid = u32::MAX,
                eTask = 0x0,
                eMovement = 0x1,
                eCount = 0x2
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum OrientationWarpTargetOffsetMode_t {
                eLiteralValue = 0x0,
                eParameter = 0x1,
                eAnimationMovementHeading = 0x2,
                eAnimationMovementHeadingAtEnd = 0x3
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum OrientationWarpMode_t {
                eInvalid = 0x0,
                eAngle = 0x1,
                eWorldPosition = 0x2
            }
            // Alignment: 4
            // Member count: 5
            #[repr(u32)]
            pub enum ParticleTransformType_t {
                PT_TYPE_INVALID = 0x0,
                PT_TYPE_NAMED_VALUE = 0x1,
                PT_TYPE_CONTROL_POINT = 0x2,
                PT_TYPE_CONTROL_POINT_RANGE = 0x3,
                PT_TYPE_COUNT = 0x4
            }
            // Alignment: 4
            // Member count: 18
            #[repr(u32)]
            pub enum ParticleAttachment_t {
                PATTACH_INVALID = u32::MAX,
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
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum CNmEventTargetEntity_t {
                Self = 0x0,
                Weapon = 0x1,
                HeldItem = 0x2,
                Custom = 0x3
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum FieldNetworkOption {
                Auto = 0x0,
                ForceEnable = 0x1,
                ForceDisable = 0x2
            }
            // Alignment: 1
            // Member count: 6
            #[repr(u8)]
            pub enum NmGraphEventTypeCondition_t {
                Entry = 0x0,
                FullyInState = 0x1,
                Exit = 0x2,
                Timed = 0x3,
                Generic = 0x4,
                Any = 0x5
            }
            // Alignment: 1
            // Member count: 10
            #[repr(u8)]
            pub enum CNmTransitionNode__TransitionOptions_t {
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
            }
            // Alignment: 1
            // Member count: 5
            #[repr(u8)]
            pub enum CNmFloatComparisonNode__Comparison_t {
                GreaterThanEqual = 0x0,
                LessThanEqual = 0x1,
                NearEqual = 0x2,
                GreaterThan = 0x3,
                LessThan = 0x4
            }
            // Alignment: 4
            // Member count: 3
            #[repr(u32)]
            pub enum VPhysXJoint_t__Flags_t {
                JOINT_FLAGS_NONE = 0x0,
                JOINT_FLAGS_BODY1_FIXED = 0x1,
                JOINT_FLAGS_USE_BLOCK_SOLVER = 0x2
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum ScriptedHeldWeaponBehavior_t {
                eInvalid = u32::MAX,
                eHolster = 0x0,
                eDeploy = 0x1,
                eDrop = 0x2
            }
            // Alignment: 1
            // Member count: 3
            #[repr(u8)]
            pub enum VelocityMetricMode {
                DirectionOnly = 0x0,
                MagnitudeOnly = 0x1,
                DirectionAndMagnitude = 0x2
            }
            // Alignment: 1
            // Member count: 5
            #[repr(u8)]
            pub enum FacingMode {
                FacingMode_Invalid = 0x0,
                FacingMode_Manual = 0x1,
                FacingMode_Path = 0x2,
                FacingMode_LookTarget = 0x3,
                FacingMode_ManualPosition = 0x4
            }
            // Alignment: 1
            // Member count: 3
            #[repr(u8)]
            pub enum VertexAlbedoFormat_t {
                VERTEX_ALBEDO_NONE = 0x0,
                VERTEX_ALBEDO_8888 = 0x1,
                VERTEX_ALBEDO_565 = 0x2
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum AimMatrixBlendMode {
                AimMatrixBlendMode_None = 0x0,
                AimMatrixBlendMode_Additive = 0x1,
                AimMatrixBlendMode_ModelSpaceAdditive = 0x2,
                AimMatrixBlendMode_BoneMask = 0x3
            }
            // Alignment: 4
            // Member count: 7
            #[repr(u32)]
            pub enum AnimationSnapshotType_t {
                ANIMATION_SNAPSHOT_SERVER_SIMULATION = 0x0,
                ANIMATION_SNAPSHOT_CLIENT_SIMULATION = 0x1,
                ANIMATION_SNAPSHOT_CLIENT_PREDICTION = 0x2,
                ANIMATION_SNAPSHOT_CLIENT_INTERPOLATION = 0x3,
                ANIMATION_SNAPSHOT_CLIENT_RENDER = 0x4,
                ANIMATION_SNAPSHOT_FINAL_COMPOSITE = 0x5,
                ANIMATION_SNAPSHOT_MAX = 0x6
            }
            // Alignment: 1
            // Member count: 5
            #[repr(u8)]
            pub enum NmTargetWarpRule_t {
                WarpXY = 0x0,
                WarpZ = 0x1,
                WarpXYZ = 0x2,
                RotationOnly = 0x3,
                FixedSection = 0x4
            }
            // Alignment: 4
            // Member count: 8
            #[repr(u32)]
            pub enum FootFallTagFoot_t {
                FOOT1 = 0x0,
                FOOT2 = 0x1,
                FOOT3 = 0x2,
                FOOT4 = 0x3,
                FOOT5 = 0x4,
                FOOT6 = 0x5,
                FOOT7 = 0x6,
                FOOT8 = 0x7
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum ChoiceMethod {
                WeightedRandom = 0x0,
                WeightedRandomNoRepeat = 0x1,
                Iterate = 0x2,
                IterateRandom = 0x3
            }
            // Alignment: 4
            // Member count: 14
            #[repr(u32)]
            pub enum AnimVectorSource {
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
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum IkTargetType {
                IkTarget_Attachment = 0x0,
                IkTarget_Bone = 0x1,
                IkTarget_Parameter_ModelSpace = 0x2,
                IkTarget_Parameter_WorldSpace = 0x3
            }
            // Alignment: 1
            // Member count: 3
            #[repr(u8)]
            pub enum RenderMeshSlotType_t {
                RENDERMESH_SLOT_INVALID = u8::MAX,
                RENDERMESH_SLOT_PER_VERTEX = 0x0,
                RENDERMESH_SLOT_PER_INSTANCE = 0x1
            }
            // Alignment: 4
            // Member count: 4
            #[repr(u32)]
            pub enum BoneMaskBlendSpace {
                BlendSpace_Parent = 0x0,
                BlendSpace_Model = 0x1,
                BlendSpace_Model_RotationOnly = 0x2,
                BlendSpace_Model_TranslationOnly = 0x3
            }
            // Alignment: 4
            // Member count: 10
            #[repr(u32)]
            pub enum MovementCapability_t {
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
            }
            // Alignment: 4
            // Member count: 5
            #[repr(u32)]
            pub enum ModelConfigAttachmentType_t {
                MODEL_CONFIG_ATTACHMENT_INVALID = u32::MAX,
                MODEL_CONFIG_ATTACHMENT_BONE_OR_ATTACHMENT = 0x0,
                MODEL_CONFIG_ATTACHMENT_ROOT_RELATIVE = 0x1,
                MODEL_CONFIG_ATTACHMENT_BONEMERGE = 0x2,
                MODEL_CONFIG_ATTACHMENT_COUNT = 0x3
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum BinaryNodeChildOption {
                Child1 = 0x0,
                Child2 = 0x1
            }
            // Alignment: 4
            // Member count: 5
            #[repr(u32)]
            pub enum NPCPhysicsHullType_t {
                eInvalid = 0x0,
                eGroundCapsule = 0x1,
                eCenteredCapsule = 0x2,
                eGenericCapsule = 0x3,
                eGroundBox = 0x4
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum JumpCorrectionMethod {
                ScaleMotion = 0x0,
                AddCorrectionDelta = 0x1
            }
            // Alignment: 4
            // Member count: 2
            #[repr(u32)]
            pub enum MoodType_t {
                eMoodType_Head = 0x0,
                eMoodType_Body = 0x1
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseCell_WaitForCursorsWithTag {
            }
            // Parent: None
            // Field count: 48
            pub mod CPulseCell_Base {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CPulse_ResumePoint {
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseCell_PickBestOutflowSelector {
            }
            // Parent: None
            // Field count: 0
            pub mod CParticleBindingRealPulse {
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseCell_WaitForObservable {
            }
            // Parent: None
            // Field count: 45
            pub mod CPulse_OutflowConnection {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CPulseGraphDef {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseCell_FireCursors {
            }
            // Parent: None
            // Field count: 48
            pub mod CPulseCell_Timeline__TimelineEvent_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CPulseCell_IntervalTimer__CursorState_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub mod CPulseCell_BaseRequirement {
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseCell_BaseState {
            }
            // Parent: None
            // Field count: 48
            pub mod OutflowWithRequirements_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub mod CPulseCell_IsRequirementValid {
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseCell_Value_Gradient {
            }
            // Parent: None
            // Field count: 45
            pub mod CPulseCursorFuncs {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod PulseNodeDynamicOutflows_t__DynamicOutflow_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod CBasePulseGraphInstance {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub mod CPulseCell_Inflow_GraphHook {
            }
            // Parent: None
            // Field count: 0
            pub mod SignatureOutflow_Resume {
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseCell_Inflow_BaseEntrypoint {
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseCell_WaitForCursorsWithTagBase {
            }
            // Parent: None
            // Field count: 48
            pub mod CPulse_InvokeBinding {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseCell_IntervalTimer {
            }
            // Parent: None
            // Field count: 45
            pub mod CPulseTestScriptLib {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub mod CPulseCell_BaseLerp {
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseCell_Value_Curve {
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub mod CPulseCell_Inflow_EventHandler {
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub mod CPulseCell_BaseFlow {
            }
            // Parent: None
            // Field count: 48
            pub mod CPulseCell_Outflow_CycleShuffled__InstanceState_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CPulseCell_BaseLerp__CursorState_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 45
            pub mod CPulseCell_WaitForCursorsWithTagBase__CursorState_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod CPulseArraylib {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod SignatureOutflow_Continue {
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseCell_Timeline {
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseCell_Inflow_EntOutputHandler {
            }
            // Parent: None
            // Field count: 48
            pub mod CPulseCell_Outflow_CycleOrdered__InstanceState_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CParticleCollectionBindingInstance {
            }
            // Parent: None
            // Field count: 48
            pub mod CPulseCell_LimitCount__InstanceState_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub mod CPulseCell_Step_DebugLog {
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub mod CPulseCell_BaseYieldingInflow {
            }
            // Parent: None
            // Field count: 48
            pub mod PulseNodeDynamicOutflows_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod CPulseCell_IsRequirementValid__Criteria_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseCell_Inflow_ObservableVariableListener {
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub mod CPulseCell_Outflow_CycleOrdered {
            }
            // Parent: None
            // Field count: 48
            pub mod PulseSelectorOutflowList_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub mod CPulseCell_Inflow_Wait {
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub mod CPulseCell_Outflow_CycleShuffled {
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseCell_Inflow_Method {
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub mod CPulseCell_BaseValue {
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseCell_BooleanSwitchState {
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub mod CPulseCell_Inflow_Yield {
            }
            // Parent: None
            // Field count: 45
            pub mod CPulseMathlib {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseCell_Unknown {
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub mod CPulseCell_Outflow_CycleRandom {
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub mod CPulseCell_Step_PublicOutput {
            }
            // Parent: None
            // Field count: 48
            pub mod CPulse_BlackboardReference {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseCell_Value_RandomInt {
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimationLayer {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CPulse_CallInfo {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseCell_InlineNodeSkipSelector {
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub mod CPulseCell_LimitCount {
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseCell_Step_CallExternalMethod {
            }
            // Parent: None
            // Field count: 48
            pub mod PulseObservableBoolExpression_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod CPulseCell_LimitCount__Criteria_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: pulse_runtime_lib
            // Field count: 0
            pub mod CPulseCell_CursorQueue {
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseCell_Value_RandomFloat {
            }
            // Parent: None
            // Field count: 45
            pub mod CPulseExecCursor {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimFrameBlockAnim {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CZeroPoseUpdateNode {
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CSlowDownOnSlopesUpdateNode {
            }
            // Parent: None
            // Field count: 0
            pub mod COrientationWarpUpdateNode {
            }
            // Parent: None
            // Field count: 45
            pub mod CAnimParameterBase {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CSequenceFinishedAnimTag {
            }
            // Parent: None
            // Field count: 48
            pub mod CSceneObjectData {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: modellib
            // Field count: 0
            pub mod CFootCycle {
            }
            // Parent: None
            // Field count: 0
            pub mod CParticleModelInput {
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CInputStreamUpdateNode {
            }
            // Parent: None
            // Field count: 0
            pub mod CIntAnimParameter {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmParameterizedBlendNode__CDefinition {
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmScaleTask {
            }
            // Parent: None
            // Field count: 54
            pub mod PerTickSettings_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmFrameSnapEvent {
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimNodePath {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod AnimNodeOutputID {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CNmSyncTrack {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CSeqBoneMaskList {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimEncodedFrames {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 57
            pub mod SampleCode {
                pub const : usize = 0x1FFEEFF; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000000; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x2C000; // 
                pub const : usize = 0x10F; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x107; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x103; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0xE02FA7; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5C006100; // 
                pub const : usize = 0x77006F00; // 
                pub const : usize = 0x66007400; // 
                pub const : usize = 0x36002E00; // 
                pub const : usize = 0x5C003A00; // 
                pub const : usize = 0x72006300; // 
                pub const : usize = 0x65004400; // 
                pub const : usize = 0x5C006D00; // 
                pub const : usize = 0x4D005C00; // 
                pub const : usize = 0x69006600; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x64006E00; // 
                pub const : usize = 0x61006C00; // 
                pub const : usize = 0x30003200; // 
                pub const : usize = 0x6E00; // 
                pub const : usize = 0x7F; // 
                pub const : usize = 0x40000000; // 
                pub const : usize = 0x73006900; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0x69004400; // 
                pub const : usize = 0x65006E00; // 
                pub const : usize = 0x8E000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE000000; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x2000000; // 
                pub const : usize = 0xFFFF00; // 
                pub const : usize = 0x15050000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x110; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CCachedPose {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmTransitionEventConditionNode__CDefinition {
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CTaskStatusAnimTag {
            }
            // Parent: None
            // Field count: 45
            pub mod CStateNodeInstanceData {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CMotionGraphConfig {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CMotionSearchNode {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimActivity {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimScriptManager {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod AnimationDecodeDebugDumpElement_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CChoreoInstanceData {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 56
            pub mod ConfigIndex {
                pub const : usize = 0x1FFEE; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x10000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2C0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0xE02F; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5C0061; // 
                pub const : usize = 0x77006F; // 
                pub const : usize = 0x660074; // 
                pub const : usize = 0x36002E; // 
                pub const : usize = 0x5C003A; // 
                pub const : usize = 0x720063; // 
                pub const : usize = 0x650044; // 
                pub const : usize = 0x5C006D; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x690066; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x64006E; // 
                pub const : usize = 0x61006C; // 
                pub const : usize = 0x300032; // 
                pub const : usize = 0x6E; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xB6400000; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x690044; // 
                pub const : usize = 0x65006E; // 
                pub const : usize = 0xAA8E0000; // 
                pub const : usize = 0xEE0000; // 
                pub const : usize = 0x800000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D; // 
                pub const : usize = 0x20000; // 
                pub const : usize = 0xFFFF; // 
                pub const : usize = 0x150500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x1; // 
            }
            // Parent: None
            // Field count: 48
            pub mod VPhysXCollisionAttributes_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CSequenceUpdateNode {
            }
            // Parent: None
            // Field count: 48
            pub mod CStateMachineInstanceData {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmPassthroughNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmClipReferenceNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CNmGraphDefinition__ExternalGraphSlot_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CAimCameraUpdateNode {
            }
            // Parent: None
            // Field count: 48
            pub mod LookData {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 57
            pub mod CSeqSeqDescFlag {
                pub const : usize = 0x1FFEEFF; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000000; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x2C000; // 
                pub const : usize = 0x10F; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x107; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x103; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0xE02FA7; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5C006100; // 
                pub const : usize = 0x77006F00; // 
                pub const : usize = 0x66007400; // 
                pub const : usize = 0x36002E00; // 
                pub const : usize = 0x5C003A00; // 
                pub const : usize = 0x72006300; // 
                pub const : usize = 0x65004400; // 
                pub const : usize = 0x5C006D00; // 
                pub const : usize = 0x4D005C00; // 
                pub const : usize = 0x69006600; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x64006E00; // 
                pub const : usize = 0x61006C00; // 
                pub const : usize = 0x30003200; // 
                pub const : usize = 0x6E00; // 
                pub const : usize = 0x7F; // 
                pub const : usize = 0x40000000; // 
                pub const : usize = 0x73006900; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0x69004400; // 
                pub const : usize = 0x65006E00; // 
                pub const : usize = 0x8E000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE000000; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x2000000; // 
                pub const : usize = 0xFFFF00; // 
                pub const : usize = 0x15050000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x110; // 
            }
            // Parent: None
            // Field count: 48
            pub mod PermModelInfo_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod StanceInfo_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 45
            pub mod IKBoneNameAndIndex_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CStanceScaleUpdateNode {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmBoneMaskBlendNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmIDEventNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod HitReactFixedSettings_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmPoseNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimTagBase {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CPerParticleVecInput {
            }
            // Parent: None
            // Field count: 48
            pub mod CMotionGraph {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmVectorInfoNode__CDefinition {
            }
            // Parent: ___Y__
            // Field count: 0
            pub mod AnimationSnapshot_t {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmBlendTask {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmVectorValueNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CSeqTransition {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 45
            pub mod CNmPoseTask {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimGraphSettingsGroup {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmVirtualParameterTargetNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimGraphDebugReplay {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CVectorAnimParameter {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmCachedPoseReadTask {
            }
            // Parent: None
            // Field count: 48
            pub mod CNmStateMachineNode__StateDefinition_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CStanceOverrideUpdateNode {
            }
            // Parent: None
            // Field count: 48
            pub mod CNmGraphDefinition__ExternalPoseSlot_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod PhysShapeMarkup_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CBlend2DInstanceData {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod LookAtBone_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CDirectPlaybackInstanceData {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmTransitionNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmFloatEaseNode__CDefinition {
            }
            // Parent: None
            // Field count: 45
            pub mod CAnimationGraphInstance {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimEncodeDifference {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CRootUpdateNode {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmTargetOffsetNode__CDefinition {
            }
            // Parent: None
            // Field count: 54
            pub mod MotionMatchingInstanceData {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CMotionGraphUpdateNode {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmFloatAngleMathNode__CDefinition {
            }
            // Parent: None
            // Field count: 57
            pub mod CAnimParamHandle {
                pub const : usize = 0x1FFEEFF; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000000; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x2C000; // 
                pub const : usize = 0x10F; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x107; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x103; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0xE02FA7; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5C006100; // 
                pub const : usize = 0x77006F00; // 
                pub const : usize = 0x66007400; // 
                pub const : usize = 0x36002E00; // 
                pub const : usize = 0x5C003A00; // 
                pub const : usize = 0x72006300; // 
                pub const : usize = 0x65004400; // 
                pub const : usize = 0x5C006D00; // 
                pub const : usize = 0x4D005C00; // 
                pub const : usize = 0x69006600; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x64006E00; // 
                pub const : usize = 0x61006C00; // 
                pub const : usize = 0x30003200; // 
                pub const : usize = 0x6E00; // 
                pub const : usize = 0x7F; // 
                pub const : usize = 0x40000000; // 
                pub const : usize = 0x73006900; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0x69004400; // 
                pub const : usize = 0x65006E00; // 
                pub const : usize = 0x8E000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE000000; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x2000000; // 
                pub const : usize = 0xFFFF00; // 
                pub const : usize = 0x15050000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x110; // 
            }
            // Parent: None
            // Field count: 48
            pub mod WeightList {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 56
            pub mod MotionIndex {
                pub const : usize = 0x1FFEE; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x10000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2C0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0xE02F; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5C0061; // 
                pub const : usize = 0x77006F; // 
                pub const : usize = 0x660074; // 
                pub const : usize = 0x36002E; // 
                pub const : usize = 0x5C003A; // 
                pub const : usize = 0x720063; // 
                pub const : usize = 0x650044; // 
                pub const : usize = 0x5C006D; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x690066; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x64006E; // 
                pub const : usize = 0x61006C; // 
                pub const : usize = 0x300032; // 
                pub const : usize = 0x6E; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xB6400000; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x690044; // 
                pub const : usize = 0x65006E; // 
                pub const : usize = 0xAA8E0000; // 
                pub const : usize = 0xEE0000; // 
                pub const : usize = 0x800000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D; // 
                pub const : usize = 0x20000; // 
                pub const : usize = 0xFFFF; // 
                pub const : usize = 0x150500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x1; // 
            }
            // Parent: None
            // Field count: 54
            pub mod FollowAttachmentSettings_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmReferencedGraphNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod AnimNodeID {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmClipSelectorNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CLookComponentUpdater {
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmIsExternalPoseSetNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CAnimGraphNetworkSettings {
            }
            // Parent: None
            // Field count: 0
            pub mod CSingleFrameUpdateNode {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmChainLookatNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmFollowBoneNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CBlockSelectionMetricEvaluator {
            }
            // Parent: None
            // Field count: 48
            pub mod MovementData {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CRemapValueComponentUpdater {
            }
            // Parent: None
            // Field count: 48
            pub mod CModelConfigList {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 57
            pub mod CSeqAutoLayerFlag {
                pub const : usize = 0x1FFEEFF; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000000; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x2C000; // 
                pub const : usize = 0x10F; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x107; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x103; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0xE02FA7; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5C006100; // 
                pub const : usize = 0x77006F00; // 
                pub const : usize = 0x66007400; // 
                pub const : usize = 0x36002E00; // 
                pub const : usize = 0x5C003A00; // 
                pub const : usize = 0x72006300; // 
                pub const : usize = 0x65004400; // 
                pub const : usize = 0x5C006D00; // 
                pub const : usize = 0x4D005C00; // 
                pub const : usize = 0x69006600; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x64006E00; // 
                pub const : usize = 0x61006C00; // 
                pub const : usize = 0x30003200; // 
                pub const : usize = 0x6E00; // 
                pub const : usize = 0x7F; // 
                pub const : usize = 0x40000000; // 
                pub const : usize = 0x73006900; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0x69004400; // 
                pub const : usize = 0x65006E00; // 
                pub const : usize = 0x8E000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE000000; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x2000000; // 
                pub const : usize = 0xFFFF00; // 
                pub const : usize = 0x15050000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x110; // 
            }
            // Parent: None
            // Field count: 48
            pub mod AnimStateID {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmIDEventConditionNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod ModelBoneFlexDriver_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CMorphData {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CFootPinningUpdateNode {
            }
            // Parent: None
            // Field count: 48
            pub mod CSeqPoseSetting {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmVirtualParameterFloatNode__CDefinition {
            }
            // Parent: None
            // Field count: 54
            pub mod CAttachment {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CSequenceUpdateNodeBase {
            }
            // Parent: None
            // Field count: 57
            pub mod CDrawCullingData {
                pub const : usize = 0x1FFEEFF; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000000; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x2C000; // 
                pub const : usize = 0x10F; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x107; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x103; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0xE02FA7; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5C006100; // 
                pub const : usize = 0x77006F00; // 
                pub const : usize = 0x66007400; // 
                pub const : usize = 0x36002E00; // 
                pub const : usize = 0x5C003A00; // 
                pub const : usize = 0x72006300; // 
                pub const : usize = 0x65004400; // 
                pub const : usize = 0x5C006D00; // 
                pub const : usize = 0x4D005C00; // 
                pub const : usize = 0x69006600; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x64006E00; // 
                pub const : usize = 0x61006C00; // 
                pub const : usize = 0x30003200; // 
                pub const : usize = 0x6E00; // 
                pub const : usize = 0x7F; // 
                pub const : usize = 0x40000000; // 
                pub const : usize = 0x73006900; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0x69004400; // 
                pub const : usize = 0x65006E00; // 
                pub const : usize = 0x8E000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE000000; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x2000000; // 
                pub const : usize = 0xFFFF00; // 
                pub const : usize = 0x15050000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x110; // 
            }
            // Parent: None
            // Field count: 54
            pub mod VPhysXJoint_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CStateNodeTransitionData {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CTimeRemainingMetricEvaluator {
            }
            // Parent: None
            // Field count: 48
            pub mod CParticleInput {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod CModelConfigElement {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmControlParameterVectorNode__CDefinition {
            }
            // Parent: None
            // Field count: 54
            pub mod FootFixedSettings {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmBoneMaskSwitchNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CModelConfig {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod AnimComponentID {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod VPhysXAggregateData_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CNmGraphDefinition {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmSelectorNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimDemoCaptureSettings {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod IKDemoCaptureSettings_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 57
            pub mod CSeqMultiFetchFlag {
                pub const : usize = 0x1FFEEFF; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000000; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x2C000; // 
                pub const : usize = 0x10F; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x107; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x103; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0xE02FA7; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5C006100; // 
                pub const : usize = 0x77006F00; // 
                pub const : usize = 0x66007400; // 
                pub const : usize = 0x36002E00; // 
                pub const : usize = 0x5C003A00; // 
                pub const : usize = 0x72006300; // 
                pub const : usize = 0x65004400; // 
                pub const : usize = 0x5C006D00; // 
                pub const : usize = 0x4D005C00; // 
                pub const : usize = 0x69006600; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x64006E00; // 
                pub const : usize = 0x61006C00; // 
                pub const : usize = 0x30003200; // 
                pub const : usize = 0x6E00; // 
                pub const : usize = 0x7F; // 
                pub const : usize = 0x40000000; // 
                pub const : usize = 0x73006900; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0x69004400; // 
                pub const : usize = 0x65006E00; // 
                pub const : usize = 0x8E000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE000000; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x2000000; // 
                pub const : usize = 0xFFFF00; // 
                pub const : usize = 0x15050000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x110; // 
            }
            // Parent: None
            // Field count: 48
            pub mod VPhysXRange_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CStopAtGoalUpdateNode {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmIDEventPercentageThroughNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimBoneDifference {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmCachedBoolNode__CDefinition {
            }
            // Parent: modellib
            // Field count: 0
            pub mod CAnimCycle {
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CActionComponentUpdater {
            }
            // Parent: None
            // Field count: 48
            pub mod CDirectionalBlendInstanceData {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CDampedPathAnimMotorUpdater {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmCachedFloatNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmFloatSwitchNode__CDefinition {
            }
            // Parent: modellib
            // Field count: 0
            pub mod CModelConfigElement_SetMaterialGroupOnAttachedModels {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmEntityAttributeEventBase {
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmVirtualParameterBoneMaskNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod SequenceData {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CPathHelperUpdateNode {
            }
            // Parent: None
            // Field count: 48
            pub mod MaterialGroup_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CMorphBundleData {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: modellib
            // Field count: 0
            pub mod CModelConfigElement_UserPick {
            }
            // Parent: None
            // Field count: 57
            pub mod CSolveIKTargetHandle_t {
                pub const : usize = 0x1FFEEFF; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000000; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x2C000; // 
                pub const : usize = 0x10F; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x107; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x103; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0xE02FA7; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5C006100; // 
                pub const : usize = 0x77006F00; // 
                pub const : usize = 0x66007400; // 
                pub const : usize = 0x36002E00; // 
                pub const : usize = 0x5C003A00; // 
                pub const : usize = 0x72006300; // 
                pub const : usize = 0x65004400; // 
                pub const : usize = 0x5C006D00; // 
                pub const : usize = 0x4D005C00; // 
                pub const : usize = 0x69006600; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x64006E00; // 
                pub const : usize = 0x61006C00; // 
                pub const : usize = 0x30003200; // 
                pub const : usize = 0x6E00; // 
                pub const : usize = 0x7F; // 
                pub const : usize = 0x40000000; // 
                pub const : usize = 0x73006900; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0x69004400; // 
                pub const : usize = 0x65006E00; // 
                pub const : usize = 0x8E000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE000000; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x2000000; // 
                pub const : usize = 0xFFFF00; // 
                pub const : usize = 0x15050000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x110; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CFootAdjustmentUpdateNode {
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CChoreoUpdateNode {
            }
            // Parent: None
            // Field count: 0
            pub mod CAimConstraint {
            }
            // Parent: None
            // Field count: 48
            pub mod CMaterialDrawDescriptor__RigidMeshPart_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CFootFallAnimTag {
            }
            // Parent: None
            // Field count: 48
            pub mod CMaterialDrawDescriptor {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CNmSkeleton {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CStepsRemainingMetricEvaluator {
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmVelocityBlendNode__CDefinition {
            }
            // Parent: _
            // Field count: 0
            pub mod CNmZeroPoseTask {
            }
            // Parent: None
            // Field count: 48
            pub mod CBodyGroupSetting {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CVectorQuantizer {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod ParticleNamedValueConfiguration_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmVectorCreateNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CParticleVecInput {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmStateNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmBoneMaskSelectorNode__CDefinition {
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmVirtualParameterVectorNode__CDefinition {
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CFootCycleMetricEvaluator {
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmConstIDNode__CDefinition {
            }
            // Parent: None
            // Field count: 54
            pub mod CHitBox {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmParameterizedSelectorNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod ModelSkeletonData_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod CNmEvent {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod NmCompressionSettings_t__QuantizationRange_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmBlendTaskBase {
            }
            // Parent: None
            // Field count: 48
            pub mod TagSpan_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmConstTargetNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CRemapValueUpdateItem {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod PermModelDataAnimatedMaterialAttribute_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CMotionDataSet {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CProductQuantizer {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CChoiceUpdateNode {
            }
            // Parent: None
            // Field count: 54
            pub mod SkeletonAnimCapture_t__Bone_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CExpressionActionUpdater {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmFloatMathNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CAnimationGraphVisualizerLine {
            }
            // Parent: None
            // Field count: 0
            pub mod CCycleControlUpdateNode {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmTargetInfoNode__CDefinition {
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CDampedValueComponentUpdater {
            }
            // Parent: _
            // Field count: 0
            pub mod CBaseConstraint {
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmControlParameterFloatNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod BlendItem_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod DampedPathMotorInstanceData_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmIsExternalGraphSlotFilledNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CNewParticleEffect {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmClipNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmMaterialAttributeEvent {
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmBlend1DNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CConcreteAnimParameter {
            }
            // Parent: None
            // Field count: 48
            pub mod CNmBoneWeightList {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CPairedSequenceUpdateNode {
            }
            // Parent: None
            // Field count: 0
            pub mod CModelConfigElement_RandomPick {
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmReferencePoseTask {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmScaleNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod VPhysXConstraint2_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 54
            pub mod CConstraintTarget {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod SkeletonAnimCapture_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimUser {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CRagdollComponentUpdater {
            }
            // Parent: None
            // Field count: 48
            pub mod ParticleNamedValueSource_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CFootDefinition {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CSeqSynthAnimDesc {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 56
            pub mod CNmLayerBlendNode__LayerDefinition_t {
                pub const : usize = 0x1FFEE; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x10000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2C0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0xE02F; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5C0061; // 
                pub const : usize = 0x77006F; // 
                pub const : usize = 0x660074; // 
                pub const : usize = 0x36002E; // 
                pub const : usize = 0x5C003A; // 
                pub const : usize = 0x720063; // 
                pub const : usize = 0x650044; // 
                pub const : usize = 0x5C006D; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x690066; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x64006E; // 
                pub const : usize = 0x61006C; // 
                pub const : usize = 0x300032; // 
                pub const : usize = 0x6E; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xB6400000; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x690044; // 
                pub const : usize = 0x65006E; // 
                pub const : usize = 0xAA8E0000; // 
                pub const : usize = 0xEE0000; // 
                pub const : usize = 0x800000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D; // 
                pub const : usize = 0x20000; // 
                pub const : usize = 0xFFFF; // 
                pub const : usize = 0x150500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x1; // 
            }
            // Parent: None
            // Field count: 45
            pub mod CAnimComponentUpdater {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CBlend2DUpdateNode {
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmVelocityBasedSpeedScaleNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CNPCPhysicsHull {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 54
            pub mod PermModelExtPart_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmCachedIDNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CLeanMatrixUpdateNode {
            }
            // Parent: None
            // Field count: 0
            pub mod CSymbolAnimParameter {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmFloatCurveEventNode__CDefinition {
            }
            // Parent: None
            // Field count: 45
            pub mod IAnimationGraphInstance {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 56
            pub mod CNmStateMachineNode__TransitionDefinition_t {
                pub const : usize = 0x1FFEE; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x10000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2C0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0xE02F; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5C0061; // 
                pub const : usize = 0x77006F; // 
                pub const : usize = 0x660074; // 
                pub const : usize = 0x36002E; // 
                pub const : usize = 0x5C003A; // 
                pub const : usize = 0x720063; // 
                pub const : usize = 0x650044; // 
                pub const : usize = 0x5C006D; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x690066; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x64006E; // 
                pub const : usize = 0x61006C; // 
                pub const : usize = 0x300032; // 
                pub const : usize = 0x6E; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xB6400000; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x690044; // 
                pub const : usize = 0x65006E; // 
                pub const : usize = 0xAA8E0000; // 
                pub const : usize = 0xEE0000; // 
                pub const : usize = 0x800000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D; // 
                pub const : usize = 0x20000; // 
                pub const : usize = 0xFFFF; // 
                pub const : usize = 0x150500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x1; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmFloatSelectorNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CTargetSelectorUpdateNode {
            }
            // Parent: None
            // Field count: 48
            pub mod SlopeData {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CHitBoxSetList {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmIsInactiveBranchConditionNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod MotionDBIndex {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CDirectPlaybackTagData {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CParticleCollectionVecInput {
            }
            // Parent: None
            // Field count: 48
            pub mod CNmGraphEventConditionNode__Condition_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CNmFloatRemapNode__RemapRange_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmCurrentSyncEventIDNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmStateCompletedConditionNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmTargetWarpEvent {
            }
            // Parent: None
            // Field count: 48
            pub mod CParamSpanUpdater {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod CAnimActionUpdater {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CParticleAnimTag {
            }
            // Parent: None
            // Field count: 48
            pub mod CCycleBase {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod FollowTargetOpFixedSettings_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmIDSwitchNode__CDefinition {
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmRootMotionEvent {
            }
            // Parent: None
            // Field count: 48
            pub mod CChoiceInstanceData {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 54
            pub mod AnimationSnapshotBase_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CEmitTagActionUpdater {
            }
            // Parent: None
            // Field count: 48
            pub mod DynamicMeshDeformParams_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmTimeConditionNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmSyncEventIndexConditionNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimStateMachineUpdater {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CUnaryUpdateNode {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmIDToFloatNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod PlayerInputMotorInstanceData_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 45
            pub mod CCompressorGroup {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CSelectorUpdateNode {
            }
            // Parent: None
            // Field count: 48
            pub mod NmPercent_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CStateMachineComponentUpdater {
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimUpdateNodeRef {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CPhysSurfaceProperties {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmBoneMaskValueNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CSeqPoseParamDesc {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CModelConfigElement_SetBodygroup {
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimMovement {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CParticleRemapFloatInput {
            }
            // Parent: None
            // Field count: 48
            pub mod FollowAttachmentData {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod FootStepTrigger {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CNmSyncTrack__EventMarker_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CPathMetricEvaluator {
            }
            // Parent: None
            // Field count: 0
            pub mod CFootLockUpdateNode {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmOrientationWarpEvent {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmIDValueNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod JiggleBoneSettings_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CJumpHelperUpdateNode {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmControlParameterBoolNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmChainLookatTask {
            }
            // Parent: None
            // Field count: 48
            pub mod FootLockPoseOpFixedSettings {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CBoneVelocityMetricEvaluator {
            }
            // Parent: None
            // Field count: 0
            pub mod CWarpSectionAnimTag {
            }
            // Parent: None
            // Field count: 45
            pub mod SelectorInstanceData_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: modellib
            // Field count: 0
            pub mod CModelConfigElement_RandomColor {
            }
            // Parent: None
            // Field count: 48
            pub mod CFootMotion {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CJiggleBoneUpdateNode {
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmConstVectorNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod MoodAnimation_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CLODComponentUpdater {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmIsTargetSetNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CPhysSurfacePropertiesVehicle {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimDataChannelDesc {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CPairedSequenceComponentUpdater {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmParticleEvent {
            }
            // Parent: None
            // Field count: 48
            pub mod MotionSelection {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CMorphRectData {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimFoot {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: modellib
            // Field count: 0
            pub mod CParentConstraint {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmCachedVectorNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CNmStateNode__TimedEvent_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CFootPositionMetricEvaluator {
            }
            // Parent: None
            // Field count: 48
            pub mod CParticleVariableRef {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CMoverInstanceData {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 45
            pub mod HSequence {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CDampedValueUpdateItem {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimLocalHierarchy {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CBlendUpdateNode {
            }
            // Parent: None
            // Field count: 0
            pub mod CQuaternionAnimParameter {
            }
            // Parent: None
            // Field count: 48
            pub mod PermModelData_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: _
            // Field count: 0
            pub mod CNmSampleTask {
            }
            // Parent: None
            // Field count: 48
            pub mod CFollowPathInstanceData {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CStaticPoseCacheBuilder {
            }
            // Parent: None
            // Field count: 48
            pub mod ModelMeshBufferData_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CSequenceGroupData {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CStateMachineUpdateNode {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmIDBasedSelectorNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CAimMatrixUpdateNode {
            }
            // Parent: None
            // Field count: 54
            pub mod SkeletonAnimCapture_t__Frame_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CModelConfigElement_Command {
            }
            // Parent: None
            // Field count: 45
            pub mod TargetSelectorInstanceData_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimEventDefinition {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 54
            pub mod CNmRootMotionData {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod AnimScriptHandle {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CFlexOp {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CHitReactUpdateNode {
            }
            // Parent: None
            // Field count: 0
            pub mod CLookAtUpdateNode {
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimBone {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimParameterManagerUpdater {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod BoneDemoCaptureSettings_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod PairedSequenceData {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod CBoneConstraintPoseSpaceMorph__Input_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod MoodAnimationLayer_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CSpeedScaleUpdateNode {
            }
            // Parent: None
            // Field count: 48
            pub mod CSequenceTagSpans {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CModelConfigElement_SetBodygroupOnAttachedModels {
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CCurrentVelocityMetricEvaluator {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmBoolValueNode__CDefinition {
            }
            // Parent: None
            // Field count: 54
            pub mod CAnimReplayFrame {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmTransitionEvent {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmFloatCurveEvent {
            }
            // Parent: None
            // Field count: 0
            pub mod CParticleTransformInput {
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CPathAnimMotorUpdaterBase {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmFootstepEventPercentageThroughNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod AimCameraOpFixedSettings_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 54
            pub mod SkeletonAnimCapture_t__Camera_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmGraphEventConditionNode__CDefinition {
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CFollowAttachmentUpdateNode {
            }
            // Parent: None
            // Field count: 0
            pub mod CMovementComponentUpdater {
            }
            // Parent: modellib
            // Field count: 0
            pub mod CModelConfigElement_SetMaterialGroup {
            }
            // Parent: None
            // Field count: 45
            pub mod IParticleEffect {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CFutureFacingMetricEvaluator {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmIDBasedClipSelectorNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CLeanMatrixInstanceData {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 54
            pub mod NmCompressionSettings_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 45
            pub mod PARTICLE_EHANDLE__ {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: MGetKV3ClassDefaults
            // Field count: 0
            pub mod CAddUpdateNode {
            }
            // Parent: None
            // Field count: 45
            pub mod CAnimEnum {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CDemoSettingsComponentUpdater {
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimDecoder {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmCachedTargetNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CPerParticleFloatInput {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmConstBoolNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CNmBitFlags {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CFootStride {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CSceneObjectData__RTProxyDrawDescriptor_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: MGetKV3ClassDefaults
            // Field count: 0
            pub mod CNmLegacyEvent {
            }
            // Parent: None
            // Field count: 48
            pub mod CStateNodeStateData {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CFootstepLandedAnimTag {
            }
            // Parent: None
            // Field count: 0
            pub mod CEnumAnimParameter {
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CHandshakeAnimTagBase {
            }
            // Parent: None
            // Field count: 48
            pub mod AnimTagID {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CWayPointHelperInstanceData {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimFrameSegment {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CSlopeComponentUpdater {
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimInputDamping {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CHitBoxSet {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 54
            pub mod ChainToSolveData_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 54
            pub mod CConstraintSlave {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CFootCycleDefinition {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CVirtualAnimParameter {
            }
            // Parent: None
            // Field count: 45
            pub mod CMotionMetricEvaluator {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CFutureVelocityMetricEvaluator {
            }
            // Parent: None
            // Field count: 48
            pub mod CNmSkeleton__SecondarySkeleton_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod CAnimUpdateNodeBase {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CSetParameterActionUpdater {
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmFixedWeightBoneMaskNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod SkeletonAnimCapture_t__FrameStamp_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CBonePositionMetricEvaluator {
            }
            // Parent: None
            // Field count: 48
            pub mod CRenderBufferBinding {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CParticleCollectionRendererFloatInput {
            }
            // Parent: None
            // Field count: 48
            pub mod CNmParameterizedBlendNode__Parameterization_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmEntityAttributeIntEvent {
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmNotNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CMeshletDescriptor {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CMorphSetData {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmAndNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CMorphConstraint {
            }
            // Parent: None
            // Field count: 0
            pub mod CClothSettingsAnimTag {
            }
            // Parent: None
            // Field count: 57
            pub mod CAnimDesc_Flag {
                pub const : usize = 0x1FFEEFF; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000000; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x2C000; // 
                pub const : usize = 0x10F; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x107; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x103; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0xE02FA7; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5C006100; // 
                pub const : usize = 0x77006F00; // 
                pub const : usize = 0x66007400; // 
                pub const : usize = 0x36002E00; // 
                pub const : usize = 0x5C003A00; // 
                pub const : usize = 0x72006300; // 
                pub const : usize = 0x65004400; // 
                pub const : usize = 0x5C006D00; // 
                pub const : usize = 0x4D005C00; // 
                pub const : usize = 0x69006600; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x64006E00; // 
                pub const : usize = 0x61006C00; // 
                pub const : usize = 0x30003200; // 
                pub const : usize = 0x6E00; // 
                pub const : usize = 0x7F; // 
                pub const : usize = 0x40000000; // 
                pub const : usize = 0x73006900; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0x69004400; // 
                pub const : usize = 0x65006E00; // 
                pub const : usize = 0x8E000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE000000; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x2000000; // 
                pub const : usize = 0xFFFF00; // 
                pub const : usize = 0x15050000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x110; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmBlend2DNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CBlendCurve {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 45
            pub mod CNmGraphNode__CDefinition {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CAnimationGraphVisualizerSphere {
            }
            // Parent: None
            // Field count: 48
            pub mod CNmClip__ModelSpaceSamplingChainLink_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimUserDifference {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmDurationScaleNode__CDefinition {
            }
            // Parent: None
            // Field count: 57
            pub mod CTransitionUpdateData {
                pub const : usize = 0x1FFEEFF; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x1000000; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x2C000; // 
                pub const : usize = 0x10F; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x107; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x103; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0xE02FA7; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5C006100; // 
                pub const : usize = 0x77006F00; // 
                pub const : usize = 0x66007400; // 
                pub const : usize = 0x36002E00; // 
                pub const : usize = 0x5C003A00; // 
                pub const : usize = 0x72006300; // 
                pub const : usize = 0x65004400; // 
                pub const : usize = 0x5C006D00; // 
                pub const : usize = 0x4D005C00; // 
                pub const : usize = 0x69006600; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x64006E00; // 
                pub const : usize = 0x61006C00; // 
                pub const : usize = 0x30003200; // 
                pub const : usize = 0x6E00; // 
                pub const : usize = 0x7F; // 
                pub const : usize = 0x40000000; // 
                pub const : usize = 0x73006900; // 
                pub const : usize = 0x101; // 
                pub const : usize = 0x69004400; // 
                pub const : usize = 0x65006E00; // 
                pub const : usize = 0x8E000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE000000; // 
                pub const : usize = 0x80000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x2000000; // 
                pub const : usize = 0xFFFF00; // 
                pub const : usize = 0x15050000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100; // 
                pub const : usize = 0x110; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CMovementHandshakeAnimTag {
            }
            // Parent: None
            // Field count: 45
            pub mod CBoneConstraintPoseSpaceBone__Input_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CSeqMultiFetch {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CPhysSurfacePropertiesSoundNames {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod NmBoneMaskSetDefinition_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CAudioAnimTag {
            }
            // Parent: None
            // Field count: 0
            pub mod CTurnHelperUpdateNode {
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimationGroup {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod NmSyncTrackTime_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 54
            pub mod CNmClip {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CToggleComponentActionUpdater {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmOverlayBlendTask {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmSpeedScaleBaseNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmValueNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimationGraphVisualizerPrimitiveBase {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CBindPoseUpdateNode {
            }
            // Parent: None
            // Field count: 48
            pub mod NmSyncTrackTimeRange_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CAnimationGraphVisualizerText {
            }
            // Parent: None
            // Field count: 48
            pub mod CFootTrajectory {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CMotionMatchingUpdateNode {
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmReferencePoseNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CSeqS1SeqDesc {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CFlexController {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod SkeletonDemoDb_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmFloatRangeComparisonNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CMotionNodeSequence {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmTwoBoneIKNode__CDefinition {
            }
            // Parent: None
            // Field count: 45
            pub mod CNmGraphInstance {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CFlexDesc {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CRenderMesh {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimSkeleton {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CFootAdjustmentInstanceData {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod VPhysics2ShapeDef_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmFloatCurveNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CAnimationGraphVisualizerAxis {
            }
            // Parent: None
            // Field count: 0
            pub mod CTwistConstraint {
            }
            // Parent: None
            // Field count: 48
            pub mod ModelBoneFlexDriverControl_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CParticleCollectionFloatInput {
            }
            // Parent: None
            // Field count: 48
            pub mod CSeqCmdLayer {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod FootPinningPoseOpFixedData_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod RenderSkeletonBone_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CDistanceRemainingMetricEvaluator {
            }
            // Parent: None
            // Field count: 0
            pub mod CDirectPlaybackUpdateNode {
            }
            // Parent: MGetKV3ClassDefaults
            // Field count: 0
            pub mod CNmIDEvent {
            }
            // Parent: None
            // Field count: 48
            pub mod CFlexRule {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod ScriptInfo_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CBoneConstraintRbf {
            }
            // Parent: None
            // Field count: 48
            pub mod CNetworkedCycle {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmSoundEvent {
            }
            // Parent: None
            // Field count: 48
            pub mod CRenderGroom {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmParameterizedClipSelectorNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CTiltTwistConstraint {
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmSpeedScaleNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CVPhysXSurfacePropertiesList {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmFloatSpringNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod RenderHairStrandInfo_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CCycleControlClipUpdateNode {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmTargetPointNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmTargetSelectorNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmFloatValueNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CWarpSectionAnimTagBase {
            }
            // Parent: None
            // Field count: 48
            pub mod CNmGraphVariationUserData {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CStaticPoseCache {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 54
            pub mod LookAtOpFixedSettings_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CRootMotion {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimGraphSettingsManager {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimGraphModelBinding {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: modellib
            // Field count: 0
            pub mod CBoneConstraintPoseSpaceBone {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmTwoBoneIKTask {
            }
            // Parent: None
            // Field count: 48
            pub mod MotionBlendItem {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CBoneConstraintPoseSpaceMorph {
            }
            // Parent: None
            // Field count: 0
            pub mod CFloatAnimParameter {
            }
            // Parent: None
            // Field count: 48
            pub mod CStateActionUpdater {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CPlayerInputAnimMotorUpdater {
            }
            // Parent: None
            // Field count: 48
            pub mod CRenderSkeleton {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CSeqIKLock {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimParamHandleMap {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmFloatClampNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmExternalPoseNode__CDefinition {
            }
            // Parent: None
            // Field count: 54
            pub mod CAnimDesc {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmRootMotionOverrideNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmTargetWarpNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CPhysSurfacePropertiesPhysics {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CAnimationGraphVisualizerPie {
            }
            // Parent: None
            // Field count: 48
            pub mod NmFloatCurveCompressionSettings_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CBoneConstraintDotToMorph {
            }
            // Parent: modellib
            // Field count: 0
            pub mod CPointConstraint {
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CCurrentRotationVelocityMetricEvaluator {
            }
            // Parent: None
            // Field count: 48
            pub mod TraceSettings_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod ParamSpanSample_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmFollowBoneTask {
            }
            // Parent: None
            // Field count: 48
            pub mod VPhysXConstraintParams_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmFloatComparisonNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmChainSolverTask {
            }
            // Parent: None
            // Field count: 48
            pub mod CStateUpdateData {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CTurnHelperInstanceData {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmTargetValueNode__CDefinition {
            }
            // Parent: modellib
            // Field count: 0
            pub mod COrientConstraint {
            }
            // Parent: None
            // Field count: 48
            pub mod CCycleClipInstanceData {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CMoverUpdateNode {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmLayerBlendNode__CDefinition {
            }
            // Parent: None
            // Field count: 45
            pub mod CAnimMotorUpdaterBase {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod JiggleBoneSettingsList_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CSubtractUpdateNode {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmFloatRemapNode__CDefinition {
            }
            // Parent: None
            // Field count: 56
            pub mod CPoseHandle {
                pub const : usize = 0x1FFEE; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x10000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2C0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0xE02F; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5C0061; // 
                pub const : usize = 0x77006F; // 
                pub const : usize = 0x660074; // 
                pub const : usize = 0x36002E; // 
                pub const : usize = 0x5C003A; // 
                pub const : usize = 0x720063; // 
                pub const : usize = 0x650044; // 
                pub const : usize = 0x5C006D; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x690066; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x64006E; // 
                pub const : usize = 0x61006C; // 
                pub const : usize = 0x300032; // 
                pub const : usize = 0x6E; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xB6400000; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x690044; // 
                pub const : usize = 0x65006E; // 
                pub const : usize = 0xAA8E0000; // 
                pub const : usize = 0xEE0000; // 
                pub const : usize = 0x800000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D; // 
                pub const : usize = 0x20000; // 
                pub const : usize = 0xFFFF; // 
                pub const : usize = 0x150500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x1; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmFootstepEventIDNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CSolveIKChainUpdateNode {
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmOrNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CSeqCmdSeqDesc {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CBoneMaskUpdateNode {
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CLeafUpdateNode {
            }
            // Parent: None
            // Field count: 0
            pub mod CParticleFloatInput {
            }
            // Parent: None
            // Field count: 48
            pub mod SolveIKChainPoseOpFixedSettings_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmEntityAttributeFloatEvent {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmIDSelectorNode__CDefinition {
            }
            // Parent: None
            // Field count: 54
            pub mod TwoBoneIKSettings_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmCachedPoseWriteTask {
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimUpdateSharedData {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 54
            pub mod CAnimAttachment {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CFootTrajectories {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmIDComparisonNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod PairedSequence_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CEditableMotionGraph {
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmConstFloatNode__CDefinition {
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CStringAnimTag {
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CBoolAnimParameter {
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CRagdollAnimTag {
            }
            // Parent: None
            // Field count: 45
            pub mod CMotionNode {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmStateMachineNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CBodyGroupAnimTag {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmModelSpaceBlendTask {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmZeroPoseNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmOrientationWarpNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod AnimParamID {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CNmBoneMaskNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod ModelAnimGraph2Ref_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod AnimationDecodeDebugDump_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmVirtualParameterIDNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CSeqScaleSet {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CDirectionalBlendUpdateNode {
            }
            // Parent: None
            // Field count: 56
            pub mod CNmGraphDefinition__ReferencedGraphSlot_t {
                pub const : usize = 0x1FFEE; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x10000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF000000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x2C0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0xE02F; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5C0061; // 
                pub const : usize = 0x77006F; // 
                pub const : usize = 0x660074; // 
                pub const : usize = 0x36002E; // 
                pub const : usize = 0x5C003A; // 
                pub const : usize = 0x720063; // 
                pub const : usize = 0x650044; // 
                pub const : usize = 0x5C006D; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x690066; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x64006E; // 
                pub const : usize = 0x61006C; // 
                pub const : usize = 0x300032; // 
                pub const : usize = 0x6E; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xB6400000; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x690044; // 
                pub const : usize = 0x65006E; // 
                pub const : usize = 0xAA8E0000; // 
                pub const : usize = 0xEE0000; // 
                pub const : usize = 0x800000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D; // 
                pub const : usize = 0x20000; // 
                pub const : usize = 0xFFFF; // 
                pub const : usize = 0x150500; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x1; // 
            }
            // Parent: None
            // Field count: 48
            pub mod ParamSpan_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CFootStepTriggerUpdateNode {
            }
            // Parent: None
            // Field count: 45
            pub mod IKTargetSettings_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: modellib
            // Field count: 0
            pub mod CModelConfigElement_SetRenderColor {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmAdditiveBlendTask {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmAnimationPoseNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CMotionSearchDB {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CMotionNodeBlend1D {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmCurrentSyncEventNode__CDefinition {
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmControlParameterTargetNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod SkeletonBoneBounds_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CTwoBoneIKUpdateNode {
            }
            // Parent: None
            // Field count: 48
            pub mod VPhysXBodyPart_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CTaskHandshakeAnimTag {
            }
            // Parent: None
            // Field count: 0
            pub mod CFollowPathUpdateNode {
            }
            // Parent: None
            // Field count: 54
            pub mod AimMatrixOpFixedSettings_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CAnimScriptComponentUpdater {
            }
            // Parent: None
            // Field count: 0
            pub mod CRagdollUpdateNode {
            }
            // Parent: None
            // Field count: 45
            pub mod CBoneConstraintBase {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: animgraphlib
            // Field count: 0
            pub mod CPathAnimMotorUpdater {
            }
            // Parent: None
            // Field count: 0
            pub mod CMaterialAttributeAnimTag {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmFootEventConditionNode__CDefinition {
            }
            // Parent: _
            // Field count: 0
            pub mod CNmFootEvent {
            }
            // Parent: None
            // Field count: 48
            pub mod CPhysSurfacePropertiesAudio {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CSeqAutoLayer {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod ModelEmbeddedMesh_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod PhysSoftbodyDesc_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimSequenceParams {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CTargetWarpUpdateNode {
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmVirtualParameterBoolNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CFollowTargetUpdateNode {
            }
            // Parent: None
            // Field count: 0
            pub mod CCPPScriptComponentUpdater {
            }
            // Parent: None
            // Field count: 48
            pub mod NetVarConfigIndex {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimData {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod IKSolverSettings_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CBinaryUpdateNode {
            }
            // Parent: None
            // Field count: 48
            pub mod CBlendNodeInstanceData {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CNmParameterizedBlendNode__BlendRange_t {
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xFF00; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x70004D; // 
                pub const : usize = 0x670069; // 
                pub const : usize = 0x610074; // 
                pub const : usize = 0x6F0064; // 
                pub const : usize = 0x740061; // 
                pub const : usize = 0x2E0030; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xE02FB640; // 
                pub const : usize = 0x700073; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x730069; // 
                pub const : usize = 0x720065; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x80; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x520002; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x15; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimKeyData {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod AttachmentHandle_t {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimMorphDifference {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 54
            pub mod FootFixedData_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: None
            // Field count: 48
            pub mod CMotionGraphGroup {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmVectorNegateNode__CDefinition {
            }
            // Parent: None
            // Field count: 48
            pub mod CNmSyncTrack__Event_t {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CParticleCollectionRendererVecInput {
            }
            // Parent: None
            // Field count: 54
            pub mod CNmTarget {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10740; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100000; // 
                pub const : usize = 0x88; // 
                pub const �: usize = 0x1F; // �
                pub const : usize = 0x10018; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x102C0; // P
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x8B7F8EA0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const P: usize = 0x4; // P
                pub const : usize = 0xE02FA840; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6D0061; // 
                pub const : usize = 0x57005C; // 
                pub const : usize = 0x5C0072; // 
                pub const : usize = 0x360032; // 
                pub const : usize = 0x6C0064; // 
                pub const : usize = 0x50005C; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x660065; // 
                pub const : usize = 0x34005C; // 
                pub const : usize = 0x360000; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x96; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x10150; // 
                pub const P: usize = 0x310038; // P
                pub const : usize = 0x4E002E; // 
                pub const : usize = 0x55002D; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20004D; // 
                pub const : usize = 0x10150; // 
                pub const : usize = 0x640064; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x20009; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5467122E; // 
                pub const : usize = 0x10150; // 
                pub const �: usize = 0x10060; // �
                pub const : usize = 0x0; // 
            }
            // Parent: animlib
            // Field count: 0
            pub mod CNmControlParameterIDNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmBodyGroupEvent {
            }
            // Parent: None
            // Field count: 0
            pub mod CWayPointHelperUpdateNode {
            }
            // Parent: None
            // Field count: 48
            pub mod CMoodVData {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 0
            pub mod CModelConfigElement_AttachedModel {
            }
            // Parent: None
            // Field count: 48
            pub mod CAnimTagManagerUpdater {
                pub const : usize = 0x10120; // 
                pub const : usize = 0x10; // 
                pub const : usize = 0x0; // 
                pub const `: usize = 0x0; // 
                pub const : usize = 0xEEFFEEFF; // 
                pub const : usize = 0x1000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x10FD0; // �
                pub const : usize = 0x10110; // 
                pub const : usize = 0x10750; // P
                pub const : usize = 0x1FE000; // 
                pub const : usize = 0x1; // 
                pub const P: usize = 0x10330; // 
                pub const : usize = 0x10150; // P
                pub const : usize = 0xE02FA810; // 
                pub const : usize = 0x630069; // 
                pub const : usize = 0x440020; // 
                pub const : usize = 0x6D0072; // 
                pub const : usize = 0x5C0030; // 
                pub const : usize = 0x6F0072; // 
                pub const : usize = 0x6F0073; // 
                pub const : usize = 0x6E0065; // 
                pub const : usize = 0x31002E; // 
                pub const : usize = 0x61004F; // 
                pub const : usize = 0x6C0000; // 
                pub const : usize = 0x4D005C; // 
                pub const : usize = 0x730077; // 
                pub const : usize = 0x6F0066; // 
                pub const : usize = 0x2D0036; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x5D00005D; // 
                pub const P: usize = 0x10150; // P
                pub const : usize = 0x6C0070; // 
                pub const : usize = 0x5C; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0xEE; // 
                pub const : usize = 0x3; // 
                pub const : usize = 0x790079; // 
                pub const : usize = 0x70; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x99A46D76; // 
                pub const : usize = 0x1D00001D; // 
                pub const : usize = 0x100F0; // �
                pub const �: usize = 0xF000; // 
            }
            // Parent: None
            // Field count: 45
            pub mod CParticleProperty {
                pub const : usize = 0x10110FF; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x45; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x400; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1032000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x61007400; // 
                pub const : usize = 0x6F006400; // 
                pub const : usize = 0x74006100; // 
                pub const : usize = 0x2E003000; // 
                pub const : usize = 0x3A004300; // 
                pub const : usize = 0x63006900; // 
                pub const : usize = 0x44002000; // 
                pub const : usize = 0x6D007200; // 
                pub const : usize = 0x5C003000; // 
                pub const : usize = 0x66006E00; // 
                pub const : usize = 0x61004400; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x6C005000; // 
                pub const : usize = 0x32003000; // 
                pub const : usize = 0x6E006500; // 
                pub const : usize = 0x7FF98B; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x1; // 
                pub const : usize = 0x69006400; // 
                pub const : usize = 0x1015000; // 
                pub const : usize = 0x44005C00; // 
                pub const : usize = 0x6E006900; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x7700; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x4D004D00; // 
                pub const : usize = 0x600; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x0; // 
                pub const : usize = 0x100F003; // 
                pub const : usize = 0x1100000; // 
            }
        }
    }
}
