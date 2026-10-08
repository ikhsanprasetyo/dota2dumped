# Generated using https://github.com/ikhsanprasetyo/source2-dumper
# 2026-10-08 20:27:02.884074100 +07:00

class Schemas:
    # Module: animationsystem.dll
    class AnimationsystemDll:
        class PulseBestOutflowRules_t:
            SORT_BY_NUMBER_OF_VALID_CRITERIA = 0x0
            SORT_BY_OUTFLOW_INDEX = 0x1
        class PulseCursorCancelPriority_t:
            None_ = 0x0
            CancelOnSucceeded = 0x1
            SoftCancel = 0x2
            HardCancel = 0x3
        class PulseMethodCallMode_t:
            SYNC_WAIT_FOR_COMPLETION = 0x0
            ASYNC_FIRE_AND_FORGET = 0x1
        class PulseCursorWakePriority_t:
            WakeElegantly = 0x0
            WakeImmediate = 0x1
        class ParticleSetMethod_t:
            PARTICLE_SET_REPLACE_VALUE = 0x0
            PARTICLE_SET_SCALE_INITIAL_VALUE = 0x1
            PARTICLE_SET_ADD_TO_INITIAL_VALUE = 0x2
            PARTICLE_SET_RAMP_CURRENT_VALUE = 0x3
            PARTICLE_SET_SCALE_CURRENT_VALUE = 0x4
            PARTICLE_SET_ADD_TO_CURRENT_VALUE = 0x5
        class SeqCmd_t:
            SeqCmd_Nop = 0x0
            SeqCmd_LinearDelta = 0x1
            SeqCmd_FetchFrameRange = 0x2
            SeqCmd_Slerp = 0x3
            SeqCmd_Add = 0x4
            SeqCmd_Subtract = 0x5
            SeqCmd_Scale = 0x6
            SeqCmd_Copy = 0x7
            SeqCmd_Blend = 0x8
            SeqCmd_Worldspace = 0x9
            SeqCmd_Sequence = 0xA
            SeqCmd_FetchCycle = 0xB
            SeqCmd_FetchFrame = 0xC
            SeqCmd_IKLockInPlace = 0xD
            SeqCmd_IKRestoreAll = 0xE
            SeqCmd_ReverseSequence = 0xF
            SeqCmd_Transform = 0x10
        class CNmEventRelevance_t:
            ClientOnly = 0x0
            ServerOnly = 0x1
            ClientAndServer = 0x2
        class BoneTransformSpace_t:
            BoneTransformSpace_Invalid = 0xFFFFFFFFFFFFFFFF
            BoneTransformSpace_Parent = 0x0
            BoneTransformSpace_Model = 0x1
            BoneTransformSpace_World = 0x2
        class CAnimationGraphVisualizerPrimitiveType:
            ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Text = 0x0
            ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Sphere = 0x1
            ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Line = 0x2
            ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Pie = 0x3
            ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Axis = 0x4
        class NmTransitionRule_t:
            AllowTransition = 0x0
            ConditionallyAllowTransition = 0x1
            BlockTransition = 0x2
        class BinaryNodeTiming:
            UseChild1 = 0x0
            UseChild2 = 0x1
            SyncChildren = 0x2
        class NmFollowBoneMode_t:
            RotationAndTranslation = 0x0
            RotationOnly = 0x1
            TranslationOnly = 0x2
        class SolveIKChainAnimNodeDebugSetting:
            SOLVEIKCHAINANIMNODEDEBUGSETTING_None = 0x0
            SOLVEIKCHAINANIMNODEDEBUGSETTING_X_Axis_Circle = 0x1
            SOLVEIKCHAINANIMNODEDEBUGSETTING_Y_Axis_Circle = 0x2
            SOLVEIKCHAINANIMNODEDEBUGSETTING_Z_Axis_Circle = 0x3
            SOLVEIKCHAINANIMNODEDEBUGSETTING_Forward = 0x4
            SOLVEIKCHAINANIMNODEDEBUGSETTING_Up = 0x5
            SOLVEIKCHAINANIMNODEDEBUGSETTING_Left = 0x6
        class CNmParticleEvent__Type_t:
            Create = 0x0
            Create_CFG = 0x1
        class ParticleFloatBiasType_t:
            PF_BIAS_TYPE_INVALID = 0xFFFFFFFFFFFFFFFF
            PF_BIAS_TYPE_STANDARD = 0x0
            PF_BIAS_TYPE_GAIN = 0x1
            PF_BIAS_TYPE_EXPONENTIAL = 0x2
            PF_BIAS_TYPE_COUNT = 0x3
        class CNmTargetWarpNode__TargetUpdateRule_t:
            None_ = 0x0
            Recalculate = 0x1
            Offset = 0x2
            RecalculateOrOffset = 0x3
        class SharedMovementGait_t:
            eInvalid = 0xFFFFFFFFFFFFFFFF
            eSlow = 0x0
            eMedium = 0x1
            eFast = 0x2
            eVeryFast = 0x3
            eCount = 0x4
        class OrientationWarpRootMotionSource_t:
            eAnimationOrProcedural = 0x0
            eAnimationOnly = 0x1
            eProceduralOnly = 0x2
        class IKTargetCoordinateSystem:
            IKTARGETCOORDINATESYSTEM_WorldSpace = 0x0
            IKTARGETCOORDINATESYSTEM_ModelSpace = 0x1
            IKTARGETCOORDINATESYSTEM_COUNT = 0x2
        class ParticleFloatType_t:
            PF_TYPE_INVALID = 0xFFFFFFFFFFFFFFFF
            PF_TYPE_LITERAL = 0x0
            PF_TYPE_NAMED_VALUE = 0x1
            PF_TYPE_RANDOM_UNIFORM = 0x2
            PF_TYPE_RANDOM_BIASED = 0x3
            PF_TYPE_COLLECTION_AGE = 0x4
            PF_TYPE_ENDCAP_AGE = 0x5
            PF_TYPE_CONTROL_POINT_COMPONENT = 0x6
            PF_TYPE_CONTROL_POINT_CHANGE_AGE = 0x7
            PF_TYPE_CONTROL_POINT_SPEED = 0x8
            PF_TYPE_CONTROL_POINT_DISTANCE = 0x9
            PF_TYPE_PARTICLE_DETAIL_LEVEL = 0xA
            PF_TYPE_CONCURRENT_DEF_COUNT = 0xB
            PF_TYPE_CLOSEST_CAMERA_DISTANCE = 0xC
            PF_TYPE_SNAPSHOT_COUNT = 0xD
            PF_TYPE_SNAPSHOT_CHANGED = 0xE
            PF_TYPE_CONTROL_POINT_IS_SET = 0xF
            PF_TYPE_RENDERER_CAMERA_DISTANCE = 0x10
            PF_TYPE_RENDERER_CAMERA_DOT_PRODUCT = 0x11
            PF_TYPE_PARTICLE_NOISE = 0x12
            PF_TYPE_PARTICLE_AGE = 0x13
            PF_TYPE_PARTICLE_AGE_NORMALIZED = 0x14
            PF_TYPE_PARTICLE_FLOAT = 0x15
            PF_TYPE_PARTICLE_INITIAL_FLOAT = 0x16
            PF_TYPE_PARTICLE_VECTOR_COMPONENT = 0x17
            PF_TYPE_PARTICLE_INITIAL_VECTOR_COMPONENT = 0x18
            PF_TYPE_PARTICLE_SPEED = 0x19
            PF_TYPE_PARTICLE_NUMBER = 0x1A
            PF_TYPE_PARTICLE_NUMBER_NORMALIZED = 0x1B
            PF_TYPE_PARTICLE_ROPE_SEGMENT = 0x1C
            PF_TYPE_PARTICLE_ROPE_SEGMENT_NORMALIZED = 0x1D
            PF_TYPE_PARTICLE_SCREENSPACE_CAMERA_DISTANCE = 0x1E
            PF_TYPE_PARTICLE_SCREENSPACE_CAMERA_DOT_PRODUCT = 0x1F
            PF_TYPE_COUNT = 0x20
        class CNmFloatAngleMathNode__Operation_t:
            ClampTo180 = 0x0
            ClampTo360 = 0x1
            FlipHemisphere = 0x2
            FlipHemisphereNegate = 0x3
        class VPhysXAggregateData_t__VPhysXFlagEnum_t:
            FLAG_IS_POLYSOUP_GEOMETRY = 0x1
            FLAG_LEVEL_COLLISION = 0x10
            FLAG_IGNORE_SCALE_OBSOLETE_DO_NOT_USE = 0x20
        class CNmRootMotionOverrideNode__OverrideFlags_t:
            AllowMoveX = 0x0
            AllowMoveY = 0x1
            AllowMoveZ = 0x2
            AllowFacingPitch = 0x3
            ListenForEvents = 0x4
        class NmEasingOperation_t:
            Linear = 0x0
            InQuad = 0x1
            OutQuad = 0x2
            InOutQuad = 0x3
            InCubic = 0x4
            OutCubic = 0x5
            InOutCubic = 0x6
            InQuart = 0x7
            OutQuart = 0x8
            InOutQuart = 0x9
            InQuint = 0xA
            OutQuint = 0xB
            InOutQuint = 0xC
            InSine = 0xD
            OutSine = 0xE
            InOutSine = 0xF
            InExpo = 0x10
            OutExpo = 0x11
            InOutExpo = 0x12
            InCirc = 0x13
            OutCirc = 0x14
            InOutCirc = 0x15
            None_ = 0x16
        class EIKEndEffectorRotationFixUpMode:
            None_ = 0x0
            MatchTargetOrientation = 0x1
            LookAtTargetForward = 0x2
            MaintainParentOrientation = 0x3
            Count = 0x4
        class MatterialAttributeTagType_t:
            MATERIAL_ATTRIBUTE_TAG_VALUE = 0x0
            MATERIAL_ATTRIBUTE_TAG_COLOR = 0x1
        class PFNoiseTurbulence_t:
            PF_NOISE_TURB_NONE = 0x0
            PF_NOISE_TURB_HIGHLIGHT = 0x1
            PF_NOISE_TURB_FEEDBACK = 0x2
            PF_NOISE_TURB_LOOPY = 0x3
            PF_NOISE_TURB_CONTRAST = 0x4
            PF_NOISE_TURB_ALTERNATE = 0x5
        class NmTargetWarpAlgorithm_t:
            Lerp = 0x0
            Hermite = 0x1
            HermiteFeaturePreserving = 0x2
            Bezier = 0x3
        class ParticleColorBlendMode_t:
            PARTICLEBLEND_DEFAULT = 0x0
            PARTICLEBLEND_OVERLAY = 0x1
            PARTICLEBLEND_DARKEN = 0x2
            PARTICLEBLEND_LIGHTEN = 0x3
            PARTICLEBLEND_MULTIPLY = 0x4
        class ParticleColorBlendType_t:
            PARTICLE_COLOR_BLEND_MULTIPLY = 0x0
            PARTICLE_COLOR_BLEND_MULTIPLY2X = 0x1
            PARTICLE_COLOR_BLEND_DIVIDE = 0x2
            PARTICLE_COLOR_BLEND_ADD = 0x3
            PARTICLE_COLOR_BLEND_SUBTRACT = 0x4
            PARTICLE_COLOR_BLEND_MOD2X = 0x5
            PARTICLE_COLOR_BLEND_SCREEN = 0x6
            PARTICLE_COLOR_BLEND_MAX = 0x7
            PARTICLE_COLOR_BLEND_MIN = 0x8
            PARTICLE_COLOR_BLEND_REPLACE = 0x9
            PARTICLE_COLOR_BLEND_AVERAGE = 0xA
            PARTICLE_COLOR_BLEND_NEGATE = 0xB
            PARTICLE_COLOR_BLEND_LUMINANCE = 0xC
        class NmTransitionRuleCondition_t:
            AnyAllowed = 0x0
            FullyAllowed = 0x1
            ConditionallyAllowed = 0x2
            Blocked = 0x3
        class ModelMeshBufferUsage_t:
            MESH_BUFFER_USAGE_NONE = 0x0
            MESH_BUFFER_USAGE_VB = 0x1
            MESH_BUFFER_USAGE_IB = 0x2
            MESH_BUFFER_USAGE_ADJACENCY = 0x4
            MESH_BUFFER_USAGE_MESHLET_TRIS = 0x8
            MESH_BUFFER_USAGE_RT_PROXY = 0x10
            MESH_BUFFER_USAGE_VERTEX_ALBEDO = 0x20
            MESH_BUFFER_USAGE_VERTEX_EMISSIVE = 0x40
            MESH_BUFFER_USAGE_MESHLETS = 0x80
            MESH_BUFFER_USAGE_ALIAS_TABLE = 0x100
        class NmGraphDebugMode_t:
            Off = 0x0
            On = 0x1
        class TargetWarpTimingMethod:
            ReachDestinationOnRootMotionEnd = 0x0
            ReachDestinationOnWarpTagEnd = 0x1
        class ScriptedMoveTo_t:
            eWait = 0x0
            eMoveWithGait = 0x3
            eTeleport = 0x4
            eWaitFacing = 0x5
            eObsoleteBackCompat1 = 0x1
            eObsoleteBackCompat2 = 0x2
        class EDemoBoneSelectionMode:
            CaptureAllBones = 0x0
            CaptureSelectedBones = 0x1
        class StepPhase:
            StepPhase_OnGround = 0x0
            StepPhase_InAir = 0x1
        class FlexOpCode_t:
            FLEX_OP_CONST = 0x1
            FLEX_OP_FETCH1 = 0x2
            FLEX_OP_FETCH2 = 0x3
            FLEX_OP_ADD = 0x4
            FLEX_OP_SUB = 0x5
            FLEX_OP_MUL = 0x6
            FLEX_OP_DIV = 0x7
            FLEX_OP_NEG = 0x8
            FLEX_OP_EXP = 0x9
            FLEX_OP_OPEN = 0xA
            FLEX_OP_CLOSE = 0xB
            FLEX_OP_COMMA = 0xC
            FLEX_OP_MAX = 0xD
            FLEX_OP_MIN = 0xE
            FLEX_OP_2WAY_0 = 0xF
            FLEX_OP_2WAY_1 = 0x10
            FLEX_OP_NWAY = 0x11
            FLEX_OP_COMBO = 0x12
            FLEX_OP_DOMINATE = 0x13
            FLEX_OP_DME_LOWER_EYELID = 0x14
            FLEX_OP_DME_UPPER_EYELID = 0x15
            FLEX_OP_SQRT = 0x16
            FLEX_OP_REMAPVALCLAMPED = 0x17
            FLEX_OP_SIN = 0x18
            FLEX_OP_COS = 0x19
            FLEX_OP_ABS = 0x1A
        class NmCachedValueMode_t:
            OnEntry = 0x0
            OnExit = 0x1
        class AnimNodeNetworkMode:
            ServerAuthoritative = 0x0
            ClientSimulate = 0x1
        class VPhysXBodyPart_t__VPhysXFlagEnum_t:
            FLAG_STATIC = 0x1
            FLAG_KINEMATIC = 0x2
            FLAG_JOINT = 0x4
            FLAG_MASS = 0x8
            FLAG_ALWAYS_DYNAMIC_ON_CLIENT = 0x10
            FLAG_DISABLE_CCD = 0x20
        class AnimParamType_t:
            ANIMPARAM_UNKNOWN = 0x0
            ANIMPARAM_BOOL = 0x1
            ANIMPARAM_ENUM = 0x2
            ANIMPARAM_INT = 0x3
            ANIMPARAM_FLOAT = 0x4
            ANIMPARAM_VECTOR = 0x5
            ANIMPARAM_QUATERNION = 0x6
            ANIMPARAM_GLOBALSYMBOL = 0x7
            ANIMPARAM_COUNT = 0x8
        class NmEasingFunction_t:
            Linear = 0x0
            Quad = 0x1
            Cubic = 0x2
            Quart = 0x3
            Quint = 0x4
            Sine = 0x5
            Expo = 0x6
            Circ = 0x7
            Back = 0x8
        class ParticleModelType_t:
            PM_TYPE_INVALID = 0x0
            PM_TYPE_NAMED_VALUE_MODEL = 0x1
            PM_TYPE_NAMED_VALUE_EHANDLE = 0x2
            PM_TYPE_CONTROL_POINT = 0x3
            PM_TYPE_COUNT = 0x4
        class IKTargetSource:
            IKTARGETSOURCE_Bone = 0x0
            IKTARGETSOURCE_AnimgraphParameter = 0x1
            IKTARGETSOURCE_COUNT = 0x2
        class PermModelInfo_t__FlagEnum:
            FLAG_TRANSLUCENT = 0x1
            FLAG_TRANSLUCENT_TWO_PASS = 0x2
            FLAG_MODEL_IS_RUNTIME_COMBINED = 0x4
            FLAG_SOURCE1_IMPORT = 0x8
            FLAG_MODEL_PART_CHILD = 0x10
            FLAG_NAV_GEN_NONE = 0x20
            FLAG_NAV_GEN_HULL = 0x40
            FLAG_NO_FORCED_FADE = 0x800
            FLAG_HAS_SKINNED_MESHES = 0x400
            FLAG_DO_NOT_CAST_SHADOWS = 0x20000
            FLAG_FORCE_PHONEME_CROSSFADE = 0x1000
            FLAG_NO_ANIM_EVENTS = 0x100000
            FLAG_ANIMATION_DRIVEN_FLEXES = 0x200000
            FLAG_IMPLICIT_BIND_POSE_SEQUENCE = 0x400000
            FLAG_MODEL_DOC = 0x800000
        class CNmFloatMathNode__Operator_t:
            Add = 0x0
            Sub = 0x1
            Mul = 0x2
            Div = 0x3
            Mod = 0x4
            Abs = 0x5
            Negate = 0x6
            Floor = 0x7
            Ceiling = 0x8
            IntegerPart = 0x9
            FractionalPart = 0xA
            InverseFractionalPart = 0xB
        class CNmSyncEventIndexConditionNode__TriggerMode_t:
            ExactlyAtEventIndex = 0x0
            GreaterThanEqualToEventIndex = 0x1
        class ParticleFloatRoundType_t:
            PF_ROUND_TYPE_INVALID = 0xFFFFFFFFFFFFFFFF
            PF_ROUND_TYPE_NEAREST = 0x0
            PF_ROUND_TYPE_FLOOR = 0x1
            PF_ROUND_TYPE_CEIL = 0x2
            PF_ROUND_TYPE_COUNT = 0x3
        class PFNoiseType_t:
            PF_NOISE_TYPE_PERLIN = 0x0
            PF_NOISE_TYPE_SIMPLEX = 0x1
            PF_NOISE_TYPE_WORLEY = 0x2
            PF_NOISE_TYPE_CURL = 0x3
        class ParticleDirectionNoiseType_t:
            PARTICLE_DIR_NOISE_PERLIN = 0x0
            PARTICLE_DIR_NOISE_CURL = 0x1
            PARTICLE_DIR_NOISE_WORLEY_BASIC = 0x2
        class AnimParamNetworkSetting:
            Auto = 0x0
            AlwaysNetwork = 0x1
            NeverNetwork = 0x2
        class MorphFlexControllerRemapType_t:
            MORPH_FLEXCONTROLLER_REMAP_PASSTHRU = 0x0
            MORPH_FLEXCONTROLLER_REMAP_2WAY = 0x1
            MORPH_FLEXCONTROLLER_REMAP_NWAY = 0x2
            MORPH_FLEXCONTROLLER_REMAP_EYELID = 0x3
        class MeshDrawPrimitiveFlags_t:
            MESH_DRAW_FLAGS_NONE = 0x0
            MESH_DRAW_FLAGS_USE_SHADOW_FAST_PATH = 0x1
            MESH_DRAW_FLAGS_USE_COMPRESSED_NORMAL_TANGENT = 0x2
            MESH_DRAW_INPUT_LAYOUT_IS_NOT_MATCHED_TO_MATERIAL = 0x8
            MESH_DRAW_FLAGS_USE_COMPRESSED_PER_VERTEX_LIGHTING = 0x10
            MESH_DRAW_FLAGS_USE_UNCOMPRESSED_PER_VERTEX_LIGHTING = 0x20
            MESH_DRAW_FLAGS_CAN_BATCH_WITH_DYNAMIC_SHADER_CONSTANTS = 0x40
            MESH_DRAW_FLAGS_DRAW_LAST = 0x80
        class TargetWarpAngleMode_t:
            eFacingHeading = 0x0
            eMoveHeading = 0x1
        class NmIKBlendMode_t:
            Effector = 0x0
            Pose = 0x1
        class ModelBoneFlexComponent_t:
            MODEL_BONE_FLEX_INVALID = 0xFFFFFFFFFFFFFFFF
            MODEL_BONE_FLEX_TX = 0x0
            MODEL_BONE_FLEX_TY = 0x1
            MODEL_BONE_FLEX_TZ = 0x2
        class CNmStateNode__TimedEvent_t__Comparison_t:
            LessThanEqual = 0x0
            GreaterThanEqual = 0x1
        class PoseType_t:
            POSETYPE_STATIC = 0x0
            POSETYPE_DYNAMIC = 0x1
            POSETYPE_INVALID = 0xFF
        class CNmClothEvent__Type_t:
            Stiffen = 0x0
            Effect = 0x1
        class CNmRootMotionData__SamplingMode_t:
            Delta = 0x0
            WorldSpace = 0x1
        class NmEventConditionRules_t:
            LimitSearchToSourceState = 0x0
            IgnoreInactiveEvents = 0x1
            PreferHighestWeight = 0x2
            PreferHighestProgress = 0x3
            OperatorOr = 0x4
            OperatorAnd = 0x5
            SearchOnlyGraphEvents = 0x6
            SearchOnlyAnimEvents = 0x7
            SearchBothGraphAndAnimEvents = 0x8
        class AnimationType_t:
            ANIMATION_TYPE_FIXED_RATE = 0x0
            ANIMATION_TYPE_FIT_LIFETIME = 0x1
            ANIMATION_TYPE_MANUAL_FRAMES = 0x2
        class AnimValueSource:
            MoveHeading = 0x0
            MoveSpeed = 0x1
            ForwardSpeed = 0x2
            StrafeSpeed = 0x3
            FacingHeading = 0x4
            LookHeading = 0x5
            LookHeadingNormalized = 0x6
            LookPitch = 0x7
            LookDistance = 0x8
            Parameter = 0x9
            WayPointHeading = 0xA
            WayPointDistance = 0xB
            BoundaryRadius = 0xC
            TargetMoveHeading = 0xD
            TargetMoveSpeed = 0xE
            AccelerationHeading = 0xF
            AccelerationSpeed = 0x10
            SlopeHeading = 0x11
            SlopeAngle = 0x12
            SlopePitch = 0x13
            SlopeYaw = 0x14
            GoalDistance = 0x15
            AccelerationLeftRight = 0x16
            AccelerationFrontBack = 0x17
            RootMotionSpeed = 0x18
            RootMotionTurnSpeed = 0x19
            MoveHeadingRelativeToLookHeading = 0x1A
            MaxMoveSpeed = 0x1B
            FingerCurl_Thumb = 0x1C
            FingerCurl_Index = 0x1D
            FingerCurl_Middle = 0x1E
            FingerCurl_Ring = 0x1F
            FingerCurl_Pinky = 0x20
            FingerSplay_Thumb_Index = 0x21
            FingerSplay_Index_Middle = 0x22
            FingerSplay_Middle_Ring = 0x23
            FingerSplay_Ring_Pinky = 0x24
        class CNmTimeConditionNode__Operator_t:
            LessThan = 0x0
            LessThanEqual = 0x1
            GreaterThan = 0x2
            GreaterThanEqual = 0x3
        class LinearRootMotionBlendMode_t:
            LERP = 0x0
            NLERP = 0x1
            SLERP = 0x2
        class RagdollPoseControl:
            Absolute = 0x0
        class IKSolverType:
            IKSOLVER_Perlin = 0x0
            IKSOLVER_TwoBone = 0x1
            IKSOLVER_Fabrik = 0x2
            IKSOLVER_DogLeg3Bone = 0x3
            IKSOLVER_CCD = 0x4
            IKSOLVER_COUNT = 0x5
        class TargetWarpCorrectionMethod:
            ScaleMotion = 0x0
            AddCorrectionDelta = 0x1
        class TargetSelectorAngleMode_t:
            eFacingHeading = 0x0
            eMoveHeading = 0x1
        class Blend2DMode:
            Blend2DMode_General = 0x0
            Blend2DMode_Directional = 0x1
        class HandshakeTagState_t:
            eInactive = 0x0
            eActive = 0x1
            eMomentarilyInactive = 0x2
        class ChoiceChangeMethod:
            OnReset = 0x0
            OnCycleEnd = 0x1
            OnResetOrCycleEnd = 0x2
        class ChoiceBlendMethod:
            SingleBlendTime = 0x0
            PerChoiceBlendTimes = 0x1
        class VPhysXConstraintParams_t__EnumFlags0_t:
            FLAG0_SHIFT_INTERPENETRATE = 0x0
            FLAG0_SHIFT_CONSTRAIN = 0x1
            FLAG0_SHIFT_BREAKABLE_FORCE = 0x2
            FLAG0_SHIFT_BREAKABLE_TORQUE = 0x3
        class ParticleFloatMapType_t:
            PF_MAP_TYPE_INVALID = 0xFFFFFFFFFFFFFFFF
            PF_MAP_TYPE_DIRECT = 0x0
            PF_MAP_TYPE_MULT = 0x1
            PF_MAP_TYPE_REMAP = 0x2
            PF_MAP_TYPE_REMAP_BIASED = 0x3
            PF_MAP_TYPE_CURVE = 0x4
            PF_MAP_TYPE_NOTCHED = 0x5
            PF_MAP_TYPE_ROUND = 0x6
            PF_MAP_TYPE_MIN = 0x7
            PF_MAP_TYPE_MAX = 0x8
            PF_MAP_TYPE_MOD = 0x9
            PF_MAP_TYPE_COUNT = 0xA
        class AnimParamVectorType_t:
            ANIMPARAM_VECTOR_TYPE_NONE = 0x0
            ANIMPARAM_VECTOR_TYPE_POSITION_WS = 0x1
            ANIMPARAM_VECTOR_TYPE_POSITION_LS = 0x2
            ANIMPARAM_VECTOR_TYPE_DIRECTION_WS = 0x3
            ANIMPARAM_VECTOR_TYPE_DIRECTION_LS = 0x4
        class CNmCurrentSyncEventNode__InfoType_t:
            IndexAndPercentage = 0x0
            IndexOnly = 0x1
            PercentageOnly = 0x2
        class BlendKeyType:
            BlendKey_UserValue = 0x0
            BlendKey_Velocity = 0x1
            BlendKey_Distance = 0x2
            BlendKey_RemainingDistance = 0x3
        class StateActionBehavior:
            STATETAGBEHAVIOR_ACTIVE_WHILE_CURRENT = 0x0
            STATETAGBEHAVIOR_FIRE_ON_ENTER = 0x1
            STATETAGBEHAVIOR_FIRE_ON_EXIT = 0x2
            STATETAGBEHAVIOR_FIRE_ON_ENTER_AND_EXIT = 0x3
            STATETAGBEHAVIOR_ACTIVE_WHILE_FULLY_BLENDED = 0x4
        class NmRootMotionBlendMode_t:
            Blend = 0x0
            Additive = 0x1
            IgnoreSource = 0x2
            IgnoreTarget = 0x3
        class NmFootPhaseCondition_t:
            LeftFootDown = 0x0
            LeftFootPassing = 0x1
            LeftPhase = 0x4
            RightFootDown = 0x2
            RightFootPassing = 0x3
            RightPhase = 0x5
            None_ = 0x6
        class ModelSkeletonData_t__BoneFlags_t:
            FLAG_NO_BONE_FLAGS = 0x0
            FLAG_BONEFLEXDRIVER = 0x4
            FLAG_CLOTH = 0x8
            FLAG_PHYSICS = 0x10
            FLAG_ATTACHMENT = 0x20
            FLAG_ANIMATION = 0x40
            FLAG_MESH = 0x80
            FLAG_HITBOX = 0x100
            FLAG_BONE_USED_BY_VERTEX_LOD0 = 0x400
            FLAG_BONE_USED_BY_VERTEX_LOD1 = 0x800
            FLAG_BONE_USED_BY_VERTEX_LOD2 = 0x1000
            FLAG_BONE_USED_BY_VERTEX_LOD3 = 0x2000
            FLAG_BONE_USED_BY_VERTEX_LOD4 = 0x4000
            FLAG_BONE_USED_BY_VERTEX_LOD5 = 0x8000
            FLAG_BONE_USED_BY_VERTEX_LOD6 = 0x10000
            FLAG_BONE_USED_BY_VERTEX_LOD7 = 0x20000
            FLAG_BONE_MERGE_READ = 0x40000
            FLAG_BONE_MERGE_WRITE = 0x80000
            FLAG_ALL_BONE_FLAGS = 0xFFFFF
            BLEND_PREALIGNED = 0x100000
            FLAG_RIGIDLENGTH = 0x200000
            FLAG_PROCEDURAL = 0x400000
        class CNmOrientationWarpNode__AlignmentMode_t:
            MovementDirection = 0x0
            AnimationEndFacing = 0x1
        class GPUParticleCollisionMode_t:
            PARTICLE_GPU_COLLISION_MODE_RT = 0x0
            PARTICLE_GPU_COLLISION_MODE_DEPTH = 0x1
            PARTICLE_GPU_COLLISION_MODE_HYBRID = 0x2
        class MorphBundleType_t:
            MORPH_BUNDLE_TYPE_NONE = 0x0
            MORPH_BUNDLE_TYPE_POSITION_SPEED = 0x1
            MORPH_BUNDLE_TYPE_NORMAL_WRINKLE = 0x2
            MORPH_BUNDLE_TYPE_COUNT = 0x3
        class CNmIDComparisonNode__Comparison_t:
            Matches = 0x0
            DoesntMatch = 0x1
        class NmPoseBlendMode_t:
            Overlay = 0x0
            Additive = 0x1
            ModelSpace = 0x2
        class ParticleFloatInputMode_t:
            PF_INPUT_MODE_INVALID = 0xFFFFFFFFFFFFFFFF
            PF_INPUT_MODE_CLAMPED = 0x0
            PF_INPUT_MODE_LOOPED = 0x1
            PF_INPUT_MODE_COUNT = 0x2
        class ResetCycleOption:
            Beginning = 0x0
            SameCycleAsSource = 0x1
            InverseSourceCycle = 0x2
            FixedValue = 0x3
            SameTimeAsSource = 0x4
        class CNmVectorInfoNode__Info_t:
            X = 0x0
            Y = 0x1
            Z = 0x2
            Length = 0x3
            AngleHorizontal = 0x4
            AngleVertical = 0x5
        class TagActionStatus:
            Inactive = 0x0
            Active = 0x1
            Fired = 0x2
        class IKChannelMode:
            TwoBone = 0x0
            TwoBone_Translate = 0x1
            OneBone = 0x2
            OneBone_Translate = 0x3
        class NmGraphValueType_t:
            Unknown = 0x0
            Bool = 0x1
            ID = 0x2
            Float = 0x3
            Vector = 0x4
            Target = 0x5
            BoneMask = 0x6
            Pose = 0x7
            Special = 0x8
        class ParticleFloatRandomMode_t:
            PF_RANDOM_MODE_INVALID = 0xFFFFFFFFFFFFFFFF
            PF_RANDOM_MODE_CONSTANT = 0x0
            PF_RANDOM_MODE_VARYING = 0x1
            PF_RANDOM_MODE_COUNT = 0x2
        class PFNoiseModifier_t:
            PF_NOISE_MODIFIER_NONE = 0x0
            PF_NOISE_MODIFIER_LINES = 0x1
            PF_NOISE_MODIFIER_CLUMPS = 0x2
            PF_NOISE_MODIFIER_RINGS = 0x3
        class ParticleVecType_t:
            PVEC_TYPE_INVALID = 0xFFFFFFFFFFFFFFFF
            PVEC_TYPE_LITERAL = 0x0
            PVEC_TYPE_LITERAL_COLOR = 0x1
            PVEC_TYPE_NAMED_VALUE = 0x2
            PVEC_TYPE_PARTICLE_VECTOR = 0x3
            PVEC_TYPE_PARTICLE_INITIAL_VECTOR = 0x4
            PVEC_TYPE_PARTICLE_VELOCITY = 0x5
            PVEC_TYPE_PARTICLE_GRAVITY = 0x6
            PVEC_TYPE_CP_VALUE = 0x7
            PVEC_TYPE_CP_RELATIVE_POSITION = 0x8
            PVEC_TYPE_CP_RELATIVE_DIR = 0x9
            PVEC_TYPE_CP_RELATIVE_RANDOM_DIR = 0xA
            PVEC_TYPE_FLOAT_COMPONENTS = 0xB
            PVEC_TYPE_FLOAT_INTERP_CLAMPED = 0xC
            PVEC_TYPE_FLOAT_INTERP_OPEN = 0xD
            PVEC_TYPE_FLOAT_INTERP_GRADIENT = 0xE
            PVEC_TYPE_RANDOM_UNIFORM = 0xF
            PVEC_TYPE_RANDOM_UNIFORM_OFFSET = 0x10
            PVEC_TYPE_CP_DELTA = 0x11
            PVEC_TYPE_CLOSEST_CAMERA_POSITION = 0x12
            PVEC_TYPE_COUNT = 0x13
        class NmFootPhase_t:
            LeftFootDown = 0x0
            RightFootPassing = 0x1
            RightFootDown = 0x2
            LeftFootPassing = 0x3
            None_ = 0x4
        class CNmTargetInfoNode__Info_t:
            AngleHorizontal = 0x0
            AngleVertical = 0x1
            Distance = 0x2
            DistanceHorizontalOnly = 0x3
            DistanceVerticalOnly = 0x4
            DeltaOrientationX = 0x5
            DeltaOrientationY = 0x6
            DeltaOrientationZ = 0x7
        class FootstepLandedFootSoundType_t:
            FOOTSOUND_Left = 0x0
            FOOTSOUND_Right = 0x1
            FOOTSOUND_UseOverrideSound = 0x2
        class FootLockSubVisualization:
            FOOTLOCKSUBVISUALIZATION_ReachabilityAnalysis = 0x0
            FOOTLOCKSUBVISUALIZATION_IKSolve = 0x1
        class CNmSoundEvent__Position_t:
            None_ = 0x0
            World = 0x1
            EntityPos = 0x2
            EntityEyePos = 0x3
            EntityAttachment = 0x4
        class FootstepJumpPhase_t:
            Unknown = 0x0
            NotJumping = 0x1
            Jumping = 0x2
            Landing = 0x4
        class NmFrameSnapEventMode_t:
            Floor = 0x0
            Round = 0x1
        class FootPinningTimingSource:
            FootMotion = 0x0
            Tag = 0x1
            Parameter = 0x2
        class DampingSpeedFunction:
            NoDamping = 0x0
            Constant = 0x1
            Spring = 0x2
            AsymmetricSpring = 0x3
        class AnimationProcessingType_t:
            ANIMATION_PROCESSING_SERVER_SIMULATION = 0x0
            ANIMATION_PROCESSING_CLIENT_SIMULATION = 0x1
            ANIMATION_PROCESSING_CLIENT_PREDICTION = 0x2
            ANIMATION_PROCESSING_CLIENT_INTERPOLATION = 0x3
            ANIMATION_PROCESSING_CLIENT_RENDER = 0x4
            ANIMATION_PROCESSING_MAX = 0x5
        class JiggleBoneSimSpace:
            SimSpace_Local = 0x0
            SimSpace_Model = 0x1
            SimSpace_World = 0x2
        class StanceOverrideMode:
            Sequence = 0x0
            Node = 0x1
        class IkEndEffectorType:
            IkEndEffector_Attachment = 0x0
            IkEndEffector_Bone = 0x1
        class AnimScriptType:
            ANIMSCRIPT_TYPE_INVALID = 0xFFFFFFFFFFFFFFFF
            ANIMSCRIPT_FUSE_GENERAL = 0x0
            ANIMSCRIPT_FUSE_STATEMACHINE = 0x1
        class CNmTimeConditionNode__ComparisonType_t:
            PercentageThroughState = 0x0
            PercentageThroughSyncEvent = 0x1
            ElapsedTime = 0x2
        class SeqPoseSetting_t:
            SEQ_POSE_SETTING_CONSTANT = 0x0
            SEQ_POSE_SETTING_ROTATION = 0x1
            SEQ_POSE_SETTING_POSITION = 0x2
            SEQ_POSE_SETTING_VELOCITY = 0x3
        class AnimParamButton_t:
            ANIMPARAM_BUTTON_NONE = 0x0
            ANIMPARAM_BUTTON_DPAD_UP = 0x1
            ANIMPARAM_BUTTON_DPAD_RIGHT = 0x2
            ANIMPARAM_BUTTON_DPAD_DOWN = 0x3
            ANIMPARAM_BUTTON_DPAD_LEFT = 0x4
            ANIMPARAM_BUTTON_A = 0x5
            ANIMPARAM_BUTTON_B = 0x6
            ANIMPARAM_BUTTON_X = 0x7
            ANIMPARAM_BUTTON_Y = 0x8
            ANIMPARAM_BUTTON_LEFT_SHOULDER = 0x9
            ANIMPARAM_BUTTON_RIGHT_SHOULDER = 0xA
            ANIMPARAM_BUTTON_LTRIGGER = 0xB
            ANIMPARAM_BUTTON_RTRIGGER = 0xC
        class SelectorTagBehavior_t:
            SelectorTagBehavior_OnWhileCurrent = 0x0
            SelectorTagBehavior_OffWhenFinished = 0x1
            SelectorTagBehavior_OffBeforeFinished = 0x2
        class HandshakeTagType_t:
            eInvalid = 0xFFFFFFFFFFFFFFFF
            eTask = 0x0
            eMovement = 0x1
            eCount = 0x2
        class OrientationWarpTargetOffsetMode_t:
            eLiteralValue = 0x0
            eParameter = 0x1
            eAnimationMovementHeading = 0x2
            eAnimationMovementHeadingAtEnd = 0x3
        class OrientationWarpMode_t:
            eInvalid = 0x0
            eAngle = 0x1
            eWorldPosition = 0x2
        class ParticleTransformType_t:
            PT_TYPE_INVALID = 0x0
            PT_TYPE_NAMED_VALUE = 0x1
            PT_TYPE_CONTROL_POINT = 0x2
            PT_TYPE_CONTROL_POINT_RANGE = 0x3
            PT_TYPE_COUNT = 0x4
        class ParticleAttachment_t:
            PATTACH_INVALID = 0xFFFFFFFFFFFFFFFF
            PATTACH_ABSORIGIN = 0x0
            PATTACH_ABSORIGIN_FOLLOW = 0x1
            PATTACH_CUSTOMORIGIN = 0x2
            PATTACH_CUSTOMORIGIN_FOLLOW = 0x3
            PATTACH_POINT = 0x4
            PATTACH_POINT_FOLLOW = 0x5
            PATTACH_EYES_FOLLOW = 0x6
            PATTACH_OVERHEAD_FOLLOW = 0x7
            PATTACH_WORLDORIGIN = 0x8
            PATTACH_ROOTBONE_FOLLOW = 0x9
            PATTACH_RENDERORIGIN_FOLLOW = 0xA
            PATTACH_MAIN_VIEW = 0xB
            PATTACH_WATERWAKE = 0xC
            PATTACH_CENTER_FOLLOW = 0xD
            PATTACH_CUSTOM_GAME_STATE_1 = 0xE
            PATTACH_HEALTHBAR = 0xF
            MAX_PATTACH_TYPES = 0x10
        class CNmEventTargetEntity_t:
            Self = 0x0
            Weapon = 0x1
            HeldItem = 0x2
            Custom = 0x3
        class FieldNetworkOption:
            Auto = 0x0
            ForceEnable = 0x1
            ForceDisable = 0x2
        class NmGraphEventTypeCondition_t:
            Entry = 0x0
            FullyInState = 0x1
            Exit = 0x2
            Timed = 0x3
            Generic = 0x4
            Any = 0x5
        class CNmTransitionNode__TransitionOptions_t:
            None_ = 0x0
            ClampDuration = 0x1
            Synchronized = 0x2
            MatchSourceTime = 0x3
            MatchSyncEventIndex = 0x4
            MatchSyncEventID = 0x5
            MatchSyncEventPercentage = 0x6
            PreferClosestSyncEventID = 0x7
            MatchTimeInSeconds = 0x8
            OffsetTimeInSeconds = 0x9
        class CNmFloatComparisonNode__Comparison_t:
            GreaterThanEqual = 0x0
            LessThanEqual = 0x1
            NearEqual = 0x2
            GreaterThan = 0x3
            LessThan = 0x4
        class VPhysXJoint_t__Flags_t:
            JOINT_FLAGS_NONE = 0x0
            JOINT_FLAGS_BODY1_FIXED = 0x1
            JOINT_FLAGS_USE_BLOCK_SOLVER = 0x2
        class ScriptedHeldWeaponBehavior_t:
            eInvalid = 0xFFFFFFFFFFFFFFFF
            eHolster = 0x0
            eDeploy = 0x1
            eDrop = 0x2
        class VelocityMetricMode:
            DirectionOnly = 0x0
            MagnitudeOnly = 0x1
            DirectionAndMagnitude = 0x2
        class FacingMode:
            FacingMode_Invalid = 0x0
            FacingMode_Manual = 0x1
            FacingMode_Path = 0x2
            FacingMode_LookTarget = 0x3
            FacingMode_ManualPosition = 0x4
        class VertexAlbedoFormat_t:
            VERTEX_ALBEDO_NONE = 0x0
            VERTEX_ALBEDO_8888 = 0x1
            VERTEX_ALBEDO_565 = 0x2
        class AimMatrixBlendMode:
            AimMatrixBlendMode_None = 0x0
            AimMatrixBlendMode_Additive = 0x1
            AimMatrixBlendMode_ModelSpaceAdditive = 0x2
            AimMatrixBlendMode_BoneMask = 0x3
        class AnimationSnapshotType_t:
            ANIMATION_SNAPSHOT_SERVER_SIMULATION = 0x0
            ANIMATION_SNAPSHOT_CLIENT_SIMULATION = 0x1
            ANIMATION_SNAPSHOT_CLIENT_PREDICTION = 0x2
            ANIMATION_SNAPSHOT_CLIENT_INTERPOLATION = 0x3
            ANIMATION_SNAPSHOT_CLIENT_RENDER = 0x4
            ANIMATION_SNAPSHOT_FINAL_COMPOSITE = 0x5
            ANIMATION_SNAPSHOT_MAX = 0x6
        class NmTargetWarpRule_t:
            WarpXY = 0x0
            WarpZ = 0x1
            WarpXYZ = 0x2
            RotationOnly = 0x3
            FixedSection = 0x4
        class FootFallTagFoot_t:
            FOOT1 = 0x0
            FOOT2 = 0x1
            FOOT3 = 0x2
            FOOT4 = 0x3
            FOOT5 = 0x4
            FOOT6 = 0x5
            FOOT7 = 0x6
            FOOT8 = 0x7
        class ChoiceMethod:
            WeightedRandom = 0x0
            WeightedRandomNoRepeat = 0x1
            Iterate = 0x2
            IterateRandom = 0x3
        class AnimVectorSource:
            MoveDirection = 0x0
            FacingPosition = 0x1
            LookDirection = 0x2
            VectorParameter = 0x3
            WayPointDirection = 0x4
            Acceleration = 0x5
            SlopeNormal = 0x6
            SlopeNormal_WorldSpace = 0x7
            LookTarget = 0x8
            LookTarget_WorldSpace = 0x9
            WayPointPosition = 0xA
            GoalPosition = 0xB
            RootMotionVelocity = 0xC
            ManualTarget_WorldSpace = 0xD
        class IkTargetType:
            IkTarget_Attachment = 0x0
            IkTarget_Bone = 0x1
            IkTarget_Parameter_ModelSpace = 0x2
            IkTarget_Parameter_WorldSpace = 0x3
        class RenderMeshSlotType_t:
            RENDERMESH_SLOT_INVALID = 0xFFFFFFFFFFFFFFFF
            RENDERMESH_SLOT_PER_VERTEX = 0x0
            RENDERMESH_SLOT_PER_INSTANCE = 0x1
        class BoneMaskBlendSpace:
            BlendSpace_Parent = 0x0
            BlendSpace_Model = 0x1
            BlendSpace_Model_RotationOnly = 0x2
            BlendSpace_Model_TranslationOnly = 0x3
        class MovementCapability_t:
            eStrafe = 0x0
            eIdleTurn = 0x1
            eStart = 0x2
            eStop = 0x3
            eInstantStop = 0x4
            eShuffle = 0x5
            ePlantedTurn = 0x6
            eUseStartAsPlantedTurn = 0x7
            eLean = 0x8
            eForwardStartOnly = 0x9
            eCount = 0xA
        class ModelConfigAttachmentType_t:
            MODEL_CONFIG_ATTACHMENT_INVALID = 0xFFFFFFFFFFFFFFFF
            MODEL_CONFIG_ATTACHMENT_BONE_OR_ATTACHMENT = 0x0
            MODEL_CONFIG_ATTACHMENT_ROOT_RELATIVE = 0x1
            MODEL_CONFIG_ATTACHMENT_BONEMERGE = 0x2
            MODEL_CONFIG_ATTACHMENT_COUNT = 0x3
        class BinaryNodeChildOption:
            Child1 = 0x0
            Child2 = 0x1
        class NPCPhysicsHullType_t:
            eInvalid = 0x0
            eGroundCapsule = 0x1
            eCenteredCapsule = 0x2
            eGenericCapsule = 0x3
            eGroundBox = 0x4
            eGroundCylinder = 0x5
            eCenteredCylinder = 0x6
        class JumpCorrectionMethod:
            ScaleMotion = 0x0
            AddCorrectionDelta = 0x1
        class MoodType_t:
            eMoodType_Head = 0x0
            eMoodType_Body = 0x1
        class CPulse_ResumePoint:
            pass
        class CParticleBindingRealPulse:
            pass
        class CPulse_OutflowConnection:
            m_SourceOutflowName = 0x0 # PulseSymbol_t
            m_nDestChunk = 0x10 # PulseRuntimeChunkIndex_t
            m_nInstruction = 0x14 # int32
            m_OutflowRegisterMap = 0x18 # PulseRegisterMap_t
        class CBasePulseGraphInstance:
            pass
        class SignatureOutflow_Resume:
            pass
        class SignatureOutflow_Continue:
            pass
        class CParticleCollectionBindingInstance:
            pass
        class CPulseCell_IsRequirementValid__Criteria_t:
            m_bIsValid = 0x0 # bool
        class CPulseCell_Unknown:
            m_UnknownKeys = 0x48 # KeyValues3
        class CPulseCell_LimitCount__Criteria_t:
            m_bLimitCountPasses = 0x0 # bool
        class CPulseExecCursor:
            pass
        class CNmFootIKTask:
            m_nLeftEffectorBoneIdx = 0x70 # int32
            m_nRightEffectorBoneIdx = 0x74 # int32
            m_leftTargetTransform = 0x80 # CTransform
            m_rightTargetTransform = 0xA0 # CTransform
            m_nLeftTargetBoneIdx = 0xC0 # int32
            m_nRightTargetBoneIdx = 0xC4 # int32
            m_leftTarget = 0xD0 # CNmTarget
            m_rightTarget = 0x100 # CNmTarget
            m_blendMode = 0x130 # NmIKBlendMode_t
            m_flBlendWeight = 0x134 # float32
            m_bIsTargetInWorldSpace = 0x138 # bool
            m_bIsRunningFromDeserializedData = 0x139 # bool
        class CNmScaleTask:
            pass
        class CAnimNodePath:
            m_path = 0x0 # AnimNodeID[11]
            m_nCount = 0x2C # int32
        class AnimNodeOutputID:
            m_id = 0x0 # uint32
        class CSeqBoneMaskList:
            m_sName = 0x0 # CBufferString
            m_nLocalBoneArray = 0x10 # CUtlVector<int16>
            m_flBoneWeightArray = 0x28 # CUtlVector<float32>
            m_flDefaultMorphCtrlWeight = 0x40 # float32
            m_morphCtrlWeightArray = 0x48 # CUtlVector<std::pair<CBufferString,float32>>
        class SampleCode:
            m_subCode = 0x0 # uint8[8]
        class AnimationDecodeDebugDumpElement_t:
            m_nEntityIndex = 0x0 # int32
            m_modelName = 0x8 # CUtlString
            m_poseParams = 0x10 # CUtlVector<CUtlString>
            m_decodeOps = 0x28 # CUtlVector<CUtlString>
            m_internalOps = 0x40 # CUtlVector<CUtlString>
            m_decodedAnims = 0x58 # CUtlVector<CUtlString>
        class ConfigIndex:
            m_nGroup = 0x0 # uint16
            m_nConfig = 0x2 # uint16
        class CSeqSeqDescFlag:
            m_bLooping = 0x0 # bool
            m_bSnap = 0x1 # bool
            m_bAutoplay = 0x2 # bool
            m_bPost = 0x3 # bool
            m_bHidden = 0x4 # bool
            m_bMulti = 0x5 # bool
            m_bLegacyDelta = 0x6 # bool
            m_bLegacyWorldspace = 0x7 # bool
            m_bLegacyCyclepose = 0x8 # bool
            m_bLegacyRealtime = 0x9 # bool
            m_bModelDoc = 0xA # bool
        class IKBoneNameAndIndex_t:
            m_Name = 0x0 # CUtlString
        class CNmPoseNode__CDefinition:
            pass
        class CNmBlendTask:
            pass
        class CNmVectorValueNode__CDefinition:
            pass
        class CSeqTransition:
            m_flFadeInTime = 0x0 # float32
            m_flFadeOutTime = 0x4 # float32
        class CNmPoseTask:
            pass
        class CNmCachedPoseReadTask:
            pass
        class CAnimParamHandle:
            m_type = 0x0 # AnimParamType_t
            m_index = 0x1 # uint8
        class WeightList:
            m_name = 0x0 # CUtlString
            m_weights = 0x8 # CUtlVector<float32>
        class CSeqAutoLayerFlag:
            m_bPost = 0x0 # bool
            m_bSpline = 0x1 # bool
            m_bXFade = 0x2 # bool
            m_bNoBlend = 0x3 # bool
            m_bLocal = 0x4 # bool
            m_bPose = 0x5 # bool
            m_bFetchFrame = 0x6 # bool
            m_bSubtract = 0x7 # bool
        class AnimStateID:
            m_id = 0x0 # uint32
        class CSeqPoseSetting:
            m_sPoseParameter = 0x0 # CBufferString
            m_sAttachment = 0x10 # CBufferString
            m_sReferenceSequence = 0x20 # CBufferString
            m_flValue = 0x30 # float32
            m_bX = 0x34 # bool
            m_bY = 0x35 # bool
            m_bZ = 0x36 # bool
            m_eType = 0x38 # int32
        class CDrawCullingData:
            m_ConeAxis = 0x0 # int8[3]
            m_ConeCutoff = 0x3 # int8
        class CSeqMultiFetchFlag:
            m_bRealtime = 0x0 # bool
            m_bCylepose = 0x1 # bool
            m_b0D = 0x2 # bool
            m_b1D = 0x3 # bool
            m_b2D = 0x4 # bool
            m_b2D_TRI = 0x5 # bool
        class CSolveIKTargetHandle_t:
            m_positionHandle = 0x0 # CAnimParamHandle
            m_orientationHandle = 0x2 # CAnimParamHandle
        class CNmZeroPoseTask:
            pass
        class CNmBlendTaskBase:
            pass
        class CNewParticleEffect:
            m_pNext = 0x10 # CNewParticleEffect*
            m_pPrev = 0x18 # CNewParticleEffect*
            m_pParticles = 0x20 # IParticleCollection*
            m_pDebugName = 0x28 # char*
            m_bDontRemove = 0x0 # bitfield:1
            m_bRemove = 0x0 # bitfield:1
            m_bNeedsBBoxUpdate = 0x0 # bitfield:1
            m_bIsFirstFrame = 0x0 # bitfield:1
            m_bAutoUpdateBBox = 0x0 # bitfield:1
            m_bAllocated = 0x0 # bitfield:1
            m_bSimulate = 0x0 # bitfield:1
            m_bShouldPerformCullCheck = 0x0 # bitfield:1
            m_bForceNoDraw = 0x0 # bitfield:1
            m_bSuppressScreenSpaceEffect = 0x0 # bitfield:1
            m_bShouldSave = 0x0 # bitfield:1
            m_bShouldSimulateDuringGamePaused = 0x0 # bitfield:1
            m_bShouldCheckFoW = 0x0 # bitfield:1
            m_bIsAsyncCreate = 0x0 # bitfield:1
            m_bFreezeTransitionActive = 0x0 # bitfield:1
            m_bFreezeTargetState = 0x0 # bitfield:1
            m_bCanFreeze = 0x0 # bitfield:1
            m_vSortOrigin = 0x40 # Vector
            m_flScale = 0x4C # float32
            m_hOwner = 0x50 # PARTICLE_EHANDLE__*
            m_pOwningParticleProperty = 0x58 # CParticleProperty*
            m_flFreezeTransitionStart = 0x70 # float32
            m_flFreezeTransitionDuration = 0x74 # float32
            m_flFreezeTransitionOverride = 0x78 # float32
            m_LastMin = 0x7C # Vector
            m_LastMax = 0x88 # Vector
            m_nSplitScreenUser = 0x94 # CSplitScreenSlot
            m_vecAggregationCenter = 0x98 # Vector
            m_RefCount = 0xD0 # int32
        class CNmReferencePoseTask:
            pass
        class CSeqSynthAnimDesc:
            m_sName = 0x0 # CBufferString
            m_flags = 0x10 # CSeqSeqDescFlag
            m_transition = 0x1C # CSeqTransition
            m_nLocalBaseReference = 0x24 # int16
            m_nLocalBoneMask = 0x26 # int16
            m_activityArray = 0x28 # CUtlVector<CAnimActivity>
        class IAnimationGraphInstance:
            pass
        class CParamSpanUpdater:
            m_spans = 0x0 # CUtlVector<ParamSpan_t>
        class CNmContactAudioTypeVData:
            pass
        class AnimationSnapshotBase_t:
            m_flRealTime = 0x0 # float32
            m_rootToWorld = 0x10 # matrix3x4a_t
            m_bBonesInWorldSpace = 0x40 # bool
            m_boneSetupMask = 0x48 # CUtlVector<uint32>
            m_boneTransforms = 0x60 # CUtlVector<matrix3x4a_t>
            m_flexControllers = 0x78 # CUtlVector<float32>
            m_SnapshotType = 0x90 # AnimationSnapshotType_t
            m_bHasDecodeDump = 0x94 # bool
            m_DecodeDump = 0x98 # AnimationDecodeDebugDumpElement_t
        class CNmBoneMaskValueNode__CDefinition:
            pass
        class CSeqPoseParamDesc:
            m_sName = 0x0 # CBufferString
            m_flStart = 0x10 # float32
            m_flEnd = 0x14 # float32
            m_flLoop = 0x18 # float32
            m_bLooping = 0x1C # bool
        class CAnimMovement:
            endframe = 0x0 # int32
            motionflags = 0x4 # int32
            v0 = 0x8 # float32
            v1 = 0xC # float32
            angle = 0x10 # float32
            vector = 0x14 # Vector
            position = 0x20 # Vector
        class CNmIDValueNode__CDefinition:
            pass
        class CNmChainLookatTask:
            pass
        class CAnimLocalHierarchy:
            m_sBone = 0x0 # CBufferString
            m_sNewParent = 0x10 # CBufferString
            m_nStartFrame = 0x20 # int32
            m_nPeakFrame = 0x24 # int32
            m_nTailFrame = 0x28 # int32
            m_nEndFrame = 0x2C # int32
        class CNmSampleTask:
            pass
        class CSequenceGroupData:
            m_sName = 0x10 # CBufferString
            m_nFlags = 0x20 # uint32
            m_localSequenceNameArray = 0x28 # CUtlVector<CBufferString>
            m_localS1SeqDescArray = 0x40 # CUtlVector<CSeqS1SeqDesc>
            m_localMultiSeqDescArray = 0x58 # CUtlVector<CSeqS1SeqDesc>
            m_localSynthAnimDescArray = 0x70 # CUtlVector<CSeqSynthAnimDesc>
            m_localCmdSeqDescArray = 0x88 # CUtlVector<CSeqCmdSeqDesc>
            m_localBoneMaskArray = 0xA0 # CUtlVector<CSeqBoneMaskList>
            m_localScaleSetArray = 0xB8 # CUtlVector<CSeqScaleSet>
            m_localBoneNameArray = 0xD0 # CUtlVector<CBufferString>
            m_localNodeName = 0xE8 # CBufferString
            m_localPoseParamArray = 0xF8 # CUtlVector<CSeqPoseParamDesc>
            m_keyValues = 0x110 # KeyValues3
            m_localIKAutoplayLockArray = 0x120 # CUtlVector<CSeqIKLock>
        class CAnimEventDefinition:
            m_nFrame = 0x8 # int32
            m_nEndFrame = 0xC # int32
            m_flCycle = 0x10 # float32
            m_flDuration = 0x14 # float32
            m_EventData = 0x18 # KeyValues3
            m_sLegacyOptions = 0x28 # CBufferString
            m_sEventName = 0x38 # CGlobalSymbol
        class CBoneConstraintPoseSpaceMorph__Input_t:
            m_inputValue = 0x0 # Vector
            m_outputWeightList = 0x10 # CUtlVector<float32>
        class MoodAnimationLayer_t:
            m_sName = 0x0 # CUtlString
            m_bActiveListening = 0x8 # bool
            m_bActiveTalking = 0x9 # bool
            m_layerAnimations = 0x10 # CUtlVector<MoodAnimation_t>
            m_flIntensity = 0x28 # CRangeFloat
            m_flDurationScale = 0x30 # CRangeFloat
            m_bScaleWithInts = 0x38 # bool
            m_flNextStart = 0x3C # CRangeFloat
            m_flStartOffset = 0x44 # CRangeFloat
            m_flEndOffset = 0x4C # CRangeFloat
            m_flFadeIn = 0x54 # float32
            m_flFadeOut = 0x58 # float32
        class CNmBoolValueNode__CDefinition:
            pass
        class IParticleEffect:
            pass
        class PARTICLE_EHANDLE__:
            unused = 0x0 # int32
        class CAnimEnum:
            m_value = 0x0 # uint8
        class CAnimDecoder:
            m_szName = 0x0 # CBufferString
            m_nVersion = 0x10 # int32
            m_nType = 0x14 # int32
        class CAnimFrameSegment:
            m_nUniqueFrameIndex = 0x0 # int32
            m_nLocalElementMasks = 0x4 # uint32
            m_nLocalChannel = 0x8 # int32
            m_container = 0x10 # CUtlBinaryBlock
        class CAnimDesc_Flag:
            m_bLooping = 0x0 # bool
            m_bAllZeros = 0x1 # bool
            m_bHidden = 0x2 # bool
            m_bDelta = 0x3 # bool
            m_bLegacyWorldspace = 0x4 # bool
            m_bModelDoc = 0x5 # bool
            m_bImplicitSeqIgnoreDelta = 0x6 # bool
            m_bAnimGraphAdditive = 0x7 # bool
        class CTransitionUpdateData:
            m_srcStateIndex = 0x0 # uint8
            m_destStateIndex = 0x1 # uint8
            m_nHandshakeMaskToDisableFirst = 0x0 # bitfield:7
            m_bDisabled = 0x0 # bitfield:1
        class CBoneConstraintPoseSpaceBone__Input_t:
            m_inputValue = 0x0 # Vector
            m_outputTransformList = 0x10 # CUtlVector<CTransform>
        class CSeqMultiFetch:
            m_flags = 0x0 # CSeqMultiFetchFlag
            m_localReferenceArray = 0x8 # CUtlVector<int16>
            m_nGroupSize = 0x20 # int32[2]
            m_nLocalPose = 0x28 # int32[2]
            m_poseKeyArray0 = 0x30 # CUtlVector<float32>
            m_poseKeyArray1 = 0x48 # CUtlVector<float32>
            m_nLocalCyclePoseParameter = 0x60 # int32
            m_bCalculatePoseParameters = 0x64 # bool
            m_bFixedBlendWeight = 0x65 # bool
            m_flFixedBlendWeightVals = 0x68 # float32[2]
        class CNmOverlayBlendTask:
            pass
        class CNmValueNode__CDefinition:
            pass
        class CSeqS1SeqDesc:
            m_sName = 0x0 # CBufferString
            m_flags = 0x10 # CSeqSeqDescFlag
            m_fetch = 0x20 # CSeqMultiFetch
            m_nLocalWeightlist = 0x90 # int32
            m_autoLayerArray = 0x98 # CUtlVector<CSeqAutoLayer>
            m_IKLockArray = 0xB0 # CUtlVector<CSeqIKLock>
            m_transition = 0xC8 # CSeqTransition
            m_SequenceKeys = 0xD0 # KeyValues3
            m_LegacyKeyValueText = 0xE0 # CBufferString
            m_activityArray = 0xF0 # CUtlVector<CAnimActivity>
            m_footMotion = 0x108 # CUtlVector<CFootMotion>
        class CNmGraphInstance:
            pass
        class CSeqCmdLayer:
            m_cmd = 0x0 # int16
            m_nLocalReference = 0x2 # int16
            m_nLocalBonemask = 0x4 # int16
            m_nDstResult = 0x6 # int16
            m_nSrcResult = 0x8 # int16
            m_bSpline = 0xA # bool
            m_flVar1 = 0xC # float32
            m_flVar2 = 0x10 # float32
            m_nLineNumber = 0x14 # int16
        class CNmFloatValueNode__CDefinition:
            pass
        class CNmTwoBoneIKTask:
            m_nEffectorBoneIdx = 0x70 # int32
            m_nEffectorTargetBoneIdx = 0x74 # int32
            m_targetTransform = 0x80 # CTransform
        class CNmContactAudioActionVData:
            pass
        class CSeqIKLock:
            m_flPosWeight = 0x0 # float32
            m_flAngleWeight = 0x4 # float32
            m_nLocalBone = 0x8 # int16
            m_bBonesOrientedAlongPositiveX = 0xA # bool
        class CNmFollowBoneTask:
            pass
        class CNmTargetValueNode__CDefinition:
            pass
        class CPoseHandle:
            m_nIndex = 0x0 # uint16
            m_eType = 0x2 # PoseType_t
        class CSeqCmdSeqDesc:
            m_sName = 0x0 # CBufferString
            m_flags = 0x10 # CSeqSeqDescFlag
            m_transition = 0x1C # CSeqTransition
            m_nFrameRangeSequence = 0x24 # int16
            m_nFrameCount = 0x26 # int16
            m_flFPS = 0x28 # float32
            m_nSubCycles = 0x2C # int16
            m_numLocalResults = 0x2E # int16
            m_cmdLayerArray = 0x30 # CUtlVector<CSeqCmdLayer>
            m_eventArray = 0x48 # CUtlVector<CAnimEventDefinition>
            m_activityArray = 0x60 # CUtlVector<CAnimActivity>
            m_poseSettingArray = 0x78 # CUtlVector<CSeqPoseSetting>
        class CNmCachedPoseWriteTask:
            pass
        class CNmModelSpaceBlendTask:
            pass
        class AnimParamID:
            m_id = 0x0 # uint32
        class AnimationDecodeDebugDump_t:
            m_processingType = 0x0 # AnimationProcessingType_t
            m_elems = 0x8 # CUtlVector<AnimationDecodeDebugDumpElement_t>
        class CSeqScaleSet:
            m_sName = 0x0 # CBufferString
            m_bRootOffset = 0x10 # bool
            m_vRootOffset = 0x14 # Vector
            m_nLocalBoneArray = 0x20 # CUtlVector<int16>
            m_flBoneScaleArray = 0x38 # CUtlVector<float32>
        class IKTargetSettings_t:
            m_TargetSource = 0x0 # IKTargetSource
            m_Bone = 0x8 # IKBoneNameAndIndex_t
            m_AnimgraphParameterNamePosition = 0x18 # AnimParamID
            m_AnimgraphParameterNameOrientation = 0x1C # AnimParamID
            m_TargetCoordSystem = 0x20 # IKTargetCoordinateSystem
        class CNmAdditiveBlendTask:
            pass
        class CSeqAutoLayer:
            m_nLocalReference = 0x0 # int16
            m_nLocalPose = 0x2 # int16
            m_flags = 0x4 # CSeqAutoLayerFlag
            m_start = 0xC # float32
            m_peak = 0x10 # float32
            m_tail = 0x14 # float32
            m_end = 0x18 # float32
        class CAnimSequenceParams:
            m_flFadeInTime = 0x0 # float32
            m_flFadeOutTime = 0x4 # float32
        class CAnimData:
            m_name = 0x10 # CBufferString
            m_animArray = 0x20 # CUtlVector<CAnimDesc>
            m_decoderArray = 0x38 # CUtlVector<CAnimDecoder>
            m_nMaxUniqueFrameIndex = 0x50 # int32
            m_segmentArray = 0x58 # CUtlVector<CAnimFrameSegment>
        class IKSolverSettings_t:
            m_SolverType = 0x0 # IKSolverType
            m_nNumIterations = 0x4 # int32
            m_EndEffectorRotationFixUpMode = 0x8 # EIKEndEffectorRotationFixUpMode
        class CAnimTagManagerUpdater:
            m_tags = 0x38 # CUtlVector<CSmartPtr<CAnimTagBase>>
