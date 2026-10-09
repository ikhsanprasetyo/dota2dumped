// Generated using https://github.com/ikhsanprasetyo/source2-dumper
// 2026-10-09 13:46:49.828411900 +07:00

#pragma once

#include <cstddef>
#include <cstdint>

namespace source2_dumper {
    namespace schemas {
        // Module: animationsystem.dll
        // Class count: 90
        // Enum count: 145
        namespace animationsystem_dll {
            // Alignment: 4
            // Member count: 2
            enum class PulseBestOutflowRules_t : uint32_t {
                SORT_BY_NUMBER_OF_VALID_CRITERIA = 0x0,
                SORT_BY_OUTFLOW_INDEX = 0x1
            };
            // Alignment: 4
            // Member count: 4
            enum class PulseCursorCancelPriority_t : uint32_t {
                None = 0x0,
                CancelOnSucceeded = 0x1,
                SoftCancel = 0x2,
                HardCancel = 0x3
            };
            // Alignment: 4
            // Member count: 2
            enum class PulseMethodCallMode_t : uint32_t {
                SYNC_WAIT_FOR_COMPLETION = 0x0,
                ASYNC_FIRE_AND_FORGET = 0x1
            };
            // Alignment: 4
            // Member count: 2
            enum class PulseCursorWakePriority_t : uint32_t {
                WakeElegantly = 0x0,
                WakeImmediate = 0x1
            };
            // Alignment: 4
            // Member count: 6
            enum class ParticleSetMethod_t : uint32_t {
                PARTICLE_SET_REPLACE_VALUE = 0x0,
                PARTICLE_SET_SCALE_INITIAL_VALUE = 0x1,
                PARTICLE_SET_ADD_TO_INITIAL_VALUE = 0x2,
                PARTICLE_SET_RAMP_CURRENT_VALUE = 0x3,
                PARTICLE_SET_SCALE_CURRENT_VALUE = 0x4,
                PARTICLE_SET_ADD_TO_CURRENT_VALUE = 0x5
            };
            // Alignment: 4
            // Member count: 17
            enum class SeqCmd_t : uint32_t {
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
            enum class CNmEventRelevance_t : uint32_t {
                ClientOnly = 0x0,
                ServerOnly = 0x1,
                ClientAndServer = 0x2
            };
            // Alignment: 4
            // Member count: 4
            enum class BoneTransformSpace_t : uint32_t {
                BoneTransformSpace_Invalid = 0xFFFFFFFF,
                BoneTransformSpace_Parent = 0x0,
                BoneTransformSpace_Model = 0x1,
                BoneTransformSpace_World = 0x2
            };
            // Alignment: 4
            // Member count: 5
            enum class CAnimationGraphVisualizerPrimitiveType : uint32_t {
                ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Text = 0x0,
                ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Sphere = 0x1,
                ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Line = 0x2,
                ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Pie = 0x3,
                ANIMATIONGRAPHVISUALIZERPRIMITIVETYPE_Axis = 0x4
            };
            // Alignment: 1
            // Member count: 3
            enum class NmTransitionRule_t : uint8_t {
                AllowTransition = 0x0,
                ConditionallyAllowTransition = 0x1,
                BlockTransition = 0x2
            };
            // Alignment: 4
            // Member count: 3
            enum class BinaryNodeTiming : uint32_t {
                UseChild1 = 0x0,
                UseChild2 = 0x1,
                SyncChildren = 0x2
            };
            // Alignment: 1
            // Member count: 3
            enum class NmFollowBoneMode_t : uint8_t {
                RotationAndTranslation = 0x0,
                RotationOnly = 0x1,
                TranslationOnly = 0x2
            };
            // Alignment: 4
            // Member count: 7
            enum class SolveIKChainAnimNodeDebugSetting : uint32_t {
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
            enum class CNmParticleEvent__Type_t : uint32_t {
                Create = 0x0,
                Create_CFG = 0x1
            };
            // Alignment: 4
            // Member count: 5
            enum class ParticleFloatBiasType_t : uint32_t {
                PF_BIAS_TYPE_INVALID = 0xFFFFFFFF,
                PF_BIAS_TYPE_STANDARD = 0x0,
                PF_BIAS_TYPE_GAIN = 0x1,
                PF_BIAS_TYPE_EXPONENTIAL = 0x2,
                PF_BIAS_TYPE_COUNT = 0x3
            };
            // Alignment: 1
            // Member count: 4
            enum class CNmTargetWarpNode__TargetUpdateRule_t : uint8_t {
                None = 0x0,
                Recalculate = 0x1,
                Offset = 0x2,
                RecalculateOrOffset = 0x3
            };
            // Alignment: 1
            // Member count: 6
            enum class SharedMovementGait_t : uint8_t {
                eInvalid = 0xFF,
                eSlow = 0x0,
                eMedium = 0x1,
                eFast = 0x2,
                eVeryFast = 0x3,
                eCount = 0x4
            };
            // Alignment: 4
            // Member count: 3
            enum class OrientationWarpRootMotionSource_t : uint32_t {
                eAnimationOrProcedural = 0x0,
                eAnimationOnly = 0x1,
                eProceduralOnly = 0x2
            };
            // Alignment: 4
            // Member count: 3
            enum class IKTargetCoordinateSystem : uint32_t {
                IKTARGETCOORDINATESYSTEM_WorldSpace = 0x0,
                IKTARGETCOORDINATESYSTEM_ModelSpace = 0x1,
                IKTARGETCOORDINATESYSTEM_COUNT = 0x2
            };
            // Alignment: 4
            // Member count: 34
            enum class ParticleFloatType_t : uint32_t {
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
                PF_TYPE_CONTROL_POINT_DISTANCE = 0x9,
                PF_TYPE_PARTICLE_DETAIL_LEVEL = 0xA,
                PF_TYPE_CONCURRENT_DEF_COUNT = 0xB,
                PF_TYPE_CLOSEST_CAMERA_DISTANCE = 0xC,
                PF_TYPE_SNAPSHOT_COUNT = 0xD,
                PF_TYPE_SNAPSHOT_CHANGED = 0xE,
                PF_TYPE_CONTROL_POINT_IS_SET = 0xF,
                PF_TYPE_RENDERER_CAMERA_DISTANCE = 0x10,
                PF_TYPE_RENDERER_CAMERA_DOT_PRODUCT = 0x11,
                PF_TYPE_PARTICLE_NOISE = 0x12,
                PF_TYPE_PARTICLE_AGE = 0x13,
                PF_TYPE_PARTICLE_AGE_NORMALIZED = 0x14,
                PF_TYPE_PARTICLE_FLOAT = 0x15,
                PF_TYPE_PARTICLE_INITIAL_FLOAT = 0x16,
                PF_TYPE_PARTICLE_VECTOR_COMPONENT = 0x17,
                PF_TYPE_PARTICLE_INITIAL_VECTOR_COMPONENT = 0x18,
                PF_TYPE_PARTICLE_SPEED = 0x19,
                PF_TYPE_PARTICLE_NUMBER = 0x1A,
                PF_TYPE_PARTICLE_NUMBER_NORMALIZED = 0x1B,
                PF_TYPE_PARTICLE_ROPE_SEGMENT = 0x1C,
                PF_TYPE_PARTICLE_ROPE_SEGMENT_NORMALIZED = 0x1D,
                PF_TYPE_PARTICLE_SCREENSPACE_CAMERA_DISTANCE = 0x1E,
                PF_TYPE_PARTICLE_SCREENSPACE_CAMERA_DOT_PRODUCT = 0x1F,
                PF_TYPE_COUNT = 0x20
            };
            // Alignment: 1
            // Member count: 4
            enum class CNmFloatAngleMathNode__Operation_t : uint8_t {
                ClampTo180 = 0x0,
                ClampTo360 = 0x1,
                FlipHemisphere = 0x2,
                FlipHemisphereNegate = 0x3
            };
            // Alignment: 4
            // Member count: 3
            enum class VPhysXAggregateData_t__VPhysXFlagEnum_t : uint32_t {
                FLAG_IS_POLYSOUP_GEOMETRY = 0x1,
                FLAG_LEVEL_COLLISION = 0x10,
                FLAG_IGNORE_SCALE_OBSOLETE_DO_NOT_USE = 0x20
            };
            // Alignment: 1
            // Member count: 5
            enum class CNmRootMotionOverrideNode__OverrideFlags_t : uint8_t {
                AllowMoveX = 0x0,
                AllowMoveY = 0x1,
                AllowMoveZ = 0x2,
                AllowFacingPitch = 0x3,
                ListenForEvents = 0x4
            };
            // Alignment: 1
            // Member count: 23
            enum class NmEasingOperation_t : uint8_t {
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
            enum class EIKEndEffectorRotationFixUpMode : uint32_t {
                None = 0x0,
                MatchTargetOrientation = 0x1,
                LookAtTargetForward = 0x2,
                MaintainParentOrientation = 0x3,
                Count = 0x4
            };
            // Alignment: 4
            // Member count: 2
            enum class MatterialAttributeTagType_t : uint32_t {
                MATERIAL_ATTRIBUTE_TAG_VALUE = 0x0,
                MATERIAL_ATTRIBUTE_TAG_COLOR = 0x1
            };
            // Alignment: 4
            // Member count: 6
            enum class PFNoiseTurbulence_t : uint32_t {
                PF_NOISE_TURB_NONE = 0x0,
                PF_NOISE_TURB_HIGHLIGHT = 0x1,
                PF_NOISE_TURB_FEEDBACK = 0x2,
                PF_NOISE_TURB_LOOPY = 0x3,
                PF_NOISE_TURB_CONTRAST = 0x4,
                PF_NOISE_TURB_ALTERNATE = 0x5
            };
            // Alignment: 1
            // Member count: 4
            enum class NmTargetWarpAlgorithm_t : uint8_t {
                Lerp = 0x0,
                Hermite = 0x1,
                HermiteFeaturePreserving = 0x2,
                Bezier = 0x3
            };
            // Alignment: 4
            // Member count: 5
            enum class ParticleColorBlendMode_t : uint32_t {
                PARTICLEBLEND_DEFAULT = 0x0,
                PARTICLEBLEND_OVERLAY = 0x1,
                PARTICLEBLEND_DARKEN = 0x2,
                PARTICLEBLEND_LIGHTEN = 0x3,
                PARTICLEBLEND_MULTIPLY = 0x4
            };
            // Alignment: 4
            // Member count: 13
            enum class ParticleColorBlendType_t : uint32_t {
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
            enum class NmTransitionRuleCondition_t : uint8_t {
                AnyAllowed = 0x0,
                FullyAllowed = 0x1,
                ConditionallyAllowed = 0x2,
                Blocked = 0x3
            };
            // Alignment: 2
            // Member count: 10
            enum class ModelMeshBufferUsage_t : uint16_t {
                MESH_BUFFER_USAGE_NONE = 0x0,
                MESH_BUFFER_USAGE_VB = 0x1,
                MESH_BUFFER_USAGE_IB = 0x2,
                MESH_BUFFER_USAGE_ADJACENCY = 0x4,
                MESH_BUFFER_USAGE_MESHLET_TRIS = 0x8,
                MESH_BUFFER_USAGE_RT_PROXY = 0x10,
                MESH_BUFFER_USAGE_VERTEX_ALBEDO = 0x20,
                MESH_BUFFER_USAGE_VERTEX_EMISSIVE = 0x40,
                MESH_BUFFER_USAGE_MESHLETS = 0x80,
                MESH_BUFFER_USAGE_ALIAS_TABLE = 0x100
            };
            // Alignment: 4
            // Member count: 2
            enum class NmGraphDebugMode_t : uint32_t {
                Off = 0x0,
                On = 0x1
            };
            // Alignment: 4
            // Member count: 2
            enum class TargetWarpTimingMethod : uint32_t {
                ReachDestinationOnRootMotionEnd = 0x0,
                ReachDestinationOnWarpTagEnd = 0x1
            };
            // Alignment: 4
            // Member count: 6
            enum class ScriptedMoveTo_t : uint32_t {
                eWait = 0x0,
                eMoveWithGait = 0x3,
                eTeleport = 0x4,
                eWaitFacing = 0x5,
                eObsoleteBackCompat1 = 0x1,
                eObsoleteBackCompat2 = 0x2
            };
            // Alignment: 4
            // Member count: 2
            enum class EDemoBoneSelectionMode : uint32_t {
                CaptureAllBones = 0x0,
                CaptureSelectedBones = 0x1
            };
            // Alignment: 4
            // Member count: 2
            enum class StepPhase : uint32_t {
                StepPhase_OnGround = 0x0,
                StepPhase_InAir = 0x1
            };
            // Alignment: 4
            // Member count: 26
            enum class FlexOpCode_t : uint32_t {
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
            enum class NmCachedValueMode_t : uint32_t {
                OnEntry = 0x0,
                OnExit = 0x1
            };
            // Alignment: 4
            // Member count: 2
            enum class AnimNodeNetworkMode : uint32_t {
                ServerAuthoritative = 0x0,
                ClientSimulate = 0x1
            };
            // Alignment: 4
            // Member count: 6
            enum class VPhysXBodyPart_t__VPhysXFlagEnum_t : uint32_t {
                FLAG_STATIC = 0x1,
                FLAG_KINEMATIC = 0x2,
                FLAG_JOINT = 0x4,
                FLAG_MASS = 0x8,
                FLAG_ALWAYS_DYNAMIC_ON_CLIENT = 0x10,
                FLAG_DISABLE_CCD = 0x20
            };
            // Alignment: 1
            // Member count: 9
            enum class AnimParamType_t : uint8_t {
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
            enum class NmEasingFunction_t : uint8_t {
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
            enum class ParticleModelType_t : uint32_t {
                PM_TYPE_INVALID = 0x0,
                PM_TYPE_NAMED_VALUE_MODEL = 0x1,
                PM_TYPE_NAMED_VALUE_EHANDLE = 0x2,
                PM_TYPE_CONTROL_POINT = 0x3,
                PM_TYPE_COUNT = 0x4
            };
            // Alignment: 4
            // Member count: 3
            enum class IKTargetSource : uint32_t {
                IKTARGETSOURCE_Bone = 0x0,
                IKTARGETSOURCE_AnimgraphParameter = 0x1,
                IKTARGETSOURCE_COUNT = 0x2
            };
            // Alignment: 4
            // Member count: 15
            enum class PermModelInfo_t__FlagEnum : uint32_t {
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
            enum class CNmFloatMathNode__Operator_t : uint8_t {
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
            enum class CNmSyncEventIndexConditionNode__TriggerMode_t : uint8_t {
                ExactlyAtEventIndex = 0x0,
                GreaterThanEqualToEventIndex = 0x1
            };
            // Alignment: 4
            // Member count: 5
            enum class ParticleFloatRoundType_t : uint32_t {
                PF_ROUND_TYPE_INVALID = 0xFFFFFFFF,
                PF_ROUND_TYPE_NEAREST = 0x0,
                PF_ROUND_TYPE_FLOOR = 0x1,
                PF_ROUND_TYPE_CEIL = 0x2,
                PF_ROUND_TYPE_COUNT = 0x3
            };
            // Alignment: 4
            // Member count: 4
            enum class PFNoiseType_t : uint32_t {
                PF_NOISE_TYPE_PERLIN = 0x0,
                PF_NOISE_TYPE_SIMPLEX = 0x1,
                PF_NOISE_TYPE_WORLEY = 0x2,
                PF_NOISE_TYPE_CURL = 0x3
            };
            // Alignment: 4
            // Member count: 3
            enum class ParticleDirectionNoiseType_t : uint32_t {
                PARTICLE_DIR_NOISE_PERLIN = 0x0,
                PARTICLE_DIR_NOISE_CURL = 0x1,
                PARTICLE_DIR_NOISE_WORLEY_BASIC = 0x2
            };
            // Alignment: 4
            // Member count: 3
            enum class AnimParamNetworkSetting : uint32_t {
                Auto = 0x0,
                AlwaysNetwork = 0x1,
                NeverNetwork = 0x2
            };
            // Alignment: 4
            // Member count: 4
            enum class MorphFlexControllerRemapType_t : uint32_t {
                MORPH_FLEXCONTROLLER_REMAP_PASSTHRU = 0x0,
                MORPH_FLEXCONTROLLER_REMAP_2WAY = 0x1,
                MORPH_FLEXCONTROLLER_REMAP_NWAY = 0x2,
                MORPH_FLEXCONTROLLER_REMAP_EYELID = 0x3
            };
            // Alignment: 4
            // Member count: 8
            enum class MeshDrawPrimitiveFlags_t : uint32_t {
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
            enum class TargetWarpAngleMode_t : uint32_t {
                eFacingHeading = 0x0,
                eMoveHeading = 0x1
            };
            // Alignment: 1
            // Member count: 2
            enum class NmIKBlendMode_t : uint8_t {
                Effector = 0x0,
                Pose = 0x1
            };
            // Alignment: 4
            // Member count: 4
            enum class ModelBoneFlexComponent_t : uint32_t {
                MODEL_BONE_FLEX_INVALID = 0xFFFFFFFF,
                MODEL_BONE_FLEX_TX = 0x0,
                MODEL_BONE_FLEX_TY = 0x1,
                MODEL_BONE_FLEX_TZ = 0x2
            };
            // Alignment: 1
            // Member count: 2
            enum class CNmStateNode__TimedEvent_t__Comparison_t : uint8_t {
                LessThanEqual = 0x0,
                GreaterThanEqual = 0x1
            };
            // Alignment: 1
            // Member count: 3
            enum class PoseType_t : uint8_t {
                POSETYPE_STATIC = 0x0,
                POSETYPE_DYNAMIC = 0x1,
                POSETYPE_INVALID = 0xFF
            };
            // Alignment: 4
            // Member count: 2
            enum class CNmClothEvent__Type_t : uint32_t {
                Stiffen = 0x0,
                Effect = 0x1
            };
            // Alignment: 1
            // Member count: 2
            enum class CNmRootMotionData__SamplingMode_t : uint8_t {
                Delta = 0x0,
                WorldSpace = 0x1
            };
            // Alignment: 1
            // Member count: 9
            enum class NmEventConditionRules_t : uint8_t {
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
            enum class AnimationType_t : uint32_t {
                ANIMATION_TYPE_FIXED_RATE = 0x0,
                ANIMATION_TYPE_FIT_LIFETIME = 0x1,
                ANIMATION_TYPE_MANUAL_FRAMES = 0x2
            };
            // Alignment: 4
            // Member count: 37
            enum class AnimValueSource : uint32_t {
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
            enum class CNmTimeConditionNode__Operator_t : uint8_t {
                LessThan = 0x0,
                LessThanEqual = 0x1,
                GreaterThan = 0x2,
                GreaterThanEqual = 0x3
            };
            // Alignment: 4
            // Member count: 3
            enum class LinearRootMotionBlendMode_t : uint32_t {
                LERP = 0x0,
                NLERP = 0x1,
                SLERP = 0x2
            };
            // Alignment: 4
            // Member count: 1
            enum class RagdollPoseControl : uint32_t {
                Absolute = 0x0
            };
            // Alignment: 4
            // Member count: 6
            enum class IKSolverType : uint32_t {
                IKSOLVER_Perlin = 0x0,
                IKSOLVER_TwoBone = 0x1,
                IKSOLVER_Fabrik = 0x2,
                IKSOLVER_DogLeg3Bone = 0x3,
                IKSOLVER_CCD = 0x4,
                IKSOLVER_COUNT = 0x5
            };
            // Alignment: 4
            // Member count: 2
            enum class TargetWarpCorrectionMethod : uint32_t {
                ScaleMotion = 0x0,
                AddCorrectionDelta = 0x1
            };
            // Alignment: 4
            // Member count: 2
            enum class TargetSelectorAngleMode_t : uint32_t {
                eFacingHeading = 0x0,
                eMoveHeading = 0x1
            };
            // Alignment: 4
            // Member count: 2
            enum class Blend2DMode : uint32_t {
                Blend2DMode_General = 0x0,
                Blend2DMode_Directional = 0x1
            };
            // Alignment: 1
            // Member count: 3
            enum class HandshakeTagState_t : uint8_t {
                eInactive = 0x0,
                eActive = 0x1,
                eMomentarilyInactive = 0x2
            };
            // Alignment: 4
            // Member count: 3
            enum class ChoiceChangeMethod : uint32_t {
                OnReset = 0x0,
                OnCycleEnd = 0x1,
                OnResetOrCycleEnd = 0x2
            };
            // Alignment: 4
            // Member count: 2
            enum class ChoiceBlendMethod : uint32_t {
                SingleBlendTime = 0x0,
                PerChoiceBlendTimes = 0x1
            };
            // Alignment: 4
            // Member count: 4
            enum class VPhysXConstraintParams_t__EnumFlags0_t : uint32_t {
                FLAG0_SHIFT_INTERPENETRATE = 0x0,
                FLAG0_SHIFT_CONSTRAIN = 0x1,
                FLAG0_SHIFT_BREAKABLE_FORCE = 0x2,
                FLAG0_SHIFT_BREAKABLE_TORQUE = 0x3
            };
            // Alignment: 4
            // Member count: 12
            enum class ParticleFloatMapType_t : uint32_t {
                PF_MAP_TYPE_INVALID = 0xFFFFFFFF,
                PF_MAP_TYPE_DIRECT = 0x0,
                PF_MAP_TYPE_MULT = 0x1,
                PF_MAP_TYPE_REMAP = 0x2,
                PF_MAP_TYPE_REMAP_BIASED = 0x3,
                PF_MAP_TYPE_CURVE = 0x4,
                PF_MAP_TYPE_NOTCHED = 0x5,
                PF_MAP_TYPE_ROUND = 0x6,
                PF_MAP_TYPE_MIN = 0x7,
                PF_MAP_TYPE_MAX = 0x8,
                PF_MAP_TYPE_MOD = 0x9,
                PF_MAP_TYPE_COUNT = 0xA
            };
            // Alignment: 4
            // Member count: 5
            enum class AnimParamVectorType_t : uint32_t {
                ANIMPARAM_VECTOR_TYPE_NONE = 0x0,
                ANIMPARAM_VECTOR_TYPE_POSITION_WS = 0x1,
                ANIMPARAM_VECTOR_TYPE_POSITION_LS = 0x2,
                ANIMPARAM_VECTOR_TYPE_DIRECTION_WS = 0x3,
                ANIMPARAM_VECTOR_TYPE_DIRECTION_LS = 0x4
            };
            // Alignment: 1
            // Member count: 3
            enum class CNmCurrentSyncEventNode__InfoType_t : uint8_t {
                IndexAndPercentage = 0x0,
                IndexOnly = 0x1,
                PercentageOnly = 0x2
            };
            // Alignment: 4
            // Member count: 4
            enum class BlendKeyType : uint32_t {
                BlendKey_UserValue = 0x0,
                BlendKey_Velocity = 0x1,
                BlendKey_Distance = 0x2,
                BlendKey_RemainingDistance = 0x3
            };
            // Alignment: 4
            // Member count: 5
            enum class StateActionBehavior : uint32_t {
                STATETAGBEHAVIOR_ACTIVE_WHILE_CURRENT = 0x0,
                STATETAGBEHAVIOR_FIRE_ON_ENTER = 0x1,
                STATETAGBEHAVIOR_FIRE_ON_EXIT = 0x2,
                STATETAGBEHAVIOR_FIRE_ON_ENTER_AND_EXIT = 0x3,
                STATETAGBEHAVIOR_ACTIVE_WHILE_FULLY_BLENDED = 0x4
            };
            // Alignment: 1
            // Member count: 4
            enum class NmRootMotionBlendMode_t : uint8_t {
                Blend = 0x0,
                Additive = 0x1,
                IgnoreSource = 0x2,
                IgnoreTarget = 0x3
            };
            // Alignment: 1
            // Member count: 7
            enum class NmFootPhaseCondition_t : uint8_t {
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
            enum class ModelSkeletonData_t__BoneFlags_t : uint32_t {
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
            // Alignment: 1
            // Member count: 2
            enum class CNmOrientationWarpNode__AlignmentMode_t : uint8_t {
                MovementDirection = 0x0,
                AnimationEndFacing = 0x1
            };
            // Alignment: 4
            // Member count: 3
            enum class GPUParticleCollisionMode_t : uint32_t {
                PARTICLE_GPU_COLLISION_MODE_RT = 0x0,
                PARTICLE_GPU_COLLISION_MODE_DEPTH = 0x1,
                PARTICLE_GPU_COLLISION_MODE_HYBRID = 0x2
            };
            // Alignment: 4
            // Member count: 4
            enum class MorphBundleType_t : uint32_t {
                MORPH_BUNDLE_TYPE_NONE = 0x0,
                MORPH_BUNDLE_TYPE_POSITION_SPEED = 0x1,
                MORPH_BUNDLE_TYPE_NORMAL_WRINKLE = 0x2,
                MORPH_BUNDLE_TYPE_COUNT = 0x3
            };
            // Alignment: 1
            // Member count: 2
            enum class CNmIDComparisonNode__Comparison_t : uint8_t {
                Matches = 0x0,
                DoesntMatch = 0x1
            };
            // Alignment: 1
            // Member count: 3
            enum class NmPoseBlendMode_t : uint8_t {
                Overlay = 0x0,
                Additive = 0x1,
                ModelSpace = 0x2
            };
            // Alignment: 4
            // Member count: 4
            enum class ParticleFloatInputMode_t : uint32_t {
                PF_INPUT_MODE_INVALID = 0xFFFFFFFF,
                PF_INPUT_MODE_CLAMPED = 0x0,
                PF_INPUT_MODE_LOOPED = 0x1,
                PF_INPUT_MODE_COUNT = 0x2
            };
            // Alignment: 4
            // Member count: 5
            enum class ResetCycleOption : uint32_t {
                Beginning = 0x0,
                SameCycleAsSource = 0x1,
                InverseSourceCycle = 0x2,
                FixedValue = 0x3,
                SameTimeAsSource = 0x4
            };
            // Alignment: 1
            // Member count: 6
            enum class CNmVectorInfoNode__Info_t : uint8_t {
                X = 0x0,
                Y = 0x1,
                Z = 0x2,
                Length = 0x3,
                AngleHorizontal = 0x4,
                AngleVertical = 0x5
            };
            // Alignment: 1
            // Member count: 3
            enum class TagActionStatus : uint8_t {
                Inactive = 0x0,
                Active = 0x1,
                Fired = 0x2
            };
            // Alignment: 4
            // Member count: 4
            enum class IKChannelMode : uint32_t {
                TwoBone = 0x0,
                TwoBone_Translate = 0x1,
                OneBone = 0x2,
                OneBone_Translate = 0x3
            };
            // Alignment: 1
            // Member count: 9
            enum class NmGraphValueType_t : uint8_t {
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
            enum class ParticleFloatRandomMode_t : uint32_t {
                PF_RANDOM_MODE_INVALID = 0xFFFFFFFF,
                PF_RANDOM_MODE_CONSTANT = 0x0,
                PF_RANDOM_MODE_VARYING = 0x1,
                PF_RANDOM_MODE_COUNT = 0x2
            };
            // Alignment: 4
            // Member count: 4
            enum class PFNoiseModifier_t : uint32_t {
                PF_NOISE_MODIFIER_NONE = 0x0,
                PF_NOISE_MODIFIER_LINES = 0x1,
                PF_NOISE_MODIFIER_CLUMPS = 0x2,
                PF_NOISE_MODIFIER_RINGS = 0x3
            };
            // Alignment: 4
            // Member count: 21
            enum class ParticleVecType_t : uint32_t {
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
            enum class NmFootPhase_t : uint8_t {
                LeftFootDown = 0x0,
                RightFootPassing = 0x1,
                RightFootDown = 0x2,
                LeftFootPassing = 0x3,
                None = 0x4
            };
            // Alignment: 4
            // Member count: 8
            enum class CNmTargetInfoNode__Info_t : uint32_t {
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
            enum class FootstepLandedFootSoundType_t : uint32_t {
                FOOTSOUND_Left = 0x0,
                FOOTSOUND_Right = 0x1,
                FOOTSOUND_UseOverrideSound = 0x2
            };
            // Alignment: 4
            // Member count: 2
            enum class FootLockSubVisualization : uint32_t {
                FOOTLOCKSUBVISUALIZATION_ReachabilityAnalysis = 0x0,
                FOOTLOCKSUBVISUALIZATION_IKSolve = 0x1
            };
            // Alignment: 4
            // Member count: 5
            enum class CNmSoundEvent__Position_t : uint32_t {
                None = 0x0,
                World = 0x1,
                EntityPos = 0x2,
                EntityEyePos = 0x3,
                EntityAttachment = 0x4
            };
            // Alignment: 1
            // Member count: 4
            enum class FootstepJumpPhase_t : uint8_t {
                Unknown = 0x0,
                NotJumping = 0x1,
                Jumping = 0x2,
                Landing = 0x4
            };
            // Alignment: 4
            // Member count: 2
            enum class NmFrameSnapEventMode_t : uint32_t {
                Floor = 0x0,
                Round = 0x1
            };
            // Alignment: 4
            // Member count: 3
            enum class FootPinningTimingSource : uint32_t {
                FootMotion = 0x0,
                Tag = 0x1,
                Parameter = 0x2
            };
            // Alignment: 4
            // Member count: 4
            enum class DampingSpeedFunction : uint32_t {
                NoDamping = 0x0,
                Constant = 0x1,
                Spring = 0x2,
                AsymmetricSpring = 0x3
            };
            // Alignment: 4
            // Member count: 6
            enum class AnimationProcessingType_t : uint32_t {
                ANIMATION_PROCESSING_SERVER_SIMULATION = 0x0,
                ANIMATION_PROCESSING_CLIENT_SIMULATION = 0x1,
                ANIMATION_PROCESSING_CLIENT_PREDICTION = 0x2,
                ANIMATION_PROCESSING_CLIENT_INTERPOLATION = 0x3,
                ANIMATION_PROCESSING_CLIENT_RENDER = 0x4,
                ANIMATION_PROCESSING_MAX = 0x5
            };
            // Alignment: 4
            // Member count: 3
            enum class JiggleBoneSimSpace : uint32_t {
                SimSpace_Local = 0x0,
                SimSpace_Model = 0x1,
                SimSpace_World = 0x2
            };
            // Alignment: 4
            // Member count: 2
            enum class StanceOverrideMode : uint32_t {
                Sequence = 0x0,
                Node = 0x1
            };
            // Alignment: 4
            // Member count: 2
            enum class IkEndEffectorType : uint32_t {
                IkEndEffector_Attachment = 0x0,
                IkEndEffector_Bone = 0x1
            };
            // Alignment: 2
            // Member count: 3
            enum class AnimScriptType : uint16_t {
                ANIMSCRIPT_TYPE_INVALID = 0xFFFF,
                ANIMSCRIPT_FUSE_GENERAL = 0x0,
                ANIMSCRIPT_FUSE_STATEMACHINE = 0x1
            };
            // Alignment: 1
            // Member count: 3
            enum class CNmTimeConditionNode__ComparisonType_t : uint8_t {
                PercentageThroughState = 0x0,
                PercentageThroughSyncEvent = 0x1,
                ElapsedTime = 0x2
            };
            // Alignment: 4
            // Member count: 4
            enum class SeqPoseSetting_t : uint32_t {
                SEQ_POSE_SETTING_CONSTANT = 0x0,
                SEQ_POSE_SETTING_ROTATION = 0x1,
                SEQ_POSE_SETTING_POSITION = 0x2,
                SEQ_POSE_SETTING_VELOCITY = 0x3
            };
            // Alignment: 4
            // Member count: 13
            enum class AnimParamButton_t : uint32_t {
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
            enum class SelectorTagBehavior_t : uint32_t {
                SelectorTagBehavior_OnWhileCurrent = 0x0,
                SelectorTagBehavior_OffWhenFinished = 0x1,
                SelectorTagBehavior_OffBeforeFinished = 0x2
            };
            // Alignment: 4
            // Member count: 4
            enum class HandshakeTagType_t : uint32_t {
                eInvalid = 0xFFFFFFFF,
                eTask = 0x0,
                eMovement = 0x1,
                eCount = 0x2
            };
            // Alignment: 4
            // Member count: 4
            enum class OrientationWarpTargetOffsetMode_t : uint32_t {
                eLiteralValue = 0x0,
                eParameter = 0x1,
                eAnimationMovementHeading = 0x2,
                eAnimationMovementHeadingAtEnd = 0x3
            };
            // Alignment: 4
            // Member count: 3
            enum class OrientationWarpMode_t : uint32_t {
                eInvalid = 0x0,
                eAngle = 0x1,
                eWorldPosition = 0x2
            };
            // Alignment: 4
            // Member count: 5
            enum class ParticleTransformType_t : uint32_t {
                PT_TYPE_INVALID = 0x0,
                PT_TYPE_NAMED_VALUE = 0x1,
                PT_TYPE_CONTROL_POINT = 0x2,
                PT_TYPE_CONTROL_POINT_RANGE = 0x3,
                PT_TYPE_COUNT = 0x4
            };
            // Alignment: 4
            // Member count: 18
            enum class ParticleAttachment_t : uint32_t {
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
            enum class CNmEventTargetEntity_t : uint32_t {
                Self = 0x0,
                Weapon = 0x1,
                HeldItem = 0x2,
                Custom = 0x3
            };
            // Alignment: 4
            // Member count: 3
            enum class FieldNetworkOption : uint32_t {
                Auto = 0x0,
                ForceEnable = 0x1,
                ForceDisable = 0x2
            };
            // Alignment: 1
            // Member count: 6
            enum class NmGraphEventTypeCondition_t : uint8_t {
                Entry = 0x0,
                FullyInState = 0x1,
                Exit = 0x2,
                Timed = 0x3,
                Generic = 0x4,
                Any = 0x5
            };
            // Alignment: 1
            // Member count: 10
            enum class CNmTransitionNode__TransitionOptions_t : uint8_t {
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
            enum class CNmFloatComparisonNode__Comparison_t : uint8_t {
                GreaterThanEqual = 0x0,
                LessThanEqual = 0x1,
                NearEqual = 0x2,
                GreaterThan = 0x3,
                LessThan = 0x4
            };
            // Alignment: 4
            // Member count: 3
            enum class VPhysXJoint_t__Flags_t : uint32_t {
                JOINT_FLAGS_NONE = 0x0,
                JOINT_FLAGS_BODY1_FIXED = 0x1,
                JOINT_FLAGS_USE_BLOCK_SOLVER = 0x2
            };
            // Alignment: 4
            // Member count: 4
            enum class ScriptedHeldWeaponBehavior_t : uint32_t {
                eInvalid = 0xFFFFFFFF,
                eHolster = 0x0,
                eDeploy = 0x1,
                eDrop = 0x2
            };
            // Alignment: 1
            // Member count: 3
            enum class VelocityMetricMode : uint8_t {
                DirectionOnly = 0x0,
                MagnitudeOnly = 0x1,
                DirectionAndMagnitude = 0x2
            };
            // Alignment: 1
            // Member count: 5
            enum class FacingMode : uint8_t {
                FacingMode_Invalid = 0x0,
                FacingMode_Manual = 0x1,
                FacingMode_Path = 0x2,
                FacingMode_LookTarget = 0x3,
                FacingMode_ManualPosition = 0x4
            };
            // Alignment: 1
            // Member count: 3
            enum class VertexAlbedoFormat_t : uint8_t {
                VERTEX_ALBEDO_NONE = 0x0,
                VERTEX_ALBEDO_8888 = 0x1,
                VERTEX_ALBEDO_565 = 0x2
            };
            // Alignment: 4
            // Member count: 4
            enum class AimMatrixBlendMode : uint32_t {
                AimMatrixBlendMode_None = 0x0,
                AimMatrixBlendMode_Additive = 0x1,
                AimMatrixBlendMode_ModelSpaceAdditive = 0x2,
                AimMatrixBlendMode_BoneMask = 0x3
            };
            // Alignment: 4
            // Member count: 7
            enum class AnimationSnapshotType_t : uint32_t {
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
            enum class NmTargetWarpRule_t : uint8_t {
                WarpXY = 0x0,
                WarpZ = 0x1,
                WarpXYZ = 0x2,
                RotationOnly = 0x3,
                FixedSection = 0x4
            };
            // Alignment: 4
            // Member count: 8
            enum class FootFallTagFoot_t : uint32_t {
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
            enum class ChoiceMethod : uint32_t {
                WeightedRandom = 0x0,
                WeightedRandomNoRepeat = 0x1,
                Iterate = 0x2,
                IterateRandom = 0x3
            };
            // Alignment: 4
            // Member count: 14
            enum class AnimVectorSource : uint32_t {
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
            enum class IkTargetType : uint32_t {
                IkTarget_Attachment = 0x0,
                IkTarget_Bone = 0x1,
                IkTarget_Parameter_ModelSpace = 0x2,
                IkTarget_Parameter_WorldSpace = 0x3
            };
            // Alignment: 1
            // Member count: 3
            enum class RenderMeshSlotType_t : uint8_t {
                RENDERMESH_SLOT_INVALID = 0xFF,
                RENDERMESH_SLOT_PER_VERTEX = 0x0,
                RENDERMESH_SLOT_PER_INSTANCE = 0x1
            };
            // Alignment: 4
            // Member count: 4
            enum class BoneMaskBlendSpace : uint32_t {
                BlendSpace_Parent = 0x0,
                BlendSpace_Model = 0x1,
                BlendSpace_Model_RotationOnly = 0x2,
                BlendSpace_Model_TranslationOnly = 0x3
            };
            // Alignment: 4
            // Member count: 11
            enum class MovementCapability_t : uint32_t {
                eStrafe = 0x0,
                eIdleTurn = 0x1,
                eStart = 0x2,
                eStop = 0x3,
                eInstantStop = 0x4,
                eShuffle = 0x5,
                ePlantedTurn = 0x6,
                eUseStartAsPlantedTurn = 0x7,
                eLean = 0x8,
                eForwardStartOnly = 0x9,
                eCount = 0xA
            };
            // Alignment: 4
            // Member count: 5
            enum class ModelConfigAttachmentType_t : uint32_t {
                MODEL_CONFIG_ATTACHMENT_INVALID = 0xFFFFFFFF,
                MODEL_CONFIG_ATTACHMENT_BONE_OR_ATTACHMENT = 0x0,
                MODEL_CONFIG_ATTACHMENT_ROOT_RELATIVE = 0x1,
                MODEL_CONFIG_ATTACHMENT_BONEMERGE = 0x2,
                MODEL_CONFIG_ATTACHMENT_COUNT = 0x3
            };
            // Alignment: 4
            // Member count: 2
            enum class BinaryNodeChildOption : uint32_t {
                Child1 = 0x0,
                Child2 = 0x1
            };
            // Alignment: 4
            // Member count: 7
            enum class NPCPhysicsHullType_t : uint32_t {
                eInvalid = 0x0,
                eGroundCapsule = 0x1,
                eCenteredCapsule = 0x2,
                eGenericCapsule = 0x3,
                eGroundBox = 0x4,
                eGroundCylinder = 0x5,
                eCenteredCylinder = 0x6
            };
            // Alignment: 4
            // Member count: 2
            enum class JumpCorrectionMethod : uint32_t {
                ScaleMotion = 0x0,
                AddCorrectionDelta = 0x1
            };
            // Alignment: 4
            // Member count: 2
            enum class MoodType_t : uint32_t {
                eMoodType_Head = 0x0,
                eMoodType_Body = 0x1
            };
            // Parent: None
            // Field count: 0
            namespace CPulse_ResumePoint {
            }
            // Parent: None
            // Field count: 0
            namespace CParticleBindingRealPulse {
            }
            // Parent: None
            // Field count: 4
            namespace CPulse_OutflowConnection {
                constexpr std::ptrdiff_t m_SourceOutflowName = 0x0; // PulseSymbol_t
                constexpr std::ptrdiff_t m_nDestChunk = 0x10; // PulseRuntimeChunkIndex_t
                constexpr std::ptrdiff_t m_nInstruction = 0x14; // int32
                constexpr std::ptrdiff_t m_OutflowRegisterMap = 0x18; // PulseRegisterMap_t
            }
            // Parent: None
            // Field count: 0
            namespace CBasePulseGraphInstance {
            }
            // Parent: None
            // Field count: 0
            namespace SignatureOutflow_Resume {
            }
            // Parent: None
            // Field count: 0
            namespace SignatureOutflow_Continue {
            }
            // Parent: None
            // Field count: 0
            namespace CParticleCollectionBindingInstance {
            }
            // Parent: None
            // Field count: 1
            namespace CPulseCell_IsRequirementValid__Criteria_t {
                constexpr std::ptrdiff_t m_bIsValid = 0x0; // bool
            }
            // Parent: None
            // Field count: 1
            namespace CPulseCell_Unknown {
                constexpr std::ptrdiff_t m_UnknownKeys = 0x48; // KeyValues3
            }
            // Parent: None
            // Field count: 1
            namespace CPulseCell_LimitCount__Criteria_t {
                constexpr std::ptrdiff_t m_bLimitCountPasses = 0x0; // bool
            }
            // Parent: None
            // Field count: 0
            namespace CPulseExecCursor {
            }
            // Parent: None
            // Field count: 12
            namespace CNmFootIKTask {
                constexpr std::ptrdiff_t m_nLeftEffectorBoneIdx = 0x70; // int32
                constexpr std::ptrdiff_t m_nRightEffectorBoneIdx = 0x74; // int32
                constexpr std::ptrdiff_t m_leftTargetTransform = 0x80; // CTransform
                constexpr std::ptrdiff_t m_rightTargetTransform = 0xA0; // CTransform
                constexpr std::ptrdiff_t m_nLeftTargetBoneIdx = 0xC0; // int32
                constexpr std::ptrdiff_t m_nRightTargetBoneIdx = 0xC4; // int32
                constexpr std::ptrdiff_t m_leftTarget = 0xD0; // CNmTarget
                constexpr std::ptrdiff_t m_rightTarget = 0x100; // CNmTarget
                constexpr std::ptrdiff_t m_blendMode = 0x130; // NmIKBlendMode_t
                constexpr std::ptrdiff_t m_flBlendWeight = 0x134; // float32
                constexpr std::ptrdiff_t m_bIsTargetInWorldSpace = 0x138; // bool
                constexpr std::ptrdiff_t m_bIsRunningFromDeserializedData = 0x139; // bool
            }
            // Parent: None
            // Field count: 0
            namespace CNmScaleTask {
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CAnimNodePath {
                constexpr std::ptrdiff_t m_path = 0x0; // AnimNodeID[11]
                constexpr std::ptrdiff_t m_nCount = 0x2C; // int32
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace AnimNodeOutputID {
                constexpr std::ptrdiff_t m_id = 0x0; // uint32
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
            namespace CSeqBoneMaskList {
                constexpr std::ptrdiff_t m_sName = 0x0; // CBufferString
                constexpr std::ptrdiff_t m_nLocalBoneArray = 0x10; // CUtlVector<int16>
                constexpr std::ptrdiff_t m_flBoneWeightArray = 0x28; // CUtlVector<float32>
                constexpr std::ptrdiff_t m_flDefaultMorphCtrlWeight = 0x40; // float32
                constexpr std::ptrdiff_t m_morphCtrlWeightArray = 0x48; // CUtlVector<std::pair<CBufferString,float32>>
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace SampleCode {
                constexpr std::ptrdiff_t m_subCode = 0x0; // uint8[8]
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace AnimationDecodeDebugDumpElement_t {
                constexpr std::ptrdiff_t m_nEntityIndex = 0x0; // int32
                constexpr std::ptrdiff_t m_modelName = 0x8; // CUtlString
                constexpr std::ptrdiff_t m_poseParams = 0x10; // CUtlVector<CUtlString>
                constexpr std::ptrdiff_t m_decodeOps = 0x28; // CUtlVector<CUtlString>
                constexpr std::ptrdiff_t m_internalOps = 0x40; // CUtlVector<CUtlString>
                constexpr std::ptrdiff_t m_decodedAnims = 0x58; // CUtlVector<CUtlString>
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace ConfigIndex {
                constexpr std::ptrdiff_t m_nGroup = 0x0; // uint16
                constexpr std::ptrdiff_t m_nConfig = 0x2; // uint16
            }
            // Parent: None
            // Field count: 11
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CSeqSeqDescFlag {
                constexpr std::ptrdiff_t m_bLooping = 0x0; // bool
                constexpr std::ptrdiff_t m_bSnap = 0x1; // bool
                constexpr std::ptrdiff_t m_bAutoplay = 0x2; // bool
                constexpr std::ptrdiff_t m_bPost = 0x3; // bool
                constexpr std::ptrdiff_t m_bHidden = 0x4; // bool
                constexpr std::ptrdiff_t m_bMulti = 0x5; // bool
                constexpr std::ptrdiff_t m_bLegacyDelta = 0x6; // bool
                constexpr std::ptrdiff_t m_bLegacyWorldspace = 0x7; // bool
                constexpr std::ptrdiff_t m_bLegacyCyclepose = 0x8; // bool
                constexpr std::ptrdiff_t m_bLegacyRealtime = 0x9; // bool
                constexpr std::ptrdiff_t m_bModelDoc = 0xA; // bool
            }
            // Parent: None
            // Field count: 1
            namespace IKBoneNameAndIndex_t {
                constexpr std::ptrdiff_t m_Name = 0x0; // CUtlString
            }
            // Parent: None
            // Field count: 0
            namespace CNmPoseNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            namespace CNmBlendTask {
            }
            // Parent: None
            // Field count: 0
            namespace CNmVectorValueNode__CDefinition {
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace CSeqTransition {
                constexpr std::ptrdiff_t m_flFadeInTime = 0x0; // float32
                constexpr std::ptrdiff_t m_flFadeOutTime = 0x4; // float32
            }
            // Parent: None
            // Field count: 0
            namespace CNmPoseTask {
            }
            // Parent: None
            // Field count: 0
            namespace CNmCachedPoseReadTask {
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CAnimParamHandle {
                constexpr std::ptrdiff_t m_type = 0x0; // AnimParamType_t
                constexpr std::ptrdiff_t m_index = 0x1; // uint8
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace WeightList {
                constexpr std::ptrdiff_t m_name = 0x0; // CUtlString
                constexpr std::ptrdiff_t m_weights = 0x8; // CUtlVector<float32>
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CSeqAutoLayerFlag {
                constexpr std::ptrdiff_t m_bPost = 0x0; // bool
                constexpr std::ptrdiff_t m_bSpline = 0x1; // bool
                constexpr std::ptrdiff_t m_bXFade = 0x2; // bool
                constexpr std::ptrdiff_t m_bNoBlend = 0x3; // bool
                constexpr std::ptrdiff_t m_bLocal = 0x4; // bool
                constexpr std::ptrdiff_t m_bPose = 0x5; // bool
                constexpr std::ptrdiff_t m_bFetchFrame = 0x6; // bool
                constexpr std::ptrdiff_t m_bSubtract = 0x7; // bool
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace AnimStateID {
                constexpr std::ptrdiff_t m_id = 0x0; // uint32
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
            namespace CSeqPoseSetting {
                constexpr std::ptrdiff_t m_sPoseParameter = 0x0; // CBufferString
                constexpr std::ptrdiff_t m_sAttachment = 0x10; // CBufferString
                constexpr std::ptrdiff_t m_sReferenceSequence = 0x20; // CBufferString
                constexpr std::ptrdiff_t m_flValue = 0x30; // float32
                constexpr std::ptrdiff_t m_bX = 0x34; // bool
                constexpr std::ptrdiff_t m_bY = 0x35; // bool
                constexpr std::ptrdiff_t m_bZ = 0x36; // bool
                constexpr std::ptrdiff_t m_eType = 0x38; // int32
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CDrawCullingData {
                constexpr std::ptrdiff_t m_ConeAxis = 0x0; // int8[3]
                constexpr std::ptrdiff_t m_ConeCutoff = 0x3; // int8
            }
            // Parent: None
            // Field count: 6
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CSeqMultiFetchFlag {
                constexpr std::ptrdiff_t m_bRealtime = 0x0; // bool
                constexpr std::ptrdiff_t m_bCylepose = 0x1; // bool
                constexpr std::ptrdiff_t m_b0D = 0x2; // bool
                constexpr std::ptrdiff_t m_b1D = 0x3; // bool
                constexpr std::ptrdiff_t m_b2D = 0x4; // bool
                constexpr std::ptrdiff_t m_b2D_TRI = 0x5; // bool
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CSolveIKTargetHandle_t {
                constexpr std::ptrdiff_t m_positionHandle = 0x0; // CAnimParamHandle
                constexpr std::ptrdiff_t m_orientationHandle = 0x2; // CAnimParamHandle
            }
            // Parent: None
            // Field count: 0
            namespace CNmZeroPoseTask {
            }
            // Parent: None
            // Field count: 0
            namespace CNmBlendTaskBase {
            }
            // Parent: None
            // Field count: 33
            namespace CNewParticleEffect {
                constexpr std::ptrdiff_t m_pNext = 0x10; // CNewParticleEffect*
                constexpr std::ptrdiff_t m_pPrev = 0x18; // CNewParticleEffect*
                constexpr std::ptrdiff_t m_pParticles = 0x20; // IParticleCollection*
                constexpr std::ptrdiff_t m_pDebugName = 0x28; // char*
                constexpr std::ptrdiff_t m_bDontRemove = 0x0; // bitfield:1
                constexpr std::ptrdiff_t m_bRemove = 0x0; // bitfield:1
                constexpr std::ptrdiff_t m_bNeedsBBoxUpdate = 0x0; // bitfield:1
                constexpr std::ptrdiff_t m_bIsFirstFrame = 0x0; // bitfield:1
                constexpr std::ptrdiff_t m_bAutoUpdateBBox = 0x0; // bitfield:1
                constexpr std::ptrdiff_t m_bAllocated = 0x0; // bitfield:1
                constexpr std::ptrdiff_t m_bSimulate = 0x0; // bitfield:1
                constexpr std::ptrdiff_t m_bShouldPerformCullCheck = 0x0; // bitfield:1
                constexpr std::ptrdiff_t m_bForceNoDraw = 0x0; // bitfield:1
                constexpr std::ptrdiff_t m_bSuppressScreenSpaceEffect = 0x0; // bitfield:1
                constexpr std::ptrdiff_t m_bShouldSave = 0x0; // bitfield:1
                constexpr std::ptrdiff_t m_bShouldSimulateDuringGamePaused = 0x0; // bitfield:1
                constexpr std::ptrdiff_t m_bShouldCheckFoW = 0x0; // bitfield:1
                constexpr std::ptrdiff_t m_bIsAsyncCreate = 0x0; // bitfield:1
                constexpr std::ptrdiff_t m_bFreezeTransitionActive = 0x0; // bitfield:1
                constexpr std::ptrdiff_t m_bFreezeTargetState = 0x0; // bitfield:1
                constexpr std::ptrdiff_t m_bCanFreeze = 0x0; // bitfield:1
                constexpr std::ptrdiff_t m_vSortOrigin = 0x40; // Vector
                constexpr std::ptrdiff_t m_flScale = 0x4C; // float32
                constexpr std::ptrdiff_t m_hOwner = 0x50; // PARTICLE_EHANDLE__*
                constexpr std::ptrdiff_t m_pOwningParticleProperty = 0x58; // CParticleProperty*
                constexpr std::ptrdiff_t m_flFreezeTransitionStart = 0x70; // float32
                constexpr std::ptrdiff_t m_flFreezeTransitionDuration = 0x74; // float32
                constexpr std::ptrdiff_t m_flFreezeTransitionOverride = 0x78; // float32
                constexpr std::ptrdiff_t m_LastMin = 0x7C; // Vector
                constexpr std::ptrdiff_t m_LastMax = 0x88; // Vector
                constexpr std::ptrdiff_t m_nSplitScreenUser = 0x94; // CSplitScreenSlot
                constexpr std::ptrdiff_t m_vecAggregationCenter = 0x98; // Vector
                constexpr std::ptrdiff_t m_RefCount = 0xD0; // int32
            }
            // Parent: None
            // Field count: 0
            namespace CNmReferencePoseTask {
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
            namespace CSeqSynthAnimDesc {
                constexpr std::ptrdiff_t m_sName = 0x0; // CBufferString
                constexpr std::ptrdiff_t m_flags = 0x10; // CSeqSeqDescFlag
                constexpr std::ptrdiff_t m_transition = 0x1C; // CSeqTransition
                constexpr std::ptrdiff_t m_nLocalBaseReference = 0x24; // int16
                constexpr std::ptrdiff_t m_nLocalBoneMask = 0x26; // int16
                constexpr std::ptrdiff_t m_activityArray = 0x28; // CUtlVector<CAnimActivity>
            }
            // Parent: None
            // Field count: 0
            namespace IAnimationGraphInstance {
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CParamSpanUpdater {
                constexpr std::ptrdiff_t m_spans = 0x0; // CUtlVector<ParamSpan_t>
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CNmContactAudioTypeVData {
            }
            // Parent: None
            // Field count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace AnimationSnapshotBase_t {
                constexpr std::ptrdiff_t m_flRealTime = 0x0; // float32
                constexpr std::ptrdiff_t m_rootToWorld = 0x10; // matrix3x4a_t
                constexpr std::ptrdiff_t m_bBonesInWorldSpace = 0x40; // bool
                constexpr std::ptrdiff_t m_boneSetupMask = 0x48; // CUtlVector<uint32>
                constexpr std::ptrdiff_t m_boneTransforms = 0x60; // CUtlVector<matrix3x4a_t>
                constexpr std::ptrdiff_t m_flexControllers = 0x78; // CUtlVector<float32>
                constexpr std::ptrdiff_t m_SnapshotType = 0x90; // AnimationSnapshotType_t
                constexpr std::ptrdiff_t m_bHasDecodeDump = 0x94; // bool
                constexpr std::ptrdiff_t m_DecodeDump = 0x98; // AnimationDecodeDebugDumpElement_t
            }
            // Parent: None
            // Field count: 0
            namespace CNmBoneMaskValueNode__CDefinition {
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
            namespace CSeqPoseParamDesc {
                constexpr std::ptrdiff_t m_sName = 0x0; // CBufferString
                constexpr std::ptrdiff_t m_flStart = 0x10; // float32
                constexpr std::ptrdiff_t m_flEnd = 0x14; // float32
                constexpr std::ptrdiff_t m_flLoop = 0x18; // float32
                constexpr std::ptrdiff_t m_bLooping = 0x1C; // bool
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MKV3TransferName
            // MGetKV3ClassDefaults
            namespace CAnimMovement {
                constexpr std::ptrdiff_t endframe = 0x0; // int32
                constexpr std::ptrdiff_t motionflags = 0x4; // int32
                constexpr std::ptrdiff_t v0 = 0x8; // float32
                constexpr std::ptrdiff_t v1 = 0xC; // float32
                constexpr std::ptrdiff_t angle = 0x10; // float32
                constexpr std::ptrdiff_t vector = 0x14; // Vector
                constexpr std::ptrdiff_t position = 0x20; // Vector
            }
            // Parent: None
            // Field count: 0
            namespace CNmIDValueNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            namespace CNmChainLookatTask {
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
            namespace CAnimLocalHierarchy {
                constexpr std::ptrdiff_t m_sBone = 0x0; // CBufferString
                constexpr std::ptrdiff_t m_sNewParent = 0x10; // CBufferString
                constexpr std::ptrdiff_t m_nStartFrame = 0x20; // int32
                constexpr std::ptrdiff_t m_nPeakFrame = 0x24; // int32
                constexpr std::ptrdiff_t m_nTailFrame = 0x28; // int32
                constexpr std::ptrdiff_t m_nEndFrame = 0x2C; // int32
            }
            // Parent: None
            // Field count: 0
            namespace CNmSampleTask {
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
            namespace CSequenceGroupData {
                constexpr std::ptrdiff_t m_sName = 0x10; // CBufferString
                constexpr std::ptrdiff_t m_nFlags = 0x20; // uint32
                constexpr std::ptrdiff_t m_localSequenceNameArray = 0x28; // CUtlVector<CBufferString>
                constexpr std::ptrdiff_t m_localS1SeqDescArray = 0x40; // CUtlVector<CSeqS1SeqDesc>
                constexpr std::ptrdiff_t m_localMultiSeqDescArray = 0x58; // CUtlVector<CSeqS1SeqDesc>
                constexpr std::ptrdiff_t m_localSynthAnimDescArray = 0x70; // CUtlVector<CSeqSynthAnimDesc>
                constexpr std::ptrdiff_t m_localCmdSeqDescArray = 0x88; // CUtlVector<CSeqCmdSeqDesc>
                constexpr std::ptrdiff_t m_localBoneMaskArray = 0xA0; // CUtlVector<CSeqBoneMaskList>
                constexpr std::ptrdiff_t m_localScaleSetArray = 0xB8; // CUtlVector<CSeqScaleSet>
                constexpr std::ptrdiff_t m_localBoneNameArray = 0xD0; // CUtlVector<CBufferString>
                constexpr std::ptrdiff_t m_localNodeName = 0xE8; // CBufferString
                constexpr std::ptrdiff_t m_localPoseParamArray = 0xF8; // CUtlVector<CSeqPoseParamDesc>
                constexpr std::ptrdiff_t m_keyValues = 0x110; // KeyValues3
                constexpr std::ptrdiff_t m_localIKAutoplayLockArray = 0x120; // CUtlVector<CSeqIKLock>
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
            namespace CAnimEventDefinition {
                constexpr std::ptrdiff_t m_nFrame = 0x8; // int32
                constexpr std::ptrdiff_t m_nEndFrame = 0xC; // int32
                constexpr std::ptrdiff_t m_flCycle = 0x10; // float32
                constexpr std::ptrdiff_t m_flDuration = 0x14; // float32
                constexpr std::ptrdiff_t m_EventData = 0x18; // KeyValues3
                constexpr std::ptrdiff_t m_sLegacyOptions = 0x28; // CBufferString
                constexpr std::ptrdiff_t m_sEventName = 0x38; // CGlobalSymbol
            }
            // Parent: None
            // Field count: 2
            namespace CBoneConstraintPoseSpaceMorph__Input_t {
                constexpr std::ptrdiff_t m_inputValue = 0x0; // Vector
                constexpr std::ptrdiff_t m_outputWeightList = 0x10; // CUtlVector<float32>
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
            namespace MoodAnimationLayer_t {
                constexpr std::ptrdiff_t m_sName = 0x0; // CUtlString
                constexpr std::ptrdiff_t m_bActiveListening = 0x8; // bool
                constexpr std::ptrdiff_t m_bActiveTalking = 0x9; // bool
                constexpr std::ptrdiff_t m_layerAnimations = 0x10; // CUtlVector<MoodAnimation_t>
                constexpr std::ptrdiff_t m_flIntensity = 0x28; // CRangeFloat
                constexpr std::ptrdiff_t m_flDurationScale = 0x30; // CRangeFloat
                constexpr std::ptrdiff_t m_bScaleWithInts = 0x38; // bool
                constexpr std::ptrdiff_t m_flNextStart = 0x3C; // CRangeFloat
                constexpr std::ptrdiff_t m_flStartOffset = 0x44; // CRangeFloat
                constexpr std::ptrdiff_t m_flEndOffset = 0x4C; // CRangeFloat
                constexpr std::ptrdiff_t m_flFadeIn = 0x54; // float32
                constexpr std::ptrdiff_t m_flFadeOut = 0x58; // float32
            }
            // Parent: None
            // Field count: 0
            namespace CNmBoolValueNode__CDefinition {
            }
            // Parent: None
            // Field count: 0
            namespace IParticleEffect {
            }
            // Parent: None
            // Field count: 1
            namespace PARTICLE_EHANDLE__ {
                constexpr std::ptrdiff_t unused = 0x0; // int32
            }
            // Parent: None
            // Field count: 1
            namespace CAnimEnum {
                constexpr std::ptrdiff_t m_value = 0x0; // uint8
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
            namespace CAnimDecoder {
                constexpr std::ptrdiff_t m_szName = 0x0; // CBufferString
                constexpr std::ptrdiff_t m_nVersion = 0x10; // int32
                constexpr std::ptrdiff_t m_nType = 0x14; // int32
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
            namespace CAnimFrameSegment {
                constexpr std::ptrdiff_t m_nUniqueFrameIndex = 0x0; // int32
                constexpr std::ptrdiff_t m_nLocalElementMasks = 0x4; // uint32
                constexpr std::ptrdiff_t m_nLocalChannel = 0x8; // int32
                constexpr std::ptrdiff_t m_container = 0x10; // CUtlBinaryBlock
            }
            // Parent: None
            // Field count: 8
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CAnimDesc_Flag {
                constexpr std::ptrdiff_t m_bLooping = 0x0; // bool
                constexpr std::ptrdiff_t m_bAllZeros = 0x1; // bool
                constexpr std::ptrdiff_t m_bHidden = 0x2; // bool
                constexpr std::ptrdiff_t m_bDelta = 0x3; // bool
                constexpr std::ptrdiff_t m_bLegacyWorldspace = 0x4; // bool
                constexpr std::ptrdiff_t m_bModelDoc = 0x5; // bool
                constexpr std::ptrdiff_t m_bImplicitSeqIgnoreDelta = 0x6; // bool
                constexpr std::ptrdiff_t m_bAnimGraphAdditive = 0x7; // bool
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CTransitionUpdateData {
                constexpr std::ptrdiff_t m_srcStateIndex = 0x0; // uint8
                constexpr std::ptrdiff_t m_destStateIndex = 0x1; // uint8
                constexpr std::ptrdiff_t m_nHandshakeMaskToDisableFirst = 0x0; // bitfield:7
                constexpr std::ptrdiff_t m_bDisabled = 0x0; // bitfield:1
            }
            // Parent: None
            // Field count: 2
            namespace CBoneConstraintPoseSpaceBone__Input_t {
                constexpr std::ptrdiff_t m_inputValue = 0x0; // Vector
                constexpr std::ptrdiff_t m_outputTransformList = 0x10; // CUtlVector<CTransform>
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
            namespace CSeqMultiFetch {
                constexpr std::ptrdiff_t m_flags = 0x0; // CSeqMultiFetchFlag
                constexpr std::ptrdiff_t m_localReferenceArray = 0x8; // CUtlVector<int16>
                constexpr std::ptrdiff_t m_nGroupSize = 0x20; // int32[2]
                constexpr std::ptrdiff_t m_nLocalPose = 0x28; // int32[2]
                constexpr std::ptrdiff_t m_poseKeyArray0 = 0x30; // CUtlVector<float32>
                constexpr std::ptrdiff_t m_poseKeyArray1 = 0x48; // CUtlVector<float32>
                constexpr std::ptrdiff_t m_nLocalCyclePoseParameter = 0x60; // int32
                constexpr std::ptrdiff_t m_bCalculatePoseParameters = 0x64; // bool
                constexpr std::ptrdiff_t m_bFixedBlendWeight = 0x65; // bool
                constexpr std::ptrdiff_t m_flFixedBlendWeightVals = 0x68; // float32[2]
            }
            // Parent: None
            // Field count: 0
            namespace CNmOverlayBlendTask {
            }
            // Parent: None
            // Field count: 0
            namespace CNmValueNode__CDefinition {
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
            namespace CSeqS1SeqDesc {
                constexpr std::ptrdiff_t m_sName = 0x0; // CBufferString
                constexpr std::ptrdiff_t m_flags = 0x10; // CSeqSeqDescFlag
                constexpr std::ptrdiff_t m_fetch = 0x20; // CSeqMultiFetch
                constexpr std::ptrdiff_t m_nLocalWeightlist = 0x90; // int32
                constexpr std::ptrdiff_t m_autoLayerArray = 0x98; // CUtlVector<CSeqAutoLayer>
                constexpr std::ptrdiff_t m_IKLockArray = 0xB0; // CUtlVector<CSeqIKLock>
                constexpr std::ptrdiff_t m_transition = 0xC8; // CSeqTransition
                constexpr std::ptrdiff_t m_SequenceKeys = 0xD0; // KeyValues3
                constexpr std::ptrdiff_t m_LegacyKeyValueText = 0xE0; // CBufferString
                constexpr std::ptrdiff_t m_activityArray = 0xF0; // CUtlVector<CAnimActivity>
                constexpr std::ptrdiff_t m_footMotion = 0x108; // CUtlVector<CFootMotion>
            }
            // Parent: None
            // Field count: 0
            namespace CNmGraphInstance {
            }
            // Parent: None
            // Field count: 9
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MKV3TransferName
            namespace CSeqCmdLayer {
                constexpr std::ptrdiff_t m_cmd = 0x0; // int16
                constexpr std::ptrdiff_t m_nLocalReference = 0x2; // int16
                constexpr std::ptrdiff_t m_nLocalBonemask = 0x4; // int16
                constexpr std::ptrdiff_t m_nDstResult = 0x6; // int16
                constexpr std::ptrdiff_t m_nSrcResult = 0x8; // int16
                constexpr std::ptrdiff_t m_bSpline = 0xA; // bool
                constexpr std::ptrdiff_t m_flVar1 = 0xC; // float32
                constexpr std::ptrdiff_t m_flVar2 = 0x10; // float32
                constexpr std::ptrdiff_t m_nLineNumber = 0x14; // int16
            }
            // Parent: None
            // Field count: 0
            namespace CNmFloatValueNode__CDefinition {
            }
            // Parent: None
            // Field count: 3
            namespace CNmTwoBoneIKTask {
                constexpr std::ptrdiff_t m_nEffectorBoneIdx = 0x70; // int32
                constexpr std::ptrdiff_t m_nEffectorTargetBoneIdx = 0x74; // int32
                constexpr std::ptrdiff_t m_targetTransform = 0x80; // CTransform
            }
            // Parent: None
            // Field count: 0
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CNmContactAudioActionVData {
            }
            // Parent: None
            // Field count: 4
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace CSeqIKLock {
                constexpr std::ptrdiff_t m_flPosWeight = 0x0; // float32
                constexpr std::ptrdiff_t m_flAngleWeight = 0x4; // float32
                constexpr std::ptrdiff_t m_nLocalBone = 0x8; // int16
                constexpr std::ptrdiff_t m_bBonesOrientedAlongPositiveX = 0xA; // bool
            }
            // Parent: None
            // Field count: 0
            namespace CNmFollowBoneTask {
            }
            // Parent: None
            // Field count: 0
            namespace CNmTargetValueNode__CDefinition {
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CPoseHandle {
                constexpr std::ptrdiff_t m_nIndex = 0x0; // uint16
                constexpr std::ptrdiff_t m_eType = 0x2; // PoseType_t
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
            namespace CSeqCmdSeqDesc {
                constexpr std::ptrdiff_t m_sName = 0x0; // CBufferString
                constexpr std::ptrdiff_t m_flags = 0x10; // CSeqSeqDescFlag
                constexpr std::ptrdiff_t m_transition = 0x1C; // CSeqTransition
                constexpr std::ptrdiff_t m_nFrameRangeSequence = 0x24; // int16
                constexpr std::ptrdiff_t m_nFrameCount = 0x26; // int16
                constexpr std::ptrdiff_t m_flFPS = 0x28; // float32
                constexpr std::ptrdiff_t m_nSubCycles = 0x2C; // int16
                constexpr std::ptrdiff_t m_numLocalResults = 0x2E; // int16
                constexpr std::ptrdiff_t m_cmdLayerArray = 0x30; // CUtlVector<CSeqCmdLayer>
                constexpr std::ptrdiff_t m_eventArray = 0x48; // CUtlVector<CAnimEventDefinition>
                constexpr std::ptrdiff_t m_activityArray = 0x60; // CUtlVector<CAnimActivity>
                constexpr std::ptrdiff_t m_poseSettingArray = 0x78; // CUtlVector<CSeqPoseSetting>
            }
            // Parent: None
            // Field count: 0
            namespace CNmCachedPoseWriteTask {
            }
            // Parent: None
            // Field count: 0
            namespace CNmModelSpaceBlendTask {
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace AnimParamID {
                constexpr std::ptrdiff_t m_id = 0x0; // uint32
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace AnimationDecodeDebugDump_t {
                constexpr std::ptrdiff_t m_processingType = 0x0; // AnimationProcessingType_t
                constexpr std::ptrdiff_t m_elems = 0x8; // CUtlVector<AnimationDecodeDebugDumpElement_t>
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
            namespace CSeqScaleSet {
                constexpr std::ptrdiff_t m_sName = 0x0; // CBufferString
                constexpr std::ptrdiff_t m_bRootOffset = 0x10; // bool
                constexpr std::ptrdiff_t m_vRootOffset = 0x14; // Vector
                constexpr std::ptrdiff_t m_nLocalBoneArray = 0x20; // CUtlVector<int16>
                constexpr std::ptrdiff_t m_flBoneScaleArray = 0x38; // CUtlVector<float32>
            }
            // Parent: None
            // Field count: 5
            namespace IKTargetSettings_t {
                constexpr std::ptrdiff_t m_TargetSource = 0x0; // IKTargetSource
                constexpr std::ptrdiff_t m_Bone = 0x8; // IKBoneNameAndIndex_t
                constexpr std::ptrdiff_t m_AnimgraphParameterNamePosition = 0x18; // AnimParamID
                constexpr std::ptrdiff_t m_AnimgraphParameterNameOrientation = 0x1C; // AnimParamID
                constexpr std::ptrdiff_t m_TargetCoordSystem = 0x20; // IKTargetCoordinateSystem
            }
            // Parent: None
            // Field count: 0
            namespace CNmAdditiveBlendTask {
            }
            // Parent: None
            // Field count: 7
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace CSeqAutoLayer {
                constexpr std::ptrdiff_t m_nLocalReference = 0x0; // int16
                constexpr std::ptrdiff_t m_nLocalPose = 0x2; // int16
                constexpr std::ptrdiff_t m_flags = 0x4; // CSeqAutoLayerFlag
                constexpr std::ptrdiff_t m_start = 0xC; // float32
                constexpr std::ptrdiff_t m_peak = 0x10; // float32
                constexpr std::ptrdiff_t m_tail = 0x14; // float32
                constexpr std::ptrdiff_t m_end = 0x18; // float32
            }
            // Parent: None
            // Field count: 2
            //
            // Metadata:
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            // MGetKV3ClassDefaults
            namespace CAnimSequenceParams {
                constexpr std::ptrdiff_t m_flFadeInTime = 0x0; // float32
                constexpr std::ptrdiff_t m_flFadeOutTime = 0x4; // float32
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
            namespace CAnimData {
                constexpr std::ptrdiff_t m_name = 0x10; // CBufferString
                constexpr std::ptrdiff_t m_animArray = 0x20; // CUtlVector<CAnimDesc>
                constexpr std::ptrdiff_t m_decoderArray = 0x38; // CUtlVector<CAnimDecoder>
                constexpr std::ptrdiff_t m_nMaxUniqueFrameIndex = 0x50; // int32
                constexpr std::ptrdiff_t m_segmentArray = 0x58; // CUtlVector<CAnimFrameSegment>
            }
            // Parent: None
            // Field count: 3
            namespace IKSolverSettings_t {
                constexpr std::ptrdiff_t m_SolverType = 0x0; // IKSolverType
                constexpr std::ptrdiff_t m_nNumIterations = 0x4; // int32
                constexpr std::ptrdiff_t m_EndEffectorRotationFixUpMode = 0x8; // EIKEndEffectorRotationFixUpMode
            }
            // Parent: None
            // Field count: 1
            //
            // Metadata:
            // MGetKV3ClassDefaults
            namespace CAnimTagManagerUpdater {
                constexpr std::ptrdiff_t m_tags = 0x38; // CUtlVector<CSmartPtr<CAnimTagBase>>
            }
        }
    }
}
