// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-04 12:25:34.881424700 +07:00

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]

pub mod source2_dumper {
    pub mod schemas {
        // Module: animationsystem.dll
        // Class count: 86
        // Enum count: 143
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
            // Member count: 2
            #[repr(u32)]
            pub enum PulseCursorWakePriority_t {
                WakeElegantly = 0x0,
                WakeImmediate = 0x1
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
            // Member count: 8
            #[repr(u8)]
            pub enum ModelMeshBufferUsage_t {
                MESH_BUFFER_USAGE_NONE = 0x0,
                MESH_BUFFER_USAGE_VB = 0x1,
                MESH_BUFFER_USAGE_IB = 0x2,
                MESH_BUFFER_USAGE_ADJACENCY = 0x4,
                MESH_BUFFER_USAGE_MESHLET_TRIS = 0x8,
                MESH_BUFFER_USAGE_RT_PROXY = 0x10,
                MESH_BUFFER_USAGE_VERTEX_ALBEDO = 0x20,
                MESH_BUFFER_USAGE_VERTEX_EMISSIVE = 0x40
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
            // Alignment: 1
            // Member count: 3
            #[repr(u8)]
            pub enum TagActionStatus {
                Inactive = 0x0,
                Active = 0x1,
                Fired = 0x2
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
            // Member count: 7
            #[repr(u32)]
            pub enum NPCPhysicsHullType_t {
                eInvalid = 0x0,
                eGroundCapsule = 0x1,
                eCenteredCapsule = 0x2,
                eGenericCapsule = 0x3,
                eGroundBox = 0x4,
                eGroundCylinder = 0x5,
                eCenteredCylinder = 0x6
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
            pub mod CPulse_ResumePoint {
            }
            // Parent: None
            // Field count: 0
            pub mod CParticleBindingRealPulse {
            }
            // Parent: None
            // Field count: 4
            pub mod CPulse_OutflowConnection {
                pub const m_SourceOutflowName: usize = 0x0; // PulseSymbol_t
                pub const m_nDestChunk: usize = 0x10; // PulseRuntimeChunkIndex_t
                pub const m_nInstruction: usize = 0x14; // int32
                pub const m_OutflowRegisterMap: usize = 0x18; // PulseRegisterMap_t
            }
            // Parent: None
            // Field count: 0
            pub mod CBasePulseGraphInstance {
            }
            // Parent: None
            // Field count: 0
            pub mod SignatureOutflow_Resume {
            }
            // Parent: None
            // Field count: 0
            pub mod SignatureOutflow_Continue {
            }
            // Parent: None
            // Field count: 0
            pub mod CParticleCollectionBindingInstance {
            }
            // Parent: None
            // Field count: 1
            pub mod CPulseCell_IsRequirementValid__Criteria_t {
                pub const m_bIsValid: usize = 0x0; // bool
            }
            // Parent: None
            // Field count: 1
            pub mod CPulseCell_Unknown {
                pub const m_UnknownKeys: usize = 0x48; // KeyValues3
            }
            // Parent: None
            // Field count: 1
            pub mod CPulseCell_LimitCount__Criteria_t {
                pub const m_bLimitCountPasses: usize = 0x0; // bool
            }
            // Parent: None
            // Field count: 0
            pub mod CPulseExecCursor {
            }
            // Parent: None
            // Field count: 12
            pub mod CNmFootIKTask {
                pub const m_nLeftEffectorBoneIdx: usize = 0x70; // int32
                pub const m_nRightEffectorBoneIdx: usize = 0x74; // int32
                pub const m_leftTargetTransform: usize = 0x80; // CTransform
                pub const m_rightTargetTransform: usize = 0xA0; // CTransform
                pub const m_nLeftTargetBoneIdx: usize = 0xC0; // int32
                pub const m_nRightTargetBoneIdx: usize = 0xC4; // int32
                pub const m_leftTarget: usize = 0xD0; // CNmTarget
                pub const m_rightTarget: usize = 0x100; // CNmTarget
                pub const m_blendMode: usize = 0x130; // NmIKBlendMode_t
                pub const m_flBlendWeight: usize = 0x134; // float32
                pub const m_bIsTargetInWorldSpace: usize = 0x138; // bool
                pub const m_bIsRunningFromDeserializedData: usize = 0x139; // bool
            }
            // Parent: None
            // Field count: 0
            pub mod CNmScaleTask {
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CAnimNodePath {
                pub const m_path: usize = 0x0; // AnimNodeID[11]
                pub const m_nCount: usize = 0x2C; // int32
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod AnimNodeOutputID {
                pub const m_id: usize = 0x0; // uint32
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // SEQ_POSE_SETTING_ROTATION
            pub mod CSeqBoneMaskList {
                pub const m_sName: usize = 0x0; // CBufferString
                pub const m_nLocalBoneArray: usize = 0x10; // CUtlVector<int16>
                pub const m_flBoneWeightArray: usize = 0x28; // CUtlVector<float32>
                pub const m_flDefaultMorphCtrlWeight: usize = 0x40; // float32
                pub const m_morphCtrlWeightArray: usize = 0x48; // CUtlVector<std::pair<CBufferString,float32>>
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod SampleCode {
                pub const m_subCode: usize = 0x0; // uint8[8]
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod AnimationDecodeDebugDumpElement_t {
                pub const m_nEntityIndex: usize = 0x0; // int32
                pub const m_modelName: usize = 0x8; // CUtlString
                pub const m_poseParams: usize = 0x10; // CUtlVector<CUtlString>
                pub const m_decodeOps: usize = 0x28; // CUtlVector<CUtlString>
                pub const m_internalOps: usize = 0x40; // CUtlVector<CUtlString>
                pub const m_decodedAnims: usize = 0x58; // CUtlVector<CUtlString>
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod ConfigIndex {
                pub const m_nGroup: usize = 0x0; // uint16
                pub const m_nConfig: usize = 0x2; // uint16
            }
            // Parent: None
            // Field count: 11
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CSeqSeqDescFlag {
                pub const m_bLooping: usize = 0x0; // bool
                pub const m_bSnap: usize = 0x1; // bool
                pub const m_bAutoplay: usize = 0x2; // bool
                pub const m_bPost: usize = 0x3; // bool
                pub const m_bHidden: usize = 0x4; // bool
                pub const m_bMulti: usize = 0x5; // bool
                pub const m_bLegacyDelta: usize = 0x6; // bool
                pub const m_bLegacyWorldspace: usize = 0x7; // bool
                pub const m_bLegacyCyclepose: usize = 0x8; // bool
                pub const m_bLegacyRealtime: usize = 0x9; // bool
                pub const m_bModelDoc: usize = 0xA; // bool
            }
            // Parent: None
            // Field count: 1
            pub mod IKBoneNameAndIndex_t {
                pub const m_Name: usize = 0x0; // CUtlString
            }
            // Parent: None
            // Field count: 0
            pub mod CNmPoseNode__CDefinition {
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
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CSeqTransition {
                pub const m_flFadeInTime: usize = 0x0; // float32
                pub const m_flFadeOutTime: usize = 0x4; // float32
            }
            // Parent: None
            // Field count: 0
            pub mod CNmPoseTask {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmCachedPoseReadTask {
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CAnimParamHandle {
                pub const m_type: usize = 0x0; // AnimParamType_t
                pub const m_index: usize = 0x1; // uint8
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod AnimNodeID {
                pub const m_id: usize = 0x0; // uint32
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CSeqAutoLayerFlag {
                pub const m_bPost: usize = 0x0; // bool
                pub const m_bSpline: usize = 0x1; // bool
                pub const m_bXFade: usize = 0x2; // bool
                pub const m_bNoBlend: usize = 0x3; // bool
                pub const m_bLocal: usize = 0x4; // bool
                pub const m_bPose: usize = 0x5; // bool
                pub const m_bFetchFrame: usize = 0x6; // bool
                pub const m_bSubtract: usize = 0x7; // bool
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod AnimStateID {
                pub const m_id: usize = 0x0; // uint32
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MKV3TransferName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CSeqPoseSetting {
                pub const m_sPoseParameter: usize = 0x0; // CBufferString
                pub const m_sAttachment: usize = 0x10; // CBufferString
                pub const m_sReferenceSequence: usize = 0x20; // CBufferString
                pub const m_flValue: usize = 0x30; // float32
                pub const m_bX: usize = 0x34; // bool
                pub const m_bY: usize = 0x35; // bool
                pub const m_bZ: usize = 0x36; // bool
                pub const m_eType: usize = 0x38; // int32
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CDrawCullingData {
                pub const m_ConeAxis: usize = 0x0; // int8[3]
                pub const m_ConeCutoff: usize = 0x3; // int8
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CSeqMultiFetchFlag {
                pub const m_bRealtime: usize = 0x0; // bool
                pub const m_bCylepose: usize = 0x1; // bool
                pub const m_b0D: usize = 0x2; // bool
                pub const m_b1D: usize = 0x3; // bool
                pub const m_b2D: usize = 0x4; // bool
                pub const m_b2D_TRI: usize = 0x5; // bool
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CSolveIKTargetHandle_t {
                pub const m_positionHandle: usize = 0x0; // CAnimParamHandle
                pub const m_orientationHandle: usize = 0x2; // CAnimParamHandle
            }
            // Parent: None
            // Field count: 0
            pub mod CNmZeroPoseTask {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmBlendTaskBase {
            }
            // Parent: None
            // Field count: 33
            pub mod CNewParticleEffect {
                pub const m_pNext: usize = 0x10; // CNewParticleEffect*
                pub const m_pPrev: usize = 0x18; // CNewParticleEffect*
                pub const m_pParticles: usize = 0x20; // IParticleCollection*
                pub const m_pDebugName: usize = 0x28; // char*
                pub const m_bDontRemove: usize = 0x0; // bitfield:1
                pub const m_bRemove: usize = 0x0; // bitfield:1
                pub const m_bNeedsBBoxUpdate: usize = 0x0; // bitfield:1
                pub const m_bIsFirstFrame: usize = 0x0; // bitfield:1
                pub const m_bAutoUpdateBBox: usize = 0x0; // bitfield:1
                pub const m_bAllocated: usize = 0x0; // bitfield:1
                pub const m_bSimulate: usize = 0x0; // bitfield:1
                pub const m_bShouldPerformCullCheck: usize = 0x0; // bitfield:1
                pub const m_bForceNoDraw: usize = 0x0; // bitfield:1
                pub const m_bSuppressScreenSpaceEffect: usize = 0x0; // bitfield:1
                pub const m_bShouldSave: usize = 0x0; // bitfield:1
                pub const m_bShouldSimulateDuringGamePaused: usize = 0x0; // bitfield:1
                pub const m_bShouldCheckFoW: usize = 0x0; // bitfield:1
                pub const m_bIsAsyncCreate: usize = 0x0; // bitfield:1
                pub const m_bFreezeTransitionActive: usize = 0x0; // bitfield:1
                pub const m_bFreezeTargetState: usize = 0x0; // bitfield:1
                pub const m_bCanFreeze: usize = 0x0; // bitfield:1
                pub const m_vSortOrigin: usize = 0x40; // Vector
                pub const m_flScale: usize = 0x4C; // float32
                pub const m_hOwner: usize = 0x50; // PARTICLE_EHANDLE__*
                pub const m_pOwningParticleProperty: usize = 0x58; // CParticleProperty*
                pub const m_flFreezeTransitionStart: usize = 0x70; // float32
                pub const m_flFreezeTransitionDuration: usize = 0x74; // float32
                pub const m_flFreezeTransitionOverride: usize = 0x78; // float32
                pub const m_LastMin: usize = 0x7C; // Vector
                pub const m_LastMax: usize = 0x88; // Vector
                pub const m_nSplitScreenUser: usize = 0x94; // CSplitScreenSlot
                pub const m_vecAggregationCenter: usize = 0x98; // Vector
                pub const m_RefCount: usize = 0xD0; // int32
            }
            // Parent: None
            // Field count: 0
            pub mod CNmReferencePoseTask {
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MKV3TransferName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CSeqSynthAnimDesc {
                pub const m_sName: usize = 0x0; // CBufferString
                pub const m_flags: usize = 0x10; // CSeqSeqDescFlag
                pub const m_transition: usize = 0x1C; // CSeqTransition
                pub const m_nLocalBaseReference: usize = 0x24; // int16
                pub const m_nLocalBoneMask: usize = 0x26; // int16
                pub const m_activityArray: usize = 0x28; // CUtlVector<CAnimActivity>
            }
            // Parent: None
            // Field count: 0
            pub mod IAnimationGraphInstance {
            }
            // Parent: None
            // Field count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod AnimationSnapshotBase_t {
                pub const m_flRealTime: usize = 0x0; // float32
                pub const m_rootToWorld: usize = 0x10; // matrix3x4a_t
                pub const m_bBonesInWorldSpace: usize = 0x40; // bool
                pub const m_boneSetupMask: usize = 0x48; // CUtlVector<uint32>
                pub const m_boneTransforms: usize = 0x60; // CUtlVector<matrix3x4a_t>
                pub const m_flexControllers: usize = 0x78; // CUtlVector<float32>
                pub const m_SnapshotType: usize = 0x90; // AnimationSnapshotType_t
                pub const m_bHasDecodeDump: usize = 0x94; // bool
                pub const m_DecodeDump: usize = 0x98; // AnimationDecodeDebugDumpElement_t
            }
            // Parent: None
            // Field count: 0
            pub mod CNmBoneMaskValueNode__CDefinition {
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MKV3TransferName
            // MGetKV3ClassDefaults
            pub mod CSeqPoseParamDesc {
                pub const m_sName: usize = 0x0; // CBufferString
                pub const m_flStart: usize = 0x10; // float32
                pub const m_flEnd: usize = 0x14; // float32
                pub const m_flLoop: usize = 0x18; // float32
                pub const m_bLooping: usize = 0x1C; // bool
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MKV3TransferName
            // MGetKV3ClassDefaults
            pub mod CAnimMovement {
                pub const endframe: usize = 0x0; // int32
                pub const motionflags: usize = 0x4; // int32
                pub const v0: usize = 0x8; // float32
                pub const v1: usize = 0xC; // float32
                pub const angle: usize = 0x10; // float32
                pub const vector: usize = 0x14; // Vector
                pub const position: usize = 0x20; // Vector
            }
            // Parent: None
            // Field count: 0
            pub mod CNmIDValueNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmChainLookatTask {
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MKV3TransferName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CAnimLocalHierarchy {
                pub const m_sBone: usize = 0x0; // CBufferString
                pub const m_sNewParent: usize = 0x10; // CBufferString
                pub const m_nStartFrame: usize = 0x20; // int32
                pub const m_nPeakFrame: usize = 0x24; // int32
                pub const m_nTailFrame: usize = 0x28; // int32
                pub const m_nEndFrame: usize = 0x2C; // int32
            }
            // Parent: None
            // Field count: 0
            pub mod CNmSampleTask {
            }
            // Parent: None
            // Field count: 14
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MKV3TransferName
            pub mod CSequenceGroupData {
                pub const m_sName: usize = 0x10; // CBufferString
                pub const m_nFlags: usize = 0x20; // uint32
                pub const m_localSequenceNameArray: usize = 0x28; // CUtlVector<CBufferString>
                pub const m_localS1SeqDescArray: usize = 0x40; // CUtlVector<CSeqS1SeqDesc>
                pub const m_localMultiSeqDescArray: usize = 0x58; // CUtlVector<CSeqS1SeqDesc>
                pub const m_localSynthAnimDescArray: usize = 0x70; // CUtlVector<CSeqSynthAnimDesc>
                pub const m_localCmdSeqDescArray: usize = 0x88; // CUtlVector<CSeqCmdSeqDesc>
                pub const m_localBoneMaskArray: usize = 0xA0; // CUtlVector<CSeqBoneMaskList>
                pub const m_localScaleSetArray: usize = 0xB8; // CUtlVector<CSeqScaleSet>
                pub const m_localBoneNameArray: usize = 0xD0; // CUtlVector<CBufferString>
                pub const m_localNodeName: usize = 0xE8; // CBufferString
                pub const m_localPoseParamArray: usize = 0xF8; // CUtlVector<CSeqPoseParamDesc>
                pub const m_keyValues: usize = 0x110; // KeyValues3
                pub const m_localIKAutoplayLockArray: usize = 0x120; // CUtlVector<CSeqIKLock>
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MKV3TransferName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MKV3TransferName
            pub mod CAnimEventDefinition {
                pub const m_nFrame: usize = 0x8; // int32
                pub const m_nEndFrame: usize = 0xC; // int32
                pub const m_flCycle: usize = 0x10; // float32
                pub const m_flDuration: usize = 0x14; // float32
                pub const m_EventData: usize = 0x18; // KeyValues3
                pub const m_sLegacyOptions: usize = 0x28; // CBufferString
                pub const m_sEventName: usize = 0x38; // CGlobalSymbol
            }
            // Parent: None
            // Field count: 2
            pub mod CBoneConstraintPoseSpaceMorph__Input_t {
                pub const m_inputValue: usize = 0x0; // Vector
                pub const m_outputWeightList: usize = 0x10; // CUtlVector<float32>
            }
            // Parent: None
            // Field count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MPropertyArrayElementNameKey
            // MPropertyDescription
            // MPropertyDescription
            // MPropertyDescription
            // MPropertyDescription
            // MPropertyDescription
            // MPropertyDescription
            pub mod MoodAnimationLayer_t {
                pub const m_sName: usize = 0x0; // CUtlString
                pub const m_bActiveListening: usize = 0x8; // bool
                pub const m_bActiveTalking: usize = 0x9; // bool
                pub const m_layerAnimations: usize = 0x10; // CUtlVector<MoodAnimation_t>
                pub const m_flIntensity: usize = 0x28; // CRangeFloat
                pub const m_flDurationScale: usize = 0x30; // CRangeFloat
                pub const m_bScaleWithInts: usize = 0x38; // bool
                pub const m_flNextStart: usize = 0x3C; // CRangeFloat
                pub const m_flStartOffset: usize = 0x44; // CRangeFloat
                pub const m_flEndOffset: usize = 0x4C; // CRangeFloat
                pub const m_flFadeIn: usize = 0x54; // float32
                pub const m_flFadeOut: usize = 0x58; // float32
            }
            // Parent: None
            // Field count: 0
            pub mod CNmBoolValueNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            pub mod IParticleEffect {
            }
            // Parent: None
            // Field count: 1
            pub mod PARTICLE_EHANDLE__ {
                pub const unused: usize = 0x0; // int32
            }
            // Parent: None
            // Field count: 1
            pub mod CAnimEnum {
                pub const m_value: usize = 0x0; // uint8
            }
            // Parent: None
            // Field count: 3
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MKV3TransferName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CAnimDecoder {
                pub const m_szName: usize = 0x0; // CBufferString
                pub const m_nVersion: usize = 0x10; // int32
                pub const m_nType: usize = 0x14; // int32
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MKV3TransferName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CAnimFrameSegment {
                pub const m_nUniqueFrameIndex: usize = 0x0; // int32
                pub const m_nLocalElementMasks: usize = 0x4; // uint32
                pub const m_nLocalChannel: usize = 0x8; // int32
                pub const m_container: usize = 0x10; // CUtlBinaryBlock
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CAnimDesc_Flag {
                pub const m_bLooping: usize = 0x0; // bool
                pub const m_bAllZeros: usize = 0x1; // bool
                pub const m_bHidden: usize = 0x2; // bool
                pub const m_bDelta: usize = 0x3; // bool
                pub const m_bLegacyWorldspace: usize = 0x4; // bool
                pub const m_bModelDoc: usize = 0x5; // bool
                pub const m_bImplicitSeqIgnoreDelta: usize = 0x6; // bool
                pub const m_bAnimGraphAdditive: usize = 0x7; // bool
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CTransitionUpdateData {
                pub const m_srcStateIndex: usize = 0x0; // uint8
                pub const m_destStateIndex: usize = 0x1; // uint8
                pub const m_nHandshakeMaskToDisableFirst: usize = 0x0; // bitfield:7
                pub const m_bDisabled: usize = 0x0; // bitfield:1
            }
            // Parent: None
            // Field count: 2
            pub mod CBoneConstraintPoseSpaceBone__Input_t {
                pub const m_inputValue: usize = 0x0; // Vector
                pub const m_outputTransformList: usize = 0x10; // CUtlVector<CTransform>
            }
            // Parent: None
            // Field count: 10
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CSeqMultiFetch {
                pub const m_flags: usize = 0x0; // CSeqMultiFetchFlag
                pub const m_localReferenceArray: usize = 0x8; // CUtlVector<int16>
                pub const m_nGroupSize: usize = 0x20; // int32[2]
                pub const m_nLocalPose: usize = 0x28; // int32[2]
                pub const m_poseKeyArray0: usize = 0x30; // CUtlVector<float32>
                pub const m_poseKeyArray1: usize = 0x48; // CUtlVector<float32>
                pub const m_nLocalCyclePoseParameter: usize = 0x60; // int32
                pub const m_bCalculatePoseParameters: usize = 0x64; // bool
                pub const m_bFixedBlendWeight: usize = 0x65; // bool
                pub const m_flFixedBlendWeightVals: usize = 0x68; // float32[2]
            }
            // Parent: None
            // Field count: 0
            pub mod CNmOverlayBlendTask {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmValueNode__CDefinition {
            }
            // Parent: None
            // Field count: 11
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MKV3TransferName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CSeqS1SeqDesc {
                pub const m_sName: usize = 0x0; // CBufferString
                pub const m_flags: usize = 0x10; // CSeqSeqDescFlag
                pub const m_fetch: usize = 0x20; // CSeqMultiFetch
                pub const m_nLocalWeightlist: usize = 0x90; // int32
                pub const m_autoLayerArray: usize = 0x98; // CUtlVector<CSeqAutoLayer>
                pub const m_IKLockArray: usize = 0xB0; // CUtlVector<CSeqIKLock>
                pub const m_transition: usize = 0xC8; // CSeqTransition
                pub const m_SequenceKeys: usize = 0xD0; // KeyValues3
                pub const m_LegacyKeyValueText: usize = 0xE0; // CBufferString
                pub const m_activityArray: usize = 0xF0; // CUtlVector<CAnimActivity>
                pub const m_footMotion: usize = 0x108; // CUtlVector<CFootMotion>
            }
            // Parent: None
            // Field count: 0
            pub mod CNmGraphInstance {
            }
            // Parent: None
            // Field count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MKV3TransferName
            pub mod CSeqCmdLayer {
                pub const m_cmd: usize = 0x0; // int16
                pub const m_nLocalReference: usize = 0x2; // int16
                pub const m_nLocalBonemask: usize = 0x4; // int16
                pub const m_nDstResult: usize = 0x6; // int16
                pub const m_nSrcResult: usize = 0x8; // int16
                pub const m_bSpline: usize = 0xA; // bool
                pub const m_flVar1: usize = 0xC; // float32
                pub const m_flVar2: usize = 0x10; // float32
                pub const m_nLineNumber: usize = 0x14; // int16
            }
            // Parent: None
            // Field count: 0
            pub mod CNmFloatValueNode__CDefinition {
            }
            // Parent: None
            // Field count: 10
            pub mod CNmTwoBoneIKTask {
                pub const m_nEffectorBoneIdx: usize = 0x70; // int32
                pub const m_nEffectorTargetBoneIdx: usize = 0x74; // int32
                pub const m_targetTransform: usize = 0x80; // CTransform
                pub const m_effectorTarget: usize = 0xA0; // CNmTarget
                pub const m_blendMode: usize = 0xD0; // NmIKBlendMode_t
                pub const m_flBlendWeight: usize = 0xD4; // float32
                pub const m_bIsTargetInWorldSpace: usize = 0xD8; // bool
                pub const m_bIsRunningFromDeserializedData: usize = 0xD9; // bool
                pub const m_flChainRotationWeight: usize = 0xDC; // float32
                pub const m_debugEffectorBoneID: usize = 0xE0; // CGlobalSymbol
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CSeqIKLock {
                pub const m_flPosWeight: usize = 0x0; // float32
                pub const m_flAngleWeight: usize = 0x4; // float32
                pub const m_nLocalBone: usize = 0x8; // int16
                pub const m_bBonesOrientedAlongPositiveX: usize = 0xA; // bool
            }
            // Parent: None
            // Field count: 0
            pub mod CNmFollowBoneTask {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmTargetValueNode__CDefinition {
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CPoseHandle {
                pub const m_nIndex: usize = 0x0; // uint16
                pub const m_eType: usize = 0x2; // PoseType_t
            }
            // Parent: None
            // Field count: 12
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MKV3TransferName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CSeqCmdSeqDesc {
                pub const m_sName: usize = 0x0; // CBufferString
                pub const m_flags: usize = 0x10; // CSeqSeqDescFlag
                pub const m_transition: usize = 0x1C; // CSeqTransition
                pub const m_nFrameRangeSequence: usize = 0x24; // int16
                pub const m_nFrameCount: usize = 0x26; // int16
                pub const m_flFPS: usize = 0x28; // float32
                pub const m_nSubCycles: usize = 0x2C; // int16
                pub const m_numLocalResults: usize = 0x2E; // int16
                pub const m_cmdLayerArray: usize = 0x30; // CUtlVector<CSeqCmdLayer>
                pub const m_eventArray: usize = 0x48; // CUtlVector<CAnimEventDefinition>
                pub const m_activityArray: usize = 0x60; // CUtlVector<CAnimActivity>
                pub const m_poseSettingArray: usize = 0x78; // CUtlVector<CSeqPoseSetting>
            }
            // Parent: None
            // Field count: 0
            pub mod CNmCachedPoseWriteTask {
            }
            // Parent: None
            // Field count: 0
            pub mod CNmModelSpaceBlendTask {
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod AnimationDecodeDebugDump_t {
                pub const m_processingType: usize = 0x0; // AnimationProcessingType_t
                pub const m_elems: usize = 0x8; // CUtlVector<AnimationDecodeDebugDumpElement_t>
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // SEQ_POSE_SETTING_ROTATION
            pub mod CSeqScaleSet {
                pub const m_sName: usize = 0x0; // CBufferString
                pub const m_bRootOffset: usize = 0x10; // bool
                pub const m_vRootOffset: usize = 0x14; // Vector
                pub const m_nLocalBoneArray: usize = 0x20; // CUtlVector<int16>
                pub const m_flBoneScaleArray: usize = 0x38; // CUtlVector<float32>
            }
            // Parent: None
            // Field count: 5
            pub mod IKTargetSettings_t {
                pub const m_TargetSource: usize = 0x0; // IKTargetSource
                pub const m_Bone: usize = 0x8; // IKBoneNameAndIndex_t
                pub const m_AnimgraphParameterNamePosition: usize = 0x18; // AnimParamID
                pub const m_AnimgraphParameterNameOrientation: usize = 0x1C; // AnimParamID
                pub const m_TargetCoordSystem: usize = 0x20; // IKTargetCoordinateSystem
            }
            // Parent: None
            // Field count: 0
            pub mod CNmAdditiveBlendTask {
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CSeqAutoLayer {
                pub const m_nLocalReference: usize = 0x0; // int16
                pub const m_nLocalPose: usize = 0x2; // int16
                pub const m_flags: usize = 0x4; // CSeqAutoLayerFlag
                pub const m_start: usize = 0xC; // float32
                pub const m_peak: usize = 0x10; // float32
                pub const m_tail: usize = 0x14; // float32
                pub const m_end: usize = 0x18; // float32
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CAnimSequenceParams {
                pub const m_flFadeInTime: usize = 0x0; // float32
                pub const m_flFadeOutTime: usize = 0x4; // float32
            }
            // Parent: None
            // Field count: 5
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MKV3TransferName
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            pub mod CAnimData {
                pub const m_name: usize = 0x10; // CBufferString
                pub const m_animArray: usize = 0x20; // CUtlVector<CAnimDesc>
                pub const m_decoderArray: usize = 0x38; // CUtlVector<CAnimDecoder>
                pub const m_nMaxUniqueFrameIndex: usize = 0x50; // int32
                pub const m_segmentArray: usize = 0x58; // CUtlVector<CAnimFrameSegment>
            }
            // Parent: None
            // Field count: 3
            pub mod IKSolverSettings_t {
                pub const m_SolverType: usize = 0x0; // IKSolverType
                pub const m_nNumIterations: usize = 0x4; // int32
                pub const m_EndEffectorRotationFixUpMode: usize = 0x8; // EIKEndEffectorRotationFixUpMode
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            pub mod CAnimTagManagerUpdater {
                pub const m_tags: usize = 0x38; // CUtlVector<CSmartPtr<CAnimTagBase>>
            }
        }
    }
}
