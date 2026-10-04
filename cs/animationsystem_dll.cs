// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-04 12:25:34.881424700 +07:00

namespace Source2Dumper.Schemas {
    // Module: animationsystem.dll
    // Class count: 86
    // Enum count: 143
    public static class AnimationsystemDll {
        // Alignment: 4
        // Member count: 2
        public enum PulseBestOutflowRules_t : uint {
            SORT_BY_NUMBER_OF_VALID_CRITERIA = 0x0,
            SORT_BY_OUTFLOW_INDEX = 0x1
        }
        // Alignment: 4
        // Member count: 4
        public enum PulseCursorCancelPriority_t : uint {
            None = 0x0,
            CancelOnSucceeded = 0x1,
            SoftCancel = 0x2,
            HardCancel = 0x3
        }
        // Alignment: 4
        // Member count: 2
        public enum PulseMethodCallMode_t : uint {
            SYNC_WAIT_FOR_COMPLETION = 0x0,
            ASYNC_FIRE_AND_FORGET = 0x1
        }
        // Alignment: 4
        // Member count: 2
        public enum PulseCursorWakePriority_t : uint {
            WakeElegantly = 0x0,
            WakeImmediate = 0x1
        }
        // Alignment: 4
        // Member count: 6
        public enum ParticleSetMethod_t : uint {
            PARTICLE_SET_REPLACE_VALUE = 0x0,
            PARTICLE_SET_SCALE_INITIAL_VALUE = 0x1,
            PARTICLE_SET_ADD_TO_INITIAL_VALUE = 0x2,
            PARTICLE_SET_RAMP_CURRENT_VALUE = 0x3,
            PARTICLE_SET_SCALE_CURRENT_VALUE = 0x4,
            PARTICLE_SET_ADD_TO_CURRENT_VALUE = 0x5
        }
        // Alignment: 4
        // Member count: 17
        public enum SeqCmd_t : uint {
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
        public enum CNmEventRelevance_t : uint {
            ClientOnly = 0x0,
            ServerOnly = 0x1,
            ClientAndServer = 0x2
        }
        // Alignment: 4
        // Member count: 4
        public enum BoneTransformSpace_t : uint {
            BoneTransformSpace_Invalid = unchecked((uint)-1),
            BoneTransformSpace_Parent = 0x0,
            BoneTransformSpace_Model = 0x1,
            BoneTransformSpace_World = 0x2
        }
        // Alignment: 4
        // Member count: 5
        public enum CAnimationGraphVisualizerPrimitiveType : uint {
            ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Text = 0x0,
            ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Sphere = 0x1,
            ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Line = 0x2,
            ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Pie = 0x3,
            ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Axis = 0x4
        }
        // Alignment: 1
        // Member count: 3
        public enum NmTransitionRule_t : byte {
            AllowTransition = 0x0,
            ConditionallyAllowTransition = 0x1,
            BlockTransition = 0x2
        }
        // Alignment: 4
        // Member count: 3
        public enum BinaryNodeTiming : uint {
            UseChild1 = 0x0,
            UseChild2 = 0x1,
            SyncChildren = 0x2
        }
        // Alignment: 1
        // Member count: 3
        public enum NmFollowBoneMode_t : byte {
            RotationAndTranslation = 0x0,
            RotationOnly = 0x1,
            TranslationOnly = 0x2
        }
        // Alignment: 4
        // Member count: 7
        public enum SolveIKChainAnimNodeDebugSetting : uint {
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
        public enum CNmParticleEvent__Type_t : uint {
            Create = 0x0,
            Create_CFG = 0x1
        }
        // Alignment: 4
        // Member count: 5
        public enum ParticleFloatBiasType_t : uint {
            PF_BIAS_TYPE_INVALID = unchecked((uint)-1),
            PF_BIAS_TYPE_STANDARD = 0x0,
            PF_BIAS_TYPE_GAIN = 0x1,
            PF_BIAS_TYPE_EXPONENTIAL = 0x2,
            PF_BIAS_TYPE_COUNT = 0x3
        }
        // Alignment: 1
        // Member count: 4
        public enum CNmTargetWarpNode__TargetUpdateRule_t : byte {
            None = 0x0,
            Recalculate = 0x1,
            Offset = 0x2,
            RecalculateOrOffset = 0x3
        }
        // Alignment: 1
        // Member count: 6
        public enum SharedMovementGait_t : byte {
            eInvalid = unchecked((byte)-1),
            eSlow = 0x0,
            eMedium = 0x1,
            eFast = 0x2,
            eVeryFast = 0x3,
            eCount = 0x4
        }
        // Alignment: 4
        // Member count: 3
        public enum OrientationWarpRootMotionSource_t : uint {
            eAnimationOrProcedural = 0x0,
            eAnimationOnly = 0x1,
            eProceduralOnly = 0x2
        }
        // Alignment: 4
        // Member count: 3
        public enum IKTargetCoordinateSystem : uint {
            IKTARGETCOORDINATESYSTEM_WorldSpace = 0x0,
            IKTARGETCOORDINATESYSTEM_ModelSpace = 0x1,
            IKTARGETCOORDINATESYSTEM_COUNT = 0x2
        }
        // Alignment: 4
        // Member count: 33
        public enum ParticleFloatType_t : uint {
            PF_TYPE_INVALID = unchecked((uint)-1),
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
        public enum CNmFloatAngleMathNode__Operation_t : byte {
            ClampTo180 = 0x0,
            ClampTo360 = 0x1,
            FlipHemisphere = 0x2,
            FlipHemisphereNegate = 0x3
        }
        // Alignment: 4
        // Member count: 3
        public enum VPhysXAggregateData_t__VPhysXFlagEnum_t : uint {
            FLAG_IS_POLYSOUP_GEOMETRY = 0x1,
            FLAG_LEVEL_COLLISION = 0x10,
            FLAG_IGNORE_SCALE_OBSOLETE_DO_NOT_USE = 0x20
        }
        // Alignment: 1
        // Member count: 5
        public enum CNmRootMotionOverrideNode__OverrideFlags_t : byte {
            AllowMoveX = 0x0,
            AllowMoveY = 0x1,
            AllowMoveZ = 0x2,
            AllowFacingPitch = 0x3,
            ListenForEvents = 0x4
        }
        // Alignment: 1
        // Member count: 23
        public enum NmEasingOperation_t : byte {
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
        public enum EIKEndEffectorRotationFixUpMode : uint {
            None = 0x0,
            MatchTargetOrientation = 0x1,
            LookAtTargetForward = 0x2,
            MaintainParentOrientation = 0x3,
            Count = 0x4
        }
        // Alignment: 4
        // Member count: 2
        public enum MatterialAttributeTagType_t : uint {
            MATERIAL_ATTRIBUTE_TAG_VALUE = 0x0,
            MATERIAL_ATTRIBUTE_TAG_COLOR = 0x1
        }
        // Alignment: 4
        // Member count: 6
        public enum PFNoiseTurbulence_t : uint {
            PF_NOISE_TURB_NONE = 0x0,
            PF_NOISE_TURB_HIGHLIGHT = 0x1,
            PF_NOISE_TURB_FEEDBACK = 0x2,
            PF_NOISE_TURB_LOOPY = 0x3,
            PF_NOISE_TURB_CONTRAST = 0x4,
            PF_NOISE_TURB_ALTERNATE = 0x5
        }
        // Alignment: 1
        // Member count: 4
        public enum NmTargetWarpAlgorithm_t : byte {
            Lerp = 0x0,
            Hermite = 0x1,
            HermiteFeaturePreserving = 0x2,
            Bezier = 0x3
        }
        // Alignment: 4
        // Member count: 5
        public enum ParticleColorBlendMode_t : uint {
            PARTICLEBLEND_DEFAULT = 0x0,
            PARTICLEBLEND_OVERLAY = 0x1,
            PARTICLEBLEND_DARKEN = 0x2,
            PARTICLEBLEND_LIGHTEN = 0x3,
            PARTICLEBLEND_MULTIPLY = 0x4
        }
        // Alignment: 4
        // Member count: 13
        public enum ParticleColorBlendType_t : uint {
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
        public enum NmTransitionRuleCondition_t : byte {
            AnyAllowed = 0x0,
            FullyAllowed = 0x1,
            ConditionallyAllowed = 0x2,
            Blocked = 0x3
        }
        // Alignment: 1
        // Member count: 8
        public enum ModelMeshBufferUsage_t : byte {
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
        public enum NmGraphDebugMode_t : uint {
            Off = 0x0,
            On = 0x1
        }
        // Alignment: 4
        // Member count: 2
        public enum TargetWarpTimingMethod : uint {
            ReachDestinationOnRootMotionEnd = 0x0,
            ReachDestinationOnWarpTagEnd = 0x1
        }
        // Alignment: 4
        // Member count: 6
        public enum ScriptedMoveTo_t : uint {
            eWait = 0x0,
            eMoveWithGait = 0x3,
            eTeleport = 0x4,
            eWaitFacing = 0x5,
            eObsoleteBackCompat1 = 0x1,
            eObsoleteBackCompat2 = 0x2
        }
        // Alignment: 4
        // Member count: 2
        public enum EDemoBoneSelectionMode : uint {
            CaptureAllBones = 0x0,
            CaptureSelectedBones = 0x1
        }
        // Alignment: 4
        // Member count: 2
        public enum StepPhase : uint {
            StepPhase_OnGround = 0x0,
            StepPhase_InAir = 0x1
        }
        // Alignment: 4
        // Member count: 26
        public enum FlexOpCode_t : uint {
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
        public enum NmCachedValueMode_t : uint {
            OnEntry = 0x0,
            OnExit = 0x1
        }
        // Alignment: 4
        // Member count: 2
        public enum AnimNodeNetworkMode : uint {
            ServerAuthoritative = 0x0,
            ClientSimulate = 0x1
        }
        // Alignment: 4
        // Member count: 6
        public enum VPhysXBodyPart_t__VPhysXFlagEnum_t : uint {
            FLAG_STATIC = 0x1,
            FLAG_KINEMATIC = 0x2,
            FLAG_JOINT = 0x4,
            FLAG_MASS = 0x8,
            FLAG_ALWAYS_DYNAMIC_ON_CLIENT = 0x10,
            FLAG_DISABLE_CCD = 0x20
        }
        // Alignment: 1
        // Member count: 9
        public enum AnimParamType_t : byte {
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
        public enum NmEasingFunction_t : byte {
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
        public enum ParticleModelType_t : uint {
            PM_TYPE_INVALID = 0x0,
            PM_TYPE_NAMED_VALUE_MODEL = 0x1,
            PM_TYPE_NAMED_VALUE_EHANDLE = 0x2,
            PM_TYPE_CONTROL_POINT = 0x3,
            PM_TYPE_COUNT = 0x4
        }
        // Alignment: 4
        // Member count: 3
        public enum IKTargetSource : uint {
            IKTARGETSOURCE_Bone = 0x0,
            IKTARGETSOURCE_AnimgraphParameter = 0x1,
            IKTARGETSOURCE_COUNT = 0x2
        }
        // Alignment: 4
        // Member count: 15
        public enum PermModelInfo_t__FlagEnum : uint {
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
        public enum CNmFloatMathNode__Operator_t : byte {
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
        public enum CNmSyncEventIndexConditionNode__TriggerMode_t : byte {
            ExactlyAtEventIndex = 0x0,
            GreaterThanEqualToEventIndex = 0x1
        }
        // Alignment: 4
        // Member count: 5
        public enum ParticleFloatRoundType_t : uint {
            PF_ROUND_TYPE_INVALID = unchecked((uint)-1),
            PF_ROUND_TYPE_NEAREST = 0x0,
            PF_ROUND_TYPE_FLOOR = 0x1,
            PF_ROUND_TYPE_CEIL = 0x2,
            PF_ROUND_TYPE_COUNT = 0x3
        }
        // Alignment: 4
        // Member count: 4
        public enum PFNoiseType_t : uint {
            PF_NOISE_TYPE_PERLIN = 0x0,
            PF_NOISE_TYPE_SIMPLEX = 0x1,
            PF_NOISE_TYPE_WORLEY = 0x2,
            PF_NOISE_TYPE_CURL = 0x3
        }
        // Alignment: 4
        // Member count: 3
        public enum ParticleDirectionNoiseType_t : uint {
            PARTICLE_DIR_NOISE_PERLIN = 0x0,
            PARTICLE_DIR_NOISE_CURL = 0x1,
            PARTICLE_DIR_NOISE_WORLEY_BASIC = 0x2
        }
        // Alignment: 4
        // Member count: 3
        public enum AnimParamNetworkSetting : uint {
            Auto = 0x0,
            AlwaysNetwork = 0x1,
            NeverNetwork = 0x2
        }
        // Alignment: 4
        // Member count: 4
        public enum MorphFlexControllerRemapType_t : uint {
            MORPH_FLEXCONTROLLER_REMAP_PASSTHRU = 0x0,
            MORPH_FLEXCONTROLLER_REMAP_2WAY = 0x1,
            MORPH_FLEXCONTROLLER_REMAP_NWAY = 0x2,
            MORPH_FLEXCONTROLLER_REMAP_EYELID = 0x3
        }
        // Alignment: 4
        // Member count: 8
        public enum MeshDrawPrimitiveFlags_t : uint {
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
        public enum TargetWarpAngleMode_t : uint {
            eFacingHeading = 0x0,
            eMoveHeading = 0x1
        }
        // Alignment: 1
        // Member count: 2
        public enum NmIKBlendMode_t : byte {
            Effector = 0x0,
            Pose = 0x1
        }
        // Alignment: 4
        // Member count: 4
        public enum ModelBoneFlexComponent_t : uint {
            MODEL_BONE_FLEX_INVALID = unchecked((uint)-1),
            MODEL_BONE_FLEX_TX = 0x0,
            MODEL_BONE_FLEX_TY = 0x1,
            MODEL_BONE_FLEX_TZ = 0x2
        }
        // Alignment: 1
        // Member count: 2
        public enum CNmStateNode__TimedEvent_t__Comparison_t : byte {
            LessThanEqual = 0x0,
            GreaterThanEqual = 0x1
        }
        // Alignment: 1
        // Member count: 3
        public enum PoseType_t : byte {
            POSETYPE_STATIC = 0x0,
            POSETYPE_DYNAMIC = 0x1,
            POSETYPE_INVALID = 0xFF
        }
        // Alignment: 1
        // Member count: 2
        public enum CNmRootMotionData__SamplingMode_t : byte {
            Delta = 0x0,
            WorldSpace = 0x1
        }
        // Alignment: 1
        // Member count: 9
        public enum NmEventConditionRules_t : byte {
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
        public enum AnimationType_t : uint {
            ANIMATION_TYPE_FIXED_RATE = 0x0,
            ANIMATION_TYPE_FIT_LIFETIME = 0x1,
            ANIMATION_TYPE_MANUAL_FRAMES = 0x2
        }
        // Alignment: 4
        // Member count: 37
        public enum AnimValueSource : uint {
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
        public enum CNmTimeConditionNode__Operator_t : byte {
            LessThan = 0x0,
            LessThanEqual = 0x1,
            GreaterThan = 0x2,
            GreaterThanEqual = 0x3
        }
        // Alignment: 4
        // Member count: 3
        public enum LinearRootMotionBlendMode_t : uint {
            LERP = 0x0,
            NLERP = 0x1,
            SLERP = 0x2
        }
        // Alignment: 4
        // Member count: 1
        public enum RagdollPoseControl : uint {
            Absolute = 0x0
        }
        // Alignment: 4
        // Member count: 6
        public enum IKSolverType : uint {
            IKSOLVER_Perlin = 0x0,
            IKSOLVER_TwoBone = 0x1,
            IKSOLVER_Fabrik = 0x2,
            IKSOLVER_DogLeg3Bone = 0x3,
            IKSOLVER_CCD = 0x4,
            IKSOLVER_COUNT = 0x5
        }
        // Alignment: 4
        // Member count: 2
        public enum TargetWarpCorrectionMethod : uint {
            ScaleMotion = 0x0,
            AddCorrectionDelta = 0x1
        }
        // Alignment: 4
        // Member count: 2
        public enum TargetSelectorAngleMode_t : uint {
            eFacingHeading = 0x0,
            eMoveHeading = 0x1
        }
        // Alignment: 4
        // Member count: 2
        public enum Blend2DMode : uint {
            Blend2DMode_General = 0x0,
            Blend2DMode_Directional = 0x1
        }
        // Alignment: 1
        // Member count: 3
        public enum HandshakeTagState_t : byte {
            eInactive = 0x0,
            eActive = 0x1,
            eMomentarilyInactive = 0x2
        }
        // Alignment: 4
        // Member count: 3
        public enum ChoiceChangeMethod : uint {
            OnReset = 0x0,
            OnCycleEnd = 0x1,
            OnResetOrCycleEnd = 0x2
        }
        // Alignment: 4
        // Member count: 2
        public enum ChoiceBlendMethod : uint {
            SingleBlendTime = 0x0,
            PerChoiceBlendTimes = 0x1
        }
        // Alignment: 4
        // Member count: 4
        public enum VPhysXConstraintParams_t__EnumFlags0_t : uint {
            FLAG0_SHIFT_INTERPENETRATE = 0x0,
            FLAG0_SHIFT_CONSTRAIN = 0x1,
            FLAG0_SHIFT_BREAKABLE_FORCE = 0x2,
            FLAG0_SHIFT_BREAKABLE_TORQUE = 0x3
        }
        // Alignment: 4
        // Member count: 9
        public enum ParticleFloatMapType_t : uint {
            PF_MAP_TYPE_INVALID = unchecked((uint)-1),
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
        public enum AnimParamVectorType_t : uint {
            ANIMPARAM_VECTOR_TYPE_NONE = 0x0,
            ANIMPARAM_VECTOR_TYPE_POSITION_WS = 0x1,
            ANIMPARAM_VECTOR_TYPE_POSITION_LS = 0x2,
            ANIMPARAM_VECTOR_TYPE_DIRECTION_WS = 0x3,
            ANIMPARAM_VECTOR_TYPE_DIRECTION_LS = 0x4
        }
        // Alignment: 1
        // Member count: 3
        public enum CNmCurrentSyncEventNode__InfoType_t : byte {
            IndexAndPercentage = 0x0,
            IndexOnly = 0x1,
            PercentageOnly = 0x2
        }
        // Alignment: 4
        // Member count: 4
        public enum BlendKeyType : uint {
            BlendKey_UserValue = 0x0,
            BlendKey_Velocity = 0x1,
            BlendKey_Distance = 0x2,
            BlendKey_RemainingDistance = 0x3
        }
        // Alignment: 4
        // Member count: 5
        public enum StateActionBehavior : uint {
            STATETAGBEHAVIOR_ACTIVE_WHILE_CURRENT = 0x0,
            STATETAGBEHAVIOR_FIRE_ON_ENTER = 0x1,
            STATETAGBEHAVIOR_FIRE_ON_EXIT = 0x2,
            STATETAGBEHAVIOR_FIRE_ON_ENTER_AND_EXIT = 0x3,
            STATETAGBEHAVIOR_ACTIVE_WHILE_FULLY_BLENDED = 0x4
        }
        // Alignment: 1
        // Member count: 4
        public enum NmRootMotionBlendMode_t : byte {
            Blend = 0x0,
            Additive = 0x1,
            IgnoreSource = 0x2,
            IgnoreTarget = 0x3
        }
        // Alignment: 1
        // Member count: 7
        public enum NmFootPhaseCondition_t : byte {
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
        public enum ModelSkeletonData_t__BoneFlags_t : uint {
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
        public enum GPUParticleCollisionMode_t : uint {
            PARTICLE_GPU_COLLISION_MODE_RT = 0x0,
            PARTICLE_GPU_COLLISION_MODE_DEPTH = 0x1,
            PARTICLE_GPU_COLLISION_MODE_HYBRID = 0x2
        }
        // Alignment: 4
        // Member count: 4
        public enum MorphBundleType_t : uint {
            MORPH_BUNDLE_TYPE_NONE = 0x0,
            MORPH_BUNDLE_TYPE_POSITION_SPEED = 0x1,
            MORPH_BUNDLE_TYPE_NORMAL_WRINKLE = 0x2,
            MORPH_BUNDLE_TYPE_COUNT = 0x3
        }
        // Alignment: 1
        // Member count: 2
        public enum CNmIDComparisonNode__Comparison_t : byte {
            Matches = 0x0,
            DoesntMatch = 0x1
        }
        // Alignment: 1
        // Member count: 3
        public enum NmPoseBlendMode_t : byte {
            Overlay = 0x0,
            Additive = 0x1,
            ModelSpace = 0x2
        }
        // Alignment: 4
        // Member count: 4
        public enum ParticleFloatInputMode_t : uint {
            PF_INPUT_MODE_INVALID = unchecked((uint)-1),
            PF_INPUT_MODE_CLAMPED = 0x0,
            PF_INPUT_MODE_LOOPED = 0x1,
            PF_INPUT_MODE_COUNT = 0x2
        }
        // Alignment: 4
        // Member count: 5
        public enum ResetCycleOption : uint {
            Beginning = 0x0,
            SameCycleAsSource = 0x1,
            InverseSourceCycle = 0x2,
            FixedValue = 0x3,
            SameTimeAsSource = 0x4
        }
        // Alignment: 1
        // Member count: 6
        public enum CNmVectorInfoNode__Info_t : byte {
            X = 0x0,
            Y = 0x1,
            Z = 0x2,
            Length = 0x3,
            AngleHorizontal = 0x4,
            AngleVertical = 0x5
        }
        // Alignment: 1
        // Member count: 3
        public enum TagActionStatus : byte {
            Inactive = 0x0,
            Active = 0x1,
            Fired = 0x2
        }
        // Alignment: 4
        // Member count: 4
        public enum IKChannelMode : uint {
            TwoBone = 0x0,
            TwoBone_Translate = 0x1,
            OneBone = 0x2,
            OneBone_Translate = 0x3
        }
        // Alignment: 1
        // Member count: 9
        public enum NmGraphValueType_t : byte {
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
        public enum ParticleFloatRandomMode_t : uint {
            PF_RANDOM_MODE_INVALID = unchecked((uint)-1),
            PF_RANDOM_MODE_CONSTANT = 0x0,
            PF_RANDOM_MODE_VARYING = 0x1,
            PF_RANDOM_MODE_COUNT = 0x2
        }
        // Alignment: 4
        // Member count: 4
        public enum PFNoiseModifier_t : uint {
            PF_NOISE_MODIFIER_NONE = 0x0,
            PF_NOISE_MODIFIER_LINES = 0x1,
            PF_NOISE_MODIFIER_CLUMPS = 0x2,
            PF_NOISE_MODIFIER_RINGS = 0x3
        }
        // Alignment: 4
        // Member count: 21
        public enum ParticleVecType_t : uint {
            PVEC_TYPE_INVALID = unchecked((uint)-1),
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
        public enum NmFootPhase_t : byte {
            LeftFootDown = 0x0,
            RightFootPassing = 0x1,
            RightFootDown = 0x2,
            LeftFootPassing = 0x3,
            None = 0x4
        }
        // Alignment: 4
        // Member count: 8
        public enum CNmTargetInfoNode__Info_t : uint {
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
        public enum FootstepLandedFootSoundType_t : uint {
            FOOTSOUND_Left = 0x0,
            FOOTSOUND_Right = 0x1,
            FOOTSOUND_UseOverrideSound = 0x2
        }
        // Alignment: 4
        // Member count: 2
        public enum FootLockSubVisualization : uint {
            FOOTLOCKSUBVISUALIZATION_ReachabilityAnalysis = 0x0,
            FOOTLOCKSUBVISUALIZATION_IKSolve = 0x1
        }
        // Alignment: 4
        // Member count: 5
        public enum CNmSoundEvent__Position_t : uint {
            None = 0x0,
            World = 0x1,
            EntityPos = 0x2,
            EntityEyePos = 0x3,
            EntityAttachment = 0x4
        }
        // Alignment: 1
        // Member count: 4
        public enum FootstepJumpPhase_t : byte {
            Unknown = 0x0,
            NotJumping = 0x1,
            Jumping = 0x2,
            Landing = 0x4
        }
        // Alignment: 4
        // Member count: 2
        public enum NmFrameSnapEventMode_t : uint {
            Floor = 0x0,
            Round = 0x1
        }
        // Alignment: 4
        // Member count: 3
        public enum FootPinningTimingSource : uint {
            FootMotion = 0x0,
            Tag = 0x1,
            Parameter = 0x2
        }
        // Alignment: 4
        // Member count: 4
        public enum DampingSpeedFunction : uint {
            NoDamping = 0x0,
            Constant = 0x1,
            Spring = 0x2,
            AsymmetricSpring = 0x3
        }
        // Alignment: 4
        // Member count: 6
        public enum AnimationProcessingType_t : uint {
            ANIMATION_PROCESSING_SERVER_SIMULATION = 0x0,
            ANIMATION_PROCESSING_CLIENT_SIMULATION = 0x1,
            ANIMATION_PROCESSING_CLIENT_PREDICTION = 0x2,
            ANIMATION_PROCESSING_CLIENT_INTERPOLATION = 0x3,
            ANIMATION_PROCESSING_CLIENT_RENDER = 0x4,
            ANIMATION_PROCESSING_MAX = 0x5
        }
        // Alignment: 4
        // Member count: 3
        public enum JiggleBoneSimSpace : uint {
            SimSpace_Local = 0x0,
            SimSpace_Model = 0x1,
            SimSpace_World = 0x2
        }
        // Alignment: 4
        // Member count: 2
        public enum StanceOverrideMode : uint {
            Sequence = 0x0,
            Node = 0x1
        }
        // Alignment: 4
        // Member count: 2
        public enum IkEndEffectorType : uint {
            IkEndEffector_Attachment = 0x0,
            IkEndEffector_Bone = 0x1
        }
        // Alignment: 2
        // Member count: 3
        public enum AnimScriptType : ushort {
            ANIMSCRIPT_TYPE_INVALID = unchecked((ushort)-1),
            ANIMSCRIPT_FUSE_GENERAL = 0x0,
            ANIMSCRIPT_FUSE_STATEMACHINE = 0x1
        }
        // Alignment: 1
        // Member count: 3
        public enum CNmTimeConditionNode__ComparisonType_t : byte {
            PercentageThroughState = 0x0,
            PercentageThroughSyncEvent = 0x1,
            ElapsedTime = 0x2
        }
        // Alignment: 4
        // Member count: 4
        public enum SeqPoseSetting_t : uint {
            SEQ_POSE_SETTING_CONSTANT = 0x0,
            SEQ_POSE_SETTING_ROTATION = 0x1,
            SEQ_POSE_SETTING_POSITION = 0x2,
            SEQ_POSE_SETTING_VELOCITY = 0x3
        }
        // Alignment: 4
        // Member count: 13
        public enum AnimParamButton_t : uint {
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
        public enum SelectorTagBehavior_t : uint {
            SelectorTagBehavior_OnWhileCurrent = 0x0,
            SelectorTagBehavior_OffWhenFinished = 0x1,
            SelectorTagBehavior_OffBeforeFinished = 0x2
        }
        // Alignment: 4
        // Member count: 4
        public enum HandshakeTagType_t : uint {
            eInvalid = unchecked((uint)-1),
            eTask = 0x0,
            eMovement = 0x1,
            eCount = 0x2
        }
        // Alignment: 4
        // Member count: 4
        public enum OrientationWarpTargetOffsetMode_t : uint {
            eLiteralValue = 0x0,
            eParameter = 0x1,
            eAnimationMovementHeading = 0x2,
            eAnimationMovementHeadingAtEnd = 0x3
        }
        // Alignment: 4
        // Member count: 3
        public enum OrientationWarpMode_t : uint {
            eInvalid = 0x0,
            eAngle = 0x1,
            eWorldPosition = 0x2
        }
        // Alignment: 4
        // Member count: 5
        public enum ParticleTransformType_t : uint {
            PT_TYPE_INVALID = 0x0,
            PT_TYPE_NAMED_VALUE = 0x1,
            PT_TYPE_CONTROL_POINT = 0x2,
            PT_TYPE_CONTROL_POINT_RANGE = 0x3,
            PT_TYPE_COUNT = 0x4
        }
        // Alignment: 4
        // Member count: 18
        public enum ParticleAttachment_t : uint {
            PATTACH_INVALID = unchecked((uint)-1),
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
        public enum CNmEventTargetEntity_t : uint {
            Self = 0x0,
            Weapon = 0x1,
            HeldItem = 0x2,
            Custom = 0x3
        }
        // Alignment: 4
        // Member count: 3
        public enum FieldNetworkOption : uint {
            Auto = 0x0,
            ForceEnable = 0x1,
            ForceDisable = 0x2
        }
        // Alignment: 1
        // Member count: 6
        public enum NmGraphEventTypeCondition_t : byte {
            Entry = 0x0,
            FullyInState = 0x1,
            Exit = 0x2,
            Timed = 0x3,
            Generic = 0x4,
            Any = 0x5
        }
        // Alignment: 1
        // Member count: 10
        public enum CNmTransitionNode__TransitionOptions_t : byte {
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
        public enum CNmFloatComparisonNode__Comparison_t : byte {
            GreaterThanEqual = 0x0,
            LessThanEqual = 0x1,
            NearEqual = 0x2,
            GreaterThan = 0x3,
            LessThan = 0x4
        }
        // Alignment: 4
        // Member count: 3
        public enum VPhysXJoint_t__Flags_t : uint {
            JOINT_FLAGS_NONE = 0x0,
            JOINT_FLAGS_BODY1_FIXED = 0x1,
            JOINT_FLAGS_USE_BLOCK_SOLVER = 0x2
        }
        // Alignment: 4
        // Member count: 4
        public enum ScriptedHeldWeaponBehavior_t : uint {
            eInvalid = unchecked((uint)-1),
            eHolster = 0x0,
            eDeploy = 0x1,
            eDrop = 0x2
        }
        // Alignment: 1
        // Member count: 3
        public enum VelocityMetricMode : byte {
            DirectionOnly = 0x0,
            MagnitudeOnly = 0x1,
            DirectionAndMagnitude = 0x2
        }
        // Alignment: 1
        // Member count: 5
        public enum FacingMode : byte {
            FacingMode_Invalid = 0x0,
            FacingMode_Manual = 0x1,
            FacingMode_Path = 0x2,
            FacingMode_LookTarget = 0x3,
            FacingMode_ManualPosition = 0x4
        }
        // Alignment: 1
        // Member count: 3
        public enum VertexAlbedoFormat_t : byte {
            VERTEX_ALBEDO_NONE = 0x0,
            VERTEX_ALBEDO_8888 = 0x1,
            VERTEX_ALBEDO_565 = 0x2
        }
        // Alignment: 4
        // Member count: 4
        public enum AimMatrixBlendMode : uint {
            AimMatrixBlendMode_None = 0x0,
            AimMatrixBlendMode_Additive = 0x1,
            AimMatrixBlendMode_ModelSpaceAdditive = 0x2,
            AimMatrixBlendMode_BoneMask = 0x3
        }
        // Alignment: 4
        // Member count: 7
        public enum AnimationSnapshotType_t : uint {
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
        public enum NmTargetWarpRule_t : byte {
            WarpXY = 0x0,
            WarpZ = 0x1,
            WarpXYZ = 0x2,
            RotationOnly = 0x3,
            FixedSection = 0x4
        }
        // Alignment: 4
        // Member count: 8
        public enum FootFallTagFoot_t : uint {
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
        public enum ChoiceMethod : uint {
            WeightedRandom = 0x0,
            WeightedRandomNoRepeat = 0x1,
            Iterate = 0x2,
            IterateRandom = 0x3
        }
        // Alignment: 4
        // Member count: 14
        public enum AnimVectorSource : uint {
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
        public enum IkTargetType : uint {
            IkTarget_Attachment = 0x0,
            IkTarget_Bone = 0x1,
            IkTarget_Parameter_ModelSpace = 0x2,
            IkTarget_Parameter_WorldSpace = 0x3
        }
        // Alignment: 1
        // Member count: 3
        public enum RenderMeshSlotType_t : byte {
            RENDERMESH_SLOT_INVALID = unchecked((byte)-1),
            RENDERMESH_SLOT_PER_VERTEX = 0x0,
            RENDERMESH_SLOT_PER_INSTANCE = 0x1
        }
        // Alignment: 4
        // Member count: 4
        public enum BoneMaskBlendSpace : uint {
            BlendSpace_Parent = 0x0,
            BlendSpace_Model = 0x1,
            BlendSpace_Model_RotationOnly = 0x2,
            BlendSpace_Model_TranslationOnly = 0x3
        }
        // Alignment: 4
        // Member count: 10
        public enum MovementCapability_t : uint {
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
        public enum ModelConfigAttachmentType_t : uint {
            MODEL_CONFIG_ATTACHMENT_INVALID = unchecked((uint)-1),
            MODEL_CONFIG_ATTACHMENT_BONE_OR_ATTACHMENT = 0x0,
            MODEL_CONFIG_ATTACHMENT_ROOT_RELATIVE = 0x1,
            MODEL_CONFIG_ATTACHMENT_BONEMERGE = 0x2,
            MODEL_CONFIG_ATTACHMENT_COUNT = 0x3
        }
        // Alignment: 4
        // Member count: 2
        public enum BinaryNodeChildOption : uint {
            Child1 = 0x0,
            Child2 = 0x1
        }
        // Alignment: 4
        // Member count: 7
        public enum NPCPhysicsHullType_t : uint {
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
        public enum JumpCorrectionMethod : uint {
            ScaleMotion = 0x0,
            AddCorrectionDelta = 0x1
        }
        // Alignment: 4
        // Member count: 2
        public enum MoodType_t : uint {
            eMoodType_Head = 0x0,
            eMoodType_Body = 0x1
        }
        // Parent: None
        // Field count: 0
        public static class CPulse_ResumePoint {
        }
        // Parent: None
        // Field count: 0
        public static class CParticleBindingRealPulse {
        }
        // Parent: None
        // Field count: 4
        public static class CPulse_OutflowConnection {
            public const nint m_SourceOutflowName = 0x0; // PulseSymbol_t
            public const nint m_nDestChunk = 0x10; // PulseRuntimeChunkIndex_t
            public const nint m_nInstruction = 0x14; // int32
            public const nint m_OutflowRegisterMap = 0x18; // PulseRegisterMap_t
        }
        // Parent: None
        // Field count: 0
        public static class CBasePulseGraphInstance {
        }
        // Parent: None
        // Field count: 0
        public static class SignatureOutflow_Resume {
        }
        // Parent: None
        // Field count: 0
        public static class SignatureOutflow_Continue {
        }
        // Parent: None
        // Field count: 0
        public static class CParticleCollectionBindingInstance {
        }
        // Parent: None
        // Field count: 1
        public static class CPulseCell_IsRequirementValid__Criteria_t {
            public const nint m_bIsValid = 0x0; // bool
        }
        // Parent: None
        // Field count: 1
        public static class CPulseCell_Unknown {
            public const nint m_UnknownKeys = 0x48; // KeyValues3
        }
        // Parent: None
        // Field count: 1
        public static class CPulseCell_LimitCount__Criteria_t {
            public const nint m_bLimitCountPasses = 0x0; // bool
        }
        // Parent: None
        // Field count: 0
        public static class CPulseExecCursor {
        }
        // Parent: None
        // Field count: 12
        public static class CNmFootIKTask {
            public const nint m_nLeftEffectorBoneIdx = 0x70; // int32
            public const nint m_nRightEffectorBoneIdx = 0x74; // int32
            public const nint m_leftTargetTransform = 0x80; // CTransform
            public const nint m_rightTargetTransform = 0xA0; // CTransform
            public const nint m_nLeftTargetBoneIdx = 0xC0; // int32
            public const nint m_nRightTargetBoneIdx = 0xC4; // int32
            public const nint m_leftTarget = 0xD0; // CNmTarget
            public const nint m_rightTarget = 0x100; // CNmTarget
            public const nint m_blendMode = 0x130; // NmIKBlendMode_t
            public const nint m_flBlendWeight = 0x134; // float32
            public const nint m_bIsTargetInWorldSpace = 0x138; // bool
            public const nint m_bIsRunningFromDeserializedData = 0x139; // bool
        }
        // Parent: None
        // Field count: 0
        public static class CNmScaleTask {
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CAnimNodePath {
            public const nint m_path = 0x0; // AnimNodeID[11]
            public const nint m_nCount = 0x2C; // int32
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class AnimNodeOutputID {
            public const nint m_id = 0x0; // uint32
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
        public static class CSeqBoneMaskList {
            public const nint m_sName = 0x0; // CBufferString
            public const nint m_nLocalBoneArray = 0x10; // CUtlVector<int16>
            public const nint m_flBoneWeightArray = 0x28; // CUtlVector<float32>
            public const nint m_flDefaultMorphCtrlWeight = 0x40; // float32
            public const nint m_morphCtrlWeightArray = 0x48; // CUtlVector<std::pair<CBufferString,float32>>
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class SampleCode {
            public const nint m_subCode = 0x0; // uint8[8]
        }
        // Parent: None
        // Field count: 6
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class AnimationDecodeDebugDumpElement_t {
            public const nint m_nEntityIndex = 0x0; // int32
            public const nint m_modelName = 0x8; // CUtlString
            public const nint m_poseParams = 0x10; // CUtlVector<CUtlString>
            public const nint m_decodeOps = 0x28; // CUtlVector<CUtlString>
            public const nint m_internalOps = 0x40; // CUtlVector<CUtlString>
            public const nint m_decodedAnims = 0x58; // CUtlVector<CUtlString>
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class ConfigIndex {
            public const nint m_nGroup = 0x0; // uint16
            public const nint m_nConfig = 0x2; // uint16
        }
        // Parent: None
        // Field count: 11
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CSeqSeqDescFlag {
            public const nint m_bLooping = 0x0; // bool
            public const nint m_bSnap = 0x1; // bool
            public const nint m_bAutoplay = 0x2; // bool
            public const nint m_bPost = 0x3; // bool
            public const nint m_bHidden = 0x4; // bool
            public const nint m_bMulti = 0x5; // bool
            public const nint m_bLegacyDelta = 0x6; // bool
            public const nint m_bLegacyWorldspace = 0x7; // bool
            public const nint m_bLegacyCyclepose = 0x8; // bool
            public const nint m_bLegacyRealtime = 0x9; // bool
            public const nint m_bModelDoc = 0xA; // bool
        }
        // Parent: None
        // Field count: 1
        public static class IKBoneNameAndIndex_t {
            public const nint m_Name = 0x0; // CUtlString
        }
        // Parent: None
        // Field count: 0
        public static class CNmPoseNode__CDefinition {
        }
        // Parent: None
        // Field count: 0
        public static class CNmBlendTask {
        }
        // Parent: None
        // Field count: 0
        public static class CNmVectorValueNode__CDefinition {
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CSeqTransition {
            public const nint m_flFadeInTime = 0x0; // float32
            public const nint m_flFadeOutTime = 0x4; // float32
        }
        // Parent: None
        // Field count: 0
        public static class CNmPoseTask {
        }
        // Parent: None
        // Field count: 0
        public static class CNmCachedPoseReadTask {
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CAnimParamHandle {
            public const nint m_type = 0x0; // AnimParamType_t
            public const nint m_index = 0x1; // uint8
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class AnimNodeID {
            public const nint m_id = 0x0; // uint32
        }
        // Parent: None
        // Field count: 8
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CSeqAutoLayerFlag {
            public const nint m_bPost = 0x0; // bool
            public const nint m_bSpline = 0x1; // bool
            public const nint m_bXFade = 0x2; // bool
            public const nint m_bNoBlend = 0x3; // bool
            public const nint m_bLocal = 0x4; // bool
            public const nint m_bPose = 0x5; // bool
            public const nint m_bFetchFrame = 0x6; // bool
            public const nint m_bSubtract = 0x7; // bool
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class AnimStateID {
            public const nint m_id = 0x0; // uint32
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
        public static class CSeqPoseSetting {
            public const nint m_sPoseParameter = 0x0; // CBufferString
            public const nint m_sAttachment = 0x10; // CBufferString
            public const nint m_sReferenceSequence = 0x20; // CBufferString
            public const nint m_flValue = 0x30; // float32
            public const nint m_bX = 0x34; // bool
            public const nint m_bY = 0x35; // bool
            public const nint m_bZ = 0x36; // bool
            public const nint m_eType = 0x38; // int32
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CDrawCullingData {
            public const nint m_ConeAxis = 0x0; // int8[3]
            public const nint m_ConeCutoff = 0x3; // int8
        }
        // Parent: None
        // Field count: 6
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CSeqMultiFetchFlag {
            public const nint m_bRealtime = 0x0; // bool
            public const nint m_bCylepose = 0x1; // bool
            public const nint m_b0D = 0x2; // bool
            public const nint m_b1D = 0x3; // bool
            public const nint m_b2D = 0x4; // bool
            public const nint m_b2D_TRI = 0x5; // bool
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CSolveIKTargetHandle_t {
            public const nint m_positionHandle = 0x0; // CAnimParamHandle
            public const nint m_orientationHandle = 0x2; // CAnimParamHandle
        }
        // Parent: None
        // Field count: 0
        public static class CNmZeroPoseTask {
        }
        // Parent: None
        // Field count: 0
        public static class CNmBlendTaskBase {
        }
        // Parent: None
        // Field count: 33
        public static class CNewParticleEffect {
            public const nint m_pNext = 0x10; // CNewParticleEffect*
            public const nint m_pPrev = 0x18; // CNewParticleEffect*
            public const nint m_pParticles = 0x20; // IParticleCollection*
            public const nint m_pDebugName = 0x28; // char*
            public const nint m_bDontRemove = 0x0; // bitfield:1
            public const nint m_bRemove = 0x0; // bitfield:1
            public const nint m_bNeedsBBoxUpdate = 0x0; // bitfield:1
            public const nint m_bIsFirstFrame = 0x0; // bitfield:1
            public const nint m_bAutoUpdateBBox = 0x0; // bitfield:1
            public const nint m_bAllocated = 0x0; // bitfield:1
            public const nint m_bSimulate = 0x0; // bitfield:1
            public const nint m_bShouldPerformCullCheck = 0x0; // bitfield:1
            public const nint m_bForceNoDraw = 0x0; // bitfield:1
            public const nint m_bSuppressScreenSpaceEffect = 0x0; // bitfield:1
            public const nint m_bShouldSave = 0x0; // bitfield:1
            public const nint m_bShouldSimulateDuringGamePaused = 0x0; // bitfield:1
            public const nint m_bShouldCheckFoW = 0x0; // bitfield:1
            public const nint m_bIsAsyncCreate = 0x0; // bitfield:1
            public const nint m_bFreezeTransitionActive = 0x0; // bitfield:1
            public const nint m_bFreezeTargetState = 0x0; // bitfield:1
            public const nint m_bCanFreeze = 0x0; // bitfield:1
            public const nint m_vSortOrigin = 0x40; // Vector
            public const nint m_flScale = 0x4C; // float32
            public const nint m_hOwner = 0x50; // PARTICLE_EHANDLE__*
            public const nint m_pOwningParticleProperty = 0x58; // CParticleProperty*
            public const nint m_flFreezeTransitionStart = 0x70; // float32
            public const nint m_flFreezeTransitionDuration = 0x74; // float32
            public const nint m_flFreezeTransitionOverride = 0x78; // float32
            public const nint m_LastMin = 0x7C; // Vector
            public const nint m_LastMax = 0x88; // Vector
            public const nint m_nSplitScreenUser = 0x94; // CSplitScreenSlot
            public const nint m_vecAggregationCenter = 0x98; // Vector
            public const nint m_RefCount = 0xD0; // int32
        }
        // Parent: None
        // Field count: 0
        public static class CNmReferencePoseTask {
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
        public static class CSeqSynthAnimDesc {
            public const nint m_sName = 0x0; // CBufferString
            public const nint m_flags = 0x10; // CSeqSeqDescFlag
            public const nint m_transition = 0x1C; // CSeqTransition
            public const nint m_nLocalBaseReference = 0x24; // int16
            public const nint m_nLocalBoneMask = 0x26; // int16
            public const nint m_activityArray = 0x28; // CUtlVector<CAnimActivity>
        }
        // Parent: None
        // Field count: 0
        public static class IAnimationGraphInstance {
        }
        // Parent: None
        // Field count: 9
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class AnimationSnapshotBase_t {
            public const nint m_flRealTime = 0x0; // float32
            public const nint m_rootToWorld = 0x10; // matrix3x4a_t
            public const nint m_bBonesInWorldSpace = 0x40; // bool
            public const nint m_boneSetupMask = 0x48; // CUtlVector<uint32>
            public const nint m_boneTransforms = 0x60; // CUtlVector<matrix3x4a_t>
            public const nint m_flexControllers = 0x78; // CUtlVector<float32>
            public const nint m_SnapshotType = 0x90; // AnimationSnapshotType_t
            public const nint m_bHasDecodeDump = 0x94; // bool
            public const nint m_DecodeDump = 0x98; // AnimationDecodeDebugDumpElement_t
        }
        // Parent: None
        // Field count: 0
        public static class CNmBoneMaskValueNode__CDefinition {
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
        public static class CSeqPoseParamDesc {
            public const nint m_sName = 0x0; // CBufferString
            public const nint m_flStart = 0x10; // float32
            public const nint m_flEnd = 0x14; // float32
            public const nint m_flLoop = 0x18; // float32
            public const nint m_bLooping = 0x1C; // bool
        }
        // Parent: None
        // Field count: 7
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MKV3TransferName
        // MGetKV3ClassDefaults
        public static class CAnimMovement {
            public const nint endframe = 0x0; // int32
            public const nint motionflags = 0x4; // int32
            public const nint v0 = 0x8; // float32
            public const nint v1 = 0xC; // float32
            public const nint angle = 0x10; // float32
            public const nint vector = 0x14; // Vector
            public const nint position = 0x20; // Vector
        }
        // Parent: None
        // Field count: 0
        public static class CNmIDValueNode__CDefinition {
        }
        // Parent: None
        // Field count: 0
        public static class CNmChainLookatTask {
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
        public static class CAnimLocalHierarchy {
            public const nint m_sBone = 0x0; // CBufferString
            public const nint m_sNewParent = 0x10; // CBufferString
            public const nint m_nStartFrame = 0x20; // int32
            public const nint m_nPeakFrame = 0x24; // int32
            public const nint m_nTailFrame = 0x28; // int32
            public const nint m_nEndFrame = 0x2C; // int32
        }
        // Parent: None
        // Field count: 0
        public static class CNmSampleTask {
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
        public static class CSequenceGroupData {
            public const nint m_sName = 0x10; // CBufferString
            public const nint m_nFlags = 0x20; // uint32
            public const nint m_localSequenceNameArray = 0x28; // CUtlVector<CBufferString>
            public const nint m_localS1SeqDescArray = 0x40; // CUtlVector<CSeqS1SeqDesc>
            public const nint m_localMultiSeqDescArray = 0x58; // CUtlVector<CSeqS1SeqDesc>
            public const nint m_localSynthAnimDescArray = 0x70; // CUtlVector<CSeqSynthAnimDesc>
            public const nint m_localCmdSeqDescArray = 0x88; // CUtlVector<CSeqCmdSeqDesc>
            public const nint m_localBoneMaskArray = 0xA0; // CUtlVector<CSeqBoneMaskList>
            public const nint m_localScaleSetArray = 0xB8; // CUtlVector<CSeqScaleSet>
            public const nint m_localBoneNameArray = 0xD0; // CUtlVector<CBufferString>
            public const nint m_localNodeName = 0xE8; // CBufferString
            public const nint m_localPoseParamArray = 0xF8; // CUtlVector<CSeqPoseParamDesc>
            public const nint m_keyValues = 0x110; // KeyValues3
            public const nint m_localIKAutoplayLockArray = 0x120; // CUtlVector<CSeqIKLock>
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
        public static class CAnimEventDefinition {
            public const nint m_nFrame = 0x8; // int32
            public const nint m_nEndFrame = 0xC; // int32
            public const nint m_flCycle = 0x10; // float32
            public const nint m_flDuration = 0x14; // float32
            public const nint m_EventData = 0x18; // KeyValues3
            public const nint m_sLegacyOptions = 0x28; // CBufferString
            public const nint m_sEventName = 0x38; // CGlobalSymbol
        }
        // Parent: None
        // Field count: 2
        public static class CBoneConstraintPoseSpaceMorph__Input_t {
            public const nint m_inputValue = 0x0; // Vector
            public const nint m_outputWeightList = 0x10; // CUtlVector<float32>
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
        public static class MoodAnimationLayer_t {
            public const nint m_sName = 0x0; // CUtlString
            public const nint m_bActiveListening = 0x8; // bool
            public const nint m_bActiveTalking = 0x9; // bool
            public const nint m_layerAnimations = 0x10; // CUtlVector<MoodAnimation_t>
            public const nint m_flIntensity = 0x28; // CRangeFloat
            public const nint m_flDurationScale = 0x30; // CRangeFloat
            public const nint m_bScaleWithInts = 0x38; // bool
            public const nint m_flNextStart = 0x3C; // CRangeFloat
            public const nint m_flStartOffset = 0x44; // CRangeFloat
            public const nint m_flEndOffset = 0x4C; // CRangeFloat
            public const nint m_flFadeIn = 0x54; // float32
            public const nint m_flFadeOut = 0x58; // float32
        }
        // Parent: None
        // Field count: 0
        public static class CNmBoolValueNode__CDefinition {
        }
        // Parent: None
        // Field count: 0
        public static class IParticleEffect {
        }
        // Parent: None
        // Field count: 1
        public static class PARTICLE_EHANDLE__ {
            public const nint unused = 0x0; // int32
        }
        // Parent: None
        // Field count: 1
        public static class CAnimEnum {
            public const nint m_value = 0x0; // uint8
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
        public static class CAnimDecoder {
            public const nint m_szName = 0x0; // CBufferString
            public const nint m_nVersion = 0x10; // int32
            public const nint m_nType = 0x14; // int32
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
        public static class CAnimFrameSegment {
            public const nint m_nUniqueFrameIndex = 0x0; // int32
            public const nint m_nLocalElementMasks = 0x4; // uint32
            public const nint m_nLocalChannel = 0x8; // int32
            public const nint m_container = 0x10; // CUtlBinaryBlock
        }
        // Parent: None
        // Field count: 8
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CAnimDesc_Flag {
            public const nint m_bLooping = 0x0; // bool
            public const nint m_bAllZeros = 0x1; // bool
            public const nint m_bHidden = 0x2; // bool
            public const nint m_bDelta = 0x3; // bool
            public const nint m_bLegacyWorldspace = 0x4; // bool
            public const nint m_bModelDoc = 0x5; // bool
            public const nint m_bImplicitSeqIgnoreDelta = 0x6; // bool
            public const nint m_bAnimGraphAdditive = 0x7; // bool
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CTransitionUpdateData {
            public const nint m_srcStateIndex = 0x0; // uint8
            public const nint m_destStateIndex = 0x1; // uint8
            public const nint m_nHandshakeMaskToDisableFirst = 0x0; // bitfield:7
            public const nint m_bDisabled = 0x0; // bitfield:1
        }
        // Parent: None
        // Field count: 2
        public static class CBoneConstraintPoseSpaceBone__Input_t {
            public const nint m_inputValue = 0x0; // Vector
            public const nint m_outputTransformList = 0x10; // CUtlVector<CTransform>
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
        public static class CSeqMultiFetch {
            public const nint m_flags = 0x0; // CSeqMultiFetchFlag
            public const nint m_localReferenceArray = 0x8; // CUtlVector<int16>
            public const nint m_nGroupSize = 0x20; // int32[2]
            public const nint m_nLocalPose = 0x28; // int32[2]
            public const nint m_poseKeyArray0 = 0x30; // CUtlVector<float32>
            public const nint m_poseKeyArray1 = 0x48; // CUtlVector<float32>
            public const nint m_nLocalCyclePoseParameter = 0x60; // int32
            public const nint m_bCalculatePoseParameters = 0x64; // bool
            public const nint m_bFixedBlendWeight = 0x65; // bool
            public const nint m_flFixedBlendWeightVals = 0x68; // float32[2]
        }
        // Parent: None
        // Field count: 0
        public static class CNmOverlayBlendTask {
        }
        // Parent: None
        // Field count: 0
        public static class CNmValueNode__CDefinition {
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
        public static class CSeqS1SeqDesc {
            public const nint m_sName = 0x0; // CBufferString
            public const nint m_flags = 0x10; // CSeqSeqDescFlag
            public const nint m_fetch = 0x20; // CSeqMultiFetch
            public const nint m_nLocalWeightlist = 0x90; // int32
            public const nint m_autoLayerArray = 0x98; // CUtlVector<CSeqAutoLayer>
            public const nint m_IKLockArray = 0xB0; // CUtlVector<CSeqIKLock>
            public const nint m_transition = 0xC8; // CSeqTransition
            public const nint m_SequenceKeys = 0xD0; // KeyValues3
            public const nint m_LegacyKeyValueText = 0xE0; // CBufferString
            public const nint m_activityArray = 0xF0; // CUtlVector<CAnimActivity>
            public const nint m_footMotion = 0x108; // CUtlVector<CFootMotion>
        }
        // Parent: None
        // Field count: 0
        public static class CNmGraphInstance {
        }
        // Parent: None
        // Field count: 9
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MKV3TransferName
        public static class CSeqCmdLayer {
            public const nint m_cmd = 0x0; // int16
            public const nint m_nLocalReference = 0x2; // int16
            public const nint m_nLocalBonemask = 0x4; // int16
            public const nint m_nDstResult = 0x6; // int16
            public const nint m_nSrcResult = 0x8; // int16
            public const nint m_bSpline = 0xA; // bool
            public const nint m_flVar1 = 0xC; // float32
            public const nint m_flVar2 = 0x10; // float32
            public const nint m_nLineNumber = 0x14; // int16
        }
        // Parent: None
        // Field count: 0
        public static class CNmFloatValueNode__CDefinition {
        }
        // Parent: None
        // Field count: 10
        public static class CNmTwoBoneIKTask {
            public const nint m_nEffectorBoneIdx = 0x70; // int32
            public const nint m_nEffectorTargetBoneIdx = 0x74; // int32
            public const nint m_targetTransform = 0x80; // CTransform
            public const nint m_effectorTarget = 0xA0; // CNmTarget
            public const nint m_blendMode = 0xD0; // NmIKBlendMode_t
            public const nint m_flBlendWeight = 0xD4; // float32
            public const nint m_bIsTargetInWorldSpace = 0xD8; // bool
            public const nint m_bIsRunningFromDeserializedData = 0xD9; // bool
            public const nint m_flChainRotationWeight = 0xDC; // float32
            public const nint m_debugEffectorBoneID = 0xE0; // CGlobalSymbol
        }
        // Parent: None
        // Field count: 4
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CSeqIKLock {
            public const nint m_flPosWeight = 0x0; // float32
            public const nint m_flAngleWeight = 0x4; // float32
            public const nint m_nLocalBone = 0x8; // int16
            public const nint m_bBonesOrientedAlongPositiveX = 0xA; // bool
        }
        // Parent: None
        // Field count: 0
        public static class CNmFollowBoneTask {
        }
        // Parent: None
        // Field count: 0
        public static class CNmTargetValueNode__CDefinition {
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CPoseHandle {
            public const nint m_nIndex = 0x0; // uint16
            public const nint m_eType = 0x2; // PoseType_t
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
        public static class CSeqCmdSeqDesc {
            public const nint m_sName = 0x0; // CBufferString
            public const nint m_flags = 0x10; // CSeqSeqDescFlag
            public const nint m_transition = 0x1C; // CSeqTransition
            public const nint m_nFrameRangeSequence = 0x24; // int16
            public const nint m_nFrameCount = 0x26; // int16
            public const nint m_flFPS = 0x28; // float32
            public const nint m_nSubCycles = 0x2C; // int16
            public const nint m_numLocalResults = 0x2E; // int16
            public const nint m_cmdLayerArray = 0x30; // CUtlVector<CSeqCmdLayer>
            public const nint m_eventArray = 0x48; // CUtlVector<CAnimEventDefinition>
            public const nint m_activityArray = 0x60; // CUtlVector<CAnimActivity>
            public const nint m_poseSettingArray = 0x78; // CUtlVector<CSeqPoseSetting>
        }
        // Parent: None
        // Field count: 0
        public static class CNmCachedPoseWriteTask {
        }
        // Parent: None
        // Field count: 0
        public static class CNmModelSpaceBlendTask {
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class AnimationDecodeDebugDump_t {
            public const nint m_processingType = 0x0; // AnimationProcessingType_t
            public const nint m_elems = 0x8; // CUtlVector<AnimationDecodeDebugDumpElement_t>
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
        public static class CSeqScaleSet {
            public const nint m_sName = 0x0; // CBufferString
            public const nint m_bRootOffset = 0x10; // bool
            public const nint m_vRootOffset = 0x14; // Vector
            public const nint m_nLocalBoneArray = 0x20; // CUtlVector<int16>
            public const nint m_flBoneScaleArray = 0x38; // CUtlVector<float32>
        }
        // Parent: None
        // Field count: 5
        public static class IKTargetSettings_t {
            public const nint m_TargetSource = 0x0; // IKTargetSource
            public const nint m_Bone = 0x8; // IKBoneNameAndIndex_t
            public const nint m_AnimgraphParameterNamePosition = 0x18; // AnimParamID
            public const nint m_AnimgraphParameterNameOrientation = 0x1C; // AnimParamID
            public const nint m_TargetCoordSystem = 0x20; // IKTargetCoordinateSystem
        }
        // Parent: None
        // Field count: 0
        public static class CNmAdditiveBlendTask {
        }
        // Parent: None
        // Field count: 7
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CSeqAutoLayer {
            public const nint m_nLocalReference = 0x0; // int16
            public const nint m_nLocalPose = 0x2; // int16
            public const nint m_flags = 0x4; // CSeqAutoLayerFlag
            public const nint m_start = 0xC; // float32
            public const nint m_peak = 0x10; // float32
            public const nint m_tail = 0x14; // float32
            public const nint m_end = 0x18; // float32
        }
        // Parent: None
        // Field count: 2
        //
        // Metadata:
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        // MGetKV3ClassDefaults
        public static class CAnimSequenceParams {
            public const nint m_flFadeInTime = 0x0; // float32
            public const nint m_flFadeOutTime = 0x4; // float32
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
        public static class CAnimData {
            public const nint m_name = 0x10; // CBufferString
            public const nint m_animArray = 0x20; // CUtlVector<CAnimDesc>
            public const nint m_decoderArray = 0x38; // CUtlVector<CAnimDecoder>
            public const nint m_nMaxUniqueFrameIndex = 0x50; // int32
            public const nint m_segmentArray = 0x58; // CUtlVector<CAnimFrameSegment>
        }
        // Parent: None
        // Field count: 3
        public static class IKSolverSettings_t {
            public const nint m_SolverType = 0x0; // IKSolverType
            public const nint m_nNumIterations = 0x4; // int32
            public const nint m_EndEffectorRotationFixUpMode = 0x8; // EIKEndEffectorRotationFixUpMode
        }
        // Parent: None
        // Field count: 1
        //
        // Metadata:
        // MGetKV3ClassDefaults
        public static class CAnimTagManagerUpdater {
            public const nint m_tags = 0x38; // CUtlVector<CSmartPtr<CAnimTagBase>>
        }
    }
}
